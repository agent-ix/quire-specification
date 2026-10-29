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
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-310: Refinement declaration admission

## Description

Admit the compare-and-set counter refinement and refuse each malformed row,
checking the emitted refinement node, its identity, object-type and
state-machine field-set rows, and the claim kinds its items require.

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
    and compare it and the unit's selection of it with those
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
    Compare each node's `semantic_type` and node id.
25. repeat mutation 13 with the selected abstract protocol clause declared
    in the unit that declares the refinement, so the clause node is local to
    the refinement's checked package; compare its `protocol` key and
    `dependencies` with mutation 13's directly selected dependency clause.
26. **Checked-package read/write vectors (RW-01 to RW-08).** Emit the
    admitted package of mutation 24 with only the `ImplA` refinement as
    `RW-01`. Pass the emitted canonical bytes through the strict V2 reader,
    then write the admitted package and read it again. Use a separate
    producer-emitted package of mutation 2 (`peek -> any`) as the base for
    `RW-04`, so the refinement is not proof-eligible and carries only its
    `declaration` occurrence. Use mutation 13's producer-emitted package as
    the base for `RW-06`, with `abstract` a `{package, node}` key of a directly
    selected dependency. Use the positive unit's producer-emitted package as
    the base for `RW-07` and `RW-08`.

    For each edited wire below, apply only that edit to its named base. Keep
    all other nodes and selected inputs valid. Rebuild the refinement's exact local
    `dependencies` list in node-id order. For `RW-06`, remove the direct
    dependency selection and the supplied dependency while leaving the
    external `{package, node}` key in the body; update the lock as well. No vector
    relies on a stale source
    map, a missing supplied package for a still-selected entry, or an
    unrelated graph-shape refusal. Let `{n}` be the refinement's index in
    `semantic_graph.nodes` after the edit and `{j}` the index of the first
    row that breaks the order in `RW-04`.

    | Vector | Base and single wire edit |
    | --- | --- |
    | `RW-01` | Unedited row-empty `ImplA` package; write, read, write and read again. |
    | `RW-04` | Mutation 2 package: swap the first two distinct `steps` entries in the wire while preserving their entry values. |
    | `RW-05` | `RW-01`: set `body.abstract` to a well-formed `NodeRef` absent from this graph, with no dependency package selector. |
    | `RW-06` | Mutation 13 package: remove the direct selection for the external package named by `body.abstract`, leaving that `{package, node}` key unchanged. |
    | `RW-07` | Positive package: replace `body.objects[0].concrete_type` with the existing local Boolean `scalar_type` key. |
    | `RW-08` | Positive package: replace `body.objects[0].fields[0].expression` with the inline Boolean literal `{term: "literal", type: B, value_kind: "boolean", value: true}`, where `B` is the package's local Boolean node id. |

27. **State-machine field-set rows (SM-01 to SM-05).** Build an abstract
    package with a `state_machine` declaration `SpecMachine` and a concrete
    package with a `state_machine` declaration `ImplMachine`; each has a
    declared field `value: Int[0, 2]`. Emit a refinement whose object row maps
    `SpecMachine::value = self.value`, with no population row. Repeat with two
    state-machine declarations on each side and one object row for each pair.
    Also emit the ordinary `CasRefinesCounter` object-type-only refinement.
    For malformed cases, omit the abstract machine's row, duplicate it, or
    replace its concrete machine key with an object-type key. For the accepted
    cases, check the closed row shape and canonical object and field order;
    pass the bytes through both V2 schemas and the strict reader. `SM-03`
    replaces the concrete
    machine key with an object-type key; `SM-04` omits the abstract machine's
    row; `SM-05` duplicates that row.
    For each selected model package in these writes, also verify exactly one
    `model_root` node whose `ModelRootOwner` identity and version join the
    selected `model_root` node, whose generated occurrence ordinal zero
    maps to the model-selection source region, and whose identity is derived
    from the selected package identity and version, independent of alias
    spelling. Verify the model-only
    refinement's `abstract` and `semantic_type` name the roots, while a
    protocol refinement keeps `protocol` at the exact clause key and the
    clause's `owner_model` unchanged. The model root is derived from existing
    model selection; the test adds no FCD field.
