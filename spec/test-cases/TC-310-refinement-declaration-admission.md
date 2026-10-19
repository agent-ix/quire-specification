---
id: TC-310
title: "Refinement declaration admission"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-001
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-090
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-250
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
    `Wrong` is a clause of `example/counter-spec` that acts only on a second
    model of that package, `Other`;
    then the parse vectors `abstract Spec::Twice::finish concrete Impl`,
    `abstract ::Twice concrete Impl`, `abstract Spec:: concrete Impl`,
    `abstract Spec:: {` and `abstract Spec::1`;
14. with mutation 13's `abstract Spec::Twice` unit and no internal row, supply
    the `refinement_internals` value `[{"refinement": "CasTwice", "internal":
    ["Spec::Twice::finish"]}]` to the compilation; then the value absent, `[]`
    and an object for another refinement;
15. compare the node of mutation 14 with the non-empty value against the node
    with the value absent; with the value's object and entries reordered; and
    with the abstract alias renamed `S` throughout (the value written
    `S::Twice::finish`);
16. a supplied library `lib` that declares its own refinement `LibR`: supply
    it with `[{"refinement": "LibR", "internal": ["A::Twice::finish"]}]`
    and compare its `package_id` and the unit's selection of it with those
    of the same library given no value; then supply the value
    `[{"refinement": "CasTwice", "internal": []}]` to the library, which does
    not declare `CasTwice`, with a unit that imports `lib` and with a unit
    that does not;
17. run against the producer under test, for each feature its support set
    omits (a producer that has built all three has nothing to run here):
    a declaration with two `history` rows; `abstract Spec::Twice` with no
    list; the list of mutation 14 under `abstract Spec` and under `abstract
    Spec::Twice`, and, when both `protocol-abstract-side` and
    `refinement-internal-list` are omitted, under `abstract Spec::Twice`;
    then the lists `["Spec::Twice::zz"]` (no such node), a repeated
    spelling, `{}`, `[{"refinement": "Nope", "internal": []}]` and
    `[{"refinement": 7, "internal": []}]`, each with `refinement-internal-list`
    omitted;
18. against a producer whose support set omits `nested-reference-row`, change
    the positive unit so a `Sequence<Reference<Queue>>[0, 2]` abstract field
    is mapped by a row of type `Sequence<Reference<Ring>>[0, 2]`, so an
    abstract `?` field of type `Reference<Queue>` is mapped by
    `Some(self.ring)`, and so a `Sequence<Reference<Ring>>` step argument is
    given for a `Sequence<Reference<Queue>>` parameter; separately map a
    whole-population reference row and a plain `Reference<Ring>` row to
    `Reference<Queue>` fields;
19. give the abstract `Queue` of the ring-buffer pair a relationship, under a
    producer whose support set holds every feature;
20. map a `?` abstract field by `none`, by `Some(self.value)`, by
    `self.value` and by `self.f` over a concrete `?` field; map an
    `Option<Int[0, 2]>` abstract field by `none`; map a field by `none` as a
    call argument; write `Some(1)` in a state clause with no function `Some`
    declared, and with one declared;
21. under `quire.temporal.infinite-trace/v1` and a unit whose own declarations
    select no layer, with the ring-buffer rows `s.ring = self` and
    `(self.head + i) mod 2`, add a row `a div b`; supply the unit's
    `integer_division_profile` choice as the lock selection's alternative
    role, three times: floor, Euclidean and none; read the unit's lock
    selection and `definition_selections` each time; then declare in the same
    unit a local protocol `Fill` (the body of TC-376's `Fill`, over the unit's
    own model `R`), give the abstract `Queue` a field `ready: Boolean` and
    write the row `ready = protocol Fill`;
22. a plain temporal clause under `quire.temporal.infinite-trace/v1`, with no
    refinement, in a unit that selects no value layer.

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
refuses `invalid_model_binding`/`malformed-declaration` at `Wrong`, since
the clause is found in the abstract package but does not act on `Spec`'s model. The parse
vectors each refuse `invalid_syntax`/`unexpected-token` before any name is
resolved: at the second `::`, at the leading `::`, at `Impl` (`concrete` read
as the protocol name), at `{` and at `1` respectively (FR-375-AC-11).
In mutation 14 the non-empty value admits the declaration with
`abstract_internal` `["Twice::finish"]` (alias-free) and the three empty
variants admit it with no `abstract_internal` entry (FR-375-AC-12). In
mutation 15 the node ids, `package_id`s and obligation identities differ
between the non-empty and the absent value and are equal across the
reordering and across the alias rename (FR-375-AC-13). In mutation 16 the
library given its list is admitted, with a node, a `package_id` and an
importer selection that differ from the library given none, and the unit
admitted; the value naming a refinement `lib` does not declare refuses
`missing_declaration`/`missing-name` at
(`lib`'s value, `/0/refinement`), and the compilation is
not admitted whether or not the unit imports `lib` (FR-375-AC-12). In
mutation 17 each omitted feature gives `unsupported_construct`/
`not-yet-implemented` as FR-375-AC-14 states: two refusals at the `history`
keywords naming `history-row`; one at `Twice` naming `protocol-abstract-side`;
one at `Spec` naming `refinement-internal-list` under each header, so two
refusals under `abstract Spec::Twice` when both features are omitted; no node
is produced and no other diagnostic is reported for the refused row. With
`refinement-internal-list` omitted, the `zz` list reports
`missing_declaration`/`missing-name` at `/0/internal/0`, the repeated spelling
`conflicting-binding` and `{}` `malformed-declaration`, `Nope`
`missing_declaration`/`missing-name` at `/0/refinement` and the number
`malformed-declaration` at `/0/refinement`, with no support-set refusal for
any of them.

In mutation 18 each of the first three changes refuses
`unsupported_construct`/`not-yet-implemented` naming `nested-reference-row` at
its expression, except the step argument, which refuses `ill_typed`/
`type-mismatch`, with no node, while the population-reference and
plain-reference rows are admitted (FR-375-AC-15). In mutation 19 the
declaration refuses `unsupported_construct`/`declaration-form` at the
abstract alias of the header, once, naming the selected profile, with no node
(FR-375-AC-16). In mutation 20 `none` and `Some(self.value)` are admitted for
the `?` field, `self.value` and `self.f` refuse `ill_typed`/`type-mismatch`,
the `Option<Int[0, 2]>` field takes `none` as an explicit `none`, `none` as a
call argument refuses `ill_typed`/`type-mismatch`, and `Some(1)` refuses
`missing_declaration`/`missing-name` at `Some` with no function declared and is
a call to it with one declared (FR-375-AC-17). In mutation 21 the ring-buffer rows and the
`a div b` row are admitted; the lock selection holds the lock's `always` rows
and the floor law for the first run and the Euclidean law for the second, so
`definition_selections` differ exactly in that role, and the third run refuses
`selection_trigger_unsatisfied` (the canonical `division-missing-for-div-rem`
vector); the `ready = protocol Fill` row refuses `unsupported_construct`/
`expression-form` at the `protocol` token naming `quire.model.complete/v1`
(FR-375-AC-18, FR-001-AC-7, FR-090-AC-10, FR-250-AC-8). In mutation 22 the
clause is admitted and the unit's selection gains no value layer or lock row
(FR-090-AC-10).
