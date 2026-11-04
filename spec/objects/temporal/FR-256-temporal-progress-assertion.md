---
id: FR-256
title: "Temporal progress assertion"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-252
    type: references
  - target: ix://agent-ix/quire-specification/FR-255
    type: references
  - target: ix://agent-ix/quire-specification/FR-266
    type: references
  - target: ix://agent-ix/quire-specification/FR-289
    type: references
---
# FR-256: Temporal progress assertion

## Description

When an evaluator consumes a validated progress or closure assertion, it SHALL apply that assertion only to the named clock, subjects, decision scope, surrounding execution, interval and history boundary.

FR-094 Inputs name them as "validated progress/completeness/closure assertions
supplied by the observation authority". FR-094 Dependencies record the ownership
split: Agent F "owns the validity, provenance and replay form of observation,
progress, completeness and contradiction assertions. This requirement owns only
their temporal interpretation."

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| assertion_identity_and_revision | String | 1..1 |  |
| named_clock | String | 1..1 |  |
| named_subjects | String | 1..1 |  |
| named_decision_scope | TemporalDecisionScope | 1..1 |  |
| named_surrounding_execution | TemporalDecisionScope | 1..1 |  |
| named_interval | BoundedTemporalInterval | 1..1 |  |
| named_history_boundary | HistoryBoundary | 1..1 |  |
| progress_point | String | 1..1 |  |
| completeness_coverage | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| assertion_identity_and_revision | type not stated in source; multiplicity not stated in source; see Field sources |
| named_clock | type not stated in source; see Field sources |
| named_subjects | type not stated in source; see Field sources |
| named_surrounding_execution | FR-258 scope with `scope_role` surrounding execution; exactly one (FR-094 Description) |
| progress_point | type not stated in source; see Field sources |
| completeness_coverage | type not stated in source; see Field sources |


| Property | Source | Note |
| --- | --- | --- |
| Assertion identity and revision | FR-094 Behavior, FR-094-AC-8 | A "same-binding progress regression or conflicting assertion revision" is a typed contradiction or refusal. |
| Named clock | FR-094 Description, FR-094-AC-4 | Progress for a foreign clock cannot settle the obligation ([FR-252](FR-252-temporal-clock-binding.md)). |
| Named subjects | FR-094 Description, FR-094-AC-4 | Progress for a foreign subject or source set cannot settle the obligation. |
| Named decision scope | FR-094 Description | One of the independent axes ([FR-258](FR-258-temporal-decision-scope.md)). |
| Named surrounding execution | FR-094 Description | A second, independent axis (FR-258). |
| Named interval | FR-094 Description | The inclusive interval it covers ([FR-255](FR-255-bounded-temporal-interval.md)). |
| Named history boundary | FR-094 Description | The past bound it covers ([FR-257](FR-257-history-boundary.md)). |
| Progress point | FR-094 Behavior | The watermark `W`, sample tick or admitted sequence position reached. |
| Completeness coverage | FR-094 Behavior | "completeness covers every required source, subject and valuation through `D`". |

FR-094 states the per-profile advance rules verbatim: "A timestamped watermark
at `W` permits settlement of a deadline `D` only when `W >= D` and completeness
covers every required source, subject and valuation through `D`. A fixed-sample
progress assertion advances sample ticks even when no business predicate is
true. Event-position time shall not advance during silence merely because wall
time passed."

## Identity and comparison

- **Identity kind** — *authored* by the observation authority, not by this
  family: the assertion carries its own identity and revision, which FR-094-AC-8
  makes decisive for detecting a conflicting revision. Its temporal
  *applicability* is a value comparison over the named bindings below.
- **Key** — the assertion identity and revision together with the named clock,
  subjects, decision scope, surrounding execution, interval and history
  boundary. Equality is componentwise over exactly those components and nothing
  else. Display name, source path, span, message text, locale and rendering
  never participate
  ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)). FR-094-AC-4
  makes the binding components decisive: "Progress for a foreign clock, subject,
  source set or binding cannot settle the obligation."
- **Key order** — lexicographic over the key components, for collection
  comparison only. It carries no causal meaning and it is not the monotone
  progress order: FR-094 states that "Progress shall be monotonic within its
  identified binding", and that monotonicity holds over the progress point under
  one equal binding, not over the key order of assertions.
- **Causal order** — none of its own. The order it reports is the admitted
  causal/sequence order of the named clock binding (FR-252), never derived from
  key order, timestamp or ingestion order. FR-113 states that "Arbitrary
  timestamp order SHALL NOT establish causality."
- **Absence** — an absent named component compares only to absence, and an
  assertion whose components do not match the assessment is never applied by
  default. FR-094 requires the evaluator to "classify end-of-file, consumer
  shutdown or a file labelled closed without complete required observations as
  an incomplete input", and FR-113-AC-4 holds that "A mismatched assertion
  definition, scope, clock, source set, or restoration basis remains
  incomplete/refused and cannot settle or restart an assessment."

## Cross-family boundary mapping

FR-289 governs an observation-produced assertion used at an inclusive temporal
deadline. The selected half-open carrier end must be strictly greater than the
inclusive deadline and the assertion must prove same-clock coverage through
that deadline for every required source and subject. An end or half-open
watermark equal to the deadline is insufficient. The temporal evaluator keeps
its inclusive `W >= D` rule only after this mapping has produced an inclusive
progress point.

## Regression and contradiction retain earlier bytes

FR-094 states that "A regressing or same-revision conflicting assertion shall
produce a typed contradiction or refusal and shall not roll back progress,
restamp closure or replace earlier result bytes", and that "A later record
inside an interval previously declared complete is a completeness contradiction
supplied by Agent F's contract. The evaluator shall supersede or invalidate
affected results without rewriting their historical bytes." FR-094 also states
that "The v1 evaluator shall not infer watermark authority or automatically
repair settled history."

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-256-AC-1 | An assertion naming a different clock, subject, source set or binding compares unequal to the assessment's binding and does not settle it. | Test |
| FR-256-AC-2 | A regressing or same-revision conflicting assertion does not replace an earlier progress point, closure stamp or result identity. | Test |
| FR-256-AC-3 | The declared key order over assertions is never cited as the monotone progress order or as a causal relation. | Test |
| FR-256-AC-4 | An end-of-file or closed-labelled input without complete required observations compares as incomplete rather than as a closure assertion. | Test |
| FR-256-AC-5 | An absent named component compares only to absence and is not defaulted to the assessment's own binding. | Test |
| FR-256-AC-6 | Settling an inclusive deadline from a half-open producer window cites the recorded conflict rather than a bridging rule, and no bridging rule is asserted here. | Test |

## Dependencies

- [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  establishes the applicability scoping, monotonicity, per-profile advance rules
  and contradiction handling.
- [FR-252](FR-252-temporal-clock-binding.md) supplies the named clock binding.
- [FR-255](FR-255-bounded-temporal-interval.md) supplies the named interval and
  its inclusive boundary.
- [FR-257](FR-257-history-boundary.md) supplies the named history boundary.
- [FR-266](../observation/FR-266-event-time-window.md) records the half-open
  window convention this artifact settles against, and the conflict with the
  inclusive temporal convention.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
