use std::{io, result};
use thiserror::Error;

pub mod sync;

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