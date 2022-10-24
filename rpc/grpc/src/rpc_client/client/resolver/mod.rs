use std::{
    time::{Instant},
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}},
    collections::VecDeque
};
use tokio::sync::{mpsc::{Sender, Receiver}, oneshot};
use rpc_core::api::ops::ClientApiOps;
use crate::protowire::{KaspadRequest, KaspadResponse};

use super::{result::Result, errors::Error};

// mod matcher;
//pub type RpcResponseFn = Arc<Box<(dyn Fn(Result<KaspadResponse>) + Sync + Send)>>;
pub type SenderResponse = tokio::sync::oneshot::Sender<Result<KaspadResponse>>;

struct Pending {
    timestamp: Instant,
    op: ClientApiOps,
    request: KaspadRequest,
    sender: SenderResponse,
}

impl Pending {
    fn new(op: ClientApiOps, request: KaspadRequest, sender: SenderResponse) -> Self {
        Self {
            timestamp: Instant::now(),
            op,
            request,
            //callback,
            sender,
        }
    }

    fn is_matching(&self, _response: &KaspadResponse, response_op: ClientApiOps) -> bool {
        self.op == response_op // && self.request.is_matching(response)
    }
}


pub struct Resolver {
    send_channel: Sender<KaspadRequest>,
    pending_calls: Arc<Mutex<VecDeque<Pending>>>,
    receiver_is_running : AtomicBool,
}

impl Resolver {
    pub fn new(send_channel: Sender<KaspadRequest>) -> Self {
        Self {
            send_channel,
            pending_calls: Arc::new(Mutex::new(VecDeque::new())),
            receiver_is_running: AtomicBool::new(false),
        }
    }

    pub async fn call(&self, op: ClientApiOps, request: impl Into<KaspadRequest>) -> Result<KaspadResponse> {
        let request: KaspadRequest = request.into();
        println!("resolver call: {:?}", request);
        if request.payload.is_some() {
            let (sender,receiver) = oneshot::channel::<Result<KaspadResponse>>();
            
            {
                let pending = Pending::new(
                    op,
                    request.clone(),
                    // Arc::new(Box::new(move |result| {
                    //     let response = match result {
                    //         Ok(data) => Ok(data.to_owned()),
                    //         Err(e) => Err(e),
                    //     };
                    //     //sender.send(response).unwrap();
                    // })),
                    sender
                );

                let mut pending_calls = self.pending_calls.lock().unwrap(); 
                pending_calls.push_back(pending);
                drop(pending_calls);
            }

            self.send_channel
                .send(request)
                .await
                .map_err(|_| Error::ChannelRecvError)?;
            
            receiver.await?
        } else {
            Err(Error::MissingRequestPayload)
        }
    }

    pub fn receiver_task(self : Arc<Self>, mut recv_channel: Receiver<KaspadResponse>) {
        self.receiver_is_running.store(true,Ordering::SeqCst);

        tokio::spawn(async move {
            loop {
                match recv_channel.recv().await {
                    Some(response) => {
                        self.handle_response(response);
                    },
                    None => {
                        println!("resolver receiver task got None");
                    }
                }
            }
            //self.receiver_is_running.store(false,Ordering::SeqCst);
        });
    }

    fn handle_response(&self, response: KaspadResponse) {
        println!("resolver handle_response: {:?}", response);
        if response.payload.is_some() {
            let response_op: ClientApiOps = response.payload.as_ref().unwrap().into();
            let mut pending_calls = self.pending_calls.lock().unwrap();
            let mut pending: Option<Pending> = None;
            if pending_calls.front().is_some() {
                if pending_calls.front().unwrap().is_matching(&response, response_op.clone()) {
                    pending = pending_calls.pop_front();
                } else {
                    pending_calls.make_contiguous();
                    let (pending_slice, _) = pending_calls.as_slices();
                    for i in (0..pending_slice.len()).rev() {
                        if pending_calls.get(i).unwrap().is_matching(&response, response_op.clone()) {
                            pending = pending_calls.remove(i);
                            break;
                        }
                    }
                }
            }
            drop(pending_calls);
            if let Some(pending) = pending {
                pending.sender.send(Ok(response));
            }
        }
    }

}