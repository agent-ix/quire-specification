---
id: TC-356
title: "Regenerative long-run fractions and statistical budgets"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-408
    type: verifies
---

# TC-356: Regenerative long-run fractions and statistical budgets

## Description

Measure long-run fractions by regeneration with asymptotic coverage, measure a per-window claim, and stop runs at budgets.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. Each statistical vector uses seed `20261001`; run it twice on two platforms and require identical sample counts, estimates and decisions.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Health`** and **`NoFault`** (from QSL ADR-024 §7.1). One object `n: Node { healthy: Boolean }`, initially `healthy = true`. Operation `Node::tick(fault)` has post `healthy = pre(healthy) and not fault`, with `random Node::tick(fault) ~ { true: 1, false: 9999999 };` and `workload Steady on Health { weight Node::tick = 1; }`. `NoFault` is `probabilistic NoFault using ev on Health under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0005 { probability >= 0.999 [ always[0, 1000] holds(n.healthy) ] }`. Its exact value is `(9999999/10000000)^1000`.
- **`Avail`**, **`LongRun`** and **`Monthly`** (from QSL ADR-024 §7.3). One object `v: Server { up: Boolean }`, initially `up = true`. `Server::tick(fail)` has pre `up` and post `up = not fail`; `Server::repair(ok)` has pre `not up` and post `up = ok`. The declarations are `random Server::tick(fail) ~ { true: 1, false: 1999 };`, `random Server::repair(ok) ~ { true: 9, false: 1 };` and `workload Steady on Avail { weight Server::tick = 1; weight Server::repair = 1; }`. `LongRun` is `probabilistic LongRun using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0002 { long_run_fraction holds(v.up) >= 0.999 }`, exact value `1800/1801`. `Monthly` is `probabilistic Monthly using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.005 { probability >= 0.99 [ fraction holds(v.up) over [0, 10000] >= rational(999, 1000) ] }`, whose probability is about `0.959`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| RG-01 | `LongRun`, `indifference 0.0002`, regenerative, `min_cycles` 1000 | Stops once the half-width is at most `0.0002`; the interval lies above `0.999`; `measured: accepted`, coverage `asymptotic`, `min_cycles` recorded |
| RG-02 | `Monthly`, `indifference 0.005`, SPRT | `measured: rejected` (true value about `0.959`), coverage `finite-sample` |
| RG-03 | A long-run claim from a transient initial state that never recurs | Stops at `max_cycle_steps`; `incomplete`, `limit-reached{limit, value, setting}` naming `max_cycle_steps`, its value and the request setting that raises it; no `measured` value |
| RG-04 | SC-01 with `max_samples` 1000 | `incomplete`, `limit-reached` naming `max_samples`, value 1000, and the request setting that raises it; no `measured` value |
| RG-05 | Fixture `Up`: one object `v: Server { up: Boolean }`, initially `up = true`, one operation `Server::tick` with post `up = pre(up)`, `workload Steady on Up { weight Server::tick = 1; }`; claim `long_run_fraction holds(v.up) <= 0.9995` with `alpha 0.01, beta 0.01, indifference 0.0002`, regenerative, `min_cycles` 1000 | Every cycle has length 1 and ratio 1, so `S² = 0` and `Â = 1`; the half-width is `z / n` with `z = z_{0.99} ≈ 2.3263478740`, rounded outward. At `n = 1000` it is about `0.00233`, above `ι`, so the run does not stop at `min_cycles`. It stops at exactly `n = 11632` cycles, the least `n` with `z / n <= 1/5000` (`z / ι ≈ 11631.74`), with the interval above `0.9995`: `measured: rejected`, coverage `asymptotic` |
| RG-06 | RG-01 cancelled by the caller after 500 cycles | `incomplete`, `cancelled{source: "requested"}`; not `limit-reached`; no `measured` value |

## Expected Results

Coverage labels, decisions and stop causes are exactly as stated.
