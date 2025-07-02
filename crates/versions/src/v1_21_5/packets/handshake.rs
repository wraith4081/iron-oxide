use iron_oxide_protocol::packet;

packet! {
    #[derive(Debug)]
    pub struct Handshake(0x00) {
        protocol_version: i32,
        server_address: String,
        server_port: u16,
        next_state: i32,
    }
}