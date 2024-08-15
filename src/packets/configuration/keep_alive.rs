use crate::raw::{PacketError, PacketReadExt, PacketWriteExt, RawPacket};

#[derive(Debug)]
pub struct KeepAlivePacket {
    pub id: i64,
}

impl TryFrom<KeepAlivePacket> for RawPacket {
    type Error = PacketError;

    fn try_from(value: KeepAlivePacket) -> Result<Self, Self::Error> {
        let mut raw = RawPacket::new(0x04);
        raw.write_long(value.id)?;

        Ok(raw)
    }
}

impl TryFrom<RawPacket> for KeepAlivePacket {
    type Error = PacketError;

    fn try_from(mut value: RawPacket) -> Result<Self, Self::Error> {
        if value.id == 0x04 {
            Ok(KeepAlivePacket {
                id: value.read_long()?,
            })
        } else {
            Err(PacketError::WrongId)
        }
    }
}
