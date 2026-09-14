---
id: FR-058
title: "Preserve retry, duplicate and partial-recovery outcomes"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-057
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-225
    type: references
  - target: ix://agent-ix/quire-specification/FR-226
    type: references
  - target: ix://agent-ix/quire-specification/FR-234
    type: references
  - target: ix://agent-ix/quire-specification/FR-235
    type: references
  - target: ix://agent-ix/quire-specification/FR-236
    type: references
  - target: ix://agent-ix/quire-specification/FR-238
    type: references
  - target: ix://agent-ix/quire-specification/FR-239
    type: references
  - target: ix://agent-ix/quire-specification/FR-287
    type: references
---
## Description

When multiple compensation observations relate to one obligation, the conformance evaluator SHALL preserve each attempt and effect identity and evaluate only the authored retry and recovery relations.

## Inputs

A registered compensation obligation, bounded retry policy, compensation
attempts/effects and any declared finite related-effect population.

## Outputs

Per-attempt and aggregate recovery facts with explicit failed, timed-out,
duplicate, pending and successful outcomes.

## Behavior

The evaluator SHALL distinguish a repeated receipt, retried attempt and duplicate
business effect. It SHALL refuse attempts beyond the authored finite retry bound.
Where recovery aggregates related effects, it SHALL require the exact declared
population scope, membership and completeness authorities before success.

A compensation attempt is keyed by one FR-035 authority-qualified opaque
runtime reference of the distinct kind `compensation-attempt`; it SHALL bind
separately to its compensation registration, operation and compensation-attempt
requirement. A compensation effect is keyed by one FR-035 authority-qualified
opaque runtime reference of kind `compensation-effect`; it SHALL bind separately
to its compensation attempt and effect requirement. Reusing one complete key
with different binding facts refuses as an identity contradiction. Re-observing
one complete key with equal facts is replay; a retry or second business effect
requires a different complete identity and remains related to its predecessor by
the authored retry or recovery relation.

The retry maximum is not an attempt ordinal and supplies neither identity nor
order. The evaluator SHALL count the distinct admitted compensation-attempt
FR-287 keys bound to one registration, including the initial attempt and every
admitted failed, timed-out or successful attempt. Replay or repeated receipt of
one complete attempt key counts once. A candidate whose admission would exceed
the maximum refuses, is not retained and does not change the prior count;
repeating that over-bound evidence remains the same refusal.

Earlier/later retry binders select the exact authored predecessor/successor
relation; they are not identity components. A missing, foreign, conflicting or
ambiguous relation refuses. Missing observation or progress evidence remains
incomplete and is never treated as zero attempts. The mechanical lexicographic
key order, timestamp, arrival order and table order are used neither to establish
retry sequence nor causality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-058-AC-1 | A failed refund attempt followed by one successful retry retains both attempts and one successful refund effect. | Test (TC-058) |
| FR-058-AC-2 | A timed-out attempt, failed attempt and duplicate effect remain three distinct outcomes. | Test (TC-058) |
| FR-058-AC-3 | Two related partial refunds satisfy the captured amount only under a complete declared population with exact arithmetic. | Test (TC-058) |
| FR-058-AC-4 | Unknown or missing population membership and attempts beyond the retry bound cannot produce recovery success. | Test (TC-058) |
| FR-058-AC-5 | A failed compensation attempt and its successful retry retain different keys under one registration, while replaying either complete key creates no third attempt. | Test (TC-058) |
| FR-058-AC-6 | The retry maximum, key order, timestamp, table order and arrival order never supply an attempt identity or retry relation; only the exact authored predecessor/successor relation orders distinct admitted attempts. | Test (TC-058) |
| FR-058-AC-7 | A compensation effect and compensation attempt use distinct authority-qualified subject kinds; replay preserves one key, while reusing either key with another registration, operation, attempt or requirement refuses as an identity contradiction. | Test (TC-058) |
| FR-058-AC-8 | The finite maximum counts distinct admitted compensation-attempt keys including the initial attempt and every admitted outcome; replay counts once, while an over-bound candidate is refused without retention or count mutation. | Test (TC-058) |
| FR-058-AC-9 | Missing observation/progress evidence remains incomplete, and a missing, foreign, conflicting or ambiguous retry relation refuses rather than supplying zero attempts or inferred order. | Test (TC-058) |

## Dependencies

- [FR-057](FR-057-enforce-commit-recovery.md).
- D-owned relationship/population identity and F-owned completeness.
- [FR-287](../../objects/protocol/FR-287-authority-qualified-runtime-subject-reference.md)
  defines the distinct compensation-attempt/effect runtime keys.
