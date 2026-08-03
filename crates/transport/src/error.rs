use thiserror::Error;

pub type Result<T, E = TransportError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("connection closed by peer")]
    ConnectionClosed,

    #[error("unexpected end of stream")]
    UnexpectedEof,
}