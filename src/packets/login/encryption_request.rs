use crate::raw::{PacketError, PacketWriteExt, RawPacket};

#[derive(Debug)]
pub struct EncryptionRequestPacket {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
    pub should_authenticate: bool,
}

impl TryFrom<EncryptionRequestPacket> for RawPacket {
    type Error = PacketError;

    fn try_from(value: EncryptionRequestPacket) -> Result<Self, Self::Error> {
        let mut raw = RawPacket::new(0x01);
        raw.write_string(&value.server_id)?;
        raw.write_byte_array(value.public_key.as_slice())?;
        raw.write_byte_array(value.verify_token.as_slice())?;
        raw.write_boolean(value.should_authenticate)?;

        Ok(raw)
    }
}
