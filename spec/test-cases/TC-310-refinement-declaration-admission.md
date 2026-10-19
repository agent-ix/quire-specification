---
id: TC-310
title: "Refinement declaration admission"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: verifies
---

# TC-310: Refinement declaration admission

## Description

Admit the compare-and-set counter refinement and refuse each malformed row,
checking the emitted refinement node, its identity and the claim kinds its
items require.

## Test Procedure

Subject: abstract package `example/counter-spec` (type `Counter`, field
`value: Int[0, 2]`, population `counters`, operation `inc()` with frame
`modifies value`) and concrete package `example/counter-cas` (type `Counter`
with `value`, `tmpA`, `busyA`, `tmpB`, `busyB`, population `counters`,
operations `beginA`, `commitA`, `retryA`, the same three for B, and `peek`).
The positive unit declares `refinement CasRefinesCounter using inf abstract
Spec concrete Impl` with one population row, one object row
`value = self.value`, one step row per concrete operation (`commitA` and
`commitB` to `Spec::Counter::inc(self)`, every other operation to `stutter`),
`assume fair weak each` rows for the six worker operations and `ensure fair
weak each Spec::Counter::inc`.

Mutations, each applied alone to the positive unit:

1. delete the `peek` step row;
2. replace the deleted `peek` row by `step Impl::Counter::peek -> any;`;
3. delete the population row; then duplicate it;
4. write `value` twice in the object row;
5. write a second step row for `beginA`;
6. map `value = self.busyA` (a `Boolean` for an `Int[0, 2]` field);
7. add `history Impl::Counter.value: Int[0, 4] = 0 { }` (a name the type
   declares); then `history Impl::Counter.commits: Integer = 0 { }`
   (unbounded); then a history row with two updates for `commitA`;
8. write `ensure fair weak Impl::Counter::beginA;` and, separately,
   `assume fair weak Spec::Counter::inc;`; then an `ensure` row with no
   granularity;
9. change the object row expression to `self.value + 0`, and separately
   reorder the step rows in source;
10. write the initial literal of a history row on a type that admits it:
    `-3` for `Int[-5, 4]`, `true` and `false` for `Boolean`, `none` for
    `Option<Int[0, 4]>`, `"a"` for a `Text` type, `rational(1, 2)` for a
    matching `Rational` type, and `Impl::Mode::idle` for the enum `Impl::Mode`;
    then, each alone, `5` and `true` for `Int[0, 4]`, `0` for `Boolean`, `none`
    for `Int[0, 4]`, `Impl::Other::idle` for `Impl::Mode`, `idle` and
    `Impl::Mode::missing` for `Impl::Mode`, and `[1]` and `float64(bits:
    0x0000000000000000)` for `Int[0, 4]`;
11. request the whole declaration and one per-step obligation for the
    `commitA` row.

## Expected Results

The positive unit is admitted with one `relation`/`refinement` node whose
body carries every row, sorted as FR-375 states (FR-375-AC-1). Mutation 1
refuses `missing_declaration`/`missing-name` naming `peek`, and mutation 2
is admitted (FR-375-AC-2). Mutations 3 to 5 refuse with the named causes,
naming both rows where two are involved (FR-375-AC-3). Mutation 6 refuses
`ill_typed`/`type-mismatch` at the row expression (FR-375-AC-4). Each
mutation of 7 refuses with its named cause, and the admitted positive history
row of TC-314 appears in no concrete clause, frame or successor relation
(FR-375-AC-5). Mutation 8 refuses `missing_declaration`/`missing-name` for
the two misplaced rows, and the unmarked row's emitted granularity is `whole`
(FR-375-AC-6). In mutation 9 the expression change changes the node id, the
`package_id` and the obligation identity of the refinement item, and the
source reorder changes none of them (FR-375-AC-7). In mutation 10 every literal on a type that admits it is admitted;
`5`, `true`, `0` for `Boolean`, `none` for `Int[0, 4]` and
`Impl::Other::idle` each refuse `ill_typed`/`type-mismatch` at the literal;
`idle` and `Impl::Mode::missing` refuse `missing_declaration`/`missing-name`;
`[1]` and the float spelling refuse `invalid_syntax`/`unexpected-token`
(FR-375-AC-9). A parser that admits only integer literals fails this case.
In step 11 the whole
declaration requires `temporal-satisfaction` and the step obligation
`operation-contract` (FR-375-AC-8).
