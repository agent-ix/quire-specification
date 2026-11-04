---
id: FR-214
title: "Protocol message"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-051
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-050
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
# FR-214: Protocol message

## Description

When a protocol binder binds a communication, the binder SHALL preserve the message as a typed subject distinct from its sends, receives and deliveries, never inferring a business effect from a send, receive, successful transport status, repeated delivery, trace identifier or equal payload alone.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| key | String | 1..1 | identity |
| typed_declaration | String | 1..1 |  |
| declaration_kind | String | 1..1 |  |
| authored_obligation | String | 1..1 |  |
| distinct_from_its_events | String | 1..1 |  |
| cardinality_independence | String | 1..1 |  |
| message_type_in_a_channel | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| key | type not stated in source; see Field sources |
| typed_declaration | type not stated in source; see Field sources |
| declaration_kind | type not stated in source; multiplicity not stated in source; see Field sources |
| authored_obligation | type not stated in source; see Field sources |
| distinct_from_its_events | type not stated in source; see Field sources |
| cardinality_independence | type not stated in source; see Field sources |
| message_type_in_a_channel | type not stated in source; multiplicity not stated in source; see Field sources |


| Property | Source | Note |
| --- | --- | --- |
| Typed declaration | FR-050 | Bound "to its exact typed declaration and configuration identity before conformance evaluation." |
| Declaration kind | FR-049-AC-1 | "substituting a payload type, endpoint or scalar identity refuses at its source location." |
| Authored obligation | FR-054-AC-1 | "At-least-once delivery admits a duplicate receipt while retaining the single authored message/effect obligation." |
| Distinct from its events | FR-051 | "the protocol binder SHALL preserve message, send, receive, delivery, attempt and effect as distinct typed subjects." |
| Cardinality independence | FR-051-AC-1 | "One message sent once, delivered twice and applied as one payment effect retains cardinalities 1, 2 and 1 for the respective subjects." |
| Message type in a channel | FR-054 | A channel binds "sender, receiver, message type, ordering policy and delivery policy"; see [FR-215](FR-215-protocol-channel.md). |

## Identity and comparison

- **Identity kind** — *authored*. FR-054-AC-1 names "the single authored
  message/effect obligation", and FR-050 binds every message reference to its
  exact typed declaration. It is not derived from a transport record.
- **Key** — one exact [FR-287](FR-287-authority-qualified-runtime-subject-reference.md)
  of kind `message`. The authored message obligation, workflow and channel are
  separately validated relationships. A static binding-requirement identity,
  payload type, endpoint, scalar, trace or equal payload is not the concrete
  message identity.
- **Key order** — FR-287's lexicographic order for canonical collection
  comparison only. It establishes neither send, receive nor delivery order.
- **Causal order** — a message's send and receive are distinct protocol events
  ordered only by the authored partial acyclic graph of FR-052; FR-052-AC-2
  states that "Reordering equal-time events does not add a causal edge absent
  from the protocol". Order is never taken from key order, timestamp or source
  position; delivery order is governed by a channel's ordering premise, which
  "orders messages only within its declared channel and key scope"
  (FR-054-AC-2) and is a premise, not a causal edge.
- **Absence** — an absent key component compares only to absence and is never
  defaulted to an empty payload, an empty label or an implicit option-none
  (FR-035-AC-8).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-214-AC-1 | One message with one send, two deliveries and one effect yields three distinct subject counts and not one merged subject. | Analysis |
| FR-214-AC-2 | Two messages with equal payload bytes or equal scalar labels are not thereby equal. | Analysis |
| FR-214-AC-3 | A payload type, endpoint or scalar identity supplied in a message position does not resolve. | Analysis |
| FR-214-AC-4 | Message equality and canonical collection order use only the FR-287 key; reusing one key for another obligation, workflow or channel refuses as an identity contradiction. | Analysis |

## Dependencies

- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)
  preserves message, send, receive, delivery, attempt and effect as distinct
  subjects.
- [FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md) binds message
  references to exact typed declarations.
- [FR-054](../../functional/protocol/FR-054-bind-channel-premises.md) binds message types
  into a channel contract.
- [FR-210](FR-210-protocol-subject-vocabulary.md) records the complete subject
  vocabulary and contextual subset rule.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) defines the
  concrete message key and non-causal comparison order.

## Resolved vocabulary membership

FR-051 lists `message` as a distinct subject. FR-049-AC-2 and FR-111 name only
the subsets they consume; their shorter lists are not exhaustive vocabularies
and do not remove `message` from FR-210.

## Status

Accepted complete-V1 message contract.
