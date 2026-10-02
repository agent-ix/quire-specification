---
id: TC-321
title: "Symmetry reduction vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-382
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-383
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-385
    type: verifies
---

# TC-321: Symmetry reduction vectors

## Description

Reduce three interchangeable configs by symmetry, check one binding orbit
under its stabiliser, concretise a lifted lasso, and settle broken and
partial symmetry declarations.

## Test Procedure

Subject: TC-320 case 1 (annotated), universe `config_history = {a, b, c}`,
one initial snapshot with all three at version 0 and no parent. Write a
state `(va, vb, vc)`. Claim `ReachesTwo`: `always eventually
holds(c.versionNumber = 2)` over `(c: ConfigVersion)`, infinite-trace.
Request: `SymmetryDeclaration{config_history, [[a, b, c]]}`.

1. Explore the model graph with and without the declaration and count
   stored states.
2. Check `ReachesTwo` under `fair weak whole attemptUpdate` with the
   declaration; count instances checked and product states per instance and
   in total; replay the counterexample.
3. Check the derived deadlock-freedom item with the declaration.
4. Change the initial snapshot to `a` at version 1, others at 0, with the
   full declaration; then with `[[a], [b, c]]`.
5. Submit declarations naming key `d`, putting `b` in two classes, naming a
   population that is not a universe of the request, and listing classes out
   of key order.

## Expected Results

Case 1 stores 10 states with the declaration and 27 without; every stored
state is a member of a distinct orbit here (FR-383-AC-1, FR-383-AC-2). Case 2
checks one instance (`c = a`) for the orbit `{a, b, c}`, exploring 30 product
states (18 with `q0`, 12 with `q1`) against 135 unreduced; it settles
`refuted`, basis `decisive-counterexample`, naming no reduction, with the
lasso `(0,0,0) -upd(b)-> (0,1,0) -upd(b)-> (0,2,0) -upd(b)-> (0,0,0)` bound
to `c = a`, concretised from the reduced loop whose first edge carries
permutation `(b c)`, and replay evaluates the clause false at position 0
(FR-383-AC-2, FR-383-AC-7, FR-385-AC-4). Case 3 settles `proved`, basis
`closed-scope`, proof basis `reduced` listing `Symmetry` with the group and
its row, `certification` `uncertified`, over the 10 canonical states
(FR-385-AC-3). In case 4 the full
declaration settles `inconclusive`, `symmetry-broken{config_history,
initial 0, generator (a b)}`, before expansion, and `[[a], [b, c]]` is
admitted (FR-382-AC-4). Each declaration of case 5 refuses
`invalid_runtime_input`/`invalid-value` naming it (FR-382-AC-1,
FR-382-AC-3).
