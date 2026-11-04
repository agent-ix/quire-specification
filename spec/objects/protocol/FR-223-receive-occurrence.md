---
id: FR-223
title: "Receive occurrence"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-051
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-049
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-221
    type: references
  - target: ix://agent-ix/quire-specification/FR-222
    type: references
  - target: ix://agent-ix/quire-specification/FR-215
    type: references
---
# FR-223: Receive occurrence

## Description

When a communication is bound to a protocol instance, the protocol binder SHALL preserve the receive as a subject distinct from the send, the message, the delivery, the attempt and the effect ([FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)).

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| key | String | 1..1 | identity |
| occurrence | ProtocolNodeOccurrence | 1..1 |  |
| related_send_occurrence | SendOccurrence | 1..1 |  |
| channel | ProtocolChannel | 1..1 |  |
| typed_payload_record | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| key | type not stated in source; see Field sources |
| typed_payload_record | type not stated in source; see Field sources |


- **occurrence** ([FR-221](FR-221-protocol-node-occurrence.md)) — a receive is
  an authored protocol node and carries the node occurrence identity.
- **related send occurrence** ([FR-222](FR-222-send-occurrence.md)) — the exact
  concrete send the receive is related to. The
  [choreography surface](../../../proposals/quire-v1/choreography-surface.md)
  states that `receive ... via C of SendNode` observes "A distinct receive
  related to the exact concrete send occurrence; duplicate transport receipts do
  not create another node occurrence."
- **channel** ([FR-215](FR-215-protocol-channel.md)) — the declared channel,
  whose premise set ([FR-216](FR-216-channel-premise.md)) is bound by FR-054.
- **typed payload record** — the bound record. A receive alone establishes no
  business effect (FR-051).

A receive does not mint an effect: FR-049-AC-2 states that "a duplicate receipt
cannot mint another effect node", and FR-054-AC-1 states that "At-least-once
delivery admits a duplicate receipt while retaining the single authored
message/effect obligation".

## Identity and comparison

- **Identity kind** — *derived*, through the node occurrence it is.
- **Key** — the complete FR-221 protocol-node-occurrence key. Equality is
  componentwise over that key and nothing else. Display name, source path,
  span, message text, locale
  and rendering never participate, and neither does the payload or a transport
  trace identifier (FR-051, FR-051-AC-4). The related send is a required
  relationship of the receive and is not a component of the complete FR-221
  occurrence key.
- **Key order** — FR-221's lexicographic order for canonical comparison only.
  FR-054-AC-2 states that "A FIFO premise orders messages only within
  its declared channel and key scope", which is a conformance premise over
  messages on one channel and one authored key expression, not a key order over
  receive identities and not a total order.
- **Causal order** — the authored partial acyclic order of FR-052, carried by
  FR-220, never derived from key order, timestamp or source position
  (FR-052-AC-2, FR-113).
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-223-AC-1 | A receive reference does not satisfy a comparison expecting a send, a delivery, an attempt or an effect. | Analysis |
| FR-223-AC-2 | A duplicate transport receipt creates no second receive occurrence and mints no effect node. | Analysis |
| FR-223-AC-3 | A FIFO premise is not read as a key order over receive identities. | Inspection |

## Dependencies

- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)
  preserves receive as a distinct typed subject.
- [FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md) forbids a
  duplicate receipt from minting another effect node.
- [FR-054](../../functional/protocol/FR-054-bind-channel-premises.md) admits the
  duplicate receipt under an at-least-once premise.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Status

Draft. Specification issue #28 fixed receipt as distinct provenance about a
delivery and retained receive occurrences separately. Executable cross-
repository evidence remains planned.
