use crate::enum_repr::EnumReprError;
use crate::raw::{PacketError, PacketReadExt, RawPacket};
use bitflags::bitflags;
use std::mem;

#[derive(Debug)]
pub struct ClientInformationPacket {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: DisplayedSkinParts,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}

#[repr(u32)]
#[derive(Debug)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

impl TryFrom<i32> for ChatMode {
    type Error = EnumReprError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 && value < 2 {
            Ok(unsafe { mem::transmute(value) })
        } else {
            Err(EnumReprError::UnknownVariant)
        }
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct DisplayedSkinParts: u8 {
        const CAPE = 0b0000_0001;
        const JACKET = 0b0000_0010;
        const LEFT_SLEEVE = 0b0000_0100;
        const RIGHT_SLEEVE = 0b0000_1000;
        const LEFT_PANTS = 0b0001_0000;
        const RIGHT_PANTS = 0b0010_0000;
        const HAT = 0b0100_0000;
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum MainHand {
    Left,
    Right,
}

impl TryFrom<i32> for MainHand {
    type Error = EnumReprError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 && value < 2 {
            Ok(unsafe { mem::transmute(value) })
        } else {
            Err(EnumReprError::UnknownVariant)
        }
    }
}

impl TryFrom<RawPacket> for ClientInformationPacket {
    type Error = PacketError;

    fn try_from(mut value: RawPacket) -> Result<Self, Self::Error> {
        if value.id == 0x00 {
            Ok(ClientInformationPacket {
                locale: value.read_string()?,
                view_distance: value.read_unsigned_byte()?,
                chat_mode: value.read_var_int().map(ChatMode::try_from)?.unwrap(),
                chat_colors: value.read_boolean()?,
                displayed_skin_parts: value
                    .read_unsigned_byte()
                    .map(DisplayedSkinParts::from_bits)?
                    .unwrap(),
                main_hand: value.read_var_int().map(MainHand::try_from)?.unwrap(),
                enable_text_filtering: value.read_boolean()?,
                allow_server_listings: value.read_boolean()?,
            })
        } else {
            Err(PacketError::WrongId)
        }
    }
}
