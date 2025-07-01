use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};
use iron_oxide_common::connection::Connection;
use iron_oxide_common::config::Config;
use iron_oxide_protocol::error::Result;

mod handlers;
mod connection_handler;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config_str = std::fs::read_to_string("server.toml")?;
    let config: Config = toml::from_str(&config_str).map_err(|e| iron_oxide_protocol::error::Error::Protocol(format!("Failed to parse server.toml: {}", e)))?;
    let config = Arc::new(config);

    let listener = TcpListener::bind(config.server.address.clone()).await?;
    info!("Server listening on {}", config.server.address);

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr);
        let config = Arc::clone(&config);
        tokio::spawn(async move {
            let connection = Connection::new(socket, config);
            if let Err(e) = connection_handler::handle_connection(connection).await {
                error!("Error handling connection: {}", e);
            }
        });
    }
}