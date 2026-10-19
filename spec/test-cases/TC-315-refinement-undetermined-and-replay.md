---
id: TC-315
title: "Refinement undetermined mappings and replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-379
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-378
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-377
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-375
    type: verifies
---

# TC-315: Refinement undetermined mappings and replay

## Description

Refute on an undefined row or history update, settle `mapping-undetermined`
for a history update that leaves its type, replay
refinement counterexamples, refuse a lasso that does not close, and settle a
parity mismatch.

## Test Procedure

1. **Undefined row.** In TC-312's positive declaration map `value = 2 div
   self.tmpA`, undefined where `tmpA = 0`, which is reachable.
2. **History leaves its type.** In TC-314 case 1 declare the history field
   `Int[0, 2]` while four commits are reachable.
3. **Replay agreement.** Replay TC-312's lost-update counterexample through
   the concrete package and the abstract subject's package.
4. **Loop that does not close.** Take TC-314 case 1 with an `ensure` row and
   a refuting lasso whose loop increments `commits`; alter the counterexample
   so its loop's last position carries a different history value from its
   entry, and replay it.
5. **Parity.** Replay TC-312's lost-update counterexample against a concrete
   package in which `commitB`'s postcondition is changed so that the failure
   recomputes at position 3, and against one where it recomputes as
   `StutterChanged`.
6. **Undefined history update.** In TC-314 case 1 make the history update
   `2 div self.tmpA`, undefined at the first reachable commit step where
   `tmpA = 0`.
7. **Failure precedence.** At one state of the ring-buffer refinement hold
   an earlier row that is incomplete (budget zero) and a later `items` row
   whose `only` is undefined; then the same state with the earlier row
   refused instead; then with an unmapped subtype object whose key sorts
   first; then with an evaluator stop injected in the earlier row; then write
   the same rows in the opposite source order.
8. **Optional and null rows.** Map a `?` abstract field by a row that is
   `none` at one state and `Some(1)` at another, an `Option<Int[0, 2]>` field
   by `none`, and a field by a row that evaluates to explicit `null` while a
   later row is undefined.

## Expected Results

Case 1 settles `refuted`, basis `decisive-counterexample`, cause
`undefined-evaluation` naming the row, the initial state (where `tmpA = 0`),
the `div`'s locus and its catalogued reason; its counterexample has
`refinement_failure` `Undefined{position: 0, row: value}` and an empty
prefix, and replay reproduces the undefined value (FR-379-AC-6,
FR-377-AC-1). Case 2 settles `inconclusive`, cause `mapping-undetermined`,
and the third commit step stays a concrete step (FR-378-AC-3). Case 3
recomputes history values and mapped states, reruns the step function at
position 4, and settles `reproduced-with-evaluated-witness` with the item
`refuted` (FR-379-AC-3). Case 4 refuses `invalid_runtime_input`/
`invalid-value` (FR-379-AC-4). Both runs of case 5 settle the item
`inconclusive`, cause `replay-parity` (FR-379-AC-5, FR-379-AC-1). Case 6
settles `refuted`, cause `undefined-evaluation`, with an `Undefined`
counterexample ending at that commit step that replays, and the step stays a
concrete step (FR-378-AC-3).
Case 7 refutes with the later undefined row in the first three runs, reports
the stop with `map` undetermined in the fourth, and gives the same reported row
in the fifth (FR-377-AC-9, FR-377-AC-11). Case 8 leaves the `?` field absent
then set to `1`, keeps the explicit `none`, and makes `map` undetermined with
the refused `null` row yet still refutes on the later undefined row
(FR-375-AC-17, FR-377-AC-10).
