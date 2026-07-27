mod heartbeat;
mod session;
mod transfer;
mod validation;

pub use heartbeat::*;
pub use session::*;
pub use transfer::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlMessage {
    Session(SessionMessage),
    Transfer(TransferMessage),
    Heartbeat(HeartbeatMessage),
}
