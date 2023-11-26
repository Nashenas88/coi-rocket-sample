#[macro_use]
extern crate rocket;
use crate::{
    postgres::PostgresPoolProvider,
    repositories::repo::RepositoryProvider,
    routes::{data::DataRoutes, traits::RocketExt as _},
    services::service::ServiceProvider,
};
use coi::container;
use mobc_postgres::{mobc::Pool, tokio_postgres::NoTls, PgConnectionManager};

mod dtos;
mod models;
mod postgres;
mod repositories;
mod routes;
mod services;

#[launch]
fn rocket() -> _ {
    env_logger::init();

    let config = format!(
        "host=127.0.0.1 dbname={} port=5432 user={} password={}",
        std::env::var("POSTGRES_DB").unwrap(),
        std::env::var("POSTGRES_USER").unwrap(),
        std::env::var("POSTGRES_PW").unwrap(),
    )
    .parse()
    .map_err(|e| format!("{}", e))
    .unwrap();
    let manager = PgConnectionManager::new(config, NoTls);
    let pool = Pool::builder().max_open(20).build(manager);
    let pool_provider = PostgresPoolProvider::new(pool);

    let container = container! {
        pool => pool_provider; singleton,
        service => ServiceProvider; scoped,
        repository => RepositoryProvider; scoped,
    };

    #[cfg(feature = "debug")]
    {
        if let Err(e) = container.analyze() {
            eprintln!("Misconfigured container: [");
            for e in e {
                eprintln!("  {}", e);
            }
            eprintln!("]");
        }

        use std::fs::File;
        use std::io::Write;
        let mut file = File::create("deps.dot").expect("Cannot create dot file");
        file.write(container.dot_graph().as_bytes())
            .expect("Cannot write graph to dot file");
    }

    rocket::build().manage(container).route(DataRoutes)
}
