use crate::{Hello, ProtocolError, ProtocolVersion, SessionClose, SessionCloseReason, wire::proto};

use super::{
    decode_peer_id, decode_session_id, encode_peer_id, encode_session_id, proto_enum_converter,
};

proto_enum_converter! {
    encode = encode_session_close_reason,
    decode = decode_session_close_reason,
    domain = SessionCloseReason,
    proto = proto::SessionCloseReason,
    error = InvalidSessionCloseReason,

    {
        Normal => CloseNormal,
        IncompatibleProtocol => CloseIncompatibleProtocol,
        AuthenticationFailed => CloseAuthenticationFailed,
        ProtocolViolation => CloseProtocolViolation,
        InternalError => CloseInternalError,
    }
}

pub(super) fn encode_hello(hello: &Hello) -> proto::Hello {
    proto::Hello {
        peer_id: Some(encode_peer_id(hello.peer_id())),
        session_id: Some(encode_session_id(hello.session_id())),
        protocol_version: Some(encode_protocol_version(hello.protocol_version())),
        device_name: hello.device_name().to_owned(),
    }
}

pub(super) fn decode_hello(hello: proto::Hello) -> Result<Hello, ProtocolError> {
    let peer_id = decode_peer_id(hello.peer_id.ok_or(ProtocolError::MissingField("peer_id"))?)?;

    let session_id =
        decode_session_id(hello.session_id.ok_or(ProtocolError::MissingField("session_id"))?)?;

    let protocol_version = decode_protocol_version(
        hello.protocol_version.ok_or(ProtocolError::MissingField("protocol_version"))?,
    )?;

    Ok(Hello::new(peer_id, session_id, protocol_version, hello.device_name)?)
}

pub(super) fn encode_session_close(close: &SessionClose) -> proto::SessionClose {
    proto::SessionClose { reason: encode_session_close_reason(close.reason()) as i32 }
}

pub(super) fn decode_session_close(
    close: proto::SessionClose,
) -> Result<SessionClose, ProtocolError> {
    Ok(SessionClose::new(decode_session_close_reason(close.reason)?))
}

fn encode_protocol_version(version: ProtocolVersion) -> proto::ProtocolVersion {
    proto::ProtocolVersion { major: u32::from(version.major()), minor: u32::from(version.minor()) }
}

fn decode_protocol_version(
    version: proto::ProtocolVersion,
) -> Result<ProtocolVersion, ProtocolError> {
    let major = u16::try_from(version.major).map_err(|_| ProtocolError::InvalidProtocolVersion)?;

    let minor = u16::try_from(version.minor).map_err(|_| ProtocolError::InvalidProtocolVersion)?;

    Ok(ProtocolVersion::new(major, minor))
}
