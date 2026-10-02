---
id: TC-338
title: "Noninterference in lockstep (universal hyperproperty)"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-395
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-399
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-400
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-401
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-191
    type: verifies
---

# TC-338: Noninterference in lockstep (universal hyperproperty)

## Description

Check the noninterference clause of TC-337 HB-01 on the leaky and secure vaults. The vault subject: model `V` with object type `Vault`, fields `h: Int[0, 1]` (secret) and `l: Int[0, 1]` (public output), population `vaults` with universe `{v}`, one operation `step(i: Int[0, 1])` with frame `modifies [l]` and no precondition, and initial snapshots `(h, l) = (0, 0)` and `(1, 0)`. The leaky model's post clause is `self.l = (i + self.h) mod 2`; the secure model's is `self.l = i`.

## Test Procedure

Run each vector through the real local Rust boundary with the explicit-state engine, unreduced, and replay every counterexample with no engine present. Each refutation is checked by replaying its counterexample, never by comparing it with one expected counterexample; the verdict, basis, cause and form are compared exactly.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| NI-01 | HB-01 on the leaky model | `refuted`, `decisive-counterexample`; a `lockstep` counterexample of two traces with equal step counts and one `loop_entry`, for example `a` from `(0, 0)` and `b` from `(1, 0)` with joint steps `(step(0), step(0))`; replay finds the body false at the failing joint position |
| NI-02 | HB-01 on the secure model | `proved`, `closed-scope`, exhaustive; 8 reachable product states |
| NI-03 | HB-01 without its `match` block, on the secure model | `refuted`: unmatched inputs give unequal `l` after one step |
| NI-04 | HB-01 on the secure model with maximum depth 1 | `inconclusive`, `bound-reached` naming depth 1, never `proved` |
| NI-06 | HB-01 on the secure model with the caller cancelling the run before exploration completes | `incomplete`, cause `cancelled`; FR-360 `failed`, execution `resource-incomplete`, basis `unavailable` |
| NI-05 | HB-01 on the leaky model, form classification | HP-2, kind `temporal-satisfaction` |

## Expected Results

The match is what makes the clause noninterference: the leaky model is refuted and the secure model proved only with it, and every refutation replays.
