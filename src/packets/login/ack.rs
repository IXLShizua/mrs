use crate::raw::{PacketError, RawPacket};

#[derive(Debug)]
pub struct LoginAckPacket;

impl TryFrom<RawPacket> for LoginAckPacket {
    type Error = PacketError;

    fn try_from(value: RawPacket) -> Result<Self, Self::Error> {
        if value.id == 0x03 {
            Ok(LoginAckPacket)
        } else {
            Err(PacketError::WrongId)
        }
    }
}