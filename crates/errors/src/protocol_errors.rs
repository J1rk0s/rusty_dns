use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Packet parsing failed")]
    ParseError,

    #[error("Failed to convert the packet to bytes")]
    BytesConversionError,

    #[error("Unknown error occured")]
    Unknown,
}