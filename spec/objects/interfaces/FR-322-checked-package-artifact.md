---
id: FR-322
title: "Immutable checked-package artifact"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_004
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-132
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-specification/FR-340
    type: references
  - target: ix://agent-ix/quire-specification/FR-150
    type: references
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
  - target: ix://agent-ix/quire-specification/FR-151
    type: references
  - target: ix://agent-ix/quire-specification/FR-153
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-019
    type: references
  - target: ix://agent-ix/quire-specification/FR-307
    type: references
---

# FR-322: Immutable checked-package artifact

## Description

When a native complete-V1 package completes linking and checking, the producer
SHALL use the immutable `quire.checked-package/v2` contract below rather than
an untyped map, display string or implementation-local structure. Its normative
closed schema and independent vectors are the I04 public boundary; native
parser/CST and implementation-private package values are not part of that
boundary.

## Properties

- `contract_version` — exactly `quire.checked-package/v2`.
- `identity_preimage` — exact `quire.checked-package-id/v2` JCS value whose
  ordered members are `edition`, `profile_selections`, `definition_selections`,
  `model_selections`, `required_features`, `dependency_selections` and the
  semantic graph's `identity_projection`. Source regions, raw artifact
  bytes/digests, diagnostic rendering, reader limits, runtime inputs, backend
  availability and presentation are excluded.
- `package_id` — a `quire.package.semantic/v2` SHA-256 digest of exactly the
  UTF-8 RFC 8785 canonical bytes of `identity_preimage`; it is not embedded in
  that preimage. Its digest value is an unprefixed 64-lowercase-hex string and
  its domain and algorithm are explicit fields.
- `lock` — exact source-document, edition, ordered profile, dependency,
  domain package selection and required-feature identities. Every byte digest
  carries its own declared domain; a raw source/definition digest or a domain
  package `sha256-jcs` digest cannot
  substitute for a semantic package digest or for another artifact role.
