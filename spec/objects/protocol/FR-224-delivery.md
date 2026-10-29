---
id: FR-224
title: "Delivery"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-051
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-054
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-222
    type: references
  - target: ix://agent-ix/quire-specification/FR-215
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
# FR-224: Delivery

## Description

When a communication is bound to a protocol instance, the protocol binder SHALL preserve the delivery as a subject distinct from the message, the send, the receive, the attempt and the effect ([FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)).

## Properties

The prose distinguishes the delivery identity from the relationships it must
satisfy and from the receipt records that provide observation provenance.

- **related send** ([FR-222](FR-222-send-occurrence.md)) — the send whose
  delivery cardinality this delivery counts against. FR-054 binds the delivery
  policy as an explicit conformance premise of the channel, and the
  [choreography surface](../../../proposals/quire-v1/choreography-surface.md)
  states the channel's delivery policy.
- **channel** ([FR-215](FR-215-protocol-channel.md)) — the channel whose sender,
  receiver, message types, ordering policy and delivery policy
  premises ([FR-216](FR-216-channel-premise.md)) are bound by FR-054.
- **transport record** — the transport observation the delivery is bound from.
  FR-051 requires "explicit relationships between transport records, attempts
  and effects" and states that the binder "SHALL NOT infer a business effect
  from a send, receive, successful transport status, repeated delivery, trace
  identifier or equal payload alone".

A delivery is not a protocol node occurrence. A concrete delivery uses its own
FR-287 identity and binds separately to the exact workflow, channel, message and
send occurrence. A receipt is a distinct observation/provenance record that
refers to a delivery. Repeated receipt of one delivery does not mint another
delivery, node occurrence or effect.

## Identity and comparison

- **Identity kind** — *authored*. The selected observation producer supplies
  the concrete opaque delivery identity; the binder neither derives nor repairs
  it.
- **Key** — one exact [FR-287](FR-287-authority-qualified-runtime-subject-reference.md)
  of kind `delivery`. Workflow, channel, message and related-send occurrence are
  mandatory binding relationships and are not key components. Reusing one key
  with different relationships refuses as an identity contradiction.
- **Key order** — FR-287's lexicographic order for canonical comparison only.
  No semantic total order over deliveries follows. The applicable ordering rule
  is a conformance premise with
  an explicitly partial scope: "A FIFO premise orders messages only within its
  declared channel and key scope" (FR-054-AC-2), and FR-054 states that the
  linker "SHALL NOT infer exactly-once delivery, FIFO order across channels,
  reliable delivery or synchronized clocks". That premise orders messages within
  one channel and one authored key expression; it does not order deliveries, and
  it does not totalize.
- **Causal order** — a delivery takes no causal edge of its own. Causal edges
  are the authored partial acyclic order of FR-052, constructed "without
  deriving causal edges from source order or timestamps"; "Reordering equal-time
  events does not add a causal edge absent from the protocol" (FR-052-AC-2); and
  "Arbitrary timestamp order SHALL NOT establish causality" (FR-113). No key
  order, were one authored, could be read as or converted into a causal edge.
- **Absence** — an absent component compares only to absence and is never
  compared to a default or an empty value. A send with zero observed deliveries
  under a `[0,b]` premise has absent deliveries, not a default one.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-224-AC-1 | A delivery reference does not satisfy a comparison expecting a send, a receive, an attempt or an effect. | Analysis |
| FR-224-AC-2 | A repeated delivery does not establish a second business effect. | Analysis |
| FR-224-AC-3 | Two exact delivery keys related to one send retain delivery cardinality two, while two receipt records referring to one delivery key retain delivery cardinality one. | Analysis |
| FR-224-AC-4 | Delivery-key order is canonical comparison only; FIFO or transport sequence requires an explicit authored premise and supplies no causal edge. | Analysis |
| FR-224-AC-5 | Reusing one delivery key for another workflow, channel, message or send refuses as an identity contradiction. | Analysis |

## Dependencies

- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)
  preserves delivery as a distinct typed subject and states the cardinality
  obligation.
- [FR-054](../../functional/protocol/FR-054-bind-channel-premises.md) binds delivery
  policy and ordering policy premises.
- [FR-059](../../functional/protocol/FR-059-assess-finite-global-conformance.md) requires
  duplicated and misordered events to be reported.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.
- [FR-210](FR-210-protocol-subject-vocabulary.md) records `receipt` and
  `delivery` as distinct complete-V1 subject kinds.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) defines the
  concrete delivery key and its non-causal comparison order.

## Status

Draft. Specification issue #28 fixed delivery identity, receipt separation and
comparison order. Executable cross-repository evidence remains planned.
