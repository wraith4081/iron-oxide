use anyhow::Result;
use iron_oxide_protocol::packet::login::{LoginStart, LoginSuccess};
use tracing::info;
use uuid::Uuid;
use crate::connection::{Connection, ConnectionState};

pub async fn handle_login(conn: &mut Connection) -> Result<()> {
    let login_start: LoginStart = conn.read_packet().await?.unwrap();
    info!("Login start from {}", login_start.name);

    // TODO: online-mode
    let uuid = Uuid::new_v3(
        &Uuid::NAMESPACE_DNS,
        format!("OfflinePlayer:{}", login_start.name).as_bytes(),
    );

    let login_success = LoginSuccess {
        uuid,
        username: login_start.name.clone(),
    };

    conn.write_packet(login_success).await?;
    info!("Login success for {}", login_start.name);

    conn.state = ConnectionState::Play;

    Ok(())
}
