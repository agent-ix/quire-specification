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
| RD-08 | HB-11 with a symmetry over one subject selected, then HB-01 with a symmetry or a state constraint selected | each settles `unsupported`, `unsupported-requested-capability` before expansion, since the preservation row reads "yes" and the analyzer builds neither the diagonal action of a key symmetry nor a state constraint for a hyper product; no verdict is returned |
| RD-09 | a clause with `fair weak V::Vault::reset` on both `a` and `b` of an otherwise swappable pair; then the same with a second pair `c`, `d` carrying no fairness | the fair class is stored whole and named in no reduction, with the unreduced stored-state count and the unreduced verdict; the second run names copy-swap with class `{c, d}` alone |
| RD-10 | a swappable pair whose body is `always holds(v.l @ a = 0) and always holds(v.l @ b = 0)`, so that the swap exchanges two acceptance sets; then `eventually holds(v.l @ a = 1) and eventually holds(v.l @ b = 1)`, whose swap fixes the acceptance condition | the first is stored whole and names no reduction; the second names copy-swap; a clause with one passing class and one failing class is stored whole and names no reduction |
| RD-11 | an HP-6 clause whose pair is swappable | stored whole as its projected product and names no copy-swap; a refutation names no reduction |

## Expected Results

A reduction applies to a hyper item only by its row, copy-swap applies only when the checked clause is verified symmetric, and reduced and unreduced runs agree.