28. **Root owner joins (MR-01 to MR-04).** Starting from the admitted selected
    model package, remove `owner` from its root (`MR-01`), add an extra
    unknown member to `ModelRootOwner` (`MR-02`), attach `ModelRootOwner` to
    an object-type node (`MR-03`), and add a second root whose owner identity
    names no supplied domain package (`MR-04`). Validate `MR-01` to `MR-03` against both V2 schemas
    (`schema.json`) and `MR-04` against `schema.json`,
    which admits it; validate every mutation with the strict reader. The
    vectors are the RFC 6902 patches of `model-root-vectors.json`.
29. **Selected-root fixtures (MR-05 to MR-09).** For every published positive
    package with `model_root` nodes, derive the root and its source-map entry
    from each row alone and compare them with the package's, last in graph
    order and last in the source map (`MR-05`, FR-322-AC-134); check that each root's `generated` ordinal-zero entry has
    at least one region, each naming a lock source, without claiming it is the
    selecting range (`MR-07`, FR-322-AC-136); derive the published
    model-root vector from its selection (`MR-08`, FR-322-AC-137); repeat the
    root, which the reader refuses `invalid_semantic_graph` at the second
    node's `node_id` (`MR-09`). `MR-04` refuses
    `missing_import`/`missing-selection` at
    `/semantic_graph/nodes/{i}/owner/identity` and is schema-valid. The model-root
    vector is a structural id, `{package, path, ordinal}`, compared
    componentwise.
30. **Refinement identity (RI-01 to RI-04) and step provenance (SP-01 to
    SP-11, each run with the class catalog (`c`) and without (`f`): 22 cases;
    OC-01 to OC-04; BI-01 to BI-07).** Build two refinements of one unit that differ
    only in name (`Alpha`, `Beta`): both are admitted as two nodes whose
    ids differ, and the package emits no internal failure
    (FR-375-AC-29); a body without `name` fails both schemas; two nodes with
    equal `name` refuse `ambiguous_declaration`/`ambiguous-name` at the later
    node's `body/name` (FR-375-AC-30). For the step provenance vectors, an
    operation class `Counter::beginA` is admitted with the no-point anchor of
    (`Counter`, `beginA`), with and without a `class_catalog`, and refused with
    the concrete model root, the model-owner node, another operation's anchor
    or a point-qualified anchor as `source_node`, and with none
    (FR-375-AC-31); `Sub::reset` of an inherited operation is a missing class,
    resolved before `source_node`; an anchor no clause names carries one
    `anchor` occurrence, the operation's declaration region (OC-01 to OC-04).
    For the binder vectors, `self` at level 0 and the parameters at levels
    1..n by declared name are admitted; a wrong name, level or type, and a
    step row with a `result` binder, are refused (FR-375-AC-32).
31. **Selected model roots (SR-01 to SR-13).** Five
    selected models are used (`acme/spec` the abstract `RA`, `acme/impl` `RB`,
    `acme/other` `RC`, `acme/bare` `RD`, `acme/plain` `RF`), each root being
    its selection's `model_root` id. They are graph cases, not
    wire packages: no two-selection wire fixture is published, so only the
    root ids and the row-empty node ids (SR-07) are compared.
    SR-01 is admitted. SR-02 swaps `RB` to `RC` with rows kept and refuses
    `invalid_model_binding`/`malformed-declaration` at the first foreign key in
    array order (`objects` before `steps`). SR-03 is row-empty over a protocol
    subject: the concrete catalog lists `Impl::Do`, which `RC` does not derive,
    and it refuses `invalid_semantic_graph` at that entry. SR-04 admits an
    empty catalog over `RD`, SR-05 swaps it to `RC`, which owns `Other::Go`,
    and refuses `invalid_semantic_graph` at the catalog array, and SR-06 swaps
    it to `RF`, which derives the same empty set: admitted. SR-07 is a
    row-empty model subject with no catalog: admitted under `RB` and `RC` as
    two nodes differing by `semantic_type` alone. SR-08 has a foreign catalog
    class and a foreign row key and refuses at the row (step 5 precedes step
    7); SR-09 has an absent protocol catalog and a foreign row and refuses at
    `/body/class_catalog` (presence precedes rows); SR-10 refuses a foreign
    `histories[].updates[].operation`. SR-11 is the reflexive refinement
    (`semantic_type` is `RA`), admitted (FR-375-AC-34). SR-12 and SR-13 fix the
    tie-break between an omitted and an extra class by catalog-key order:
    `[Zed::Run]` against derived `[Other::Go]` refuses at the `concrete` array,
    `[Aaa::Run]` at entry 0 (FR-375-AC-33).

