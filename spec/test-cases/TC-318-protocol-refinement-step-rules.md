---
id: TC-318
title: "Protocol refinement over the model-refinement step rules"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-177
    type: verifies
---

# TC-318: Protocol refinement over the model-refinement step rules

## Description

Decide protocol refinement with FR-377's step function, internal actions as
`stutter` steps, and divergence-freedom independent of fairness.

## Test Procedure

Specification protocol: one visible step `ack` from `idle` to `done`.
Implementation protocol: internal step `prep` from `idle` to `ready`,
visible `ack` from `ready` to `done`, and internal `spin`, a self-loop at
`ready`, under a profile that makes divergence observable.

1. Check the implementation with `spin` removed.
2. Check it with `spin`, under an empty fairness set and under
   `fair weak whole ack` on the implementation.
3. Encode the same two models as state models with a model refinement:
   `prep -> stutter`, `spin -> stutter`, `ack -> ack(self)`, with `F_A`
   empty, and check.

## Expected Results

Case 1 refines: `prep` passes as a `stutter` step and `ack` matches the
specification's `ack` by observation. Case 2 refutes with a divergence
counterexample (the `spin` loop at `ready`, whose specification state can
move) under both fairness sets. Case 3 holds: the model refinement reads the
stutter tail through empty abstract fairness. The same step function and
counterexample type serve all three (FR-177-AC-6).
