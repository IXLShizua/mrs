use crate::errors::EnumReprError;

#[repr(u32)]
#[derive(Debug)]
pub enum ChatMode {
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}

impl TryFrom<i32> for ChatMode {
    type Error = EnumReprError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            _ => Err(EnumReprError::UnknownVariant),
        }
    }
}
