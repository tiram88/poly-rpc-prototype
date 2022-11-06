use std::{
    time::{Duration, Instant},
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering, AtomicU64}},
    collections::VecDeque
};
use async_trait::async_trait;
use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};
use tokio::{sync::{mpsc::{self, Sender, Receiver}, oneshot}};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{codec::CompressionEncoding, transport::{Endpoint, Channel}};
use tonic::Streaming;
use rpc_core::{
    api::ops::{RpcApiOps, SubscribeCommand},
    Notification,
    notify::{
        subscriber::SubscriptionManager,
        listener::ListenerID,
    },
    utils::triggers::DuplexTrigger,
    NotificationType,
    RpcResult,
    NotificationSender,
};
use crate::protowire::{
    KaspadRequest, KaspadResponse, GetInfoRequestMessage, rpc_client::RpcClient, kaspad_request,
};
use super::{result::Result, errors::Error};

use matcher::*;
mod matcher;

pub type SenderResponse = tokio::sync::oneshot::Sender<Result<KaspadResponse>>;

#[derive(Debug)]
struct Pending {
    timestamp: Instant,
    op: RpcApiOps,
    request: KaspadRequest,
    sender: SenderResponse,
}

impl Pending {
    fn new(op: RpcApiOps, request: KaspadRequest, sender: SenderResponse) -> Self {
        Self {
            timestamp: Instant::now(),
            op,
            request,
            sender,
        }
    }

    fn is_matching(&self, response: &KaspadResponse, response_op: RpcApiOps) -> bool {
        self.op == response_op && self.request.is_matching(response)
    }
}


/// A struct to handle messages flowing to (requestes) and from (responses) a protowire server.
/// Incoming responses are associated to pending requests based on their matching operation
/// type and, for some operations like [`ClientApiOps::GetBlock`], on their properties.
/// 
/// Data flow:
/// ```
/// // KaspadRequest -> send_channel -> recv -> stream -> send -> recv_channel -> KaspadResponse
/// ```
///
/// Execution flow:
/// ```
/// // | call --------------------------------------------------------------------------------->|
/// //                                 | sender_task ----------->| receiver_task -------------->|
/// ```
/// 
/// 
/// #### Further development
/// 
/// TODO:
/// 
/// Carry any subscribe call result up to the initial RpcApiGrpc::start_notify execution.
/// For now, RpcApiGrpc::start_notify only gets a result reflecting the call to
/// Notifier::try_send_dispatch. This is not complete.
/// 
/// Design/flow:
/// 
/// Currently call is blocking until receiver_task or timeout_task do solve the pending.
/// So actual concurrency must happen higher in the code.
/// Is there a better way to handle the flow?
/// 
#[derive(Debug)]
pub struct Resolver {
    _inner: RpcClient<Channel>,

    // Pushing incoming notifications forward
    notify_channel: NotificationSender,

    // Sending to server
    send_channel: Sender<KaspadRequest>,
    pending_calls: Arc<Mutex<VecDeque<Pending>>>,
    sender_is_running : AtomicBool,
    sender_shutdown : DuplexTrigger,

    // Receiving from server
    receiver_is_running : AtomicBool,
    receiver_shutdown : DuplexTrigger,

    // Pending timeout cleaning task
    timeout_is_running : AtomicBool,
    timeout_shutdown : DuplexTrigger,
    timeout_timer_interval : AtomicU64,
    timeout_duration : AtomicU64,
}

impl Resolver {
    pub(crate) fn new(
        client: RpcClient<Channel>,
        notify_channel: NotificationSender,
        send_channel: Sender<KaspadRequest>
    ) -> Self {
        Self {
            _inner: client,
            notify_channel,
            send_channel,
            pending_calls: Arc::new(Mutex::new(VecDeque::new())),
            sender_is_running: AtomicBool::new(false),
            sender_shutdown : DuplexTrigger::new(),
            receiver_is_running: AtomicBool::new(false),
            receiver_shutdown : DuplexTrigger::new(),
            timeout_is_running: AtomicBool::new(false),
            timeout_shutdown : DuplexTrigger::new(),
            timeout_duration : AtomicU64::new(5_000),
            timeout_timer_interval : AtomicU64::new(1_000),
       }
    }

