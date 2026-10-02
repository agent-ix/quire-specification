---
id: TC-312
title: "Counter refinement vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-377
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-379
    type: verifies
---

# TC-312: Counter refinement vectors

## Description

Decide the compare-and-set counter refinement and its broken variants:
holding, lost update, divergence with and without abstract fairness, and the
`any` variant that hides the first lost update and refutes a later one.

## Test Procedure

Use TC-310's positive declaration. Universe `counters = {c}`; abstract
initial state `value = 0`; concrete initial state with every integer field 0
and both `busy` fields false. Write a concrete state `(value, tmpA, busyA,
tmpB, busyB)`. Concrete clauses: `beginA` needs `not busyA` and sets `busyA`
and `tmpA = pre(value)`; `commitA` needs `busyA and tmpA = value and value <
2` and sets `value = pre(tmpA) + 1`, clearing `busyA`; `retryA` needs `busyA
and tmpA != value` and clears `busyA`; the same for B; `peek` changes
nothing. Abstract: `inc` needs `value < 2` and sets `value = pre(value) + 1`.

1. **Holds.** Check the positive declaration.
2. **Lost update.** Change both commit preconditions to `busyA and tmpA < 2`
   (and the B analogue), drop the `retry` operations and their rows, and
   check.
3. **`any` variant.** In the lost-update model write both commit rows as
   `-> any` and check.
4. **Divergence.** In the positive model replace the `assume` rows by
   `assume fair weak each` rows for `commitA` and `commitB` only, and check.
5. **Safety refinement.** Remove the `ensure` row from case 4 and check.
6. **Per-step.** Request the per-step simulation obligations for cases 1 and
   2; request them for case 3 and for a declaration with a hidden field.
7. **Per-step with a history field.** Add to the positive declaration the
   row `history Impl::Counter.commits: Int[0, 4] = 0 { on
   Impl::Counter::commitA: pre(self.commits) + 1; on Impl::Counter::commitB:
   pre(self.commits) + 1; }`, request the per-step simulation obligations,
   and check the refinement.

Replay each refutation under FR-379.

## Expected Results

Case 1 explores 28 reachable concrete states, every edge passes the step
function, no fair stutter SCC exists below `value` 2, and the item settles
`proved`, basis `closed-scope` (FR-377-AC-2, FR-377-AC-8, FR-379-AC-1). Case
2 explores 36 states and settles `refuted`, basis `decisive-counterexample`,
with the prefix `(0,0,f,0,f)`, `beginA`, `beginB`, `commitA`, `commitB` and
failure `AbstractStepRejected{position: 4, transition: inc(c), cause:
Postcondition}`; replay recomputes mapped values 0, 0, 0, 1, 1 and the same
failure (FR-379-AC-2, FR-379-AC-7). In case 3 the first lost update, at
position 4, passes the `any` rule as a stutter; the item settles `refuted`
with failure `NoAbstractMatch{position: 6}` at the first failing edge in
canonical breadth-first order, where a commit holding a stale scratch value
lowers `value` from 2 to 1 (for example `beginA`, `beginB`, `commitA`,
`beginA`, `commitA`, then `commitB` from `(2, 1, f, 0, t)` to
`(1, 1, f, 0, f)`), which no `inc` and no stutter matches, and the
counterexample replays (FR-377-AC-2, FR-379-AC-2). Case 4 settles `refuted`
with the lasso of empty stem and one `peek` loop step at `(0,0,f,0,f)`,
failure `Divergence{inc}`, fair under `F_C` (FR-377-AC-4, FR-379-AC-2). Case
5 settles `proved` (FR-377-AC-4). In case 6 each step obligation of case 1
settles `proved`, basis `decisive-witness`; the `commitB` obligation of case 2
fails from `(1, 0, f, 0, t)` and settles `inconclusive`, induction not
closed, never `refuted`; no per-step obligation is routed for case 3's `any`
row or for the hidden-field declaration (FR-377-AC-7, FR-379-AC-1). In case 7
no per-step obligation is routed, and the refinement item is decided by the
explicit-state engine alone and settles `proved`, as case 1 does
(FR-377-AC-7).
