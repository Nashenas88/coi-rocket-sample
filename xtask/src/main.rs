use bollard::{Docker, API_DEFAULT_VERSION};
use clap::Parser;
use std::{
    io::{self, ErrorKind},
    process::{Child, Command, ExitStatus},
    thread,
    time::Duration,
};
use thiserror::Error;
use tokio_postgres::{Client, Error as PostgresError, NoTls};

#[derive(Parser)]
#[clap(
    about = "Each subcommand listed below will run the subcommand before it, in this order:
build, run, init, seed

Once init or seed has been run, you can just call the run subcommand and reuse
the existing data."
)]
enum Step {
    #[clap(about = "Build the postgres docker image")]
    Build,
    #[clap(about = "Run the postgres docker image")]
    Run,
    #[clap(about = "Initialize the database in the postgres docker image")]
    Init,
    #[clap(about = "Seed dummy data to the postgres docker image")]
    Seed,
}

#[derive(Error, Debug)]
enum XtaskError {
    #[error("Io Error")]
    Io(#[from] io::Error),

    #[error("Postgres error: {0}")]
    Postgres(#[from] PostgresError),

    #[error("Failed to connect to Docker: {0}")]
    Docker(#[from] bollard::errors::Error),

    #[error("Command `{0}` did not exit: {1}")]
    Exit(String, ExitStatus),

    #[error("Uknown error: {0}")]
    Unknown(String),
}

type Result<T> = std::result::Result<T, XtaskError>;

const DOCKER_COMMAND: &str = "docker";
const DOCKER_URI: &str = "unix:///var/run/docker.sock";
const DOCKER_IMAGE_NAME: &str = "postgres:latest";

fn build_step() -> Result<()> {
    let mut command = build()?;
    success_check(command.wait(), DOCKER_COMMAND)
}

async fn run_step() -> Result<()> {
    let docker = Docker::connect_with_socket(DOCKER_URI, 120, API_DEFAULT_VERSION)?;
    let images = docker.list_images::<String>(None).await?;
    if !images.iter().any(|i| i.id == DOCKER_IMAGE_NAME) {
        build_step()?;
    }
    let mut command = run()?;
    success_check(command.wait(), DOCKER_COMMAND)
}

async fn init_step() -> Result<()> {
    let docker = Docker::connect_with_socket(DOCKER_URI, 120, API_DEFAULT_VERSION)?;
    let containers = docker.list_containers::<String>(None).await?;
    let container = containers
        .iter()
        .find(|c| c.image.as_deref() == Some(DOCKER_IMAGE_NAME));
    if container.is_some() {
        let mut client = make_client().await?;
        init_db(&mut client).await
    } else {
        let images = docker.list_images::<String>(None).await?;
        if !images.iter().any(|i| {
            i.repo_tags
                .iter()
                .any(|t| t == &format!("{}:alpine-12", DOCKER_IMAGE_NAME))
        }) {
            build_step()?;
        }
        let mut command = run()?;
        let handle = tokio::spawn(async move {
            thread::sleep(Duration::from_secs(5));
            let mut client = make_client().await?;
            init_db(&mut client).await
        });
        if let Ok(Err(e)) = handle.await {
            command.kill().map_err(Into::<XtaskError>::into)?;
            Err(XtaskError::Unknown(format!("{:?}", e)))
        } else {
            Ok(())
        }
    }
}

async fn seed_step() -> Result<()> {
    let docker = Docker::connect_with_socket(DOCKER_URI, 120, API_DEFAULT_VERSION)?;
    let containers = docker.list_containers::<String>(None).await?;
    let container = containers
        .iter()
        .find(|c| c.image.as_deref() == Some(DOCKER_IMAGE_NAME));
    if container.is_some() {
        let mut client = make_client().await?;
        init_db(&mut client).await?;
        seed(&mut client).await
    } else {
        let images = docker.list_images::<String>(None).await?;
        if !images.iter().any(|i| i.id == DOCKER_IMAGE_NAME) {
            build_step()?;
        }
        let mut command = run()?;
        let handle = tokio::spawn(async move {
            thread::sleep(Duration::from_secs(5));
            let mut client = make_client().await?;
            init_db(&mut client).await?;
            seed(&mut client).await
        });
        if let Ok(Err(e)) = handle.await {
            command.kill().map_err(Into::<XtaskError>::into)?;
            Err(XtaskError::Unknown(format!("{:?}", e)))
        } else {
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    let step = Step::parse();
    if let Err(e) = match step {
        Step::Build => build_step(),
        Step::Run => run_step().await,
        Step::Init => init_step().await,
        Step::Seed => seed_step().await,
    } {
        eprintln!("Failed to run step: {}", e);
    }
}

fn check_not_found(command: &str) -> impl Fn(io::Error) -> io::Error + '_ {
    move |e| {
        if e.kind() == ErrorKind::NotFound {
            io::Error::new(
                ErrorKind::NotFound,
                format!("{} not found on this system: {}", command, e),
            )
        } else {
            e
        }
    }
}

fn success_check(res: io::Result<ExitStatus>, command: &str) -> Result<()> {
    let status = res?;
    if status.success() {
        Ok(())
    } else {
        Err(XtaskError::Exit(command.to_owned(), status))
        // Err(io::Error::new(
        //     ErrorKind::Other,
        //     format!(
        //         "{} could not run successfully: exit code {:?}",
        //         command,
        //         status.code()
        //     ),
        // ).into())
    }
}

fn build() -> Result<Child> {
    Command::new("docker")
        .arg("compose")
        .arg("build")
        .spawn()
        .map_err(check_not_found("docker"))
        .map_err(Into::into)
}

fn run() -> Result<Child> {
    Command::new("docker")
        .arg("compose")
        .arg("up")
        .arg("-d")
        .spawn()
        .map_err(check_not_found("docker"))
        .map_err(Into::into)
}

async fn make_client() -> Result<Client> {
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host=127.0.0.1 dbname={} port=5432 user={} password={}",
            std::env::var("POSTGRES_DB").unwrap(),
            std::env::var("POSTGRES_USER").unwrap(),
            std::env::var("POSTGRES_PW").unwrap(),
        ),
        NoTls,
    )
    .await
    .map_err(XtaskError::from)?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

async fn init_db(client: &mut Client) -> Result<()> {
    let commands = include_str!("sql/init.sql");
    client.batch_execute(commands).await.map_err(Into::into)
}

async fn seed(client: &mut Client) -> Result<()> {
    client
        .batch_execute(include_str!("sql/seed.sql"))
        .await
        .map_err(Into::into)
}
