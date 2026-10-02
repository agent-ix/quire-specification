---
id: TC-360
title: "Exact finite-horizon values"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-413
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-407
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-411
    type: verifies
---

# TC-360: Exact finite-horizon values

## Description

Compute finite-horizon probabilities and quantiles exactly or as sound dyadic intervals, and settle proofs and refutations.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Health`** and **`NoFault`** (from QSL ADR-024 §7.1). One object `n: Node { healthy: Boolean }`, initially `healthy = true`. Operation `Node::tick(fault)` has post `healthy = pre(healthy) and not fault`, with `random Node::tick(fault) ~ { true: 1, false: 9999999 };` and `workload Steady on Health { weight Node::tick = 1; }`. `NoFault` is `probabilistic NoFault using ev on Health under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0005 { probability >= 0.999 [ always[0, 1000] holds(n.healthy) ] }`. Its exact value is `(9999999/10000000)^1000`.
- **`Service`** and **`P95`** (from QSL ADR-024 §7.2). One object `s: Server { phase: Phase, attempts: Int[0, 2] }`, `Phase` one of `Idle`, `Busy` and `Done`, initially `phase = Idle` and `attempts = 0`. `Server::request` has pre `phase = Idle` and post `phase = Busy and attempts = 0`. `Server::attempt(d, outcome)` has pre `phase = Busy` and post `attempts = pre(attempts) + 1`, with `phase = Done` when `outcome = Ok` or `pre(attempts) = 1` and `phase = Busy` otherwise. `Server::reset` has pre `phase = Done` and post `phase = Idle`. The declarations are `random Server::attempt(d) ~ { 1 ms: 90, 3 ms: 7, 12 ms: 3 };`, `random Server::attempt(outcome) ~ { Ok: 98, Fail: 2 };`, `reward duration on Server::attempt = d;` and `workload Steady on Service { weight Server::request = 1; weight Server::attempt = 1; weight Server::reset = 1; }`. One operation is enabled at each state. `M` is the measure `accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10`. `P95` is `probabilistic P95 using ev on Service under Steady on origin with alpha 0.01, beta 0.01, indifference 0.01 { quantile 0.95 of (accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10) <= 5 ms }`. `Pr(M <= 5 ms) = 24233/25000` and `Pr(M <= 2 ms) = 4491/5000`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| EF-01 | `NoFault` with `max_rational_bits` at least 23,254 | `proved`, `exact-value{(9999999/10000000)^1000}`; the claim's confidence parameters are not read |
| EF-02 | `NoFault` with `max_rational_bits` 1024 and `precision_bits` 64 | `proved`, `value-bounds{lower, upper, backward-induction{64}}` with width at most `2 · 1001 · 2^-64` |
| EF-03 | `P95` at `5 ms` | `proved`, `exact-value{24233/25000}` |
| EF-04 | `P95` at `2 ms` | `refuted`, value `4491/5000`, with FR-413 evidence |
| EF-05 | EF-03's result | Truth `satisfied`, basis `closed-scope`, no `measured` value |
| EF-06 | `Two` is one object `t: Box { done: Boolean }`, initially `done = false`, with `Box::go(d)` (pre `not done`, post `done`), `random Box::go(d) ~ { 1: 1, 3: 1 };`, `reward cost on Box::go = d;` and `workload W on Two { weight Box::go = 1; }`; `M` is `accumulate cost from holds(not t.done) until holds(t.done) within 1`, which is 1 or 3 with probability `1/2` each; `quantile 0.5 of M >= 3` and `quantile 0.5 of M >= 1`, each under `W` and under every scheduler | `refuted`, value `Pr(M < 3) = 1/2` equal to `q`, under both clauses, with a one-path set `go(d = 1)` (`V = 1/2`, `N = 0`) that replays by the FR-413 quantile rule (over every scheduler by the zero-maximum second pass, whose restricted process activates with probability 1); `proved`, `exact-value{0}` for `Pr(M < 1)` under both |
| EF-07 | EF-06's `Two` with `M` changed to `accumulate cost from holds(t.done and not t.done) until holds(t.done) within 1`, which never activates; `quantile 0.5 of M >= 3` and `quantile 0.5 of M <= 1`, each under `W` and under every scheduler | Activation probability 0; each claim settles `proved` under both clauses |

## Expected Results

Values are the exact rationals stated and every verdict carries its evidence.
