use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Error {
    pub(crate) id: ErrorId,
    pub(crate) description: String,
}

#[derive(Serialize)]
pub(crate) enum ErrorId {
    NotFound = 1,
    InternalError = 2,
}
