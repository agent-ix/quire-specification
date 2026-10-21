---
id: TC-344
title: "Projection-aligned hyperproperty"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-398
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-400
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-401
    type: verifies
---

# TC-344: Projection-aligned hyperproperty

## Description

Check alignment on a projection with the busy/mix vault: field `busy: Boolean`; `step(i)` requires `not busy`, sets `l = i` and sets `busy` to `h = 1`; `mix` requires `busy`, clears it and leaves `l`. A run with secret 1 takes a `mix` after every `step`; a run with secret 0 never does.

## Test Procedure

Run each vector through the real local Rust boundary with the explicit-state engine and replay every counterexample with no engine present. Each refutation is checked by replaying its counterexample, never by comparing it with one expected counterexample; the verdict, basis, cause and form are compared exactly.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| PA-01 | noninterference matching only `step` arguments, in lockstep | `refuted`: lockstep pairs `a`'s second `step` with `b`'s `mix` |
| PA-02 | PA-01 with `align skip { V::Vault::mix }` | HP-6; `proved`, `closed-scope` |
| PA-03 | PA-02 on a leaky variant whose `step` sets `l = (i + h) mod 2` | `refuted`; a `projected` counterexample with per-step marks and equal visible counts, with total lengths free (fixture-dependent: see PA-05); replay evaluates the three-valued prefix false |
| PA-04 | two paths with the same visible steps and different numbers of `mix` steps | the same projected prefix and the same match values |
| PA-05 | PA-03 with the secret-1 run caught mid-operation at `(h, l, busy) = (1, 0, true)` and the other run idle at `(0, 0, false)`; then both runs idle, the secret-1 run at `(1, 0, false)` | the first: `refuted`, `projected` counterexample at projected position 1 with one visible step per trace and total lengths 1 and 2 (the mid-operation run takes a skipped `mix` first); the second: `refuted` by one visible step of each trace, total lengths 1 and 1; both replay |

## Expected Results

A secret that changes how many internal steps a run takes does not read as a leak once the internal operation is skipped.
