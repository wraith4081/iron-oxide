use tracing::info;
use iron_oxide_common::connection::{Connection, ConnectionState};
use iron_oxide_protocol::error::{Error, Result};
use crate::handlers;

pub async fn handle_connection(mut conn: Connection) -> Result<()> {
    loop {
        match conn.state {
            ConnectionState::Handshaking => {
                let new_state = handlers::handshake::handle_handshake(&mut conn).await?;
                conn.state = new_state;
            }
            ConnectionState::Status => {
                if let Err(e) = handlers::status::handle_status(&mut conn).await {
                    match e {
                        Error::Io(ref io_err) if io_err.kind() == std::io::ErrorKind::UnexpectedEof => {
                            info!("Client closed connection during status");
                        }
                        _ => return Err(e),
                    }
                }
                return Ok(());
            }
            ConnectionState::Login => {
                let new_state = handlers::login::handle_login(&mut conn).await?;
                conn.state = new_state;
            }
            ConnectionState::Configuration => {
                let config = conn.config.clone();
                let new_state =
                    handlers::configuration::handle_configuration(&mut conn, config)
                        .await?;
                conn.state = new_state;
            }
            ConnectionState::Play => {
                info!("Play not implemented");
                return Ok(());
            }
        }
    }
}