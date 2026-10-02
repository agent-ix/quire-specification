---
id: TC-328
title: "Stutter invariance of since and triggered"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-161
    type: verifies
---

# TC-328: Stutter invariance of since and triggered

## Description

Confirm that since and triggered are non-strict, and that inserting a
repeated state changes no truth value of a formula without previous or
interval operators, while a formula with previous can change.

## Test Procedure

Atoms `p` and `q` over a two-field state. Lasso `L`: prefix `s0 = (q)`,
`s1 = (p)`, loop `s2 = (p)`, `s3 = (q)`. `L'` is `L` with `s1` repeated
once (a stutter position inserted after position 1).

1. Evaluate `p since q` and `p triggered q` at every position of `L`,
   including position 0, where `q` holds.
2. Evaluate on `L` and `L'`, at corresponding positions: `always (p or q)`,
   `eventually q`, `p until q`, `q release p`, `historically (p or q)`,
   `once q`, `p since q`, `p triggered q`.
3. Evaluate `previous q` on `L` at position 1 and on `L'` at positions 1
   and 2, both of which hold the state `s1`.

## Expected Results

In case 1 `p since q` holds at position 0 because `q` holds there, and
`p triggered q` equals `not(not p since not q)` at every position. In case 2
every formula has the same value on `L` and `L'` at corresponding
positions. In case 3 `previous q` is true on `L` at position 1 and on `L'` at
position 1, and false on `L'` at position 2, the inserted repeat of `s1`, so
a formula with previous changes under the insertion (FR-161-AC-12).
