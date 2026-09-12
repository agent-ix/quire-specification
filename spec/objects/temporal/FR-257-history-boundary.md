---
id: FR-257
title: "History boundary"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-092
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-253
    type: references
---
# FR-257: History boundary

## Description

When a bounded past operator is evaluated, the evaluator SHALL distinguish positions before an authoritative execution origin from required history that was omitted or evicted.

FR-092 states the distinction as a rule about what a supplied boundary proves:
"A supplied history cutoff is not an execution origin."

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Boundary kind | FR-092 Behavior | Either an authoritative execution origin or a supplied history cutoff. The two are not interchangeable. |
| Boundary position | FR-092 Inputs | Expressed under the selected clock binding ([FR-252](FR-252-temporal-clock-binding.md)). |
| Closure authority | FR-092 Inputs, FR-094 Behavior | An origin is authoritative; a cutoff is merely supplied. |
| Covered history | FR-092 Behavior | Whether "every needed position is inside the available history". |

FR-092 states the consequence verbatim: "If the cutoff does not prove that every
needed position is inside the available history, then the evaluator shall report
missing history rather than apply false extension." FR-094 states the
requirement from the other side: "Past evaluation shall require history through
its computed lower boundary or an authoritative execution origin."

Only an authoritative origin authorizes the false-extension reading. FR-092:
"For an anchor whose history is complete back to an authoritative origin, the
false-extension profiles shall treat atomic predicates before that origin as
false and Boolean constants as unchanged."

## Identity and comparison

- **Identity kind** — *value*. A history boundary has no identity of its own and
  is compared componentwise.
- **Key** — (boundary kind, boundary position, clock binding, closure
  authority). Equality is componentwise over exactly these and nothing else.
  Display name, source path, span, message text, locale and rendering never
  participate ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)). Boundary kind
  is a key component because FR-092-AC-2 holds that "The same visible suffix
  yields a Boolean when its start is the authoritative execution origin and
  missing-history incomplete when it is merely a cutoff": equal positions under
  different kinds are not the same boundary.

  Boundary kind participates as the distinction FR-092 states in prose — an
  authoritative execution origin versus a supplied history cutoff — and **not**
  as a label. No requirement fixes a spelling, a wire form or a closed
  vocabulary for boundary kinds (see **Open gap**), so no exact-string match
  over a boundary-kind label is established and none is asserted here. Where a
  boundary carries no admitted kind, the kind is absent and the key is not
  established for that boundary; an unspelled or unmatched kind is not silently
  read as either an origin or a cutoff.
- **Key order** — the order over the boundary position within one equal clock
  binding and kind, for collection comparison only. It carries no causal
  meaning, and an earlier boundary is not a causal predecessor of a later one.
  FR-090 states that the evaluator "shall not use timestamp order alone to
  establish causality or break a tie for an order-sensitive operator".
- **Causal order** — none of its own; where a past operator is order-sensitive
  the order used is the admitted causal/sequence order of the clock binding
  (FR-252, FR-090-AC-4), never derived from key order, timestamp or ingestion
  order.
- **Absence** — an absent boundary compares only to absence and is never
  defaulted to the earliest available record, to the start of the file or to an
  origin. FR-092 is explicit that missing history stays missing: "Missing,
  evicted, stale or binding-mismatched history never becomes false padding or a
  conclusive Boolean" (FR-092-AC-4). FR-094-AC-6 holds that "Missing
  predecessor/history on restart requires replay or an admitted
  state-restoration contract and cannot resume as a fresh complete stream."
- **Digest domain** — a boundary or authority component carried as a digest
  retains the domain it was derived under
  ([FR-201](../foundation/FR-201-canonical-identity-domain.md)); an identity
  crossing domains is unequal regardless of byte equality.

## A boundary is not an anchor

The anchor ([FR-253](FR-253-temporal-evaluation-anchor.md)) is the position
offsets are counted from; the history boundary decides whether the positions so
counted exist as admitted history. FR-092 reads past offsets at `i-d` from the
anchor and separately requires history back to the boundary; neither substitutes
for the other.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-257-AC-1 | An authoritative execution origin and a supplied history cutoff at the same position compare unequal. | Analysis |
| FR-257-AC-2 | A cutoff that does not cover every needed position yields missing history rather than false extension. | Analysis |
| FR-257-AC-3 | An absent history boundary compares only to absence and is not defaulted to the earliest available record. | Analysis |
| FR-257-AC-4 | Missing, evicted or stale history is not represented as a conclusive Boolean. | Analysis |
| FR-257-AC-5 | The declared key order over boundary positions is never cited as a causal relation. | Analysis |
| FR-257-AC-6 | No boundary-kind label spelling is asserted by this artifact, and a boundary whose kind is absent or unmatched is not compared as either an origin or a cutoff. | Inspection |

## Dependencies

- [FR-092](../../functional/temporal/FR-092-evaluate-bounded-past.md) establishes the
  origin/cutoff distinction and the missing-history rule.
- [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  establishes the history requirement through the computed lower boundary and
  the restart rule.
- [FR-253](FR-253-temporal-evaluation-anchor.md) supplies the anchor the offsets
  are counted from.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Open gap

No requirement states which admitted history boundary kinds exist beyond the two
FR-092 contrasts, nor how an "admitted state-restoration contract"
(FR-094-AC-6) is identified, nor whether a restored boundary is an origin or a
cutoff. No boundary-kind vocabulary is enumerated here. FR-092 names the two it
contrasts in English prose only and fixes no label spelling, field name or wire
form for either, so the key above compares the stated distinction and not a
label string.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
