use crate::protowire;

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

impl From<&rpc_core::RpcError> for protowire::RpcError {
    fn from(item: &rpc_core::RpcError) -> Self {
        Self {
            message: item.to_string(),
        }
    }
}