## Expected Results

For every producer-emitted refinement with step rows, inspect the closed
`refinement_step_rows` sidecar: one entry per `body.steps` position names the
retained refinement node id, that position's zero-based ordinal, and the
exact original checked source row region. Reverse the source order of two
semantically distinct rows while keeping the same canonical `steps` order:
the sidecar regions follow their rows, and the refinement node id
remains equal. For a proof-eligible refinement, each generated
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
(FR-375-AC-6). In mutation 9 the expression change changes the node's value and the
obligation identity of the refinement item, and the
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
two protocol subjects carry `protocol` as a `{package, node}` key and join the
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
mutation 15 the node bodies and obligation identities differ
between the non-empty and the absent value and are equal across the
reordering and across the alias rename (FR-375-AC-13). In mutation 16 the
library given its list is admitted, with a node and an
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
their `semantic_type` keys differ, and each node carries its respective
`semantic_type`. The local abstract
model key appears in each node's `dependencies` despite the empty row arrays
(FR-375-AC-7). In mutation 25 the local clause is a `NodeRef` in `protocol` and appears in
`dependencies`; mutation 13's external clause remains a `{package, node}` key
joined through the direct dependency selection (FR-375-AC-11).

For the checked class catalog, start from a producer-emitted protocol refinement
with one concrete `parallel` node (`fork` and `join`), one concrete attempt,
one abstract attempt with receiver and argument binders, and a `tso` parallel.
Read the emitted `class_catalog`. Repeat with an abstract model subject and
with a selected abstract protocol clause in a directly selected dependency.
For each wire variant below, keep the source row sidecar and generated
`source_map` occurrences valid. Rewrite the refinement's references; rebuild its exact dependencies
except in `CC-07` (FR-375-AC-23).

