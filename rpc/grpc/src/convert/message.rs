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

impl From<&rpc_core::NotifyBlockAddedRequest> for protowire::NotifyBlockAddedRequestMessage {
    fn from(_item: &rpc_core::NotifyBlockAddedRequest) -> Self {
        Self {}
    }
}

impl From<&RpcResult<rpc_core::NotifyBlockAddedResponse>> for protowire::NotifyBlockAddedResponseMessage {
    fn from(item: &RpcResult<rpc_core::NotifyBlockAddedResponse>) -> Self {
        Self {
            error: item
                .as_ref()
                .map_err(|x| protowire::RpcError::from(x))
                .err(),
        }
    }
}

impl From<&rpc_core::GetInfoRequest> for protowire::GetInfoRequestMessage {
    fn from(_item: &rpc_core::GetInfoRequest) -> Self {
        Self {}
    }
}

impl From<&RpcResult<rpc_core::GetInfoResponse>> for protowire::GetInfoResponseMessage {
    fn from(item: &RpcResult<rpc_core::GetInfoResponse>) -> Self {
        match item {
            Ok(req) => Self {
                p2p_id: req.p2p_id.clone(),
                mempool_size: req.mempool_size,
                server_version: req.server_version.clone(),
                is_utxo_indexed: req.is_utxo_indexed,
                is_synced: req.is_synced,
                error: None,
            },
            Err(err) => Self {
                p2p_id: String::default(),
                mempool_size: 0,
                server_version: String::default(),
                is_utxo_indexed: false,
                is_synced: false,
                error: Some(err.into()),
            }
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
        //         .expect("in absence of a block, an error message is present")
        //         .into())
        // }
        item.block.as_ref()
            .map_or_else(
                //|| Err(item.error.as_ref().expect("in absence of a block, an error message is present").into()),
                || item.error
                                .as_ref()
                                .map_or(Err(RpcError::MissingRpcFieldError("GetBlockResponseMessage".to_string(), "error".to_string())), |x| Err(x.into())),
                |x| rpc_core::RpcBlock::try_from(x))
            .map(|x| rpc_core::GetBlockResponse { block: x }
        )
    }
}

impl TryFrom<&protowire::NotifyBlockAddedRequestMessage> for rpc_core::NotifyBlockAddedRequest {
    type Error = RpcError;
    fn try_from(_item: &protowire::NotifyBlockAddedRequestMessage) -> RpcResult<Self> {
        Ok(Self {})
    }
}

impl TryFrom<&protowire::NotifyBlockAddedResponseMessage> for rpc_core::NotifyBlockAddedResponse {
    type Error = RpcError;
    fn try_from(item: &protowire::NotifyBlockAddedResponseMessage) -> RpcResult<Self> {
        item.error.as_ref()
            .map_or(Ok(rpc_core::NotifyBlockAddedResponse{}), |x| Err(x.into()))
    }
}

impl TryFrom<&protowire::GetInfoRequestMessage> for rpc_core::GetInfoRequest {
    type Error = RpcError;
    fn try_from(_item: &protowire::GetInfoRequestMessage) -> RpcResult<Self> {
        Ok(Self {})
    }
}

impl TryFrom<&protowire::GetInfoResponseMessage> for rpc_core::GetInfoResponse {
    type Error = RpcError;
    fn try_from(item: &protowire::GetInfoResponseMessage) -> RpcResult<Self> {
        if let Some(err) = item.error.as_ref() {
            Err(err.into())
        } else {
            Ok(Self {
                p2p_id: item.p2p_id.clone(),
                mempool_size: item.mempool_size,
                server_version: item.server_version.clone(),
                is_utxo_indexed: item.is_utxo_indexed,
                is_synced: item.is_synced,
            })
        }
    }
}