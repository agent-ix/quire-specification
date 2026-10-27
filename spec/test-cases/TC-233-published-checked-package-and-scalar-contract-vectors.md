---
id: TC-233
title: "Published checked-package and scalar contract vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-340
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-271-AC-7
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272-AC-22
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272-AC-23
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272-AC-24
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272-AC-25
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-78
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-79
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-75
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-114
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-81
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-82
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-83
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-108
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-118
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-478-AC-3
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-341
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-342
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-464-AC-19
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-451
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-201
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-142
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-044
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-140
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-270
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-271
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272
    type: verifies
  - target: ix://agent-ix/quire-specification/interface_004
    type: verifies
---

# TC-233: Published checked-package and scalar contract vectors

## Description

Qualify the published QSpec contract artifacts themselves: the V2
checked-package schema and fixtures, the nominal node-identity preimage
schema and vectors, the operation catalog including text-to-number identities,
its clause and control operation identities, and the compound-unit value identity schema and
vectors, and the complete-value and complete-model definition locks and
package-selection vectors. This case proves producer-side schema closure,
digest recomputation, and operation-identity and diagnostic-cause closure
through QSpec's Rust qualification. The dependency-closure reader outcomes
below have implementation evidence in quire-contract-ir's
`diamond_with_different_transitive_package_ids_refuses` and
`transitive_only_dependency_reference_refuses_missing_selection` tests; the
QSpec crate itself does not contain a production reader, linker, lowerer, or
evaluator. Other reader and evaluator criteria remain with TC-217 and TC-187.

## Test Procedure

Validate each positive fixture against its published schema and apply every
authored adverse mutation. Rebuild each V2 identity preimage from the lock
selections alone, require it to equal the shipped `identity_preimage`, which
carries no `identity_projection`, add the identity projection derived from
the occurrence-free graph to form the package-id preimage, recompute the JCS
SHA-256 package id, then edit each excluded member and each named preimage member, requiring each
excluded member injected into the identity preimage to be refused. Add an
`identity_projection` member to the all-families `identity_preimage` for each
`adverse.json` `shipped_projection_cases` entry (the occurrence-free graph,
and `[]`) and require the schema to refuse it, a reader refusing
`malformed_wire` at `/identity_preimage/identity_projection` (FR-322-AC-100). Insert the
published dependency selections of `dependency-selection-vectors.json` into
the all-families package's lock and identity preimage, require the schema to
admit it and its recorded package id to recompute, require each authored
entry mutation, and each `model_entry_mutations` entry placed in
`model_selections`, to fail the schema in either member at the mutated entry, and
apply each order vector's selections to require its recorded order outcome.
Treat the members the vectors file lists in `schema_only` as schema and order vectors only, not as supplied packages. Check supply order with the `supply_vectors`: neither entry supplied refuses the first; the first supplied and the second not refuses the second; the first supplied but stale and the second not refuses `stale_dependency`/`content-mismatch` at the first, not `missing-selection` at the second (FR-322-AC-84). Check a `dependency_reference` refusal's locus: `missing-selection` at the term's `package` member, `missing-name` at its `node` member and `operator-ineligible` at the term, under the term's pointer in its node's `body`. Check a function whose parameter is of a declared record type, and one whose body only references a declared record, to decide the signature type nodes: only the first is ineligible.
Apply the published closure vectors to two direct dependencies whose own locks
select the same transitive identity at different package IDs; require
`invalid_package`/`conflicting-definition` with both dependency paths as loci.
Recompute each direct dependency's recorded `package_id` from its base
`identity_preimage` with that lock's selections substituted. Check that the
diamond reaches one identity at distinct real IDs, and that the reference's
package is in the derived closure but absent from the importer's direct entries.
With one direct dependency, name its transitive-only selection in a
`dependency_reference` and require `missing_declaration`/`missing-selection`,
even though the named package is in the derived closure. Validate a `dependency_reference` term against both schemas, and its bare-digest, other-domain, missing-`node` and extra-member mutations; place it as the callee of the `function-call` operation vector and recompute the application-node id after changing only its `package` digest, then only its `node` (FR-322-AC-36). Validate
`fixtures/positive-dependency-library.json` and recompute its `package_id`,
then read `fixtures/positive-dependency-call.json`, whose function calls the
library's `Int[0, 9] -> Boolean` function through a `dependency_reference`
callee, with the library supplied under its identity, and then apply each
FR-322-AC-37 mutation `dependency-call-vectors.json` publishes, recomputing
node keys and the `package_id` after the patch: no dependency package
supplied, a `package` absent from `dependency_selections`, a `node` with no
`declaration`, a `node` naming a declared record or a function returning one,
a function over `Set<R>` and one over a tuple holding `R` for a declared
record `R`, a function over `Reference<M::T>` for a model type `M::T`,
the term as a non-callee argument, the term as a node body root, and the target listed in
`dependencies`. Recompute
every nominal node and compound-unit vector identity and apply each authored
mutation, requiring the exact refusing check it names (`schema` or `semantic`),
including `-0` spellings and the `dimension-equivalence-` mutations (descending
ids, one node twice, one id, a stale pair, a stale `root_scale`, a zero, a
negative, a missing and an unreduced `root_scale`, an owner outside the lock;
the one-id, zero, negative and missing mutations are `malformed_wire`, the
others `invalid_semantic_graph`, FR-322-AC-139),
and require the semantic oracle itself to refuse `-0` and to refuse a repeated
pair, a derived pair of unequal normalized maps or of a ratio the base roots do
not fix, and a cycle of the `dimension-equivalence-` vectors whose ratios do not
multiply to one (FR-322-AC-118). Require every qualification-catalog entry of both definition locks to name
an authored artifact under a unique role and identity, require each
complete-value catalog role to occupy exactly one always, conditional or
exactly-one slot and each complete-model role the always slot, and evaluate
every accepted and refused
package-selection vector with an independent selection oracle. Check the
operation catalog's closure against both V2 schemas and the value lock;
recompute every operation vector and application node key; classify every
operation mutation, ordering case and declaration mutation with an
independent operation and declaration oracle; check the published fixture's
`state`/`frame` node body against an independent frame oracle, requiring the
frame's `semantic_type` to be a `model`/`object_type`, `model`/`process` or
`model`/`state_machine` node, each `modifies` relationship entry, field entry (declaring node plus field name)
and `creates`/`deletes` entry to join a declared dependency of the entry's
own required meaning, each member to be duplicate-free and sorted
ascending by entry order key (declaring node digest, then field name), and
a frame body carrying two simultaneous
defects to report the one diagnostic FR-340's fixed precedence names
regardless of which member carries which defect; splice every published
`frame_mutations` vector into the semantic graph and read it with the full
normative reader (`read_package_semantics`, not the standalone frame oracle
alone), evaluated at the reader's frame step, where identity projection,
`package_id` and source-map checks are outside what any vector's
expectation covers; requiring each vector's exact code, locus and (where
the vector states one) cause, including a field entry on an event type, a
relationship or a systems node, a relationship entry naming an object type,
a `model`/`record_value_type` or `model`/`event_type` entry misdirected into
`creates` or `deletes`, an undeclared field entry, two field entries of one
declaring node in descending name order, two vectors that each carry two
simultaneous meaning-join defects (one across member groups, one within one
member group at two digests), and the one vector whose `second_frame`
member is a complete second `state`/`frame` node inserted into the graph
verbatim alongside the mutated published frame node, requiring the
lower-node-id-digest frame's own defect to be reported over the
higher-digest frame's own defect; check that the identical defect set
sorted from two different collection orders yields the identical
diagnostic; require every published fixture's frame node to carry the four
members `modifies`, `creates`, `deletes` and `deletes_scoped` and a `scope` on
each field entry, and the schema to refuse a frame body missing any of the four
members and a field entry without a `scope` (FR-340-AC-7); rewrite the
clause-operations package with its declaring type as a `model`/`process` and
then as a `model`/`state_machine` node and read each with the full reader,
requiring the frame, the anchor, the invariant and the precondition to be
admitted with the declaring node as their context and the type of `self` a
reference over it (FR-342-AC-7, FR-342-AC-8, FR-341-AC-11), and rewrite its
declaring node, or the anchor's `context`, as a record value type, an event
type, a persistence interface or a systems interface, requiring the frame step
or the anchor step to refuse it (FR-340-AC-12, FR-342-AC-9); read each
`receiver_frame_cases` vector of `model-owner-form-vectors.json` through the
full reader over the all-families package with the frame typed by an object
type, a process or a state machine, requiring a `receiver` field entry and a
`receiver` `deletes_scoped` entry to be admitted in the object type's frame and
refused `invalid_model_binding`/`malformed-declaration` in a process's or a
state machine's frame, at the entry's declaring node and at the frame node
respectively (FR-340-AC-19, FR-340-AC-20); splice the `frame_mutations` vectors that carry two scopes of one field and a misordered `deletes_scoped` into the all-families frame, requiring `invalid_model_binding`/`conflicting-binding` at the declaring node for two scopes of one field whichever order they are listed in (outranking a `deletes_scoped` or `modifies` order defect but not another meaning-join defect), and `invalid_semantic_graph` at the frame node for a `deletes_scoped` array that is not `receiver` first and then parameters by name (FR-340-AC-16, FR-340-AC-17); read each `deletes_scoped_parameter_cases` vector with a model attached, requiring a `parameter` entry that names no reference parameter of the anchor's operation to refuse `missing_declaration`/`missing-name` at the frame node in the state step, after the frame step (FR-340-AC-17); validate the clause-operations package's `state`/`state_clause`,
`state`/`operation_anchor`, `state`/`frame` and `state`/`transition` nodes
against the schema's node-form body rules and occurrence roles, and apply
each authored body, member, argument-shape and role mutation, requiring the
schema to refuse it; read the package's invariant and precondition through
the full reader, requiring the state step to admit them, and read each
authored anchor, parameter, `self`-type, arity, body-root misplaced-clause
and anchor-before-clause mutation, requiring its exact state refusal and
locus; require a `quire.op.state.clause` application nested in another
clause's condition to fail the schema and refuse `malformed_wire` at the
nested application;
and parse the native-diagnostics rows to require each cause in
exactly its code's row and the Diagnostic schema pairing. Parse the
`stage_limit_exceeded` and `duplicate_selection` rows of both native-diagnostics
and FR-272 and require each to list exactly FR-272-AC-10's causes. Parse the FR-272 rows of `unknown_wire`, `unknown_required_feature`, `unknown_contract_version` and `unknown_required_capability` and the `Diagnostic` schema's pairing of `unsupported-wire` and `unsupported-feature`, and require FR-322-AC-78's one pairing. Parse the rows of the eight FR-322 reader codes in both and require FR-272-AC-22's causes and payload names, and FR-271 to hold one row for each (FR-271-AC-7). Parse
the `resource_exhausted` and `cancelled` rows of both and require each to list
exactly FR-272-AC-16's causes and payload keys, and the `conflicting_declaration`
rows to list exactly FR-272-AC-17's cause and payload keys. Parse the
`runtime_invariant` rows of both and require FR-272-AC-18's cause and
population-identity payload keys. Parse the `invalid_result` rows of both and
the first cells of FR-331's "Result entries" table, and require FR-272-AC-23's
causes in all three and its payload keys in both rows. Parse the `invalid_quire_context` rows of both and require FR-272-AC-24's one cause and its `view` and `form` payload keys, parse the `unimplemented_capability` rows of both and require FR-272-AC-25's three causes and its `backend`, `construct`, `construct_name`, `location` and `capability` payload keys, and require the `resource_exhausted` rows and
`value-accounting.md` to state FR-272-AC-19's `stage` rule. Parse
the rows of FR-272-AC-11's eleven codes in both documents and require each to
list exactly that criterion's causes and payload keys, and require FR-272's
canonical payload-name list to hold every one of those keys. A payload key is
any backticked name written `key`: after the cause list, canonical or not.
Require FR-270's
Revision cell to quote the catalog header's revision, and FR-271's values
table to hold exactly one row for each FR-271-AC-6 code. Require both
`invalid_source_identity` rows to name the two labels in order and to state
FR-272-AC-12's labels-before-path precedence; require the catalog's payload
value spellings to hold FR-272-AC-13's domain forms and widths, the flag order
FR-148 names and the labels in the row's order; and require FR-044,
`quire.value.accounting/v1` and the catalog to state FR-044-AC-10's
evaluation-time refusals and `sum` carve-out, with FR-145 keeping a non-member
`sum` seed or prefix a located undefined outcome; and require FR-140 and the
decimal, unit-target and integer-target membership sentences of
`quire.value.accounting/v1` to carry the same `sum` carve-out on
`decimal_out_of_domain` and the unit target refusals (FR-140-AC-5).
Validate a Diagnostic entry carrying each of `type-mismatch`,
`missing-selection`, `unsupported-selection`, `wrong-selection-role`,
`declaration-form`, `expression-form` and `not-yet-implemented` with
every code of the schema's code enum (FR-322-AC-47).
Recompute the QSL FR-092 key of every structural node, grouped or not, of
every published positive fixture, and the recursive-records
fixture's group digests, and read every published positive fixture with the
full reader (FR-322-AC-50). Apply each `derived_shape_key_mutations` entry of
`adverse.json` to its base fixture's node,
keep the node id, recompute `package_id`, and read it with the full reader
(FR-322-AC-48, FR-322-AC-49); relabel the recursive-records `Tree` group with
a label that is not its group digest, rekey each member under it, rewrite
every reference and read it (FR-322-AC-49).

