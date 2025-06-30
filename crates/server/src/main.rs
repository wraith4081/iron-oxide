use std::{fs, io};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};
use iron_oxide_common::connection::Connection;
use iron_oxide_common::config::Config;

mod handlers;
mod connection_handler;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();

    let config_str = fs::read_to_string("server.toml").expect("Failed to read server.toml");
    let config: Config = toml::from_str(&config_str).expect("Failed to parse server.toml");
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
