use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RpcError {

    #[error("Not implemented")]
    NotImplemented,

    #[error("Error: {0}")]
    String(String),

    #[error("Integer downsize conversion error {0}")]
    IntConversionError(#[from] TryFromIntError),

    #[error("Hex parsing error: {0}")]
    HexParsingError(#[from] faster_hex::Error),

    #[error("Blue work parsing error: {0}")]
    RpcBlueWorkTypeParseError(#[from] std::num::ParseIntError),

    #[error("Missing required field {0}.{1}")]
    MissingRpcFieldError(String, String),
}