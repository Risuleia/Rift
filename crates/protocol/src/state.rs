/// Current state of a peer session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Transport connection has been established.
    Connected,

    /// Hello messages have been exchanged.
    HelloExchanged,

    /// Peer authentication is in progress.
    Authenticating,

    /// Peer identity has been verified.
    Authenticated,

    /// Capabilities are being negotiated.
    NegotiatingCapabilities,

    /// Session is fully established and ready for transfers.
    Established,

    /// Session shutdown has begun.
    Closing,

    /// Session has been closed.
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferState {
    Offered,
    Accepted,
    ReceivingManifest,
    Ready,
    Transferring,
    Verifying,
    Completed,
    Rejected,
    Cancelled,
    Failed,
}
