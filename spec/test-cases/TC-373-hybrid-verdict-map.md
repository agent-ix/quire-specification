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
| HY-06 | A `hybrid-decl` whose only continuous variable has rate 1 in every mode | `invalid_timed_model`/`rate-one-hybrid` |
| HY-07 | A solver stopped by its time budget; a run ended by the caller's `deadline_ms`; a result spelled `limit-reached` for the time budget, or `time-limit` | `incomplete`, `resource-incomplete`, `timed-out`; `incomplete`, `cancelled{source: deadline}`; the last two spellings refused `invalid_result`/`unknown-cause`; FR-193-AC-8, FR-331-AC-29 |
| HY-08 | `max_solver_time` 1.5 s, 2 s and 0.2 s; two requests differing only in `max_solver_time`, and two differing only in `max_solver_steps`; a `timed-out` result offered to the cache | 2 s, 2 s, 1 s; equal request identity and cache key bytes, then different ones; stored by neither request, with no wall-time member; FR-193-AC-9 |

## Expected Results

Each solver result settles exactly as stated.
