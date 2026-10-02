---
id: TC-331
title: "Possible claims on the ConfigVersion subject"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-390
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-391
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-392
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-393
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-394
    type: verifies
---

# TC-331: Possible claims on the ConfigVersion subject

## Description

Check `possible` on the modified ConfigVersion unit, whose `attemptUpdate` advances `versionNumber` modulo 3, with universe `config_history = {a, b}` and one initial snapshot `(0, 0)`. The graph has 9 nodes and 18 edges, every node closed.

## Test Procedure

Run each vector through the real local Rust boundary with the explicit-state engine, then replay every returned evidence value through the replay boundary with no engine present. Each refutation and each witnessed proof is checked by replaying its evidence, never by comparing it with one expected evidence value; the verdict, basis, settlement method and cause are compared exactly.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| PV-01 | `possible ReachesTwo using v over (c: Config::ConfigVersion) { c.versionNumber = 2 }`, `witness_samples` 0 | each instance `proved`, `decisive-witness`; source `explored`; the canonical explored witness from `(0, 0)` for `c = a` is `upd(a)`, `upd(a)`; replay evaluates `P` true at the last node |
| PV-02 | PV-01 with `witness_samples` 64 and `seed` 0 | each instance `proved`, `decisive-witness`; every source `sampled` with seed 0 and a trace index below 64; replay reproduces |
| PV-03 | `possible ReachesThree … { c.versionNumber = 3 }` | each instance `refuted`, `closed-scope`; a `trap` with an empty stem at `(0, 0)`; replay explores the closure afresh, 9 nodes, with no target node |
| PV-04 | the subject's deadlock-freedom item | `proved`, `closed-scope` |
| PV-05 | PV-03's trap evidence with its `initial` index 1 | refused `invalid_runtime_input`/`invalid-value`: the subject has one initial state |

## Expected Results

Every `proved` and `refuted` result settles only after its evidence replays, the settlement method names the witness source of every initial state, and PV-01 and PV-02 settle the same verdict.
