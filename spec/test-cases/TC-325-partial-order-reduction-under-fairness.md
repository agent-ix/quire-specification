---
id: TC-325
title: "Partial-order reduction under fairness"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-383
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-385
    type: verifies
---

# TC-325: Partial-order reduction under fairness

## Description

Check that fairness visibility keeps a fair representative of every fair
behaviour: a protocol whose channel faults cannot starve a branch under the
default scheduler constraints, and the `Gate` unit, where fairness
visibility changes the reduced product and the verdict.

## Test Procedure

1. **`Mark`.** Object type `Cell` with `done: Boolean`, universe `{c}`,
   `c.done` false; operation `setDone()` with frame `modifies self.done`,
   precondition `not self.done`, postcondition `self.done`. Channel `h`,
   `unordered`, capacity 2, non-blocking overflow, faults `duplicate` and
   `lose`, one-value messages. Protocol `Mark` over `(k: Cell)` `on origin`
   runs `parallel Both { branch left send S on h; branch right attempt D on
   setDone } join all [left, right]` then `finish`. Claim `always eventually
   holds(k.done)` under its default scheduler constraints (`fair weak whole`
   per branch and the root), with partial-order reduction selected; then
   with `scheduling adversarial`.
2. **`Gate`.** Object type `Gate` with `a`, `c`, `d`, `done: Boolean`, one
   object `g`, all false, receiver-scoped frames: `arm()` sets `c` (pre
   `not self.c`); `arm2()` sets `d` (pre `self.c and not self.d`); `go()`
   sets `done` (pre `self.c and self.d and not self.done`); `toggle()` flips
   `a`. Canonical order `arm`, `arm2`, `go`, `toggle`. Write a state
   (`a`, `c`, `d`, `done`). Claim `eventually holds(x.done)` under
   `fair weak go`, with partial-order reduction selected; record the ample
   set at `(F,F,F,F)` and the visibility of `arm` and `arm2`. Then check the
   same claim with an empty fairness set.

3. **`Gate` with the guard in the postcondition.** Case 2's unit with `go`
   changed to pre `not self.done` and post `self.done and pre(self.c) and
   pre(self.d)`. Check `eventually holds(x.done)` under `fair weak go` with
   partial-order reduction selected, and without it; record `go`'s enabling
   footprint and the ample set at `(F,F,F,F)`.

Replay every refutation with fairness re-established.

## Expected Results

In case 1, under the default constraints, `send(S)` and `attempt(D)` end
their branches and write the parallel instance the root's `join` reads, and
`fork` writes the new threads' control, so all three are visible; the
`duplicate` and `lose` steps are invisible; the reduced product keeps all 20
product states, the only non-trivial accepting SCC (`(F, end, D, 1)` and
`(F, end, D, 2)` under `q1`) is rejected because `right`'s class is enabled
and never taken there, and the item settles `proved`, proof basis `reduced`
(FR-383-AC-5, FR-385-AC-8). Under `scheduling adversarial` the fairness set
is empty and the item settles `refuted` with the lasso `fork`, `send(S)`,
then the loop `duplicate`, `lose`, which replays (FR-385-AC-8). In case 2
`arm` and `arm2` write `go`'s enabling footprint and are visible, the ample
set at `(F,F,F,F)` is `{toggle}`, the cycle `(F,F,F,F) ⇄ (T,F,F,F)` is kept
with `go` disabled on it, and the item settles `refuted` with the loop
`toggle, toggle`, which replays as weakly fair for `go` (FR-383-AC-5). With
the empty fairness set no fairness visibility applies, `{arm}` is the ample
set at `(F,F,F,F)`, and the item settles `refuted` through the cycle that
remains, whose counterexample replays (FR-385-AC-4).

In case 3 `go`'s enabling footprint is its whole read footprint, holding
`c`, `d`, `done` and the membership of `g`, so `arm` and `arm2` are visible
under `fair weak go` and the ample set at `(F,F,F,F)` is `{toggle}`; the
reduced run stores the same 6 states as the unreduced run and settles
`refuted` with the loop `toggle, toggle`, which replays (FR-383-AC-5).
