---
id: TC-355
title: "Okamoto and SPRT decisions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-408
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-410
    type: verifies
---

# TC-355: Okamoto and SPRT decisions

## Description

Decide probability bounds and quantiles with the fixed-sample and sequential methods at the claim's confidence parameters, with exact rational bounds.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. Each statistical vector uses seed `20261001`; run it twice on two platforms and require identical sample counts, estimates and decisions. Each claim states `alpha 0.01, beta 0.01` and `m = 1`.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Health`** and **`NoFault`** (from QSL ADR-024 §7.1). One object `n: Node { healthy: Boolean }`, initially `healthy = true`. Operation `Node::tick(fault)` has post `healthy = pre(healthy) and not fault`, with `random Node::tick(fault) ~ { true: 1, false: 9999999 };` and `workload Steady on Health { weight Node::tick = 1; }`. `NoFault` is `probabilistic NoFault using ev on Health under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0005 { probability >= 0.999 [ always[0, 1000] holds(n.healthy) ] }`. Its exact value is `(9999999/10000000)^1000`.
- **`Service`** and **`P95`** (from QSL ADR-024 §7.2). One object `s: Server { phase: Phase, attempts: Int[0, 2] }`, `Phase` one of `Idle`, `Busy` and `Done`, initially `phase = Idle` and `attempts = 0`. `Server::request` has pre `phase = Idle` and post `phase = Busy and attempts = 0`. `Server::attempt(d, outcome)` has pre `phase = Busy` and post `attempts = pre(attempts) + 1`, with `phase = Done` when `outcome = Ok` or `pre(attempts) = 1` and `phase = Busy` otherwise. `Server::reset` has pre `phase = Done` and post `phase = Idle`. The declarations are `random Server::attempt(d) ~ { 1 ms: 90, 3 ms: 7, 12 ms: 3 };`, `random Server::attempt(outcome) ~ { Ok: 98, Fail: 2 };`, `reward duration on Server::attempt = d;` and `workload Steady on Service { weight Server::request = 1; weight Server::attempt = 1; weight Server::reset = 1; }`. One operation is enabled at each state. `M` is the measure `accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10`. `P95` is `probabilistic P95 using ev on Service under Steady on origin with alpha 0.01, beta 0.01, indifference 0.01 { quantile 0.95 of (accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10) <= 5 ms }`. `Pr(M <= 5 ms) = 24233/25000` and `Pr(M <= 2 ms) = 4491/5000`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| SC-01 | `NoFault`, `indifference 0.0005`, Okamoto | `N = 9,210,341`; the interval lies above 0.999; `measured: accepted`, coverage `finite-sample` |
| SC-02 | `NoFault`, SPRT | Increments `ln(0.9985/0.9995)` per holding sample and `ln 3` per failing sample as conservative rational bounds; thresholds `±ln 100`; with no failing sample it accepts after 4,601 samples; `measured: accepted` |
| SC-03 | `P95`, `indifference 0.01`, Okamoto | `N = 23,026`; estimate near `24233/25000` and empirical nearest-rank p95 `3 ms`; `measured: accepted` |
| SC-04 | `P95` with threshold `2 ms`, SPRT | `measured: rejected` (true value `4491/5000` below `0.94`); the pipeline fails; the result value is `measured`, never `refuted` |
| SC-05 | A quantile claim whose activation fails on some samples | Not-activated samples advance the trace index and count toward `max_draws` only |
| SC-06 | A `mean of` claim with method SPRT | `invalid-request` at negotiation, `invalid_capability`/`not-statistical` naming the item and the method; no sample is drawn |
| SC-07 | Every logarithm, square root and normal quantile used by SC-01 to SC-04 | Each is a pair of rational bounds within `2^-64` of the true value; `N` uses the upper logarithm bound, the SPRT lower and upper sums use the increments' lower and upper bounds, the rejection threshold its upper bound and the acceptance threshold its lower bound; the decisions are identical on both platforms |
| SC-08 | SC-03 rerun with the same seed; rerun with seed `20261002` | Identical samples and decision; a new, independent measurement with the same obligation identity |
| SC-09 | An SPRT run, by a fixture whose counts put the true sum within `2^-70` of `ln(1/α')`, so that the lower sum is below the threshold's upper bound | The test does not reject at that sample and keeps sampling; it rejects only once the lower sum reaches the threshold's upper bound |

## Expected Results

Sample counts, thresholds and decisions are exactly as stated; estimates and interval ends are exact rationals; reruns reproduce bit for bit.
