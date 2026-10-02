---
id: TC-365
title: "Exact limits, precision budget and partial evidence"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-411
    type: verifies
---

# TC-365: Exact limits, precision budget and partial evidence

## Description

Stop exact runs at budgets with named causes, and settle from evidence found before a stop.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Avail`**, **`LongRun`** and **`Monthly`** (from QSL ADR-024 §7.3). One object `v: Server { up: Boolean }`, initially `up = true`. `Server::tick(fail)` has pre `up` and post `up = not fail`; `Server::repair(ok)` has pre `not up` and post `up = ok`. The declarations are `random Server::tick(fail) ~ { true: 1, false: 1999 };`, `random Server::repair(ok) ~ { true: 9, false: 1 };` and `workload Steady on Avail { weight Server::tick = 1; weight Server::repair = 1; }`. `LongRun` is `probabilistic LongRun using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0002 { long_run_fraction holds(v.up) >= 0.999 }`, exact value `1800/1801`. `Monthly` is `probabilistic Monthly using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.005 { probability >= 0.99 [ fraction holds(v.up) over [0, 10000] >= 0.999 ] }`, whose probability is about `0.959`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| XL-01 | A claim whose value is within `2^-80` of its threshold, `max_rational_bits` 64, `max_precision_bits` 64, `max_policy_iterations` 0 | `incomplete`, `precision-budget{lower, upper}` with the straddling interval |
| XL-02 | `Monthly` with `max_states` 1000 | `incomplete`, `limit-reached` naming `max_states`, value 1000, and the request setting that raises it |
| XL-03 | A `>= θ` claim whose lower certificate on the explored part already passes `θ` when `max_states` stops exploration | `proved` from the partial evidence |
| XL-04 | A `<= θ` claim whose path set on the explored part already passes `θ` when a limit stops exploration | `refuted` from the partial evidence |
| XL-05 | `Monthly` cancelled by the caller with cancellation identity `op-17` before its backward induction completes | `incomplete`, `cancelled{source: "op-17"}`; not `limit-reached`; no value |

## Expected Results

Each stop and partial settlement is exactly as stated.
