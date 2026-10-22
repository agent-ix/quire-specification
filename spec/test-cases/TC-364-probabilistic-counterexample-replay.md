---
id: TC-364
title: "Probabilistic counterexample replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-413
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-411
    type: verifies
---

# TC-364: Probabilistic counterexample replay

## Description

Replay path-set and subsystem evidence, check witness schedulers, and refuse tampered evidence. Engines may return different valid witnesses; each vector checks that the returned evidence replays.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Service`** and **`P95`** (from QSL ADR-024 §7.2). One object `s: Server { phase: Phase, attempts: Int[0, 2] }`, `Phase` one of `Idle`, `Busy` and `Done`, initially `phase = Idle` and `attempts = 0`. `Server::request` has pre `phase = Idle` and post `phase = Busy and attempts = 0`. `Server::attempt(d, outcome)` has pre `phase = Busy` and post `attempts = pre(attempts) + 1`, with `phase = Done` when `outcome = Ok` or `pre(attempts) = 1` and `phase = Busy` otherwise. `Server::reset` has pre `phase = Done` and post `phase = Idle`. The declarations are `random Server::attempt(d) ~ { 1 ms: 90, 3 ms: 7, 12 ms: 3 };`, `random Server::attempt(outcome) ~ { Ok: 98, Fail: 2 };`, `reward duration on Server::attempt = d;` and `workload Steady on Service { weight Server::request = 1; weight Server::attempt = 1; weight Server::reset = 1; }`. One operation is enabled at each state. `M` is the measure `accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10`. `P95` is `probabilistic P95 using ev on Service under Steady on origin with alpha 0.01, beta 0.01, indifference 0.01 { quantile 0.95 of (accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10) <= 5 ms }`. `Pr(M <= 5 ms) = 24233/25000` and `Pr(M <= 2 ms) = 4491/5000`.
- **`Link`** and **`Deliver`** (from QSL ADR-028 §15.4). One object `m: Msg { delivered: Boolean, attempts: Int[0, 2] }`, initially `delivered = false` and `attempts = 0`. `Msg::send_a(lost)` and `Msg::send_b(lost)` each have pre `not delivered and attempts < 2` and post `attempts = pre(attempts) + 1 and delivered = not lost`, with `random Msg::send_a(lost) ~ { true: 1, false: 9 };` and `random Msg::send_b(lost) ~ { true: 1, false: 4 };`. A state with `delivered` or `attempts = 2` is terminal. `workload Even on Link { weight Msg::send_a = 1; weight Msg::send_b = 1; }`. `Deliver` is `probabilistic Deliver using ev on Link under every scheduler on origin { probability >= 0.97 [ eventually[0, 2] holds(m.delivered) ] }`. Its minimum is `24/25`, its maximum `99/100`, and its value under `Even` `391/400`.
- **`Avail`**, **`LongRun`** and **`Monthly`** (from QSL ADR-024 §7.3). One object `v: Server { up: Boolean }`, initially `up = true`. `Server::tick(fail)` has pre `up` and post `up = not fail`; `Server::repair(ok)` has pre `not up` and post `up = ok`. The declarations are `random Server::tick(fail) ~ { true: 1, false: 1999 };`, `random Server::repair(ok) ~ { true: 9, false: 1 };` and `workload Steady on Avail { weight Server::tick = 1; weight Server::repair = 1; }`. `LongRun` is `probabilistic LongRun using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0002 { long_run_fraction holds(v.up) >= 0.999 }`, exact value `1800/1801`. `Monthly` is `probabilistic Monthly using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.005 { probability >= 0.99 [ fraction holds(v.up) over [0, 10000] >= rational(999, 1000) ] }`, whose probability is about `0.959`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| PC-01 | `P95` at `2 ms` refutation | A path set reaching past `1/20`, for example one path `request`, `attempt(d = 3 ms, Ok)` of probability `343/5000`; replay checks supports and recomputes the probability |
| PC-02 | `Deliver` at 0.97 refutation | A path set reaching past `3/100`, for example `send_b` lost twice with probability `1/25` under a witness taking `send_b` |
| PC-03 | `Monthly` exact refutation | A subsystem certificate the FR-412 checker accepts |
| PC-04 | A path set with one path a prefix of another; a step that departs from its scheduler; a drawn value outside its support | Each refuses `invalid_runtime_input`/`invalid-value` |
| PC-05 | A path set whose recomputed sum does not pass the bound | `inconclusive`, `replay-parity` |
| PC-06 | A refutation of a claim with a fairness set, and a variant whose witness induces an unfair bottom component | Path probabilities include the witness's choice probabilities and replay accepts; the unfair variant settles `inconclusive`, `replay-parity` |
| PC-07 | `Lookup` of TC-359 on the Flaky subject with exact evidence | `refuted`, `decisive-counterexample`, cause `undefined-evaluation` naming the initial state, the product state after `step(ok = false)`, the `lookup`'s locus and `absent-key`, although the bound holds on the defined values; the evidence is an undefined path of the one step `step(ok = false)` with probability `1/10`, and replay reproduces the undefined value |
| PC-08 | PC-07's undefined path with its step's drawn value `ok = true` and the matching post-state digest | The claim is defined at the last state: `inconclusive`, `replay-parity` |
| PC-09 | The `quantile 0.5 of M >= 3` refutation on TC-360's `Two` subject, a path set of the one path `go(d = 1)` | `V = 1/2`, `N = 0`; `1/2 >= 1/2 · (1 − 0)` holds with equality, so replay settles `refuted`, `decisive-counterexample` |
| PC-10 | A `quantile 0.5 of M >= 3` refutation on a subject that activates with probability `1/2` and, once activated, has `M = 1` or `M = 3` with probability `1/2` each: a path set of the violating path (mass `1/4`) and the not-activated path (mass `1/2`); then the violating path alone | `V = 1/4`, `N = 1/2`, and `1/4 >= 1/2 · (1 − 1/2)`: `refuted`. Alone, `N = 0` and `1/4 < 1/2`: `inconclusive`, `replay-parity` |

## Expected Results

Every replay outcome is exactly as stated.
