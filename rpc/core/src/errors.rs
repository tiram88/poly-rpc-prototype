use thiserror::Error;

#[derive(Debug, Error)]
pub enum RpcError {

    #[error("Not implemented")]
    NotImplemented,

    #[error("Error: {0}")]
    String(String),

    #[error("Hex parsing error: {0}")]
    HexParsingError(#[from] faster_hex::Error),

    #[error("Blue work parsing error: {0}")]
    RpcBlueWorkTypeParseError(#[from] std::num::ParseIntError),

    #[error("Missing block header")]
    MissingBlockHeaderError,

    #[error("Missing block verbose data")]
    MissingBlockVerboseDataError,
}