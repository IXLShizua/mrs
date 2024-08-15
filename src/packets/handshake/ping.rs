use crate::raw::{PacketError, PacketReadExt, PacketWriteExt, RawPacket};

#[derive(Debug)]
pub struct PingRequestPacket {
    pub id: i64,
}

impl TryFrom<RawPacket> for PingRequestPacket {
    type Error = PacketError;

    fn try_from(mut value: RawPacket) -> Result<Self, Self::Error> {
        if value.id == 0x01 {
            Ok(PingRequestPacket {
                id: value.read_long()?,
            })
        } else {
            Err(PacketError::WrongId)
        }
    }
}

#[derive(Debug)]
pub struct PingResponsePacket;

impl TryFrom<PingResponsePacket> for RawPacket {
    type Error = PacketError;

    fn try_from(value: PingResponsePacket) -> Result<Self, Self::Error> {
        let mut raw = RawPacket::new(0x01);
        raw.write_long(12345)?;

        Ok(raw)
    }
}
