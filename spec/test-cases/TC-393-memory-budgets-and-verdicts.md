---
id: TC-393
title: "Memory budgets, bound-limited states and verdicts"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-439
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-437
    type: verifies
---

# TC-393: Memory budgets, bound-limited states and verdicts

## Description

Check the memory budgets, their defaults, bound-limited and boundary states,
the `memory-bound-reached` cause and its precedence, the statement of the
budget in every result, and garbage collection keeping loops finite.

## Test Procedure

Build protocol `Loop` with `parallel P memory tso`: branch `w` stores `x := 1`
then `x := 0` inside a `repeat` with no maximum and a guard that stays true, so it never terminates,
and branch `r` loads `x` once. Check `always holds(true)` and the variants
below through the real local Rust boundary, recording verdict, cause,
bound-limited state count and method.

## Vectors

| Vector | Variant | Expected result |
| --- | --- | --- |
| MB-01 | `max_store_buffer` 2 | `inconclusive`, cause `memory-bound-reached` naming `max_store_buffer`, value 2, the count of bound-limited states and the member that raises it; no bound-limited state is terminal or a deadlock |
| MB-02 | no budget member | runs with 4; method states `max_store_buffer` 4, reached |
| MB-03 | MB-01 with claim `always holds(x.v = 0)` | `refuted` with a counterexample within the budget |
| MB-04 | `max_store_buffer` 2 and 3 | the same obligation identity |
| MB-05 | MB-01 with a state constraint that also cuts states | cause `constraint-reached` |
| MB-06 | the `on each` subject of TC-381 with a `tso` `parallel`, `max_live_instances` 1 and `max_store_buffer` 1, both reached | cause `memory-bound-reached` |
| MB-07 | TC-390's `SB` under `ra`, and a variant in which both threads load `x` after `Sx` | in the variant, `x0` is removed in the step after which no view, message views included, points at it; after `join all` each location keeps one message; the run states `max_messages` 4, not reached |
| MB-08 | TC-390 SB-02 | `proved` verdicts and refutations both state the budget used and whether it was reached |
| MB-09 | MB-01 with an empty population `extras` and the claim `always (holds(x.v = 1) implies holds(lookup<M::Cell>(extras, x) absent undefined .v = 0))`, while the run also reaches bound-limited states | `refuted`, cause `undefined-evaluation`, not `inconclusive` with `memory-bound-reached`; the counterexample carries the memory component and replays |

## Expected Results

Every verdict, cause, count and method statement matches the table.
