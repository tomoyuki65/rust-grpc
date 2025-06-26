use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[allow(dead_code)]
    #[error("{message}")]
    CustomError { status: Status, message: String },
}
