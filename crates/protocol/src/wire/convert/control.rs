use crate::{
    message::{HeartbeatMessage, SessionMessage, TransferMessage},
    ControlMessage,
    ProtocolError,
};

use super::{
    super::proto,
    capabilities::*,
    heartbeat::*,
    session::*,
    transfer::*,
};

pub(in crate::wire) fn encode_control_message(message: &ControlMessage) -> proto::ControlEnvelope {
    use proto::control_envelope::Message;

    let message = match message {
        ControlMessage::Session(message) => match message {
            SessionMessage::Hello(hello) => Message::Hello(encode_hello(hello)),
            SessionMessage::Capabilities(capabilities) => {
                Message::Capabilities(encode_capabilities(capabilities))
            }
            SessionMessage::Close(close) => Message::SessionClose(encode_session_close(close)),
        },

        ControlMessage::Transfer(message) => match message {
            TransferMessage::Offer(offer) => Message::TransferOffer(encode_transfer_offer(offer)),
            TransferMessage::Accept(accept) => {
                Message::TransferAccept(encode_transfer_accept(accept))
            }
            TransferMessage::Reject(reject) => {
                Message::TransferReject(encode_transfer_reject(reject))
            }
            TransferMessage::ManifestStart(start) => {
                Message::ManifestStart(encode_manifest_start(start))
            }
            TransferMessage::ManifestBatch(batch) => {
                Message::ManifestBatch(encode_manifest_batch(batch))
            }
            TransferMessage::ManifestEnd(end) => Message::ManifestEnd(encode_manifest_end(end)),
            TransferMessage::NeedChunks(need) => Message::NeedChunks(encode_need_chunks(need)),
            TransferMessage::Cancel(cancel) => {
                Message::TransferCancel(encode_transfer_cancel(cancel))
            }
            TransferMessage::Complete(complete) => {
                Message::TransferComplete(encode_transfer_complete(complete))
            }
            TransferMessage::Verified(verified) => {
                Message::TransferVerified(encode_transfer_verified(verified))
            }
            TransferMessage::Failed(failed) => {
                Message::TransferFailed(encode_transfer_failed(failed))
            }
        },

        ControlMessage::Heartbeat(message) => match message {
            HeartbeatMessage::Ping { nonce } => Message::Ping(encode_ping(*nonce)),

            HeartbeatMessage::Pong { nonce } => Message::Pong(encode_pong(*nonce)),
        },
    };

    proto::ControlEnvelope { message: Some(message) }
}

pub(in crate::wire) fn decode_control_message(
    envelope: proto::ControlEnvelope,
) -> Result<ControlMessage, ProtocolError> {
    use proto::control_envelope::Message;

    let message = envelope.message.ok_or(ProtocolError::MissingField("message"))?;

    Ok(match message {
        Message::Hello(hello) => {
            ControlMessage::Session(SessionMessage::Hello(decode_hello(hello)?))
        }

        Message::Capabilities(capabilities) => ControlMessage::Session(
            SessionMessage::Capabilities(decode_capabilities(capabilities)?),
        ),

        Message::SessionClose(close) => {
            ControlMessage::Session(SessionMessage::Close(decode_session_close(close)?))
        }

        Message::TransferOffer(offer) => {
            ControlMessage::Transfer(TransferMessage::Offer(decode_transfer_offer(offer)?))
        }

        Message::TransferAccept(accept) => {
            ControlMessage::Transfer(TransferMessage::Accept(decode_transfer_accept(accept)?))
        }

        Message::TransferReject(reject) => {
            ControlMessage::Transfer(TransferMessage::Reject(decode_transfer_reject(reject)?))
        }

        Message::ManifestStart(start) => {
            ControlMessage::Transfer(TransferMessage::ManifestStart(decode_manifest_start(start)?))
        }

        Message::ManifestBatch(batch) => {
            ControlMessage::Transfer(TransferMessage::ManifestBatch(decode_manifest_batch(batch)?))
        }

        Message::ManifestEnd(end) => {
            ControlMessage::Transfer(TransferMessage::ManifestEnd(decode_manifest_end(end)?))
        }

        Message::NeedChunks(need) => {
            ControlMessage::Transfer(TransferMessage::NeedChunks(decode_need_chunks(need)?))
        }

        Message::TransferCancel(cancel) => {
            ControlMessage::Transfer(TransferMessage::Cancel(decode_transfer_cancel(cancel)?))
        }

        Message::TransferComplete(complete) => {
            ControlMessage::Transfer(TransferMessage::Complete(decode_transfer_complete(complete)?))
        }

        Message::TransferVerified(verified) => {
            ControlMessage::Transfer(TransferMessage::Verified(decode_transfer_verified(verified)?))
        }

        Message::TransferFailed(failed) => {
            ControlMessage::Transfer(TransferMessage::Failed(decode_transfer_failed(failed)?))
        }

        Message::Ping(ping) => ControlMessage::Heartbeat(decode_ping(ping)),

        Message::Pong(pong) => ControlMessage::Heartbeat(decode_pong(pong)),
    })
}
