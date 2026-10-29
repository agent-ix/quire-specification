---
id: TC-369
title: "Timed semantics, verdicts and replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-416
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-161
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-181
    type: verifies
---

# TC-369: Timed semantics, verdicts and replay

## Description

Settle timed claims over the `Rpc` model, replay timed counterexamples with exact delays, and refuse invalid timed evidence.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Rpc`**, **`NoLateReply`** and **`Settles`** (from QSL ADR-026 §11). `time Rpc dense unit ms;`. One object `c: Call { phase: Phase, inflight: Boolean, late: Boolean, x: Clock }`, `Phase` one of `Idle`, `Waiting`, `Replied` and `TimedOut`; universe `{c}`; initially `phase = Idle`, `inflight` and `late` false, `x = 0`. `Call::send` has pre `phase = Idle` and post `phase = Waiting`, `inflight`, `not late` and `x = 0`. `Call::reply` has pre `inflight and x >= 1 ms` and post `not inflight`, `late = (pre(phase) = TimedOut)`, and `phase = Replied` when `pre(phase) = Waiting`, unchanged otherwise. `Call::timeout` has pre `phase = Waiting and x >= T` and post `phase = TimedOut`. `Call::reset` has pre `not inflight and (phase = Replied or phase = TimedOut)` and post `phase = Idle`. The time invariants are `x <= 3 ms` while `inflight` and `x <= T` while `phase = Waiting`. `T` is a time constant each vector fixes. `NoLateReply` is `temporal NoLateReply using inf over (k: Rpc::Call) clock "model-steps" on origin { always holds(not k.late) }`. `Settles` is `temporal Settles using timed over (k: Rpc::Call) clock "model-time" on origin { always (holds(k.phase = Waiting) implies eventually[0 ms, 3 ms] holds(k.phase != Waiting)) }`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| TS-01 | `NoLateReply` with `T = 3 ms` | `refuted` with steps `send` (delay 0), `timeout` (delay 3), `reply` (delay 0), `trace_position` 3; replay checks each delay against the time invariants and guards in exact rationals |
| TS-02 | `NoLateReply` with `T = 4 ms` | `proved`, basis `zone-certified` or, all constraints being closed, `exhaustive` |
| TS-03 | `Settles` with `T = 3 ms` and `T = 4 ms`; with `eventually[0 ms, 3 ms)` | `proved` for both; `refuted` with `send` at 0 then `reply` after delay 3 |
| TS-04 | `always (holds(p) implies eventually[2 ms, 2 ms] holds(q))` over `Rpc`; the same formula over one timed trace | `unsupported`, `punctual-interval`; evaluated on the trace |
| TS-05 | A trace with a stutter step inserted at an equal time stamp | Every timed formula's value is unchanged |
| TS-06 | A behaviour taking infinitely many steps before 1 ms | Outside the admitted set |
| TS-07 | An idle tail at `Idle` for delays 1, 10 and 1000 ms | Each is an admitted behaviour |
| TS-08 | A timed lasso whose loop has zero total delay; one whose unreset clock is below its largest constant at entry | `invalid_runtime_input`/`invalid-value`; loop closure fails, `invalid_runtime_input`/`invalid-value` |
| TS-09 | A replayed step with a delay that violates `x <= 3 ms`; a guard false at the delayed state | `invalid_runtime_input`/`invalid-value` |
| TS-10 | A weak fairness constraint on `reply` enabled at every instant of a loop with positive delay where `reply` is never taken | That accepting cycle does not refute |
| TS-11 | A digital model `D` declared `time D tick D::tick period 5 ms unit ms;`, with `tick` taken at steps 1 and 3 | Time stamps 5 ms at positions 1 and 2, and 10 ms at position 3 |
| TS-12 | A lasso whose loop constraints admit no rational solution and no other accepting cycle | `inconclusive`, `lasso-not-concretized` |
| TS-13 | `always holds((lookup<R::Call>(calls, rw) absent undefined).late = false)` over `Rpc` with `T = 4 ms`, `calls` the population of `Call` and `rw` a `Reference<R::Call>` valued `w`, absent from `{c}` | `refuted`, cause `undefined-evaluation`: `where` names position 0, time stamp 0 ms, the `Idle` state key, the clock valuation `x = 0` and the `lookup`'s locus; `cause` is `absent-key`; the counterexample is the empty prefix with `trace_position` 0, and timed replay reproduces the undefined value there |
| TS-14 | `Rpc` extended with `ts: Sequence<Timer>[0, 3]`: insert one element, delay 2 ms, insert a fresh element at the front, delay 1 ms, filter out the front element; replay the trace | Valuations: slot 0 at `0`, then `2`, then slot 0 at `0` and slot 1 at `2`, then `1` and `3`, then slot 0 at `3` with no slot 1; each state key ignores the clock values; replay applies carry, fresh constant and drop and matches every post-state key |

## Expected Results

Verdicts, delays and replay outcomes are exactly as stated, and every counterexample replays with no provider present.
