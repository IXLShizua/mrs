mod chat_mode;
mod displayed_skin_parts;
mod main_hand;

use crate::raw::{PacketError, PacketReadExt, RawPacket};

pub use chat_mode::*;
pub use displayed_skin_parts::*;
pub use main_hand::*;

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

impl TryFrom<RawPacket> for ClientInformationPacket {
    type Error = PacketError;

    fn try_from(mut value: RawPacket) -> Result<Self, Self::Error> {
        if value.id == 0x00 {
            Ok(ClientInformationPacket {
                locale: value.read_string()?,
                view_distance: value.read_unsigned_byte()?,
                chat_mode: value.read_var_int().map(ChatMode::try_from)??,
                chat_colors: value.read_boolean()?,
                displayed_skin_parts: value
                    .read_unsigned_byte()
                    .map(DisplayedSkinParts::from_bits)?
                    .unwrap(),
                main_hand: value.read_var_int().map(MainHand::try_from)??,
                enable_text_filtering: value.read_boolean()?,
                allow_server_listings: value.read_boolean()?,
            })
        } else {
            Err(PacketError::WrongId)
        }
    }
}
