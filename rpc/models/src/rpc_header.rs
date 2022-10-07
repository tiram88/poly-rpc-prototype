use serde::{Deserialize, Serialize};
use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};
use crate::prelude::RpcHash;

// pub struct RpcError {
//     pub message : String,
// }

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockHeader  {
    pub version: u32,
    pub parents: Vec<RpcBlockLevelParents>,
    pub hash_merkle_root: RpcHash,
    pub accepted_id_merkle_root: RpcHash,
    pub utxo_commitment: RpcHash,
    pub timestamp: i64,
    pub bits: u32,
    pub nonce: u64,
    pub daa_score: u64,
    pub blue_work: u128,
    pub pruning_point: RpcHash,
    pub blue_score: u64,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockLevelParents  {
    pub parent_hashes: Vec<RpcHash>,
}