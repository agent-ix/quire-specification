---
id: TC-352
title: "Every-scheduler and fair-scheduler semantics"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-406
    type: verifies
---

# TC-352: Every-scheduler and fair-scheduler semantics

## Description

Decide minimum and maximum over every scheduler, the scheduler's choice after a draw, and the infimum over schedulers fair for a stated fairness set.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Link`** and **`Deliver`** (from QSL ADR-028 §15.4). One object `m: Msg { delivered: Boolean, attempts: Int[0, 2] }`, initially `delivered = false` and `attempts = 0`. `Msg::send_a(lost)` and `Msg::send_b(lost)` each have pre `not delivered and attempts < 2` and post `attempts = pre(attempts) + 1 and delivered = not lost`, with `random Msg::send_a(lost) ~ { true: 1, false: 9 };` and `random Msg::send_b(lost) ~ { true: 1, false: 4 };`. A state with `delivered` or `attempts = 2` is terminal. `workload Even on Link { weight Msg::send_a = 1; weight Msg::send_b = 1; }`. `Deliver` is `probabilistic Deliver using ev on Link under every scheduler on origin { probability >= 0.97 [ eventually[0, 2] holds(m.delivered) ] }`. Its minimum is `24/25`, its maximum `99/100`, and its value under `Even` `391/400`.
- **`Coin`**, **`Terminates`** and **`FairTerminates`** (from QSL ADR-028 §15.6). One object `p: Proc { done: Boolean }`, initially `done = false`. `Proc::flip(heads)` has pre `not done` and post `done = heads`, with `random Proc::flip(heads) ~ { true: 1, false: 1 };`. `Proc::wait` has pre `not done` and post `done = pre(done)`. A state with `done` is terminal. `Terminates` is `probabilistic Terminates using inf on Coin under every scheduler on origin { probability >= 0.99 [ eventually holds(p.done) ] }`. `FairTerminates` is the same claim with `under every scheduler fair strong Proc::flip`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| ES-01 | `Link` (`send_a` delivers with `9/10`, `send_b` with `4/5`, at most two attempts), `eventually[0,2] holds(m.delivered)` | Minimum `24/25`, maximum `99/100`; under `Even` the value is `391/400`, and `17/20` after one loss |
| ES-02 | An action whose drawn vector has two post-states `p`, `q`, with the event true only at `p`, under every scheduler | The minimum takes `q` and the maximum `p`; the choice is made after the draw, per drawn vector |
| ES-03 | `Coin` `Terminates`: `probability >= 0.99 [eventually holds(p.done)]` under every scheduler | Minimum 0 (the scheduler that always takes `wait`) |
| ES-04 | `Coin` `FairTerminates` with `fair strong Proc::flip`, and with `fair weak Proc::flip` | Fair infimum 1 for both: the live state with `wait` is an end component that omits the enabled `flip`, so no fair end component remains |
| ES-05 | `FairTerminates` and `Terminates` compiled together | Two distinct obligation identities; the fairness set is part of the identity |

## Expected Results

Each value is the exact rational stated; fairness changes only claims that state a fairness set; the post-draw choice is resolved per drawn vector.
