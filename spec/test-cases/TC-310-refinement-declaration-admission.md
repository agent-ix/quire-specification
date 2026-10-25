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
    selection and `definition_selections` each time; then, in a separate run
    on the floor-law unit, add exactly these declarations: the profiles
    `profile p = "quire.protocol.complete/v1";` and
    `profile m = "quire.model.complete/v1";` (the unit-level profile `m`
    admits the clauses below and changes that run's lock selection, which the
    run does not read; it does not widen the refinement's row layers); the
    clauses `pre PutPre using m on R::Slot::put { self.ring.size < 2 }` and
    `post PutPost using m on R::Slot::put { self.value = x }`; the protocol

    ```text
    protocol Fill using p over (k: R::Ring) on origin {
      role w on R::Ring;
      run sequence Main {
        attempt A by w on R::Slot::put contracts [PutPre, PutPost] as (a: R::Ring) { true };
      }
      finish Done as (d: R::Ring) { true };
    }
    ```

    (the shape of TC-376's `Fill`, written over the unit's own model `R`, with
    typed bindings as the protocol grammar requires); the abstract `Queue`
    field `ready: Boolean`; and the row `ready = protocol Fill`. The
    refinement stays `using inf`, the infinite-trace alias, and the protocol
    has its own alias `p`;
22. a plain temporal clause under `quire.temporal.infinite-trace/v1`, with no
    refinement, in a unit that selects no value layer.
23. a protocol-subject refinement whose rows include `step duplicate`, `step lose`, `step spawn`, `step retire`, `step fork`, `step join`, a template's `cattempt` and `cend` rows, `step memory flush` and `step memory drain`, written in one order and then in the exact reverse of it, so every tied pair and the two memory rows swap their relative order.
24. select one abstract model `Spec` whose declaration node is local to the
    declaring package and has no populations, and two distinct
    concrete model declarations `ImplA` and `ImplB`, each with no operations;
    declare one refinement from `Spec` to each concrete model under the same
    infinite-trace profile, with empty `populations`, `objects`, `steps`,
    `histories`, `assume` and `ensure` arrays and otherwise identical bodies.
    Compare each node's `semantic_type`, closed node-id preimage and node id.
25. repeat mutation 13 with the selected abstract protocol clause declared
    in the unit that declares the refinement, so the clause node is local to
    the refinement's checked package; compare its `protocol` key and
    `dependencies` with mutation 13's directly selected dependency clause.
26. **Checked-package read/write vectors (RW-01 to RW-08).** Emit the
    admitted package of mutation 24 with only the `ImplA` refinement as
    `RW-01`. Keep a second local abstract model `Spec2`, with no populations
    or operations, in that package for `RW-03`; its key is distinct from
    `Spec`'s. Pass the emitted canonical bytes through the strict V2 reader,
    then write the admitted package and read it again. Use a separate
    producer-emitted package of mutation 2 (`peek -> any`) as the base for
    `RW-04`, so the refinement is not proof-eligible and carries only its
    `declaration` occurrence. Use mutation 13's producer-emitted package as
    the base for `RW-06`, with `abstract` a `PackageNodeKey` of a directly
    selected dependency. Use the positive unit's producer-emitted package as
    the base for `RW-07` and `RW-08`.

    For each edited wire below, apply only that edit to its named base. Keep
    all other nodes and selected inputs valid. Recompute the package's
    identity projection and `package_id` after the edit. Except for the two
    deliberately stale-key cases, recompute the refinement's closed
    `quire.refinement-node/v1` key, rewrite references and source-map
    occurrence keys to the new node id, and rebuild its exact local
    `dependencies` list in digest order. For `RW-06`, remove the direct
    dependency selection and the supplied dependency while leaving the
    external `PackageNodeKey` in the body; update the lock and selection
    preimage as well. No vector relies on a stale package id, a stale source
    map, a missing supplied package for a still-selected entry, or an
    unrelated graph-shape refusal. Let `{n}` be the refinement's index in
    `semantic_graph.nodes` after the edit and `{j}` the index of the first
    row that breaks the order in `RW-04`.

    | Vector | Base and single wire edit |
    | --- | --- |
    | `RW-01` | Unedited row-empty `ImplA` package; write, read, write and read again. |
    | `RW-02` | `RW-01`: replace only `semantic_type` with the existing local `ImplB` model key, retaining the refinement `node_id`. |
    | `RW-03` | `RW-01`: replace only `body.abstract` with local `Spec2`'s key, rebuild `dependencies`, but retain the refinement `node_id`. |
    | `RW-04` | Mutation 2 package: swap the first two distinct `steps` entries in the wire while preserving their entry values; rekey the refinement. |
    | `RW-05` | `RW-01`: set `body.abstract` to a well-formed `NodeRef` absent from this graph, with no dependency package selector; rekey the refinement. |
    | `RW-06` | Mutation 13 package: remove the direct selection for the external package named by `body.abstract`, leaving that `PackageNodeKey` unchanged; rekey the refinement. |
    | `RW-07` | Positive package: replace `body.objects[0].concrete_type` with the existing local Boolean `scalar_type` key; rekey the refinement. |
    | `RW-08` | Positive package: replace `body.objects[0].fields[0].expression` with the inline Boolean literal `{term: "literal", type: B, value_kind: "boolean", value: true}`, where `B` is the package's local Boolean node key; rekey the refinement. |

## Expected Results

For every producer-emitted refinement with step rows, inspect the closed
`refinement_step_rows` sidecar: one entry per `body.steps` position names the
retained refinement node id, that position's zero-based ordinal, and the
exact original checked source row region. Reverse the source order of two
semantically distinct rows while keeping the same canonical `steps` order:
the sidecar regions follow their rows, and the refinement node id and
`package_id` remain equal. For a proof-eligible refinement, each generated
owner occurrence at ordinal `i + 1` has that sidecar row region. For a
refinement made ineligible by an `any` target, the sidecar still has its
rows and the owner has no generated occurrence (FR-375-AC-22,
FR-322-AC-129).

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
two protocol subjects carry `protocol` as a `PackageNodeKey` and join the
external clause through `dependency_selections`, without listing it in their
local `dependencies`,
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
vector); the `ready = protocol Fill` row, in the separate run, is the only refusal of that run (the other declarations listed are each admitted) and refuses `unsupported_construct`/
`expression-form` at the `protocol` token naming `quire.protocol.complete/v1`, the layer that admits the form and that the rows' layer set lacks
(FR-375-AC-18, FR-001-AC-7, FR-090-AC-10, FR-250-AC-8). In mutation 22 the
clause is admitted and the unit's selection gains no value layer or lock row
(FR-090-AC-10). In mutation 23 the node's `steps` array is identical for the two source orders, the tied rows ordered by alias-free class spelling in UTF-8 byte order and the two `memory` rows after every declaration-naming row, `drain` before `flush` (FR-375-AC-19).
In mutation 24 both refinements are admitted, their bodies are identical,
their `semantic_type` keys differ, both closed preimages carry their respective
`semantic_type`, and their recomputed node ids differ. The local abstract
model key appears in each node's `dependencies` despite the empty row arrays
(FR-375-AC-7). In mutation 25 the local clause is a `NodeRef` in `protocol` and appears in
`dependencies`; mutation 13's external clause remains a `PackageNodeKey`
joined through the direct dependency selection (FR-375-AC-11).

