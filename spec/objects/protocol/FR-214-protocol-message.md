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
---
# FR-214: Protocol message

## Description

When a protocol binder binds a communication, the binder SHALL preserve the message as a typed subject distinct from its sends, receives and deliveries, never inferring a business effect from a send, receive, successful transport status, repeated delivery, trace identifier or equal payload alone.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Typed declaration | FR-050 | Bound "to its exact typed declaration and configuration identity before conformance evaluation." |
| Declaration kind | FR-049-AC-1 | "substituting a payload type, endpoint or scalar identity refuses at its source location." |
| Authored obligation | FR-054-AC-1 | "At-least-once delivery admits a duplicate receipt while retaining the single authored message/effect obligation." |
| Distinct from its events | FR-051 | "the protocol binder SHALL preserve message, send, receive, delivery, attempt and effect as distinct typed subjects." |
| Cardinality independence | FR-051-AC-1 | "One message sent once, delivered twice and applied as one payment effect retains cardinalities 1, 2 and 1 for the respective subjects." |
| Message type in a channel | FR-054 | A channel binds "sender, receiver, message types, ordering policy, delivery cardinality and duplication premises"; see [FR-215](FR-215-protocol-channel.md). |

## Identity and comparison

- **Identity kind** — *authored*. FR-054-AC-1 names "the single authored
  message/effect obligation", and FR-050 binds every message reference to its
  exact typed declaration. It is not derived from a transport record.
- **Key** — **the prose does not fix the key components.** It fixes exclusions:
  a payload type, endpoint or scalar identity in a message position refuses
  (FR-049-AC-1); equal payload alone establishes nothing (FR-051 behavior); and
  "equal scalar labels" do not make two references the same (FR-049-AC-7).
  Equality is componentwise over the key and nothing else; display name, source
  path, span, message text, locale and rendering never participate (FR-035,
  FR-049-AC-7, FR-047). This artifact supplies no key components.
- **Key order** — none. Because the key components are not fixed, no total
  order over the key is stated and none is supplied here.
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
- **Digest domain** — no component named by the prose is a digest. Where a
  message identity is carried beside a digest in provenance, that digest's
  domain is selected under `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-214-AC-1 | One message with one send, two deliveries and one effect yields three distinct subject counts and not one merged subject. | Analysis |
| FR-214-AC-2 | Two messages with equal payload bytes or equal scalar labels are not thereby equal. | Analysis |
| FR-214-AC-3 | A payload type, endpoint or scalar identity supplied in a message position does not resolve. | Analysis |
| FR-214-AC-4 | This artifact declares no key components and no key order for a message, and cites the prose exclusions instead. | Inspection |

## Dependencies

- [FR-051](../../functional/FR-051-preserve-communication-identities.md)
  preserves message, send, receive, delivery, attempt and effect as distinct
  subjects.
- [FR-050](../../functional/FR-050-bind-protocol-instances.md) binds message
  references to exact typed declarations.
- [FR-054](../../functional/FR-054-bind-channel-premises.md) binds message types
  into a channel contract.
- [FR-210](FR-210-protocol-subject-vocabulary.md) records the subject
  vocabulary and its unreconciled lists.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

1. No requirement states what constitutes a message's identity, only what cannot
   substitute for it. Whether the key includes the message type, the channel,
   the sending role instance or the workflow instance is unstated.
2. FR-051 lists `message` as a distinct subject; FR-049-AC-2 and FR-111 do not
   list it at all. FR-210 records that disagreement.
3. No key order over messages can be established until the key components are.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
