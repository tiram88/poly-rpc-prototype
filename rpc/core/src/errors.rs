use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum RpcError {
    #[error("Not implemented")]
    NotImplemented,

    #[error("{0}")]
    String(String),

    #[error("Integer downsize conversion error {0}")]
    IntConversionError(#[from] TryFromIntError),

    #[error("Hex parsing error: {0}")]
    HexParsingError(#[from] faster_hex::Error),

    #[error("Blue work parsing error {0}")]
    RpcBlueWorkTypeParseError(std::num::ParseIntError),

    #[error("Integer parsing error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Invalid script class: {0}")]
    InvalidRpcScriptClass(String),

    #[error("Missing required field {0}.{1}")]
    MissingRpcFieldError(String, String),
}

impl From<String> for RpcError {
    fn from(value: String) -> Self {
        RpcError::String(value)
    }
}

impl From<&str> for RpcError {
    fn from(value: &str) -> Self {
        RpcError::String(value.to_string())
    }
}
