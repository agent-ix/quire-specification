# CheckedPackage V2 public interchange

> BLOCKED-BY QSpec #517: this file still cites the removed `sha256-jcs` / `quire.verification.jcs` identity domain (FR-201); delete that wording when #517 lands.

This directory is the normative public transport contract for I04 immutable
checked packages. It is intentionally a schema-and-vector boundary, not a
native Rust package API: QSL remains free to use internal parser/CST/package
structures, but any external consumer receives only the contract.

## Identity regeneration pending STD-583

BLOCKED-BY STD-583 (owner: planner titan:QSL:plan): the `package_id`,
`node_id`, root-key, effective-id and view hex values in the fixtures and
vectors of this directory are kept only because each is a node or package
identity input of a fixture that must keep validating; they regenerate when
the structural identity lands. This PR adds no recompute or known-answer test.
The same ticket owns the `byte-digest-mismatch` and `digest-domain-mismatch` causes of FR-323-AC-28 and TC-277 BP-03 and BP-04, which exist only for `RawSourceRef` / `quire.source.bytes/v1`, and the remaining digest machinery, which this PR does not
remove: `package_id`, structural node keys, `quire.cache-key/v1`,
`RawSourceRef` and `quire.source.bytes/v1`, definition digests, the library
`package_id` binding, state keys, observation and protocol result digests,
FR-331 state digests and the FR-201 identity-domain rows.

## Contract contents

- `schema.json` defines the closed `quire.checked-package/v2` wire shape.
- `node-identity-preimage.schema.json` defines the closed RFC 8785 preimages for
  complete-V1 enum, dimension, dimension-equivalence and declared-unit node identities.
- The closed `quire.checked-operation-catalog/v1` is not kept here. Its only copy is
  `contracts/checked-operation-catalog-v1.json` in `agent-ix/quire-verification-contracts`,
  which the tests read through that crate. It holds
  every application operation identity with its operator class, operand
  positions and rest, result form, constraints, required law roles (integer
  division, IEEE 754-2019 default, Unicode 17.0.0 text), mode kind (rounding,
  text profile or absence), member kind and structural leaf source.
- `fixtures/positive-operation-identities.json` is a V2 package whose
  application nodes carry integer, IEEE, text, structural, projection, model,
  call and collection operations keyed by their application-node preimages.
  `fixtures/positive-clause-operations.json` is a V2 package whose clause
  applications (`temporal`, `protocol_control`, `claim`)
  each carry a lock-selected profile law and a `profile_operator` member
  (a `state_transition` application does too and is published in
  `fixtures/positive-state-transition.json`). The
  same package carries an invariant and a precondition as
  `state`/`state_clause` nodes whose `quire.op.state.clause` applications
  carry a `state_clause` member and no law, the precondition's
  `state`/`operation_anchor` node, its `state`/`frame` node with a scoped
  field entry and the four frame members, and the `value`/`parameter` node
  for `self` (FR-340, FR-341, FR-342).
