---
id: FR-238
title: "Authored recovery relation"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-057
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-058
    type: specifies
---
# FR-238: Authored recovery relation

## Description

When evaluating compensation, the conformance evaluator SHALL decide recovery only through the compensation's exact typed recovery relation (FR-057).

FR-057 adds that the evaluator "SHALL retain the target and achieved states used
by the recovery relation."

FR-057 states that the evaluator "SHALL NOT infer recovery success from
compensation-operation success".

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Authored recovery predicate | FR-057 Inputs | Evaluated under the selected state and temporal profiles. |
| Declared type | FR-057 Description | The relation is "exact typed"; FR-057-AC-1 and FR-057-AC-2 exhibit a full-restoration relation and an authored partial-recovery relation. |
| Target state | FR-057 Behavior | Retained by the evaluator. |
| Achieved state | FR-057 Behavior | Retained by the evaluator. |
| Selected state and temporal profiles | FR-057 Inputs | The profiles the predicate is evaluated under. |
| Declared population scope, membership and completeness authorities | FR-058 Behavior | Required before success where recovery aggregates related effects. |

FR-058 states that "Where recovery aggregates related effects, it SHALL require
the exact declared population scope, membership and completeness authorities
before success", and FR-058-AC-3 states that "Two related partial refunds satisfy
the captured amount only under a complete declared population with exact
arithmetic."

## Identity and comparison

- **Identity kind** — *authored*. The relation is an authored predicate written
  in native source and selected per compensation; FR-057 calls it "the
  compensation's exact typed recovery relation" and "an authored recovery
  predicate".
- **Key** — not established. No requirement names the components that identify
  one recovery relation. Under FR-009 and FR-047, display name, source path,
  span, message text, locale and rendering never participate; FR-057-AC-1
  additionally shows that a relation is not satisfied by operation success but
  only when "the captured amount and target state match", which constrains
  satisfaction rather than identity.
- **Key order** — none. The key components are not fixed by the prose, so no
  total order over them can be declared. Full restoration and partial recovery
  are not ranked: FR-057-AC-2 states that "A successful partial refund can
  satisfy an authored partial-recovery relation while failing full restoration",
  which makes them separate authored relations, not two points on a scale.
- **Causal order** — none of its own. The relation is evaluated over the retained
  target and achieved states; the ordering of the compensation events it reads is
  the authored partial acyclic order of
  [FR-052](../../functional/protocol/FR-052-represent-bounded-control.md) and is never
  derived from key order, timestamp or source position.
- **Absence** — an absent input compares only to absence. FR-057-AC-4 states that
  "Missing state, commit or compensation-effect inputs yield incomplete/pending
  rather than recovery success", and FR-058-AC-4 that "Unknown or missing
  population membership and attempts beyond the retry bound cannot produce
  recovery success". An absent target or achieved state is never compared to a
  default or empty state.
- **Digest domain** — no key component is established, so none is a digest here.
  Where a relation or profile identity is carried as a digest, it retains the
  domain selected under
  [FR-201](../foundation/FR-201-canonical-identity-domain.md).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-238-AC-1 | Recovery is decided only by evaluating the authored typed relation over the retained target and achieved states. | Analysis |
| FR-238-AC-2 | A partial-recovery relation and a full-restoration relation are separate relations and neither ranks above the other. | Analysis |
| FR-238-AC-3 | An aggregating relation reports success only under the exact declared population scope, membership and completeness authorities. | Analysis |
| FR-238-AC-4 | A missing target state, achieved state or population member compares only to absence and cannot produce recovery success. | Analysis |

## Dependencies

- [FR-057](../../functional/protocol/FR-057-enforce-commit-recovery.md) establishes the
  typed recovery relation and its retained states.
- [FR-058](../../functional/protocol/FR-058-preserve-retry-and-partial-recovery.md)
  establishes the declared-population requirement for aggregating recovery.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

The prose establishes that a recovery relation is authored, typed, per
compensation, and evaluated under selected profiles. It does not establish:

- **Key** — which components identify one recovery relation, and whether the
  selected state and temporal profile identities participate in it.
- **Order** — no key order follows, since the components are unfixed.

FR-057 names "a full-restoration relation" and "an authored partial-recovery
relation" as examples inside acceptance criteria. No requirement states that
these are a closed set of relation types, so they are not enumerated here.

## Status

Draft. This artifact records existing meaning structurally. It establishes no new
obligation and claims no implementation coverage.
