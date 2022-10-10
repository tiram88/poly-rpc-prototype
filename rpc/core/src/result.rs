use crate::RpcError;

pub type RpcResult<T> = std::result::Result<T, RpcError>;