- `dependency_selections` — the importing package's resolved library closure
  ([FR-307](../../functional/tooling/FR-307-package-reusable-semantic-libraries.md)):
  one `DependencySelection` per library identity, each
  `{identity, version, package_id}`, in strictly ascending UTF-8 byte order
  of `identity`. `identity` and `version` are the library identity and
  version string an `import "L" version "v" digest "d"` names. `package_id`
  is the dependency's own `quire.package.semantic/v2` `package_id`
  (`{domain, algorithm, digest}`, the `PackageId` shape), whose `digest` is
  the import's `d`; a `quire.definition.bytes/v1`, `quire.source.bytes/v1`
  or `sha256-jcs` domain in `package_id` refuses `digest_domain_mismatch`.
  A second entry whose `identity` equals an earlier entry's refuses
  `invalid_package`/`conflicting-definition` (FR-307's diamond cause), with
  both entries as loci. Otherwise an entry whose `identity` is not strictly
  after its predecessor's in UTF-8 byte order refuses
  `invalid_package`/`invalid-value` at that entry's pointer. A
  `dependency_reference`'s `package` equals one entry's `package_id`. A
  supplied dependency package binds to that entry only when its identity,
  version and recomputed `package_id` equal the entry's; otherwise it
  refuses as FR-307 states (`stale_dependency`/`revision-mismatch` for a
  version difference, `stale_dependency`/`byte-digest-mismatch` for a
  `package_id` difference). The same entries, in the same order, are the
  identity preimage's `dependency_selections`, so each dependency's
  `package_id` enters the importing package's `package_id`.
- `semantic_graph` — one `quire.checked-semantic-graph/v2` closed tagged-node
  union. Its current tags are `scalar_type`, `composite_type`,
  `bounded_domain`, `value`, `expression`, `function`, `model`, `relation`,
  `state`, `temporal`, `protocol`, `claim` and `correspondence`. A node key is
  (`node_id.domain`, `node_id.digest`); node identifiers are unique within the
  graph and the `identity_projection` is the typed-key ordered projection of
  every node excluding source occurrences and presentation. References name a
  compatible declared target by node key; a `state` node with
  `semantic_form: "frame"` carries its references in the closed
  `modifies`/`creates`/`deletes` body shape
  [FR-340](FR-340-checked-package-frame-location.md) specifies. A recursive reference is legal only
  when every cycle member names the same explicit `recursion_group`; all other
  graph cycles refuse. Every node has one or more exact source occurrences.
- `operation` — every `application` semantic term, wherever it occurs in a node
  body, carries `operator`, `operation: {identity, laws, mode, member,
  leaves}`, `result_type` (the node key of the package type the application
  denotes) and `arguments`; every `literal` term carries `type` (the node key of
  its package type) beside `value_kind` and `value`. Neither type is inferred:
  a reader checks both by the rules of "Operation families, groups and
  constraints" below. `identity` is one
  member of the closed `quire.checked-operation-catalog/v1` published as
  `contracts/checked-operation-catalog-v1.json` in `agent-ix/quire-verification-contracts`
  (rev `61f4a44d76d51cc53b2dfe7ed10aba7b23cfa6ef`, SHA-256
  `b67263208e38d1c80b74bf83a43aedc6fc2e50fe7caaa0e7a83d58340aadfdd6`), whose entry names the
  operation's operator class, operand positions and rest, result form,
  constraints, required law roles, mode kind, member kind and leaf source.
  `laws` is the ordered list of `{role, definition}` pairs whose roles are
  exactly the entry's roles. For a value law role the `DefinitionRef` is one of
  that role's catalogued definitions (integer division truncating/floor/Euclidean,
  IEEE 754-2019 default, Unicode 17.0.0 text) and an exact member of the lock's
  `definition_selections`. For a clause operation (`quire.op.temporal.clause`,
  `quire.op.claim.clause`: role `temporal_profile`; `quire.op.protocol.control`,
  `quire.op.state.transition`: role `protocol_profile`) the `DefinitionRef` is
  a published profile definition of that role, matched by identity, revision
  and raw-byte digest, and equals the lock's `profile_selections` row of that
  role. `mode` is `null` when the entry names no mode kind
  and otherwise one catalogued `rounding`, `text_profile` or `absence` value.
  Types are authoritative for a `rounding` or `text_profile` mode: the value
  equals the single value pinned by the operand and result types, and a
  disagreement is refused rather than resolved in favour of either. `member` is
  `null` or the entry's `field`, `position`, `element`, `relationship_end`,
  `operation` or `type_argument` member naming its declaring node, so a
  projection, navigation or dispatched call resolves its name only in that
  node (for a model-owned member, in its exposed effective members, step 3
  of "Model-owned members"), or
  for a clause the `{kind: "profile_operator", operator}` member naming one
  operator the law's profile defines (the eight bounded temporal operators of
  `quire.temporal.bounded-facet/v1`, or the FR-052 bounded control constructs).
  An `operation` member's declaring node is a `model` node whose own
  `semantic_form` is `object_type`, `process`, `state_machine` or
  `persistence_interface`, the forms
  [FR-208](../foundation/FR-208-quire-meaning-vocabulary.md#operation-contracts)'s
  Operation contracts give a dispatched operation; naming any other model
  form, including `systems_interface` (whose ordered feature signatures
  [FR-152](../../functional/type-model/FR-152-bind-systems-model-structures.md)
  gives structural-conformance meaning, never a `dispatch_call` binding),
  refuses `operator-ineligible`/`ill_typed` the same as any other operand
  outside the entry.
  An absent profile law, a law of the wrong profile kind, a profile the lock
  does not select and an operator the profile does not define refuse as
  `operation-law-missing`, `operation-law-mismatch`, `operation-law-unselected`
  and `operation-member-mismatch` respectively.
  `leaves` is empty unless the entry names a leaf source; it then lists,
  in declaration order, one `{path, laws, mode}` entry for every text leaf of
  the compared type (`operand:0`, `inner:0`) or, for `result_inner`, of the
  element type of a `set`, `bag` or `ordered_set` result (`field:<name>`,
  `position:<n>` and `inner` segments), with the text profile law and the mode
  that leaf's type pins. IEEE values have no
  `eq`, `ne`, ordering or `negate` operation: `quire.op.ieee.numeric_equal`,
  `quire.op.ieee.total_order` and `quire.op.ieee.bit_identical` are three
  distinct identities, and a structural comparison reaching a float leaf is
  ineligible. A V2 reader refuses the first failing check in the order owned by
  `quire.native.diagnostics/v1` revision `1-draft.4`: `unknown-operation`,
  `operation-class-mismatch`, `operation-law-missing`,
  `operation-law-mismatch`, `operation-law-unselected`,
  `operation-mode-mismatch` and `operation-member-mismatch` (code
  `invalid_package`); then each argument application under the same order;
  then `operation-mode-type-mismatch` (`invalid_package`) and
  `operator-ineligible` (arity, operand family, constraint, member or result
  type outside the entry; code `ill_typed`); then the leaves.
- `application_node_preimage` — every node whose body contains an application
  is keyed by the JCS SHA-256 of `{version: "quire.application-node/v1",
  node_tag, semantic_form, semantic_type, declaration, recursion, body}`.
  `declaration` is the node's `declaration` member or `null`; `recursion` is
  `null` or `{size, ordinal}` of the node inside its `recursion_group` in graph
  order, and a body reference to a member of that group becomes
  `{term: "group_reference", ordinal}`. Its operation identity, laws, mode,
  member and leaves therefore participate in its node id and, through the
  `identity_projection`, in the package id. An `expression` whose body root is
  an application has the `semantic_form` fixed by its operator class; the
  node's `dependencies` are exactly the unique digest-ascending reference
  targets and member declarations of its body. A contradictory form or
  dependency list is `invalid_semantic_graph`. A node whose retained key differs
  from the digest of its preimage refuses as `invalid_package` with cause
  `stale-node-key`, after every graph-shape refusal and before any declaration,
  frame or operation refusal, at the first such node in ascending retained
  digest order. A frame body refusal, meaning-join or canonical-order
  ([FR-340](FR-340-checked-package-frame-location.md)), is reported after
  every declaration refusal and before any operation refusal, at the first
  `state`/`frame` node in ascending node-id digest order that FR-340's own
  meaning-join/canonical-order precedence selects a defect for.
- `dependency_reference` — a reference to a declaration exported by a
  `dependency_selections` package: `PackageNodeKey{package, node}`, where
  `package` is that dependency's declared `quire.package.semantic/v2`
  identity and `node` is a `WireNodeId` naming the declaration inside that
  dependency's own wire package (QSL ADR-013 T-3, QC-10). Per R-10, a node id
  read from any wire — including a package's own emitted V2 package — stays
  a `WireNodeId`; it becomes a `NodeKey` only by lookup in a checked package
  compiled from digest-addressed source, and never by minting one directly
  from wire bytes, whether in the dependency's own namespace or the
  importing package's. A reader resolves a `dependency_reference` only
  against the named dependency's admitted package. A `dependency_reference`
  naming a `package` absent from `dependency_selections` refuses
  `missing_declaration`/`missing-selection`.
- `model_member` — a `field` or `operation` member whose declaring node is a
  model declaration node: a `model` or `relation` node that carries no
  `declaration` or whose `node_id` equals a model declaration node key of a
  selected domain package (step 2 below), keyed with a `ModelOwner` (QSL
  FR-094). Its body is `aggregate{[]}` and names
  no member, so the reader derives the member from the domain package the
  lock selects for its owner, in the four steps of
  "Model-owned members" below.
- `declaration` — `{qualified_name}`, the name a named, source-declared node
  introduces, with the `qualified_declaration` grammar. It is carried exactly by
  a `scalar_type`, `composite_type`, `bounded_domain`, `function`, `model`,
  `protocol` or `claim` node, or a `value` node other than an `enum_value`, that
  has a `declaration` source occurrence. A builtin or anonymous type reached
  only through `type` occurrences, an expression, a let value and every other
  node must not carry it, and a producer never invents a name.
  `qualified_name` is the package-local declaration path: its segments name the
  declaring scopes from the package root to the declaration. A consumer
  resolves a qualified reference `a::Name` by first resolving `a` to exactly one
  locked package (itself or a `dependency_selections` entry) and then matching
  `Name`, byte for byte, against the `qualified_name` path of that package's
  declarations; no case folding, normalization, suffix match or search across
  packages applies. It enters the node identity preimage. On a node carrying a
  `nominal_identity_preimage` it equals that preimage's
  `qualified_declaration`, else the reader refuses as `invalid_package` with
  cause `declaration-nominal-mismatch`; two nodes carrying the same name refuse
  as `ambiguous_declaration` with cause `ambiguous-name`, both before any
  operation refusal; a model-owned member's `ambiguous-name` is the operation
  refusal of step 3 of "Model-owned members". Library exports resolve only through this member; a
  consumer never recovers a name from source occurrences, display text or a
  node's position.
- `nominal_identity_preimage` — each applicable enum declaration/member,
  dimension or declared-unit node key uses the closed schema and known JCS/SHA-256
  vectors in `proposals/checked-package-v2`. Owner subjects must join an exact
  lock selection. Qualified-name segments, decimal-string integers, normalized
  rationals and sorted/ordered member/term rules are fixed by that contract. A
  strict reader reconstructs each applicable preimage and refuses a mismatched
  `quire.checked-semantic-node/v1` digest as `invalid_semantic_graph`. It also
  joins enum member declaration/case to `semantic_type`, dependency and literal
  body; dimension terms to self-type/dependencies; and unit dimension/target to
  `semantic_type`/dependencies. A valid preimage cannot authorize contradictory
  semantic fields.
- `source_map` — the total semantic-occurrence to original source-region
  mapping. An occurrence key is (`node_id`, `role`, `ordinal`), where role is
  `declaration`, `type`, `expression`, `anchor`, `claim` or `generated`; a
  source region is a half-open UTF-8 byte interval `[start,end)` under one exact
  source-document identity/revision/raw-byte digest. An occurrence may name
  multiple ordered regions; regions for that one occurrence/document cannot
  overlap, while regions belonging to different occurrences may nest or
  overlap. A path, display name or search result is not a source identity.
- `capability_report` — one closed disposition for every required feature and
  selected model capability. Backend/provider installation is absent.
- `diagnostics` — catalog-qualified, stage-qualified typed causes and exact
  available loci. Message text is not a classification or identity component.

`read` receives limits from its caller, and the domain package documents its
`model_selections` name, keyed by `sha256-jcs` digest (FR-154's package
input); they are not artifact data and never participate in package identity. Its closed `ReadResult` is exactly one of:
`admitted { package }`; `refused { code, stage, cause, loci }`; or
`incomplete { limit_kind, limit, consumed, stage, loci }`. The v2 refusal code
vocabulary is `unknown_contract_version`, `malformed_wire`, `duplicate_member`,
`unknown_member`, `noncanonical_wire`, `stale_dependency`,
`digest_domain_mismatch`, `unknown_required_capability`,
`invalid_semantic_graph`, `invalid_source_map`, `unsupported_node_tag`,
`invalid_package`, `ill_typed`, `ambiguous_declaration`, `missing_declaration`,
`invalid_model_binding`, `missing_import`, `unsupported_construct` and
`wrong_snapshot`. The `stale-node-key`, operation and declaration causes
above, and the `missing-name` and `malformed-declaration` frame-entry causes
[FR-340](FR-340-checked-package-frame-location.md) defines, are carried by
`invalid_package`, `ill_typed`, `ambiguous_declaration`,
`missing_declaration` or `invalid_model_binding`, each with the code the
native diagnostics catalog pairs with it, and by no other code; this
exclusivity is over those (code, cause) pairs. The (code, cause) pairs of
"Model-owned members" form a separate set, each a pairing the native
diagnostics catalog lists: step 1 reports the
FR-154 intake and `quire.model.complete/v1` normalization causes under their
own codes (`stale_dependency`, `missing_import`, `invalid_model_binding`,
`missing_declaration`, `unsupported_construct` and `wrong_snapshot`), and
steps 2 to 4 report `missing_declaration`/`missing-selection`,
`invalid_package`/`stale-node-key`, `ambiguous_declaration`/`ambiguous-name`
and `ill_typed`/`operator-ineligible`.
Before expensive semantic work, the reader measures raw wire bytes; a byte
overage returns `incomplete`. It then performs strict syntax/member validation;
a malformed wire returns `refused`. It charges depth, graph nodes/edges,
occurrences/loci, diagnostics and validation visits as they are traversed; the
first exhausted limit returns `incomplete` with its actual counter and no
admitted package. Equal-limit input is admitted if all later checks succeed.

## Model-owned members

A `model_member` is resolved in four steps. Step 1 runs before graph
admission. Steps 2 to 4 run at the application's `operator-ineligible`
check, and their refusals take that check's place in the refusal order.
Steps 1 and 2 also serve the `conforming_reference` constraint ("Reference
conformance" below).

1. **Selection evidence.** After strict syntax and member validation and
   before graph admission, the reader admits the document of each
   `model_selections` row, in lock order, under FR-154's admission checks in
   FR-154's order: the row's digest domain is `sha256-jcs`, else
   `stale_dependency`/`digest-domain-mismatch`; the caller supplies a
   document under the row's digest, else `missing_import`/`missing-selection`;
   the SHA-256 of the document's RFC 8785 bytes equals that digest, else
   `stale_dependency`/`byte-digest-mismatch`; and the document's own identity
   and version equal the row's, else
   `invalid_model_binding`/`wrong-model-selection`. The reader then reads the
   document's declarations under FR-154 and normalizes them under the
   `quire.model.complete/v1` rules. The read's refusal is the first refusal
   in FR-154's report order, with its own code, cause and loci, at stage
   `package_read`. The caller supplies `ModelNormalizationLimitsV1` among the
   reader's limits, and an exhausted one is the read's `incomplete` result.
2. **Owner recovery.** For each admitted selection (`identity`, `version`) and
   each object type and relationship declaration of its document, the reader
   computes the declaration's model declaration node key: the JCS SHA-256 of
   the closed `ModelDeclarationNode` preimage of
   `node-identity-preimage.schema.json`, `{version:
   "quire.structural-node/v1", node_tag, semantic_form, semantic_type: null,
   declaration: null, recursion: null, owner: {kind: "model", identity,
   version, node}, body: {term: "aggregate", members: []}}`, where `node` is
   the declaration's IR node identity. `node_tag`/`semantic_form` is
   `model`/`object_type` for an object type with no `interfaceFeatures`,
   `model`/`systems_interface` for an object type with `interfaceFeatures` and
   `relation`/`relationship` for a relationship. Each key computed charges
   one validation visit. The declaring node's `ModelOwner` is the owner whose
   key equals the node's `node_id` digest and whose preimage `node_tag` and
   `semantic_form` equal the node's, and its declaration `T` is the one that
   key was computed from. The preimage's (`identity`, `version`, `node`,
   `node_tag`, `semantic_form`) determine the key, FR-321 admits one
   selection per `identity` and a document declares each IR node identity
   once, so at most one owner matches. A declaring node that no owner
   matches, whether its owner is unselected or its key names another
   declaration, refuses `missing_declaration`/`missing-selection`. A matched
   node whose `semantic_type` is not its own `node_id`, or that carries a
   `declaration` or a `recursion_group`, or whose body is not
   `{term: "aggregate", members: []}`, refuses `invalid_package`/`stale-node-key`,
   because its key is not the digest of its own preimage.
3. **Resolution.** The member resolves among the effective members of `T`
   that name resolution exposes: `T`'s own and those inherited along its
   declared supertypes, less every member a redefinition hides and every
   redefining member of a less derived owner that phase 4 hides
   (`quire.model.complete/v1` phases 3 and 4). A `field` member considers the
   field members and an `operation` member the operation members. A member
   matches when the last `/` segment of its IR node identity equals `name`,
   byte for byte. Exactly one match resolves the member; two or more refuse
   `ambiguous_declaration`/`ambiguous-name`, which FR-150's effective view
   re-raises at this member; none refuses `ill_typed`/`operator-ineligible`.
4. **Member type.** A field member's type is derived from its `typeRef`, its
   multiplicity and its presence; an operation member's type is derived from
   its result's `typeRef` and multiplicity, and each parameter's type from
   the parameter's `typeRef` and multiplicity. The element type comes from
   the declared type:

   | Declared type | Element type | Node |
   | --- | --- | --- |
   | `ix://quire/native/Boolean` | `Boolean` | `scalar_type`/`boolean` |
   | `ix://quire/native/Integer` | `Integer` | `scalar_type`/`integer` |
   | a value type (FR-208 `quire.meaning.model.value-type/v1`) of the same domain package bound as `Int[lo, hi]`, `lo <= hi`, as TC-196's `Count` and `Small` and TC-197's `Count` are | `Int[lo, hi]` | `bounded_domain`/`integer_range` with bounds `lo` and `hi` over the `Integer` node |
   | an object type `O` of the same domain package | `Reference<O>` | `composite_type`/`reference` over `O`'s model declaration node |

   The multiplicity `{lower, upper, ordered, unique}` then gives the type.
   `[1, 1]` gives the element type `E`. Any other finite `[l, u]` gives
   `K<E>[l, u]`, the `bounded_domain`/`collection_bounds` node with bounds
   `l` and `u` over the `composite_type` node of form `K` over `E`'s node.
   `[0, unbounded]` gives `K<E>`, the `composite_type` node of form `K` over
   `E`'s node, with no `collection_bounds` node. `K` is `set` (unordered,
   unique), `bag` (unordered, not unique), `sequence` (ordered, not unique)
   or `ordered_set` (ordered, unique). A lower bound of `0` makes an empty
   collection legal, and a presence of `optional` makes a field optional
   (`quire.model.complete/v1` Presence row), so `[0, 1]` gives `K<E>[0, 1]`
   and a field whose presence is `optional` has `Option<X>`, the
   `composite_type`/`option` node over the node of the type `X` the
   multiplicity gives. A declared type outside the table, an `unbounded`
   upper bound with a lower bound above `0` and an operation with no result
   give the member no type, and the application refuses
   `ill_typed`/`operator-ineligible`.

   Each node the tables name is the anonymous structural node, with no
   `declaration`, exactly as QSL FR-092 and FR-094 key it, so the node that
   stands for a type, and every application node id that references it, is
   canonical.

