---
id: TC-316
title: "Ring-buffer to queue data refinement"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-376
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-377
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-379
    type: verifies
---

# TC-316: Ring-buffer to queue data refinement

## Description

Check the ring-buffer refinement of a queue, whose abstract `items` is built
from the whole slot population, refute a broken `take`, and settle an `only`
that finds no slot.

## Test Procedure

Use TC-311's packages. Abstract operations: `enq(x: Int[0, 1])`, enabled with
fewer than two items, appends `x`; `deq(): Int[0, 1]`, enabled when
non-empty, returns the first item and keeps the rest. Concrete operations:
`Slot::put(x)` writes `x` into the slot at index `(head + size) mod 2` of
its ring and increments `size`; `Slot::take()` returns the value of the slot
at `head`, advances `head` and decrements `size`. The declaration:

```text
refinement RingIsQueue using inf abstract Q concrete R {
  population Q::queues from R::rings;
  object Q::Queue from R::Ring {
    items = seq(i in 0 .. self.size - 1:
              only(s in R::slots where s.ring = self and s.index = (self.head + i) mod 2: s.value));
  }
  step R::Slot::put  -> Q::Queue::enq(self.ring, x);
  step R::Slot::take -> Q::Queue::deq(self.ring);
}
```

1. Check the declaration over universes `rings = {r}`, `slots = {s0, s1}`.
2. From the state `head = 1`, `size = 2`, `s1.value = 1`, `s0.value = 0`,
   take one step of `take`.
3. Replace `take` by a broken version that returns the slot at `head + 1`,
   and check.
4. Remove slot `s1` from the universe and check.

## Expected Results

Case 1 settles `proved` (FR-377-AC-2). In case 2 the step maps `[1, 0]` to
`[0]` with concrete result 1, and passes the abstract-operation rule for
`deq(r)` with the abstract result bound to 1 (FR-376-AC-5, FR-377-AC-2).
Case 3 settles `refuted` with `AbstractStepRejected{…, transition: deq(r),
cause: Postcondition}` at the broken `take` that returns 0 from a mapped
`[1, 0]`, and the counterexample replays (FR-379-AC-2). Case 4 settles
`refuted`, cause `undefined-evaluation`, at the first reachable state where
the row's `only` finds no slot with index 1, naming the `items` row and the
`only`'s locus, with an `Undefined` counterexample ending at that state that
replays to the same undefined value (FR-376-AC-3, FR-379-AC-6).
