use crate::raw;
use thiserror::Error;

mod handshake;
mod configuration;
mod login;
mod play;

pub use handshake::*;
pub use configuration::*;
pub use login::*;
pub use play::*;

#[derive(Debug, Error)]
pub enum PacketHandleError {
    #[error("Packet parsing error occurred")]
    PacketError(#[from] raw::PacketError),

    #[error("Expected packet has not been sent")]
    PacketNotSend,
}
