---
id: FR-235
title: "Compensation attempt"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-058
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-051
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-236
    type: references
  - target: ix://agent-ix/quire-specification/FR-225
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
# FR-235: Compensation attempt

## Description

When multiple compensation observations relate to one obligation, the conformance evaluator SHALL preserve each attempt and effect identity and evaluate only the authored retry and recovery relations (FR-058).

FR-058 states that the evaluator "SHALL distinguish a repeated receipt, retried
attempt and duplicate business effect", and FR-051 requires "message, send,
receive, delivery, attempt and effect as distinct typed subjects".

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Attempt identity | FR-058 Description, FR-051 | Preserved per attempt and held distinct from the effect identity. |
| Owning registration | FR-058 Inputs | The registered compensation obligation the attempt relates to ([FR-233](FR-233-compensation-registration.md)). |
| Related effect identity | FR-058 Description, FR-051 | Where an effect was established, retained as a distinct subject. |
| Attempt outcome | FR-058 Outputs | [FR-236](FR-236-compensation-attempt-outcome.md). |

FR-058-AC-1 states that "A failed refund attempt followed by one successful retry
retains both attempts and one successful refund effect." That refund attempt is
a compensation attempt, so **this artifact owns the FR-058-AC-1 example**.

### Relation to FR-225

An attempt is typed twice in this layer. This artifact holds the **compensation**
attempt of FR-058: an attempt against a registered compensation obligation
([FR-233](FR-233-compensation-registration.md)) carrying a per-attempt outcome
([FR-236](FR-236-compensation-attempt-outcome.md)).
[FR-225](FR-225-operation-attempt.md) holds the **forward** operation attempt of
FR-051, one of the six distinct typed subjects "message, send, receive,
delivery, attempt and effect".

The kinds are deliberately different: this object uses
`compensation-attempt`; FR-225 uses `operation-attempt`. Neither identity can
satisfy the other's binding.

## Identity and comparison

- **Identity kind** — *authored*. The selected observation producer supplies one
  concrete opaque identity for each compensation attempt.
- **Key** — one exact [FR-287](FR-287-authority-qualified-runtime-subject-reference.md)
  of kind `compensation-attempt`. Registration, operation and retry relation are
  mandatory binding relationships rather than key components. Reusing one key
  with different binding facts refuses as an identity contradiction.
- **Key order** — FR-287's lexicographic order for canonical comparison only.
  Attempt count is bounded by the authored finite retry maximum, but the maximum
  and earlier/later binders supply neither identity nor order.
- **Causal order** — the authored partial acyclic order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md). A retried
  attempt follows the attempt it retries, and an attempt is causally after its
  registration and before any effect it establishes. That order is authored and
  never derived from key order, timestamp or source position.
- **Absence** — an absent component compares only to absence. An attempt with no
  established effect has an absent related effect identity; FR-051-AC-3 states
  that it "remains an attempt and cannot activate effect-dependent compensation",
  and that absence is never compared to a default or empty effect identity.
- **Digest domain** — FR-287 retains the authority digest and its FR-201 domain.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-235-AC-1 | A failed attempt and its successful retry are retained as two attempts rather than merged. | Analysis |
| FR-235-AC-2 | An attempt identity never compares equal to an effect identity. | Analysis |
| FR-235-AC-3 | A transport trace identifier, successful transport status, repeated delivery or equal payload alone does not establish an attempt or effect identity. | Analysis |
| FR-235-AC-4 | An attempt with no established effect retains an absent effect identity that compares only to absence. | Analysis |
| FR-235-AC-5 | Compensation-attempt equality and canonical order use only the FR-287 key; reusing one key for another registration, operation or retry relation refuses. | Analysis |

## Dependencies

- [FR-058](../../functional/protocol/FR-058-preserve-retry-and-partial-recovery.md).
- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md).
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-225](FR-225-operation-attempt.md) types the forward operation attempt this
  one is held distinct from.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) defines the
  concrete compensation-attempt key and non-causal comparison order.

## Status

Draft. Specification issue #28 fixed the concrete compensation-attempt key,
retry relationship boundary and comparison order. Executable cross-repository
evidence remains planned.