- `fixtures/positive-state-transition.json` is a V2 package carrying the
  Boolean model-field transition `Switch.set(flag: Boolean)` over the required
  Boolean field `on` of `acme/orders` (the `Switch` entity is in
  `domain-package-acme-orders.json` and its projection in
  `model-member-type-vectors.json`): the `Switch` model node, its
  `Reference<Switch>`, the `self` and `flag` parameters, a `deref` and a field
  `project` node (the subject), the `flag || on` term, the frame, the no-point
  operation anchor, the precondition and postcondition state clauses and one
  `state`/`transition` node whose `quire.op.state.transition` application binds
  `anchor`, `subject`, `inputs`, `pre` and `post` (FR-341 "State transition
  body", FR-464-AC-19, TC-233). Its keys and `package_id` follow "Regenerating
  protocol fixture keys".
- `fixtures/positive-extension-nodes.json` is a V2 package carrying the
  `retention Audit { subject: "audit", days: 30 }` form of a selected
  extension as one `extension`/`extension_form` node, its two clause subnodes
  and the `retention` entry of `lock.definition_selections` (FR-469,
  TC-951); `fixtures/positive-all-families.json` carries the same form as its
  fourteenth node tag, and the `Tank` hybrid model (one `hybrid_model`, one
  `hybrid_mode` and one `hybrid_transition` node, a rate and a reset value
  that are bare variable references, and a `hybrid` variable whose rate is
  the literal `1`) as its fifteenth (FR-481, TC-966). `node-identity-preimage.schema.json` defines the
  `ExtensionFormNode` preimage that keys it.
- `fixtures/positive-model-tuple-union.json` is a V2 package that selects the
  domain package `acme/shapes` (tuples `Pair`, `Swap`, `Nest`, `Cell`; variant
  types `Shape`, `Color`, `List`): its `model`/`tuple_value_type` and
  `model`/`variant_type` declaration nodes, the `model_root`, and the
  `union_value`, `enum_value`, `quire.op.tuple.project`, `case` and equality nodes
  over them, each keyed by its application-node or nominal preimage
  (FR-322 "Tuple value types and payload-bearing variant types", FR-440,
  FR-441, TC-940). `domain-package-acme-shapes.json` is its selected document, a full FCD
  semantic IR 2.0.0 domain package (FR-322-AC-151; its tuple `positions` depend on IR-1059 / FCD tuple support) and
  `model-tuple-union-vectors.json` publishes its projection (`domain_package`, derived
  from the document); the `model_root` node names the document by identity, the declaration node keys, the application-node
  key vectors and the family, position, member, union-value, enum-value,
  equality, identity, recursion, intake and wire cases the QSpec test runs over
  them; `node-identity-vectors.json` carries the keys
  `model-tuple-value-type-pair` and `model-variant-type-shape-union` and the
  `union_member_vectors` `union-member-model-shape-both` and
  `union-member-model-shape-empty`.
- `contract-package-selection-vectors.json` publishes the typed source-selection
  rows `T`, `D` and `U` and the four compatibility cases of TC-935 (FR-330-AC-10)
  as the authoritative machine-readable form, so a conformance test reads them
  (for example through `QUIRE_SPECIFICATION_DIR`) rather than copying them; the QSpec
  test asserts they equal the Markdown of TC-935 (FR-322-AC-180).
- `option-value-vectors.json` selects an option element type and checked
  payload from published package fixtures, plus closed-body and reader
  refusal mutations for present `option_value` nodes (FR-322, TC-971); the
  qualification test derives the complete preimages and keys from those
  fixture nodes instead of copying node digests.
- `extension-definitions.json` publishes the FR-467 extension definitions
  (`policy`, `quota`, `retention`) a reader holds when it reads these
  fixtures, each as `{reference, definition}` with `definition` exactly the
  `{grammar_schema, typed_node_schema}` members, closed by
  `extension-definitions.schema.json`. A reader holds the published
  definition of each `lock.definition_selections` entry of the fixture it
  reads. `extension-definition-vectors.json` lists the withheld-definition
  controls: a fixture read with `retention` withheld refuses
  `unknown_required_capability` at the extension node (FR-469, TC-951).
- `hybrid-node-vectors.json` holds the graph nodes of QSL FR-447's `Braking`
  hybrid declaration and its `SlowEnough` claim: one `hybrid`/`hybrid_model`
  node, two `hybrid`/`hybrid_mode` nodes, one `hybrid`/`hybrid_transition`
  node, the continuous variables' `value`/`parameter` nodes, the term nodes
  and the claim's `temporal_clause` and formula nodes, each keyed by its own
  preimage, with the `-(2 * a)` term nodes of an identity case and eight
  schema refusals, and as `restored_nodes` the `Braking2` model with two
  initial entries and a reset relation, its `Slows` claim with an `each`
  activation, a capture, hybrid fairness and a timed interval, and the
  `model-steps` claim `Recent` reading `once` (FR-481, TC-966).
  `node-identity-preimage.schema.json`
  defines the `HybridNode` preimage that keys each `hybrid` node.
- `model-owner-form-vectors.json` supplies a domain package projection that
  also declares processes, state machines and a persistence interface, with
  the owner-join, frame-field, anchor, clause-signature and receiver-frame
  cases over them (FR-322-AC-72, FR-340-AC-19/AC-20, FR-341-AC-11/AC-12,
  FR-342-AC-7 to AC-9). Its model declaration node keys are recomputed by the
  tests from each preimage.
- `class-catalog-vectors.json` supplies the canonical `class_catalog` of TC-233 `CCW-10` (effect, commit, event, attempt, `cend`, the three drain entries and `memory flush`) with schema and reader mutations; node digests are `sha256("std-524/" + label)`.
- `refinement-step-occurrence-vectors.json` supplies the cases `RS-01` to `RS-24` of
  FR-322's refinement step occurrences: a refinement owner's `declaration` and
  `generated` `source_map` entries against its step rows, its `steps` order and
  whether it is proof-eligible (its hidden fields derived from `abstract_fields` against the `objects` rows). Cases `RS-18` to `RS-24` cover the closed, identity-excluded `refinement_step_rows` sidecar and its comparison with generated occurrences. The refinement members of these cases remain illustrative and are not a published package; a real entry uses the closed `RefinementBody` in `schema.json` and is compared by node key.
- `refinement-identity-vectors.json` supplies the refinement-name identity
  vectors `RI-01` to `RI-04` (two names, two JCS SHA-256 node ids; a body
  without `name` fails the closed preimage schema; an equal `name` is an
  `ambiguous-name`), the operation-class `source_node` cases `SP-01` to
  `SP-11` (the no-point `operation_anchor` of the declaring type, with and
  without a class catalog) and the row-expression binder cases `BI-01` to
  `BI-07` of FR-375-AC-29 to FR-375-AC-32, and the `selected_roots` section
  (`SR-01` to `SR-13`, five selected model roots recomputed from their
  selections, concrete-identity cases of FR-375-AC-33 and FR-375-AC-34).
- `node-identity-vectors.json` supplies known JCS SHA-256 vectors for those
  preimages, plus `operation_vectors` and `operation_mutations` for
  application-node preimages.
- `node-identity-vectors.json`'s `integer_encoding_vectors` fix FR-322's
  spelling of integers in a node-key preimage: counters at 2^53-1 (JSON
  numbers) and at 2^53 (a decimal string), and an `integer_range` bound
  above 2^53 (a decimal string, as every integer value is).
- `node-identity-vectors.json`'s `frame_mutations` are applied to the
  semantic graph and evaluated at the frame step of `read_package_semantics`
  (FR-322, FR-340); identity projection, `package_id` and source-map checks
  are outside a vector's expectation. Most vectors replace the all-
  families fixture's published frame node's `dependencies` and
  `modifies`/`creates`/`deletes` in place: each `dependencies`, `creates`
  and `deletes` entry is a bare digest string, and each `modifies` entry is
  a `FrameModifiesEntry` (`{kind: "relationship", declaration}` or
  `{kind: "field", declaration, name, scope}`) whose `declaration` is a bare digest
  string; every bare digest is substituted into the node as the `NodeRef`
  `{domain: "quire.checked-semantic-node/v1", digest}` (`schema.json`'s
  `NodeId`). One vector additionally carries a
  `second_frame` member: a complete second `state`/`frame` node object,
  every member a real semantic-graph node requires, inserted into the graph
  verbatim, so the vector exercises the reader's visit order across two
  frame nodes, which ascends by node-id digest.
- `dependency-selection-vectors.json` carries two `DependencySelection`
  entries (`{identity, package_id}`), the `package_id` of
  `fixtures/positive-all-families.json` with those entries as its lock's and
  identity preimage's `dependency_selections`, and entry mutations the schema
  refuses: a `Selection` or `DefinitionRef` shape, a missing `package_id`, a
  bare-digest `package_id`, a `package_id` in another digest domain and an
  extra `version` member. A lock with a `model_selections` member fails the schema (a model is recorded by its `model_root` node only; FR-322).
  Its `order_vectors` pin FR-322's order rule: entries in UTF-8 byte
  order (where UTF-16 order differs) are admitted, a repeated identity
  refuses `invalid_package`/`conflicting-definition` and a misordered entry
  `invalid_package`/`invalid-value`, each with its loci.
  The members its top-level `schema_only` lists (`dependency_selections`,
  `entry_mutations`,
  `order_vectors` and `supply_vectors`) carry placeholder `package_id` or
  digest values, or symbolic supply states, that name no admitted package:
  they exercise the schema, the order rule and the supply order only, and a
  reader that requires every entry's package to be supplied refuses them
  `missing_import`/`missing-selection` at entry 0 by design. Only
  `closure_vectors`, whose digests are recomputed from real packages, and
  `dependency-call-vectors.json` name packages a reader can be supplied. Each `supply_vectors` entry lists the
  direct entries in lock order with a symbolic `supply` state (`bound`,
  `stale` for a supplied package whose recomputed `package_id` differs, or
  `absent`) and the outcome FR-322-AC-84 requires.
  Its `closure_vectors` contain `direct_dependencies` of the importer and
  `dependency_locks` for those direct packages; each dependency lock names a
  fixture `base` and its own `dependency_selections`. The diamond vector
  carries two importer-rooted selection-path JSON-pointer `loci` and a
  conflicting-definition `outcome`. The transitive-only vector carries a
  `dependency_reference` and its call `use`, with an expected missing-selection
  `outcome`. The reader outcomes remain planned; QSpec checks the recorded
  package IDs and the producer-side closure relationships.
- Both schemas carry the `dependency_reference` term
  `{term: "dependency_reference", package, node}` (FR-322): `package` is a
  `PackageId` naming a `dependency_selections` entry and `node` a `NodeId`
  naming a declaration node of that dependency. The node-identity preimage
  spells it as the wire does, so both components enter the referencing
  node's id, and it is never listed in a node's `dependencies`.
- `fixtures/positive-dependency-library.json` is an admitted library package,
  supplied under the library identity `test/predicates`, whose recorded
  `package_id` recomputes. It declares `Lib::is_small`, a `function` over
  `Int[0, 9]` returning `Boolean` whose signature type nodes carry no
  `declaration` or `ModelOwner`, together with the declared record
  `Lib::Point`, a function returning it, functions over `Set<Point>`, over a
  tuple holding `Point` and over `Reference<acme/orders::Order>`, and
  `value`/`parameter` nodes that carry no `declaration`.
  `fixtures/positive-dependency-call.json` selects that library by its
  `package_id` and declares `App::check`, whose body is a
  `quire.op.function.call` with a `dependency_reference` callee naming
  `Lib::is_small`, the Boolean node as `result_type` and no `dependencies`
  entry for the target; it is admitted when the library is supplied.
  `dependency-call-vectors.json` carries FR-322-AC-37's mutations of that
  package as patches over it: no dependency supplied, a `package` that is no
  direct selection, a `node` carrying no `declaration`, a `node` naming the
  record or a function whose signature reaches it or a model type, the term
  as a non-callee argument and as a node body root, and the target listed in
  `dependencies`. Each records its refusal `outcome` and its `locus`, the JSON
  pointer FR-322 fixes for it in the mutated package; node keys and the
  `package_id` are recomputed after the patch.
- `fixtures/positive-compound-unit.json` is a V2 package of four nodes: a
  nominal `dimension` (`Length`), a nominal root `unit` `U` (`metre`), the
  builtin `Integer`, and the structural `scalar_type`/`compound_unit` node
  `metre^1` that holds `unit` = `reference(U)` and an exponent. It is the base
  of `nominal-rekey-vectors.json` (`NK-00` to `NK-14`, RFC 6902 patches ending in
  the recomputed `package_id`), whose cases rekey `U` and observe the nominal
  key stage (FR-322-AC-160 to FR-322-AC-162) and the order of the key-and-join
  stages (`NK-06` to `NK-14`, FR-322-AC-169, FR-322-AC-170).
- `fixtures/` contains independently authored positive package documents and
  structural refusal/mutation cases. Exact reader-limit boundaries,
  duplicate-member/noncanonical raw-byte cases and production-reader outcomes
  remain TC-217 Contract IR consumer evidence; this schema qualification does
  not claim them complete.
- `model-effective-declaration.schema.json` and
  `model-effective-declaration-vectors.json` define the QSL-derived
  `quire.model.effective-declaration/v2`, `quire.model.effective-view/v2` and
  `quire.model.object-universe/v2` preimages selected by
  `quire.model.complete/v1`, with JCS SHA-256 vectors, the reference key order
  and invalid mutations. They add no CheckedPackage V2 wire member.
- `model-member-type-vectors.json` carries a domain package as a projection
  of the declaration records FR-154 intake reads from it, the model declaration node vectors
  that recover a declaring node's `ModelOwner`, the member and selection
  cases for FR-322's "Model-owned members", the operation redefinition cases of
  FR-154-AC-16, and the reference equality, reference upcast and
  field projection cases for its "Reference conformance" and model field
  projection rules (TC-281). The SHA-256 of the RFC 8785 bytes
  of the document `domain-package-acme-orders.json` is the `sha256-jcs` digest
  the `model_root` nodes name; `domain_package` is derived from it and not digested.
- The checked semantic graph is a closed `quire.checked-semantic-graph/v2`
  tagged union. It is source-language-neutral transport, not Contract IR and
  not an alternative language semantic authority.
- A protocol reference is a `protocol`/`protocol_reference` node in that graph,
  with one closed `{term: "reference", target: NodeRef}` body and one or more
  `expression` occurrences, one per source reference to the target. FR-322 binds its
  target to a local protocol node by exact typed key; no extra graph collection
  or dependency-package target lookup is defined.
- The structural type forms used by QSL's StateModel lowering are settled by
  FR-322: a source type alias forwards to the node of its resolved type and
  creates no `alias` node; `Reference<T>` is an anonymous
  `composite_type`/`reference`; `Population<T>[N]` is an anonymous
  `bounded_domain`/`model_population`; and a formed compound quantity unit is
  an anonymous `scalar_type`/`compound_unit`. Their bodies and refusal rules
  are specified by FR-322's derived type forms section.
- A population a domain package declares and a state clause names (`M::P`) is
  a `relation`/`population` model declaration node (FR-322 "Population
  references"): self-typed, with an empty body and a `ModelOwner` that is the
  population's declaration key, keyed like every model declaration node by the
  closed `ModelDeclarationNode` preimage of `node-identity-preimage.schema.json`.
  It is operand 0 of `quire.op.model.all_instances` and `quire.op.model.lookup`
  and carries no member type or maximum. `fixtures/adverse.json`'s
  `population_reference_cases` read it through the full reader against the FCD populations of its `selected_document_populations`, and
  `model-owner-form-vectors.json` carries the `populations` list of its
  `acme/runs` projection, the node vectors `P7` and `P8` and the owner cases
  `PR-01` to `PR-06`.

## Identity rules

The package key is the selected contract version and a
`quire.package.semantic/v2` digest over a non-circular JCS preimage. The
package shows `identity_preimage`, the `quire.checked-package-id/v2` object
containing ordered edition, profile, definition, model, feature and dependency
selections and no node. The package-id preimage is that object plus the
checked semantic graph's identity projection, one occurrence-free entry per
node in graph order (the producer's order of `semantic_graph.nodes`, which a
reader does not sort and which enters the id); a reader derives the
projection from the graph and a package never ships it, so every node is on
the wire once. An `identity_preimage` carrying `identity_projection` fails the
schema. The preimage excludes
package identity itself, source-occurrence/source-map data, raw-byte artifacts,
presentation, runtime inputs, installed backends, diagnostic wording and reader
limits.

Raw source documents, native definition documents, `sha256-jcs` domain packages,
checked semantic graph/package identities and generated Contract IR identities
remain separately typed. Equal hash bytes never authorize substitution across
those domains.

For the nominal scalar node forms covered by the node-preimage schema, the
`quire.checked-semantic-node/v1` digest is SHA-256 of the exact RFC 8785 JCS
preimage. An owner subject must match an admitted source, definition or model
selection; source/definition owners use authority plus identity, and model
owners use the domain package identity plus the IR node identity: a checked
package selects at most one domain package per identity (FR-321), so the owner's
`identity` scopes a `ModelOwner` node's identity to its declaring domain
package. A node id depends only on content: an unchanged source- or
definition-owned declaration retains its identity across unrelated document
changes. A structural node carries its `owner` member (`SourceOwner` on a
declared node, `ModelOwner` on a model declaration node or clause function, none
elsewhere), which the derived identity projection carries, so a reader re-derives
every structural key from the package alone. `fixtures/positive-two-owners-a.json`
and `-b.json` declare the same records under two `SourceOwner`s and differ in
every owner-keyed node id. Qualified names are arrays of ASCII identifier segments.
Arbitrary-precision integers/rationals use canonical decimal strings with no
leading zero and no `-0`, a positive denominator, greatest-common-divisor one
and exactly `0/1` for zero. Unordered enum members and dimension terms are
ascending by their canonical JCS key; ordered enum members retain declaration
order. Duplicate/zero-exponent terms refuse.

The JSON Schema fixes structural shape; the strict reader additionally enforces
these semantic constraints. A targetless root unit has scale `1/1` and offset
`0/1`; every non-root unit has a non-null same-dimension target, nonzero scale,
and an acyclic path to the unique root. Derived-dimension terms reference base
dimension keys. Cross-field joins are exact: an enum member's `semantic_type`
and sole dependency equal its preimage declaration key and its enum literal
body equals the preimage case; a dimension is self-typed and depends exactly on
its term keys; a unit's `semantic_type` equals its dimension key and its
dependencies are exactly that dimension plus its non-null target; a
`scalar_type`/`dimension_equivalence` node (FR-142 "Dimension equivalence") is
self-typed, carries no `declaration`, holds a `SourceOwner` and the keys of the
two dimension nodes it relates, distinct and in ascending digest order, and a
`root_scale` (a positive reduced rational: one canonical root of the first node
is `root_scale` canonical roots of the second), and depends on exactly those
keys. The `root_scale` values of the base pairs multiply to one around every
cycle, and a pair of two non-base nodes carries the ratio their base roots fix. Readers recompute the equivalence classes from
these nodes, so no other member records one. Changing a
body/type/dependency while retaining the nominal preimage therefore refuses.
Runtime compound-unit values are evaluator-owned
`quire.value.compound-unit/v1` identities defined with the complete-value
profile; they are not I04 graph nodes. The concrete RFC 6902
patches in `invalid_mutations` retain the base vector's old node digest and must
all refuse as `invalid_semantic_graph`.

## Reader boundary

A reader selects exactly this version, rejects duplicate and unknown members,
checks canonical bytes and all identity joins before exposing a graph, and
reports malformed/stale/cross-domain/unknown-capability input as typed refusal.
For a node form covered by the node-preimage schema, it reconstructs the closed
preimage from the graph and lock, hashes its JCS bytes and requires exact
`node_id` equality; a stale nominal key is `invalid_semantic_graph` at that
node's `node_id`, reported by the nominal key stage, which follows the
application-node key stage and precedes the derived-shape key stage (so it
precedes every structural `stale-node-key`), while a stale application-node or
structural key is `invalid_package` with cause `stale-node-key`. A dangling
`reference` whose target is the recomputed key of a stale nominal node is
dependent on that rekey and is deferred to the nominal stage
(`nominal-rekey-vectors.json`, FR-322-AC-160 and FR-322-AC-161).
The node forms a refinement row reads (history read, whole-population value,
`count`/`only`, `seq`) and their recomputed keys and refusals are in
`refinement-row-node-vectors.json` (FR-322-AC-163 to FR-322-AC-168).
Its outcome is the closed sum type admitted/refused/incomplete. It enforces
caller-provided byte/node/edge/occurrence/diagnostic/work limits;
exhaustion is incomplete, not an empty graph or a successful package. A node
`body` follows the closed five-stratum grammar of FR-322 ("Body grammar"), so
the JSON depth of every package is fixed by the schema and the reader has no
depth limit; a body outside the grammar is a malformed wire.

The QSpec qualification crate validates schema/vector correspondence only. The
Contract IR consumer owns the production Rust strict reader and lowering path.

## Operation identity rules

Every V2 `application` term carries its `operator`,
`operation: {identity, laws, mode, member, leaves}`, its declared
`result_type` and its `arguments`, and every `literal` term carries its
declared `type`, so the package alone gives each application exactly one
meaning and no type is inferred. The identity is a member of the
`quire-verification-contracts` operation catalog, and its entry fixes the rest:

- the operator class equals the entry's class;
- `laws` lists exactly the entry's roles in order. A value role
  (`integer_division`, `ieee_profile`, `text_profile`) names one of that role's
  catalogued definitions that is also a member of the lock's
  `definition_selections`. A clause role (`temporal_profile` for
  `quire.op.temporal.clause` and `quire.op.claim.clause`, `protocol_profile`
  for `quire.op.protocol.control` and `quire.op.state.transition`) names a
  published profile definition of that kind that equals the lock's
  `profile_selections` row of that role;
- `mode` is `null` when the entry names no mode kind, and otherwise one of that
  kind's values; for `rounding` and `text_profile` the operand and result types
  are authoritative and must pin exactly that value. `quire.op.quantity.convert`
  and `quire.op.collection.sum.quantity` carry a `rounding` mode checked this
  way;
- `member` is `null` or the entry's member kind naming its declaring node
  (record field, tuple position, element, relationship end, dispatched
  operation or type argument), or for a clause
  `{kind: "profile_operator", operator}` naming an operator the law's profile
  defines, or for `quire.op.state.clause` `{kind: "state_clause", clause}`
  naming the clause kind (FR-341). `quire.op.model.reaches_field` carries a
  `field` member and the `reference_edge` constraint (FR-322 "Reaches over a
  field"). A dispatched operation's declaring node is a `model` node whose own
  `semantic_form` is `object_type`, `process`, `state_machine` or
  `persistence_interface`
  ([FR-322-AC-25](../../spec/objects/interfaces/FR-322-checked-package-artifact.md));
  naming any other model form -- including `systems_interface`, whose
  ordered feature signatures
  [FR-152's Systems-model roles table](../../spec/functional/type-model/FR-152-bind-systems-model-structures.md#systems-model-roles)
  gives structural-conformance meaning (FR-151), never a
  `dispatch_call` binding -- is `operator-ineligible`/`ill_typed` here. FR-208
  separately pins a declaring-node cause for three of those other forms:
  `unsupported_construct`/`declaration-form` on a `record_value_type`
  (FR-208-AC-4, which cannot declare an operation at all) and
  `invalid_model_binding`/`malformed-declaration` on an `event_type`
  (FR-208-AC-4) or a `namespace` (FR-208-AC-8) -- that is FR-208's own refusal
  at the declaring node, a separate rule from this package's
  `operator-ineligible` reference check above;
- `leaves` is empty unless the entry names a leaf source, and then names every
  text leaf of the compared type (`operand:0`, `inner:0`) or of a `set`, `bag`
  or `ordered_set` result's element type (`result_inner`: the collection
  constructors, `map`, `flat_map`, `flatten` and `quire.op.collection.convert`)
  with its law and type-pinned mode;
- the arguments fit the entry's operand positions, rest, constraints and result
  form, and `result_type` is the type that form derives.

The families, groups, constraints, result forms and FR-149 conversion classes
are decided by the normative table in
[FR-322](../../spec/objects/interfaces/FR-322-checked-package-artifact.md#operation-families-groups-and-constraints).
`quire.op.rational.div` takes `integer` or `rational` operands and promotes them
exactly (`promotes_exact`); `quire.op.numeric.narrow` and
`quire.op.rational.narrow` are checked range and rational narrowing
obligations with no mode, and only `quire.op.numeric.convert_rounding`, a
decimal scale reduction, rounds.

IEEE values have no `eq`, `ne` or ordering identity: numeric equality, total
order and bit identity are three distinct identities, and the source `=` and
`!=` over floats lower to `quire.op.ieee.numeric_equal` and its
`quire.op.boolean.not`. `quire.op.ieee.float32.negate` and
`quire.op.ieee.float64.negate` flip the sign bit of one float, with the
`ieee_profile` law, no mode and the operand's type as result. A
`quire.op.structural.eq` or `.ne` over a type holding a float lists that float
leaf with the `ieee_profile` law.

A reader reports the first refusal in the order owned by
`quire.native.diagnostics/v1`: graph-shape refusals, then
`stale-node-key` (`invalid_package`) for an application node whose retained key
is not its preimage digest, then declaration refusals, then frame refusals at
the first `state`/`frame` node carrying one, visiting them in ascending
node-id digest order
(that node's own `missing_declaration`/`invalid_model_binding` meaning-join
defect outranking its `invalid_semantic_graph` canonical-order defect, per
FR-340), then state refusals (the placement of `quire.op.state.clause`, then
each `state`/`operation_anchor` node and then each `state`/`state_clause`
node, visiting each kind in ascending node-id digest order, per FR-341 and
FR-342), then nodes, visited in ascending node-id digest order and, inside a
node, applications in pre-order. These visit orders choose the refusal to
report; they are not an order of `semantic_graph.nodes`. One
application checks `unknown-operation`, `operation-class-mismatch`,
`operation-law-missing`, `operation-law-mismatch`,
`operation-law-unselected`, `operation-mode-mismatch` and
`operation-member-mismatch` (code `invalid_package`); then its argument
applications; then `operation-mode-type-mismatch` (`invalid_package`) and
`operator-ineligible` (`ill_typed`); then its leaves.

Every node whose body contains an application has `node_id.digest` equal to the
JCS SHA-256 of `{version: "quire.application-node/v1", node_tag, semantic_form,
semantic_type, declaration, recursion, body}`, where a reference to a member of
the node's own recursion group becomes a `group_reference` ordinal. An
expression whose root is an application has the semantic form fixed by its
operator class, and its dependencies are exactly the unique digest-ascending
reference targets and member declarations of its body. Every other node's
dependencies follow FR-322's `dependencies` rules. Changing division from
floor to Euclidean therefore changes the node id and the package id, and the
`division-law-unselected-by-package-lock` mutation shows that a law absent from
the lock refuses.

## Model-owned members

A `field` or `operation` member whose declaring node is a model declaration
node (a `model` or `relation` node with no `declaration`, keyed with a
`ModelOwner`) is resolved through the domain package the lock selects. The
node's body stays `aggregate{[]}`. `read` receives the selected domain package
documents by `sha256-jcs` digest and admits each under FR-154 before graph
admission. It recovers the declaring node's `ModelOwner` by recomputing the
`ModelDeclarationNode` key of every object type and relationship declaration
of each selected document and matching the node's id. It resolves the member
name among the declaring type's exposed effective members, own and inherited,
and derives the member type from the resolved field or operation record. The
rule, its type tables and its refusals are
[FR-322's "Model-owned members"](../../spec/objects/interfaces/FR-322-checked-package-artifact.md#model-owned-members).

A field of a model object is `quire.op.record.project` over a
`quire.op.model.deref` result: `deref` yields the object type's model
declaration node, of family `object`, which the projection's `field_owner`
operand group admits. `quire.op.reference.eq` and `quire.op.reference.ne`
compare two references whose object types conform one to the other
(`conforming_reference`), and `quire.op.reference.upcast` retypes a
reference only to a type its object type conforms to (`conforming_upcast`),
each decided over the owners the same recovery yields
([FR-322's "Reference conformance"](../../spec/objects/interfaces/FR-322-checked-package-artifact.md#reference-conformance)).
The operation catalog's `constraint_kinds` vocabulary lists `conforming_upcast`
beside `conforming_reference`.

## Declarations

A named, source-declared `scalar_type`, `composite_type`, `bounded_domain`,
`function`, `model`, `protocol` or `claim` node, and a non-enum `value` node,
carries `declaration: {qualified_name}` exactly when it has a `declaration`
source occurrence; builtin and anonymous nodes and every other node carry none,
and a producer never invents a name. `qualified_name` is the package-local
declaration path; `a::Name` resolves `a` to one locked package and then matches
`Name` byte for byte against that package's declaration paths. The member is
part of the node identity. On a nominal node it equals the nominal
`qualified_declaration` (else `declaration-nominal-mismatch`, `invalid_package`),
and two nodes declaring one name refuse as `ambiguous-name`
(`ambiguous_declaration`). Library exports resolve only through this member.
- `fixtures/positive-integer-literal-references.json` is the reference form of the `three` application of `positive-two-owners-a.json` (its integer literal operands are references to `value`/`literal` nodes); the reader admits it under a node identity different from the inline form's.
- `timed-slot-selector-vectors.json` publishes standalone `clock_place` nodes
  for the slot selector of a template step (FR-322-AC-148..FR-322-AC-150,
  TC-367 CK-26): positive collection kinds, refused selectors and the guard
  cases, whose package-level mutations the tests replay. Its `presence`
  section publishes the derived `presence_read` predicate over a clock place
  (FR-322-AC-155..157), and `model_join` the `object_type` node of `Order`
  with and without `v: Option<Clock>`. `domain-package-acme-timed-slots.json`
  is the full selected document (FR-322-AC-240, FR-322-AC-241; it declares
  the `Order-order-Order` relationship and the `size` operation that the
  package's model-owned nodes 7 and 30 name) and
  `fixtures/positive-timed-slot-nodes.json` the complete package that selects
  it and carries the `SL`, `PR` and model nodes; the `GD-02` and `GD-03`
  `patch` members are RFC 6902 patches on that package (FR-322-AC-242,
  FR-322-AC-243). The case nodes are embedded both in the vectors file and
  in the package fixture; their equality is enforced through `package_node`,
  so the two copies cannot drift. Each node key is the SHA-256 of the
  RFC 8785 structural or application preimage, and `SL-06` equals the
  published fixture place. Its `aggregate_order` section publishes the
  declared-order rule of the timed-root aggregates, the place path key that
  orders places of one region, and the replayable `AO-02`..`AO-10` patches
  (FR-322-AC-260, FR-322-AC-265, TC-367 CK-30, CK-36).
- `timed-clause-node-vectors.json` publishes the clause forms (`clock_guard`,
  `clock_reset`, `time_invariant`, `clock_constraint`, in both kinds) of
  FR-322 "Clause nodes" as the `forms` field tables and the replayable
  `CL-01`..`CL-56` cases, the `QN-01`, `QN-02` quantified refusals and the `excluded_forms` producer cases (`GJ-01`..`GJ-03`, `CW-01`..`CW-05`, `SD-01`, `SD-02`: a guard disjunct with no clock constraint, a clock write that is not a reset to a constant, a selection reset reading element data) (FR-322-AC-261..FR-322-AC-267, FR-342-AC-11, FR-415-AC-25, TC-367
  CK-32..CK-38). A `clock_place` is exactly `model`, `path` and `resolved_path` (the `guards` field is removed; `CL-27` refuses a `guards` binding). The member keys that order nodes of one source region are structural (a term is keyed by its form and body shape, never by a digest or a source region), and `CL-33`..`CL-53` pin each component. `fixtures/positive-timed-clause-nodes.json` is the complete
  package that carries one node of each form and `domain-package-acme-timed-clauses.json`
  its selected document; the cases are RFC 6902 patches on that package.
- `clock-presence-frame-vectors.json` publishes the frame rule of FR-415-AC-21
  (no implicit reset on a presence change) for an `Option<Clock>` place
  (TC-367 CK-31).

## Regenerating protocol fixture keys

No generator is checked in (first-party tooling here is Rust; a Rust
generator is a separate ticket). After a body edit, the three protocol
fixtures `fixtures/positive-all-families.json`,
`fixtures/positive-timed-model-nodes.json` and
`fixtures/positive-tick-timed-model-nodes.json` are regenerated by hand as
follows. Do not regenerate other fixtures this way: for example
`positive-operation-identities.json` holds text literals that are typed
differently on purpose.

1. Every text literal (`value_kind: "text"`) of a protocol node carries as
   `type` the package's `scalar_type`/`text` node id (FR-464).
2. Recompute each application node's `node_id.digest` as SHA-256 over the RFC
   8785 (JCS) bytes of its `quire.application-node/v1` preimage:
   `{version, node_tag, semantic_form, semantic_type, declaration (or null),
   recursion ({size, ordinal} within its `recursion_group`, else null), body}`
   where references to members of the node's own recursion group in `body` are
   `{"term": "group_reference", "ordinal": n}`. `application_node_preimage` in
   `tests/checked_package_v2.rs` is the reference. A changed key is rewritten
   everywhere it is referenced (dependencies, bodies, `source_map`); repeat
   until no key changes.
3. Drop `source_map` entries whose `node_id` names no node of the graph.
4. Recompute `package_id.digest` as SHA-256 over the JCS bytes of
   `identity_preimage` plus `identity_projection`, the node list in graph
   order with each node's `occurrences` removed (`package_id_preimage` in the
   test). The source map is not part of package identity.
5. Rebuild what derives from a package id elsewhere:
   `dependency-selection-vectors.json` carries the all-families package id as
   the leaf id of its `closure_vectors`, the ids of the direct dependencies
   built on it, and its own top-level `package_id`; `cargo test --test
   checked_package_v2` names each stale value. The top-level
   `dependency_selections` there are placeholder ids (`3333...`, `4444...`)
   that no admitted package has, so the reader refuses them `missing_import`;
   never replace them with a fixture's own package id (a test checks this).
6. Node-position loci follow the fixture: a vector whose mutation appends a
   node (`timed-model-node-vectors.json` TM-23) names the new fixture length.