    pub(crate) async fn connect(address: String, notify_channel: NotificationSender) -> Result<Arc<Self>> {
        let channel = Endpoint::from_shared(address.clone())?
            .timeout(tokio::time::Duration::from_secs(5))
            .connect_timeout(tokio::time::Duration::from_secs(20))
            .tcp_keepalive(Some(tokio::time::Duration::from_secs(5)))
            .connect()
            .await?;

        let mut client = RpcClient::new(channel)
            .send_compressed(CompressionEncoding::Gzip)
            .accept_compressed(CompressionEncoding::Gzip);

        // External channel
        let (send_channel, recv) = mpsc::channel(2);

        // Force the opening of the stream when connected to a go kaspad server
        //
        // TODO: This wll also be useful to save here the actual capability of the server
        // to handle request ids for req/resp matching.
        send_channel.send(GetInfoRequestMessage {}.into()).await?;

        // Internal channel
        let (send, recv_channel) = mpsc::channel(2);

        // Actual KaspadRequest to KaspadResponse stream
        let stream: Streaming<KaspadResponse> = client
            .message_stream(ReceiverStream::new(recv))
            .await?
            .into_inner();

        let resolver = Arc::new(Resolver::new(client, notify_channel, send_channel));

        // KaspadRequest timeout cleaner
        resolver.clone().timeout_task();

        // KaspaRequest sender
        resolver.clone().sender_task(stream, send);

        // KaspadResponse receiver
        resolver.clone().receiver_task(recv_channel);

        Ok(resolver)
    }

