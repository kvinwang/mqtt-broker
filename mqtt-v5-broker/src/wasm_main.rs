use futures::future::try_join_all;
use log::{debug, info};
use crate::{
    broker::{Broker, BrokerMessage},
    client,
};

use sidevm::{net::TcpListener, task};
use tokio::sync::mpsc::Sender;
use anyhow::Result;

/// Bind tcp address TODO: make this configurable
const TCP_LISTENER_ADDR: &str = "0.0.0.0:1883";

/// Websocket tcp address TODO: make this configurable
const WEBSOCKET_TCP_LISTENER_ADDR: &str = "0.0.0.0:8080";

async fn tcp_server_loop(broker_tx: Sender<BrokerMessage>) -> Result<()> {
    info!("Listening on {}", TCP_LISTENER_ADDR);
    let listener = TcpListener::bind(TCP_LISTENER_ADDR).await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        debug!("Client {} connected (tcp)", addr);
        client::spawn(stream, broker_tx.clone());
    }
}

async fn websocket_server_loop(broker_tx: Sender<BrokerMessage>) -> Result<()> {
    info!("Listening on {}", WEBSOCKET_TCP_LISTENER_ADDR);
    let listener = TcpListener::bind(WEBSOCKET_TCP_LISTENER_ADDR).await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        debug!("Client {} connected (websocket)", addr);
        client::spawn_websocket(socket, broker_tx.clone()).await;
    }
}

fn init_logging() {
    sidevm::logger::Logger::with_max_level(log::LevelFilter::Debug).init();
}

#[sidevm::main]
async fn main() {
    init_logging();

    let broker = Broker::new();
    let broker_tx = broker.sender();
    let broker = task::spawn(async {
        broker.run().await;
        Ok(())
    });

    let tcp_listener = task::spawn(tcp_server_loop(broker_tx.clone()));
    let websocket_listener = task::spawn(websocket_server_loop(broker_tx));

    try_join_all([broker, tcp_listener, websocket_listener]).await.expect("Failed to join all tasks");
}
