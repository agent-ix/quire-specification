---
id: TC-357
title: "Bonferroni over tests and symmetry reduction of tests"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-408
    type: verifies
---

# TC-357: Bonferroni over tests and symmetry reduction of tests

## Description

Run one test per initial state and binding with Bonferroni-split error rates, and one test per orbit under an admitted symmetry.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. Each statistical vector uses seed `20261001`; run it twice on two platforms and require identical sample counts, estimates and decisions.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| BF-01 | A subject with two initial states and claim `alpha 0.01, beta 0.01` | `m = 2`; each test at `α' = β' = 1/200`; Okamoto's `N` uses `ln 200`; one test entry per initial state |
| BF-02 | Two symmetric objects whose workload weights and random-parameter distributions are invariant under the symmetry, two bindings in one orbit | `m = 1`, one test for the least binding in key order |
| BF-03 | BF-02 with one random parameter of a reference type weighted unequally across the class | No reduction; `m = 2` |

## Expected Results

Test counts, per-test rates and the reported entries are exactly as stated.
