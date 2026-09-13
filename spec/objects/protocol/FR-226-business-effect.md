---
id: FR-226
title: "Business effect"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-051
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-058
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-221
    type: references
  - target: ix://agent-ix/quire-specification/FR-225
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
# FR-226: Business effect

## Description

When a business observation is bound to a protocol instance, the protocol binder SHALL preserve the effect as a subject distinct from the message, the send, the receive, the delivery and the attempt, established only through an explicit authored relationship and never inferred from a transport fact ([FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)).

## Properties

- **occurrence** ([FR-221](FR-221-protocol-node-occurrence.md)) — an effect is
  an authored protocol node.
- **establishing attempt** ([FR-225](FR-225-operation-attempt.md)) — the
  concrete attempt the effect is of. The
  [choreography surface](../../../proposals/quire-v1/choreography-surface.md)
  states that `effect ... of AttemptNode` observes "A separately established
  successful business effect of that concrete attempt. A successful transport
  status or an attempt's existence is insufficient."
- **captures** — the immutable captured amounts, identifiers and anchors that
  depend on the effect. FR-055 requires captures to be "immutable for that
  obligation" and FR-055-AC-1 activates a refund obligation "with E1's captured
  amount and identity".

An effect is never inferred. FR-051 states that the binder "SHALL NOT infer a
business effect from a send, receive, successful transport status, repeated
delivery, trace identifier or equal payload alone", and FR-049-AC-2 states that
"a duplicate receipt cannot mint another effect node".

## Identity and comparison

- **Identity kind** — *authored*. The selected observation producer supplies
  the concrete opaque business-effect identity.
- **Key** — one exact [FR-287](FR-287-authority-qualified-runtime-subject-reference.md)
  of kind `business-effect`. Effect node, effect requirement and establishing
  attempt are mandatory typed relationships and are not key components. Reusing
  one effect key with another relationship refuses as an identity contradiction.
- **Key order** — FR-287's lexicographic order for canonical comparison only.
  Population membership and exact arithmetic do not order or identify effects.
- **Causal order** — the authored partial acyclic order of FR-052, carried by
  FR-220, never derived from key order, timestamp or source position
  (FR-052-AC-2, FR-113). The commit-ordering relations of FR-057-AC-3 are
  authored causal relations, not a key order.
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value. An attempt with no established effect has an absent
  effect, which is never compared to an unsuccessful effect or to an empty one
  (FR-051-AC-3).
- **Digest domain** — FR-287 retains the authority digest and its FR-201 domain.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-226-AC-1 | An effect reference does not satisfy a comparison expecting a send, a receive, a delivery or an attempt. | Analysis |
| FR-226-AC-2 | A successful transport status, a repeated delivery, a trace identifier and an equal payload each fail to establish an effect. | Analysis |
| FR-226-AC-3 | Re-observing one complete effect key with equal binding facts is replay of one effect; a second exact key validly bound to the same singleton effect occurrence is retained as the duplicate-business-effect outcome. | Analysis |
| FR-226-AC-4 | Effect-key order is canonical comparison only and establishes no causal, commit or recovery order. | Analysis |
| FR-226-AC-5 | The establishing attempt is a mandatory relationship, not an effect-key component; reusing one effect key with another attempt refuses rather than creating a second effect. | Analysis |

## Dependencies

- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)
  preserves effect as a distinct typed subject and forbids inference.
- [FR-058](../../functional/protocol/FR-058-preserve-retry-and-partial-recovery.md)
  requires each effect identity to be preserved and names the duplicate effect.
- [FR-057](../../functional/protocol/FR-057-enforce-commit-recovery.md) supplies the
  commit-ordering relations an effect participates in.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) defines the
  concrete effect key and non-causal comparison order.

## Status

Draft. Specification issue #28 fixed effect identity, replay versus duplicate
effect disposition and comparison order. Executable cross-repository evidence
remains planned.
