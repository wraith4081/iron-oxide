use anyhow::Result;
use iron_oxide_versions::VersionManager;
use crate::connection::{Connection, ConnectionState};

pub async fn handle_login(conn: &mut Connection) -> Result<()> {
    let version = VersionManager::get_version(conn.protocol_version)?;
    match version.protocol_version() {
        766 => {
            iron_oxide_versions::v1_20_6::handlers::login::handle_login(conn).await?;
            conn.state = ConnectionState::Play;
            Ok(())
        }
        _ => unreachable!(),
    }
}