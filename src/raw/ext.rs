use uuid::Uuid;
use crate::raw::Result;
use crate::types::position::Position;

pub trait PacketReadExt {
    fn read_boolean(&mut self) -> Result<bool>;
    fn read_byte(&mut self) -> Result<i8>;
    fn read_unsigned_byte(&mut self) -> Result<u8>;
    fn read_short(&mut self) -> Result<i16>;
    fn read_unsigned_short(&mut self) -> Result<u16>;
    fn read_int(&mut self) -> Result<i32>;
    fn read_long(&mut self) -> Result<i64>;
    fn read_float(&mut self) -> Result<f32>;
    fn read_double(&mut self) -> Result<f64>;
    fn read_string(&mut self) -> Result<String>;
    fn read_text_component(&mut self);
    fn read_json_text_component(&mut self);
    fn read_identifier(&mut self) -> Result<()>;
    fn read_var_int(&mut self) -> Result<i32>;
    fn read_var_long(&mut self) -> Result<i64>;
    fn read_entity_metadata(&mut self) -> Result<()>;
    fn read_slot(&mut self) -> Result<()>;
    fn read_nbt(&mut self) -> Result<()>;
    fn read_position(&mut self) -> Result<Position>;
    fn read_angle(&mut self) -> Result<()>;
    fn read_uuid(&mut self) -> Result<Uuid>;
    fn read_bitset(&mut self) -> Result<()>;
    fn read_fixed_bitset(&mut self) -> Result<()>;
    fn read_byte_array(&mut self) -> Result<Vec<u8>>;
}

pub trait PacketWriteExt {
    fn write_boolean(&mut self, data: bool) -> Result<()>;
    fn write_byte(&mut self, data: i8) -> Result<()>;
    fn write_unsigned_byte(&mut self, data: u8) -> Result<()>;
    fn write_short(&mut self, data: i16) -> Result<()>;
    fn write_unsigned_short(&mut self, data: u16) -> Result<()>;
    fn write_int(&mut self, data: i32) -> Result<()>;
    fn write_long(&mut self, data: i64) -> Result<()>;
    fn write_float(&mut self, data: f32) -> Result<()>;
    fn write_double(&mut self, data: f64) -> Result<()>;
    fn write_string(&mut self, data: &str) -> Result<()>;
    fn write_text_component(&mut self) -> Result<()>;
    fn write_json_text_component(&mut self) -> Result<()>;
    fn write_identifier(&mut self) -> Result<()>;
    fn write_var_int(&mut self, data: i32) -> Result<()>;
    fn write_var_long(&mut self, data: i64) -> Result<()>;
    fn write_entity_metadata(&mut self) -> Result<()>;
    fn write_slot(&mut self) -> Result<()>;
    fn write_nbt(&mut self) -> Result<()>;
    fn write_position(&mut self, data: &Position) -> Result<()>;
    fn write_angle(&mut self) -> Result<()>;
    fn write_uuid(&mut self, data: &Uuid) -> Result<()>;
    fn write_bitset(&mut self) -> Result<()>;
    fn write_fixed_bitset(&mut self) -> Result<()>;
    fn write_byte_array(&mut self, data: &[u8]) -> Result<()>;
}
