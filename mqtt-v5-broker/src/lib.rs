pub mod broker;
pub mod client;
mod tree;

use anyhow::Result;
use broker::{Broker, BrokerMessage};
use futures::channel::mpsc::Sender;
use futures::future::try_join_all;

use log::{debug, info};

use sidevm::{net::TcpListener, task};

mod allocator;

/// Bind tcp address TODO: make this configurable
const TCP_LISTENER_ADDR: &str = "0.0.0.0:1883";

/// Websocket tcp address TODO: make this configurable
const WEBSOCKET_TCP_LISTENER_ADDR: &str = "0.0.0.0:8083";

async fn tcp_server_loop(broker_tx: Sender<BrokerMessage>) -> Result<()> {
    info!("Listening TCP on {}", TCP_LISTENER_ADDR);
    let listener = TcpListener::listen(TCP_LISTENER_ADDR).await?;

    loop {
        let stream = listener.accept().await?;
        debug!("Client connected (tcp)");
        client::spawn(stream, broker_tx.clone());
    }
}

async fn websocket_server_loop(broker_tx: Sender<BrokerMessage>) -> Result<()> {
    info!("Listening Websocket on {}", WEBSOCKET_TCP_LISTENER_ADDR);
    let listener = TcpListener::listen(WEBSOCKET_TCP_LISTENER_ADDR).await?;

    loop {
        let socket = listener.accept().await?;
        debug!("Client connected (websocket)");
        client::spawn_websocket(socket, broker_tx.clone()).await;
    }
}

#[sidevm::main]
async fn main() {
    sidevm::logger::Logger::with_max_level(log::Level::Debug).init();

    info!("Starting broker");
    let broker = Broker::new();
    let broker_tx = broker.sender();
    let broker = task::spawn(async {
        broker.run().await;
        Ok(())
    });

    let tcp_listener = task::spawn(tcp_server_loop(broker_tx.clone()));
    let websocket_listener = task::spawn(websocket_server_loop(broker_tx));

    try_join_all([broker, tcp_listener, websocket_listener]).await.unwrap();
}
