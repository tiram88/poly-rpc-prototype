use std::str::FromStr;
use rpc_models::RpcHash;
use crate::protowire;

impl From<rpc_models::RpcBlockHeader> for protowire::RpcBlockHeader {
    fn from(item: rpc_models::RpcBlockHeader) -> protowire::RpcBlockHeader {
        protowire::RpcBlockHeader {
            version: item.version,
            parents: item.parents.iter().map(|x| protowire::RpcBlockLevelParents::from(x)).collect(),
            hash_merkle_root: item.hash_merkle_root.to_string(),
            accepted_id_merkle_root: item.accepted_id_merkle_root.to_string(),
            utxo_commitment: item.utxo_commitment.to_string(),
            timestamp: item.timestamp,
            bits: item.bits,
            nonce: item.nonce,
            daa_score: item.daa_score,
            blue_work: item.blue_work.to_string(),
            pruning_point: item.pruning_point.to_string(),
            blue_score: item.blue_score,
        }
    }
}

impl From<&rpc_models::RpcBlockLevelParents> for protowire::RpcBlockLevelParents {
    fn from(item: &rpc_models::RpcBlockLevelParents) -> protowire::RpcBlockLevelParents {
        protowire::RpcBlockLevelParents {
            parent_hashes: item.parent_hashes.iter().map(|x| x.to_string()).collect(),
        }
    }
}

impl From<protowire::RpcBlockHeader> for rpc_models::RpcBlockHeader {
    fn from(item: protowire::RpcBlockHeader) -> rpc_models::RpcBlockHeader {
        rpc_models::RpcBlockHeader {
            version: item.version,
            parents: item.parents.iter().map(|x| rpc_models::RpcBlockLevelParents::from(x)).collect(),
            hash_merkle_root: RpcHash::from_str(&item.hash_merkle_root).unwrap(),
            accepted_id_merkle_root: RpcHash::from_str(&item.accepted_id_merkle_root).unwrap(),
            utxo_commitment: RpcHash::from_str(&item.utxo_commitment).unwrap(),
            timestamp: item.timestamp,
            bits: item.bits,
            nonce: item.nonce,
            daa_score: item.daa_score,
            blue_work: item.blue_work.parse().unwrap_or_default(),
            pruning_point: RpcHash::from_str(&item.pruning_point).unwrap(),
            blue_score: item.blue_score,
        }
    }
}

impl From<&protowire::RpcBlockLevelParents> for rpc_models::RpcBlockLevelParents {
    fn from(item: &protowire::RpcBlockLevelParents) -> rpc_models::RpcBlockLevelParents {
        rpc_models::RpcBlockLevelParents {
            parent_hashes: item.parent_hashes.iter().map(|x| RpcHash::from_str(x).unwrap()).collect(),
        }
    }
}
