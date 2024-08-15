use byteorder::ReadBytesExt;
use std::{io, result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VarLongError {
    #[error("")]
    Write(io::Error),

    #[error("")]
    Read(io::Error),

    #[error("")]
    Incomplete,

    #[error("")]
    TooLarge,
}

pub type Result<T> = result::Result<T, VarLongError>;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(target_os = "macos")
))]
pub fn encode<T: io::Write>(value: i64, mut write: T) -> Result<()> {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    // Break the number into 7-bit parts and spread them out into a vector
    let mut res = [0_u64; 2];
    {
        let x = value as u64;

        res[0] = unsafe { _pdep_u64(x, 0x7f7f7f7f7f7f7f7f) };
        res[1] = unsafe { _pdep_u64(x >> 56, 0x000000000000017f) }
    };
    let stage1: __m128i = unsafe { std::mem::transmute(res) };

    // Create a mask for where there exist values
    // This signed comparison works because all MSBs should be cleared at this point
    // Also handle the special case when num == 0
    let minimum =
        unsafe { _mm_set_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff_u8 as i8) };
    let exists = unsafe { _mm_or_si128(_mm_cmpgt_epi8(stage1, _mm_setzero_si128()), minimum) };
    let bits = unsafe { _mm_movemask_epi8(exists) };

    // Count the number of bytes used
    let bytes_needed = 32 - bits.leading_zeros() as u8; // lzcnt on supported CPUs

    // Fill that many bytes into a vector
    let ascend = unsafe { _mm_setr_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15) };
    let mask = unsafe { _mm_cmplt_epi8(ascend, _mm_set1_epi8(bytes_needed as i8)) };

    // Shift it down 1 byte so the last MSB is the only one set, and make sure only
    // the MSB is set
    let shift = unsafe { _mm_bsrli_si128::<1>(mask) };
    let msbmask = unsafe { _mm_and_si128(shift, _mm_set1_epi8(128_u8 as i8)) };

    // Merge the MSB bits into the vector
    let merged = unsafe { _mm_or_si128(stage1, msbmask) };
    let bytes = unsafe { std::mem::transmute::<__m128i, [u8; 16]>(merged) };

    write
        .write_all(unsafe { bytes.get_unchecked(..bytes_needed as usize) })
        .map_err(VarLongError::Write)?;

    Ok(())
}

#[cfg(any(
    not(any(target_arch = "x86", target_arch = "x86_64")),
    target_os = "macos"
))]
fn encode<T: io::Write>(value: i32, mut write: T) -> Result<()> {
    use byteorder::WriteBytesExt;

    let mut val = value as u64;
    loop {
        if val & 0b1111111111111111111111111111111111111111111111111111111110000000 == 0 {
            inner.write_u8(val as u8).map_err(VarLongError::Write)?;
            return Ok(bytes_vec.to_vec());
        }
        inner
            .write_u8(val as u8 & 0b01111111 | 0b10000000)
            .map_err(VarLongError::Write)?;
        val >>= 7;
    }
}

pub fn decode<T: io::Read>(mut value: T) -> Result<i64> {
    let mut val = 0;
    for i in 0..10 {
        let byte = value.read_u8().map_err(VarLongError::Read)?;
        val |= (i64::from(byte) & 0b01111111) << (i * 7);
        if byte & 0b10000000 == 0 {
            return Ok(val);
        }
    }

    Err(VarLongError::TooLarge)
}
