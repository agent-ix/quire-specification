---
id: TC-311
title: "Population-valued mapping expressions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-376
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-377
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-379
    type: verifies
---

# TC-311: Population-valued mapping expressions

## Description

Type and evaluate the six population-valued forms, including the ring-buffer
to queue data refinement, and the undefinedness of `only`.

## Test Procedure

Concrete package `example/ring-buffer`: type `Ring` with `head: Int[0, 1]`
and `size: Int[0, 2]`, population `rings` with universe `{r}`; type `Slot`
with `ring: Reference<Ring>`, `index: Int[0, 1]` and `value: Int[0, 1]`,
population `slots` with universe `{s0, s1}` (indices 0 and 1). Abstract
package `example/queue-spec`: type `Queue` with
`items: Sequence<Int[0, 1]>[0, 2]`, population `queues`.

1. Evaluate, at the state `head = 1`, `size = 2`, `s0.value = 0`,
   `s1.value = 1`: `R::slots`; `count(s in R::slots where s.value = 1)`;
   `sum(s in R::slots: s.value)`; `exists(s in R::slots where s.index = 1)`;
   `all(s in R::slots where s.ring = r)`;
   `only(s in R::slots where s.index = 0: s.value)`;
   `seq(i in 0 .. 1: i)` and `seq(i in 1 .. 0: i)`.
2. Type the ring mapping row `items = seq(i in 0 .. self.size - 1: only(s in
   R::slots where s.ring = self and convert<Integer>(s.index) = (self.head + i) mod 2:
   s.value))` and read its declared maximum length.
3. Evaluate the row at the state of step 1, then again with the slot keys
   `s0` and `s1` exchanged (indices and values moved with them).
4. Evaluate `only(s in R::slots where s.index = 2: s.value)` (no match) and
   `only(s in R::slots: s.value)` (two matches) as a field row.
5. Refuse: a `where` condition `s.value`; a `seq` bound `true`; a `count`
   outside a refinement declaration; `count(x in R::missing)`.
6. Type `sum(s in R::slots where s.ring = self: s.value + 1)` against
   `Int[0, 4]` and against `Int[2, 4]` at `slots = {s0, s1}`, and evaluate it
   where no slot satisfies the condition.
7. Read the undefined reason and payload of step 4's two `only` rows.
8. Parse `count(s in R::slots)`, `exists(s in R::slots)`,
   `seq(i in 0..1: i)` and `seq(i in 0 .. 1: i)` in a field row, parse
   `exists(s in R::slots: s.value = 1)` in a state clause, write
   `count(s in R::slots)` in a state clause, and write
   `only(s in R::slots where s.value = 1: s.value)` and
   `seq(i in 0 .. 1: i)` in a state clause.

## Expected Results

Step 1 yields the set `{s0, s1}` typed `Set<Reference<Slot>>[0, 2]`, `1`,
`1`, `true`, `true`, `0`, `[0, 1]` and `[]` (FR-376-AC-1, FR-376-AC-2). Step
2 admits the row with declared maximum length 2 (FR-376-AC-2). Step 3 yields
`items = [1, 0]` both times (FR-376-AC-5, FR-377-AC-1). Step 4 makes the row
undefined at that state, and the refinement settles `refuted`, cause
`undefined-evaluation`, for both the no-match and the two-match `only`
(FR-376-AC-3, FR-377-AC-1, FR-379-AC-6). Step 5 refuses `ill_typed`/`type-mismatch` twice,
`unsupported_construct`/`expression-form`, and `missing_declaration`/`missing-name`
(FR-376-AC-4). Step 6 gives the bound `[0, 4]`, admits `Int[0, 4]`, refuses
`Int[2, 4]` with `ill_typed`/`type-mismatch` and evaluates to `0` (FR-376-AC-6).
Step 7 reads the reason `only-not-one` with the population binding, the `only`
locus and the row, identical for no match and for two (FR-376-AC-7). Step 8
parses the four field-row spellings as the productions of the shared grammar,
reads the state-clause `exists` as the quantifier, scans `0..1` and `0 .. 1`
alike, refuses the state-clause `count(s in R::slots)` with
`unsupported_construct`/`expression-form`, and refuses the state-clause `only`
and `seq` spellings with `invalid_syntax`/`unexpected-token` at their `in`
token (FR-376-AC-8).
