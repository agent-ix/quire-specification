---
id: TC-295
title: "Temporal counterexample wire, replay and refusals"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-364
    type: verifies
---

# TC-295: Temporal counterexample wire, replay and refusals

## Description

Replay of model counterexamples: the wire members, the four replay refusals
and the parity outcome.

## Test Procedure

Start from the TC-293 step 2 counterexample `X` (loop of three `upd(a)` steps,
`c = b`, fairness (`weak`, `attemptUpdate`, `whole`)) and the TC-290 Counter
subject. Derive:

1. `X` unchanged.
2. `X` with the loop's last step removed, so the loop does not close.
3. `X` replayed against the clause with `fair weak each …`.
4. `X` with the second step's `post_state` replaced by the digest of `(2,0)`.
5. A Counter prefix with four `inc` steps, the fourth at value 3.
6. A Counter lasso whose loop is a terminal stutter step at value 2.
7. `X` with `trace_position` 0 against `always eventually holds(c.versionNumber
   = 0)`, which is true on the lasso.
8. `X` with the member `fairness` removed, and with an extra member `engine`.
9. A safety refutation of TC-290's `always holds(x.value <= 2)`.

## Expected Results

1. Reproduces: `reproduced-with-evaluated-witness`, item `refuted`
   (FR-364-AC-3).
2. Refuses `invalid_runtime_input`/`invalid-value` (FR-364-AC-4).
3. Refuses as an unfair lasso naming (`weak`, `attemptUpdate`, `each`) with
   the starved receiver `b`; no `refuted` (FR-364-AC-5).
4. Refuses `stale_dependency`/`revision-mismatch` (FR-364-AC-4).
5. Refuses `invalid_runtime_input`/`invalid-value`: `inc` is not enabled at
   value 3 (FR-364-AC-4).
6. Refuses `invalid_runtime_input`/`invalid-value`: value 2 is not terminal
   (FR-364-AC-4).
7. `inconclusive`, cause `replay-parity` (FR-364-AC-6).
8. Both refuse before replay (FR-364-AC-2).
9. A finite prefix with an empty `loop`, `kind: formula`, which reproduces
   (FR-364-AC-1).
