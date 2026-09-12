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
---
# FR-223: Receive occurrence

## Description

When a communication is bound to a protocol instance, the protocol binder SHALL preserve the receive as a subject distinct from the send, the message, the delivery, the attempt and the effect ([FR-051](../../functional/FR-051-preserve-communication-identities.md)).

## Properties

- **occurrence** ([FR-221](FR-221-protocol-node-occurrence.md)) — a receive is
  an authored protocol node and carries the node occurrence identity.
- **related send occurrence** ([FR-222](FR-222-send-occurrence.md)) — the exact
  concrete send the receive is related to. The
  [choreography surface](../../../proposals/quire-v1/choreography-surface.md)
  states that `receive ... via C of SendNode` observes "A distinct receive
  related to the exact concrete send occurrence; duplicate transport receipts do
  not create another node occurrence."
- **channel** ([FR-216](FR-216-channel-premise.md)) — the declared channel,
  whose premises are bound by FR-054.
- **typed payload record** — the bound record. A receive alone establishes no
  business effect (FR-051).

A receive does not mint an effect: FR-049-AC-2 states that "a duplicate receipt
cannot mint another effect node", and FR-054-AC-1 states that "At-least-once
delivery admits a duplicate receipt while retaining the single authored
message/effect obligation".

## Identity and comparison

- **Identity kind** — *derived*, through the node occurrence it is.
- **Key** — (`static declaration path`, `workflow instance identity`,
  `iteration identity`), as held by FR-221. Equality is componentwise over that
  key and nothing else. Display name, source path, span, message text, locale
  and rendering never participate, and neither does the payload or a transport
  trace identifier (FR-051, FR-051-AC-4). The related send is a required
  relation of the receive; no requirement states whether it is also a component
  of the receive's key, and this artifact adds none.
- **Key order** — none is established. No requirement fixes a total order over
  receives. FR-054-AC-2 states that "A FIFO premise orders messages only within
  its declared channel and key scope", which is a conformance premise over
  messages on one channel and one authored key expression, not a key order over
  receive identities and not a total order.
- **Causal order** — the authored partial acyclic order of FR-052, carried by
  FR-220, never derived from key order, timestamp or source position
  (FR-052-AC-2, FR-113).
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value.
- **Digest domain** — no component of this key is a digest; a digest carried in
  provenance carries the domain it was derived under,
  `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-223-AC-1 | A receive reference does not satisfy a comparison expecting a send, a delivery, an attempt or an effect. | Analysis |
| FR-223-AC-2 | A duplicate transport receipt creates no second receive occurrence and mints no effect node. | Analysis |
| FR-223-AC-3 | A FIFO premise is not read as a key order over receive identities. | Inspection |

## Dependencies

- [FR-051](../../functional/FR-051-preserve-communication-identities.md)
  preserves receive as a distinct typed subject.
- [FR-049](../../functional/FR-049-bind-native-choreography-syntax.md) forbids a
  duplicate receipt from minting another effect node.
- [FR-054](../../functional/FR-054-bind-channel-premises.md) admits the
  duplicate receipt under an at-least-once premise.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Open gap

Two requirements describe the same repetition with different subjects, and
neither cross-references the other:

- FR-051-AC-1: "One message sent once, delivered twice and applied as one
  payment effect retains cardinalities 1, 2 and 1 for the respective subjects."
- FR-049-AC-2: "Send, receive, delivery, operation attempt, effect and
  compensation references remain distinct; a duplicate receipt cannot mint
  another effect node."

FR-051-AC-1 counts two deliveries, while the choreography surface states that
duplicate transport receipts "do not create another node occurrence". No
requirement states whether the second delivery has a receive occurrence, a
separate identity of its own, or no identity at all. This artifact records both
sides and resolves neither; see [FR-224](FR-224-delivery.md).

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
