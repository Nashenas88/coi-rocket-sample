use crate::models::data::Data;
use crate::postgres::PostgresPool;
use crate::repositories::error::Error;
use async_trait::async_trait;
use coi::Inject;
use serde::Deserialize;
use std::sync::Arc;
use tokio_postgres::NoTls;

#[derive(Deserialize)]
pub(crate) struct DbData {
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
pub(crate) trait IRepository: Inject {
    async fn get(&self, id: i64) -> Result<DbData, Error>;
    async fn get_all(&self) -> Result<Vec<DbData>, Error>;
}

#[derive(Inject)]
#[coi(provides pub(crate) dyn IRepository with Repository::new(pool))]
struct Repository {
    #[coi(inject)]
    pool: Arc<PostgresPool<NoTls>>,
}

#[async_trait]
impl IRepository for Repository {
    async fn get(&self, id: i64) -> Result<DbData, Error> {
        let client = self.pool.get().await?;
        let statement = client
            .prepare("SELECT id, name FROM data WHERE id=$1::BIGINT")
            .await?;
        let row = match client.query_one(&statement, &[&id]).await {
            Ok(row) => row,
            Err(e) => {
                log::error!("Query failed with id {}: {:?}", id, e);
                return Err(e.into());
            }
        };
        Ok(DbData {
            id: row.get(0),
            name: row.get(1),
        })
    }

    async fn get_all(&self) -> Result<Vec<DbData>, Error> {
        let client = self.pool.get().await?;
        let statement = client.prepare("SELECT id, name FROM data LIMIT 50").await?;
        let rows = client.query(&statement, &[]).await?;
        Ok(rows
            .into_iter()
            .map(|r| DbData {
                id: r.get(0),
                name: r.get(1),
            })
            .collect::<Vec<_>>())
    }
}

impl Repository {
    fn new(pool: Arc<PostgresPool<NoTls>>) -> Self {
        Self { pool }
    }
}
