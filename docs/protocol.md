# Rift Protocol Specification

**Version:** 1.0  
**Status:** Draft  
**Protocol Version:** 1.0

---

## 1. Overview

Rift is a binary application-layer protocol designed for high-performance, peer-to-peer file transfer over reliable transports such as TCP and QUIC.

The protocol is built around a layered architecture that separates transport framing, message serialization, and application semantics. This separation allows independent evolution of the transport layer without affecting the protocol itself.

Protocol messages are serialized using Protocol Buffers and encapsulated inside binary frames.

---

## 2. Design Goals

The protocol has been designed with the following goals:

- Low protocol overhead
- Deterministic binary wire format
- Cross-platform interoperability
- Forward extensibility
- Strong validation of malformed input
- Transport independence
- Efficient transfer of large datasets
- Support for future protocol extensions

---

## 3. Architecture

The protocol stack is divided into multiple independent layers.

```text
+---------------------------+
| Application               |
+---------------------------+
| Domain Messages           |
+---------------------------+
| Protocol Converters       |
+---------------------------+
| Protocol Buffers          |
+---------------------------+
| Codec                     |
+---------------------------+
| Binary Frames             |
+---------------------------+
| TCP / QUIC                |
+---------------------------+
```

Each layer has a single responsibility.

| Layer | Responsibility |
|--------|----------------|
| Domain | Application message model |
| Converter | Domain ↔ Protocol Buffer conversion |
| Codec | Serialization and deserialization |
| Frame | Binary framing |
| Transport | Reliable byte transport |

---

# 4. Frame Format

Every protocol message is transmitted inside a frame.

```
0                   4                   8                  12                 16
+-------------------+-------------------+-------------------+-------------------+
| Magic ("RIFT")    | Ver | Flags | Type|    Payload Length |     Reserved      |
+-------------------+-------------------+-------------------+-------------------+
|                           Payload (variable length)                           |
+--------------------------------------------------------------------------- ---+
```

## Header Layout

| Offset | Size | Field |
|--------:|-----:|------|
| 0 | 4 | Magic |
| 4 | 1 | Header Version |
| 5 | 1 | Frame Flags |
| 6 | 2 | Frame Type |
| 8 | 4 | Payload Length |
| 12 | 4 | Reserved |

### Magic

ASCII value:

```
RIFT
```

Frames with an invalid magic value MUST be rejected.

---

### Header Version

Current version:

```
1
```

Peers MUST reject unknown header versions.

---

### Frame Flags

Flags modify the interpretation of the frame.

Currently defined:

| Flag | Value |
|------|------:|
| END_OF_STREAM | `0x01` |

Unknown flag bits MUST be rejected.

---

### Frame Type

| Value | Meaning |
|------:|---------|
| `0x0001` | Control |
| `0x0002` | Chunk Header |

Unknown frame types MUST be rejected.

---

### Payload Length

Unsigned 32-bit integer stored in network byte order.

The payload length specifies the exact number of payload bytes immediately following the frame header.

The received payload length MUST match the value contained in the header.

---

### Reserved Field

Reserved for future protocol extensions.

Current implementations MUST:

- encode the field as zero
- reject frames where the field is non-zero

---

# 5. Serialization

Control messages are serialized using Protocol Buffers.

```
ControlMessage
        │
        ▼
Protocol Converter
        │
        ▼
ControlEnvelope
        │
        ▼
Protocol Buffers
        │
        ▼
Frame Payload
```

Chunk payloads are not serialized using Protocol Buffers.

---

# 6. Message Categories

The protocol defines three message families.

## Session

Responsible for session establishment and negotiation.

Messages:

- Hello
- Capabilities
- SessionClose

---

## Transfer

Responsible for file transfer.

Messages:

- TransferOffer
- TransferAccept
- TransferReject
- ManifestStart
- ManifestBatch
- ManifestEnd
- NeedChunks
- TransferCancel
- TransferComplete
- TransferVerified
- TransferFailed

---

## Heartbeat

Responsible for connection liveness.

Messages:

- Ping
- Pong

---

# 7. Session Establishment

A session begins with protocol negotiation.

