---
id: TC-374
title: "Stochastic delays, timed sampling and exact digital checking"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-420
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-421
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-411
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-407
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-413
    type: verifies
---

# TC-374: Stochastic delays, timed sampling and exact digital checking

## Description

Sample stochastic timed models with exact rational delays, and check probabilistic timed automata exactly through digital clocks.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Rpc`**, **`NoLateReply`** and **`Settles`** (from QSL ADR-026 §11). `time Rpc dense unit ms;`. One object `c: Call { phase: Phase, inflight: Boolean, late: Boolean, x: Clock }`, `Phase` one of `Idle`, `Waiting`, `Replied` and `TimedOut`; universe `{c}`; initially `phase = Idle`, `inflight` and `late` false, `x = 0`. `Call::send` has pre `phase = Idle` and post `phase = Waiting`, `inflight`, `not late` and `x = 0`. `Call::reply` has pre `inflight and x >= 1 ms` and post `not inflight`, `late = (pre(phase) = TimedOut)`, and `phase = Replied` when `pre(phase) = Waiting`, unchanged otherwise. `Call::timeout` has pre `phase = Waiting and x >= T` and post `phase = TimedOut`. `Call::reset` has pre `not inflight and (phase = Replied or phase = TimedOut)` and post `phase = Idle`. The time invariants are `x <= 3 ms` while `inflight` and `x <= T` while `phase = Waiting`. `T` is a time constant each vector fixes. `NoLateReply` is `temporal NoLateReply using inf over (k: Rpc::Call) clock "model-steps" on origin { always holds(not k.late) }`. `Settles` is `temporal Settles using timed over (k: Rpc::Call) clock "model-time" on origin { always (holds(k.phase = Waiting) implies eventually[0 ms, 3 ms] holds(k.phase != Waiting)) }`.
- **`Retx`**, **`Deadline`** and **`MeanTime`** (from QSL ADR-028 §15.5). `time Retx dense unit ms;`. One object `m: Msg { delivered: Boolean, x: Clock }`, initially `delivered = false` and `x = 0`. `Msg::send(lost)` has pre `not delivered and x >= 1 ms` and post `delivered = not lost and x = 0`, with `random Msg::send(lost) ~ { true: 1, false: 9 };` and no `delay` declaration. The time invariant is `x <= 2 ms` while `not delivered`. A state with `delivered` is terminal. `Deadline` is `probabilistic Deadline using timed on Retx under every scheduler on origin { probability >= 0.99 [ eventually[0 ms, 4 ms] holds(m.delivered) ] }`. `MeanTime` is `probabilistic MeanTime using timed on Retx under every scheduler on origin { expected elapsed until holds(m.delivered) <= 3 ms }`.
- **`RareLate`** (from QSL ADR-026 §11). `Rpc` with `T = 3 ms`, starting from the state just after `send` (`phase = Waiting`, `inflight`, `late` false, `x = 0`), with `delay Call::reply ~ discrete { 1 ms: 90, 3 ms: 10 };`, `delay Call::timeout ~ uniform;`, `delay Call::send ~ exponential(rational(1, 1000));`, `delay Call::reset ~ exponential(rational(1, 1000));` and `workload Even on Rpc { weight Call::send = 1; weight Call::reply = 1; weight Call::timeout = 1; weight Call::reset = 1; }`. Its claim is `probabilistic RareLate using timed on Rpc under Even on origin with alpha 0.01, beta 0.01, indifference 0.005 { probability <= 0.01 [ eventually[0 ms, 3 ms] holds(c.late) ] }`, run with seed `20261001`. `timeout`'s window from the start is the single point 3 ms, so the event holds exactly when `reply` draws 3 ms and the tie is ordered `timeout` first: its exact probability is `1/10 · 1/2 = 1/20`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| ST-01 | `RareLate`, SPRT | Exact event probability `1/20`; `measured: rejected`; every sampled delay is an exact rational and each sample replays with its delays |
| ST-02 | `RareLate` with `delay Call::reply ~ uniform` | Event probability 0; `measured: accepted`; the every-behaviour `NoLateReply` stays `refuted` |
| ST-03 | `RareLate` with no `delay` declaration on `timeout`, statistical | `unsupported`, `not-stochastic` |
| ST-04 | A state with bounded delay and no racing identity; an identity with an unbounded window and `uniform` | `unsupported`, `not-stochastic{state, identity}` |
| ST-05 | `RareLate` with `max_sample_steps` 1; `RareLate` with `max_sample_steps` omitted | `incomplete`, `limit-reached` naming `max_sample_steps`, value 1; the run uses `2^24` and completes |
| ST-06 | `RareLate` with `q = 16` and `q = 32` | Each basis records its `q`; delays lie on the stated grid |
| ST-07 | `Lock`: `time Lock dense unit ms;`, one object `c: Gate { phase: P, x: Clock }`, `P` one of `A` and `B`, initially `phase = A` and `x = 0`, `x` never reset; `Gate::go` has pre `phase = A` and post `phase = B`, with `delay Gate::go ~ discrete { 1/2 ms: 1 };`; `time invariant when phase = B { x <= 1 ms }`; no operation is enabled in `B`; `workload One on Lock { weight Gate::go = 1; }`; claim `probabilistic Late using timed on Lock under One on origin with alpha 0.01, beta 0.01, indifference 0.005 { probability <= 0.5 [ eventually[0 ms, 2 ms] holds(c.phase = B) ] }`, statistical | The sample takes `go` at `1/2` ms and delays to `(B, x = 1 ms)`, where no positive delay is admissible and no identity is enabled; `unsupported`, `time-locked-sample{state}` naming that state; no witness kept |
| DG-01 | `Retx` `Deadline`: `probability >= 0.99 [eventually[0 ms, 4 ms] holds(m.delivered)]` under every scheduler | `proved`, `exact-value{99/100}` at equality; the maximum is `9999/10000` |
| DG-02 | `Deadline` at 0.995 | `refuted`; a one-path set of probability `1/100` with losses at 2 and 4 ms, whose last state needs time past 4 ms for the next `send` |
| DG-03 | `Retx` `MeanTime`: `expected elapsed until holds(m.delivered) <= 3 ms` under every scheduler | `proved`, `exact-value{20/9 ms}`; the minimum is `10/9 ms` |
| DG-04 | `Retx` with guard `x > 1 ms`; `RareLate` exact; `Retx` with a `delay` declaration under every scheduler; a reachable cycle of discrete moves; an unadmitted timed form | `strict-clock-constraint`; `delay-distribution`; `delay-distribution`; `zero-delay-cycle`; `timed-form-shape`, each `unsupported` |
| DG-05 | `RareLate` with every delay `discrete` (`send` and `reset` `discrete { 1000 ms: 1 }`, `timeout` `discrete { 3 ms: 1 }`), exact | The subject under `Even` is a Markov chain; `refuted` with value `1/20` |
| DG-06 | DG-05 with no `delay` declaration on `reply`, exact | `reply`'s delay is the scheduler's choice in `[1 ms, 3 ms]` after `timeout`'s draw of 3 ms. A delay below 3 ms lets `reply` win, so the event fails; a delay of 3 ms ties, and `Even` orders `timeout` first with probability `1/2`. The maximum is `1/2` and the minimum 0, so `probability <= 0.01` settles `refuted` with value `1/2`, and the evidence carries a witness scheduler (FR-413) that takes `reply`'s delay 3 ms and a one-path set (`timeout` then `reply`, both at 3 ms) of probability `1/2`. The same counterexample without `scheduler` refuses `invalid_runtime_input`/`missing-member` |

## Expected Results

Every measurement, exact value, verdict and cause is exactly as stated.
