use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Packet parsing failed")]
    ParseError,

    #[error("Unknown error occured")]
    Unknown,
}