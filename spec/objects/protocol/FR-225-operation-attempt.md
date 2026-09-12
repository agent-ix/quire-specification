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
---
# FR-225: Operation attempt

## Description

When a communication or business observation is bound to a protocol instance, the protocol binder SHALL preserve the attempt as a subject distinct from the message, the send, the receive, the delivery and the effect ([FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md)).

## Properties

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
typed subjects "message, send, receive, delivery, attempt and effect" — and it
derives its key from the node occurrence of
[FR-221](FR-221-protocol-node-occurrence.md).
[FR-235](FR-235-compensation-attempt.md) holds the **compensation** attempt of
FR-058 — an attempt against a registered compensation obligation
([FR-233](FR-233-compensation-registration.md)) carrying a per-attempt outcome
([FR-236](FR-236-compensation-attempt-outcome.md)) — and it declares its key
**not established**, because FR-058 supplies no components for it.

The two identity treatments are not compatible, and no requirement states
whether a compensation attempt is a protocol node occurrence at all. This
artifact does not extend its derived key to FR-235, and FR-235 does not adopt
one from here.

The worked example FR-058-AC-1, "A failed refund attempt followed by one
successful retry retains both attempts and one successful refund effect", is a
*refund* attempt and therefore a compensation attempt: it is owned by
[FR-235](FR-235-compensation-attempt.md). The retry-discrimination analysis in
the **Open gap** below is retained here because the node-occurrence key is what
makes the failure demonstrable, but it is stated about the FR-235 subject and
FR-235 records the same unresolved question against its own unestablished key.

## Identity and comparison

- **Identity kind** — *derived*, through the node occurrence it is.
- **Key** — (`static declaration path`, `workflow instance identity`,
  `iteration identity`), as held by FR-221. Equality is componentwise over that
  key and nothing else. Display name, source path, span, message text, locale
  and rendering never participate, and a transport trace ID substituted for an
  authored attempt identity "refuses before conformance evaluation"
  (FR-051-AC-4). This key does not distinguish two attempts made against one
  static attempt node outside a bounded iteration; see **Open gap**.
- **Key order** — none is established. No requirement fixes a total order over
  attempts. A retry bound is a count, not an order: FR-058 states that the
  evaluator "SHALL refuse attempts beyond the authored finite retry bound" and
  FR-049-AC-6 states that "Retry bounds count the initial attempt". A count
  admits no comparison between two attempts.
- **Causal order** — the authored partial acyclic order of FR-052, carried by
  FR-220, never derived from key order, timestamp or source position
  (FR-052-AC-2, FR-113). The "followed by" of FR-058-AC-1 is an authored
  ordering relation, not a key order.
- **Absence** — an absent key component compares only to absence, never to a
  default or empty value. An attempt outside every bounded iteration has an
  absent iteration identity, and that absence is never compared to ordinal zero.
- **Digest domain** — no component of this key is a digest; a digest carried in
  provenance carries the domain it was derived under,
  `ix://agent-ix/quire-specification/FR-201`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-225-AC-1 | An attempt reference does not satisfy a comparison expecting a send, a receive, a delivery or an effect. | Analysis |
| FR-225-AC-2 | An attempt with no established effect remains an attempt and activates no effect-dependent compensation. | Analysis |
| FR-225-AC-3 | A timed-out attempt, a failed attempt and a duplicate effect remain three distinct outcomes. | Analysis |
| FR-225-AC-4 | No requirement text supplies a component that distinguishes two attempts of one static attempt node within one workflow instance and one iteration identity. | Inspection |

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

## Open gap

**A retried attempt is required to be retained as a second subject, and the
prose supplies no component that separates it from the first.**

- FR-058: "the conformance evaluator SHALL preserve each attempt and effect
  identity and evaluate only the authored retry and recovery relations."
- FR-058-AC-1: "A failed refund attempt followed by one successful retry retains
  both attempts and one successful refund effect."

The only identity components in the prose are the node occurrence components,
and those are constant across a retry unless the retry is authored inside a
bounded iteration, which no requirement requires it to be. No requirement states
whether a retry is a new iteration ordinal of the attempt node, a distinct
occurrence under some other component, or an outcome recorded against one
occurrence.

To close this gap an owner would have to decide, and a requirement would have to
state:

1. Whether a retry of an attempt is an iteration occurrence of the attempt node,
   and if so, which construct supplies its ordinal.
2. If a retry is not an iteration occurrence, which component distinguishes
   attempt 1 from attempt 2, given that the trace identifier (FR-051-AC-4) and
   the timestamp (FR-113) are excluded.
3. Whether any order over attempts exists beyond the authored causal edges, and
   if one is authored, the explicit statement that it carries no causal meaning.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
