use std::str::FromStr;
use rpc_core::{RpcHash, RpcError, RpcResult};
use crate::protowire;

impl From<&rpc_core::RpcBlock> for protowire::RpcBlock {
    fn from(item: &rpc_core::RpcBlock) -> protowire::RpcBlock {
        protowire::RpcBlock {
            header: Some(protowire::RpcBlockHeader::from(&item.header)),
            transactions: vec![],
            verbose_data: Some(protowire::RpcBlockVerboseData::from(&item.verbose_data)),
        }
    }
}

impl From<&rpc_core::RpcBlockVerboseData> for protowire::RpcBlockVerboseData {
    fn from(item: &rpc_core::RpcBlockVerboseData) -> protowire::RpcBlockVerboseData {
        protowire::RpcBlockVerboseData {
            hash: item.hash.to_string(),
            difficulty: item.difficulty,
            selected_parent_hash: item.selected_parent_hash.to_string(),
            transaction_ids: item.transaction_ids
                .iter()
                .map(|x| x.to_string())
                .collect(),
            is_header_only: item.is_header_only,
            blue_score: item.blue_score.into(),
            children_hashes: item.children_hashes
                .iter()
                .map(|x| x.to_string())
                .collect(),
            merge_set_blues_hashes: item.merge_set_blues_hashes
                .iter()
                .map(|x| x.to_string())
                .collect(),
            merge_set_reds_hashes: item.merge_set_reds_hashes
                .iter()
                .map(|x| x.to_string())
                .collect(),
            is_chain_block: item.is_chain_block,
        }
    }
}


impl TryFrom<&protowire::RpcBlock> for rpc_core::RpcBlock {
    type Error = RpcError;
    fn try_from(item: & protowire::RpcBlock) -> RpcResult<rpc_core::RpcBlock> {
        let block = rpc_core::RpcBlock {
            header: item.header
                .as_ref()
                .ok_or(RpcError::MissingBlockHeaderError)?
                .try_into()?,
            transactions: vec![],
            verbose_data: item.verbose_data
                .as_ref()
                .ok_or(RpcError::MissingBlockVerboseDataError)?
                .try_into()?,
        };
        Ok(block)
    }
}

impl TryFrom<&protowire::RpcBlockVerboseData> for rpc_core::RpcBlockVerboseData {
    type Error = RpcError;
    fn try_from(item: &protowire::RpcBlockVerboseData) -> RpcResult<rpc_core::RpcBlockVerboseData> {
        let verbose_data = rpc_core::RpcBlockVerboseData {
            hash: RpcHash::from_str(&item.hash)?,
            difficulty: item.difficulty,
            selected_parent_hash: RpcHash::from_str(&item.selected_parent_hash)?,
            transaction_ids: item.transaction_ids
                .iter()
                .map(|x| RpcHash::from_str(x))
                .collect::<RpcResult<Vec<RpcHash>>>()?,
            is_header_only: item.is_header_only,
            blue_score: item.blue_score.into(),
            children_hashes: item.children_hashes
                .iter()
                .map(|x| RpcHash::from_str(x))
                .collect::<RpcResult<Vec<RpcHash>>>()?,
            merge_set_blues_hashes: item.merge_set_blues_hashes
                .iter()
                .map(|x| RpcHash::from_str(x))
                .collect::<RpcResult<Vec<RpcHash>>>()?,
            merge_set_reds_hashes: item.merge_set_reds_hashes
                .iter()
                .map(|x| RpcHash::from_str(x))
                .collect::<RpcResult<Vec<RpcHash>>>()?,
            is_chain_block: item.is_chain_block,
        };
        Ok(verbose_data)
    }
}
