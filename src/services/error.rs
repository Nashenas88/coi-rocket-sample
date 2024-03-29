#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Error calling repository: {0}")]
    RepoError(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}
