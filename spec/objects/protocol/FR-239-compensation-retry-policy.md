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
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
# FR-239: Compensation retry policy

## Description

When compensation attempts are evaluated, the conformance evaluator SHALL refuse attempts beyond the authored finite retry bound (FR-058).

When a compensation template is checked, the checker SHALL admit as its attempt
type only a record type or an event type and SHALL refuse every other type
written after `attempts N of`. Under the Complete-V1 profile (`quire.value.complete/v1`) the
source grammar admits any `type-ref` in that position, so the refusal belongs to
the checker.

FR-058 adds that the evaluator evaluates "only the authored retry and recovery
relations".

FR-056 requires each registration to be bound to its retry policy, and FR-058
Inputs names "bounded retry policy" as an input to the evaluation.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| authored_finite_retry_bound | String | 1..1 |  |
| owning_registration | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| authored_finite_retry_bound | type not stated in source; see Field sources |
| owning_registration | type not stated in source; multiplicity not stated in source; see Field sources |


| Property | Source | Note |
| --- | --- | --- |
| Authored finite retry bound | FR-058 Behavior | "It SHALL refuse attempts beyond the authored finite retry bound." FR-049-AC-6 states what it counts from: "Retry bounds count the initial attempt." |
| Owning registration | FR-056 Description | Each registration is bound to its retry policy ([FR-233](FR-233-compensation-registration.md)). |

FR-058-AC-4 states that "Unknown or missing population membership and attempts
beyond the retry bound cannot produce recovery success", and FR-058-AC-1 that "A
failed refund attempt followed by one successful retry retains both attempts".
Refusing an attempt beyond the bound does not discard the attempts already
retained.

The count is the cardinality of distinct admitted compensation-attempt FR-287
keys bound to one registration. It includes the initial attempt and all admitted
failed, timed-out and successful attempts. Replay or repeated receipt of one key
counts once. An over-bound candidate is refused before retention and does not
increment or otherwise mutate the prior count. Retry order comes only from the
exact authored predecessor/successor relation; malformed or absent evidence is
never repaired from key, timestamp, table or arrival order.

## Identity and comparison

- **Identity kind** — *value*. A retry policy has no identity of its own; it is
  compared componentwise and belongs to exactly one registration.
- **Key** — the authored finite retry bound, compared componentwise and nothing
  else. Display name, source path, span, message text, locale and rendering never
  participate (FR-009, FR-047).
- **Key order** — the exact numeric order of the single finite-bound component,
  used only for canonical collection comparison. A larger bound has no
  precedence, preference, strength or causal meaning.
- **Causal order** — none. A retry policy is a bound on a count of attempts, not
  a node in the authored partial order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md). The order of
  the attempts it bounds is authored and is never derived from key order,
  timestamp or source position.
- **Absence** — an absent retry bound compares only to absence. FR-058 admits
  only an authored finite bound, so absence is never compared to zero, to an
  unbounded policy or to an implicit default.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-239-AC-1 | An attempt beyond the authored finite retry bound is refused and cannot produce recovery success. | Analysis |
| FR-239-AC-2 | Refusing an attempt beyond the bound leaves the previously retained attempts unchanged. | Analysis |
| FR-239-AC-3 | An absent retry bound compares only to absence and is never defaulted to zero or to an unbounded policy. | Analysis |
| FR-239-AC-4 | Canonical collection comparison uses the bound's exact numeric order, and a larger bound is not read as a later, preferred or stronger policy. | Inspection |
| FR-239-AC-5 | The attempt count is the number of distinct admitted compensation-attempt FR-287 keys under one registration, including the initial attempt and every admitted outcome; replay or repeated receipt counts once. | Test (TC-058) |
| FR-239-AC-6 | A candidate beyond the maximum refuses before retention and leaves the previous count unchanged; repeating it neither increments the count nor changes the refusal. | Test (TC-058) |
| FR-239-AC-7 | Only an exact authored predecessor/successor relation orders retries; missing observation/progress remains incomplete, while a missing, foreign, conflicting or ambiguous relation refuses. | Test (TC-058) |
| FR-239-AC-8 | The checker admits a record type or an event type as the attempt type after `attempts N of` and refuses every other type with `invalid_model_binding`/`malformed-declaration` at the type, retaining the compensation, the member `attempts` and the type as written, and compares no retry binder with a refused type: `attempts 3 of M::RefundAttempt`, a record type, checks, while `attempts 3 of Set<M::Total>[0, 3]`, `attempts 2 of Integer` and `attempts 3 of M::Mode`, a variant type, are each refused once. | Test (TC-380) |
| FR-239-AC-9 | Under the Complete-V1 profile (`quire.value.complete/v1`) the source grammar parses any `type-ref` after `attempts N of`, including `Set<M::Total>[0, 3]`, `Integer` and a model-owned name, with no diagnostic. | Test (TC-380) |

## Dependencies

- [FR-058](../../functional/protocol/FR-058-preserve-retry-and-partial-recovery.md)
  establishes the bounded retry policy and the refusal rule.
- [FR-056](../../functional/protocol/FR-056-register-compensation.md) binds each
  registration to its retry policy.
- [FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md) states
  that retry bounds count the initial attempt.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-287](FR-287-authority-qualified-runtime-subject-reference.md) supplies the
  distinct admitted compensation-attempt key.

## Status

Proposed closure for specification issue #31. Spec Language publishes the
finite maximum, distinct attempt binding requirements and authored retry
relation. Protocol's compensation algebra is not implemented, so executable
evidence remains planned under TC-058.