```
Peer A                                 Peer B

Hello
-------------------------------------->

                               Hello
<--------------------------------------

Capabilities
-------------------------------------->

                         Capabilities
<--------------------------------------
```

Only after successful negotiation may transfer messages be exchanged.

---

# 8. File Transfer Flow

A typical transfer follows the sequence below.

```
TransferOffer
-------------------------------------->

                          TransferAccept
<--------------------------------------

ManifestStart
-------------------------------------->

ManifestBatch
-------------------------------------->

ManifestBatch
-------------------------------------->

ManifestEnd
-------------------------------------->

                             NeedChunks
<--------------------------------------

ChunkHeader
-------------------------------------->

Chunk Data
-------------------------------------->

...

TransferComplete
-------------------------------------->

                         TransferVerified
<--------------------------------------
```

Transfers may be cancelled or rejected at any stage.

---

# 9. Identifiers

The protocol defines several globally unique identifiers.

- PeerId
- SessionId
- TransferId
- ChunkId

Identifiers are represented as UUIDs.

Requirements:

- exactly 16 bytes
- encoded in binary form
- validated during decoding

Invalid identifier lengths MUST be rejected.

---

# 10. Capabilities

Capabilities are exchanged during session negotiation.

Capability groups include:

- Transport
- Compression
- Chunking
- Features

Each capability list:

- represents peer preference order
- MUST NOT contain duplicates
- MUST contain valid enumeration values

Capability negotiation is performed by intersecting supported capabilities while preserving preference ordering.

---

# 11. Manifest

A transfer manifest describes the directory hierarchy and files that comprise a transfer.

Entry types:

- File
- Directory

Files reference one or more chunks.

Directories contain no payload data.

Relative paths are validated before a manifest is accepted.

---

# 12. Heartbeat

Heartbeat messages verify peer liveness.

```
Ping(nonce)
----------------------->

               Pong(nonce)
<-----------------------
```

The returned nonce MUST match the transmitted nonce.

---

# 13. Validation Rules

Implementations MUST reject malformed protocol messages.

Validation includes, but is not limited to:

- invalid frame magic
- unsupported header version
- unknown frame types
- unknown frame flags
- non-zero reserved field
- payload length mismatch
- invalid UUID length
- unknown protobuf enum values
- missing required message fields
- invalid protocol version
- duplicate capabilities
- invalid relative paths
- invalid chunk metadata
- malformed manifests

Validation failures are considered protocol errors.

---

# 14. Error Handling

Malformed frames MUST NOT be processed.

Typical error handling includes:

| Condition | Result |
|-----------|--------|
| Invalid frame | Reject frame |
| Invalid protobuf | Reject message |
| Unknown enum | Reject message |
| Unsupported protocol version | Close session |
| Invalid manifest | Fail transfer |
| Invalid identifier | Reject message |

Implementations SHOULD fail early when malformed input is detected.

---

# 15. Versioning

Protocol versions follow semantic versioning.

```
Major.Minor
```

Major version changes introduce breaking protocol changes.

Minor version changes remain backwards compatible within the same major version.

Protocol version negotiation occurs during the Hello exchange.

---

# 16. Protocol Limits

Protocol limits are implementation-defined constants.

Examples include:

- maximum control frame size
- maximum chunk header size
- maximum path length
- maximum chunk count
- maximum manifest size

The authoritative values are defined in:

```
src/limits.rs
```

---

# 17. Extensibility

The protocol has been designed to support future extensions without modifying the frame format.

Potential future extensions include:

- QUIC transport
- Compression algorithms
- Delta transfer
- Deduplication
- Multipath transfer
- Transfer resume
- End-to-end encryption
- Additional capability negotiation

Reserved frame header fields and extensible Protocol Buffer messages provide compatibility for future protocol revisions.

---

# 18. Compliance Requirements

An implementation is considered protocol compliant if it:

- correctly encodes protocol frames
- correctly decodes protocol frames
- validates all mandatory protocol constraints
- rejects malformed input
- negotiates compatible protocol versions
- preserves protocol ordering semantics
- conforms to the binary frame format described in this specification

---

## References

- RFC 2119 — Key words for use in RFCs to Indicate Requirement Levels
- RFC 4122 — Universally Unique Identifier (UUID)
- Protocol Buffers Language Specification