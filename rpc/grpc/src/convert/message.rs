use std::str::FromStr;
use rpc_core::{
    RpcHash, RpcError, RpcResult
};
use crate::protowire;

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

impl From<&rpc_core::GetBlockRequest> for protowire::GetBlockRequestMessage {
    fn from(item: &rpc_core::GetBlockRequest) -> Self {
        Self {
            hash: item.hash.to_string(),
            include_transactions: item.include_transactions,
        }
    }
}

impl From<&RpcResult<rpc_core::GetBlockResponse>> for protowire::GetBlockResponseMessage {
    fn from(item: &RpcResult<rpc_core::GetBlockResponse>) -> Self {
        // match item {
        //     Ok(response) => {
        //         Self {
        //             block: Some((&response.block).into()),
        //             error: None,
        //         }
        //     },
        //     Err(err) => {
        //         Self {
        //             block: None,
        //             error: Some(err.into()),
        //         }
        //     }
        // };
        Self {
            block: item
                .as_ref()
                .map(|x| protowire::RpcBlock::from(&x.block))
                .ok(),
            error: item
                .as_ref()
                .map_err(|x| protowire::RpcError::from(x))
                .err(),
        }
    }
}

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

impl TryFrom<&protowire::GetBlockRequestMessage> for rpc_core::GetBlockRequest {
    type Error = RpcError;
    fn try_from(item: &protowire::GetBlockRequestMessage) -> RpcResult<Self> {
        Ok(Self {
            hash: RpcHash::from_str(&item.hash)?,
            include_transactions: item.include_transactions,
        })
    }
}

impl TryFrom<&protowire::GetBlockResponseMessage> for rpc_core::GetBlockResponse {
    type Error = RpcError;
    fn try_from(item: &protowire::GetBlockResponseMessage) -> RpcResult<Self> {
        // if item.block.is_some() {
        //     Ok(Self {
        //         block: rpc_core::RpcBlock::try_from(item.block.as_ref().unwrap())?,
        //     })
        // } else {
        //     Err(item.error
        //         .as_ref()
        //         .expect("in absence of a block, an error message is present").message
        //         .to_string()
        //         .into())
        // }
        item.block.as_ref().map_or_else(
            || Err(item.error
            .as_ref()
            .expect("in absence of a block, an error message is present").message
            .to_string()
            .into()),
            |x| rpc_core::RpcBlock::try_from(x))
            .map(|x| rpc_core::GetBlockResponse { block: x }
        )
    }
}