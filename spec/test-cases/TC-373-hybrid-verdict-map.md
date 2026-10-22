---
id: TC-373
title: "Hybrid solver verdict map"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-193
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-373: Hybrid solver verdict map

## Description

Map external hybrid solver results onto verdicts, with no solver result settling `refuted`.

## Test Procedure

Register a fixture solver backend through the real local Rust boundary that returns each scripted result, and compare the settled result with the expected result.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| HY-01 | A δ-complete solver `unsat` for bounded unreachability within horizon 10 s and 5 jumps | `proved`, `bounded-solver{method, horizon: 10 s, jumps: 5}` |
| HY-02 | An outward-rounded flowpipe that satisfies the property | `proved`, `bounded-solver` |
| HY-03 | `δ-sat`; a flowpipe meeting the unsafe set with a reported trace; `unknown`; a method without sound rounding | `inconclusive`, `solver-inconclusive`, retaining the trace as unverified evidence; never `refuted` |
| HY-04 | A solver stopped by its step budget at `max_solver_steps` 1,000 | `incomplete`, `resource-incomplete`, `limit-reached{limit: max_solver_steps, value: 1000, setting: hybrid.max_solver_steps}`; FR-193-AC-8, FR-331-AC-29 |
| HY-05 | A hybrid item with no registered candidate | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
| HY-06 | A `hybrid-decl` whose only continuous variable has the rate literal `1` in every mode; then one whose only variable has the rate `2 - 1` in every mode | `invalid_timed_model`/`rate-one-hybrid`; then admitted, the variable hybrid dynamics; FR-193-AC-7 |
| HY-07 | A solver that exhausts `max_solver_time`; a plugin provider that overruns `deadline_ms`; a run the caller's frontend ends at its own deadline; a `limit-reached{limit: max_solver_time}` or `time-limit` cause on a hybrid item | `incomplete`, `resource-incomplete`, `timed-out`; the same, `timed-out`; `incomplete`, `cancelled{source: deadline}`; the last two refused `invalid_result`/`unknown-cause`; FR-193-AC-8, FR-331-AC-29 |
| HY-08 | A caller's `max_solver_time` of 1.5 s, 2 s and 0.2 s written as wire values, then a wire value of 1.5; two requests differing only in `max_solver_time`, and two differing only in `max_solver_steps`; a `timed-out` result offered to the cache | wire 2, 2, 1; 1.5 refused before consumption; equal request identity and cache key bytes, then different ones; stored by neither request, with no wall-time member; FR-193-AC-9, FR-331-AC-29 |
| HY-09 | A `solver-inconclusive` cause with a `HybridTrace` whose `start` is in mode `Moving` and whose one jump takes `transition` 0 to `Stopped`; one with no `trace`; then that trace with `mode` `Halted`, with `start` in `Stopped`, with a jump `transition` 1, with a jump by `transition` 0 reaching `Moving`, with `v` removed from an `enclosure`, with `w` added to an `enclosure`, and with an `Interval` whose `lower` exceeds its `upper`, each read by the reader holding the checked model; and a `trace` on the `solver-inconclusive` cause of an FR-468 refinement item | each of the first two `inconclusive`, `solver-inconclusive`, the trace kept on the record and absent from `counterexamples`, never replayed; the next seven refused `invalid_result`/`invalid-value` at the member; the last refused `invalid_result`/`unknown-member` at `/cause/trace`; FR-193-AC-10 |

## Expected Results

Each solver result settles exactly as stated.