The positive fixtures are rebuilt in the same change. Producing them is a
fixture-production step, not a test step: every node of a kind QSL's v2
emission writes (QSL FR-093-AC-19's classification) is produced by running
QSL's emitter, which replaces the non-emittable placeholders, among them
`positive-all-families.json`'s declared boolean `Example::Flag` and the
nodes typed at it, and `positive-operation-identities.json`'s undeclared
record node and its placeholder function `double` and model `Counter`.
Two sets of nodes are hand authored. The first is each node of a kind QSL
never writes (FR-093-AC-19), kept so that the all-families fixture carries
all 15 node tags (FR-322-AC-13); it is keyed by FR-092/FR-094's rule, the
`quire.structural-node/v1` preimage FR-322's derived-shape key stage
recomputes. The second is the all-families fixture's `Tank` hybrid model,
written by hand in the form QSL's S4 emits (FR-481); each of its `hybrid`
nodes is keyed by the same `quire.structural-node/v1` preimage, and each of
its term nodes by FR-322's preimage for its form. The pass/fail oracle for every node, emitted or hand authored,
is this case's independent key recomputation and the full reader's
acceptance, never the node's provenance. Regenerating the fixtures needs
QSL's emitter to write `owner` (QSL FR-093-AC-21, agent-ix/quire-spec-language#648),
which needs IR's `CheckedSemanticNodeV2` `owner` field (IR-646) first. The published fixtures carry the node `owner` member (FR-322
"Owner member schema"). In every positive fixture each declared structural node carries
the `SourceOwner` of its fixture's lock source (`agent-ix`/`example`, or
`a`/`u` in the recursive-records fixture), and the fixture's `package_id` is
recomputed over it. The undeclared
`relation`/`relationship` nodes of `positive-all-families.json` and
`positive-operation-identities.json` become model declaration nodes: each
carries M4's `ModelOwner` (`acme/orders`, `ix://acme/orders/relationship/Order-order-Order`), is
its own `semantic_type`, has an empty body and is keyed M4; each fixture's
lock and identity preimage select `acme/orders` under the digest of
`model-member-type-vectors.json`'s `domain_package`, and every node and
vector naming the old key (the all-families frame node, the operation
fixture's reachability node and the `frame_mutations` and
`derived_shape_key_mutations` that name them) is rekeyed or rewritten. The
wire cases of `model-member-type-vectors.json` carry the owners TC-280
lists. `adverse.json` gains `owner_mutations`, each applied to its base
fixture with the node id kept and
`package_id` recomputed, and `fixtures/` gains `positive-two-owners-a.json`
and `positive-two-owners-b.json`, whose declarations carry the qualified
names QSL emits (`["Point"]`, `["List"]`, `["three"]`), so their `Point`,
`List` and `Integer` nodes are the nodes QSL FR-093-AC-22 emits. `three` is
not in AC-22's source: it is hand authored and keyed by FR-322's
`quire.application-node/v1` rule.