| Vector | Single catalog or row change | Expected result |
| --- | --- | --- |
| `CC-01` | Unedited producer catalog | Admitted; concrete `fork` and `join` are distinct sorted classes of the same parallel node, the abstract attempt's ordered binder types match its checked operation, and `memory flush` records its `tso` source default. External abstract keys remain `{package, node}` keys. |
| `CC-03` | Remove `join` and its deciding step row together; separately repeat `fork`, reverse two entries, or add a class of another model | Duplicate or reversed entries refuse `invalid_semantic_graph` in refinement step 2; omission or foreign class refuses at the catalog array or entry in protocol step 7. |
| `CC-04` | Associate `fork` with an existing unrelated control key and change its deciding row's `source_node` to the same key; separately change the abstract attempt binder type to another existing type while its target argument list is empty | Both refuse `invalid_model_binding`/`malformed-declaration` at the catalog entry in protocol step 7, with row/catalog consistency intact. |
| `CC-05` | Change a step's `source_class`, `source_node`, target class or target node independently of the catalog | Missing class refuses `missing_declaration`/`missing-name`; wrong node or subject refuses `invalid_model_binding`/`malformed-declaration`, at the row member in step 5. |
| `CC-06` | Give an abstract attempt target zero arguments, then the complete typed list, then one fewer or a wrong-typed argument | Both first forms admit (zero leaves all binders open); short nonempty list refuses `ill_typed`/`operator-ineligible`, wrong type `ill_typed`/`type-mismatch`, in step 5. |
| `CC-07` | Remove one local catalog node or binder type from the exact `dependencies`; retain an external catalog key there | `invalid_semantic_graph` at `/dependencies` before the refinement step. |
| `CC-08` | Replace a source-default `tso` `memory flush` class and its deciding row with the same absent kind; separately omit the catalog on a protocol refinement | The absent kind refuses `invalid_semantic_graph` at the class entry in protocol step 7; missing catalog refuses at `/body/class_catalog` in refinement step 4. |
| `CC-09` | `RW-01`'s model-only refinement with no catalog; then add an empty catalog | The first admits; the second refuses `invalid_semantic_graph` at `/body/class_catalog`, reader step 4. |
| `CC-10` | From `CC-01`, request analysis twice over the same admitted package: once with its source-default `tso` choice and once with an override to `sc` under FR-434 | The checked catalog is byte-identical; the resolved memory model and resulting obligation identities differ. Package admission makes no claim that an override's memory class is a source class. |
| `CC-11` | From `CC-04`'s wrong `fork` association and matching row, also make an independent protocol-body defect: a `send` binder whose type disagrees with its channel's `carries` type (FR-466-AC-7). Rewrite every changed send-node id in the graph, catalog and deciding row, and keep the catalog binder types equal to the edited binder while preserving the wrong `fork` association. First read with both defects; then repair only the send binder type and rebuild affected node ids and dependencies; separately introduce a step-row class mismatch with both defects present. | The first read reports the binder-type refusal in protocol step 4 before the catalog graph join; after repair, the catalog association refuses in protocol step 7; the separate row mismatch refuses in the earlier refinement step 5 before either protocol defect. |
| `CC-12` | Use source `parallel` controls whose defaults include `sc`, `tso` and `ra`; supply, in canonical order `ra`, `sc`, `tso`, a `memory drain` entry for each, carrying `drain: store-buffer-flush`, plus a `memory flush` entry for `tso` without `drain` | Both schemas admit the entries; the catalog is reconstructed from the subject's source defaults. Drain flushes pending writes under the selected model's visibility rule, is a no-op under `sc`, and under `ra` gives no additional visibility or ordering beyond FR-437. |
| `CC-13` | Omit `drain` from `memory drain`, use another drain spelling, add `drain` to `memory flush`, or pair `memory flush` with `sc` or `ra` | Both schemas refuse the malformed memory-class entry. |
| `CC-14` | Give two protocol clauses distinct owner-model edges and a refinement with no deciding rows; then retarget one clause's edge and rebuild it | Classes of every clause whose `owner_model` is the object-type node of a model owned by the selected concrete declaration, even with no deciding rows, are included; the foreign clause is excluded; retargeting changes the clause identity and which concrete catalog receives its classes. |
| `CC-15` | From `CC-12`, reorder the three `memory drain` entries to `tso`, `sc`, `ra`; separately repeat the `sc` drain entry; separately repeat a declared class entry; separately select `tso` by two source `parallel` controls | Reordered, repeated-pair and repeated-spelling catalogs refuse `invalid_semantic_graph` at the first out-of-order or later duplicate entry in reader step 2; two controls selecting `tso` yield one `tso` drain and one `tso` flush entry. The canonical `CC-12` order is `memory drain` for `ra`, `sc`, `tso`, then `memory flush` for `tso`. Uniqueness is by (`class`, `memory_model`) (FR-375-AC-26). |
| `CC-16` | A concrete protocol clause `activation on each (x: T)` with non-record `T`, then with record `T{p, q}`, then `activation on origin`; the abstract clause likewise | `on each` catalogues `activate <protocol>` with the clause node and `binder_types` `[T]`, then one key per field of `T` in effective field order; `on origin` catalogues none (FR-375-AC-27). |
| `CC-17` | From `CC-16`, associate `activate` with the trigger's parameter node instead of its clause, drop the entry, add one for an `origin` clause, swap two `binder_types`, or give a wrong type | A wrong node association, a swapped or wrong binder type each refuse `invalid_model_binding`/`malformed-declaration` at the catalog entry in FR-464 step 7; a dropped entry refuses `invalid_semantic_graph` at the catalog array, and an entry added for an `origin` clause refuses `invalid_semantic_graph` at that entry. |
| `CC-18` | Concrete protocol `Impl` with attempt `Do`, effect `E`, commit `K`, event `ev` and template `T` (with `cend`) under `sc`, `tso` and `ra` parallels (so `tso` also contributes `memory flush`), and abstract protocol `Req` with attempt `Ask`, effect `AE` and commit `AK`; read the producer catalog. Independent mutations of it, each from the unedited catalog: (a) drop `Impl::E`; (b) drop `Impl::K`; (c) drop `cend Impl::T`; (d) repeat the `Impl::E` entry; (e) add `cend Impl::Do` (an attempt, not a template) in canonical position; (f) add `Impl::Ghost` naming no node; (g) set the `Impl::K` entry's `node` to `E`'s key | The unedited concrete array is, in order: `Impl::Do`, `Impl::E`, `Impl::K`, `Impl::T`, `Impl::ev`, `cend Impl::T`, then `memory drain` for `ra`, `sc`, `tso`, then `memory flush` for `tso`; the abstract array is `Req::AE`, `Req::AK`, `Req::Ask`; admitted (the wire form is `CCF-01` of `class-catalog-vectors.json`). (a), (b), (c) each refuse `invalid_semantic_graph` at `/semantic_graph/nodes/{n}/body/class_catalog/concrete`; (d) refuses `invalid_semantic_graph` at the later duplicate entry, reader step 2; (e) and (f) each refuse `invalid_semantic_graph` at the added entry, protocol step 7; (g) refuses `invalid_model_binding`/`malformed-declaration` at the `Impl::K` entry, protocol step 7 (FR-375-AC-28). |
| `CC-19` | `CC-18` with a protocol that has no effect, commit or template node; then add an `Impl::E` entry naming no node | The first is admitted with no effect, commit or `cend` entry; the second refuses `invalid_semantic_graph` at the added entry (FR-375-AC-28). |

