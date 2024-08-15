use crate::raw::{PacketError, RawPacket};

#[derive(Debug)]
pub struct DisconnectPacket {
    pub reason: String,
}

impl TryFrom<DisconnectPacket> for RawPacket {
    type Error = PacketError;

    fn try_from(value: DisconnectPacket) -> Result<Self, Self::Error> {
        todo!()
    }
}