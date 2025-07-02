use iron_oxide_common::config::Config;
use iron_oxide_common::connection::{Connection, ConnectionState};
use iron_oxide_protocol::error::Result;
use iron_oxide_versions::VersionManager;
use std::sync::Arc;

pub async fn handle_configuration(
    conn: &mut Connection,
    config: Arc<Config>,
) -> Result<ConnectionState> {
    let version = VersionManager::get_version(conn.protocol_version)?;
    match version.protocol_version() {
        770 => {
            iron_oxide_versions::v1_21_5::handlers::configuration::handle_configuration(
                conn, config,
            )
            .await?;
            Ok(ConnectionState::Play)
        }
        _ => unreachable!(),
    }
}