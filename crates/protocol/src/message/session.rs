use crate::{Capabilities, MessageError, PeerId, ProtocolVersion, SessionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionMessage {
    Hello(Hello),
    Capabilities(Capabilities),
    Close(SessionClose),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hello {
    peer_id: PeerId,
    session_id: SessionId,
    protocol_version: ProtocolVersion,
    device_name: String,
}

impl Hello {
    pub fn new(
        peer_id: PeerId,
        session_id: SessionId,
        protocol_version: ProtocolVersion,
        device_name: impl Into<String>,
    ) -> Result<Self, MessageError> {
        let device_name = device_name.into();

        super::validation::validate_text_field(
            &device_name,
            crate::limits::MAX_DEVICE_NAME_BYTES,
            MessageError::EmptyDeviceName,
            |actual, maximum| MessageError::DeviceNameTooLong { actual, maximum },
            MessageError::InvalidDeviceName,
        )?;

        Ok(Self { peer_id, session_id, protocol_version, device_name })
    }

    pub const fn peer_id(&self) -> PeerId {
        self.peer_id
    }

    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub const fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }

    pub fn device_name(&self) -> &str {
        &self.device_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionClose {
    reason: SessionCloseReason,
}

impl SessionClose {
    pub const fn new(reason: SessionCloseReason) -> Self {
        Self { reason }
    }

    pub const fn reason(&self) -> SessionCloseReason {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionCloseReason {
    Normal,
    IncompatibleProtocol,
    AuthenticationFailed,
    ProtocolViolation,
    InternalError,
}
