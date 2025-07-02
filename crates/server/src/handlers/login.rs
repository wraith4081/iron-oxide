use iron_oxide_common::connection::{Connection, ConnectionState};
use iron_oxide_protocol::error::Result;
use iron_oxide_versions::VersionManager;

pub async fn handle_login(conn: &mut Connection) -> Result<ConnectionState> {
    let version = VersionManager::get_version(conn.protocol_version)?;
    match version.protocol_version() {
        770 => {
            iron_oxide_versions::v1_21_5::handlers::login::handle_login(conn).await?;
            Ok(ConnectionState::Configuration)
        }
        _ => unreachable!(),
    }
}
