use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {

    #[error("Error: {0}")]
    String(String),

    #[error("Tonic error {0}")]
    TonicStatus(#[from] tonic::Status),
}

impl From<Error> for RpcError {
    fn from(value: String) -> Self {
        RpcError::String(value)
    }
}

