---
id: FR-360
title: "Infinite-trace result disposition vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-161
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-241
    type: references
  - target: ix://agent-ix/quire-specification/FR-242
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-391
    type: references
  - target: ix://agent-ix/quire-specification/FR-399
    type: references
  - target: ix://agent-ix/quire-specification/FR-174
    type: references
  - target: ix://agent-ix/quire-specification/FR-175
    type: references
---
# FR-360: Infinite-trace result disposition vocabulary

## Description

When an infinite-trace, state-graph or model-subject hyperproperty analysis
result is reported, the strict reader SHALL
match its label as an exact string from this closed vocabulary and SHALL read
each label only through the exact assessment-execution ([FR-241](../protocol/FR-241-assessment-execution-disposition.md)),
truth ([FR-242](../protocol/FR-242-truth-disposition.md)) and settlement-basis
([FR-243](../protocol/FR-243-settlement-basis.md)) combination named below.
FR-161 states the vocabulary directly: "Proved, refuted, inconclusive,
unsupported or failed result with premises, witness/counterexample and
provenance."

## Scope

The vocabulary covers infinite-trace results, state-graph results
([FR-391](../../functional/analysis/FR-391-settle-state-graph-claims.md)) and
hyperproperty and step-relation results over a model subject
([FR-399](../../functional/analysis/FR-399-settle-hyperproperty-results.md)). It also covers refinement results
([FR-379](../../functional/analysis/FR-379-report-and-replay-refinement-counterexamples.md),
[FR-177](../../functional/protocol/FR-177-check-protocol-refinement.md)), reduced-run results
([FR-385](../../functional/analysis/FR-385-report-reduced-verdicts.md)) and composition and guarantee
results ([FR-488](../../functional/ir-proof/FR-488-attach-a-temporal-guarantee-to-implementation-rust.md),
[FR-489](../../functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md)).
It also covers local projection and realizability results
([FR-174](../../functional/protocol/FR-174-project-global-protocols.md),
[FR-175](../../functional/protocol/FR-175-analyze-protocol-realizability.md)).
Each label keeps the exact FR-241, FR-242 and FR-243 combination below for
every one of them.

## Values

| Value | Description |
| --- | --- |
| `proved` | Execution `completed` (FR-241); truth `satisfied` (FR-242); settlement basis `closed-scope` or `decisive-witness` (FR-243). The provider returns replayable evidence for every admitted infinite trace (FR-161-AC-1). |
| `refuted` | Execution `completed` (FR-241); truth `violated` (FR-242); settlement basis `closed-scope` or `decisive-counterexample` (FR-243). A finite prefix plus lasso or provider-declared complete witness replays to an admitted infinite trace (FR-161). |
| `inconclusive` | Execution `completed` (FR-241); truth `pending` (FR-242); settlement basis `unsettled` (FR-243, restated by [FR-094-AC-7](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)). A finite-prefix monitor returns this label rather than `proved` for an unresolved liveness property (FR-161-AC-2). |
| `unsupported` | Execution `unsupported` (FR-241); truth `unavailable` (FR-242); settlement basis `unavailable` (FR-243). Returned for an unsupported operator, a missing fairness premise ([FR-362](../../functional/temporal/FR-362-apply-fairness-constraints.md): a fairness constraint over a trace that carries no enabledness), a profile the subject does not admit ([FR-361](../../functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md)), or no candidate that decides the item ([FR-368](../../functional/temporal/FR-368-negotiate-temporal-model-check-items.md)). |
| `failed` | Execution `failed` or `resource-incomplete` (FR-241); truth `unavailable` (FR-242); settlement basis `unavailable` (FR-243). `failed` covers a selected liveness provider that fails for this exact claim; `resource-incomplete` covers a caller-configurable run limit that stopped the analysis before it completed its method, such as the automaton-state limit of [FR-367](../../functional/temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md). Neither invokes a fallback nor changes an independent sibling result (FR-094-AC-9). |

`unsupported` and `failed` project to the same truth and settlement-basis pair,
and `failed` itself carries two FR-241 execution dispositions. Every one of
these non-conclusive outcomes stays distinct through the FR-241
execution-disposition axis, which this vocabulary never substitutes for or
collapses (FR-161-AC-3).

A verdict over a model subject maps onto these labels by the verdict table of
[FR-363](../../functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md).
A completed analysis that found no counterexample up to a depth is
`inconclusive`, never `proved`.

## Identity and comparison

- **Identity kind** — *value*. A result-disposition label has no identity of
  its own and is compared componentwise as an exact string.
- **Key** — the label itself, matched exactly. Display name, source path, span,
  message text, locale and rendering never participate
  ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)).
- **Key order** — lexicographic over the label, for collection comparison only.
  It carries no soundness ranking, precedence or causal meaning; `refuted` is
  not "between" `proved` and `inconclusive` under it.
- **Causal order** — none. A result-disposition label is not a causal node.
- **Absence** — an absent result-disposition compares only to absence and is
  never defaulted to `inconclusive` or to any other member.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-360-AC-1 | Each member's value equals the exact label string in the table. | Inspection |
| FR-360-AC-2 | `proved` and `refuted` are accepted only with execution `completed`, their exact truth value, and a `closed-scope`, `decisive-witness` or `decisive-counterexample` settlement basis as named above. | Analysis |
| FR-360-AC-3 | `inconclusive` is accepted only with execution `completed`, truth `pending` and settlement basis `unsettled`. | Analysis |
| FR-360-AC-4 | `unsupported` is accepted only with FR-241 execution `unsupported`, `failed` only with FR-241 execution `failed` or `resource-incomplete`, the exhausted and provider-failure cases stay distinct on that axis, and neither label is accepted with truth `satisfied` or `violated`. | Analysis |
| FR-360-AC-5 | An absent result-disposition compares only to absence and receives no default member. | Analysis |

## Dependencies

- [FR-161](../../functional/temporal/FR-161-analyze-infinite-trace-properties.md)
  establishes the five labels and their infinite-trace meaning.
- [FR-241](../protocol/FR-241-assessment-execution-disposition.md),
  [FR-242](../protocol/FR-242-truth-disposition.md) and
  [FR-243](../protocol/FR-243-settlement-basis.md) supply the three axes this
  vocabulary maps onto; it introduces no new axis and no new member of any of
  them.
- [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  restates the settlement-basis vocabulary at FR-094-AC-7 and grounds the
  `inconclusive` -> `unsettled` mapping.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Status

Draft. This artifact records the FR-161 result vocabulary structurally as a
composition of the existing FR-241/FR-242/FR-243 axes. It establishes no new
obligation and claims no implementation coverage.
