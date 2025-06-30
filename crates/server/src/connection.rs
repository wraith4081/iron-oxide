use std::io;
use tokio::net::TcpStream;
use tracing::{error, info};
use crate::handlers;

pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
}

pub struct Connection {
    pub stream: TcpStream,
    pub state: State,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            state: State::Handshaking,
        }
    }

    pub async fn handle(&mut self) -> io::Result<()> {
        loop {
            match self.state {
                State::Handshaking => {
                    self.state = handlers::handshake::handle_handshake(&mut self.stream).await?;
                }
                State::Status => {
                    if let Err(e) = handlers::status::handle_status(&mut self.stream).await {
                        if e.kind() == io::ErrorKind::UnexpectedEof {
                            info!("Client closed connection during status");
                            return Ok(());
                        }
                        error!("Error handling status: {}", e);
                        return Err(e);
                    }
                }
                State::Login => {
                    info!("Login not implemented");
                    return Ok(());
                }
                State::Play => {
                    info!("Play not implemented");
                    return Ok(());
                }
            }
        }
    }
}
