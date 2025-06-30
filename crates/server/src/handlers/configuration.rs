use anyhow::Result;
use crate::connection::{Connection, ConnectionState};

pub async fn handle_configuration(conn: &mut Connection) -> Result<ConnectionState> {
    // TODO: Implement configuration handling
    Ok(ConnectionState::Play)
}