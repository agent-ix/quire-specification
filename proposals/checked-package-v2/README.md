# CheckedPackage V2 public interchange

This directory is the normative public transport contract for I04 immutable
checked packages. It is intentionally a schema-and-vector boundary, not a
native Rust package API: QSL remains free to use internal parser/CST/package
structures, but any external consumer receives only the versioned contract.

## Contract contents

- `schema.json` defines the closed `quire.checked-package/v2` wire shape.
- `node-identity-preimage.schema.json` defines the closed RFC 8785 preimages for
  complete-V1 enum, dimension and declared-unit node identities.
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
  applications (`temporal`, `protocol_control`, `state_transition`, `claim`)
  each carry a lock-selected profile law and a `profile_operator` member. The
  same package carries an invariant and a precondition as
  `state`/`state_clause` nodes whose `quire.op.state.clause` applications
  carry a `state_clause` member and no law, the precondition's
  `state`/`operation_anchor` node, its `state`/`frame` node with a field
  entry, and the `value`/`parameter` node for `self` (FR-340, FR-341,
  FR-342).
- `node-identity-vectors.json` supplies known JCS SHA-256 vectors for those
  preimages, plus `operation_vectors` and `operation_mutations` for
  application-node preimages.
- `node-identity-vectors.json`'s `frame_mutations` are applied to the
  semantic graph and evaluated at the frame step of `read_package_semantics`
  (FR-322, FR-340); identity projection, `package_id` and source-map checks
  are outside a vector's expectation. Most vectors replace the all-
  families fixture's published frame node's `dependencies` and
  `modifies`/`creates`/`deletes` in place: each `dependencies`, `creates`
  and `deletes` entry is a bare digest string, and each `modifies` entry is
  a `FrameModifiesEntry` (`{kind: "relationship", declaration}` or
  `{kind: "field", declaration, name}`) whose `declaration` is a bare digest
  string; every bare digest is substituted into the node as the `NodeRef`
  `{domain: "quire.checked-semantic-node/v1", digest}` (`schema.json`'s
  `NodeId`). One vector additionally carries a
  `second_frame` member: a complete second `state`/`frame` node object,
  every member a real semantic-graph node requires, inserted into the graph
  verbatim, so the vector exercises the reader's ascending-node-id-digest
  order across two frame nodes.
- `dependency-selection-vectors.json` carries two `DependencySelection`
  entries (`{identity, version, package_id}`), the `package_id` of
  `fixtures/positive-all-families.json` with those entries as its lock's and
  identity preimage's `dependency_selections`, and entry mutations the schema
  refuses: a `Selection` or `DefinitionRef` shape, a missing `package_id` or
  `version`, a bare-digest `package_id` and a `package_id` in another digest
  domain. Its `order_vectors` pin FR-322's order rule: entries in UTF-8 byte
  order (where UTF-16 order differs) are admitted, a repeated identity
  refuses `invalid_package`/`conflicting-definition` and a misordered entry
  `invalid_package`/`invalid-value`, each with its loci.
- Both schemas carry the `dependency_reference` term
  `{term: "dependency_reference", package, node}` (FR-322): `package` is a
  `PackageId` naming a `dependency_selections` entry and `node` a `NodeId`
  naming a declaration node of that dependency. The node-identity preimage
  spells it as the wire does, so both components enter the referencing
  node's id, and it is never listed in a node's `dependencies`.
- `fixtures/` contains independently authored positive package documents and
  structural refusal/mutation cases. Exact reader-limit boundaries,
  duplicate-member/noncanonical raw-byte cases and production-reader outcomes
  remain TC-217 Contract IR consumer evidence; this schema qualification does
  not claim them complete.
- `model-effective-declaration.schema.json` and
  `model-effective-declaration-vectors.json` define the QSL-derived
  `quire.model.effective-declaration/v1`, `quire.model.effective-view/v1` and
  `quire.model.object-universe/v1` preimages selected by
  `quire.model.complete/v1`, with JCS SHA-256 vectors, the reference key order
  and invalid mutations. They add no CheckedPackage V2 wire member.
