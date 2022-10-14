use serde::{Deserialize, Serialize};
use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};

use crate::{
    RpcBlock, RpcHash,
};

/// GetBlockRequest requests information about a specific block
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockRequest {
    /// The hash of the requested block
    pub hash: RpcHash,

    /// Whether to include transaction data in the response
    pub include_transactions: bool,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockResponse {
    pub block: RpcBlock,

    // According to app\rpc\rpchandlers\get_block.go
    // block and error as mutually exclusive
    // RpcError error = 1000;
}
