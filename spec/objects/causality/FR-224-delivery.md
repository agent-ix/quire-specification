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
  - target: ix://agent-ix/quire-specification/FR-216
    type: references
---
# FR-224: Delivery

## Description

When a communication is bound to a protocol instance, the protocol binder SHALL preserve the delivery as a subject distinct from the message, the send, the receive, the attempt and the effect ([FR-051](../../functional/FR-051-preserve-communication-identities.md)).

## Properties

The prose establishes what a delivery is related to and what it is counted
against. It establishes no component that distinguishes one delivery from
another; see **Open gap**.

- **related send** ([FR-222](FR-222-send-occurrence.md)) — the send whose
  delivery cardinality this delivery counts against. FR-054 binds "delivery
  cardinality" as an explicit conformance premise of the channel, and the
  [choreography surface](../../../proposals/quire-v1/choreography-surface.md)
  states that "Channel `delivery [a,b]` declares inclusive per-send delivery
  cardinality with `0 <= a <= b` and finite b".
- **channel** ([FR-216](../protocol/FR-216-channel-premise.md)) — the channel whose sender,
  receiver, message types, ordering policy, delivery cardinality and duplication
  premises are bound by FR-054.
- **transport record** — the transport observation the delivery is bound from.
  FR-051 requires "explicit relationships between transport records, attempts
  and effects" and states that the binder "SHALL NOT infer a business effect
  from a send, receive, successful transport status, repeated delivery, trace
  identifier or equal payload alone".

A delivery is not a protocol node occurrence. The choreography surface states
that duplicate transport receipts "do not create another node occurrence", so
the occurrence key of [FR-221](FR-221-protocol-node-occurrence.md) does not
reach a delivery, and a second delivery of one send acquires no identity from
it.

## Identity and comparison

- **Identity kind** — **not established**. No requirement states whether a
  delivery identity is authored, derived or a value. FR-051 requires delivery to
  be preserved as "distinct typed subjects" and FR-051-AC-1 requires one send
  "delivered twice" to retain cardinality 2, which requires two deliveries to be
  told apart, yet no requirement supplies the component that tells them apart.
- **Key** — **none is established**. What the prose does establish is
  exclusion, not composition: a transport trace ID substituted for an authored
  identity "refuses before conformance evaluation" (FR-051-AC-4), and an equal
  payload alone establishes nothing (FR-051). Display name, source path, span,
  message text, locale and rendering never participate in any identity in this
  context ([FR-200](../foundation/FR-200-quire-assessment-domain.md)). This
  artifact supplies no key.
- **Key order** — **none is established**, and no total order over deliveries
  exists in the prose. The only ordering sentence is a conformance premise with
  an explicitly partial scope: "A FIFO premise orders messages only within its
  declared channel and key scope" (FR-054-AC-2), and FR-054 states that the
  linker "SHALL NOT infer exactly-once delivery, FIFO order across channels,
  reliable delivery or synchronized clocks". That premise orders messages within
  one channel and one authored key expression; it does not order deliveries, and
  it does not totalize. This artifact supplies no key order.
- **Causal order** — a delivery takes no causal edge of its own. Causal edges
  are the authored partial acyclic order of FR-052, constructed "without
  deriving causal edges from source order or timestamps"; "Reordering equal-time
  events does not add a causal edge absent from the protocol" (FR-052-AC-2); and
  "Arbitrary timestamp order SHALL NOT establish causality" (FR-113). No key
  order, were one authored, could be read as or converted into a causal edge.
- **Absence** — an absent component compares only to absence and is never
  compared to a default or an empty value. A send with zero observed deliveries
  under a `[0,b]` premise has absent deliveries, not a default one.
- **Digest domain** — no key exists here to contain a digest. Where a delivery
  is exchanged beside a digest in provenance, that digest carries the domain it
  was derived under, `ix://agent-ix/quire-specification/FR-201`, and an identity
  crossing domains is unequal regardless of byte equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-224-AC-1 | A delivery reference does not satisfy a comparison expecting a send, a receive, an attempt or an effect. | Analysis |
| FR-224-AC-2 | A repeated delivery does not establish a second business effect. | Analysis |
| FR-224-AC-3 | No requirement text supplies a component tuple that distinguishes two deliveries of one send. | Inspection |
| FR-224-AC-4 | No requirement text supplies a total order over deliveries; the FIFO premise is scoped to one channel and one authored key expression. | Inspection |

## Dependencies

- [FR-051](../../functional/FR-051-preserve-communication-identities.md)
  preserves delivery as a distinct typed subject and states the cardinality
  obligation.
- [FR-054](../../functional/FR-054-bind-channel-premises.md) binds delivery
  cardinality, ordering policy and duplication premises.
- [FR-059](../../functional/FR-059-assess-finite-global-conformance.md) requires
  duplicated and misordered events to be reported.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.
- [FR-210](../protocol/FR-210-protocol-subject-vocabulary.md) records the
  unresolved `receipt` and `delivery` membership question.

## Open gap

**Delivery has neither a uniqueness key nor a total order in the prose, while
three requirements each depend on one.** This artifact records the gap and
supplies neither.

What is required of a delivery:

- FR-051: "the protocol binder SHALL preserve message, send, receive, delivery,
  attempt and effect as distinct typed subjects."
- FR-051-AC-1: "One message sent once, delivered twice and applied as one
  payment effect retains cardinalities 1, 2 and 1 for the respective subjects."
- FR-059: "It SHALL report unexpected, missing, misordered, duplicated and
  wrongly correlated events without repairing them."

What the prose supplies instead:

- FR-054: "The linker SHALL NOT infer exactly-once delivery, FIFO order across
  channels, reliable delivery or synchronized clocks."
- FR-054-AC-2: "A FIFO premise orders messages only within its declared channel
  and key scope."
- FR-051: "The binder SHALL NOT infer a business effect from a send, receive,
  successful transport status, repeated delivery, trace identifier or equal
  payload alone."
- FR-051-AC-4: "A transport trace ID substituted for an authored workflow,
  attempt or effect identity refuses before conformance evaluation."
- FR-113: "Arbitrary timestamp order SHALL NOT establish causality."

Every candidate component a reader might reach for — trace identifier,
timestamp, payload, arrival position — is either explicitly excluded or
explicitly denied causal force. Nothing is admitted in its place.

To close this gap an owner would have to decide, and a requirement would have to
state:

1. Which components constitute a delivery identity, given that the trace
   identifier and the payload are excluded and the delivery is not a node
   occurrence.
2. Whether "duplicated" in FR-059 means a second delivery of one send, which
   FR-054-AC-1 explicitly admits, or a second business effect, which
   FR-051-AC-2 refuses. The two readings require different comparisons.
3. Whether any order over deliveries exists beyond the per-channel, per-key FIFO
   premise, and if one is authored, the explicit statement that it carries no
   causal meaning (FR-052-AC-2, FR-113).
4. Whether `delivery` and `receipt` are one subject or two, which
   [FR-210](../protocol/FR-210-protocol-subject-vocabulary.md) also records as
   unresolved.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