- `model-member-type-vectors.json` carries a domain package as a projection
  of the declaration records FR-154 intake reads from it, the model declaration node vectors
  that recover a declaring node's `ModelOwner`, the member and selection
  cases for FR-322's "Model-owned members", and the reference equality and
  field projection cases for its "Reference conformance" and model field
  projection rules (TC-281). The SHA-256 of the RFC 8785 bytes
  of its `domain_package` member stands for the document's `sha256-jcs`
  digest.
- The checked semantic graph is a closed `quire.checked-semantic-graph/v2`
  tagged union. It is source-language-neutral transport, not Contract IR and
  not an alternative language semantic authority.

## Identity rules

The package key is the selected contract version and a
`quire.package.semantic/v2` digest over a non-circular JCS preimage. The
preimage is exactly the `quire.checked-package-id/v2` object containing ordered
edition, profile, definition, model, feature and dependency selections plus the
checked semantic graph's typed-key ordered identity projection. It excludes
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
package selects at most one domain package per identity (FR-321), so the
owner's `identity` scopes a `ModelOwner` node's identity to its declaring
domain package. A node id depends only on content: a domain package's
declared version label is not part of it. Source/definition revisions and
digests remain package lock evidence but are excluded from the semantic node
preimage, allowing an unchanged source- or definition-owned declaration to
retain identity across unrelated document revisions.
Qualified names are arrays of ASCII identifier segments. Arbitrary-precision
integers/rationals use canonical decimal strings with no leading zero and no
`-0`, a positive denominator, greatest-common-divisor one and exactly `0/1` for
zero. Unordered enum members
and dimension terms are ascending by their canonical JCS key; ordered enum
members retain declaration order. Duplicate/zero-exponent terms refuse.

The JSON Schema fixes structural shape; the strict reader additionally enforces
these semantic constraints. A targetless root unit has scale `1/1` and offset
`0/1`; every non-root unit has a non-null same-dimension target, nonzero scale,
and an acyclic path to the unique root. Derived-dimension terms reference base
dimension keys. Cross-field joins are exact: an enum member's `semantic_type`
and sole dependency equal its preimage declaration key and its enum literal
body equals the preimage case; a dimension is self-typed and depends exactly on
its term keys; a unit's `semantic_type` equals its dimension key and its
dependencies are exactly that dimension plus its non-null target. Changing a
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
`node_id` equality; stale nominal keys are `invalid_semantic_graph`, while a stale application-node key is `invalid_package` with cause `stale-node-key`.
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

IEEE values have no `eq`, `ne`, ordering or `negate`: numeric equality, total
order and bit identity are three distinct identities, and only
`quire.op.ieee.total_order` orders floats.

A reader reports the first refusal in the order owned by
`quire.native.diagnostics/v1`: graph-shape refusals, then
`stale-node-key` (`invalid_package`) for an application node whose retained key
is not its preimage digest, then declaration refusals, then frame refusals at
the first `state`/`frame` node in ascending node-id digest order carrying one
(that node's own `missing_declaration`/`invalid_model_binding` meaning-join
defect outranking its `invalid_semantic_graph` canonical-order defect, per
FR-340), then state refusals (the placement of `quire.op.state.clause`, then
each `state`/`operation_anchor` node and then each `state`/`state_clause`
node in ascending node-id digest order, per FR-341 and FR-342), then nodes
in ascending node-id digest order and, inside a
node, applications in pre-order. One
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
reference targets and member declarations of its body. Changing division from
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
(`conforming_reference`), decided over the owners the same recovery yields
([FR-322's "Reference conformance"](../../spec/objects/interfaces/FR-322-checked-package-artifact.md#reference-conformance)).

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
