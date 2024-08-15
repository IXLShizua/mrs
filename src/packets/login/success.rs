use uuid::Uuid;
use crate::raw::{PacketError, PacketWriteExt, RawPacket};

#[derive(Debug)]
pub struct LoginSuccessPacket {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<Property>,
    pub strict_error_handling: bool
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
}

impl TryFrom<LoginSuccessPacket> for RawPacket {
    type Error = PacketError;

    fn try_from(value: LoginSuccessPacket) -> Result<Self, Self::Error> {
        let mut raw = RawPacket::new(0x02);
        raw.write_uuid(&value.uuid)?;
        raw.write_string(&value.username)?;
        raw.write_var_int(value.properties.len() as i32)?;

        for property in value.properties {
            raw.write_string(&property.name)?;
            raw.write_string(&property.value)?;
            raw.write_boolean(false)?;
        }
        
        raw.write_boolean(value.strict_error_handling)?;
        
        Ok(raw)
    }
}