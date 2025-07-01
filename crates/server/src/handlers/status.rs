use iron_oxide_common::connection::Connection;
use iron_oxide_protocol::error::Result;
use iron_oxide_versions::VersionManager;

pub async fn handle_status(conn: &mut Connection) -> Result<()> {
    let version = VersionManager::get_version(conn.protocol_version)?;
    match version.protocol_version() {
        766 => {
            iron_oxide_versions::v1_20_6::handlers::status::handle_status(
                conn,
                conn.config.players.max_players,
                conn.config.server.motd.clone(),
            )
            .await
        }
        _ => unreachable!(),
    }
}
