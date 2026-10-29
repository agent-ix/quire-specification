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
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
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
- **Key** — (`FR-288 qualified protocol-definition identity`, `exact
  declaration-local compensation handle`, `exact declaration-local typed
  recovery-expression handle`). The admitted typed expression retains its
  selected model, state, temporal and clock semantic dependencies; a missing or
  mismatched dependency refuses linking rather than being guessed into the key.
  The local handle claims no source-free typed-expression identity (FR-015).
  Target and achieved state values constrain evaluation and do not identify the
  relation. Display name, source path, span, message text, locale and rendering
  never participate.
- **Key order** — lexicographic over the three key components for canonical
  comparison only. Full restoration and partial recovery
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

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-238-AC-1 | Recovery is decided only by evaluating the authored typed relation over the retained target and achieved states. | Analysis |
| FR-238-AC-2 | A partial-recovery relation and a full-restoration relation are separate relations and neither ranks above the other. | Analysis |
| FR-238-AC-3 | An aggregating relation reports success only under the exact declared population scope, membership and completeness authorities. | Analysis |
| FR-238-AC-4 | A missing target state, achieved state or population member compares only to absence and cannot produce recovery success. | Analysis |
| FR-238-AC-5 | Two relations compare equal exactly when all three declared key components compare equal; equal predicate text in different compensations remains unequal, while changed target/achieved operands retain the same selected relation. | Analysis |
| FR-238-AC-6 | A missing or mismatched model, state, temporal or clock dependency refuses the typed expression binding rather than being omitted or appended as a guessed key component. | Analysis |

## Dependencies

- [FR-057](../../functional/protocol/FR-057-enforce-commit-recovery.md) establishes the
  typed recovery relation and its retained states.
- [FR-058](../../functional/protocol/FR-058-preserve-retry-and-partial-recovery.md)
  establishes the declared-population requirement for aggregating recovery.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-288](FR-288-qualified-protocol-definition-identity.md) supplies the exact
  linked protocol-definition component.

## Remaining vocabulary note

FR-057 names "a full-restoration relation" and "an authored partial-recovery
relation" as examples. They are not a closed enumeration or a ranking; the
identity rule above does not turn them into one.

## Status

Draft. Specification issue #28 fixed the recovery-relation key and comparison
order. Executable cross-repository evidence remains planned.
