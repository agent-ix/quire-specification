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

Replay of model-arm and observed-arm counterexamples: the wire members, the
replay refusals, the missing fairness premise and the parity outcome.

## Test Procedure

Start from the TC-293 step 2 counterexample `X` (loop of three `upd(a)` steps,
`c = b`, fairness (`weak`, `attemptUpdate`, `whole`)) and the TC-290 Counter
subject. Derive:

1. `X` unchanged.
2. `X` with the loop's last step removed, so the loop does not close.
3. `X` replayed against the clause with `fair weak each …`.
4. `X` with the second step's `post_state` replaced by the state key of `(2,0)`.
5. A Counter prefix with four `inc` steps, the fourth at value 3.
6. A Counter lasso whose loop is a terminal stutter step at value 2.
7. `X` with `trace_position` 0 against `always eventually holds(c.versionNumber
   = 0)`, which is true on the lasso.
8. `X` with the member `fairness` removed, with an extra member `engine`, and
   with `trace` `"sampled"`.
9. A safety refutation of TC-290's `always holds(x.value <= 2)`.
10. An observed-arm counterexample for `eventually[0,1] holds(c.value = 2)`
    under the event-position profile: `trace` `"observed"`, three steps whose
    documents hold `c.value` 0, 1 and 2, `interval` `{lower: "0", upper: "1"}`,
    `trace_position` 0, empty `fairness`, every document held by the byte
    provision.
11. Step 10's counterexample against the clause under `fair weak inc`, with
    `fairness` (`weak`, `inc`, `whole`).
12. Step 10's counterexample with its third document missing from the
    provision.
13. Step 10's counterexample with `interval` `{lower: "0", upper: "2"}`.
14. An infinite-trace formula counterexample for `eventually[2,*] holds(p)`
    with `interval` `{lower: "2", upper: null}`, replayed against that clause
    and against `eventually[3,*] holds(p)`.

## Expected Results

1. Reproduces: `reproduced-with-evaluated-witness`, item `refuted`
   (FR-364-AC-3).
2. Refuses `invalid_runtime_input`/`invalid-value` (FR-364-AC-4).
3. Refuses as an unfair lasso naming (`weak`, `attemptUpdate`, `each`) with
   the starved receiver `b`; no `refuted` (FR-364-AC-5).
4. Withdrawn: the recorded post-state key is not checked against a successor.
5. Refuses `invalid_runtime_input`/`invalid-value`: `inc` is not enabled at
   value 3 (FR-364-AC-4).
6. Refuses `invalid_runtime_input`/`invalid-value`: value 2 is not terminal
   (FR-364-AC-4).
7. `inconclusive`, cause `replay-parity` (FR-364-AC-6).
8. All three refuse before replay (FR-364-AC-2).
9. A finite prefix with an empty `loop`, `kind: formula`, which reproduces
   (FR-364-AC-1).
10. `reproduced-with-evaluated-witness`: positions 0 and 1 hold `c.value` 0
    and 1, so the formula is false at position 0 (FR-364-AC-9).
11. `unsupported`, cause `missing-fairness-premise`, naming
    (`weak`, `inc`, `whole`), with no formula evaluated (FR-364-AC-9).
12. Refuses `invalid_runtime_input`/`invalid-value` (FR-364-AC-9).
13. Withdrawn: `interval` is not compared with the clause's.
14. Withdrawn: `interval` is not compared with the clause's.
