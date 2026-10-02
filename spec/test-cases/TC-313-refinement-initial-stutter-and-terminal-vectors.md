---
id: TC-313
title: "Refinement initial, stutter and terminal-state vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-377
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-379
    type: verifies
---

# TC-313: Refinement initial, stutter and terminal-state vectors

## Description

Refute an initial-state mismatch and a stutter that changes a mapped field,
and read concrete halts under every combination of the two models'
`terminal` members.

## Test Procedure

Start from TC-312's positive model.

1. **Initial mismatch.** Set the concrete initial `value` to 1.
2. **Stutter changes the map.** Change `peek`'s postcondition to
   `self.value = 0` with frame `modifies value`, and keep `peek -> stutter`.
3. **Halt at an abstract-terminal state.** Remove `peek`, the retry
   operations and their rows from the concrete model. Every concrete
   terminal state then has `value = 2`, which maps to the abstract state
   where `inc` is disabled. Check under each combination of a concrete
   and an abstract `terminal` member, each drawn from: none,
   `terminal when self.value = 2` and `terminal any`.
4. **Halt where the abstract model can move.** In TC-312's positive model add
   a Boolean field `stopped` (initially false), an operation `stop()`
   enabled when `value = 0` that sets `stopped`, the row
   `step Impl::Counter::stop -> stutter;`, and `not self.stopped` to every
   other concrete precondition, `peek` included. Check with `F_A` empty and
   with `ensure fair weak each Spec::Counter::inc`, under the same
   combinations of `terminal` members.
5. **Stuttering is not a deadlock.** Repeat case 4 with `peek`'s
   precondition left unchanged, so after `stop` only `peek` is enabled, with
   `F_A` empty; inspect the request's items and verdicts.
6. **Derived concrete terminal predicate.** In case 3, write the concrete
   member `terminal when self.value = 2`, the abstract
   `terminal when self.value = 2` read through `value = self.value`, and
   check the concrete deadlock-freedom item.

## Expected Results

Case 1 settles `refuted` with failure `InitialNotAbstract{initial: 0}`
(FR-377-AC-3, FR-379-AC-2). Case 2 settles `refuted` with
`StutterChanged{position}` at the first `peek` from a state with `value` 1
(FR-377-AC-2). Case 3 holds under every combination (FR-377-AC-5). Case
4 holds with `F_A` empty and settles `refuted`, `Divergence{inc}`, with the
`ensure` row, identically under every combination (FR-377-AC-5). In case
5 the `peek`-only state is not reported as a deadlock and the request
carries the concrete deadlock-freedom item separately from the refinement
item, which holds (FR-377-AC-6). Case 6 settles the concrete deadlock-freedom item
`proved` (FR-377-AC-8).
