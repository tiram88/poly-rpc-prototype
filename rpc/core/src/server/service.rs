//! Core server implementation for ClientAPI

use std::{time::{SystemTime, UNIX_EPOCH}, str::FromStr, vec, sync::Arc};
use async_trait::async_trait;
use hashes::Hash;
use crate::{model::*, notify::{notifier::Notifier, channel::Channel, collector::Collector as CollecterT}};
use crate::errors::*;
use crate::result::*;
use crate::api::rpc;
use super::collector::{Collector, ConsensusNotificationChannel};

#[derive(Debug)]
pub struct RpcApi{
    notifier: Arc<Notifier>,
    collector: Arc<Collector>,
}

impl RpcApi {
    pub fn new() -> Arc<Self> {
        let notifier = Arc::new(Notifier::new(false));

        // FIXME: the channel receiver should be obtained by registering to a consensus notification service
        let consensus_notifications: ConsensusNotificationChannel = Channel::default();

        let collector = Arc::new(Collector::new(consensus_notifications.receiver(), notifier.clone()));

        Arc::new(Self {
            notifier,
            collector,
        })
    }

    pub fn start(&self) -> RpcResult<()> {
        self.notifier.clone().start()?;
        self.collector.clone().start()?;
        Ok(())
    }

    pub async fn stop(&self) -> RpcResult<()> {
        self.collector.clone().stop().await?;
        self.notifier.clone().stop().await?;
        Ok(())
    }
    
}

#[async_trait]
impl rpc::RpcApi for RpcApi {
    async fn get_block(&self, req: GetBlockRequest) -> RpcResult<GetBlockResponse> {

        // This is a test to simulate a consensus error
        if req.hash.as_bytes()[0] == 0 {
            return Err(RpcError::String(format!("Block {0} not found", req.hash)));
        }

        // This is a test to simulate a respons containing a block
        Ok(GetBlockResponse { block: create_dummy_rpc_block() })
    }

    async fn get_info(&self, _req: GetInfoRequest) -> RpcResult<GetInfoResponse> {
        // Info should be queried from consensus
        Ok(GetInfoResponse{
            p2p_id: "test".to_string(),
            mempool_size: 1,
            server_version: "0.12.8".to_string(),
            is_utxo_indexed: false,
            is_synced: false,
        })
    }
}




fn create_dummy_rpc_block() -> RpcBlock {
    let sel_parent_hash = Hash::from_str("5963be67f12da63004ce1baceebd7733c4fb601b07e9b0cfb447a3c5f4f3c4f0").unwrap();
    RpcBlock {
        header: RpcBlockHeader {
            version: 1,
            parents: vec![],
            hash_merkle_root: Hash::from_str("4b5a041951c4668ecc190c6961f66e54c1ce10866bef1cf1308e46d66adab270").unwrap(),
            accepted_id_merkle_root: Hash::from_str("1a1310d49d20eab15bf62c106714bdc81e946d761701e81fabf7f35e8c47b479").unwrap(),
            utxo_commitment: Hash::from_str("e7cdeaa3a8966f3fff04e967ed2481615c76b7240917c5d372ee4ed353a5cc15").unwrap(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
            bits: 1,
            nonce: 1234,
            daa_score: 123456,
            blue_work: 1234567890.into(),
            pruning_point: Hash::from_str("7190c08d42a0f7994b183b52e7ef2f99bac0b91ef9023511cadf4da3a2184b16").unwrap(),
            blue_score: 12345678901,
        },
        transactions: vec![],
        verbose_data: RpcBlockVerboseData {
            hash: Hash::from_str("8270e63a0295d7257785b9c9b76c9a2efb7fb8d6ac0473a1bff1571c5030e995").unwrap(),
            difficulty: 5678.0,
            selected_parent_hash: sel_parent_hash.clone(),
            transaction_ids: vec![],
            is_header_only: true,
            blue_score: 98765,
            children_hashes: vec![],
            merge_set_blues_hashes: vec![],
            merge_set_reds_hashes: vec![],
            is_chain_block: true,
        },
    }
}