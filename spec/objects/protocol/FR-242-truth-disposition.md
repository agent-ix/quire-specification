---
id: FR-242
title: "Truth disposition vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-062
    type: specifies
---
# FR-242: Truth disposition vocabulary

## Description

When a result reports the truth of an assessed obligation, the strict reader SHALL match the truth disposition as an exact string from this closed vocabulary, in which neither `pending` nor `unavailable` is a Boolean success.

## Values

| Value | Description |
| --- | --- |
| `satisfied` | Assessment execution is `completed`, activation is `triggered`, the settlement basis is `closed-scope` or `decisive-witness`, and every fact in the exact decision-support set is complete (FR-061). Projects to E02 `logical.outcome` `satisfied` (FR-062). |
| `violated` | Assessment execution is `completed`, activation is `triggered`, the settlement basis is `closed-scope` or `decisive-counterexample`, and every fact in the exact decision-support set is complete (FR-061). Projects to E02 `logical.outcome` `violated` (FR-062). |
| `pending` | Assessment execution is `completed`, activation is `triggered`, decision-scope progress is open and the settlement basis is `unsettled`; admitted continuations do not yet preserve one Boolean truth (FR-061). Projects to E02 `logical.outcome` `inconclusive` (FR-062). |
| `unavailable` | Truth is not available: assessment execution is not `completed`, activation is `inactive` or `activation-unknown`, or a required decision-support fact is missing (FR-061). Projects to E02 `not-evaluated` after non-completed execution or inactive activation, and to E02 `unknown` after completed execution with triggered or unknown activation (FR-062). |

## Identity and comparison

- **Identity kind** — *value*. A truth label has no identity of its own and is
  compared componentwise as an exact string.
- **Key** — the label itself. Matching is exact. Display name, source path, span,
  message text, locale and rendering never participate in identity; FR-061 states
  a strict reader "SHALL NOT infer independence from distinct display names or
  implementations".
- **Key order** — lexicographic over the label, for collection comparison only. It
  carries no truth ordering, severity ranking or causal meaning, and `pending` is
  not "between" `satisfied` and `violated` under it.
- **Causal order** — none. A truth disposition is not a causal node. Its
  supersession lineage under FR-061 is a separate causal fact and is never
  derived from key order, timestamp or source position.
- **Absence** — an absent truth compares only to absence. It is distinct from
  `unavailable`, which is a supplied value, and it is never defaulted.

## Temporal boundary correspondence

FR-091 and FR-092 use this vocabulary directly for completed temporal truths:
`satisfied`, `violated` and, where future progress remains open, `pending`.
Their incomplete or refused evaluator outcomes carry no truth fact at the lower
boundary. The canonical FR-061 adapter represents that absence as the supplied
value `unavailable` while retaining the distinct execution disposition and
missing premise. It never treats absence itself as `unavailable` before that
explicit adaptation. An `indeterminate` outcome of the timestamped finite-window
profile (FR-091) likewise carries no truth fact and adapts to `unavailable` with
settlement basis `unavailable`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-242-AC-1 | Each member's value equals the exact label string in the table. | Inspection |
| FR-242-AC-2 | `pending`, `unavailable` and an absent truth are each distinguishable from `satisfied` and from each other without diagnostic prose. | Analysis |
| FR-242-AC-3 | An absent truth compares only to absence and is never defaulted to `unavailable`. | Analysis |
| FR-242-AC-4 | FR-091 and FR-092 emit the canonical `satisfied`/`violated` spellings for completed truths, and their non-completed lower outcomes reach this axis only through the explicit `unavailable` adaptation. | Inspection |

## Dependencies

- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) establishes the
  truth axis and its combination rules.
- [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
  establishes the projection of each truth state.
- [FR-091](../../functional/temporal/FR-091-evaluate-bounded-future.md) and
  [FR-092](../../functional/temporal/FR-092-evaluate-bounded-past.md) supply the temporal
  outcomes and explicit lower-boundary adaptation stated above.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Accepted FS02 vocabulary ruling. These four labels are authoritative at the
canonical-result boundary and the temporal correspondence is explicit.
