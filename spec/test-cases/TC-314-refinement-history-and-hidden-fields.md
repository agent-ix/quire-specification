---
id: TC-314
title: "Refinement history and hidden-field vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-378
    type: verifies
---

# TC-314: Refinement history and hidden-field vectors

## Description

Check a refinement that needs a history field, the same refinement with that
field hidden, and a refinement that needs a prophecy and holds through a
hidden field.

## Test Procedure

1. **History.** Abstract `Counter` with `total: Int[0, 4]` counting
   increments (`inc` sets `total = pre(total) + 1`, precondition
   `total < 4`). Concrete counter: TC-312's positive model with `value:
   Int[0, 1]` wrapping (`commitA` writes `(pre(tmpA) + 1) mod 2`) and a
   commit budget of four. The declaration maps `total = self.commits` with
   `history Impl::Counter.commits: Int[0, 4] = 0 { on Impl::Counter::commitA:
   pre(self.commits) + 1; on Impl::Counter::commitB: pre(self.commits) + 1; }`
   and `commitA`/`commitB -> Spec::Counter::inc(self)`. Check it; compare the
   set of concrete behaviours with and without the history row, and the
   concrete package bytes.
2. **History on creation.** Add a population of counters whose `create`
   operation makes a new object; check that the new object's `commits` is 0
   and that a `begin` step leaves every `commits` unchanged.
3. **Hidden, safety only.** Remove the `total` row so `total` is hidden, with
   no `ensure` row.
4. **Hidden with liveness.** Case 3 with `ensure fair weak each
   Spec::Counter::inc`.
5. **Prophecy through a hidden field.** Abstract coin: `toss()` sets `side`
   to either value and `shown` to true; `look(): Boolean` returns `side`.
   Concrete coin: `toss()` sets `tossed`; `reveal(): Boolean`, enabled when
   `tossed and not revealed`, sets `revealed` and returns either value. The declaration leaves `side` hidden, maps
   `shown = self.tossed`, `toss -> toss(self)`, `reveal -> look(self)`. Check
   it; then try each functional mapping `side = true` and `side = false`.
6. **Divergence with a hidden field.** TC-318's protocols with a hidden
   abstract field `mode: Boolean`, free at the abstract initial states, and
   the specification's `ack` enabled only when `not mode`. Check the
   implementation with `spin`, with `F_A` empty and under `fair weak whole
   ack`. Then make `ack` enabled for either `mode` and check again.

## Expected Results

Case 1 settles `proved`; the history row leaves the concrete behaviours, the
concrete package and every other concrete verdict unchanged (FR-378-AC-1,
FR-378-AC-2, FR-378-AC-7). Case 2 shows the created object at 0 and `begin`
keeping every history value (FR-378-AC-1). Case 3 decides the safety half
with the candidate set: after a `commit` step the set holds the abstract
successors whose visible part matches; the item settles `proved`
(FR-378-AC-4). Case 4 settles its safety half and its liveness half
`unsupported`, `unsupported_projection`/`unsupported-requested-capability`,
naming liveness through hidden abstract fields (FR-378-AC-6, FR-378-AC-7).
Case 5 holds: after `toss` the candidate set holds both sides and `reveal`
prunes it to the returned side; each functional mapping settles `refuted`
with `AbstractStepRejected` at the `reveal` that returns the other side
(FR-378-AC-4, FR-378-AC-5).

In case 6, with `ack` requiring `not mode`, the candidate set at `ready`
holds both abstract states, the `mode = true` state enables no visible step,
and the `spin` loop is not a divergence. With `ack` enabled for either
`mode` every member enables `ack`, and the item settles `refuted` with the
divergence lasso, the `spin` loop at `ready`, which replays. Both verdicts
are the same with `F_A` empty and under `fair weak whole ack`, and neither
settles `unsupported` (FR-378-AC-8).
