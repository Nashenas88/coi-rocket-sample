#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

use crate::{
    // postgres::PostgresPoolProvider,
    repositories::repo::RepositoryProvider,
    routes::{
        data::DataRoutes,
        traits::RocketExt as _,
    },
    services::service::ServiceProvider,
};
use coi::container;
// use mobc_postgres::{mobc::Pool, tokio_postgres::NoTls, PgConnectionManager};

mod dtos;
mod models;
// mod postgres;
mod repositories;
mod routes;
mod services;

fn main() -> Result<(), String> {
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    env_logger::init();

    // let config = "host=127.0.0.1 dbname=docker port=45432 user=docker password=docker"
    //     .parse()
    //     .map_err(|e| format!("{}", e))?;
    // let manager = PgConnectionManager::new(config, NoTls);
    // let pool = Pool::builder().max_open(20).build(manager);
    // let pool_provider = PostgresPoolProvider::new(pool);

    let container = container! {
        // pool => pool_provider; singleton,
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

    rocket::ignite()
        .route(DataRoutes)
        .manage(container)
        .launch();

    Ok(())

    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(container.clone())
    //         .wrap(middleware::Compress::default())
    //         .wrap(middleware::Logger::default())
    //         .configure(routes::data::route_config)
    // })
    // .bind("127.0.0.1:8000")
    // .map_err(|e| format!("{}", e))?
    // .run()
    // .await
    // .map_err(|e| format!("{}", e))
}
