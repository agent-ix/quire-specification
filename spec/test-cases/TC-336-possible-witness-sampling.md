---
id: TC-336
title: "Witness sampling for possibility claims"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-392
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-391
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-336: Witness sampling for possibility claims

## Description

Check the sampling phase's defaults, walk indexing, settlement method and verdict independence.

## Test Procedure

Run each vector through the real local Rust boundary twice and compare the two runs; record the walks drawn, their trace indices, the terminal record's seed and `witness_samples`, the verdict and the sources.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| WS-01 | TC-331 PV-02 with no seed and no `witness_samples` in the request | the routed provider's published `witness_samples` and `seed`; both values in the terminal record |
| WS-02 | a subject with 3 initial states, `witness_samples` 5 | walk `r` from initial state `i` has trace index `5i + r`; each stops at its first `P` node |
| WS-03 | WS-02 with `witness_samples` 0 | no walk drawn; same verdict with sources `explored` |
| WS-04 | TC-331 PV-03 under three different seeds and `witness_samples` 0, 1 and 64 | `refuted` with a trap in every run |
| WS-05 | a subject where sampling finds witnesses for some initial states only | exploration runs, keeps the sampled witnesses with source `sampled`, and finds the rest with source `explored` |
| WS-06 | any vector run twice with the same subject, claim, limits and seed | identical walks, result and evidence |
| WS-07 | a `possible` claim whose every initial state reaches `P` within 2 steps on every walk, on a model with a branch, off every walk of the 64, to a node where `P` reads `lookup` of an absent key | `refuted`, `decisive-counterexample`, cause `undefined-evaluation`, with `witness_samples` 64 and with `witness_samples` 0: the same verdict, the same `undefined` arm |
| WS-08 | WS-07's model without the undefined branch and with a long counter branch, `max_states` set so the run stops after sampling found a witness for every initial state | `inconclusive`, cause `well-definedness-unchecked`, the sampled witnesses with their seed and trace indices, and the statement that well-definedness was not checked |
| WS-09 | WS-08 with no `max_states` set | `proved`, `decisive-witness`, sources `sampled`; the terminal record states the search horizon, and the horizon is not among the request's limits |
| WS-10 | two requests differing only in `seed`, two differing only in `witness_samples`, and two differing only in `max_walk_steps` | different request identities |
| WS-11 | TC-331 PV-03 (`ReachesThree`, a cyclic graph with no node where `P` holds) with defaults, and with `max_walk_steps` 10 | every walk ends at `max_walk_steps` (the provider's published default, then 10) without a witness; the terminal record names `max_walk_steps`, its value and its setting; exploration runs and the item settles `refuted` with the trap at `(0, 0)` |

## Expected Results

Sampling changes how an item is settled and never which verdict it reaches, and the result always says how it was settled.
