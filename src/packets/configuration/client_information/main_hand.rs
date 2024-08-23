use crate::errors::EnumReprError;

#[repr(u32)]
#[derive(Debug)]
pub enum MainHand {
    Left = 0,
    Right = 1,
}

impl TryFrom<i32> for MainHand {
    type Error = EnumReprError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MainHand::Left),
            1 => Ok(MainHand::Right),
            _ => Err(EnumReprError::UnknownVariant),
        }
    }
}
