use crate::errors::EnumReprError;

#[repr(u32)]
#[derive(Debug)]
pub enum HandshakeNextState {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

impl TryFrom<i32> for HandshakeNextState {
    type Error = EnumReprError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HandshakeNextState::Status),
            2 => Ok(HandshakeNextState::Login),
            _ => Err(EnumReprError::UnknownVariant),
        }
    }
}
