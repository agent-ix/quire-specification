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
operations `beginA`, `commitA`, `retryA`, the same three for B, and `peek`;
enums `Mode { idle, busy }` and `Other { idle }`).
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
    `Option<Int[0, 4]>`, `"a"` for `Text[0, 4; nfc]`, `rational(1, 2)` for
    `Rational[0, 1; 1, 2]`, and `Impl::Mode::idle` for `Impl::Mode`; then,
    each alone, `5` and `true` for `Int[0, 4]`, `0` for `Boolean`, `none` for
    `Int[0, 4]`, `Impl::Other::idle` for `Impl::Mode`, `idle` and
    `Impl::Mode::missing` for `Impl::Mode`, and `sequence[1]` and
    `float64(bits: 0x0000000000000000)` for `Int[0, 4]`;
11. write `ensure fair strong each Spec::Counter::inc;` in place of the
    positive unit's `ensure` row, and separately `assume fair strong
    whole Impl::Counter::retryA;` beside the weak rows, and separately
    `assume fair strong Impl::Counter::retryA;` in its place; then each alone,
    both of those two rows together, `assume fair weak
    Impl::Counter::beginA;` twice, `assume fair weak whole
    Impl::Counter::beginA;` beside `assume fair weak
    Impl::Counter::beginA;`,
    `assume fair each Impl::Counter::beginA;` (no kind) and
    `assume fair always Impl::Counter::beginA;`;
12. request the whole declaration and one per-step obligation for the
    `commitA` row;
13. give the abstract package two protocol clauses `Twice` and `Twice2` with
    identical bodies, and write the header `abstract Spec::Twice` with
    operation-target step rows only; then the same rows under `abstract
    Spec::Twice2` and under `abstract Spec`; then, each alone, the header
    changed to `abstract Spec::Thrice` (no such clause), `abstract
    Spec::Counter` (a type), `abstract Spec::counters` (a population),
    `abstract Nope::Twice` (no such alias) and `abstract Spec::Wrong`, where
    `Wrong` is a clause of the declaring package that acts only on `Impl`;
    then the parse vectors `abstract Spec::Twice::finish concrete Impl`,
    `abstract ::Twice concrete Impl`, `abstract Spec:: concrete Impl`,
    `abstract Spec:: {` and `abstract Spec::1`.

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
source reorder changes none of them (FR-375-AC-7). In mutation 10 every
literal on a type that admits it is admitted; `5`, `true`, `0` for `Boolean`,
`none` for `Int[0, 4]` and `Impl::Other::idle` each refuse
`ill_typed`/`type-mismatch` at the literal; `idle` and `Impl::Mode::missing`
refuse `missing_declaration`/`missing-name`; `sequence[1]` and the float
spelling refuse `invalid_syntax`/`unexpected-token` (FR-375-AC-9). A parser
that admits only integer literals fails this case. In mutation 11 the strong `each` row and the explicit `strong whole` row
are admitted with entries of kind `strong` (granularity `each` and `whole`),
the unmarked strong row gives the same entry as the explicit `whole` row, the
weak rows keep kind `weak`, the node id differs from the positive unit's, and
the kindless and `always` rows each refuse `invalid_syntax`/`unexpected-token`
at the token after `fair` (FR-375-AC-10); the identical pairs, including the
`weak` and `weak whole` pair, each refuse `invalid_model_binding`/
`conflicting-binding` naming both rows (FR-375-AC-3). In step 12 the whole
declaration requires `temporal-satisfaction` and the step obligation
`operation-contract` (FR-375-AC-8).
In mutation 13 the three declarations (`Spec::Twice`, `Spec::Twice2`, `Spec`)
use identical rows and are each admitted with three distinct node ids; the
two protocol subjects carry `protocol` and list the clause in `dependencies`,
and the `abstract Spec` node has neither. `Spec::Thrice`, `Spec::Counter`,
`Spec::counters` and `Nope::Twice` each refuse `missing_declaration`/
`missing-name` at `Thrice`, `Counter`, `counters` and `Nope`; `Spec::Wrong`
refuses `invalid_model_binding`/`malformed-declaration` at `Wrong`. The parse
vectors each refuse `invalid_syntax`/`unexpected-token` before any name is
resolved: at the second `::`, at the leading `::`, at `Impl` (`concrete` read
as the protocol name), at `{` and at `1` respectively (FR-375-AC-11).
