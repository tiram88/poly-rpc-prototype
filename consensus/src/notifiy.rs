use crate::channel::Channel;
use async_std::channel::Receiver;
use consensus_core::{
    block::Block,
    blockhash::new_unique,
    header::Header,
    stubs::{BlockAddedNotification, Notification as ConsensusNotification},
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub type ConsensusNotificationChannel = Channel<Arc<ConsensusNotification>>;

#[derive(Debug, Default)]
pub struct RandomBlockProducer {
    channel: ConsensusNotificationChannel,
    terminate: Arc<AtomicBool>,
}

impl RandomBlockProducer {
    pub fn new() -> Self {
        Self { channel: Channel::default(), terminate: Arc::new(AtomicBool::new(false)) }
    }

    pub async fn start(self: &Arc<Self>) -> Receiver<Arc<ConsensusNotification>> {
        let sender = self.channel.sender();
        let terminate = self.terminate.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(1000)).await;
                let block: Block = Block {
                    header: Header {
                        hash: new_unique(),
                        version: 1,
                        parents_by_level: vec![],
                        hash_merkle_root: new_unique(),
                        accepted_id_merkle_root: new_unique(),
                        utxo_commitment: new_unique(),
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
                        bits: 1,
                        nonce: 1,
                        daa_score: 1,
                        blue_work: 1,
                        blue_score: 1,
                        pruning_point: new_unique(),
                    },
                    transactions: Arc::new(vec![]),
                };

                println!("Emit block {0}", block.header.hash.clone());
                let notification: ConsensusNotification = ConsensusNotification::BlockAdded(BlockAddedNotification { block });
                match sender.try_send(Arc::new(notification)) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("Emit error: {:?}", err);
                    }
                }

                if terminate.load(Ordering::SeqCst) {
                    break;
                }
            }
        });

        self.channel.receiver()
    }

    pub fn stop(self: Arc<Self>) {
        self.terminate.store(true, Ordering::SeqCst)
    }
}
