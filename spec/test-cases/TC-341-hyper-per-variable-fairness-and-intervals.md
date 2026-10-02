---
id: TC-341
title: "Per-variable fairness and mixed interval bodies"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-395
    type: verifies
---

# TC-341: Per-variable fairness and mixed interval bodies

## Description

Check that each variable's fairness set filters only its own behaviours, and that a mixed interval body is evaluated over joint positions of a tuple of lassos.

## Test Procedure

Run each vector through the real local Rust boundary with the explicit-state engine and replay every counterexample with no engine present. Each refutation is checked by replaying its counterexample, never by comparing it with one expected counterexample; the verdict, basis, cause and form are compared exactly.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| PF-01 | a two-copy clause over a model with a Boolean field `done`, an always-enabled operation `work` that sets `done` and an always-enabled operation `idle` that changes nothing, body `always eventually holds(x.done @ a)`, `a` with `fair weak M::T::work;`, `b` with no constraint | `proved`: `a`'s fairness excludes behaviours that idle forever |
| PF-02 | PF-01 with the fairness set moved from `a` to `b` | `refuted`; the counterexample's `a` trace idles on its loop and its fairness is checked against `a`'s empty set |
| PF-03 | a clause whose body is `always (holds(p @ a) implies eventually[0,2] holds(q @ b))` on components with loop lengths 2 and 3 | evaluated over a joint lasso with stem the longer stem and period 6; the verdict and any counterexample replay |

## Expected Results

Fairness is read per variable on its own component, and interval operators count joint positions.
