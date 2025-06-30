use std::io;
use tokio::net::TcpListener;
use tracing::{error, info};
use crate::connection::Connection;

mod connection;
mod handlers;
mod network;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:25565").await?;
    info!("Server listening on 127.0.0.1:25565");

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr);
        tokio::spawn(async move {
            let mut connection = Connection::new(socket);
            if let Err(e) = connection.handle().await {
                error!("Error handling connection: {}", e);
            }
        });
    }
}
