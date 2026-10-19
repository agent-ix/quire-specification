---
id: TC-340
title: "Hyperproperty vacuity, terminal stutter and deadlock items"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-395
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-399
    type: verifies
---

# TC-340: Hyperproperty vacuity, terminal stutter and deadlock items

## Description

Check vacuous matches, an undetermined match, stutter matching of terminated components, and the deadlock-freedom items of hyper subjects.

## Test Procedure

Run each vector through the real local Rust boundary with the explicit-state engine and replay any counterexample. Each refutation is checked by replaying its counterexample, never by comparing it with one expected counterexample; the verdict, basis, cause and form are compared exactly.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| VS-01 | HB-01 with `match { a.step.op = V::Vault::step and b.step.op = stutter }` on the vault, which has no terminal state | `inconclusive`, `vacuous-match`, never `proved` |
| VS-02 | a model where one run reaches a terminal state after one step and another after two, with `match { a.step.op = b.step.op }` and a body comparing a field | the one-step run's component stutters with label `stutter` and is paired only with a stuttering component; the verdict and any counterexample replay |
| VS-03 | HB-01 with the match conjunct `a.step.receiver = b.step.receiver` added, on the secure vault whose `step` also requires `self.l = 0`, so both components of a matched pair reach a terminal state and then take their terminal-stutter step; then a match conjunct whose argument read is refused at a reachable joint step | both receivers are none at the stutter steps, the conjunct holds, and the clause is `proved`, `closed-scope`, with no undefined evaluation; `inconclusive`, `match-undetermined` |
| VS-04 | a request with HB-02 over aliases `S` and `N` | exactly two deadlock-freedom items, one per subject; the hyper item reports no deadlock |
| VS-05 | HB-01 with `behaviours (v: V::Vault)` under universes where `Vault` has no object; then the same clause over a type no object of the universes conforms to; then the same clause with the `Vault` population given no universe | the first two settle `inconclusive`, `unsettled`, `no-instance` before any state is explored; the third returns `requires-bound` | FR-399-AC-11 |

## Expected Results

A clause that matches no fair universal tuple is never proved, and terminal stutter is matched explicitly.
