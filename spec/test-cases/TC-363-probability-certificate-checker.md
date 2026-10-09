---
id: TC-363
title: "Probability certificate checker"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-412
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-363: Probability certificate checker

## Description

Accept valid certificates and reject or refuse invalid ones, with the checker recomputing every qualitative set itself.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. Each mutation starts from the EC-01 certificate.

## Models

The models below are defined here in full; the citation names the QSL design record each one comes from. `ev` selects `quire.temporal.event-position.false-extension/v1`, `inf` selects `quire.temporal.infinite-trace/v1` and `timed` selects `quire.temporal.timed/v1`.

- **`Link`** and **`Deliver`** (from QSL ADR-028 §15.4). One object `m: Msg { delivered: Boolean, attempts: Int[0, 2] }`, initially `delivered = false` and `attempts = 0`. `Msg::send_a(lost)` and `Msg::send_b(lost)` each have pre `not delivered and attempts < 2` and post `attempts = pre(attempts) + 1 and delivered = not lost`, with `random Msg::send_a(lost) ~ { true: 1, false: 9 };` and `random Msg::send_b(lost) ~ { true: 1, false: 4 };`. A state with `delivered` or `attempts = 2` is terminal. `workload Even on Link { weight Msg::send_a = 1; weight Msg::send_b = 1; }`. `Deliver` is `probabilistic Deliver using ev on Link under every scheduler on origin { probability >= 0.97 [ eventually[0, 2] holds(m.delivered) ] }`. Its minimum is `24/25`, its maximum `99/100`, and its value under `Even` `391/400`.
- **`Coin`**, **`Terminates`** and **`FairTerminates`** (from QSL ADR-028 §15.6). One object `p: Proc { done: Boolean }`, initially `done = false`. `Proc::flip(heads)` has pre `not done` and post `done = heads`, with `random Proc::flip(heads) ~ { true: 1, false: 1 };`. `Proc::wait` has pre `not done` and post `done = pre(done)`. A state with `done` is terminal. `Terminates` is `probabilistic Terminates using inf on Coin under every scheduler on origin { probability >= 0.99 [ eventually holds(p.done) ] }`. `FairTerminates` is the same claim with `under every scheduler fair strong Proc::flip`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| EC-01 | `Link` `Deliver` at 0.95, `exact` certificate with values `24/25`, `4/5`, 1, 0 | Accepted; the item settles `proved` |
| EC-02 | EC-01 with the value at the one-loss state raised to `9/10` | `inconclusive`, `certificate-rejected` with rule `inequality-fails` at `{state}`, the one-loss state |
| EC-03 | A `lower` certificate with one rank missing; one with a non-decreasing rank; one without `ranking`; an `exact` certificate with EC-01's values for `Deliver` at 0.99, carrying that item's identity | `certificate-rejected` with rule `rank-missing` at the unranked state; `rank-broken` at the state whose rank does not decrease; `malformed` at member `ranking`; `threshold-missed` at the initial state, whose value `24/25` is below the threshold |
| EC-04 | EC-01 with another item's identity | Refused `invalid_runtime_input`/`conflicting-identity` |
| EC-05 | A `lower` certificate naming a subset of states | Checked over the named states and their successors only, unnamed successors at value 0 |
| EC-06 | `FairTerminates` `exact` certificate whose objective names the fairness set | Accepted after the checker recomputes that no fair end component exists |

## Expected Results

Each certificate is accepted, rejected or refused exactly as stated.
