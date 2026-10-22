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
| VS-01 | (a) HB-01 with `match { a.step.op = V::Vault::step and b.step.op = stutter }` on the vault, which has no terminal state, so no matched fair universal tuple exists; (b) HB-01's clause on a variant of VS-03's secure vault whose population `vaults` has universe `{v, w}`, instance `v`, settled by an inductive HP-2 proof at depth 1 that comes with no tuple; (c) the proof of (b) with a tuple of fair lassos for `v` whose second lasso's first `step` takes `i = 1` where the first lasso's takes `i = 0`, so HB-01's conjunct `a.step.Vault::step.i = b.step.Vault::step.i` fails at that joint step on replay; (d) the proof of (b) with a matched fair tuple that replays, both lassos taking `step` with `i = 0`, but for the instance `w`; (e) the proof of (b) with that matched fair tuple for `v`, replaying under `μ_U` and each subject's fairness; (f) the proof of (e) over a variant of that subject with no initial state | (a) to (d) `inconclusive`, `unsettled`, `vacuous-match`, never `proved`; (e) `proved`, `decisive-witness`; (f) `inconclusive`, `no-initial-state`, not `vacuous-match` or `proved` |
| VS-02 | a model where one run reaches a terminal state after one step and another after two, with `match { a.step.op = b.step.op }` and a body comparing a field | the one-step run's component stutters with label `stutter` and is paired only with a stuttering component; the verdict and any counterexample replay |
| VS-03 | HB-01 with the match conjunct `a.step.receiver = b.step.receiver` added, on the secure vault whose `step` also requires `self.l = 0`, so both components of a matched pair reach a terminal state and then take their terminal-stutter step; then a match conjunct whose argument read is refused at a reachable joint step | both receivers are none at the stutter steps, the conjunct holds, and the clause is `proved`, `closed-scope`, with no undefined evaluation; `inconclusive`, `match-undetermined` |
| VS-04 | a request with HB-02 over aliases `S` and `N` | exactly two deadlock-freedom items, one per subject; the hyper item reports no deadlock |
| VS-05 | HB-01's clause under universes where `Vault` has no object; then HB-02's clause with the `S::Device` universe empty and `N::Device` of 3 keys, then with `N::Device` empty and `S::Device` of 2 keys; then HB-01's clause with the `Vault` population given no universe | the first three settle `inconclusive`, `unsettled`, `no-instance` before any state is explored (FR-399-AC-11); the last returns `requires-bound` |
| VS-06 | TC-344 PA-02's `align skip` clause (HP-6) with the `match` conjunct `false`, which no joint visible step satisfies | `proved`, `closed-scope`; never `vacuous-match`, since the empty projected prefix at the initial states counts as matched |
| VS-07 | HB-01's clause on VS-03's secure vault settled by an HP-2 k-induction outcome at depth 1 whose step query has a counterexample; then one whose solver returns `unknown` at depth 1; then one whose solver stops for a resource internal to it that no request limit names; then one stopped by the request's named work meter | The first three `inconclusive`, `unsettled`, `induction-not-closed{depth: 1}`, never `proved`; the fourth `failed`, execution `resource-incomplete`, FR-331 `incomplete`, `limit-reached` naming the meter (FR-399-AC-12) |

## Expected Results

A clause for which no matched fair universal tuple is established is never proved, except an HP-6 clause, which always has its position-0 prefix; and terminal stutter is matched explicitly.
