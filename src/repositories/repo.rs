use crate::{
    models::data::Data,
    // postgres::PostgresPool,
    repositories::error::Error,
};
use async_trait::async_trait;
use coi::Inject;
// use mobc_postgres::tokio_postgres::NoTls;
use serde::Deserialize;
// use serde_tokio_postgres::from_row;
// use std::sync::Arc;

#[derive(Deserialize)]
pub struct DbData {
    id: i64,
    name: String,
}

impl From<DbData> for Data {
    fn from(data: DbData) -> Self {
        Self {
            id: data.id,
            name: data.name,
        }
    }
}

#[async_trait]
pub trait IRepository: Inject {
    async fn get(&self, id: i64) -> Result<DbData, Error>;
    async fn get_all(&self) -> Result<Vec<DbData>, Error>;
}

#[derive(Inject)]
#[coi(provides pub dyn IRepository with Repository{})] //::new(pool))]
struct Repository {
    // #[coi(inject)]
    // pool: Arc<PostgresPool<NoTls>>,
}

#[async_trait]
impl IRepository for Repository {
    async fn get(&self, id: i64) -> Result<DbData, Error> {
        Ok(DbData {
            id,
            name: String::from("Paul"),
        })
        // let client = self.pool.get().await?;
        // let statement = client
        //     .prepare("SELECT id, name FROM data WHERE id=$1::BIGINT")
        //     .await?;
        // let row = client.query_one(&statement, &[&id]).await?;
        // let data = from_row::<DbData>(row)?;
        // Ok(data)
    }

    async fn get_all(&self) -> Result<Vec<DbData>, Error> {
        Ok(vec![DbData {
            id: 0,
            name: String::from("Paul"),
        }])
        // let client = self.pool.get().await?;
        // let statement = client.prepare("SELECT id, name FROM data LIMIT 50").await?;
        // let rows = client.query(&statement, &[]).await?;
        // let names = rows
        //     .into_iter()
        //     .map(|r| {
        //         from_row::<DbData>(r)
        //             .map(Into::into)
        //             .map_err(Into::<Error>::into)
        //     })
        //     .collect::<Result<Vec<_>, _>>()?;
        // Ok(names)
    }
}

// impl Repository {
//     fn new(pool: Arc<PostgresPool<NoTls>>) -> Self {
//         Self { pool }
//     }
// }
