use crate::{
    CancelReason, ManifestBatch, ManifestEnd, ManifestStart, NeedChunks, ProtocolError, TransferAccept, TransferCancel, TransferComplete, TransferFailed, TransferFailureReason, TransferOffer, TransferReject, TransferRejectReason, TransferVerified,
};

use super::{
    super::proto,
    ids::*,
    manifest::*,
    proto_enum_converter
};

proto_enum_converter!(
    encode = encode_transfer_reject_reason,
    decode = decode_transfer_reject_reason,
    domain = TransferRejectReason,
    proto = proto::TransferRejectReason,
    error = InvalidTransferRejectReason,
    {
        Declined => RejectDeclined,
        InsufficientSpace => RejectInsufficientSpace,
        UnsupportedContent => RejectUnsupportedContent,
        PolicyDenied => RejectPolicyDenied,
        Busy => RejectBusy,
    }
);

proto_enum_converter!(
    encode = encode_cancel_reason,
    decode = decode_cancel_reason,
    domain = CancelReason,
    proto = proto::CancelReason,
    error = InvalidCancelReason,
    {
        UserRequested => CancelUserRequested,
        InsufficientSpace => CancelInsufficientSpace,
        PolicyDenied => CancelPolicyDenied,
        ShuttingDown => CancelShuttingDown,
    }
);

proto_enum_converter!(
    encode = encode_transfer_failure_reason,
    decode = decode_transfer_failure_reason,
    domain = TransferFailureReason,
    proto = proto::TransferFailureReason,
    error = InvalidTransferFailureReason,
    {
        ProtocolViolation => FailureProtocolViolation,
        IntegrityFailure => FailureIntegrityFailure,
        StorageFailure => FailureStorageFailure,
        UnsupportedCapability => FailureUnsupportedCapability,
        InternalError => FailureInternalError,
    }
);

pub(super) fn encode_transfer_offer(offer: &TransferOffer) -> proto::TransferOffer {
    proto::TransferOffer {
        transfer_id: Some(encode_transfer_id(offer.transfer_id())),
        display_name: offer.display_name().to_owned(),
        total_size: offer.total_size(),
        entry_count: offer.entry_count(),
    }
}

pub(super) fn decode_transfer_offer(
    offer: proto::TransferOffer,
) -> Result<TransferOffer, ProtocolError> {
    let transfer_id =
        decode_transfer_id(offer.transfer_id.ok_or(ProtocolError::MissingField("transfer_id"))?)?;

    Ok(TransferOffer::new(transfer_id, offer.display_name, offer.total_size, offer.entry_count)?)
}

pub(super) fn encode_transfer_accept(accept: &TransferAccept) -> proto::TransferAccept {
    proto::TransferAccept { transfer_id: Some(encode_transfer_id(accept.transfer_id())) }
}

pub(super) fn decode_transfer_accept(
    accept: proto::TransferAccept,
) -> Result<TransferAccept, ProtocolError> {
    Ok(TransferAccept::new(decode_transfer_id(
        accept.transfer_id.ok_or(ProtocolError::MissingField("transfer_id"))?,
    )?))
}

pub(super) fn encode_transfer_reject(reject: &TransferReject) -> proto::TransferReject {
    proto::TransferReject {
        transfer_id: Some(encode_transfer_id(reject.transfer_id())),
        reason: encode_transfer_reject_reason(reject.reason()) as i32,
    }
}

pub(super) fn decode_transfer_reject(
    reject: proto::TransferReject,
) -> Result<TransferReject, ProtocolError> {
    Ok(TransferReject::new(
        decode_transfer_id(reject.transfer_id.ok_or(ProtocolError::MissingField("transfer_id"))?)?,
        decode_transfer_reject_reason(reject.reason)?,
    ))
}

pub(super) fn encode_manifest_start(start: &ManifestStart) -> proto::ManifestStart {
    proto::ManifestStart {
        transfer_id: Some(encode_transfer_id(start.transfer_id())),
        entry_count: start.entry_count(),
        total_size: start.total_size(),
    }
}

pub(super) fn decode_manifest_start(
    start: proto::ManifestStart,
) -> Result<ManifestStart, ProtocolError> {
    Ok(ManifestStart::new(
        decode_transfer_id(start.transfer_id.ok_or(ProtocolError::MissingField("transfer_id"))?)?,
        start.entry_count,
        start.total_size,
    ))
}