    pub(crate) async fn call(&self, op: RpcApiOps, request: impl Into<KaspadRequest>) -> Result<KaspadResponse> {
        let request: KaspadRequest = request.into();
        println!("resolver call: {:?}", request);
        if request.payload.is_some() {
            let (sender,receiver) = oneshot::channel::<Result<KaspadResponse>>();
            
            {
                let pending = Pending::new(
                    op,
                    request.clone(),
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

    #[allow(unused_must_use)]
    fn timeout_task(self : Arc<Self>) {   
        self.timeout_is_running.store(true, Ordering::SeqCst);

        tokio::spawn(async move {
            
            let shutdown = self.timeout_shutdown.request.listener.clone().fuse();
            pin_mut!(shutdown);

            loop {
                
                let timeout_timer_interval = Duration::from_millis(self.timeout_timer_interval.load(Ordering::SeqCst));
                let delay = tokio::time::sleep(timeout_timer_interval).fuse();
                pin_mut!(delay);

                select! {
                    _ = shutdown => { break; },
                    _ = delay => {
                        println!("[Resolver] running timeout task");
                        let mut pending_calls = self.pending_calls.lock().unwrap();
                        let mut purge = Vec::<usize>::new();
                        let timeout = Duration::from_millis(self.timeout_duration.load(Ordering::Relaxed));

                        pending_calls.make_contiguous();
                        let (pending_slice, _) = pending_calls.as_slices();
                        for i in (0..pending_slice.len()).rev() {
                            let pending = pending_calls.get(i).unwrap();
                            if pending.timestamp.elapsed() > timeout {
                                purge.push(i);
                            }
                        }

                        for index in purge.iter() {
                            let pending = pending_calls.remove(*index);
                            if let Some(pending) = pending {

                                println!("[Resolver] timeout task purged request emmited {:?}", pending.timestamp);

                                // This attribute doesn't seem to work at expression level
                                // So it is duplicated at fn level
                                #[allow(unused_must_use)]
                                pending.sender.send(Err(Error::Timeout));
                            }
                        }
                    },
                }
            }

            println!("[Resolver] terminating timeout task");
            self.timeout_is_running.store(false, Ordering::SeqCst);
            self.timeout_shutdown.response.trigger.trigger();
        });

    }

    fn sender_task(self : Arc<Self>, mut stream: Streaming<KaspadResponse>, send: Sender<KaspadResponse>) {
        self.sender_is_running.store(true, Ordering::SeqCst);

        tokio::spawn(async move {
            loop {
                println!("[Resolver] sender task loop");

                if send.is_closed() {
                    println!("[Resolver] sender_task sender is closed");
                    break;
                }
                
                let shutdown = self.sender_shutdown.request.listener.clone();
                pin_mut!(shutdown);

                tokio::select! {
                    _ = shutdown => { break; }
                    message = stream.message() => {
                        match message {
                            Ok(msg) => {
                                match msg {
                                    Some(response) => {
                                        if let Err(err) = send.send(response).await {
                                            println!("[Resolver] sender_task sender error: {:?}", err);
                                        }
                                    },
                                    None =>{
                                        println!("[Resolver] sender_task sender error: no payload");
                                        break;
                                    }
                                }
                            },
                            Err(err) => {
                                println!("[Resolver] sender_task sender error: {:?}", err);
                            }
                        }
                    }
                }
            }
            
            println!("[Resolver] terminating sender task");
            self.sender_is_running.store(false,Ordering::SeqCst);
            self.sender_shutdown.response.trigger.trigger();
        });
    }

    fn receiver_task(self: Arc<Self>, mut recv_channel: Receiver<KaspadResponse>) {
        self.receiver_is_running.store(true,Ordering::SeqCst);

        tokio::spawn(async move {


            loop {
                println!("[Resolver] receiver task loop");

                let shutdown = self.receiver_shutdown.request.listener.clone();
                pin_mut!(shutdown);
    
                tokio::select! {
                    _ = shutdown => { break; }
                    Some(response) = recv_channel.recv() => { self.handle_response(response); }
                }
            }

            println!("[Resolver] terminating receiver task");
            self.receiver_is_running.store(false,Ordering::SeqCst);
            self.receiver_shutdown.response.trigger.trigger();
        });
    }

    #[allow(unused_must_use)]
    fn handle_response(&self, response: KaspadResponse) {
        println!("resolver handle_response: {:?}", response);
        if response.is_notification() {
            if let Ok(notification) = Notification::try_from(&response) {

                // Here we ignore any returned error
                self.notify_channel.try_send(Arc::new(notification));
                
            }
        } else if response.payload.is_some() {
            let response_op: RpcApiOps = response.payload.as_ref().unwrap().into();
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

                // This attribute doesn't seem to work at expression level
                // So it is duplicated at fn level
                #[allow(unused_must_use)]
                pending.sender.send(Ok(response));
            }
        }
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.stop_timeout().await?;
        self.stop_sender().await?;
        self.stop_receiver().await?;
        Ok(())
    }

    async fn stop_sender(&self) -> Result<()> {
        if self.sender_is_running.load(Ordering::SeqCst) != true {
            return Ok(());
        }

        self.sender_shutdown.request.trigger.trigger();
        self.sender_shutdown.response.listener.clone().await;

        Ok(())
    }

    async fn stop_receiver(&self) -> Result<()> {
        if self.receiver_is_running.load(Ordering::SeqCst) != true {
            return Ok(());
        }

        self.receiver_shutdown.request.trigger.trigger();
        self.receiver_shutdown.response.listener.clone().await;

        Ok(())
    }

    async fn stop_timeout(&self) -> Result<()> {
        if self.timeout_is_running.load(Ordering::SeqCst) != true {
            return Ok(());
        }

        self.timeout_shutdown.request.trigger.trigger();
        self.timeout_shutdown.response.listener.clone().await;
        
        Ok(())
    }

}

#[async_trait]
impl SubscriptionManager for Resolver {
    async fn start_notify(self: Arc<Self>, _: ListenerID, notification_type: NotificationType) -> RpcResult<()> {
        // FIXME: Enhance protowire with Subscribe Commands (handle Stop also)
        let request = kaspad_request::Payload::from_notification_type(&notification_type, SubscribeCommand::Start);
        self.clone().call((&request).into(), request).await?;
        Ok(())
    }

    async fn stop_notify(self: Arc<Self>, _: ListenerID, notification_type: NotificationType) -> RpcResult<()> {
        // FIXME: Enhance protowire with Subscribe Commands (handle Stop also)
        let request = kaspad_request::Payload::from_notification_type(&notification_type, SubscribeCommand::Stop);
        self.clone().call((&request).into(), request).await?;
        Ok(())
    }
}