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

Map external hybrid solver results onto verdicts, check the hybrid source forms and claims, and replay hybrid traces, a hybrid item settling `refuted` only through exact replay.

## Test Procedure

Register a fixture solver backend through the real local Rust boundary that returns each scripted result, and compare the settled result with the expected result. Check each source form of HY-10 to HY-14 through the frontend, and replay each trace of HY-15 and HY-16 through the reader holding the checked model.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| HY-01 | A δ-complete solver `unsat` for bounded unreachability within horizon 10 s and 5 jumps | `proved`, `bounded-solver{method, horizon: 10 s, jumps: 5}` |
| HY-02 | An outward-rounded flowpipe that satisfies the property | `proved`, `bounded-solver` |
| HY-03 | `δ-sat`; a flowpipe meeting the unsafe set with a reported trace; `unknown`; a method without sound rounding | `inconclusive`, `solver-inconclusive`, retaining the trace with its replay outcome; `refuted` only by HY-16's exact replay |
| HY-04 | A solver stopped by its step budget at `max_solver_steps` 1,000 | `incomplete`, `resource-incomplete`, `limit-reached{limit: max_solver_steps, value: 1000, setting: hybrid.max_solver_steps}`; FR-193-AC-8, FR-331-AC-29 |
| HY-05 | A hybrid item with no registered candidate | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
| HY-06 | A `hybrid-decl` whose only continuous variable has the rate literal `1` in every mode; then one whose only variable has the rate `2 - 1` in every mode | `invalid_timed_model`/`rate-one-hybrid`; then admitted, the variable hybrid dynamics; FR-193-AC-7 |
| HY-07 | A solver that exhausts `max_solver_time`; a plugin provider that overruns `deadline_ms`; a run the caller's frontend ends at its own deadline; a `limit-reached{limit: max_solver_time}` or `time-limit` cause on a hybrid item | `incomplete`, `resource-incomplete`, `timed-out`; the same, `timed-out`; `incomplete`, `cancelled{source: deadline}`; the last two refused `invalid_result`/`unknown-cause`; FR-193-AC-8, FR-331-AC-29 |
| HY-08 | A caller's `max_solver_time` of 1.5 s, 2 s and 0.2 s written as wire values, then a wire value of 1.5; two requests differing only in `max_solver_time`, and two differing only in `max_solver_steps`; a `timed-out` result offered to the cache | wire 2, 2, 1; 1.5 refused before consumption; equal request identity and cache key bytes, then different ones; stored by neither request, with no wall-time member; FR-193-AC-9, FR-331-AC-29 |
| HY-09 | A `solver-inconclusive` cause with a `HybridTrace` whose `start` is in mode `Moving` and whose one jump takes `transition` 0 to `Stopped`; one with no `trace`; then that trace with `mode` `Halted`, with `start` in `Stopped`, with a jump `transition` 1, with a jump by `transition` 0 reaching `Moving`, with `v` removed from an `enclosure`, with `w` added to an `enclosure`, and with an `Interval` whose `lower` exceeds its `upper`, each read by the reader holding the checked model; and a `trace` on the `solver-inconclusive` cause of an FR-468 refinement item | each of the first two `inconclusive`, `solver-inconclusive`, the trace kept on the record with its static replay outcome and absent from `counterexamples`; the next seven refused `invalid_result`/`invalid-value` at the member; the last refused `invalid_result`/`unknown-member` at `/cause/trace`; FR-193-AC-10 |
| HY-10 | `Braking` with `initial { mode Moving when { v = 20 and a >= 1 and a <= 2 and t = 0 }; mode Stopped when { v = 0 and t = 0 }; }`; `Braking` with no `initial` clause and with `initial { mode Moving when { true }; }`; `initial { mode Halted when { true }; }`; a model whose initial set is empty | two initial modes; equal initial sets, `Moving`'s invariant at time 0; `missing_declaration`/`missing-name`; `inconclusive`, `no-initial-state`; FR-193-AC-11 |
| HY-11 | The flows `v' = -a * v`, `x' = if v > 1 then -1 else 0` and `v' = drag(v)` with `drag(s: Rational[-100, 100; 1, 1]): Rational[-1000, 0; 1, 10]` `{ - s * s / 10 }`, a guard `v * v >= 2`, a `let` and a `!=`; then `v' = v div 2`, a call passing `v` to an `Integer` parameter, and a model that reaches `1 / v` at `v = 0` | admitted, `drag` defined at `v = 3/2` and without a value at `v = 101`; `ill_typed`/`type-mismatch` twice; no `proved` for the last; FR-193-AC-12 |
| HY-12 | Resets `{ v = pre(v) / 2 and a >= 1 and a <= 2 }`, `{ v = v + 1 }`, `{ v = 0 and v >= 0 }`, `{ v + a = 1 }`, `{ v = 0 or v = 1 }`, and a reset whose post-states all break the target invariant | halved `v`, `a` in `[1, 2]`, `t` kept; pre-jump `v` plus 1; `conflicting_declaration`/`duplicate-declaration`; `invalid_model_binding`/`malformed-declaration` twice; the transition never enabled; FR-193-AC-13 |
| HY-13 | Over `Braking`: `always [0, 10] holds(h.v < 30)`; `always [0, 10] (holds(h.v = 0) implies once [0, 2] holds(h.v <= 1))`; `holds(h.v > 0) since [0, 3] holds(h.v = 20)`; `eventually [0 s, 1500 ms] holds(h.v < 19)`; `always (0.5 s, 2 s] holds(h.v <= 20)`; `[2 kg, 3 kg]`; `(1 s, 1 s]`; `eventually [0, 1] holds(h.v = 0)` under `clock "model-steps"` | holds on every behaviour; both past formulas read pointwise over instants; intervals `[0, 3/2]` and `(1/2, 2]` seconds; `ill_typed`/`type-mismatch`; `invalid_timed_model`/`invalid-interval`; an interval of jumps; FR-193-AC-14 |
| HY-14 | `on each (q: Braking) when (q.v > 10)` with `capture v0: Rational = q.v;` over `eventually [0, 10] holds(h.v < v0)`; `on each (q: Other)`; `fair Moving;`, `fair strong each Moving::Stopped;` and `fair Moving::Halted;` | one obligation per instant with `v > 10`, each with its own `v0`; `ill_typed`/`type-mismatch`; `Moving`'s transitions, those targeting `Stopped`, then `missing_declaration`/`missing-name`; FR-193-AC-15 |
| HY-15 | Static replay of a trace whose `start` box meets no initial entry; of one whose `before` box misses the guard by less than its `delta` 1/100; of that trace with no `delta` | `inconsistent{at: /start, check: 1}`; `consistent`; `inconsistent` at check 4; each item still `inconclusive`; FR-193-AC-16 |
| HY-16 | The point trace from `(Moving, v = 20, a = 2, t = 0)` with `before` `(time 10, v = 0, a = 2, t = 10)` and transition 0 to `(Stopped, time 10, v = 0, a = 2, t = 10)` against `always [0, 12] holds(h.v > 0)`; that trace with `before` `v = 1`, as a `solver-inconclusive` trace and as a provider's `refuted` entry; a point trace through a mode with the flow `v' = v` | `reproduced-with-evaluated-witness`, `refuted`, `HybridCounterexample` kind `formula`; `not-reproduced` at check 2, `inconclusive`; `inconclusive`, `replay-parity`; static replay only, never `refuted`; FR-193-AC-17 |

## Expected Results

Each solver result settles exactly as stated.
