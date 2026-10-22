---
id: TC-362
title: "Exact long-run fractions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-411
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-412
    type: verifies
---

# TC-362: Exact long-run fractions

## Description

Compute long-run fractions by bottom components and check the gain–bias certificate.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Avail`**, **`LongRun`** and **`Monthly`** (from QSL ADR-024 §7.3). One object `v: Server { up: Boolean }`, initially `up = true`. `Server::tick(fail)` has pre `up` and post `up = not fail`; `Server::repair(ok)` has pre `not up` and post `up = ok`. The declarations are `random Server::tick(fail) ~ { true: 1, false: 1999 };`, `random Server::repair(ok) ~ { true: 9, false: 1 };` and `workload Steady on Avail { weight Server::tick = 1; weight Server::repair = 1; }`. `LongRun` is `probabilistic LongRun using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.0002 { long_run_fraction holds(v.up) >= 0.999 }`, exact value `1800/1801`. `Monthly` is `probabilistic Monthly using ev on Avail under Steady on origin with alpha 0.01, beta 0.01, indifference 0.005 { probability >= 0.99 [ fraction holds(v.up) over [0, 10000] >= rational(999, 1000) ] }`, whose probability is about `0.959`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| LR-01 | `LongRun` | `proved`, `exact-value{1800/1801}` with a `long-run` certificate: gain `1800/1801`, bias 0 at `up` and `−2000/1801` at `down` |
| LR-02 | The checker on LR-01's certificate | Accepts after checking both multichain equations by exact equality |
| LR-03 | A weighted long-run claim whose bottom component has every cycle of total weight 0 | `unsupported`, `zero-weight-component{component}` |

## Expected Results

Values and certificate checks are exactly as stated.
