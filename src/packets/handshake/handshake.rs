use crate::enum_repr::EnumReprError;
use crate::raw::{PacketError, PacketReadExt, RawPacket};
use std::mem;

#[derive(Debug)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_addr: String,
    pub server_port: u16,
    pub next_state: HandshakeNextState,
}

#[repr(u32)]
#[derive(Debug)]
pub enum HandshakeNextState {
    Status,
    Login,
}

impl TryFrom<i32> for HandshakeNextState {
    type Error = EnumReprError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 && value < 2 {
            Ok(unsafe { mem::transmute(value) })
        } else {
            Err(EnumReprError::UnknownVariant)
        }
    }
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
            next_state,
        })
    }
}
