---
id: TC-368
title: "Timed profile, clock bindings and interval openness"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-090
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-250
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-252
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-255
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-416
    type: verifies
---

# TC-368: Timed profile, clock bindings and interval openness

## Description

Select the timed profile, bind `model-time` and `model-steps`, and compare intervals with open and closed ends.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Rpc`**, **`NoLateReply`** and **`Settles`** (from QSL ADR-026 §11). `time Rpc dense unit ms;`. One object `c: Call { phase: Phase, inflight: Boolean, late: Boolean, x: Clock }`, `Phase` one of `Idle`, `Waiting`, `Replied` and `TimedOut`; universe `{c}`; initially `phase = Idle`, `inflight` and `late` false, `x = 0`. `Call::send` has pre `phase = Idle` and post `phase = Waiting`, `inflight`, `not late` and `x = 0`. `Call::reply` has pre `inflight and x >= 1 ms` and post `not inflight`, `late = (pre(phase) = TimedOut)`, and `phase = Replied` when `pre(phase) = Waiting`, unchanged otherwise. `Call::timeout` has pre `phase = Waiting and x >= T` and post `phase = TimedOut`. `Call::reset` has pre `not inflight and (phase = Replied or phase = TimedOut)` and post `phase = Idle`. The time invariants are `x <= 3 ms` while `inflight` and `x <= T` while `phase = Waiting`. `T` is a time constant each vector fixes. `NoLateReply` is `temporal NoLateReply using inf over (k: Rpc::Call) clock "model-steps" on origin { always holds(not k.late) }`. `Settles` is `temporal Settles using timed over (k: Rpc::Call) clock "model-time" on origin { always (holds(k.phase = Waiting) implies eventually[0 ms, 3 ms] holds(k.phase != Waiting)) }`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| TB-01 | A claim over `Rpc` with no `clock` member; one with `clock "model-time"`; one with `clock "model-steps"` | `model-time` with profile `quire.temporal.timed/v1`; the same binding and obligation identity; `model-steps` |
| TB-02 | A claim over an untimed model with `clock "model-time"` | `invalid_timed_model`/`model-time-on-untimed-model` |
| TB-03 | `model-time` bindings differing in unit (`ms` and `us`), in source kind, or in tick period | Three unequal bindings |
| TB-04 | `[0 ms, 3 ms]`, `[0 ms, 3 ms)`, `(0 ms, 3 ms]`, `(0 ms, 3 ms)`; an offset of exactly `3 ms` | Four unequal intervals; the offset lies in the first and third |
| TB-05 | `(2 ms, 2 ms]` | `invalid_timed_model`/`invalid-interval` |
| TB-06 | `[0, 3]` under event-position and `[0 ms, 3 ms]` under the timed profile | Unequal intervals with distinct meanings |
| TB-07 | Unbounded `always holds(p)` under the timed profile; the timed profile's TL target | Admitted; absent target |
| TB-08 | Profile selection by an unknown identity, or two profiles | Refused before evaluation, naming the clause |

## Expected Results

Every selection, binding and interval comparison is exactly as stated.
