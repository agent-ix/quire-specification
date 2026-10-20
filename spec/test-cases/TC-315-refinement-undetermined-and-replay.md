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
  - target: ix://agent-ix/quire-specification/FR-377-AC-12
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-377-AC-13
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
7. **Failure precedence.** Use TC-316's ring-buffer refinement over a fixture
   variant: `Ring` also has `next: Reference<Ring>` and `Queue` has
   `next: Reference<Queue>` with the row `next = self.next`; `Ring` has a
   subtype `SubRing` that `rings` admits and no object row names. Hold the
   state `head = 1`, `size = 2`, `s0.value = 0`, `s1.value = 1` for ring `r`,
   and run: (a) the `items` row changed to `only(s in R::slots where
   s.ring = self: s.value)` (undefined at `r`), with a `SubRing` object `a`
   whose key sorts before `r`; (b) the original `items` row (defined at `r`)
   with the same `SubRing` object; (c) the variant of (a) without the
   `SubRing` object and with `Queue` also declaring `also: Int[0, 1]` and the
   row `also = only(s in R::slots where s.ring = self: s.value)` (also
   undefined at `r`), then again with the `also` and `items` rows written in
   the opposite source order; (d) ring `a` of type `Ring` with `head = 0`,
   `size = 0` and `next` = `q`, `q` of type `SubRing` (also `head = 0`,
   `size = 0`), with the original `items` row, then with `q` of type `Ring`;
   (e) read the relationship extent of the mapped state of the plain fixture.
8. **Optional rows.** Map a `?` abstract field by a row that is `none` at one
   state and `Some(1)` at another, and an `Option<Int[0, 2]>` field by `none`.
9. **Failure precedence across a step.** Use TC-316's ring-buffer refinement
   with the row `put -> enq(self.ring, A1, A2)` and, for each case, choose
   the receiver `R`, the arguments `A1` and `A2`, the history update `H` and
   a post-state row `P` from: defined, incomplete (an expression exceeding
   its fresh meter), undefined, out of type (`H` only) or of an unmapped type
   (`P` only). Run (a) `R` incomplete, `A1` undefined; (b) `A1` incomplete,
   `P` undefined; (c) `A1` and `A2` undefined; (d) `H` out of type, `P`
   incomplete; (e) `P` of an unmapped type, nothing undefined; (f) `A1`
   undefined and a broken evaluator invariant in `P`; (g) every read defined
   and the abstract subject's postcondition exceeding its per-evaluation
   meter; (h) as (g) with `A1` undefined.

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
Case 7: (a) refutes, reporting the undefined `items` row, because the undefined
row outranks the unmapped type; (b) is undetermined for the `SubRing` object
`a` naming its object row; (c) reports the `also` row first, being the lower
field name, in both source orders, for object `r`; (d) is undetermined naming the `next` row
and object `a` with `q` of type `SubRing` and maps `next` to the `Queue`
reference with key `q` with `q` of type `Ring`; (e) lists no relationship
extent (FR-377-AC-9, FR-377-AC-11). Case 8 leaves the `?` field absent then
set to `1` and keeps the explicit `none` (FR-375-AC-17, FR-377-AC-10).

Case 9: (a) refutes, reporting `A1`; (b) refutes, reporting `P`; (c) refutes,
reporting `A1`; (d) settles `inconclusive`, `mapping-undetermined`, reporting
`H`; (e) settles `inconclusive`, `mapping-undetermined`, reporting `P`; (f)
settles `failed`; (g) settles `incomplete`, `resource-incomplete`,
`limit-reached`; (h) refutes, reporting `A1` (FR-377-AC-12, FR-377-AC-13).
