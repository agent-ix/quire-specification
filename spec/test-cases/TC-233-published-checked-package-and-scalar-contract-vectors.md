---
id: TC-233
title: "Published checked-package and scalar contract vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-340
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-341
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-342
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
schema and vectors, the operation catalog and its clause and control
operation identities, and the compound-unit value identity schema and
vectors, and the complete-value and complete-model definition locks and
package-selection vectors. This case proves producer-side schema closure,
digest recomputation and operation-identity and diagnostic-cause closure
only. It does not exercise a production reader, linker, lowerer or evaluator;
those criteria remain with TC-217 and TC-187.

## Test Procedure

Validate each positive fixture against its published schema and apply every
authored adverse mutation. Rebuild each V2 identity preimage from the lock
selections and occurrence-free graph, recompute the JCS SHA-256 package id,
then edit each excluded member and each named preimage member, requiring each
excluded member injected into the identity preimage to be refused. Insert the
published dependency selections of `dependency-selection-vectors.json` into
the all-families package's lock and identity preimage, require the schema to
admit it and its recorded package id to recompute, require each authored
entry mutation, and each `model_entry_mutations` entry placed in
`model_selections`, to fail the schema in either member at the mutated entry, and
apply each order vector's selections to require its recorded order outcome. Validate a `dependency_reference` term against both schemas, and its bare-digest, other-domain, missing-`node` and extra-member mutations; place it as the callee of the `function-call` operation vector and recompute the application-node id after changing only its `package` digest, then only its `node` (FR-322-AC-36). Read
a package whose function calls a dependency `function` returning `Boolean`
through a `dependency_reference` callee, with its dependency package supplied,
and then its FR-322-AC-37 mutations: no dependency package supplied, a
`package` absent from `dependency_selections`, a `node` with no
`declaration`, a `node` naming a declared record or a function returning one,
a function over `Set<R>` and one over a tuple holding `R` for a declared
record `R`, a function over `Reference<M::T>` for a model type `M::T`,
the term as a non-callee argument, the term as a node body root, and the target listed in
`dependencies`. Recompute
every nominal node and compound-unit vector identity and apply each authored
mutation, requiring the exact refusing check it names (`schema` or `semantic`),
including `-0` spellings, and require the semantic oracle itself to refuse
`-0`. Require every qualification-catalog entry of both definition locks to name
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
and FR-272 and require each to list exactly FR-272-AC-10's causes. Parse
the `resource_exhausted` and `cancelled` rows of both and require each to list
exactly FR-272-AC-16's causes and payload keys, and the `conflicting_declaration`
rows to list exactly FR-272-AC-17's cause and payload keys. Parse the
`runtime_invariant` rows of both and require FR-272-AC-18's cause and
population-identity payload keys, and require the `resource_exhausted` rows and
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
`missing-selection`, `unsupported-selection` and `wrong-selection-role` with
every code of the schema's code enum (FR-322-AC-47).
Recompute the QSL FR-092 key of every structural node, grouped or not, of
every published positive fixture, and the recursive-records
fixture's group digests, and read every published positive fixture with the
full reader (FR-322-AC-50). Apply each `derived_shape_key_mutations` entry of
`adverse.json` to its base fixture's node and `identity_projection` entry,
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
Only a node of a kind QSL never writes (FR-093-AC-19), kept so that the
all-families fixture carries all 13 node tags (FR-322-AC-13), is hand
authored, and it is keyed by FR-092/FR-094's rule: the
`quire.structural-node/v1` preimage FR-322's derived-shape key stage
recomputes. The pass/fail oracle for every node, emitted or hand authored,
is this case's independent key recomputation and the full reader's
acceptance, never the node's provenance. Regenerating the fixtures needs
QSL's emitter to write `owner` (QSL FR-093-AC-21, agent-ix/quire-spec-language#648),
which needs IR's `CheckedSemanticNodeV2` `owner` field (IR-646) first. The published fixtures carry the node `owner` member (FR-322
"Owner member schema"). In every positive fixture each declared structural node carries
the `SourceOwner` of its fixture's lock source (`agent-ix`/`example`, or
`a`/`u` in the recursive-records fixture); its `identity_projection` entry
gains the member and the fixture's `package_id` is recomputed. The undeclared
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
fixture with the node id kept, the projection entry patched and
`package_id` recomputed, and `fixtures/` gains `positive-two-owners-a.json`
and `positive-two-owners-b.json`, whose declarations carry the qualified
names QSL emits (`["Point"]`, `["List"]`, `["three"]`), so their `Point`,
`List` and `Integer` nodes are the nodes QSL FR-093-AC-22 emits. `three` is
not in AC-22's source: it is hand authored and keyed by FR-322's
`quire.application-node/v1` rule.

