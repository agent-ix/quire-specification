---
id: FR-369
title: "Name the strong fairness a refutation's loop would exclude"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-362
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-364
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
---

# FR-369: Name the strong fairness a refutation's loop would exclude

## Description

When a lasso counterexample of a liveness item reproduces, the replayer SHALL
attach to the item's terminal record one warning per strong fairness
constraint that would make the loop unfair.

## Inputs

A lasso counterexample that settled `reproduced-with-evaluated-witness`
([FR-364](FR-364-replay-temporal-counterexamples.md)), the enabled transition
identities replay recomputed at each loop state, and the clause's resolved
fairness set.

## Outputs

Zero or more `fairness.strong-would-exclude` warnings on the item's
[FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
terminal record.

## Behavior

For each operation `Op` of the model, in ascending order of its qualified name
compared bytewise as UTF-8, the replayer SHALL:

1. If some loop state enables `Op` and no loop step takes `Op`, suggest
   (`strong`, `Op`, `whole`).
2. Otherwise, if some transition identity of `Op` is enabled at a loop state
   and no loop step takes that identity, suggest (`strong`, `Op`, `each`) with
   the first such identity in canonical transition order.

A reproduced lasso is fair under every constraint the clause carries, so no
suggestion names one of them. Each suggestion is one warning with code
`fairness.strong-would-exclude` and these members: `constraint`, the suggested
`{kind, operation, granularity}`; `identity`, the starved transition identity
for `each` and `null` for `whole`; and `position`, the first loop position at
which the suggested constraint is enabled.

The warning is diagnostic only. It lies outside the counterexample, its replay
identity and the obligation identity, and it changes no verdict. Replay
computes it from the enabledness it already recomputes, so a refutation from
any engine receives it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-369-AC-1 | A reproduced lasso under `fair weak each Op`, whose loop takes `Op` with one identity and never takes another identity that is enabled at a loop state, carries one `fairness.strong-would-exclude` warning naming (`strong`, `Op`, `each`), that identity and the first loop position enabling it. | Test (TC-304) |
| FR-369-AC-2 | A reproduced lasso whose loop enables `Op` at some state and never takes it carries a warning naming (`strong`, `Op`, `whole`) with `identity` `null`. | Test (TC-304) |
| FR-369-AC-3 | A refutation whose clause carries (`strong`, `Op`, `whole`) receives no warning naming that constraint, and a loop that takes every enabled identity carries no warning. | Test (TC-304) |
| FR-369-AC-4 | Removing every `fairness.strong-would-exclude` warning from a terminal record changes neither the verdict, the counterexample, its replay identity nor the obligation identity. | Test (TC-304) |
| FR-369-AC-5 | A refutation produced by any engine and reproduced by replay receives the same warnings for the same lasso. | Test (TC-304) |

## Dependencies

- [FR-362](FR-362-apply-fairness-constraints.md) owns enabledness, the kinds
  and the granularities.
- [FR-364](FR-364-replay-temporal-counterexamples.md) owns the replay that
  recomputes enabledness.

## References

- quire-spec-language ADR-019 SV-6, RU-4 and QS-8; Linear STD-132 (paired with
  QSL-365).
