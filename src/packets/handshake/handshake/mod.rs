mod next_state;

use crate::raw::{PacketError, PacketReadExt, RawPacket};

pub use next_state::*;

#[derive(Debug)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_addr: String,
    pub server_port: u16,
    pub next_state: HandshakeNextState,
}

impl TryFrom<RawPacket> for HandshakePacket {
    type Error = PacketError;

    fn try_from(mut value: RawPacket) -> Result<Self, Self::Error> {
        let protocol_version = value.read_var_int()?;
        let server_addr = value.read_string()?;
        let server_port = value.read_unsigned_short()?;
        let next_state = value.read_var_int()?;

        Ok(HandshakePacket {
            protocol_version,
            server_addr,
            server_port,
            next_state: HandshakeNextState::try_from(next_state)?,
        })
    }
}