Validate every node of every positive fixture against the owner presence
rule and require each owner-bearing node's `owner` to equal its preimage's. Apply each `owner_mutations` schema case (a
declared record without `owner` and with a `ModelOwner`, a model
declaration node without `owner` and with a `SourceOwner`, an `owner` on the
`Integer`, nominal enum and application nodes, a `DefinitionOwner`, a
`SourceOwner` with an extra member and a `ModelOwner` with `version`) and
read each with the full reader (FR-322-AC-51). Read the owner-change
mutation, whose new source is added to `lock.sources` and receives the
node's `declaration` occurrence regions, the unjoined
`SourceOwner` and `ModelOwner` mutations, the unjoined-owner mutation
combined with a stale key at a node of lower node-id digest, and the
source-map edit moving a declaration occurrence's region to another lock
source (FR-322-AC-51, FR-322-AC-52). Read both two-owner fixtures, compare
their node ids, group labels and `package_id`s, recompute each one's
in-group keys and group label from that package alone, compare the
application-keyed `three` node's id across both, and read package
`a` carrying package `b`'s `owner` members, with `b`'s source added to its
lock and its declaration regions moved to it (FR-322-AC-53).

`positive-all-families.json` also carries one clause function, a `function`
node with no `declaration`, owned by the `ModelOwner` of the operation member
`ix://acme/orders/Order/size` and whose body holds the one `clause` binding
`precondition` (QSL FR-094); `model-member-type-vectors.json`'s
`domain_package` projection (derived from the selected document `domain-package-acme-orders.json`,
which every model-selecting fixture selects by its JCS digest) declares
`Order-order-Order` with the ends `order` and `next` (`bidirectional`) and `Order-invoiced-Invoice`
with the one end `invoiced` (no `targetEnd.role`, `source-to-target`) and
`Order-follows-Order` (`source-to-target`, ends `follows` and `followedBy`).
`adverse.json` `relationship_end_cases` holds `relationship-end-forward-without-inverse`
(admitted), `relationship-end-inverse-against-direction` (`ill_typed`/`operator-ineligible`,
FR-152) and `relationship-end-names-missing-inverse`
(`missing_declaration`/`missing-name`), all FR-322-AC-56.
`adverse.json` `reaches_population_cases` holds `reaches-without-population-keeps-its-member`,
`reaches-population-max-3-admitted` and `reaches-population-max-0-admitted` (a
`model_population` node over `Set<Reference<Order>>` named by the `population` key of the
`Order-order-Order` `reaches` application, whose `dependencies` hold it and whose key is
recomputed), each admitted; `reaches-population-names-integer-range`,
`reaches-population-names-absent-node` and `reaches-population-over-another-model-declaration`
(`ill_typed`/`operator-ineligible`); and `reaches-population-omitted-from-dependencies`
(`invalid_semantic_graph`), all FR-322-AC-73.
`adverse.json` `unbounded_population_cases` holds `population-without-max-is-unbounded` and
`population-max-3-stays-bounded` (a `model_population` node over `Set<Reference<Order>>` with no
`max` member, and with `max` `3`, each named by the `population` key of the `Order-order-Order`
`reaches` application), admitted; and `population-max-negative-refused`,
`population-max-not-an-integer-refused` and `population-max-with-a-second-member-refused`
(`invalid_package`/`invalid-derived-shape`), all FR-322-AC-74. The bounded populations PO1 (`max` 3) and `max` 0 and the unbounded population PO0 are also
recomputed from their hand-written RFC 8785 preimages to the keys FR-322 publishes.
`adverse.json` `population_reference_cases` holds `parameter-operand-admitted`
(the published fixture's `Population<Order>[2]` binding operand),
`counters-all-instances-admitted`, `counters-lookup-admitted` and `archive-all-instances-admitted`
(a `relation`/`population` node owned by a population of the selected document, whose `counters` and `archive` entries `adverse.json` publishes as `selected_document_populations` (FCD populations keyed by `identity`), referenced as
operand 0, with its key and its application's key recomputed; the `counters` and `archive`
nodes, applications and package ids differ and `counters` is one node for both operations),
each admitted; `population-owned-by-an-object-type` and `population-owned-by-an-undeclared-node`
(`missing_declaration`/`missing-selection` at the population node);
`population-without-an-owner` (`malformed_wire`); `population-entry-without-an-identity-refused` (entries that carry a `node` and no `identity`), `population-entry-with-a-node-member-refused` (the published entries plus a `node` member) and `population-entry-lacking-only-an-identity-refused` (the published entries with one `identity` removed), each `invalid_model_binding`/`malformed-declaration` at `package_read`, at `/lock/model_selections/0`, before the owner join (FR-322-AC-82); and
`population-as-a-relationship-declaration`, `population-as-a-field-declaration`,
`population-as-an-operation-declaration`, `population-as-a-reaches-population-link` and
`population-as-a-collection-operand`
(`ill_typed`/`operator-ineligible`), all FR-322-AC-82 and FR-322-AC-83.
`adverse.json` gains `clause_owner_cases`
(`clause-precondition-operation-owner` admitted;
`clause-precondition-owned-by-object-type`,
`clause-invariant-owned-by-operation-member`, `clause-invariant-owned-by-systems-interface`, `clause-kind-postcondition`,
`clause-without-kind` and `clause-two-kind-bindings` each refused
`missing_declaration`/`missing-selection` at the clause function's node id),
read with the full reader (FR-322-AC-51, FR-322-AC-55). The `state`/`state_clause` nodes of
`positive-clause-operations.json` carry no `owner`, and the schema refuses one
that does.

Only the `correspondence`/`abstraction_relation` form is outside the
derived-shape key stage; the all-families `correspondence`/`source_locus`
node stays a structural node the stage keys. `positive-clause-operations.json`
gains one `correspondence`/`abstraction_relation` node (FR-451): no `owner`, empty `objects` and
`populations`, and one `frames` entry keyed by the fixture's
(`ConfigVersion`, `attemptUpdate`) `operation_anchor` pair (FR-451-AC-2),
its own `semantic_type`, its body targets as `dependencies` and a
`declaration` occurrence, keyed by FR-451's
`quire.abstraction-relation-node/v1` preimage. Read the package with the
full reader, and read it again with one `function` segment of its `frames` entry's
body changed, canonical order and `dependencies` kept, its node id kept and
`package_id` recomputed (FR-322-AC-54).


### Scalar equality operation vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| SE-01 | `quire.op.string.eq` over two `string` operands, result `boolean` | Identity enumerated by both schemas; application admitted | FR-322-AC-67 |
| SE-02 | `quire.op.string.ne` over two `string` operands | Identity enumerated by both schemas; application admitted | FR-322-AC-67 |
| SE-03 | `quire.op.uuid.eq` over two `uuid` operands | Identity enumerated by both schemas; application admitted | FR-322-AC-67 |
| SE-04 | `quire.op.uuid.ne` over two `uuid` operands | Identity enumerated by both schemas; application admitted | FR-322-AC-67 |
| SE-05 | `quire.op.timestamp.eq` over two `timestamp` operands | Identity enumerated by both schemas; application admitted | FR-322-AC-67 |
| SE-06 | `quire.op.timestamp.ne` over two `timestamp` operands | Identity enumerated by both schemas; application admitted | FR-322-AC-67 |
| SE-07 | `quire.op.string.eq` over a `string` and a `text` operand | `ill_typed`/`operator-ineligible` | FR-322-AC-67 |
| SE-08 | `quire.op.integer.lt` over two `uuid` operands | `ill_typed`/`operator-ineligible` | FR-322-AC-67 |
| SE-09 | literal of `value_kind` `string`, `uuid` and `timestamp`, each typed by its own family with a canonical value | Admitted by both schemas and the literal rule | FR-322-AC-67 |
| SE-10 | literal of `value_kind` `uuid` typed by a `string` type | `ill_typed`/`operator-ineligible` | FR-322-AC-67 |
| SE-11 | `quire.op.uuid.eq` over a `uuid` and a `string` operand; `quire.op.timestamp.eq` over a `timestamp` and a `uuid` operand | `ill_typed`/`operator-ineligible` | FR-322-AC-67 |
| SE-12 | `quire.op.string.eq` carrying a `text_profile` law | `invalid_package`/`operation-law-mismatch` | FR-322-AC-67 |
| SE-13 | `quire.op.uuid.eq` carrying a `text_profile` mode | `invalid_package`/`operation-mode-mismatch` | FR-322-AC-67 |
| SE-14 | `quire.op.integer.add` over two `uuid` operands; `quire.op.integer.lt` over two `timestamp` operands | `ill_typed`/`operator-ineligible` | FR-322-AC-67 |
| SE-15 | literal of `value_kind` `string` typed by `uuid` or `text`; literal of `value_kind` `timestamp` typed by `integer` | `ill_typed`/`operator-ineligible` | FR-322-AC-67 |
| SE-16 | literal of `value_kind` `uuid` with an uppercase value, no hyphens or 35 characters | `invalid_package`/`invalid-value` | FR-322-AC-67 |
| SE-17 | literal of `value_kind` `timestamp` with value `-0`, `+1`, `01`, empty text or `170141183460469231731687303715884105728` | `invalid_package`/`invalid-value` | FR-322-AC-67 |
| SE-18 | `quire.op.text.eq` over two `string` operands with a `text_profile` law and mode | `invalid_package`/`operation-mode-type-mismatch` | FR-322-AC-67 |
| SE-19 | literal of `value_kind` `string` whose `value` is a number, `null` or an array | `invalid_package`/`invalid-value` | FR-322-AC-67 |
| QL-01 | literal of `value_kind` `quantity` typed by a unit node, values `3`, `0`, `-5`, `1/1000` and `-7/2` | Admitted with family `quantity`; both schemas enumerate `quantity` | FR-322-AC-79 |
| QL-02 | literal of `value_kind` `quantity` typed by `integer`, `string` or `boolean` | `ill_typed`/`operator-ineligible` | FR-322-AC-79 |
| QL-03 | literal of `value_kind` `integer` typed by a unit node | `ill_typed`/`operator-ineligible` | FR-322-AC-79 |
| QL-04 | literal of `value_kind` `quantity` with value `03`, `+3`, `2/4`, `3/1`, `0/5`, `1/0`, `2.5` or empty text | `invalid_package`/`invalid-value` | FR-322-AC-79 |
| RL-01 | literal of `value_kind` `rational` typed by a `scalar_type`/`rational` node, values `0`, `1`, `-3`, `1/2` and `-7/2` | Admitted with family `rational` | FR-322-AC-114 |
| RL-02 | literal of `value_kind` `rational` with value `2/2`, `1/1`, `02`, `+1`, `2/4`, `1/0`, `0/5`, `1/-2`, `2.5` or empty text | `invalid_package`/`invalid-value` | FR-322-AC-114 |
| NS-01 | nodes of one `declaration` name `ms`: a `scalar_type`/`unit`, a `composite_type`/`record` and a `scalar_type`/`dimension` | Admitted | FR-322-AC-81 |
| NS-02 | two `scalar_type`/`unit` nodes of one name | `ambiguous_declaration`/`ambiguous-name` at the later node | FR-322-AC-81 |
| NS-03 | a `composite_type`/`record` and a `scalar_type`/`enum` of one name | `ambiguous_declaration`/`ambiguous-name` at the later node | FR-322-AC-81 |
| SE-20 | literal of `value_kind` `uuid` typed by a `string` type with an uppercase value; literal of `value_kind` `timestamp` typed by an `integer` type with value `01` | `ill_typed`/`operator-ineligible`, the family refusal first | FR-322-AC-67 |
| FB-01 | literal of `value_kind` `float32_bits` typed by a `float32` type, values `3f000000`, `00000000`, `80000000` and `7fc00000`; literal of `value_kind` `float64_bits` typed by a `float64` type, values `3fe0000000000000`, `0000000000000000`, `8000000000000000` and `7ff8000000000001` | Admitted with families `float32` and `float64` | FR-322-AC-108 |
| FB-02 | literal of `value_kind` `float32_bits` with value `3F000000`, `0x3f000000`, `3f00000`, `3f0000000`, `0.5` or the JSON number `1056964608` | `invalid_package`/`invalid-value` | FR-322-AC-108 |
| FB-03 | literal of `value_kind` `float64_bits` with value `3FE0000000000000`, `0x3fe0000000000000`, `3fe000000000000`, `3fe00000000000000`, `0.5` or the JSON number `4602678819172646912` | `invalid_package`/`invalid-value` | FR-322-AC-108 |
| FB-04 | literal of `value_kind` `float32_bits` typed by a `float64` type with value `3F000000`; literal of `value_kind` `float64_bits` typed by a `string` type with value `0.5` | `ill_typed`/`operator-ineligible`, the family refusal first | FR-322-AC-108 |

### Text-to-number operation identity vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| TN-01 | `quire.op.text.to_integer` over a `text` operand, `integer` result and `type_argument` member naming that result | Admitted by both V2 schemas and the operation catalog with `text_profile`, no mode and `member_family` `integer` | FR-322-AC-17; FR-478-AC-3 |
| TN-02 | `quire.op.text.to_decimal` over a `text` operand and a `decimal` result | Admitted with `text_profile`, `rounding` pinned by the decimal result, and `member_family` `decimal` | FR-322-AC-17; FR-478-AC-3 |
| TN-03 | `quire.op.text.to_rational` over a `text` operand and a `rational` result | Admitted with `text_profile`, no mode and `member_family` `rational` | FR-322-AC-17; FR-478-AC-3 |
| TN-04 | `quire.op.text.to_float32` over a `text` operand and a `float32` result | Admitted with `text_profile` and `ieee_profile`, `rounding` pinned by the result, and `member_family` `float32` | FR-322-AC-17; FR-478-AC-3 |
| TN-05 | `quire.op.text.to_float64` over a `text` operand and a `float64` result | Admitted with `text_profile` and `ieee_profile`, `rounding` pinned by the result, and `member_family` `float64` | FR-322-AC-17; FR-478-AC-3 |
| TN-06 | `quire.op.text.to_quantity` over a `text` operand and a `quantity` result | Admitted with `text_profile`, `rounding` pinned by the quantity result, and `member_family` `quantity` | FR-322-AC-17; FR-478-AC-3 |
| TN-07 | `quire.op.string.to_integer` over a `string` operand, `integer` result and `type_argument` member naming that result | Admitted by both V2 schemas and the operation catalog with no law, no mode and `member_family` `integer` | FR-322-AC-17; FR-478-AC-3 |
| TN-08 | `quire.op.string.to_decimal` over a `string` operand and a `decimal` result | Admitted with no law, `rounding` pinned by the decimal result, and `member_family` `decimal` | FR-322-AC-17; FR-478-AC-3 |
| TN-09 | `quire.op.string.to_rational` over a `string` operand and a `rational` result | Admitted with no law, no mode and `member_family` `rational` | FR-322-AC-17; FR-478-AC-3 |
| TN-10 | `quire.op.string.to_float32` over a `string` operand and a `float32` result | Admitted with `ieee_profile`, `rounding` pinned by the result, and `member_family` `float32` | FR-322-AC-17; FR-478-AC-3 |
| TN-11 | `quire.op.string.to_float64` over a `string` operand and a `float64` result | Admitted with `ieee_profile`, `rounding` pinned by the result, and `member_family` `float64` | FR-322-AC-17; FR-478-AC-3 |
| TN-12 | `quire.op.string.to_quantity` over a `string` operand and a `quantity` result | Admitted with no law, `rounding` pinned by the quantity result, and `member_family` `quantity` | FR-322-AC-17; FR-478-AC-3 |
| TN-13 | TN-01 with its result and `type_argument` changed to `Boolean` | `ill_typed`/`operator-ineligible` because the catalogued `member_family` is `integer` | FR-322-AC-17; FR-478-AC-3 |
| TN-14 | Each of TN-07 through TN-12 with an added `text_profile` law | `invalid_package`/`operation-law-mismatch` | FR-322-AC-17; FR-478-AC-3 |

### IEEE negate and float leaf vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| NF-01 | `quire.op.ieee.float64.negate` over one `Float64[nearest-even]` operand, `result_type` that operand's type node, the `ieee_profile` law, mode `null`; the same with `quire.op.ieee.float32.negate` over `Float32[toward-zero]` | Identities enumerated by both schemas; application admitted | FR-322-AC-111 |
| NF-02 | `quire.op.ieee.float64.negate` over a `float32` operand; `quire.op.ieee.float32.negate` over a `float64` operand; over an `integer` operand; over two operands | `ill_typed`/`operator-ineligible` | FR-322-AC-111 |
| NF-03 | NF-01's `float64` application with `result_type` `Float64[toward-zero]` | `ill_typed`/`operator-ineligible` | FR-322-AC-111 |
| NF-04 | NF-01's application with a `rounding` mode, with no law, and in a lock without the `ieee_profile` selection | `invalid_package`/`operation-mode-mismatch`; `operation-law-missing`; `operation-law-unselected` | FR-322-AC-111 |
| NF-05 | `quire.op.ieee.float64.eq`, `quire.op.ieee.float64.ne` and `quire.op.ieee.float64.lt` | `invalid_package`/`unknown-operation` | FR-322-AC-111 |
| NF-06 | `quire.op.structural.eq` over `record R { x: Float64[nearest-even]; t: Text[0,8;nfc]; }` | Admitted with the leaves `["field:x"]` (`ieee_profile`, mode `null`) then `["field:t"]` (the `nfc` law and mode) | FR-322-AC-112 |
| NF-07 | `quire.op.structural.ne` over `Option<Float64[nearest-even]>`, `union U { V(Float32[exact]), N }` and `Sequence<Float64[nearest-even]>` | Admitted with the leaves `["inner"]`, `["member:V", "position:0"]` and `["inner"]` | FR-322-AC-112 |
| NF-08 | NF-06's application with the `field:x` leaf omitted, with a `text_profile` law on it, and with a mode on it | The first failing leaf check refuses as for a text leaf | FR-322-AC-112 |

### Decimal literal vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| DL-01 | literal of `value_kind` `decimal` typed by `Decimal[0, 1000; 0, 2; nearest-even]` with values `1.25`, `0.05`, `10`, `0.00`, `1000`, `0.5`, `1000.00` and `1.250` | Admitted by both schemas and the literal rule: each is a member by value (FR-140), whatever its written coefficient and scale | FR-322-AC-113 |
| DL-02 | the same type with values `.5`, `1.`, `+1.5`, `01.5`, `-0.0`, `1e2`, `1/4` and empty text | `invalid_package`/`invalid-value` | FR-322-AC-113 |
| DL-03 | the same type with values `1.255` (membership scale 3), `1001`, `1000.01` and `-1` | `invalid_package`/`invalid-value` | FR-322-AC-113 |
| DL-04 | values `1.0` and `1.00` typed by one decimal type | Two literals with two node keys | FR-322-AC-113 |
| DL-05 | `decimal` literal typed by an `integer` or `string` node with value `.5` | `ill_typed`/`operator-ineligible`, the family refusal first | FR-322-AC-113 |
| DL-06 | literals typed by `Decimal[-1000, 1000; 1, 2; nearest-even]` with values `-0.5`, `5` and `100`, then `1001` | The first three admitted (`5` has membership scale 1 and coefficient 50); `1001` is `invalid_package`/`invalid-value` | FR-322-AC-113 |

### One-occurrence collection vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| OO-01 | `quire.op.collection.sequence.drop_first` over a `Sequence<Integer>[0,9]` operand and `quire.op.collection.ordered_set.drop_first` over an `OrderedSet<Integer>[0,9]` operand, each with `result_type` of the operand's family and bound `[0,8]` | Identities enumerated by both schemas; applications admitted with no law, mode, member or leaf | FR-322-AC-117 |
| OO-02 | `quire.op.collection.sequence.drop_first` over a `Set<Integer>[0,9]` operand and over a `Bag<Integer>[0,9]` operand; `quire.op.collection.ordered_set.drop_first` over a `Sequence<Integer>[0,9]` operand; `quire.op.collection.sequence.drop_first` over two operands | `ill_typed`/`operator-ineligible` | FR-322-AC-117 |
| OO-03 | `quire.op.collection.remove_one` over `Bag<Text[0,8;nfc]>[0,9]` and a `Text[0,8;nfc]` value; over `Set<Float64[nearest-even]>[0,9]` and a `Float64[nearest-even]` value; over `Sequence<Integer>[0,9]` and an `Integer` value | Admitted with the empty-path `nfc` text leaf, the empty-path `ieee_profile` float leaf with mode `null`, and no leaf | FR-322-AC-117 |
| OO-04 | OO-03's `Sequence<Integer>[0,9]` application with a `Text[0,8;nfc]` value, with one operand and with `result_type` `Set<Integer>[0,9]`; OO-01's applications with a `text_profile` law, a `rounding` mode or a `field` member | `ill_typed`/`operator-ineligible`; `invalid_package`/`operation-law-mismatch`, `operation-mode-mismatch` and `operation-member-mismatch` | FR-322-AC-117 |

### Structural node preimage vectors

Build each preimage exactly as FR-322 "Structural node preimage" spells it and hash its RFC 8785 bytes.

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| SP-01 | P4: `value`/`parameter`, name `x`, level `0`, `semantic_type` the `Int[0, 9]` node, `declaration` and `recursion` `null`, no `owner` | Key `ebe64b4c3bc2cc5f3460a43480f8979f5df24c5e7665d9e89890f43517dd8eda` | FR-322-AC-97 |
| SP-02 | P4 with level `1`; P4 with name `y` | Each key differs from SP-01's and from the other's | FR-322-AC-97 |
| SP-03 | U2: `scalar_type`/`compound_unit` over `U_metre` exponent `1` and `U_second` exponent `-1`, self-typed | Key `bcf431d15781d66fcdd2514e251004373eb64a3ef21e37f7b2535f2a871821e7` | FR-322-AC-97 |
| SP-04 | A `value`/`parameter` node whose body binds `level` before `name`; one with a third binding | Each fails the V2 schema; `malformed_wire` | FR-322-AC-97 |
| SP-05 | Every `value`/`parameter` node of every published positive fixture | Its `node_id` is the key of its preimage; the full reader admits each fixture | FR-322-AC-97 |

### Event type vectors

The owner join, projection and equality behaviour of an event type (FR-322-AC-68) and record conformance (FR-322-AC-69) are decided by TC-281's `EV-` and `RCF-` vectors.

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| ET-01 | A `model`/`event_type` node with a `ModelOwner`, no `declaration`, its own id as `semantic_type` and an empty aggregate body, in a package | Admitted by the V2 schema; its `ModelDeclarationNode` preimage validates against the node-identity preimage schema | FR-322-AC-68 |
| ET-02 | ET-01 with no `owner` | Fails the V2 schema; `malformed_wire` | FR-322-AC-68 |
| ET-03 | ET-01 with a `SourceOwner` and no `declaration` | Fails the V2 schema; `malformed_wire` | FR-322-AC-68 |
| ET-04 | ET-01 with a `declaration` and a `ModelOwner` | Fails the V2 schema; `malformed_wire` | FR-322-AC-68 |

### Node dependency vectors

Rebuild each node's `dependencies` by FR-322's `dependencies` rules 1 to 4 (the body artifact's own list for a form that states one) and compare it with the list as written. Each mutation recomputes `package_id` before reading.

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| DP-01 | Every node of every published positive fixture | The written list equals the rebuilt list; each fixture's `package_id` recomputes | FR-322-AC-96 |
| DP-02 | `positive-all-families.json`'s `expression`/`reference` node `7916…` and `correspondence`/`source_locus` node `eda3…` | Each lists `8a2a…`, its body target | FR-322-AC-96 |
| DP-03 | `positive-all-families.json`'s `bounded_domain`/`integer_range` node `652c…`; `positive-operation-identities.json`'s `bounded_domain`/`model_population` node `1ffe…` | `652c…` lists `07f6…`, its `semantic_type`; `1ffe…` lists `0ec8…` alone | FR-322-AC-96 |
| DP-04 | DP-02's `expression`/`reference` node with `dependencies: []` | `invalid_semantic_graph` at the node's `/dependencies` | FR-322-AC-96 |
| DP-05 | DP-03's `integer_range` node with `dependencies: []` | `invalid_semantic_graph` at the node's `/dependencies` | FR-322-AC-96 |
| DP-06 | DP-03's `model_population` node with the Integer node `07f6…` added | `invalid_semantic_graph` at the node's `/dependencies` | FR-322-AC-96 |
| DP-07 | a `value`/`parameter` node of `positive-all-families.json` listing its `semantic_type` | `invalid_semantic_graph` at the node's `/dependencies` | FR-322-AC-96 |

Add the lock's `refinement_internals` member to the all-families package: one
object for `R` with two spellings in canonical order, and the package with the
member absent; then the mutations `[]`, `null`, an object with `"internal": []`,
an object with a third key, objects out of byte order, spellings out of byte
order, a repeated `refinement` and a repeated spelling.

Check a compiler-derived node: a refinement owner with step rows `R0` and `R1` from which one node per row is derived; require the node derived from `R1` to have an occurrence of role `generated`, ordinal 1 and the region of `R1`'s source row, and the same after swapping the source order of the rows (FR-322-AC-87).

Scan every published producer-output package (every published positive package except the reader-admission fixture `positive-integer-literal-references.json`) for the applications of `quire.op.integer.add`, `sub`, `mul`, `negate`, `eq` and `ne`: require each integer literal operand typed at the builtin Integer node to be an inline `literal` term and no such operand to reference a `value`/`literal` node, and require the `three` application of the two-owner packages and the `integer.mul` and `integer.add` applications of the operation package to hold their operands inline, and the `7` and `-2` literal nodes to be present (FR-322-AC-92). For each operator the rule lists in FR-322 "Integer literal operands", require an `inline_literal_cases` entry whose preimage is closed under the application-node schema, whose JCS SHA-256 is recorded and differs from `reference_form_sha256`, and no entry for an operator the rule omits. Read `positive-integer-literal-references.json`, the reference form of the `three` application, through the reader: it is admitted, and its `three` node identity and `package_id` differ from the inline form's (FR-322-AC-93).

Check the refinement step occurrences of `refinement-step-occurrence-vectors.json`: `RS-01` (admitted, with an entry of another node ignored) and `RS-02` (rows written in the order `R1`, `R0` against the `steps` order `R0`, `R1`) are admitted with ordinal 0 at the `declaration` region and ordinal `i + 1` at the region of the row at `steps` position `i` (FR-322-AC-88, FR-322-AC-89). Each case carries an independent `refinement_step_rows` entry for each canonical step position, including the ineligible cases. `RS-03` (ordinal 3), `RS-04` (a repeated key), `RS-05` (ordinal 1 with the other row's region), `RS-06` (an `expression` occurrence at ordinal 1) and `RS-07` (a `declaration` occurrence at ordinal 1) are each refused `invalid_package`/`invalid-value` at the offending `source_map` entry, and `RS-08` (ordinal 2 omitted), `RS-09` (the `declaration` occurrence omitted) and `RS-10` and `RS-11` (`generated` ordinals 0 and 1 omitted) at `/source_map` (FR-322-AC-90); `RS-12` (a history field, `declaration` alone) and `RS-16` (the `Counter` rows `value` and `total` written in the other order) are admitted, and `RS-13` (an `any` row), `RS-14` (a hidden field: the abstract `Counter` has fields `total` and `value` and the `objects` entry has a row for `value` alone), `RS-15` (a history field with a `generated` entry) and `RS-17` (two `objects` entries of which only the second has a hidden field) are each refused at their first `generated` entry (FR-322-AC-91). `RS-18` omits the sidecar and `RS-21` omits one row, both refusing at `/refinement_step_rows`; `RS-19` names a non-owner, `RS-20` repeats an owner/ordinal pair, `RS-23` reverses canonical order and `RS-24` names an out-of-range ordinal, each refusing at the offending sidecar entry. `RS-22` changes only the first sidecar region and refuses at generated ordinal 1's `source_map` entry while retaining the same node and package identities (FR-322-AC-128/129).

### Refinement class catalog wire vectors

Validate both closed V2 schemas against a refinement body carrying
`class_catalog: {concrete: [...], abstract: [...]}`. Each declared entry has
exactly `class`, `node` and ordered `binder_types`; a concrete key is a local
`NodeRef`, an abstract key is a local `NodeRef` or selected dependency
`PackageNodeKey`. A memory entry has exactly `class` and `memory_model`, with
no node or binder types. Use the producer-emitted `CC-01` catalog of TC-310
as the positive protocol case and `RW-01` without a catalog as the positive
model-only case (FR-322-AC-130).

| Vector | Schema or identity change | Expected result |
| --- | --- | --- |
| `CCW-01` | Positive protocol case; reorder source step rows but keep the canonical body, and update both `refinement_step_rows` regions and the matching generated owner `source_map` occurrence regions | Both schemas accept the body and preimage; each generated occurrence equals its sidecar row region, while the refinement node id and `package_id` stay equal. |
| `CCW-02` | Remove a declared class entry's `node` or `binder_types`, add any extra member, give a concrete entry an external key, or give an abstract entry an ill-shaped key | Both schemas refuse the malformed member. |
| `CCW-03` | Add `node` or `binder_types` to a memory entry, omit `drain` from `memory drain`, add `drain` to `memory flush`, use a memory class spelling other than `memory drain` or `memory flush`, or use a second word after the kind | Both schemas refuse the malformed member. |
| `CCW-04` | Change one catalog class, node, binder type, source-default memory model or drain member; rebuild exact dependencies and recompute `package_id`, but retain the refinement node id | The edited projection matches the new `package_id`; the closed preimage digest changes and the dedicated refinement identity check refuses `invalid_package`/`stale-node-key` before class joins. |
| `CCW-05` | Rekey the changed refinement and update references and package identity, but leave a local catalog node or binder type out of exact `dependencies` | Graph shape refuses `invalid_semantic_graph` at `/dependencies` before the refinement body step. |
| `CCW-06` | Remove a protocol clause's `owner_model`, put an external key there, or remove its local owner node from exact dependencies | Both schemas refuse the absent edge or external-key shape; graph admission refuses a missing owner dependency. |
| `CCW-07` | Retarget `owner_model` to another selected model and recompute the clause node id and package id | Both schemas admit the closed local edge; the clause key changes and the reader returns the new owner identity unchanged. |
| `CCW-08` | Memory entries `memory drain` for `ra`, `sc` and `tso`, each with `drain: store-buffer-flush`, and `memory flush` for `tso`, in that order | Both schemas admit each entry (a schema states shape, not order); the reader admits the canonical order and refuses a reordered or repeated (`class`, `memory_model`) pair as `invalid_semantic_graph` at the entry in step 2 (FR-322-AC-133). |
| `CCW-09` | A declared class entry `{class: "activate P", node, binder_types}` on the concrete (`NodeRef`) and abstract (`RefinementKey`) side | Both schemas admit it as an ordinary declared class (FR-322-AC-133). |

### Graph order vectors

Graph order is the producer's order of `semantic_graph.nodes`; the reader does not sort it, and it enters `package_id` through the identity projection (FR-322 `semantic_graph`). Each vector except GO-04 recomputes `package_id` over the identity projection derived in the new graph order before reading.

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| GO-01 | Each published positive fixture with no `recursion_group`, as published | Admitted by the full reader; every such fixture except `positive-nominal-identities.json` lists its nodes out of ascending node-id digest order | FR-322-AC-95 |
| GO-02 | GO-01's fixtures with `semantic_graph.nodes` reversed | Admitted; `package_id` differs from the published one | FR-322-AC-95 |
| GO-03 | GO-01's fixtures with `semantic_graph.nodes` sorted ascending by node-id digest | Admitted; `package_id` differs from the published one exactly when the sort changes the node sequence | FR-322-AC-95 |
| GO-04 | `positive-all-families.json` with `semantic_graph.nodes` reversed and the recorded `package_id` kept | The projection derived in the new order differs from the published order's and the recorded `package_id` does not recompute | FR-322-AC-95 |

### Body-step precedence vector

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| BS-01 | Three otherwise schema-valid package variants with valid state-graph, temporal, abstraction, refinement, terminal and step-relation graphs: (a) timed-model defect plus protocol defect; (b) (a) with the timed-model defect repaired; (c) a hybrid defect added to (a) | (a) reports the timed-model step's refusal before protocol; (b) reports the protocol step's refusal; (c) reports the hybrid step's refusal before timed-model. This pins `hybrid < timed-model < protocol` and prevents either neighboring step from running first. | FR-322-AC-121 |
| BS-02 | One otherwise schema-valid package with valid state-graph and temporal graphs and three independent body defects: an abstraction object's duplicate field binding (FR-451-AC-7), a refinement `objects[0].concrete_type` naming the existing Boolean scalar node (TC-310 `RW-07`), and a terminal `when` condition naming an integer-typed expression (FR-476-AC-4). Rebuild changed node keys, exact dependencies, identity projection and `package_id`; keep the refinement source sidecar and occurrence map valid. Read (a) with all three defects, (b) after repairing only the abstraction defect, and (c) after also repairing only the refinement defect. | (a) reports `invalid_model_binding`/`conflicting-binding` from the abstraction step; (b) reports `invalid_model_binding`/`malformed-declaration` from the refinement step; (c) reports `ill_typed`/`non-boolean-root` from the terminal step. This pins `abstraction < refinement < terminal` without an earlier graph, source, identity or state refusal. | FR-322-AC-121 |

### State constraint clause vectors

Each vector edits the published clause-operations package and rekeys the edited node before reading, so the read reaches the state step.

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| CN-01 | The invariant node's kind member `clause` rewritten to `constraint` | Valid against both V2 schemas; admitted by the state step; its node id differs from the invariant's | FR-341-AC-13 |
| CN-02 | CN-01's constraint with its anchor argument referencing the operation anchor | `invalid_model_binding`/`malformed-declaration` at the anchor's target | FR-341-AC-13 |
| CN-03 | CN-01's constraint with a second parameter reference in its aggregate | `ill_typed`/`operator-ineligible` at the clause node | FR-341-AC-13 |

### Execution point identity vectors

Each vector in `node-identity-vectors.json`'s `operation_vectors` applies the
execution point to the published `positive-clause-operations.json` state-clause
body, then recomputes the `quire.application-node/v1` JCS SHA-256. Both V2
schemas admit each preimage. On complete package reads, exercise valid
initialization/handler invariants and pre/post operation clauses, rekey the
clause and corresponding anchor, and verify that each accepted point round
trips and changes both applicable identities. Mutate a variant, name, or
clause/anchor agreement and require the first FR-341/FR-342 refusal.

| Vector | Execution point | Expected SHA-256 | Criteria |
| --- | --- | --- | --- |
| `execution-point-initialization` | `initialization{name: "boot"}` | `0f573793a678636dc4803eba16ff75a3432a44a2d39670b908e3aa9f607cd6f9` | FR-322-AC-131; FR-341-AC-14 |
| `execution-point-handler` | `handler{name: "validate"}` | `c9e9e7bd0021df4fef1aa0a52c10172c288bab4ccf05099e2215b0c082554497` | FR-322-AC-131; FR-341-AC-14 |
| `execution-point-pre` | `pre{operation: "attemptUpdate"}` | `011830fb0053fc357fb07d7f4118b397743918ef9e93bc0cc5c0465066828b60` | FR-322-AC-131; FR-341-AC-14 |
| `execution-point-post` | `post{operation: "attemptUpdate"}` | `065de29f4474005bc23d5cb5561f77ca997ef26d60657d64f5a95a0809461d45` | FR-322-AC-131; FR-341-AC-14 |

### State transition vectors

Vector ids are `XT-nn` (TC-374's `ST-nn` and TC-369's `TS-nn` are other vectors).
`fixtures/positive-state-transition.json` is the published Boolean model-field
transition: `Switch.set(flag: Boolean)` over the required Boolean field `on` of
`acme/orders`, with the precondition `flag || on` and the postcondition `on`,
anchored at one no-point `operation_anchor`. `Switch` (fields `id`, `on`,
`level`; operation `set`) is published in `domain-package-acme-orders.json`
and its projection in `model-member-type-vectors.json`. Its nodes and
`package_id` are recomputed with the README's "Regenerating protocol fixture
keys" procedure. Each schema vector edits that package and requires the closed
V2 schema to refuse it; each reader vector rekeys the edit and requires the
first FR-341 state-step refusal.

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| XT-01 | The unedited package | Valid against `schema.json`; every application preimage valid under the preimage schema; every node key and the `package_id` recompute; every application, the subject projection included, admitted by the operation step | FR-341-AC-17 |
| XT-02 | The transition's `anchor` and `subject` bindings swapped | Fails the schema | FR-341-AC-15 |
| XT-03 | The `post` binding removed; a sixth binding added | Fails the schema | FR-341-AC-15 |
| XT-04 | `pre` holding the precondition twice; `subject` an empty aggregate | Fails the schema | FR-341-AC-15 |
| XT-05 | Member operator `sequence`; law role `temporal_profile`; a `rounding` mode; occurrence role `declaration` | Fails the schema | FR-341-AC-15 |
| XT-06 | Member operator `sequence` on the transition | `invalid_package`/`operation-member-mismatch` at the operation step | FR-341-AC-15; FR-464-AC-19 |
| XT-07 | A `quire.op.protocol.control` application naming operator `transition` | `invalid_package`/`operation-member-mismatch` | FR-464-AC-19 |
| XT-08 | The anchor's `operation` rewritten, rekeyed | The anchor, both clauses and the transition take new ids; the `package_id` changes | FR-341-AC-17 |
| XT-09 | A source-map region moved | Every node id and the `package_id` unchanged | FR-341-AC-17 |
| XT-10 | `anchor` naming the `Switch` model node | `invalid_model_binding`/`malformed-declaration` at the anchor's target | FR-341-AC-16 |
| XT-11 | `subject` naming the `self` parameter; a subject projecting a field the frame does not modify | `invalid_model_binding`/`malformed-declaration` at the subject's target | FR-341-AC-16 |
| XT-12 | `subject` a projection of the Integer field `level` (typed Integer); a projection of the optional Boolean field `Order.note` (typed `Option<Boolean>`) | `ill_typed`/`operator-ineligible` at the transition node; the same projection declaring `Boolean` refuses `ill_typed`/`operator-ineligible` at the projection (operation step) | FR-341-AC-16 |
| XT-13 | `inputs` naming `self`; `inputs` missing the Boolean parameter `flag` | `invalid_model_binding`/`malformed-declaration` at the input; `ill_typed`/`operator-ineligible` at the transition node | FR-341-AC-16 |
| XT-14 | `pre` naming the postcondition; `post` naming the precondition; `post` anchored at another operation | `invalid_model_binding`/`malformed-declaration` at the clause's target | FR-341-AC-16 |
| XT-15 | A second transition over the same `anchor` and `subject` | `ambiguous_declaration`/`ambiguous-name` at the later node in ascending node-id digest order | FR-341-AC-16 |
| XT-16 | The transition application nested as an argument of the `or` expression; the transition node's body replaced by the `or` application | Fails the schema | FR-341-AC-18 |
| XT-17 | The transition application as the body root of the `or` expression node | `ill_typed`/`operator-ineligible` at that node, before any anchor check | FR-341-AC-18; FR-341-AC-16 |
| XT-18 | The lock without the `protocol_profile` selection | `invalid_package`/`operation-law-unselected` | FR-341-AC-15 |

XT-10 through XT-15 and XT-17 are reader outcomes the QSpec crate does not
implement; the quire-contract-ir reader owns their execution evidence.

## Expected Results

Each event type vector gives exactly its recorded result.

Every positive artifact validates and recomputes its recorded identity. Excluded
edits leave the package id unchanged and named edits change it. Every mutation
is refused by exactly its named check. Each dependency-selection entry mutation fails the
schema at the mutated entry; its `outcome` is the reader refusal a strict
reader reports for it. Each dependency-selection order vector is admitted by
the schema and admitted or refused by the order rule with exactly its
recorded outcome and loci. The new closure vectors record the diamond's two
dependency paths and its `invalid_package`/`conflicting-definition` refusal,
and the transitive-only reference's `missing_declaration`/`missing-selection`
refusal. The quire-contract-ir reader implements both outcomes: its diamond
test refuses conflicting transitive selections with both importer-rooted
selection paths, and its transitive-reference test refuses a transitive-only
package as `missing_declaration`/`missing-selection`. A `dependency_reference`
term is admitted by both schemas and each of its four mutations fails both; as a call's callee it keys a valid application-node preimage, and changing its `package` or its `node` each gives a new, distinct id. The dependency library is admitted with its recorded `package_id`,
and the dependency call package that selects it by that id is admitted with it
supplied. Each published mutation passes the V2 schema and refuses
`missing_import`/`missing-selection` at the selection entry,
`missing_declaration`/`missing-selection` at the term's `package`,
`missing_declaration`/`missing-name` at its `node`,
`ill_typed`/`operator-ineligible` (seven times) at the term, and
`invalid_semantic_graph` at the node's `dependencies`, each exactly as its
vector records. Every catalog entry names an authored
artifact, and every selection vector is admitted or refused with exactly
its recorded selection code. The published fixture's frame node is admitted
with no defect; every frame-entry meaning-join and canonical-order mutation
is refused with exactly its named cause, and a frame body carrying two
simultaneous defects reports the same one diagnostic regardless of which
member carries which defect. Every published `frame_mutations` vector
refuses through the full normative reader with exactly its recorded code,
locus and cause, including every wrong-meaning frame entry, both
two-simultaneous-meaning-join-defect precedence vectors, and the
two-frame-node vector, whose lower-node-id-digest frame's own defect is
reported ahead of the higher-digest frame's own defect; a process or state
machine as the declaring type is admitted with its frame, anchor and clauses,
and a frame of either refuses each receiver-scoped member; the identical
defect set sorted from two different collection orders yields the identical
diagnostic; and the `stage_limit_exceeded` and `duplicate_selection` rows of
native-diagnostics and FR-272 each list exactly FR-272-AC-10's causes, and
the `resource_exhausted` and `cancelled` rows of both each list exactly
FR-272-AC-16's causes and payload keys; the `conflicting_declaration` rows each list
exactly FR-272-AC-17's cause and payload keys; the `runtime_invariant` rows each
list FR-272-AC-18's cause and keys and the `stage` rule matches FR-272-AC-19; the rows of FR-272-AC-11's eleven codes each list exactly that criterion's
causes and payload keys, every one of them a canonical FR-272 payload name;
FR-270's Revision cell quotes the header's revision; FR-271 has one row
for each FR-271-AC-6 code; both `invalid_source_identity` rows state the
labels-before-path precedence; the payload value spellings match FR-272-AC-13;
and FR-044, FR-140, value-accounting, the catalog and FR-145 give a `sum` seed
or prefix one outcome in every family. The Diagnostic schema admits
`type-mismatch` only with `ill_typed`, `missing-selection` only with
`missing_import` and `missing_declaration`, and `unsupported-selection` and
`wrong-selection-role` only with `unknown_language`, `unknown_edition` and
`unknown_profile`, and `declaration-form`, `expression-form` and
`not-yet-implemented` only with `unsupported_construct`.
Every structural node, grouped or not, of every published positive fixture
carries the key QSL's FR-092 key function mints, the recursive-records
group labels are their group digests, and the full reader admits every
published positive fixture. Each structural-node mutation, including the
declared record `Example::Point`, ungrouped or in-group, refuses `invalid_package`/`stale-node-key` at the tampered node,
and the relabelled `Tree` group refuses `invalid_package`/`stale-node-key` at
its member with the lowest node-id digest.
Every owner-bearing node of every positive fixture carries its preimage's
`owner`; no other node carries one. Each
owner schema mutation fails the schema and refuses `malformed_wire`. The
owner-change mutation refuses `invalid_package`/`stale-node-key` at the
changed node; each unjoined owner refuses
`missing_declaration`/`missing-selection` at its node, also when a node of
lower digest is stale; the source-map edit, which changes no node id and
not the `package_id`, refuses `invalid_package`/`invalid-value` at the
moved occurrence's `source_map` entry. Both two-owner fixtures are admitted;
their `Point` ids, `List` group labels and `List` group member ids differ,
their `Integer` id and their application-keyed `three` id are each equal and their `package_id`s differ, each package's
in-group keys recompute from it alone, and package `a` with package `b`'s
owners refuses `invalid_package`/`stale-node-key`. The clause-operations
package with its `correspondence`/`abstraction_relation` node passes the
derived-shape key stage untouched and is admitted, and the body-edited node
refuses `invalid_semantic_graph` at FR-451's abstraction step 1, not
`invalid_package`/`stale-node-key`.

The canonical `refinement_internals` member validates against the schema and is
in no identity preimage; its absence validates too. The `[]`, `null`,
empty-`internal` and third-key mutations fail the schema at that value; the
two out-of-order mutations refuse `invalid_package`/`invalid-value` at the later
entry's pointer and the two repeats refuse `invalid_package`/`conflicting-definition`
with both entries as loci (FR-322-AC-77).
