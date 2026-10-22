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
| MB-07 | TC-390's `SB` under `ra`; a variant in which both threads load `x` after `Sx`; and a branch repeating, with no maximum and a guard that stays true, a `relaxed` store of `x` and then a `relaxed` load of `x`, with `max_messages` 1 and `always holds(true)` | in the first variant, `x0` is removed in the step after which no running thread's current view points at it, while the message views and `S` still do; after `join all` each location keeps one message; the run states `max_messages` 4, not reached; the looping branch settles `proved`, states `max_messages` 1 not reached, and holds one live message of `x` in every state (FR-437-AC-4) |
| MB-08 | TC-390 SB-02 | `proved` verdicts and refutations both state the budget used and whether it was reached |
| MB-09 | MB-01 with an empty population `extras` and the claim `always (holds(x.v = 1) implies holds(lookup<M::Cell>(extras, x) absent undefined .v = 0))`, while the run also reaches bound-limited states | `refuted`, cause `undefined-evaluation`, not `inconclusive` with `memory-bound-reached`; the counterexample carries the memory component and replays |
| MB-10 | the load-buffering litmus (TC-391 LT-06), whose two branches each load one location then store the other, as an item resolved to `ra`, then to `sc`, then to `tso`; a variant with both loads made `acquire`; a variant with one load made `acquire` | under `ra` the terminal record `warnings` hold two `memory.load-buffering-shape{load, store}` warnings, one per branch, each naming that branch's load and store, and the value, label, basis, counterexample and obligation identity equal the same request settled without the warnings; under `sc` and `tso` and in the both-`acquire` variant no warning; in the one-`acquire` variant one warning, for the other branch (FR-439-AC-9) |
| MB-11 | `SB` (TC-390) resolved to `sc`, `tso` and `ra` with one registered temporal candidate whose descriptor has no `memory_models`, and one whose descriptor has `memory_models` [`sc`, `tso`, `ra`] | the first candidate is a candidate for the `sc` item only; the second for all three (FR-439-AC-9) |

## Expected Results

Every verdict, cause, count and method statement matches the table.
