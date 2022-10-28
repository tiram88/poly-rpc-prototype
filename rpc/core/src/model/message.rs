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


/// NotifyBlockAddedRequest registers this connection for blockAdded notifications.
///
/// See: [`BlockAddedNotification`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyBlockAddedRequest;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyBlockAddedResponse {
    // RpcError error = 1000;
}

/// BlockAddedNotification is sent whenever a blocks has been added (NOT accepted)
/// into the DAG.
///
/// See: [`NotifyBlockAddedRequest`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockAddedNotification {
    block: RpcBlock,
}

/// GetInfoRequest returns info about the node.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    pub p2p_id: String,
    pub mempool_size: u64,
    pub server_version: String, // FIXME ?
    pub is_utxo_indexed: bool,
    pub is_synced: bool,
    // RpcError error = 1000;
}
