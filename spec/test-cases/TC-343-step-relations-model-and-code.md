---
id: TC-343
title: "Step relations: model claims and code claims"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-397
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-399
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-400
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-401
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-402
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
---

# TC-343: Step relations: model claims and code claims

## Description

Check a determinism step relation as a model claim over reachable transitions and as a separate code claim over invariant-satisfying pre-states.

## Test Procedure

Run each model-claim vector through the real local Rust boundary with the explicit-state engine; run each code-claim vector through the registered code-claim backend; replay every counterexample. Each refutation is checked by replaying its counterexample, never by comparing it with one expected counterexample; the verdict, basis, cause and form are compared exactly.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| SR-01 | TC-337 HB-11 on a model whose `step` may set `l` to either value | `refuted`, `decisive-counterexample`; a `step-tuple` counterexample, two prefix-plus-transition traces, that replays with the body false |
| SR-02 | HB-11 on the secure model | `proved`, `closed-scope` |
| SR-03 | a relation violated only from a pre-state that is not reachable | model claim `proved` |
| SR-04 | SR-01 with `max_relation_tuples` 1 | `incomplete`, cause `limit-reached` naming `max_relation_tuples`; with no value set it runs under the provider's published default |
| SR-05 | HB-11 with `V::Vault::step` bound to an implementation by an abstraction relation | two items: model claim `temporal-satisfaction`, code claim `operation-contract`, with different obligation identities |
| SR-06 | SR-05 against an implementation that is nondeterministic in `l` | code claim `refuted`; its counterexample's pre-states are admitted as snapshots, satisfy the invariants, and the body evaluates false with no reachability prefix |
| SR-07 | a code claim whose declared invariants no state satisfies | `inconclusive` with the vacuity cause, never `proved` |
| SR-08 | HB-11 with no implementation binding | only the model-claim item |

## Expected Results

A model claim and a code claim are separate obligations with separate evidence, and neither discharges the other.
