---
id: FR-239
title: "Compensation retry policy"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-058
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-056
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-049
    type: specifies
---
# FR-239: Compensation retry policy

## Description

When compensation attempts are evaluated, the conformance evaluator SHALL refuse attempts beyond the authored finite retry bound (FR-058).

FR-058 adds that the evaluator evaluates "only the authored retry and recovery
relations".

FR-056 requires each registration to be bound to its retry policy, and FR-058
Inputs names "bounded retry policy" as an input to the evaluation.

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Authored finite retry bound | FR-058 Behavior | "It SHALL refuse attempts beyond the authored finite retry bound." FR-049-AC-6 states what it counts from: "Retry bounds count the initial attempt." |
| Owning registration | FR-056 Description | Each registration is bound to its retry policy ([FR-233](FR-233-compensation-registration.md)). |

FR-058-AC-4 states that "Unknown or missing population membership and attempts
beyond the retry bound cannot produce recovery success", and FR-058-AC-1 that "A
failed refund attempt followed by one successful retry retains both attempts".
Refusing an attempt beyond the bound does not discard the attempts already
retained.

## Identity and comparison

- **Identity kind** — *value*. A retry policy has no identity of its own; it is
  compared componentwise and belongs to exactly one registration.
- **Key** — the authored finite retry bound, compared componentwise and nothing
  else. Display name, source path, span, message text, locale and rendering never
  participate (FR-009, FR-047).
- **Key order** — none is declared. The bound is a number, but no requirement
  establishes an order over retry policies, and the numeric order of two bounds
  carries no precedence, no preference and no causal meaning. Collection
  comparison must not read a larger bound as a later or stronger policy.
- **Causal order** — none. A retry policy is a bound on a count of attempts, not
  a node in the authored partial order of
  [FR-052](../../functional/FR-052-represent-bounded-control.md). The order of
  the attempts it bounds is authored and is never derived from key order,
  timestamp or source position.
- **Absence** — an absent retry bound compares only to absence. FR-058 admits
  only an authored finite bound, so absence is never compared to zero, to an
  unbounded policy or to an implicit default.
- **Digest domain** — not applicable; the key is a finite bound, not a digest.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-239-AC-1 | An attempt beyond the authored finite retry bound is refused and cannot produce recovery success. | Analysis |
| FR-239-AC-2 | Refusing an attempt beyond the bound leaves the previously retained attempts unchanged. | Analysis |
| FR-239-AC-3 | An absent retry bound compares only to absence and is never defaulted to zero or to an unbounded policy. | Analysis |
| FR-239-AC-4 | No key order over retry policies is declared, and a larger bound is not read as a later or stronger policy. | Inspection |

## Dependencies

- [FR-058](../../functional/FR-058-preserve-retry-and-partial-recovery.md)
  establishes the bounded retry policy and the refusal rule.
- [FR-056](../../functional/FR-056-register-compensation.md) binds each
  registration to its retry policy.
- [FR-049](../../functional/FR-049-bind-native-choreography-syntax.md) states
  that retry bounds count the initial attempt.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

[FR-049](../../functional/FR-049-bind-native-choreography-syntax.md)-AC-6 states
that "Retry bounds count the initial attempt", so the bound counts attempts
including the first and not retries after it.
[FR-225](FR-225-operation-attempt.md) quotes the same sentence. That part of the
gap is closed by requirement text and is not restated as open here.

What remains unstated is narrower. FR-058 requires the evaluator to "distinguish
a repeated receipt, retried attempt and duplicate business effect" and to refuse
"attempts beyond the authored finite retry bound", but no requirement states
whether a refused over-bound attempt is itself counted against the bound, nor
whether a repeated receipt of one attempt counts as one attempt or as many.
Neither is supplied here.

## Status

Draft. This artifact records existing meaning structurally. It establishes no new
obligation and claims no implementation coverage.
