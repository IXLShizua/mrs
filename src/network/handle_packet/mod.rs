use crate::network::connection::State;
use crate::raw::{PacketError, RawPacket, TryFromRawPacket, TryIntoRawPacket};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct PacketKey {
    state: State,
    id: i32,
}

struct HandlersMap {
    inner: HashMap<PacketKey, Box<dyn PreHandlePacket>>,
}

impl HandlersMap {
    pub fn new() -> HandlersMap {
        HandlersMap {
            inner: HashMap::new(),
        }
    }

    pub fn put<T>(&mut self, state: State, id: i32, handler: T)
    where
        T: PreHandlePacket + 'static,
    {
        let key = PacketKey { state, id };
        let handler = Box::new(handler) as Box<dyn PreHandlePacket>;

        if let None = self.inner.get(&key) {
            self.inner.insert(key, handler);
        }
    }

    pub fn get(&self, state: State, id: i32) -> Option<&Box<dyn PreHandlePacket>> {
        self.inner.get(&PacketKey { state, id })
    }
}

pub trait PreHandlePacket {
    fn pre_handle(&self, packet: RawPacket) -> Result<(), PacketError>;
}

pub trait HandleIncomingPacket {
    type Packet: TryFromRawPacket;

    fn handle(&self, packet: Self::Packet) -> Result<(), PacketError>;
}

impl<T> PreHandlePacket for T
where
    T: HandleIncomingPacket,
{
    fn pre_handle(&self, packet: RawPacket) -> Result<(), PacketError> {
        let packet = T::Packet::try_from(packet)?;

        T::handle(self, packet)
    }
}

pub trait HandleOutgoingPacket {
    type Packet: TryIntoRawPacket;

    fn handle(&self, packet: RawPacket) -> Result<(), PacketError>;
}