pub(super) fn encode_manifest_batch(batch: &ManifestBatch) -> proto::ManifestBatch {
    proto::ManifestBatch {
        transfer_id: Some(encode_transfer_id(batch.transfer_id())),
        sequence: batch.sequence(),
        entries: batch.entries().iter().map(encode_manifest_entry).collect(),
    }
}

pub(super) fn decode_manifest_batch(
    batch: proto::ManifestBatch,
) -> Result<ManifestBatch, ProtocolError> {
    let transfer_id =
        decode_transfer_id(batch.transfer_id.ok_or(ProtocolError::MissingField("transfer_id"))?)?;

    let entries =
        batch.entries.into_iter().map(decode_manifest_entry).collect::<Result<Vec<_>, _>>()?;

    Ok(ManifestBatch::new(transfer_id, batch.sequence, entries)?)
}

pub(super) fn encode_manifest_end(end: &ManifestEnd) -> proto::ManifestEnd {
    proto::ManifestEnd {
        transfer_id: Some(encode_transfer_id(end.transfer_id())),
        batch_count: end.batch_count(),
    }
}

pub(super) fn decode_manifest_end(end: proto::ManifestEnd) -> Result<ManifestEnd, ProtocolError> {
    Ok(ManifestEnd::new(
        decode_transfer_id(end.transfer_id.ok_or(ProtocolError::MissingField("transfer_id"))?)?,
        end.batch_count,
    ))
}

pub(super) fn encode_need_chunks(need: &NeedChunks) -> proto::NeedChunks {
    proto::NeedChunks {
        transfer_id: Some(encode_transfer_id(need.transfer_id())),
        request_id: need.request_id(),
        chunk_ids: need.chunks().iter().copied().map(encode_chunk_id).collect(),
    }
}

pub(super) fn decode_need_chunks(need: proto::NeedChunks) -> Result<NeedChunks, ProtocolError> {
    let transfer_id =
        decode_transfer_id(need.transfer_id.ok_or(ProtocolError::MissingField("transfer_id"))?)?;

    let chunks = need.chunk_ids.into_iter().map(decode_chunk_id).collect::<Result<Vec<_>, _>>()?;

    Ok(NeedChunks::new(transfer_id, need.request_id, chunks)?)
}

pub(super) fn encode_transfer_cancel(cancel: &TransferCancel) -> proto::TransferCancel {
    proto::TransferCancel {
        transfer_id: Some(encode_transfer_id(cancel.transfer_id())),
        reason: encode_cancel_reason(cancel.reason()) as i32,
    }
}

pub(super) fn decode_transfer_cancel(
    cancel: proto::TransferCancel,
) -> Result<TransferCancel, ProtocolError> {
    Ok(TransferCancel::new(
        decode_transfer_id(cancel.transfer_id.ok_or(ProtocolError::MissingField("transfer_id"))?)?,
        decode_cancel_reason(cancel.reason)?,
    ))
}

pub(super) fn encode_transfer_complete(complete: &TransferComplete) -> proto::TransferComplete {
    proto::TransferComplete { transfer_id: Some(encode_transfer_id(complete.transfer_id())) }
}

pub(super) fn decode_transfer_complete(
    complete: proto::TransferComplete,
) -> Result<TransferComplete, ProtocolError> {
    Ok(TransferComplete::new(decode_transfer_id(
        complete.transfer_id.ok_or(ProtocolError::MissingField("transfer_id"))?,
    )?))
}

pub(super) fn encode_transfer_verified(verified: &TransferVerified) -> proto::TransferVerified {
    proto::TransferVerified { transfer_id: Some(encode_transfer_id(verified.transfer_id())) }
}

pub(super) fn decode_transfer_verified(
    verified: proto::TransferVerified,
) -> Result<TransferVerified, ProtocolError> {
    Ok(TransferVerified::new(decode_transfer_id(
        verified.transfer_id.ok_or(ProtocolError::MissingField("transfer_id"))?,
    )?))
}

pub(super) fn encode_transfer_failed(failed: &TransferFailed) -> proto::TransferFailed {
    proto::TransferFailed {
        transfer_id: Some(encode_transfer_id(failed.transfer_id())),
        reason: encode_transfer_failure_reason(failed.reason()) as i32,
    }
}

pub(super) fn decode_transfer_failed(
    failed: proto::TransferFailed,
) -> Result<TransferFailed, ProtocolError> {
    Ok(TransferFailed::new(
        decode_transfer_id(failed.transfer_id.ok_or(ProtocolError::MissingField("transfer_id"))?)?,
        decode_transfer_failure_reason(failed.reason)?,
    ))
}
