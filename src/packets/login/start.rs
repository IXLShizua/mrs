use crate::raw::{PacketError, PacketReadExt, RawPacket};
use uuid::Uuid;

#[derive(Debug)]
pub struct LoginStartPacket {
    pub name: String,
    pub player_uuid: Uuid,
}

impl TryFrom<RawPacket> for LoginStartPacket {
    type Error = PacketError;

    fn try_from(mut value: RawPacket) -> Result<Self, Self::Error> {
        if value.id == 0x00 {
            Ok(LoginStartPacket {
                name: value.read_string()?,
                player_uuid: value.read_uuid()?,
            })
        } else {
            Err(PacketError::WrongId)
        }
    }
}
