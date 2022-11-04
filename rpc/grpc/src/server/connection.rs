use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, atomic::{AtomicBool, Ordering}},
};
use futures::{
    pin_mut,
};
use tokio::sync::mpsc;
use rpc_core::{notify::{
    listener::{ListenerReceiverSide, ListenerID},
    notifier::Notifier
}, utils::triggers::DuplexTrigger};
use crate::{
    protowire::KaspadResponse,
    server::StatusResult,
};

pub type GrpcSender = mpsc::Sender<StatusResult<KaspadResponse>>;

pub(crate) struct GrpcConnection {
    _address: SocketAddr,
    sender: GrpcSender,
    nofity_listener: ListenerReceiverSide,
    collect_shutdown: Arc<DuplexTrigger>,
    collect_is_running: Arc<AtomicBool>,
}

impl GrpcConnection {
    pub(crate) fn new (
        address: SocketAddr,
        sender: GrpcSender,
        nofity_listener: ListenerReceiverSide
    ) -> Self {
        Self {
            _address: address,
            sender,
            nofity_listener,
            collect_shutdown: Arc::new(DuplexTrigger::new()),
            collect_is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub(crate) fn start(self: Arc<Self>) {
        self.collect_task();
    }

    pub(crate) async fn _send(&self, message: StatusResult<KaspadResponse>) {
        match self.sender.send(message).await {
            Ok(_) => {}
            Err(err) => {
                println!("[send] SendError: to {}, {:?}", self._address, err);
                // TODO: drop this connection
            }
        }
    }

    async fn stop(self: Arc<Self>) {
        self.stop_collect().await
    }
    
    fn collect_task(&self) {
        let sender = self.sender.clone();
        let collect_shutdown = self.collect_shutdown.clone();
        let collect_is_running = self.collect_is_running.clone();
        let recv_channel = self.nofity_listener.recv_channel.clone();
        collect_is_running.store(true, Ordering::SeqCst);

        tokio::task::spawn(async move {

            loop {

                let shutdown = collect_shutdown.request.listener.clone();
                pin_mut!(shutdown);

                tokio::select! {
                    _ = shutdown => { break; }
                    notification = recv_channel.recv() => {
                        match notification {
                            Ok(notification) => {
                                match sender.send(Ok((&*notification).into())).await {
                                    Ok(_) => (),
                                    Err(err) => {
                                        println!("[Connection] notification sender error: {:?}", err);
                                    },
                                }
                            },
                            Err(err) => {
                                println!("[Connection] notification receiver error: {:?}", err);
                            }
                        }
                    }
                }

            }
            collect_is_running.store(false, Ordering::SeqCst);
            collect_shutdown.response.trigger.trigger();
        });
        

    }

    async fn stop_collect(&self) {
        if self.collect_is_running.load(Ordering::SeqCst) != true {
            return;
        }

        self.collect_shutdown.request.trigger.trigger();
        self.collect_shutdown.response.listener.clone().await;
    }

}

pub(crate) struct GrpcConnectionManager {
    connections: HashMap<SocketAddr, Arc<GrpcConnection>>,
    notifier: Arc<Notifier>,
}

impl GrpcConnectionManager {
    pub fn new(notifier: Arc<Notifier>) -> Self {
        Self {
            connections: HashMap::new(),
            notifier,
        }
    }

    pub(crate) async fn register(&mut self, address: SocketAddr, sender: GrpcSender) -> ListenerID {
        println!("register a new gRPC connection from: {}", address);
        let notifiy_listener = self.notifier.clone().register_new_listener(None);
        let connection = Arc::new(GrpcConnection::new(
            address,
            sender,
            notifiy_listener));
        match self.connections.insert(address.clone(), connection.clone()) {
            Some(_prev) => {
                //prev.sender.closed().await
            },
            None => {}
        }
        connection.clone().start();
        connection.nofity_listener.id
    }

    pub(crate) async fn unregister(&mut self, address: SocketAddr) {
        println!("dismiss a gRPC connection from: {}", address);
        if let Some(connection) = self.connections.remove(&address) {
            //connection.sender.closed().await;
            connection.stop().await;
        }
    }
}