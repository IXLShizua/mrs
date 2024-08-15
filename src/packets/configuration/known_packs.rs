use crate::raw::{PacketError, PacketReadExt, PacketWriteExt, RawPacket};

#[derive(Debug)]
pub struct KnownPacksPacket {
    pub known_packs: Vec<Pack>,
}

#[derive(Debug)]
pub struct Pack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

impl TryFrom<KnownPacksPacket> for RawPacket {
    type Error = PacketError;

    fn try_from(value: KnownPacksPacket) -> Result<Self, Self::Error> {
        let mut raw = RawPacket::new(0x0E);
        raw.write_var_int(value.known_packs.len() as i32)?;

        for pack in value.known_packs {
            raw.write_string(&pack.namespace)?;
            raw.write_string(&pack.id)?;
            raw.write_string(&pack.version)?;
        }

        Ok(raw)
    }
}

impl TryFrom<RawPacket> for KnownPacksPacket {
    type Error = PacketError;

    fn try_from(mut value: RawPacket) -> Result<Self, Self::Error> {
        if value.id == 0x07 {
            let packs_count = value.read_var_int()?;
            let known_packs = (0..packs_count)
                .map(|_| {
                    Ok(Pack {
                        namespace: value.read_string()?,
                        id: value.read_string()?,
                        version: value.read_string()?,
                    })
                })
                .collect::<Result<Vec<_>, PacketError>>()?;

            Ok(KnownPacksPacket { known_packs })
        } else {
            Err(PacketError::WrongId)
        }
    }
}
