---
id: TC-351
title: "Step probability, Markov condition and measure per initial state"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-406
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-407
    type: verifies
---

# TC-351: Step probability, Markov condition and measure per initial state

## Description

Compute exact step probabilities and event probabilities under a workload, settle `not-markov`, and evaluate a claim per initial state and binding.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. Values are compared as exact rationals in lowest terms.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Health`** and **`NoFault`** (from QSL ADR-024 §7.1). One object `n: Node { healthy: Boolean }`, initially `healthy = true`. Operation `Node::tick(fault)` has post `healthy = pre(healthy) and not fault`, with `random Node::tick(fault) ~ { true: 1, false: 9999999 };` and `workload Steady on Health { weight Node::tick = 1; }`. `NoFault` is `probabilistic NoFault using ev on Health under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0005 { probability >= 0.999 [ always[0, 1000] holds(n.healthy) ] }`. Its exact value is `(9999999/10000000)^1000`.
- **`Service`** and **`P95`** (from QSL ADR-024 §7.2). One object `s: Server { phase: Phase, attempts: Int[0, 2] }`, `Phase` one of `Idle`, `Busy` and `Done`, initially `phase = Idle` and `attempts = 0`. `Server::request` has pre `phase = Idle` and post `phase = Busy and attempts = 0`. `Server::attempt(d, outcome)` has pre `phase = Busy` and post `attempts = pre(attempts) + 1`, with `phase = Done` when `outcome = Ok` or `pre(attempts) = 1` and `phase = Busy` otherwise. `Server::reset` has pre `phase = Done` and post `phase = Idle`. The declarations are `random Server::attempt(d) ~ { 1 ms: 90, 3 ms: 7, 12 ms: 3 };`, `random Server::attempt(outcome) ~ { Ok: 98, Fail: 2 };`, `reward duration on Server::attempt = d;` and `workload Steady on Service { weight Server::request = 1; weight Server::attempt = 1; weight Server::reset = 1; }`. One operation is enabled at each state. `M` is the measure `accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10`. `P95` is `probabilistic P95 using ev on Service under Steady on origin with alpha 0.01, beta 0.01, indifference 0.01 { quantile 0.95 of (accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10) <= 5 ms }`. `Pr(M <= 5 ms) = 24233/25000` and `Pr(M <= 2 ms) = 4491/5000`.
- **`Avail`**, **`LongRun`** and **`Monthly`** (from QSL ADR-024 §7.3). One object `v: Server { up: Boolean }`, initially `up = true`. `Server::tick(fail)` has pre `up` and post `up = not fail`; `Server::repair(ok)` has pre `not up` and post `up = ok`. The declarations are `random Server::tick(fail) ~ { true: 1, false: 1999 };`, `random Server::repair(ok) ~ { true: 9, false: 1 };` and `workload Steady on Avail { weight Server::tick = 1; weight Server::repair = 1; }`. `LongRun` is `probabilistic LongRun using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0002 { long_run_fraction holds(v.up) >= 0.999 }`, exact value `1800/1801`. `Monthly` is `probabilistic Monthly using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.005 { probability >= 0.99 [ fraction holds(v.up) over [0, 10000] >= rational(999, 1000) ] }`, whose probability is about `0.959`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| SP-01 | `Service` under `Steady` (one operation enabled per state), `d ~ {1 ms: 90, 3 ms: 7, 12 ms: 3}`, `outcome ~ {Ok: 98, Fail: 2}` | `Pr(M <= 1 ms) = 441/500`, `Pr(M <= 2 ms) = 4491/5000`, `Pr(M <= 3 ms) = 2417/2500`, `Pr(M <= 5 ms) = 24233/25000` for the accumulate measure of `duration` from `Busy` until `Done` within 10 |
| SP-02 | A state with operations `a` (weight 1, two enabled identities) and `b` (weight 3, one identity), no random parameters | Each identity of `a` has step probability `1/8`; `b`'s has `3/4` |
| SP-03 | A variant of `Health` whose `tick` with `fault = false` has two post-states, under `Steady` | `unsupported`, `not-markov{state, transition, post_states}` naming both post-states |
| SP-04 | A drawn vector with no post-state, under `Steady` and under every scheduler | `unsupported`, `not-markov` under both clauses |
| SP-05 | A subject with two initial states `s0`, `s1` where the bound holds from `s0` only | The claim does not hold; the result reports one entry per initial state |
| SP-06 | A weak fairness constraint over `Avail` under `Steady`, and an every-behaviour liveness claim FR-161 proves on `Avail` | The constraint holds with probability 1; the proved claim holds on every positive-probability behaviour |

## Expected Results

Every probability is the exact rational stated; `not-markov` carries its full payload; a claim holds only when it holds from every initial state and binding.
