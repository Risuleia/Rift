use crate::message::HeartbeatMessage;

use super::super::proto;
pub(super) fn encode_ping(nonce: u64) -> proto::Ping {
    proto::Ping { nonce }
}

pub(super) fn decode_ping(ping: proto::Ping) -> HeartbeatMessage {
    HeartbeatMessage::Ping { nonce: ping.nonce }
}

pub(super) fn encode_pong(nonce: u64) -> proto::Pong {
    proto::Pong { nonce }
}

pub(super) fn decode_pong(pong: proto::Pong) -> HeartbeatMessage {
    HeartbeatMessage::Pong { nonce: pong.nonce }
}
