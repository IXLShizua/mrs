use crate::network::connection::State;
use crate::raw::{PacketError, RawPacket};
use crate::types::text_component::TextComponent;

#[derive(Debug)]
pub struct DisconnectPacket {
    pub state: State,
    pub reason: TextComponent,
}

impl TryFrom<DisconnectPacket> for RawPacket {
    type Error = PacketError;

    fn try_from(value: DisconnectPacket) -> Result<Self, Self::Error> {
        let id = match value.state {
            State::Login => Ok(0x00),
            State::Configuration => Ok(0x02),
            State::Play => Ok(0x1D),
            _ => Err(PacketError::WrongId),
        }?;
        let mut raw = RawPacket::new(id);

        Ok(raw)
    }
}
