use mobc_postgres::mobc::Error as MobcError;
use mobc_postgres::tokio_postgres::Error as PostgresError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Pool error: {0}")]
    Mobc(#[from] MobcError<PostgresError>),
    #[error("Postgress error: {0}")]
    Postgres(#[from] PostgresError),
}
