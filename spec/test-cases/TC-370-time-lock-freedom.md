---
id: TC-370
title: "Time-lock freedom and vacuity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-417
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-161
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
---

# TC-370: Time-lock freedom and vacuity

## Description

Derive the time-lock-freedom item, refute it with `time-lock` evidence, separate deadlocks from time-locks, and settle vacuous proofs.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Rpc`**, **`NoLateReply`** and **`Settles`** (from QSL ADR-026 §11). `time Rpc dense unit ms;`. One object `c: Call { phase: Phase, inflight: Boolean, late: Boolean, x: Clock }`, `Phase` one of `Idle`, `Waiting`, `Replied` and `TimedOut`; universe `{c}`; initially `phase = Idle`, `inflight` and `late` false, `x = 0`. `Call::send` has pre `phase = Idle` and post `phase = Waiting`, `inflight`, `not late` and `x = 0`. `Call::reply` has pre `inflight and x >= 1 ms` and post `not inflight`, `late = (pre(phase) = TimedOut)`, and `phase = Replied` when `pre(phase) = Waiting`, unchanged otherwise. `Call::timeout` has pre `phase = Waiting and x >= T` and post `phase = TimedOut`. `Call::reset` has pre `not inflight and (phase = Replied or phase = TimedOut)` and post `phase = Idle`. The time invariants are `x <= 3 ms` while `inflight` and `x <= T` while `phase = Waiting`. `T` is a time constant each vector fixes. `NoLateReply` is `temporal NoLateReply using inf over (k: Rpc::Call) clock "model-steps" on origin { always holds(not k.late) }`. `Settles` is `temporal Settles using timed over (k: Rpc::Call) clock "model-time" on origin { always (holds(k.phase = Waiting) implies eventually[0 ms, 3 ms] holds(k.phase != Waiting)) }`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| TL-01 | A request with items over two distinct timed subjects and one untimed subject | Exactly two derived items, each of kind `time-lock-freedom`, FR-290 kind `temporal-satisfaction`, identity (subject, `time-lock-freedom`) |
| TL-02 | `Rpc` with `reply` guarded `x > 3 ms`, `timeout` guarded `x > T`, `T = 3 ms` | Time-lock-freedom `refuted` with `time-lock` evidence: stem `send` at 0, final delay 3; replay finds no admissible positive delay and no enabled identity |
| TL-03 | TL-02's deadlock-freedom item; both items with non-strict guards | Deadlock freedom `proved`; both `proved` |
| TL-04 | A non-local time-lock (a Zeno cycle of steps with zero delay under a time invariant), and a trap whose fresh exploration finds a quiescent state | `refuted` after the fresh unreduced exploration completes with no target; `inconclusive`, `replay-parity` |
| TL-05 | A proof over a timed subject whose initial state has no fair time-divergent behaviour | `inconclusive`, `no-admitted-behaviour` |
| TL-06 | The unmodified `Rpc` time-lock-freedom item | `proved` with a zone certificate the checker accepts |

## Expected Results

Every derived item and verdict is exactly as stated.
