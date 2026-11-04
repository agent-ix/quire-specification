---
id: FR-225
title: "Operation attempt"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-051
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-058
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-221
    type: references
  - target: ix://agent-ix/quire-specification/FR-235
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
# FR-225: Operation attempt

## Description

When a communication or business observation is bound to a protocol instance, the protocol binder SHALL preserve the attempt as a subject distinct from the message, the send, the receive, the delivery and the effect ([FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)).

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| key | String | 1..1 | identity |
| occurrence | ProtocolNodeOccurrence | 1..1 |  |
| role | String | 1..1 |  |
| operation | String | 1..1 |  |
| outcome | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| key | type not stated in source; see Field sources |
| role | type not stated in source; see Field sources |
| operation | type not stated in source; see Field sources |
| outcome | type not stated in source; see Field sources |


- **occurrence** ([FR-221](FR-221-protocol-node-occurrence.md)) — an attempt is
  an authored protocol node. The
  [choreography surface](../../../proposals/quire-v1/choreography-surface.md)
  states that `attempt ... by R on M::T::op contracts [...]` observes "One
  operation attempt under the named role and operation".
- **role** — the named role performing the attempt, bound to its exact typed
  declaration by FR-050.
- **operation** — the named operation. The listed state pre- and postcondition
  declarations must match this operation and its invocation anchors
  (choreography surface).
- **outcome** — failed, timed out, pending or successful. FR-058 requires the
  evaluator to "distinguish a repeated receipt, retried attempt and duplicate
  business effect", and FR-058-AC-2 keeps "A timed-out attempt, failed attempt
  and duplicate effect" as three distinct outcomes.

An attempt establishes no effect: "A payment attempt with no established effect
remains an attempt and cannot activate effect-dependent compensation"
(FR-051-AC-3).

### Relation to FR-235

An attempt is typed twice in this layer. This artifact holds the **forward**
operation attempt of FR-051 — the attempt node that is one of the six distinct
typed subjects "message, send, receive, delivery, attempt and effect". It binds
to, but is not identified by, the node occurrence of
[FR-221](FR-221-protocol-node-occurrence.md).
[FR-235](FR-235-compensation-attempt.md) holds the **compensation** attempt of
FR-058 — an attempt against a registered compensation obligation
([FR-233](FR-233-compensation-registration.md)) carrying a per-attempt outcome
([FR-236](FR-236-compensation-attempt-outcome.md)).

The kinds are deliberately different: this object uses `operation-attempt` and
FR-235 uses `compensation-attempt`. Neither can satisfy the other's binding.

The worked example FR-058-AC-1, "A failed refund attempt followed by one
successful retry retains both attempts and one successful refund effect", is a
*refund* attempt and therefore a compensation attempt owned by FR-235.

## Identity and comparison

- **Identity kind** — *authored*. The observation producer supplies one concrete
  opaque identity for each forward operation attempt.
- **Key** — one exact [FR-287](FR-287-authority-qualified-runtime-subject-reference.md)
  of kind `operation-attempt`. Node occurrence, role and operation are mandatory
  binding relationships rather than key components. Reusing one key with
  different relationships refuses as an identity contradiction.
- **Key order** — FR-287's lexicographic order for canonical comparison only. A
  retry bound is a count, not an identity or order; retry sequence is an
  explicit authored relationship between two different attempt keys.
- **Causal order** — the authored partial acyclic order of FR-052, carried by
  FR-220, never derived from key order, timestamp or source position
  (FR-052-AC-2, FR-113). The "followed by" of FR-058-AC-1 is an authored
  ordering relation, not a key order.
- **Absence** — every FR-287 component is required and never defaulted. An
  attempt without an effect retains an absent effect relationship, not an empty
  effect identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-225-AC-1 | An attempt reference does not satisfy a comparison expecting a send, a receive, a delivery or an effect. | Analysis |
| FR-225-AC-2 | An attempt with no established effect remains an attempt and activates no effect-dependent compensation. | Analysis |
| FR-225-AC-3 | A timed-out attempt, a failed attempt and a duplicate effect remain three distinct outcomes. | Analysis |
| FR-225-AC-4 | Two attempts at one node are distinct only when their complete FR-287 keys differ; replay of one key remains one attempt, and reusing it with other node/role/operation facts refuses. | Analysis |

## Dependencies

- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)
  preserves attempt as a distinct typed subject.
- [FR-058](../../functional/protocol/FR-058-preserve-retry-and-partial-recovery.md)
  requires each attempt identity to be preserved across retry.
- [FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md) states
  that retry bounds count the initial attempt.
- [FR-235](FR-235-compensation-attempt.md) types the compensation attempt and
  owns the FR-058-AC-1 refund example.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) defines the
  concrete forward-attempt key and non-causal comparison order.

## Status

Draft. Specification issue #28 fixed the concrete attempt key, retry relation
boundary and comparison order. Executable cross-repository evidence remains
planned.
