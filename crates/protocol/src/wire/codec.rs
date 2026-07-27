use prost::Message;

use crate::{
    ControlMessage, ProtocolError,
    wire::{
        Frame, FrameFlags, FrameType,
        convert::{decode_control_message, encode_control_message},
        proto,
    },
};

pub fn encode(message: &ControlMessage) -> Result<Frame, ProtocolError> {
    let envelope = encode_control_message(message);

    let payload = envelope.encode_to_vec();

    Frame::new(FrameType::Control, FrameFlags::NONE, payload)
}

pub fn decode(frame: Frame) -> Result<ControlMessage, ProtocolError> {
    if frame.header().frame_type() != FrameType::Control {
        return Err(ProtocolError::UnexpectedFrameType {
            expected: FrameType::Control,
            actual: frame.header().frame_type(),
        });
    }

    let envelope = proto::ControlEnvelope::decode(frame.payload())?;

    decode_control_message(envelope)
}
