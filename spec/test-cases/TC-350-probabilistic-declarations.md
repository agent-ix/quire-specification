---
id: TC-350
title: "Probabilistic declarations admission"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-405
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272
    type: verifies
---

# TC-350: Probabilistic declarations admission

## Description

Admit and refuse random parameters, rewards, delay members and workloads, and check that each declaration enters the identity of the obligations that read it.

## Test Procedure

Run each vector through the real local Rust boundary of the frontend or provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. For PD-09, compile two packages that differ only in the named element and compare the obligation identities of a claim that reads it.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| PD-01 | `random Server::attempt(d) ~ { 1 ms: 90, 3 ms: 7, 12 ms: 3 }` on a `Quantity<ms>` parameter of the domain package, where the checked package declares the dimension `Time` and the unit `ms` of `Time` | Admitted; probabilities `9/10`, `7/100`, `3/100`; the support `{1 ms, 3 ms, 12 ms}` is the FR-181 domain of `d` |
| PD-02 | A precondition `pre d > 1 ms` on the operation with random `d` | `invalid_probabilistic_model`/`random-read-in-precondition` at the read's locus |
| PD-03 | A distribution repeating `1 ms`; a weight `0`; a value `true` for a `Quantity<ms>` parameter | `repeated-support-value`, `invalid-weight`, `value-outside-type` respectively, each at the value's locus |
| PD-04 | `reward refund on Shop::buy = n - 3` with `n: Int[0, 5]` | `invalid_probabilistic_model`/`negative-reward` naming `Shop::buy`, `refund` and `n = 0` |
| PD-05 | A reward reading `self.balance`; a reward of type `Boolean` | `state-read-in-reward` at the read; `reward-type` |
| PD-06 | `workload Steady on Service` weighting `request` and `attempt` but not `reset`; one weighting `reset` twice; one naming `Other::op`; one with weight `-1` | `incomplete-workload`, `duplicate-weight`, `unknown-operation`, `invalid-weight` under `invalid_probabilistic_model` |
| PD-07 | `delay Call::reply ~ discrete { 1 ms: 90, 3 ms: 10 };` on a `time dense` model, and the same on an untimed model | Admitted on the dense model; `invalid_probabilistic_model`/`delay-on-untimed-model` on the untimed one |
| PD-08 | An operation of a dense model with no `delay` member | Admitted; its delay is nondeterministic (FR-420) |
| PD-09 | Two packages differing in one workload weight, one support value, or one reward expression | Every obligation reading the changed element has a different identity; other obligations keep theirs |
| PD-10 | `random Server::attempt(d) ~ { 1 ms: 1 };` followed by `random Server::attempt(d) ~ { 3 ms: 1 };` | `invalid_probabilistic_model`/`duplicate-distribution` at the second declaration, naming `Server::attempt` and `d` |

## Expected Results

Every admitted declaration has the stated probabilities and support; every refusal carries exactly its named `invalid_probabilistic_model` cause and payload from the FR-272 row; identities change exactly for the obligations that read a changed element.
