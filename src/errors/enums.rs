use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnumReprError {
    #[error("Unknown enum variant")]
    UnknownVariant
}