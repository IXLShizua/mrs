use byteorder::ReadBytesExt;
use std::{io, result};
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[derive(Debug, Error)]
pub enum VarIntError {
    #[error("From io: {}", 0)]
    Io(#[from] io::Error),

    #[error("Buf is not complete VarInt")]
    Incomplete,

    #[error("VarInt is too large")]
    TooLarge,
}

pub type Result<T> = result::Result<T, VarIntError>;

pub fn encode<T: io::Write>(value: i32, mut read: T) -> Result<()> {
    let x = value as u64;
    let stage1 = (x & 0x000000000000007f)
        | ((x & 0x0000000000003f80) << 1)
        | ((x & 0x00000000001fc000) << 2)
        | ((x & 0x000000000fe00000) << 3)
        | ((x & 0x00000000f0000000) << 4);

    let leading = stage1.leading_zeros();

    let unused_bytes = (leading - 1) >> 3;
    let bytes_needed = 8 - unused_bytes;

    // set all but the last MSBs
    let msbs = 0x8080808080808080;
    let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);

    let merged = stage1 | (msbs & msbmask);
    let bytes = merged.to_le_bytes();

    read.write_all(unsafe { bytes.get_unchecked(..bytes_needed as usize) })?;

    Ok(())
}

pub fn decode<T: io::Read>(mut value: T) -> Result<i32> {
    let mut val = 0;

    for i in 0..5 {
        let byte = value.read_u8().map_err(|_| VarIntError::Incomplete)?;
        val |= (i32::from(byte) & 0b01111111) << (i * 7);

        if byte & 0b10000000 == 0 {
            return Ok(val);
        }
    }

    Err(VarIntError::TooLarge)
}

pub async fn async_encode<T: AsyncWrite + Unpin>(value: i32, mut write: T) -> Result<()> {
    let x = value as u64;
    let stage1 = (x & 0x000000000000007f)
        | ((x & 0x0000000000003f80) << 1)
        | ((x & 0x00000000001fc000) << 2)
        | ((x & 0x000000000fe00000) << 3)
        | ((x & 0x00000000f0000000) << 4);

    let leading = stage1.leading_zeros();

    let unused_bytes = (leading - 1) >> 3;
    let bytes_needed = 8 - unused_bytes;

    // set all but the last MSBs
    let msbs = 0x8080808080808080;
    let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);

    let merged = stage1 | (msbs & msbmask);
    let bytes = merged.to_le_bytes();

    write
        .write_all(unsafe { bytes.get_unchecked(..bytes_needed as usize) })
        .await?;

    Ok(())
}

pub async fn async_decode<T: AsyncRead + Unpin>(mut read: T) -> Result<i32> {
    let mut val = 0;

    for i in 0..5 {
        let byte = read.read_u8().await.map_err(|_| VarIntError::Incomplete)?;
        val |= (i32::from(byte) & 0b01111111) << (i * 7);

        if byte & 0b10000000 == 0 {
            return Ok(val);
        }
    }

    Err(VarIntError::TooLarge)
}

pub fn length(value: i32) -> usize {
    match value {
        0 => 1,
        n => (31 - n.leading_zeros() as usize) / 7 + 1,
    }
}
