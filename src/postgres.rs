use coi::{Inject, Provide};
use mobc_postgres::{
    mobc::{Connection, Error as MobcError, Manager, Pool},
    PgConnectionManager,
};

#[derive(Inject)]
pub(crate) struct PostgresPool<T>(Pool<PgConnectionManager<T>>)
where
    PgConnectionManager<T>: Manager;

impl<T> PostgresPool<T>
where
    PgConnectionManager<T>: Manager,
{
    pub(crate) async fn get(
        &self,
    ) -> Result<
        Connection<PgConnectionManager<T>>,
        MobcError<<PgConnectionManager<T> as Manager>::Error>,
    > {
        self.0.get().await
    }
}

#[derive(Provide)]
#[coi(provides PostgresPool<T> with PostgresPool(self.0.clone()))]
pub(crate) struct PostgresPoolProvider<T>(Pool<PgConnectionManager<T>>)
where
    PgConnectionManager<T>: Manager;

impl<T> PostgresPoolProvider<T>
where
    PgConnectionManager<T>: Manager,
{
    pub(crate) fn new(pool: Pool<PgConnectionManager<T>>) -> Self {
        Self(pool)
    }
}