In mutation 26, `RW-01` is admitted on both reads; its rewritten canonical
package has the same refinement node id and `package_id` as the first write.
Every other vector is schema-valid and identity-consistent at the package
level, so the strict reader returns `refused`, exposes no admitted package,
and writes no package from that read. The first refusal is exactly:

| Vector | Code / cause | Locus and reader step |
| --- | --- | --- |
| `RW-02` | `invalid_package` / `stale-node-key` | `/semantic_graph/nodes/{n}/node_id`, refinement step 1; the changed `semantic_type` enters the dedicated preimage. |
| `RW-03` | `invalid_package` / `stale-node-key` | `/semantic_graph/nodes/{n}/node_id`, refinement step 1; the changed body enters the same preimage. |
| `RW-04` | `invalid_semantic_graph` / no cause | `/semantic_graph/nodes/{n}/body/steps/{j}`, refinement step 2. |
| `RW-05` | `missing_declaration` / `missing-name` | `/semantic_graph/nodes/{n}/body/abstract`, refinement step 3; a direct dependency holding the same digest does not satisfy the local key. |
| `RW-06` | `missing_declaration` / `missing-selection` | `/semantic_graph/nodes/{n}/body/abstract/package`, refinement step 3; supplying the removed dependency without restoring its direct selection changes nothing. |
| `RW-07` | `invalid_model_binding` / `malformed-declaration` | `/semantic_graph/nodes/{n}/body/objects/0/concrete_type`, refinement step 4. |
| `RW-08` | `ill_typed` / `type-mismatch` | `/semantic_graph/nodes/{n}/body/objects/0/fields/0/expression`, refinement step 4. |

`RW-02` and `RW-03` are FR-375-AC-20's retained-key refusals.
`RW-04` to `RW-08` exercise FR-375-AC-21's ordered checks and both row
target and row type joins. Each is refused after the earlier FR-322 stages
have admitted its shape, source map, dependency list and package identity.
