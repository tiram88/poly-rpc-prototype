use crate::protowire;

impl From<rpc_models::RpcBlock> for protowire::RpcBlock {
    fn from(item: rpc_models::RpcBlock) -> protowire::RpcBlock {
        protowire::RpcBlock {
            header: Some(item.header.into()),
            transactions: vec![],
            verbose_data: None,
        }
    }
}