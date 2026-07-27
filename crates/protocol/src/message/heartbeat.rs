#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeartbeatMessage {
    Ping { nonce: u64 },
    Pong { nonce: u64 },
}