use anyhow::Result;
use tracing::info;
use uuid::Uuid;
use crate::stream::ConnectionIO;
use crate::v1_20_6::packets::login::{LoginStart, LoginSuccess};

pub async fn handle_login(conn: &mut (impl ConnectionIO + Send)) -> Result<()> {
    let login_start: LoginStart = conn.read_packet_io().await?.unwrap();
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

    conn.write_packet_io(login_success).await?;
    info!("Login success for {}", login_start.name);

    Ok(())
}
