use crate::errors::EnumReprError;
use crate::types::position::Position;
use crate::types::{var_int, var_long};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};
use std::string::FromUtf8Error;
use std::{io, result};
use thiserror::Error;
use uuid::Uuid;

mod byte_vec;
mod ext;

pub use byte_vec::*;
pub use ext::*;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("The packet has an invalid ID")]
    WrongId,

    #[error("Error during interaction with IO")]
    Io(#[from] io::Error),

    #[error("Error while interacting with VarInt")]
    VarInt(#[from] var_int::VarIntError),

    #[error("Error while interacting with VarLong")]
    VarLong(#[from] var_long::VarLongError),

    #[error("Error while string decoding")]
    StringDecode(#[from] FromUtf8Error),

    #[error("Error while interacting with enum")]
    EnumRepr(#[from] EnumReprError),
}

pub type Result<T> = result::Result<T, PacketError>;

#[derive(Debug)]
pub struct RawPacket {
    pub id: i32,
    pub inner: ByteVec,
}

impl RawPacket {
    pub fn new(id: i32) -> RawPacket {
        RawPacket {
            id,
            inner: ByteVec::new(),
        }
    }

    pub fn new_with_data(id: i32, data: &[u8]) -> RawPacket {
        RawPacket {
            id,
            inner: ByteVec::from(data.to_vec()),
        }
    }

    pub fn id_len(&self) -> usize {
        var_int::length(self.id)
    }

    pub fn data_len(&self) -> usize {
        self.inner.len()
    }

    pub fn encode<T>(self) -> Result<T>
    where
        T: TryFromRawPacket,
    {
        T::try_from(self)
    }

    pub fn decode<T>(packet: T) -> Result<RawPacket>
    where
        T: TryIntoRawPacket,
    {
        T::try_into(packet)
    }
}

impl PacketReadExt for RawPacket {
    fn read_boolean(&mut self) -> Result<bool> {
        self.inner
            .read_u8()
            .map_err(PacketError::Io)
            .map(|v| v == 1)
    }

    fn read_byte(&mut self) -> Result<i8> {
        self.inner.read_i8().map_err(PacketError::Io)
    }

    fn read_unsigned_byte(&mut self) -> Result<u8> {
        self.inner.read_u8().map_err(PacketError::Io)
    }

    fn read_short(&mut self) -> Result<i16> {
        self.inner.read_i16::<BigEndian>().map_err(PacketError::Io)
    }

    fn read_unsigned_short(&mut self) -> Result<u16> {
        self.inner.read_u16::<BigEndian>().map_err(PacketError::Io)
    }

    fn read_int(&mut self) -> Result<i32> {
        self.inner.read_i32::<BigEndian>().map_err(PacketError::Io)
    }

    fn read_long(&mut self) -> Result<i64> {
        self.inner.read_i64::<BigEndian>().map_err(PacketError::Io)
    }

    fn read_float(&mut self) -> Result<f32> {
        self.inner.read_f32::<BigEndian>().map_err(PacketError::Io)
    }

    fn read_double(&mut self) -> Result<f64> {
        self.inner.read_f64::<BigEndian>().map_err(PacketError::Io)
    }

    fn read_string(&mut self) -> Result<String> {
        let length = self.read_var_int()?;
        let mut str_buf = vec![0u8; length as usize];
        self.inner
            .read_exact(&mut str_buf)
            .map_err(PacketError::Io)?;

        String::from_utf8(str_buf).map_err(PacketError::StringDecode)
    }

    fn read_text_component(&mut self) {
        todo!()
    }

    fn read_json_text_component(&mut self) {
        todo!()
    }

    fn read_identifier(&mut self) -> Result<()> {
        todo!()
    }

    fn read_var_int(&mut self) -> Result<i32> {
        var_int::sync::decode(&mut self.inner).map_err(PacketError::VarInt)
    }

    fn read_var_long(&mut self) -> Result<i64> {
        var_long::sync::decode(&mut self.inner).map_err(PacketError::VarLong)
    }

    fn read_entity_metadata(&mut self) -> Result<()> {
        todo!()
    }

    fn read_slot(&mut self) -> Result<()> {
        todo!()
    }

    fn read_nbt(&mut self) -> Result<()> {
        todo!()
    }

    fn read_position(&mut self) -> Result<Position> {
        let val = self.read_long()?;

        Ok(Position {
            x: val >> 38,
            y: val << 52 >> 52,
            z: val << 26 >> 38,
        })
    }

    fn read_angle(&mut self) -> Result<()> {
        todo!()
    }

    fn read_uuid(&mut self) -> Result<Uuid> {
        self.inner
            .read_u128::<BigEndian>()
            .map_err(PacketError::Io)
            .map(Uuid::from_u128)
    }

    fn read_bitset(&mut self) -> Result<()> {
        todo!()
    }

    fn read_fixed_bitset(&mut self) -> Result<()> {
        todo!()
    }

    fn read_byte_array(&mut self) -> Result<Vec<u8>> {
        let length = self.read_var_int()?;
        let mut buf = vec![0u8; length as usize];
        self.inner.read_exact(&mut buf)?;

        Ok(buf)
    }
}

impl PacketWriteExt for RawPacket {
    fn write_boolean(&mut self, data: bool) -> Result<()> {
        self.inner.write_u8(data as u8).map_err(PacketError::Io)
    }

    fn write_byte(&mut self, data: i8) -> Result<()> {
        self.inner.write_i8(data).map_err(PacketError::Io)
    }

    fn write_unsigned_byte(&mut self, data: u8) -> Result<()> {
        self.inner.write_u8(data).map_err(PacketError::Io)
    }

    fn write_short(&mut self, data: i16) -> Result<()> {
        self.inner
            .write_i16::<BigEndian>(data)
            .map_err(PacketError::Io)
    }

    fn write_unsigned_short(&mut self, data: u16) -> Result<()> {
        self.inner
            .write_u16::<BigEndian>(data)
            .map_err(PacketError::Io)
    }

    fn write_int(&mut self, data: i32) -> Result<()> {
        self.inner
            .write_i32::<BigEndian>(data)
            .map_err(PacketError::Io)
    }

    fn write_long(&mut self, data: i64) -> Result<()> {
        self.inner
            .write_i64::<BigEndian>(data)
            .map_err(PacketError::Io)
    }

    fn write_float(&mut self, data: f32) -> Result<()> {
        self.inner
            .write_f32::<BigEndian>(data)
            .map_err(PacketError::Io)
    }

    fn write_double(&mut self, data: f64) -> Result<()> {
        self.inner
            .write_f64::<BigEndian>(data)
            .map_err(PacketError::Io)
    }

    fn write_string(&mut self, data: &str) -> Result<()> {
        let str_len = data.len();
        self.write_var_int(str_len as i32)?;
        self.inner
            .write_all(data.as_bytes())
            .map_err(PacketError::Io)
    }

    fn write_text_component(&mut self) -> Result<()> {
        todo!()
    }

    fn write_json_text_component(&mut self) -> Result<()> {
        todo!()
    }

    fn write_identifier(&mut self) -> Result<()> {
        todo!()
    }

    fn write_var_int(&mut self, data: i32) -> Result<()> {
        var_int::sync::encode(data, &mut self.inner).map_err(PacketError::VarInt)
    }

    fn write_var_long(&mut self, data: i64) -> Result<()> {
        var_long::sync::encode(data, &mut self.inner).map_err(PacketError::VarLong)
    }

    fn write_entity_metadata(&mut self) -> Result<()> {
        todo!()
    }

    fn write_slot(&mut self) -> Result<()> {
        todo!()
    }

    fn write_nbt(&mut self) -> Result<()> {
        todo!()
    }

    fn write_position(&mut self, data: &Position) -> Result<()> {
        let val = ((data.x & 0x3FFFFFF) << 38) | ((data.z & 0x3FFFFFF) << 12) | (data.y & 0xFFF);

        self.write_long(val)
    }

    fn write_angle(&mut self) -> Result<()> {
        todo!()
    }

    fn write_uuid(&mut self, data: &Uuid) -> Result<()> {
        self.inner
            .write_u128::<BigEndian>(data.as_u128())
            .map_err(PacketError::Io)
    }

    fn write_bitset(&mut self) -> Result<()> {
        todo!()
    }

    fn write_fixed_bitset(&mut self) -> Result<()> {
        todo!()
    }

    fn write_byte_array(&mut self, data: &[u8]) -> Result<()> {
        self.write_var_int(data.len() as i32)?;
        self.inner.extend_from_slice(data);

        Ok(())
    }
}

pub trait TryIntoRawPacket: TryInto<RawPacket, Error = PacketError> {}
impl<T> TryIntoRawPacket for T where T: TryInto<RawPacket, Error = PacketError> {}

pub trait TryFromRawPacket: TryFrom<RawPacket, Error = PacketError> {}
impl<T> TryFromRawPacket for T where T: TryFrom<RawPacket, Error = PacketError> {}

impl TryFrom<RawPacket> for Vec<u8> {
    type Error = PacketError;

    fn try_from(value: RawPacket) -> result::Result<Self, Self::Error> {
        let mut bytes = ByteVec::new();
        let length = (value.id_len() + value.data_len()) as i32;

        var_int::sync::encode(length, &mut bytes).map_err(PacketError::VarInt)?;
        var_int::sync::encode(value.id, &mut bytes).map_err(PacketError::VarInt)?;
        bytes
            .write_all(value.inner.as_slice())
            .map_err(PacketError::Io)?;

        Ok(bytes.to_vec())
    }
}

impl TryFrom<Vec<u8>> for RawPacket {
    type Error = PacketError;

    fn try_from(value: Vec<u8>) -> result::Result<Self, Self::Error> {
        let mut value = ByteVec::from(value);

        let _length = var_int::sync::decode(&mut value).map_err(PacketError::VarInt)?;
        let id = var_int::sync::decode(&mut value).map_err(PacketError::VarInt)?;

        Ok(RawPacket::new_with_data(id, value.as_slice()))
    }
}

pub trait TryIntoVec {
    fn try_into_vec(self) -> Result<Vec<u8>>;
}

pub trait TryFromVec<T> {
    fn try_from_vec(value: Vec<u8>) -> Result<T>;
}

impl<T> TryIntoVec for T
where
    T: TryIntoRawPacket,
{
    fn try_into_vec(self) -> Result<Vec<u8>> {
        let raw = self.try_into()?;

        Vec::try_from(raw)
    }
}

impl<T> TryFromVec<T> for T
where
    T: TryFromRawPacket,
{
    fn try_from_vec(value: Vec<u8>) -> Result<T> {
        let raw = RawPacket::try_from(value)?;

        Self::try_from(raw)
    }
}