Validate every node of every positive fixture against the owner presence
rule and require each owner-bearing node's `owner` to equal its preimage's
and its projection entry's. Apply each `owner_mutations` schema case (a
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
`adverse.json` gains `clause_owner_cases`
(`clause-precondition-operation-owner` admitted;
`clause-precondition-owned-by-object-type`,
`clause-invariant-owned-by-operation-member`, `clause-invariant-owned-by-systems-interface`, `clause-kind-postcondition`,
`clause-without-kind` and `clause-two-kind-bindings` each refused
`missing_declaration`/`missing-selection` at the clause function's node id)
and `projection_owner_mutations` (`projection-source-owner-differs-from-node`
and `projection-model-owner-differs-from-node`, each refused
`invalid_package`/`invalid-value` at
`/identity_preimage/identity_projection/{i}/owner`), read with the full
reader (FR-322-AC-51, FR-322-AC-55). The `state`/`state_clause` nodes of
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
body changed, canonical order and `dependencies` kept, its node id kept, its
projection entry patched and `package_id` recomputed (FR-322-AC-54).


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
| SE-20 | literal of `value_kind` `uuid` typed by a `string` type with an uppercase value; literal of `value_kind` `timestamp` typed by an `integer` type with value `01` | `ill_typed`/`operator-ineligible`, the family refusal first | FR-322-AC-67 |

### Event type vectors

The owner join, projection and equality behaviour of an event type (FR-322-AC-68) and record conformance (FR-322-AC-69) are decided by TC-281's `EV-` and `RCF-` vectors.

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| ET-01 | A `model`/`event_type` node with a `ModelOwner`, no `declaration`, its own id as `semantic_type` and an empty aggregate body, in a package | Admitted by the V2 schema; its `ModelDeclarationNode` preimage validates against the node-identity preimage schema | FR-322-AC-68 |
| ET-02 | ET-01 with no `owner` | Fails the V2 schema; `malformed_wire` | FR-322-AC-68 |
| ET-03 | ET-01 with a `SourceOwner` and no `declaration` | Fails the V2 schema; `malformed_wire` | FR-322-AC-68 |
| ET-04 | ET-01 with a `declaration` and a `ModelOwner` | Fails the V2 schema; `malformed_wire` | FR-322-AC-68 |

## Expected Results

Each event type vector gives exactly its recorded result.

Every positive artifact validates and recomputes its recorded identity. Excluded
edits leave the package id unchanged and named edits change it. Every mutation
is refused by exactly its named check. Each dependency-selection entry mutation fails the
schema at the mutated entry; its `outcome` is the reader refusal a strict
reader reports for it. Each dependency-selection order vector is admitted by
the schema and admitted or refused by the order rule with exactly its
recorded outcome and loci. A `dependency_reference` term is admitted by both schemas and each of its four mutations fails both; as a call's callee it keys a valid application-node preimage, and changing its `package` or its `node` each gives a new, distinct id. The dependency call package is admitted, and its
mutations refuse `missing_import`/`missing-selection`,
`missing_declaration`/`missing-selection`, `missing_declaration`/`missing-name`,
`ill_typed`/`operator-ineligible` (seven times) and `invalid_semantic_graph`. Every catalog entry names an authored
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
`unknown_profile`.
Every structural node, grouped or not, of every published positive fixture
carries the key QSL's FR-092 key function mints, the recursive-records
group labels are their group digests, and the full reader admits every
published positive fixture. Each structural-node mutation, including the
declared record `Example::Point`, ungrouped or in-group, refuses `invalid_package`/`stale-node-key` at the tampered node,
and the relabelled `Tree` group refuses `invalid_package`/`stale-node-key` at
its member with the lowest node-id digest.
Every owner-bearing node of every positive fixture carries its preimage's
`owner`, and so does its projection entry; no other node carries one. Each
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
