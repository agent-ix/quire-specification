---
id: TC-347
title: "Hyperproperty reductions and copy-swap symmetry"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-403
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-401
    type: verifies
---

# TC-347: Hyperproperty reductions and copy-swap symmetry

## Description

Check the preservation rows for hyper forms and compiler-verified copy-swap symmetry. The vault subject: model `V` with object type `Vault`, fields `h: Int[0, 1]` (secret) and `l: Int[0, 1]` (public output), population `vaults` with universe `{v}`, one operation `step(i: Int[0, 1])` with frame `modifies [l]` and no precondition, and initial snapshots `(h, l) = (0, 0)` and `(1, 0)`. The leaky model's post clause is `self.l = (i + self.h) mod 2`; the secure model's is `self.l = i`.

## Test Procedure

Run each vector reduced and unreduced through the real local Rust boundary with the explicit-state engine; record the verdicts, the reduced basis and the stored-state counts, and replay every counterexample with no engine present.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| RD-01 | TC-338 NI-01 and NI-02 with no reduction selected | copy-swap applies; leaky stores 7 classes, secure 6; verdicts equal to the unreduced runs; the secure proof names copy-swap with class `{a, b}` |
| RD-02 | NI-01 found under copy-swap | an ordinary `lockstep` counterexample in quantifier order that replays |
| RD-03 | a clause whose body `always holds(v.l @ a <= v.l @ b)` is not unchanged under the swap | no copy-swap applied |
| RD-04 | HB-01 with `a` and `b` carrying different fairness sets | no copy-swap applied |
| RD-05 | HB-01 with partial-order reduction selected | `inconclusive`, `reduction-not-preserving`, before expansion |
| RD-06 | `Opaque` of TC-339 with a state constraint selected | `inconclusive`, `reduction-not-preserving`, before expansion |
| RD-07 | HB-02 over two subjects with a diagonal symmetry selected | `inconclusive`, `reduction-not-preserving`, before expansion |
| RD-08 | HB-11 with a symmetry over one subject selected, then HB-01 with a symmetry or a state constraint selected | each selection, taken on its own (HB-11's symmetry, HB-01's symmetry, HB-01's state constraint), settles `unsupported`, `unsupported_projection`/`unsupported-requested-capability` before expansion naming the reduction, since its cell is admitted and the provider builds none of them for a hyper product; no selection is ignored and no verdict is returned |
| RD-12 | TC-339 `Opaque` (HP-3) with a key symmetry selected; PA-02's `align skip` clause (HP-6) with a key symmetry selected, then with a state constraint selected | each settles `unsupported`, `unsupported_projection`/`unsupported-requested-capability` before expansion naming the reduction |
| RD-09 | a vault variant whose `step` toggles `l` and which declares `reset` (enabled at `l = 1`, keeping `l = 1`); the clause with body `always holds(v.l @ a = v.l @ b)`, `match { a.step.op = b.step.op }` and `fair weak V::Vault::reset` on both `a` and `b`, from `a = (0, 0)` and `b = (0, 1)` as `(h, l)`; then the same clause with a second pair `c`, `d` that carries no fairness | the first has copy group `{a, b}` and is stored whole at 4 product states (the unreduced count) with no reduction named, `refuted`, its counterexample replaying fair; in the second the pair `c`, `d` alone is reduced: its stored-state count is below the unreduced count and the verdict equals the unreduced run's, and a refutation names no reduction (a variant of the second clause on which the run is proved names copy-swap with class `{c, d}` alone) |
| RD-10 | on the secure vault, the body `always holds(v.l @ a = 0) and always holds(v.l @ b = 0)`; then, on a vault variant whose `step` toggles `l`, from the initial states `(h, l) = (0, 0)` and `(0, 1)`, the body `eventually holds(v.l @ a = 1) and eventually holds(v.l @ b = 1)`; then a clause with one class that passes and one that fails | the first has copy group `{a, b}` but its swap exchanges two acceptance sets, so it is stored whole with the unreduced stored-state count and no reduction named; the second's swap fixes its acceptance condition and it stores 5 states against the unreduced 8; the third is stored whole with the unreduced count, for the passing class as well |
| RD-11 | an HP-6 clause whose pair is swappable | stored whole as its projected product and names no copy-swap; a refutation names no reduction |

## Expected Results

A reduction is admitted to a hyper item only by its row and a selected reduction the provider does not build settles `unsupported`; copy-swap applies only when the checked clause is verified symmetric, and reduced and unreduced runs agree.
