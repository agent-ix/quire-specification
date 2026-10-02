---
id: TC-373
title: "Hybrid solver verdict map"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-193
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
| HY-04 | A solver stopped by its time budget | `incomplete`, `limit-reached` naming the budget |
| HY-05 | A hybrid item with no registered candidate | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
| HY-06 | A `hybrid-decl` whose only continuous variable has rate 1 in every mode | `invalid_timed_model`/`rate-one-hybrid` |

## Expected Results

Each solver result settles exactly as stated.
