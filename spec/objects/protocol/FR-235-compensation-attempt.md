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
delivery, attempt and effect", and it derives its key from the node occurrence of
[FR-221](FR-221-protocol-node-occurrence.md).

The two identity treatments differ and are not reconciled: FR-225 declares a
derived node-occurrence key, and this artifact declares its key **not
established**, because FR-058 supplies no components for a compensation attempt
and no requirement states that a compensation attempt is a protocol node
occurrence. No key is adopted from FR-225 here, and neither artifact is merged
into the other.

FR-225 retains a retry-discrimination analysis in its own open gap, because its
node-occurrence key is what makes the failure demonstrable; the subject of that
analysis is the FR-058-AC-1 refund attempt owned here, and the same unresolved
question is recorded against this artifact's unestablished key in the **Open
gap** below.

## Identity and comparison

- **Identity kind** — not established by the prose. FR-058 requires the evaluator
  to "preserve each attempt and effect identity", and FR-051 keeps attempt and
  effect distinct, but no requirement states whether an attempt identity is
  authored in native source or derived from components. The gap is recorded below
  rather than filled.
- **Key** — not established. The prose states only what the key is *not*: FR-051
  forbids inferring "a business effect from a send, receive, successful transport
  status, repeated delivery, trace identifier or equal payload alone", and
  FR-051-AC-4 refuses "A transport trace ID substituted for an authored workflow,
  attempt or effect identity". Display name, source path, span, message text,
  locale and rendering never participate (FR-009, FR-047). Whatever the key
  components are, an attempt identity never compares equal to an effect identity:
  FR-058-AC-2 holds "A timed-out attempt, failed attempt and duplicate effect" as
  "three distinct outcomes".
- **Key order** — none. The key components are not fixed by the prose, so no
  order over them can be declared. Attempt count is bounded by the authored
  finite retry bound ([FR-239](FR-239-compensation-retry-policy.md)), but a bound
  is not an order.
- **Causal order** — the authored partial acyclic order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md). A retried
  attempt follows the attempt it retries, and an attempt is causally after its
  registration and before any effect it establishes. That order is authored and
  never derived from key order, timestamp or source position.
- **Absence** — an absent component compares only to absence. An attempt with no
  established effect has an absent related effect identity; FR-051-AC-3 states
  that it "remains an attempt and cannot activate effect-dependent compensation",
  and that absence is never compared to a default or empty effect identity.
- **Digest domain** — no key component of this object is established, so none is
  a digest here. Where an attempt or effect identity is carried as a digest, it
  retains the domain selected under
  [FR-201](../foundation/FR-201-canonical-identity-domain.md).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-235-AC-1 | A failed attempt and its successful retry are retained as two attempts rather than merged. | Analysis |
| FR-235-AC-2 | An attempt identity never compares equal to an effect identity. | Analysis |
| FR-235-AC-3 | A transport trace identifier, successful transport status, repeated delivery or equal payload alone does not establish an attempt or effect identity. | Analysis |
| FR-235-AC-4 | An attempt with no established effect retains an absent effect identity that compares only to absence. | Analysis |

## Dependencies

- [FR-058](../../functional/protocol/FR-058-preserve-retry-and-partial-recovery.md).
- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md).
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-225](FR-225-operation-attempt.md) types the forward operation attempt this
  one is held distinct from.

## Open gap

The prose establishes that an attempt has an identity, that it is distinct from
an effect identity, and several things that cannot serve as it. It does not
establish:

- **Identity kind** — whether an attempt identity is authored or derived.
- **Key** — which components constitute it. FR-058 requires distinguishing "a
  repeated receipt, retried attempt and duplicate business effect" without
  stating the components that separate them.
- **Key order** — no order follows, since the components are unfixed.

FR-058-AC-1 makes this concrete for the example this artifact owns: a failed
refund attempt and its successful retry must both be retained, and no stated
component separates them. [FR-225](FR-225-operation-attempt.md) records the same
failure from the other side, where the node-occurrence key is constant across a
retry.

No key, order or identity kind is invented here.

## Status

Draft. This artifact records existing meaning structurally. It establishes no new
obligation and claims no implementation coverage.