In mutation 26, `RW-01` is admitted on both reads; its rewritten canonical
package has the same refinement node id as the first write.
Every other vector is schema-valid at the package
level, so the strict reader returns `refused`, exposes no admitted package,
and writes no package from that read. The first refusal is exactly:

| Vector | Code / cause | Locus and reader step |
| --- | --- | --- |
| `RW-04` | `invalid_semantic_graph` / no cause | `/semantic_graph/nodes/{n}/body/steps/{j}`, refinement step 2. |
| `RW-05` | `missing_declaration` / `missing-name` | `/semantic_graph/nodes/{n}/body/abstract`, refinement step 3; a direct dependency holding the same node id does not satisfy the local key. |
| `RW-06` | `missing_declaration` / `missing-selection` | `/semantic_graph/nodes/{n}/body/abstract/package`, refinement step 3; supplying the removed dependency without restoring its direct selection changes nothing. |
| `RW-07` | `invalid_model_binding` / `malformed-declaration` | `/semantic_graph/nodes/{n}/body/objects/0/concrete_type`, refinement step 5. |
| `RW-08` | `ill_typed` / `type-mismatch` | `/semantic_graph/nodes/{n}/body/objects/0/fields/0/expression`, refinement step 5. |

`RW-04` to `RW-08` exercise FR-375-AC-21's ordered checks and both row
target and row type joins. Each is refused after the earlier FR-322 stages
have admitted its shape, source map and dependency list.

For mutation 27, `SM-01`, `SM-02` and the ordinary object-type-only case are
admitted with the existing `objects` entry shape and no additional wire
member. The state-machine type keys are present in local dependencies and the
row has the same closed shape as an object-type row. `SM-03` refuses
`invalid_model_binding`/`malformed-declaration` at
`objects[0].concrete_type`; `SM-04` refuses
`missing_declaration`/`missing-name` naming the unmapped abstract machine;
`SM-05` refuses `invalid_model_binding`/`conflicting-binding` naming the
duplicate rows. The object-type-only refinement is admitted without a
state-machine node or synthetic anchor. Each selected package has exactly
one `model_root` with `ModelRootOwner` joined to its selection row, generated
occurrence zero mapped to the existing model-selection source region, and an
id equal to the published model-root vector. Its alias is not in the
id. For a model subject, `abstract` and the refinement's
`semantic_type` identify the selected roots; for a protocol subject,
`abstract` identifies the abstract root, `protocol` identifies the exact
clause, and the clause's `owner_model` remains its exact declaring node. The
reader admits both V2 schemas and the package without a new FCD field.

For mutation 28, `MR-01` fails the schema because the root lacks its required
owner; `MR-02` fails the closed-owner schema; `MR-03` fails the graph-owner
rule because a root owner is restricted to `model`/`model_root`. `MR-04`
refuses `missing_import`/`missing-selection` at the root owner identity
because it names no supplied domain package; no digest or version is compared.