For a model-owned member, the `member` result form yields the step 4 type,
and the application's `result_type` is admitted exactly when it names the
node of that type. `member_of` holds exactly when operand 0 resolves to the
declaring node and step 3 resolves the name to a field member of `T`. For a
`dispatch_call`, operand 0 has type `Reference<X>`, where `X`'s model
declaration node is the declaring node; the arguments after the receiver
equal the resolved operation's parameters in number; and the type node of
each argument is the node of its parameter's type. Any other receiver,
argument count or argument type refuses `ill_typed`/`operator-ineligible`.
The declaring node's body stays `aggregate{[]}`, and the member's type is
the step 4 type.

A field of a model object is read as `quire.op.record.project` over a
`quire.op.model.deref` application. The `deref` operand has type
`Reference<O>`, and its `result_type` is `O`'s model declaration node, the
`inner:0` result, of family `object` ("Operation families, groups and
constraints"). The projection's operand is that
`deref` application, its `member` is `{kind: "field", name, declaration}`
with `declaration` naming `O`'s model declaration node, and its
`result_type` is the step 4 type of the resolved field. The `field_owner`
operand group admits the `object` operand, and `member_of` then holds exactly
as stated above. A `Reference<O>` operand has family `reference`, which
`field_owner` does not admit, so a projection over a reference that is not
dereferenced refuses `ill_typed`/`operator-ineligible`.

### Reference conformance

`quire.op.reference.eq` and `quire.op.reference.ne` carry the
`conforming_reference` constraint over operands 0 and 1, each of type
`Reference<X>`, the `composite_type`/`reference` node over `X`'s model
declaration node. At the application's `operator-ineligible` check, the
reader recovers the `ModelOwner` of each operand's model declaration node by
step 2, with step 2's refusals, including when both operands resolve to one
type node, and the two declarations are `A` and `B`. The constraint holds
exactly when `A` and `B` are object type declarations, with or without
`interfaceFeatures`, of one selected document and one conforms to the other
under
[FR-151](../../functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md)'s
conformance relation: they are the same effective type, or a chain of
declared supertypes in that document's `quire.model.complete/v1`
normalization leads from one to the other. This is the pair
[FR-149](../../functional/type-model/FR-149-apply-complete-equality-matrix.md)'s
`Reference<S>` to `Reference<T>` upcast row admits in either direction, so
[FR-153-AC-6](../../functional/type-model/FR-153-query-closed-model-environments.md)'s
subtype object compares with a reference of its supertype. Operand order
does not matter. Conformance follows declared supertypes only, so a
`systems_interface` declaration conforms through its declared supertypes and
never through FR-152 structural conformance. Every supertype edge is a
member of one admitted document, so declarations of two documents never
conform. Two object types with no supertype chain in either direction, and
a relationship declaration, refuse `ill_typed`/`operator-ineligible`. That
is the cause of every catalog operand check; FR-149-AC-12 and TC-198 L08
report the same pair, found while checking source, as
`ill_typed`/`type-mismatch`.

## Operation families, groups and constraints

This section is normative; the catalog file is its machine-readable copy and a
disagreement between them is a catalog defect, not a reader choice.

A type node's **family** is read from the type the node resolves to through its
`bounded_domain` base chain: `scalar_type` forms `boolean`, `integer`,
`rational`, `decimal`, `float32`, `float64` and `text` are their own family, a
`unit` is `quantity`, an enum is `ordered_enum` when its nominal preimage is
ordered and `enum` otherwise; `composite_type` forms `option`, `record`,
`tuple`, `sequence`, `set`, `bag`, `ordered_set` and `reference` are their own
family, and a model declaration node whose `semantic_form` is `object_type` or
`systems_interface` is `object`. A `relation` node and a `model` node of any
other form have no family, so a term they type fits no operand position. An
`object` term is admitted only at a `field_owner` operand position: `object`
is a member of no other group, so the `inner:0` result of
`quire.op.model.deref` is read only through a field projection ("Model-owned
members"). The `type_argument` member of `quire.op.reference.upcast` names a
`Reference<T>` node, whose family is `reference`. A term's family is its
type's family, except that a function reference is `function`, a relation reference is `population`,
an aggregate is `aggregate`, a binding is `binder` (whose *bound* is the
descriptor of its value) and a clause application is `clause`.

| Group | Families | Decision rule |
| --- | --- | --- |
| `any_value` | `boolean`, `integer`, `rational`, `decimal`, `float32`, `float64`, `text`, `enum`, `ordered_enum`, `quantity`, `option`, `record`, `tuple`, `sequence`, `set`, `bag`, `ordered_set`, `reference` | An operand position admits a term whose family is a member. |
| `any_term` | `any_value` plus `function`, `population`, `aggregate`, `binder`, `clause` | Used only as the rest position of clause and call operations. |
| `exact_numeric` | `integer`, `rational`, `decimal` | Exact numeric operands of conversions. |
| `rational_promotable` | `integer`, `rational` | Operands promoted exactly by `n ↦ n/1` (FR-146, FR-149). |
| `ieee_float` | `float32`, `float64` | IEEE operands; no `eq`, `ne`, ordering or `negate`. |
| `collection_kind` | `sequence`, `set`, `bag`, `ordered_set` | Collection operands and `element` members. |
| `structural_kind` | `option`, `record`, `tuple`, `sequence`, `set`, `bag`, `ordered_set` | Structural comparison operands. |
| `enum_kind` | `enum`, `ordered_enum` | Enum operands. |
| `field_owner` | `record`, `object` | Operands of a `field` projection: a record value, or a dereferenced model object whose field the "Model-owned members" rule resolves. |

An operand position or rest names a family or group; a supplied argument fits
exactly when its family is that family or a member of that group, the argument
count equals the positions (or is at least that with a rest), and every
constraint holds. The result form derives the application's result:
`<family>` that family; `operand:<n>` operand n's descriptor; `kind:<n>` operand
n's family (the element type comes from `result_type`); `inner:<n>` the element
type of operand n; `return:<n>` the return type of function operand n; `member`
the type the member names; `absence` `option` for mode `empty`, otherwise
`reference`. The application's `result_type` must have the derived family and,
when the derived result names a type, be that same type node; a clause result
is declared `boolean`. An `expression` or `function` whose body root is the
application has `semantic_type` equal to that `result_type`. A `literal` term's
`type` must have a family its `value_kind` admits (`boolean`, `integer`,
`rational`, `decimal`, `float32_bits`→`float32`, `float64_bits`→`float64`,
`text`, `enum`→`enum` or `ordered_enum`, `none`→`option`). Any failure is
`operator-ineligible`.

| Constraint | Decision rule |
| --- | --- |
| `same_family` | The two named operands have equal families. |
| `same_type` | The two named operands resolve to the same type node. |
| `conforming_reference` | The two named `reference` operands conform: they resolve to the same type node, or their object types conform one to the other as "Reference conformance" below decides. |
| `same_dimension` | Operand 0's unit and the `type_argument` member's unit carry the same `dimension_node_id`. |
| `inner_type` | Operand 1's family is the element family of collection operand 0 and, when operand 1 names a type, the same element type node. |
| `member_of` | Operand 0 resolves to the member's declaring type. |
| `member_family` | The member's type family is the named family or a member of the named group. |
| `bound_family` | The bound of binder operand n has the named family or group. |
| `bound_is_member` | The bound of binder operand n resolves to the same type node as the `type_argument` member. |
| `exact_conversion` | FR-149 classifies operand 0's type to the member type as exact. |
| `range_narrowing` | FR-149 classifies it as range narrowing: a checked obligation that the value lies in the target range (`unproved-range` when not proved), never rounding; `mode` is `null`. |
| `rational_narrowing` | FR-149 classifies a rational-to-rational conversion as rational narrowing: a checked obligation that the reduced numerator and denominator lie in the target bounds (`unproved-range`); `mode` is `null`. |
| `scale_reduction` | FR-149 classifies it as scale reduction to a decimal; this is the only conversion that rounds, and it requires a `rounding` mode. |
| `promotes_exact` | Every named operand is in `rational_promotable` and is promoted exactly to its rational value before the operation. |
| `uniform_rest` | Every rest argument has the same family. |

FR-149 classifies a conversion from declared bounds alone, first match wins:
**exact** when every source value is a target value (an integer range into
`integer` or a containing range; a range into rational or decimal bounds whose
numerators, denominators or coefficients at the minimum scale contain it;
contained rational bounds; a decimal into rational bounds covering its
coefficients and `10^scale_max`; a decimal into a decimal whose scale range
contains its own and whose coefficients contain its own, zero included when the
minimum scale widens); **scale reduction** when the target is `decimal` and the
source is a decimal of larger maximum scale, an unbounded decimal or rational,
or a rational whose maximum denominator exceeds 1; **rational narrowing** for
any other rational-to-rational conversion; **range narrowing** for any other
conversion from `integer`, from a decimal to a rational or decimal, from a
bounded rational to an integer type or (denominator bound 1) a decimal, or from
a decimal of scale 0 to an integer
type. Every other pair has no conversion. `quire.op.numeric.convert` requires
exact, `quire.op.numeric.narrow` range narrowing,
`quire.op.rational.narrow` rational narrowing and
`quire.op.numeric.convert_rounding` scale reduction.

A mode kind in `type_pinned_modes` (`rounding` for `decimal`, `float32`,
`float64` and `quantity`; `text_profile` for `text`) is pinned by each operand
type (a binder's bound) and the result type, or by the result type alone for a
`convert`; a type without a `rounding` binding pins `exact`. The pins must be
one value equal to `mode.value`, else `operation-mode-type-mismatch`. So
`quire.op.quantity.convert` and `quire.op.collection.sum.quantity` carry a
`rounding` mode that must agree with the quantity types, and
`quire.op.collection.set`, `bag`, `ordered_set`, `map`, `flat_map`, `flatten`
and `quire.op.collection.convert` carry the text leaves of a keyed result's
element type.

## Identity and validation

The key is (`contract_version`, `package_id`); runtime inputs, backend
availability, filesystem paths, display ordering and diagnostic rendering are
excluded. A strict reader first validates the selected contract version, closed
wire shape and canonical bytes, then recomputes all required raw and semantic
digests before graph admission. Unknown contract versions, missing or duplicate
members, out-of-domain values, stale dependencies, cross-domain substitutions,
unknown required features and non-canonical encodings refuse before consumption.
An unknown object member is a V2 refusal. An unrecognized `node_tag` refuses
as `unsupported_node_tag` (QSL ADR-013 O-14, QC-19). While
`quire.checked-semantic-graph/v2` is prerelease, QSpec revises this closed
`node_tag` set in place, adding a tag with no `contract_version` bump; a
reader refuses a `node_tag` outside its currently-selected set explicitly
with `unsupported_node_tag` rather than admitting or inferring it. A transport relay may
retain raw bytes selecting a newer graph revision without interpreting them;
that is not an admitted V2 package and a V2 lowerer refuses it. A reader
refuses any unrecognized `contract_version` rather than inferring the missing
identity evidence for it. Exact selected resource limits are admitted;
one-over-limit input returns an incomplete resource disposition without a
partial checked package.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-322-AC-1 | Every admitted semantic occurrence maps to at least one original source region. | Analysis |
| FR-322-AC-2 | Changing backend capability availability cannot change package admission or package identity. | Analysis |
| FR-322-AC-3 | Changing a meaning-affecting lock member changes the package identity. | Analysis |
| FR-322-AC-4 | The public reader refuses unknown versions, malformed/duplicate members, stale dependencies, cross-domain digest substitutions, unknown required capabilities and non-canonical bytes before exposing an admitted package. | Test (TC-217) |
| FR-322-AC-5 | The canonical positive, boundary, refusal and mutation vectors distinguish raw source and definition byte digests and domain package `sha256-jcs` digests from checked-semantic and generated-IR identities, even when digest bytes are equal. | Test (TC-217) |
| FR-322-AC-6 | Exact selected limits admit the boundary vector; one-over-byte, depth, node, edge, locus, diagnostic and work inputs return incomplete resource accounting and no partial checked package. | Test (TC-217) |
| FR-322-AC-7 | A consumer obtains the complete closed semantic graph through the public schema and never through a native parser/CST/package implementation type. | Inspection |
| FR-322-AC-8 | Two conformers derive the same package id from the specified JCS identity preimage; changing an excluded raw artifact/source-map/presentation or reader-limit member does not change it, while changing any named preimage member does. | Test (TC-217) |
| FR-322-AC-9 | The reader result is exactly admitted, refused or incomplete; malformed input and the first exhausted measured limit remain distinguishable and carry their specified typed provenance/accounting. | Test (TC-217) |
| FR-322-AC-10 | Enum, dimension and declared-unit node keys match the closed JCS preimage vectors; stale-body keys, non-normalized rationals, invalid owner joins and every other noncanonical member or term order this artifact refuses -- including an application node's `dependencies` list -- refuse as `invalid_semantic_graph`, except a `state`/`frame` node's own `modifies`/`creates`/`deletes` canonical order, which [FR-340-AC-4](FR-340-checked-package-frame-location.md) owns. | Test (TC-217) |
| FR-322-AC-13 | The published V2 checked-package schema admits its positive fixture carrying all 13 closed node tags with an exact source-map entry for every occurrence, and refuses each authored version, digest-domain and node mutation. | Test (TC-233) |
| FR-322-AC-14 | Each published V2 package fixture's `package_id` equals the RFC 8785 JCS SHA-256 of the identity preimage rebuilt from its lock selections and occurrence-free semantic graph; editing a source-map region, node source occurrence, raw source digest, capability disposition or diagnostic catalog leaves that id unchanged, while editing the edition, a profile/definition/model/dependency selection, a required feature, a node member or node order changes it. | Test (TC-233) |
| FR-322-AC-15 | Each published enum, dimension and declared-unit node vector validates against the closed preimage schema and recomputes its recorded JCS SHA-256; each authored semantic mutation is refused by exactly its named schema or semantic check, an absent owner fails the package lock join, and inside the published nominal package a retained preimage with a changed body, type, dependency or unjoined owner is refused by the published cross-field or owner-join constraint. | Test (TC-233) |
| FR-322-AC-17 | The published operation catalog is closed: both V2 schemas carry identical operation definitions and enumerate exactly its identities, law roles, mode kinds and values, member kinds and operator classes; every law-role `DefinitionRef`, digest included, is a complete-value lock definition; every entry's operands, rest, result form, laws, mode, member, leaf source and constraints use only catalogued families, groups and kinds; integer `div`/`rem` require the integer-division law and `mod` none, IEEE operations require the IEEE profile and have no `eq`, `ne`, ordering or `negate` identity, text operations require the text profile and text mode, `all_instances` takes one population, `lookup` requires an absence mode, `navigate` and `reaches` a relationship end, and the record, tuple and index projections, numeric/total-order/bit-identical IEEE comparisons, `deref` and `dispatch_call` are present. | Test (TC-233) |
| FR-322-AC-18 | Each published operation vector validates against the closed node-preimage schema, recomputes its recorded JCS SHA-256 and is distinct from every other vector; every application node of the published operation package is keyed by its application-node preimage and is a published vector, and a vector outside the package refuses as `operation-law-unselected` when placed in it; floor-to-Euclidean division changes the node and package ids; a retained key refuses as `stale-node-key`; a contradictory form, missing member-declaration dependency, stray recursion group or extra dependency is refused; an application without an operation fails the V2 schema; and the all-families function body calls a published function, changes its node id when rewritten as `dispatch_call` and then refuses as `operation-member-mismatch`. | Test (TC-233) |
| FR-322-AC-19 | Each published operation mutation never keeps its stale key and either rekeys to exactly its named counterpart vector and is admitted exactly when its laws are lock-selected, refuses as `operation-law-unselected` until the lock also selects its law and again under the stale selection, or is refused by exactly its named schema or semantic check with its recorded cause and that cause's code; together the mutations exercise every operation refusal cause. | Test (TC-233) |
| FR-322-AC-21 | Every node of each published V2 package carries a `declaration` member exactly when it declares; the schema refuses a declaring node without it and an expression with it; the member enters the application-node preimage and package id; a name duplicated on two nodes refuses as `ambiguous-name` at the node later in node-id digest order; and a nominal node whose declaration differs from its `qualified_declaration` refuses as `declaration-nominal-mismatch` before any ambiguity. | Test (TC-233) |
| FR-322-AC-22 | Every clause application of the published clause package carries exactly one profile law of its catalogued role equal to the lock's `profile_selections` row and a `profile_operator` member the profile defines; removing the law refuses as `operation-law-missing`, a protocol profile in the temporal role as `operation-law-mismatch`, a lock without the temporal profile row as `operation-law-unselected`, and an undefined, other-profile or absent operator as `operation-member-mismatch`. | Test (TC-233) |
| FR-322-AC-23 | Every application carries a `result_type` and every literal a `type`; a root `result_type` differing from the node `semantic_type`, a `result_type` the operation cannot produce, and a literal whose `type` family its `value_kind` does not admit each refuse as `operator-ineligible`, while the same literal with its declared type is admitted. | Test (TC-233) |
| FR-322-AC-24 | The FR-149 conversion classification is decided from declared bounds: an integer range into a containing integer or rational range is exact, an integer range whose scaled bounds exceed a decimal range is range narrowing, a decimal of scale 3 into scale 2 is scale reduction and a rational into narrower rational bounds is rational narrowing; `quire.op.rational.div` over integer-range operands carries `promotes_exact` and its published vector is admitted, while a decimal operand refuses as `operator-ineligible`. | Test (TC-233) |
| FR-322-AC-25 | An `operation` member resolves only against a declaring `model` node of `semantic_form` `object_type`, `process`, `state_machine` or `persistence_interface`; every other model form, including `systems_interface`, refuses `operator-ineligible`/`ill_typed`. | Test (TC-233) |
| FR-322-AC-26 | A reference to a dependency package's node is carried as `PackageNodeKey{package, node}`, where `node` is a `WireNodeId`; per R-10 it becomes a `NodeKey` only by lookup in a checked package compiled from digest-addressed source, never minted directly from wire bytes in either the dependency's or the importing package's namespace, and a reader resolves it only against the named dependency's admitted package, refusing a `package` absent from `dependency_selections` as `missing_declaration`/`missing-selection`. | Analysis |
| FR-322-AC-27 | An unrecognized `node_tag` refuses as `unsupported_node_tag`; while `quire.checked-semantic-graph/v2` is prerelease, QSpec revises the closed `node_tag` set in place with no `contract_version` bump per addition. | Analysis |
| FR-322-AC-28 | A `ModelOwner` node-identity preimage carries the declaring domain package's `identity` and `version` beside `node`; the published `ModelOwner` vectors validate against the closed preimage schema and recompute their recorded JCS SHA-256; two otherwise-identical `ModelOwner` preimages that differ only in `version` compute distinct node ids, so a `NodeKey` stays unique across domain-package versions. | Test (TC-279) |
| FR-322-AC-29 | Each published model declaration node vector validates against the closed `ModelDeclarationNode` preimage schema and recomputes its recorded JCS SHA-256. A reader recovers a declaring node's `ModelOwner` only from a lock-selected domain package with the node's own tag and form, so `Order`'s node keyed under an unselected version, and the relationship node's id carried by a `model` node, refuse `missing_declaration`/`missing-selection`. A matched node whose `semantic_type` is not its own id, or that carries a `declaration` or a `recursion_group`, or whose body has members, refuses `invalid_package`/`stale-node-key`. | Test (TC-280) |
| FR-322-AC-30 | A model-owned `field` or `operation` member resolves among the declaring type's exposed effective members, own and inherited. A field declared by a supertype resolves on the subtype's node, and a redefined operation resolves to the most derived redefinition. An operation on a `systems_interface` node refuses `ill_typed`/`operator-ineligible`. Two exposed fields of one name refuse `ambiguous_declaration`/`ambiguous-name`. An undeclared name, a name of the other member kind and a relationship declaring node each refuse `ill_typed`/`operator-ineligible`. | Test (TC-280) |
| FR-322-AC-31 | A resolved model member's type follows the declared-type, multiplicity and presence tables. The derived types are `Int[0, 9]` from an integer value type, `Integer` from a constrained native `Integer`, `Option<Boolean>`, `Set<Integer>[0, 3]`, `Sequence<Integer>[0, 5]`, `Set<Integer>[0, 1]`, `Set<Integer>` for `[0, unbounded]`, `Reference<Invoice>` and an operation's `Integer` result. A `Text` field, a `[1, unbounded]` field and an operation with no result each refuse `ill_typed`/`operator-ineligible`. So does a call whose receiver is not a reference to the declaring node, or whose arguments differ in number or type node from the operation's parameters. | Test (TC-280) |
| FR-322-AC-32 | A lock-selected domain package document is checked as evidence before graph admission: a document supplied under the selected digest with matching identity and version passes the selection admission checks, while a digest domain other than `sha256-jcs` refuses `stale_dependency`/`digest-domain-mismatch`, no document refuses `missing_import`/`missing-selection`, an altered document `stale_dependency`/`byte-digest-mismatch` and a document whose version differs from the selection `invalid_model_binding`/`wrong-model-selection`. | Test (TC-280) |
| FR-322-AC-33 | `quire.op.reference.eq` and `quire.op.reference.ne` admit two references to selected object types of which one conforms to the other along declared supertypes: two of one type, a subtype and its supertype in either operand order, a subtype two levels down, and a type with two supertypes against either. A systems interface with no declared supertype against an unrelated object type, two unrelated object types, a reference to a relationship, even compared with itself, and an operand of family `object` refuse `ill_typed`/`operator-ineligible`. An operand whose owner is not selected refuses `missing_declaration`/`missing-selection`, even compared with itself. | Test (TC-281) |
| FR-322-AC-34 | `quire.op.record.project` over a `quire.op.model.deref` result reads a field of a model object: the `deref` result has family `object`, the projection resolves the field on the declaring object type, own or inherited, and its result is the field's member type. A projection over a reference that is not dereferenced, a `field` member naming a node other than the operand's object type, a `deref` whose `result_type` is not the operand's object type, a `deref` result of a relationship, and a projection whose `result_type` is not the field's member type refuse `ill_typed`/`operator-ineligible`. | Test (TC-281) |
| FR-322-AC-35 | A `dependency_selections` entry is a closed `DependencySelection` `{identity, version, package_id}` whose `package_id` is a `quire.package.semantic/v2` `PackageId`: the published package carrying two such entries, one per identity in ascending identity order in both its lock and identity preimage, is admitted by the V2 schema and its recorded `package_id` recomputes from them, and changing a dependency's `package_id` changes it; an entry with a `Selection` or `DefinitionRef` shape, no `package_id`, no `version`, a bare-digest `package_id` or a `package_id` in another digest domain fails the schema in either member, at the mutated entry. Entries ordered by UTF-8 bytes where UTF-16 code-unit order differs are admitted; a second entry of one identity refuses `invalid_package`/`conflicting-definition` with both entries as loci, and an entry not strictly after its predecessor refuses `invalid_package`/`invalid-value` at its pointer. | Test (TC-233) |

## Dependencies

- [interface_004](./interface_004-checked-package.md) owns the operation boundary.
- FR-132 owns the behavior that produces or consumes this value.
- [FR-195](../../functional/ir-proof/FR-195-lower-complete-contract-ir.md)
  consumes this boundary for exact Contract IR lowering.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) owns comparison of
  the distinct identity/digest domains.
- [FR-154](../../functional/type-model/FR-154-admit-domain-package-model.md)
  owns domain package admission, and
  [FR-150](../../functional/type-model/FR-150-preserve-model-normalization-provenance.md)
  owns the effective view a model-owned member resolves in.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
