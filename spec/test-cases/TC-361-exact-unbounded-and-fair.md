---
id: TC-361
title: "Exact unbounded reachability, expected rewards and fairness sets"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-411
    type: verifies
---

# TC-361: Exact unbounded reachability, expected rewards and fairness sets

## Description

Settle every-scheduler bounds, unbounded reachability, expected rewards and claims with fairness sets.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Link`** and **`Deliver`** (from QSL ADR-028 §15.4). One object `m: Msg { delivered: Boolean, attempts: Int[0, 2] }`, initially `delivered = false` and `attempts = 0`. `Msg::send_a(lost)` and `Msg::send_b(lost)` each have pre `not delivered and attempts < 2` and post `attempts = pre(attempts) + 1 and delivered = not lost`, with `random Msg::send_a(lost) ~ { true: 1, false: 9 };` and `random Msg::send_b(lost) ~ { true: 1, false: 4 };`. A state with `delivered` or `attempts = 2` is terminal. `workload Even on Link { weight Msg::send_a = 1; weight Msg::send_b = 1; }`. `Deliver` is `probabilistic Deliver using ev on Link under every scheduler on origin { probability >= 0.97 [ eventually[0, 2] holds(m.delivered) ] }`. Its minimum is `24/25`, its maximum `99/100`, and its value under `Even` `391/400`.
- **`Coin`**, **`Terminates`** and **`FairTerminates`** (from QSL ADR-028 §15.6). One object `p: Proc { done: Boolean }`, initially `done = false`. `Proc::flip(heads)` has pre `not done` and post `done = heads`, with `random Proc::flip(heads) ~ { true: 1, false: 1 };`. `Proc::wait` has pre `not done` and post `done = pre(done)`. A state with `done` is terminal. `Terminates` is `probabilistic Terminates using inf on Coin under every scheduler on origin { probability >= 0.99 [ eventually holds(p.done) ] }`. `FairTerminates` is the same claim with `under every scheduler fair strong Proc::flip`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| EU-01 | `Link` `Deliver` at 0.97 over every scheduler | `refuted` (minimum `24/25`) |
| EU-02 | `Deliver` at 0.95; `Deliver` under `Even` | `proved`, `exact-value{24/25}`; `proved`, `exact-value{391/400}` |
| EU-03 | An unbounded reachability claim whose value equals its non-strict threshold | `proved` through exact policy iteration |
| EU-04 | `expected accumulate cost until holds(B) <= 10` where some scheduler reaches `B` with probability `1/2` | `refuted`: the maximum exceeds every finite threshold |
| EU-05 | `Coin` `Terminates`; `FairTerminates` with `fair strong Proc::flip`; with `fair weak Proc::flip` | `refuted` with minimum 0; `proved`, `exact-value{1}`; `proved`, `exact-value{1}` |
| EU-06 | A bounded event with a fairness set | Value equal to the unrestricted minimum; the result records the fairness set |

## Expected Results

Verdicts and values are exactly as stated.
