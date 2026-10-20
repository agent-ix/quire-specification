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
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/FR-370
    type: references
  - target: ix://agent-ix/quire-specification/FR-342
    type: references
  - target: ix://agent-ix/quire-specification/FR-464
    type: references
  - target: ix://agent-ix/quire-specification/FR-465
    type: references
  - target: ix://agent-ix/quire-specification/FR-466
    type: references
  - target: ix://agent-ix/quire-specification/FR-440
    type: references
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-432
    type: references
  - target: ix://agent-ix/quire-specification/FR-441
    type: references
  - target: ix://agent-ix/quire-specification/FR-043
    type: references
  - target: ix://agent-ix/quire-specification/FR-150
    type: references
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/FR-208
    type: references
  - target: ix://agent-ix/quire-specification/FR-340
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
  - target: ix://agent-ix/quire-specification/FR-151
    type: references
  - target: ix://agent-ix/quire-specification/FR-153
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-019
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-093
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-092
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-094
    type: references
  - target: ix://agent-ix/quire-specification/FR-307
    type: references
  - target: ix://agent-ix/quire-specification/FR-460
    type: references
  - target: ix://agent-ix/quire-specification/FR-461
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

- `contract_version` — exactly `quire.checked-package/v2`, the document-type
  tag that tells a checked package apart from every other document.
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
  domain package selection and required-feature identities. A definition
  selection is a `DefinitionRef` `{authority, identity}`. Every byte digest
  carries its own declared domain; a raw source digest or a domain
  package `sha256-jcs` digest cannot
  substitute for a semantic package digest or for another artifact role.
- `dependency_selections` — the importing package's resolved library closure
  ([FR-307](../../functional/tooling/FR-307-package-reusable-semantic-libraries.md)):
  one `DependencySelection` per library identity, each
  `{identity, package_id}`, in strictly ascending UTF-8 byte order
  of `identity`. `identity` is the library identity an import names. A library
  identity is a non-empty string with no segment structure: `/`, `.` and
  every other character are ordinary bytes, and two identities are equal
  only when their UTF-8 bytes are equal. `package_id`
  is the dependency's own `quire.package.semantic/v2` `package_id`
  (`{domain, algorithm, digest}`, the `PackageId` shape), recomputed from
  the library compiled from the source supplied under `identity`; an import
  names only the identity and states no digest. A `quire.source.bytes/v1`
  or `sha256-jcs` domain in `package_id` refuses `digest_domain_mismatch`.
  A second entry whose `identity` equals an earlier entry's refuses
  `invalid_package`/`conflicting-definition` (FR-307's diamond cause), with
  both entries as loci. Otherwise an entry whose `identity` is not strictly
  after its predecessor's in UTF-8 byte order refuses
  `invalid_package`/`invalid-value` at that entry's pointer. A
  `dependency_reference`'s `package` equals one entry's `package_id`. A
  supplied dependency package binds to that entry only when its identity and
  recomputed `package_id` equal the entry's; a `package_id` difference
  refuses `stale_dependency`/`content-mismatch` naming the entry's
  `identity`, its `package_id` and the recomputed one. The same entries, in the same order, are the
  identity preimage's `dependency_selections`, so each dependency's
  `package_id` enters the importing package's `package_id`. A reader
  admitting a package with a non-empty `dependency_selections` is supplied
  each selected dependency's admitted package; an entry with none supplied
  refuses `missing_import`/`missing-selection` at that entry's pointer,
  before any `dependency_reference` check.
- `refinement_internals` — the compilation's refinement internal list
  ([FR-432](../../functional/protocol/FR-432-map-every-protocol-step-in-refinements.md);
  [FR-375](../../functional/analysis/FR-375-declare-model-refinement.md)),
  a lock member only: an array of closed `{refinement, internal}` objects,
  present only when some refinement's list is non-empty and absent otherwise
  (absent is the empty list, never `null` or `[]`). It is canonical: objects
  ascend by `refinement` and each `internal` array ascends by class spelling,
  both in UTF-8 byte order, with no empty `internal` and no repeated
  `refinement` or spelling. An object or spelling not strictly after its
  predecessor refuses `invalid_package`/`invalid-value` at its pointer, and a
  repeated one `invalid_package`/`conflicting-definition` with both entries as
  loci. A reader checks only this shape; whether each spelling resolves is
  decided by the recompile (FR-432). The member is not in `identity_preimage`:
  its effect on meaning is already in the refinement nodes' `abstract_internal`
  (FR-375), and a lock that disagrees with the nodes recompiles to another
  `package_id`, which replay refuses (FR-323).
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
  [FR-340](FR-340-checked-package-frame-location.md) specifies; a `state`
  node with `semantic_form: "state_clause"` carries the
  `quire.op.state.clause` application
  [FR-341](FR-341-checked-package-state-clause-body.md) specifies, and one
  with `semantic_form: "operation_anchor"` the binding aggregate
  [FR-342](FR-342-checked-package-operation-anchor-body.md) specifies. A
  `protocol` node whose `semantic_form` is not `protocol_reference` carries the body
  [FR-464](FR-464-checked-package-protocol-node-bodies.md),
  [FR-465](FR-465-checked-package-protocol-declaration-bodies.md) and
  [FR-466](FR-466-checked-package-protocol-control-bodies.md) specify. A
  `protocol` node with `semantic_form: "protocol_reference"` is the sole
  protocol-reference carrier. Its body is exactly
  `{term: "reference", target: NodeRef}`: `target` is the local
  `quire.checked-semantic-node/v1` identity of a `protocol` node in this same
  `semantic_graph.nodes` collection. It has one or more `expression`
  occurrences, one for each source reference to that target, and no other role;
  no `declaration`, `owner`, `nominal_identity_preimage` or `recursion_group`,
  the same `semantic_type` as its target, and `dependencies` containing
  exactly that target. A reader identifies the carrier by its tag and form,
  never by a name, a generic reference term in another node or a separate
  collection. A `dependency_reference` cannot be its body or its target;
  selected dependency semantics remain limited to the function-call callee
  described below. The reference body and target enter the carrier's
  structural node-key preimage (QSL FR-092) and the package identity
  projection. Occurrences enter neither, so every source reference to one
  protocol shares the one carrier, keyed once per target. The occurrences of a
  carrier have distinct ordinals, and each has its own `source_map` entry,
  keyed (`node_id`, `expression`, `ordinal`), naming that reference's exact
  source regions. A carrier without an `expression` occurrence, with an
  occurrence of any other role, or whose occurrences or `source_map` entries
  repeat an (`node_id`, `role`, `ordinal`) key is refused as
  FR-322-AC-66 states. The closed V2 schema fixes the carrier's body shape; after
  schema, graph-shape and stale-node-key checks, the reader resolves its
  target by exact typed node key. An absent target refuses
  `missing_declaration`/`missing-name`; a present non-`protocol` target or
  unequal `semantic_type` refuses `ill_typed`/`operator-ineligible`, at the
  carrier's `/body/target` pointer. Carriers are checked in ascending node-id
  digest order at the declaration step, before frame and operation checks.
  The earlier graph-shape step compares `dependencies` with the body target
  without resolving that target; an unequal list is
  `invalid_semantic_graph` at `/dependencies`. Target existence, tag and type
  are checked only at the later carrier join. The carrier has no package
  selector: a key absent locally
  fails this join even if a selected dependency contains that key.
  For a worked admitted example, take a package whose graph holds a
  conforming protocol clause node `P`
  ([FR-464](FR-464-checked-package-protocol-node-bodies.md)) and add one
  carrier `R` with `node_tag: "protocol"`,
  `semantic_form: "protocol_reference"`, `body: {term: "reference",
  target: P.node_id}`, `semantic_type: P.semantic_type`,
  `dependencies: [P.node_id]` and one `expression` occurrence (a second
  source reference to `P` adds a second `expression` occurrence, ordinal 1, and
  its own `source_map` entry to the same `R`; it adds no node). `R`'s node key
  is the `quire.structural-node/v1` digest of its preimage with
  `declaration: null` and `recursion: null`. Include `R` in the graph and
  identity projection in typed-key order and in the source map (`R`'s
  occurrences in ascending ordinal, which is not a requirement), then
  recompute `package_id`; the reader admits this package and resolves `R` to
  `P`. Replacing `R.body.target` and its sole dependency together with a local
  non-protocol node's key, or with a key absent from the graph, while
  recomputing the carrier and package identities,
  gives the respective typed refusals above. Adding the second occurrence to `R`
  leaves `R.node_id.digest` and `package_id` unchanged.
  A `composite_type` node with `semantic_form: "union"`, a `value` node with
  `semantic_form: "union_value"` and an `expression` node with
  `semantic_form: "case"` carry the bodies
  [FR-440](FR-440-checked-package-union-and-case-nodes.md) specifies, and
  each union member is keyed as
  [FR-441](FR-441-union-member-key.md) specifies. A
  `value` node with `semantic_form: "parameter"` stands for a value a binder
  binds (a function parameter, or the `self`, `result` and operation
  parameters of a state clause), and its `semantic_type` is the bound
  value's type. A model member (field, operation, clause) has no node of
  its own: it is named by its declaring node and its name. A recursive reference is legal only
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
  `contracts/checked-operation-catalog-v1.json` in `agent-ix/quire-verification-contracts`,
  whose entry names the
  operation's operator class, operand positions and rest, result form,
  constraints, required law roles, mode kind, member kind and leaf source.
  `laws` is the ordered list of `{role, definition}` pairs whose roles are
  exactly the entry's roles. For a value law role the `DefinitionRef` names,
  by authority and identity, one of that role's catalogued definitions (integer division truncating/floor/Euclidean,
  IEEE 754-2019 default, Unicode 17.0.0 text) and an exact member of the lock's
  `definition_selections`. For a clause operation (`quire.op.temporal.clause`,
  `quire.op.claim.clause`: role `temporal_profile`; `quire.op.protocol.control`,
  `quire.op.state.transition`: role `protocol_profile`) the `DefinitionRef` is
  a published profile definition of that role, matched by authority and
  identity, and equals the lock's `profile_selections` row of that
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
  for a `quire.op.protocol.control`, `quire.op.state.transition` or
  `quire.op.claim.clause` application the `{kind: "profile_operator", operator}`
  member naming one operator the law's profile defines (for a
  `quire.op.protocol.control` application the operators of
  [FR-464](FR-464-checked-package-protocol-node-bodies.md)'s catalog: the
  protocol clause, its declarations and its controls; or for a claim the
  temporal operator it applies), or for
  `quire.op.temporal.clause` no member: a temporal clause's body, its
  `quire.op.temporal.*` operator identities and their `temporal_interval` and
  `fairness` members are
  [FR-370](FR-370-checked-package-temporal-clause-body.md)'s,
  or for `quire.op.state.clause` the `{kind: "state_clause", clause}` member
  naming the clause kind (`invariant`, `precondition` or `postcondition`,
  [FR-341](FR-341-checked-package-state-clause-body.md)). A state clause
  carries no law.
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
  A `field`, `operation` or `relationship_end` member whose declaring node is
  a model declaration node resolves through that node's `owner` in the
  selected document ("Model-owned members" steps 2 and 3), never through the
  declaring node's body, which is empty and names no member: a `field` or
  `operation` by name among the declaring type's exposed effective members,
  and a `relationship_end` among the named ends of the relationship the owner
  names: its `sourceEnd.role` (forward) and, only when the document declares
  one, its `targetEnd.role` (inverse). A `targetEnd` with no `role` has no
  inverse end, and a `relationship_end` naming a role the relationship does not
  name, the missing inverse included, refuses `missing_declaration`/`missing-name`
  (FR-322-AC-56); no name is synthesized for an absent inverse. A reader checks
  only that a present role is a non-empty string. The wire member names the
  relationship node as its `declaration` and the end's role as its `name`; the
  optional `population` key and its checks are "Population link of a
  `reaches`" below;
  [FR-152](../../functional/type-model/FR-152-bind-systems-model-structures.md)'s
  runtime navigation `r.name` over `r: Reference<T>` is the source lookup that
  produces it. Traversing the named end obeys FR-152's direction rule, even
  when the inverse is named: the forward end (`sourceEnd.role`, source to
  target) needs the relationship's `direction` to be `source-to-target`,
  `bidirectional` or `undirected`, and the inverse end (`targetEnd.role`,
  target to source) needs `target-to-source`, `bidirectional` or `undirected`;
  any other traversal refuses `ill_typed`/`operator-ineligible`.
  An absent profile law, a law of the wrong profile kind, a profile the lock
  does not select and an operator the profile does not define refuse as
  `operation-law-missing`, `operation-law-mismatch`, `operation-law-unselected`
  and `operation-member-mismatch` respectively.
  `leaves` is empty unless the entry names a leaf source; it then lists,
  in the order of the "Structural leaf walk" below, one `{path, laws, mode}`
  entry for every text leaf of the compared type (`operand:0`, `inner:0`)
  or, for `result_inner`, of the element type of a `set`, `bag` or
  `ordered_set` result (`field:<name>`, `member:<name>`, `position:<n>` and
  `inner` segments), with the text profile law and the mode that leaf's
  type pins, and one `{path, laws: [], mode: null}` entry for every
  recursion leaf, whose path ends in a `recursion:<d>` segment. IEEE values have no
  `eq`, `ne`, ordering or `negate` operation: `quire.op.ieee.numeric_equal`,
  `quire.op.ieee.total_order` and `quire.op.ieee.bit_identical` are three
  distinct identities, and a structural comparison reaching a float leaf is
  ineligible. A V2 reader refuses the first failing check in the order owned by
  `quire.native.diagnostics/v1`: `unknown-operation`,
  `operation-class-mismatch`, `operation-law-missing`,
  `operation-law-mismatch`, `operation-law-unselected`,
  `operation-mode-mismatch` and `operation-member-mismatch` (code
  `invalid_package`); then each argument application under the same order;
  then `operation-mode-type-mismatch` (`invalid_package`) and
  `operator-ineligible` (arity, operand family, constraint, member or result
  type outside the entry; code `ill_typed`); then the leaves.
- `application_node_preimage` — every `value`/`union_value` node, and every
  other node whose body contains an application, is keyed by the JCS SHA-256
  of `{version: "quire.application-node/v1",
  node_tag, semantic_form, semantic_type, declaration, recursion, body}`.
  `declaration` is the node's `declaration` member or `null`; `recursion` is
  `null` or `{size, ordinal}` of the node inside its `recursion_group` in graph
  order, and a body reference to a member of that group becomes
  `{term: "group_reference", ordinal}`. Its operation identity, laws, mode,
  member and leaves therefore participate in its node id and, through the
  `identity_projection`, in the package id. The `union_value` selection
  applies even to a nullary construction or a payload consisting only of
  references: its body remains the aggregate
  [FR-440](FR-440-checked-package-union-and-case-nodes.md) specifies, with
  no construction application, operator or operation identity added. This
  exception does not select `record_value` or `tuple_value` nodes whose
  bodies contain no application. An `expression` whose body root is
  an application has the `semantic_form` fixed by its operator class; the
  node's `dependencies` are exactly the unique digest-ascending `reference`
  term targets, member declarations and member `population` nodes of its body;
  a `dependency_reference` contributes none. A contradictory form or
  dependency list is `invalid_semantic_graph`. A node whose retained key differs
  from the digest of its preimage refuses as `invalid_package` with cause
  `stale-node-key`, after every graph-shape refusal and before any declaration,
  frame or operation refusal, at the first such node in ascending retained
  digest order. A frame body refusal, meaning-join or canonical-order
  ([FR-340](FR-340-checked-package-frame-location.md)), is reported after
  every declaration refusal and before any state or operation refusal, at
  the first `state`/`frame` node in ascending node-id digest order that
  FR-340's own meaning-join/canonical-order precedence selects a defect for.
  A state refusal (the placement of `quire.op.state.clause`, then
  [FR-342](FR-342-checked-package-operation-anchor-body.md)'s
  `operation_anchor` joins, then
  [FR-341](FR-341-checked-package-state-clause-body.md)'s `state_clause`
  joins) is reported after every frame refusal and before any operation
  refusal, in the order FR-341's "Reader order" fixes.
- derived-shape key stage — after the application-node key refusal above and
  before any declaration, frame, state or operation refusal, the reader
  re-derives by QSL FR-092's `quire.structural-node/v1` preimage the key of
  every **structural node**: a node with no `nominal_identity_preimage` and
  whose body holds no application, model declaration nodes and clause
  functions included (QSL FR-094 keys a model-owned node by the same
  preimage), except a `value`/`union_value` node or a
  `correspondence`/`abstraction_relation` node. A `union_value` is checked
  at the application-node key stage above. An `abstraction_relation`
  node is keyed by its own `{version: "quire.abstraction-relation-node/v1",
  body}` preimage
  ([FR-451](FR-451-checked-package-abstraction-relation-body.md)), which
  FR-451's abstraction step checks after the state step; this stage neither
  re-keys nor refuses it. Only that form is excluded: a `correspondence` node of
  any other form (`source_locus`, `model_correspondence`, `binding_role`,
  `profile_correspondence`) whose body holds no application is a structural
  node and is keyed by this stage. So a consumer reading a range, bound, element type or field
  type from such a node reads the content its key names. The stage first
  joins the `owner` of every owner-bearing node, ascending by retained
  node-id digest, and refuses the first owner that fails, before either
  pass below:
  - A `SourceOwner` joins when its (`authority`, `identity`) equals a
    `lock.sources` entry's, else `missing_declaration`/`missing-selection`
    at that node's `node_id`. It is also coherent with the source map: every
    region of the node's `declaration` occurrence names a `source` whose
    (`authority`, `identity`) equals the owner's, else
    `invalid_package`/`invalid-value` at that occurrence's `source_map`
    entry, because a declared node's owner is the source unit that declares
    it.
  - A `ModelOwner` joins when its `identity` equals a `model_selections`
    row's `identity` and the admitted document of that selection ("Model-owned
    members" step 1) declares its `node` as the kind the node stands for: on
    a model declaration node, a declaration of the kind its
    `node_tag`/`semantic_form` names (step 2's table), and on a clause
    function, the kind its clause kind names. A clause function's **clause
    kind** is the text literal of the one `clause` binding of its body, the
    `DeclaredClauseKind` of QSL FR-094's clause functions: `invariant`,
    `precondition` or `body`. A postcondition is not a clause function: it is
    FR-341's `state`/`state_clause` node, an application-keyed node that
    carries no `owner`. An `invariant` clause function is owned by an object
    type with no `interfaceFeatures` (`model`/`object_type`, never a systems
    interface), by a record value type (`model`/`record_value_type`) or by an
    event type (`model`/`event_type`);
    a `precondition` or `body` clause function is owned by an operation
    member of an object type, the operation's context; a clause function with
    no, a repeated, a non-text or another `clause` binding has no owner kind it can join. Any
    other owner refuses `missing_declaration`/`missing-selection`
    at that node's `node_id`, whether or not any member, frame, anchor or
    reference reaches the node. Each document lookup charges one validation
    visit. A structural node outside every recursion group,
  declared or not and of any shape, is current when its key is the JCS
  SHA-256 of its preimage: `recursion` `null`, `semantic_type` `null`
  exactly when the node is its own type, and `owner` present exactly when
  the node carries one, equal to its `owner` member. A **model declaration
  node** is current only when it is also its own `semantic_type`, carries no
  `recursion_group` and has the body `{term: "aggregate", members: []}`: the
  closed `ModelDeclarationNode` preimage of
  `node-identity-preimage.schema.json`. A **derived shape**, an
  undeclared ungrouped node of `scalar_type` `boolean` and `integer`,
  `composite_type` `reference`, `option`, `set`, `bag`, `sequence` and
  `ordered_set`, `bounded_domain` `integer_range`, `collection_bounds` and
  `model_population`, or `scalar_type` `compound_unit`, is current only when
  it is also, for a `scalar_type` or `composite_type`, its own
  `semantic_type`. A structural node that
  carries a `recursion_group` is an **in-group structural node**. The members
  of a group are the nodes carrying its label, and a member's `ordinal` is its
  position among them in graph order. Such a node is current when its key is the JCS
  SHA-256 of its QSL FR-092 group-local preimage (`recursion`
  `{size, ordinal}`, each body `reference` to a member and a `semantic_type`
  naming a member written `{term: "group_reference", ordinal}`, `owner`
  equal to the node's `owner` member when it carries one) with
  `recursion.group` set to its retained `recursion_group`. The stage runs two
  passes. The first visits every structural node, grouped or not, ascending
  by retained node-id digest, and refuses the first node that is not
  current. The second
  recomputes each group's digest, the JCS SHA-256 of the array of the digests
  of its members' group-local preimages in ordinal order (an in-group
  application node's group-local preimage is its
  `quire.application-node/v1` preimage), and refuses a group whose label is
  not that digest at the group member with the lowest retained node-id
  digest, taking the group whose such member is lowest first. Either refusal
  is `invalid_package` with cause `stale-node-key` at that node's `node_id`.
  A tampered body or `semantic_type` of an in-group node fails the first
  pass at that node, because the other members' group-local preimages name
  it by ordinal and not by key; a changed graph order of a group's members
  fails the first pass at a member whose ordinal changed. A changed `owner`
  fails the first pass at that node, and for an in-group declared node it
  also changes the group digest, so two groups whose members differ only by
  owner have different labels and different member keys. A conformant
  package carries on every structural node the key QSL's FR-092 key function
  (FR-094's for a model-owned node) mints from the node's own members.
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
  On the wire it is the term
  `{term: "dependency_reference", package, node}`, where `package` has the
  `PackageId` shape and `node` the `NodeId` shape. It stands only as the
  callee, argument 0, of a `quire.op.function.call` application, and names
  a declaration node of the dependency, never a node of the importing
  graph. The node identity
  preimage spells it identically, so the dependency's `package_id` and the
  referenced node id both enter the referencing node's id: a new
  dependency `package_id` gives every node that references into it a new
  id. A node's `dependencies` list names nodes of its own graph only, so a
  `dependency_reference` target is never listed there; it joins through
  `dependency_selections` instead. A `node` that is not a node carrying a
  `declaration` in the named dependency's admitted package refuses
  `missing_declaration`/`missing-name`. A `dependency_reference` names a
  `function` declaration whose parameter and result types are
  package-independent: no node in the transitive closure of the signature
  type nodes' `dependencies` carries a
  `declaration` or a `ModelOwner`. Such types (builtin, bounded-domain and anonymous
  structural types over them) have the same node id in every package. The
  application's `return:0` result is that function's result type; the
  referencing node's `result_type` names that type's node, which the
  importing graph carries under the same id. A `dependency_reference` that
  names any other declaration, names a function one of whose reachable
  signature type nodes carries a `declaration` or a `ModelOwner` (a
  `Set<R>`, a tuple holding `R`, a quantity over a declared unit, or
  `Reference<M::T>` for a model type `M::T`), or
  stands anywhere other than a `quire.op.function.call` callee refuses
  `ill_typed`/`operator-ineligible`. A reader checks each
  `dependency_reference` in step 7 of the first-refusal order, at its place
  in its node's pre-order term walk wherever it stands (an application
  argument, a binding value, an aggregate member or the body root):
  `missing-selection`, then `missing-name`, then `operator-ineligible`.
- `model_member` — a `field` or `operation` member whose declaring node is a
  model declaration node: a `model` or `relation` node that carries no
  `declaration` and carries its `ModelOwner` (QSL FR-094) as `owner`. Its
  body is `aggregate{[]}` and names
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
- `owner` — the owner a node's identity preimage hashes, carried on the node
  so that a consumer re-derives every structural key, in-group keys
  included, from the package alone. An **owner-bearing node** is a
  structural node (no `nominal_identity_preimage`, and a body whose root is
  not an `application`) that either carries a `declaration` or is a
  **model-owned node** (QSL FR-094) carrying none: a model declaration node
  (`model`/`object_type`, `model`/`record_value_type`, `model`/`event_type`,
  `model`/`variant_type`, `model`/`state_machine`, `model`/`process`,
  `model`/`persistence_interface`, `model`/`systems_interface` or
  `relation`/`relationship`) or a clause function (`function`). A declared
  owner-bearing node carries the `SourceOwner` of the source unit that
  declares it, `{kind: "source", authority, identity}`; a model-owned node
  carries its `ModelOwner`, `{kind: "model", identity, node}`, with no
  `version` member (FR-322-AC-28). The value is exactly the `owner` member of
  the node's `quire.structural-node/v1` preimage, so the reader copies it
  into that preimage unchanged. A projection entry whose `owner` differs
  from its node's, or is present where the node's is absent, refuses
  `invalid_package`/`invalid-value` at that entry's `owner`,
  `/identity_preimage/identity_projection/{i}/owner`, before the owner join.
  Every other node carries no `owner`: a
  builtin or anonymous type, an undeclared value, parameter or expression,
  any other undeclared node, a nominal node (its owner is its
  `nominal_identity_preimage`'s) and a node keyed by
  `quire.application-node/v1`, a `state`/`state_clause` node included (its body root is
  an application, FR-341). That preimage has no owner by design: an
  application node is content-addressed, so the same value under two owners
  is one node, and a re-derivation recomputes it identically from its
  content, so no owner can be forged into or out of its key. A
  `DefinitionOwner` is never a node `owner`, because a definition-owned
  declaration is a nominal node. `owner` is a member of the node's
  `identity_projection` entry, so it enters `package_id`. A reader reads a
  node's owner only from this member: never from the source map, a source
  occurrence, the lock or the node's position. The V2 schema states the
  rule ("Owner member schema" below), and a node that breaks it refuses
  `malformed_wire` at the node.
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
  semantic fields. An `enum_value` node whose `semantic_type` is a
  `model`/`variant_type` node joins the same way, and its `case` SHALL be a case
  of the variant type that the `ModelOwner` selection of its `semantic_type` node declares (the cases of
  `quire.meaning.model.variant-type/v1` in the admitted document, FR-208), else
  the reader refuses `invalid_semantic_graph` at the `enum_value` node, as it
  does for a scalar enum whose case is not among its declaration's members.
  The `case` of such a node and of its member preimage is the declared case
  name, any non-empty string, matched byte for byte as UTF-8; the identifier
  requirement of the member preimage holds for a `scalar_type`/`enum` member
  only (FR-141).
- `source_map` — the total semantic-occurrence to original source-region
  mapping. An occurrence key is (`node_id`, `role`, `ordinal`), where role is
  `declaration`, `type`, `expression`, `anchor`, `claim` or `generated`; a
  source region is a half-open UTF-8 byte interval `[start,end)` under one exact
  source-document identity/raw-byte digest. An occurrence may name
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
`invalid_model_binding`, `missing_import`, `unknown_profile`,
`unsupported_construct` and `wrong_snapshot`. A V2 reader SHALL pair the cause
`unsupported-wire` only with `unknown_wire` and `unsupported-feature` only
with `unknown_required_feature`, as FR-272 and the `Diagnostic` schema's
`DiagnosticCausePairing` do; `unknown_contract_version` and
`unknown_required_capability` carry no cause (FR-272); the ContractPackage
reader refuses a nested refusal record that pairs either with a cause, as
FR-330 states (FR-322-AC-78 covers the pairing). The `stale-node-key`, operation and declaration causes
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
steps 3 and 4 report `ambiguous_declaration`/`ambiguous-name` and
`ill_typed`/`operator-ineligible` (a model declaration node that is not its
own preimage, or whose owner names no selected declaration of its kind,
was refused earlier by the derived-shape key stage, and step 2 refuses
nothing).
The remaining (code, cause) pairs a V2 reader reports are each the pairing
[FR-272](../foundation/FR-272-typed-diagnostic-cause.md) lists, and the V2
`Diagnostic` schema admits each cause only with its code:
`ill_typed`/`type-mismatch` (FR-440's union value join);
`missing_import`/`missing-selection` and `missing_declaration`/`missing-selection`;
and `unknown_profile`/`unsupported-selection` and
`unknown_profile`/`wrong-selection-role`
([FR-370](FR-370-checked-package-temporal-clause-body.md)'s profile check),
the schema admitting those two causes with any of FR-272's
`unknown_language`, `unknown_edition` and `unknown_profile`.
Before expensive semantic work, the reader measures raw wire bytes; a byte
overage returns `incomplete`. It then performs strict syntax/member validation,
including the body grammar below; a malformed wire returns `refused`. It
charges graph nodes/edges, occurrences/loci, diagnostics and validation visits
as they are traversed; the first exhausted limit returns `incomplete` with its
actual counter, its configured bound and its setting name
([FR-461](../../functional/foundation/FR-461-name-the-limit-value-and-setting.md)),
and no admitted package. Equal-limit input is admitted if all later checks
succeed. The reader has no depth limit: the body grammar fixes the JSON depth
of every package
([FR-460](../../functional/foundation/FR-460-bound-every-stage-by-resource-limits.md)).

## Body grammar

A node's `body` is drawn from a closed grammar of five strata. No production
names itself or a higher stratum, so the JSON depth of every V2 package is
fixed by the schema, whatever the model's size.

| Stratum | Productions |
| --- | --- |
| Leaf | `literal`, `reference`, `dependency_reference` |
| Group | `aggregate` whose members are each a Leaf or a `binding` whose value is a Leaf |
| Tuple | `aggregate` whose members are each a Leaf or a Group |
| Member | a Leaf, a Group, or a `binding` whose value is a Leaf, a Group or a Tuple |
| Body | a Leaf; an `application` whose `arguments` are each a Member; an `aggregate` whose members are each a Member; a `frame` body ([FR-340](FR-340-checked-package-frame-location.md)); or an `abstraction_relation` body ([FR-451](FR-451-checked-package-abstraction-relation-body.md)) |

Every composite subterm is its own node, reached by `reference`: an
application never stands inside another application's arguments, an
aggregate's members or a binding's value. The `state_clause`
([FR-341](FR-341-checked-package-state-clause-body.md)) and `operation_anchor`
([FR-342](FR-342-checked-package-operation-anchor-body.md)) bodies are Body
productions. A diagnostic's `details` are each a Member; a `details` term
may reference a `composite_type`/`union` or `value`/`union_value` node, and
no `details` term references a `temporal`/`formula`, `temporal`/`fairness`
or `expression`/`case` node
([FR-370](FR-370-checked-package-temporal-clause-body.md) "Placement and
profile fit"). The node-identity
preimage of a body follows the same grammar, with `group_reference` as a
further Leaf, so every body preimage has a schema-fixed depth. A reader
refuses a body outside the grammar as `malformed_wire`, never as a limit,
with two exceptions, each refused `ill_typed`/`operator-ineligible` at the
misplaced application's `operator` member:

- an application of the `temporal_formula`, `temporal_fairness` or `case`
  operator class inside a diagnostic's `details` term, located at
  `/diagnostics/entries/{e}/details/{d}/operator` when the term is that
  application ([FR-370](FR-370-checked-package-temporal-clause-body.md));
- a `quire.op.control.case` application nested inside another term
  ([FR-440](FR-440-checked-package-union-and-case-nodes.md) "Case
  placement").

An application of any operator class other than `case` nested inside a
node body or inside another term is no exception: it is refused
`malformed_wire` at the nested application. This holds for every such
class, including a
`temporal_formula`, `temporal_fairness` or `temporal` application
([FR-370](FR-370-checked-package-temporal-clause-body.md) "Placement and
profile fit") and a `quire.op.state.clause` application
([FR-341](FR-341-checked-package-state-clause-body.md)). Such nesting is
expressed through references to the nodes whose body roots hold those
applications. Only an application at the body root of a node its class
does not place it in refuses `ill_typed`/`operator-ineligible` at that
node. When one body or one `details` term holds several offending
constructs, the reader reports the first one in document pre-order,
outermost first. A `details` term that is a `temporal_formula` application
with a nested `case` argument refuses at
`/diagnostics/entries/{e}/details/{d}/operator`, and a `details` aggregate
whose members are a `temporal_formula` application and then a `case`
application refuses at
`/diagnostics/entries/{e}/details/{d}/members/0/operator`.

A producer whose expression has a deeper shape emits each composite subterm
as its own node, so the grammar holds for an expression of any depth.

## Owner member schema

`schema.json` gives `NodeBase` and `NodeProjection` the optional property
`owner`, `{"$ref": "#/$defs/NodeOwner"}`, where `NodeOwner` is
`{"oneOf": [SourceOwner, ModelOwner]}` over the existing closed
`SourceOwner` and `ModelOwner` definitions. Both node definitions carry one
further rule, `OwnerPresenceRule`, in their `allOf`. A node is
**structural** for this rule when it carries no `nominal_identity_preimage`
and its `body.term` is not `application` (an application stands only at a
body root, so this is "its body holds no application"). This includes a
`correspondence`/`abstraction_relation` node, which the derived-shape key
stage excludes, but the rule never asks it for an `owner`: `DeclarationTagRules`
forbids `declaration` on every `correspondence` node and the form is in
neither model-owned row, so it falls in the last row and carries none.

| Node | `owner` |
| --- | --- |
| structural, carries `declaration` | required, `SourceOwner` |
| structural, no `declaration`, `node_tag`/`semantic_form` `model`/`object_type`, `model`/`record_value_type`, `model`/`event_type`, `model`/`variant_type`, `model`/`state_machine`, `model`/`process`, `model`/`persistence_interface`, `model`/`systems_interface` or `relation`/`relationship` | required, `ModelOwner` |
| structural, no `declaration`, `node_tag` `function` | required, `ModelOwner` |
| every other node | absent |

A node that breaks the rule, or whose `owner` is a `DefinitionOwner`, has a
member outside its owner's closed definition or a `ModelOwner` carrying
`version`, fails the schema, and a reader refuses it `malformed_wire` at
`/semantic_graph/nodes/{i}/owner`, or at `/semantic_graph/nodes/{i}` when a
required `owner` is absent; the same holds for its `identity_projection`
entry. An identity projection entry carries exactly its node's `owner`; a
well-shaped entry whose `owner` differs from its node's refuses
`invalid_package`/`invalid-value` at `/identity_preimage/identity_projection/{i}/owner`
(the schema cannot state the equality), reported before the owner join
(FR-322-AC-51).

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
   `stale_dependency`/`content-mismatch` with the selected and the recomputed
   digest; and the document's own identity equals the row's, else
   `invalid_model_binding`/`wrong-model-selection`. The reader then reads the
   document's declarations under FR-154 and normalizes them under the
   `quire.model.complete/v1` rules. The selected document is an FCD semantic IR
   2.0.0 domain package (FR-154 step 3; filament-core-data FR-091 and FR-095):
   `contractVersion`, `package`, `constructs` and `types`, each type carrying its
   `fields`, `operations`, `supertypes` and `relationships`. A relationship's IR
   node identity is FCD's, `ix://<org>/<name>/relationship/<Name>-<verb>-<Target>`,
   and it is declared in its source type's `relationships` with `sourceEnd` and
   `targetEnd`. The declarations steps 2 to 4 read are the projection of that
   document, a derived view that holds no member the document does not: the
   published `domain-package-acme-orders.json` is the document and `domain_package`
   in `model-member-type-vectors.json` is its projection. The read's refusal is the first refusal
   in FR-154's report order, with its own code, cause and loci, at stage
   `package_read`. The caller supplies `ModelNormalizationLimitsV1` among the
   reader's limits, and an exhausted one is the read's `incomplete` result.
2. **Declaration lookup.** A declaring node carries its `ModelOwner` as
   `owner`, and the derived-shape key stage has already joined that owner
   to a `model_selections` row and found the node current: its key is the
   JCS SHA-256 of the closed `ModelDeclarationNode` preimage of
   `node-identity-preimage.schema.json`, `{version:
   "quire.structural-node/v1", node_tag, semantic_form, semantic_type: null,
   declaration: null, recursion: null, owner: {kind: "model", identity,
   node}, body: {term: "aggregate", members: []}}`, built from the node's own
   `owner`. The key depends only on content: the domain package's declared
   version label is not part of it. The declaration `T` is the one that the
   admitted document of the selection whose `identity` equals the owner's
   `identity` declares under the IR node identity equal to the owner's
   `node`, and of the kind the node's `node_tag`/`semantic_form` names: an
   object type with no `interfaceFeatures` for `model`/`object_type`, a
   record value type of FR-208's `quire.meaning.model.record-value-type/v1`
   for `model`/`record_value_type`, an event type of FR-208's
   `quire.meaning.model.event-type/v1` for `model`/`event_type`, a variant type of
   FR-208's `quire.meaning.model.variant-type/v1` for `model`/`variant_type`, a
   state machine of FR-208's `quire.meaning.model.state-machine/v1` for
   `model`/`state_machine`, a process of FR-208's
   `quire.meaning.model.process/v1` for `model`/`process`, a persistence
   interface of FR-208's `quire.meaning.model.persistence-interface/v1` for
   `model`/`persistence_interface`, an object type with `interfaceFeatures` for
   `model`/`systems_interface` and a relationship for `relation`/`relationship`.
   A process and a state machine declare fields, supertypes of their own meaning
   and operations, so step 3 resolves their members as it does an object
   type's. FR-321 admits one selection
   per `identity` and a document declares each IR node identity once, so at
   most one declaration matches. The derived-shape key stage's owner join
   has already found `T` for every model declaration node, so step 2 has no
   refusal of its own: a declaring node whose owner names no declaration of
   its kind was refused `missing_declaration`/`missing-selection` there. A node that carries a
   `declaration` carries a `SourceOwner` (the V2 schema refuses a
   `ModelOwner` on it), so it is not a model declaration node and has no
   `T`.
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
   | `ix://quire/native/String` | `String`, a native with no text profile; it is not `Text` and has no `Text` bounds, profile or leaf | `scalar_type`/`string` |
   | `ix://quire/native/UUID` | `UUID`, compared by its canonical payload under FR-149's UUID row; no ordering | `scalar_type`/`uuid` |
   | `ix://quire/native/Timestamp` | `Timestamp`, compared by its canonical payload under FR-149's Timestamp row; no ordering and no arithmetic | `scalar_type`/`timestamp` |
   | a value type (FR-208 `quire.meaning.model.value-type/v1`) of the same domain package bound as `Int[lo, hi]`, `lo <= hi`, as TC-196's `Count` and `Small` and TC-197's `Count` are | `Int[lo, hi]` | `bounded_domain`/`integer_range` with bounds `lo` and `hi` over the `Integer` node |
   | a value type of the same domain package bound as `Text[min, max; profile]`, `0 <= min <= max`, with `profile` one of FR-141's `unicode-scalars`, `nfc`, `nfd`, `nfkc`, `nfkd` and `binary-utf8` | `Text[min, max; profile]`, whose length, bounds and comparison are FR-141's under `profile` | `bounded_domain`/`text_bounds` with bounds `min` and `max` and `text_profile` `profile` over the `scalar_type`/`text` node |
   | a record value type `R` of the same domain package, admitted under FR-208 | `R`, a value with no object identity | the self-typed `model`/`record_value_type` declaration node owned by `R`; no `composite_type`/`reference` is introduced |
   | a variant type `V` of the same domain package, admitted under FR-208 | `V`, an unordered enumeration whose cases are the case names of `V` | the self-typed `model`/`variant_type` declaration node owned by `V`; no `composite_type`/`reference` and no unit-owned `scalar_type`/`enum` node is introduced |
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

   Each node the tables name has no `declaration`; the record value type is
   model-owned, while the other derived types are anonymous structural nodes,
   exactly as QSL FR-092 and FR-094 key them. The node that
   stands for a type, and every application node id that references it, is
   canonical.

### Derived type forms and forwarding

The following forms are part of the closed V2 semantic vocabulary. They are
anonymous structural nodes: each has `declaration: null`, no `owner`, no
`nominal_identity_preimage` and no `recursion_group`. A source type alias
introduces no node. After resolving an alias, every type annotation and every
application result type names the resolved type's node, so an alias cannot
create a second `composite_type`, `bounded_domain` or `scalar_type` identity. A
`Quantity<u>` annotation likewise introduces no node: it names the
`scalar_type`/`unit` node of `u`, the type of a quantity in that unit (family
`quantity`), under the same id in every package that holds that unit (FR-142).

| Source type | Node and semantic type | Body | Worked accepted vector |
| --- | --- | --- | --- |
| `Reference<O>` | `composite_type`/`reference`, self-typed | `aggregate{[reference(M_O)]}`, where `M_O` is `O`'s model declaration node. A derived reference target SHALL be a `model`/`object_type`, `model`/`process`, `model`/`state_machine`, `model`/`systems_interface` or `relation`/`relationship` declaration; scalar, value, function and other composite nodes are refused. | `Ref = Reference<Order>` and a parameter typed `Ref` both name R1 (`249d3baf1f4728325f4b5194d54320ad6d7e11bc88de7a339504ea6582fe1fbb`); R1's only body target is M1 (`6f006fa31e8ceeb4c7e6ffb71a68a7883643c931f90d487d35239d49390c596b`) |
| `Population<O>[N]`; `Population<O>` with no maximum | `bounded_domain`/`model_population`, semantic type `S_O`, where `S_O` is `composite_type`/`set` over `R_O` | `aggregate{[binding "max" = literal integer N]}`; for `Population<O>` with no maximum, `aggregate{[]}`, the `max` member absent | `Bucket = Population<Order>[3]` names PO1 (`859d6b01a132a14711a8a4568e24a7ab54e3255b78bbcc62b14e4293dbf71f9a`); PO1's `semantic_type` is S1 (`b8a0b4c1fcbcb8672f21a16c2373abed9ba6469cc9fff3288408349f288dbed5`); changing `Bucket` to `Population<Order>[5]` names a different key; `Open = Population<Order>` names PO0 (`f65a8239f4fbc1b6e75bfeed4a578e1c219a27f155a82ddfb7aeeb22657c3b8b`), whose `semantic_type` is also S1 |
| a formed quantity over a compound unit | `scalar_type`/`compound_unit`, self-typed | `aggregate` of one `aggregate{[binding "unit" = reference(U), binding "exponent" = literal integer e]}` per root unit, in `quire.value.compound-unit/v1` order; dimensionless is `aggregate{[]}`. Each `U` is a targetless canonical nominal root with `nominal_identity_preimage.version = quire.unit-node/v1`, empty body and `semantic_type` equal to its dimension node. | `metre / second` names U2 (`bcf431d15781d66fcdd2514e251004373eb64a3ef21e37f7b2535f2a871821e7`); its first term references `U_metre` with exponent `1`, its second references `U_second` with exponent `-1` |

The accepted vectors above are compact presentations of the exact RFC 8785
preimages. The full preimage bodies are the rows above, and the named keys are
the corresponding settled structural-node vectors. They are deliberately
independent of a package version and of a source alias spelling.

A self-typed structural `scalar_type`/`unit` is not an alternative root: the
published V2 schema requires `nominal_identity_preimage` for this node form,
with a `UnitPreimage`, and therefore does not admit a structural unit node.

The corresponding checks are closed, with the stage that owns each check made
explicit. A source alias that names no type refuses
`missing_declaration`/`missing-name` before lowering. The derived-shape key
stage checks the retained key against the complete body. Its named refusal
vectors are:

| Mutation vector | Mutation with the original key retained | Recomputed-key case |
| --- | --- | --- |
| `RM-REF-TARGET` | Change R1's `reference(M1)` target to a scalar node; the schema remains valid, but the reader refuses `invalid_package`/`invalid-derived-shape` at R1. | Replace R1's key with the digest of the changed preimage before reading; the reader still refuses the scalar target. |
| `RM-POP-MAX` | Change PO1's integer `max` to negative `-1`; the schema remains valid, but the reader refuses `invalid_package`/`invalid-derived-shape` at PO1. | Replace PO1's key with the digest of the changed preimage before reading; the reader still refuses the negative bound. |
| `RM-CU-ORDER` | Swap U2's two unit terms while retaining U2's key; the schema remains valid, but the reader refuses `invalid_package`/`invalid-derived-shape` at U2. | Replace U2's key with the digest of the changed preimage before reading; the reader still refuses noncanonical term order. |

The executable reader matrix covers the accepted and refused cells below:

| Derived shape | Accepted form | Refusal mutations exercised after key recomputation |
| --- | --- | --- |
| `composite_type`/`reference` | body target is a model declaration node | scalar, function and value targets |
| `bounded_domain`/`model_population` | `semantic_type` is `set<reference<T>>`; either one canonical nonnegative integer `max` binding (a bounded population) or no member (the unbounded population) | negative `max`; non-integer `max`; a member beside `max`; arbitrary set element |
| `scalar_type`/`compound_unit` | self-typed node; targetless canonical nominal unit roots whose semantic type is the dimension key; nonzero exponents in strict digest order | non-self-typed node; non-unit root; reordered terms |

The required-`max` rule applies to a bounded population only. A
`model_population` node whose `max` member is absent is the unbounded
population, the node a source `Population<O>` with no maximum derives. It is
admitted, with the same `semantic_type` rule and the empty `aggregate` body, and
its key is a function of that body, so it differs from the key of every bounded
`Population<O>[N]` and from the key of the unbounded `Set<Reference<O>>`
(`composite_type`/`set` alone, with no `collection_bounds`). Its worked accepted
vector is PO0 (`f65a8239f4fbc1b6e75bfeed4a578e1c219a27f155a82ddfb7aeeb22657c3b8b`):
the preimage of PO1 with the `max` member removed, so the same S1 as
`semantic_type` and the body `{"members":[],"term":"aggregate"}`. No `max` is
invented for the unbounded node: the absent `max` is not `0` (its key differs
from the key of the node with `max` `0`) and is not any other bound, and a `max` that is present is a bounded population read
as before. A `population` link of a `quire.op.model.reaches` application that
names an unbounded population node is admitted and states no bound for the
application.

These vectors distinguish a retained-key mutation from a recomputed-key
candidate. The reader checks both the key and the semantic shape: a
`composite_type`/`reference` whose body target is not a model declaration
node, a `bounded_domain`/`model_population` whose `semantic_type` is not the
`set<reference<T>>` node or whose `max` is negative or not an integer
literal, or which has any member other than a single `max`, and a
`scalar_type`/`compound_unit` whose root is not a
targetless canonical nominal unit, whose exponent is zero, whose terms are
duplicated or not in canonical order, or whose body is not the stated
aggregate is refused by the semantic shape check. Adding a declaration, owner
or nominal preimage to any of these
anonymous forms fails the V2 schema or the owner/declaration rules before key
recomputation. An alias that resolves to a valid form has no separate refusal
or identity: its resolved form is checked exactly once.

For a model-owned member, the `member` result form yields the step 4 type,
and the application's `result_type` is admitted exactly when it names the
node of that type. `member_of` holds exactly when operand 0 resolves to the
declaring node, or to a record type node that conforms to it ("Record
conformance"), and step 3 resolves the name to a field member of `T`. For a
`dispatch_call`, operand 0 has type `Reference<X>`, where `X`'s model
declaration node is the declaring node; the arguments after the receiver
equal the resolved operation's parameters in number; and the type node of
each argument is the node of its parameter's type, or, for a record-typed
parameter, a record type that conforms to it ("Record conformance"). Any other
receiver, argument count or argument type refuses
`ill_typed`/`operator-ineligible`.
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
reader looks up the declaration of each operand's model declaration node by
step 2, with step 2's refusal, including when both operands resolve to one
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

### Record conformance

A record value type and an event type are `model` declaration nodes of family
`record` ("Operation families, groups and constraints"). A value of a record
subtype is a value of each ancestor
([FR-208](../foundation/FR-208-quire-meaning-vocabulary.md)), and it stays the
subtype's own value: the lowered form carries it by `ref`, with no conversion
node between the value and the position that takes it. The reader decides
whether a record type node `A` **conforms** to a record type node `P` exactly
when `A` and `P` are one node, or both are `model` declaration nodes of one
form (`record_value_type` or `event_type`) and the declaration step 2 finds for
`P` is among the declared supertypes, at any depth, of the declaration it finds
for `A` in that document's `quire.model.complete/v1` normalization. Each lookup
carries step 2's refusal. Conformance is not symmetric: an ancestor does not
conform to a descendant, a record of no supertype chain with `P` does not
conform to `P`, and declarations of two documents never conform. It applies
only to a record type node itself. A `K<A>` or `Option<A>` type node does not
conform to `K<P>` or `Option<P>`, because a collection's or option's type
identity includes its element type node: a child **value** is admitted at a
`P`-typed slot, never a `K<child>` type at a `K<P>` type.

Conformance decides these positions:

- **Calls.** The arguments of a `quire.op.function.call` application after
  its callee equal the callee's parameters in number, and the arguments of an
  operation call (`dispatch_call`) after its receiver equal the resolved
  operation's parameters in number; the count is checked first. The parameters
  of a `dispatch_call` come from the selected document (step 4). The parameters
  of a `function.call` come from the callee `function` node's body: its first
  binding, named `parameters`, is an `aggregate` of `reference` terms to the
  `value`/`parameter` nodes in parameter order, each typed by its
  `semantic_type`, including a parameter the body never reads (QSL FR-092
  "Function nodes"). A callee whose body has no `parameters` binding takes no
  parameter, and a `dependency_reference` callee uses the same binding of the
  named dependency node. That binding is the one source of a function's
  declared parameter types; no other member carries them. Each argument's type
  node equals the type node of the parameter at its position, or, when that
  parameter's type is a record type node `P`, is a record type node that
  conforms to `P`. The catalog entry of `quire.op.function.call` carries no
  constraint for this, so the reader checks it at the application's
  `operator-ineligible` check.
- **Projection.** `quire.op.record.project` over an operand of record type node
  `A` carries a `field` member whose `declaration` names a record type node
  `D` that `A` conforms to (`member_of`), so a field `D` declares or inherits is
  read from a value of `A` or of any descendant of `D`. Step 3 resolves `name`
  on `D`, and the application's `result_type` is the step 4 type that `D` gives
  the field, whichever descendant redefines the field with a narrower type. A
  `declaration` naming a node that `A` does not conform to refuses.
- **Slots.** Wherever a rule of this artifact or of
  [FR-440](FR-440-checked-package-union-and-case-nodes.md) compares a term's
  type node with the type node of a slot, a record type node that conforms to
  the slot's record type node satisfies it, and any other type node must equal
  it as before. The slots are the element type node of a collection (the
  `inner_type` constraint of `quire.op.collection.contains`, the rest arguments
  of `quire.op.collection.sequence`, `set`, `bag` and `ordered_set` against the
  element type node of the application's `result_type`), the payload type node
  of an `Option`, a tuple position, a union member payload position (FR-440
  "Union value") and a record field. The family checks (`uniform_rest`, the
  operand groups) are unchanged: a child and its ancestor both have family
  `record`.
- **Results.** A result position compares the declared result of a node with
  the type node of the term that produces it. A `function` node's declared
  result is its `semantic_type`, and its body is an `aggregate` whose `body`
  binding is a `reference` to the node of its root expression (QSL FR-092
  "Function nodes"); the reader compares that referenced node's
  `semantic_type` with the function's. They are equal, or, when the function's
  `semantic_type` is a record type node `R`, the root's is a record type node
  that conforms to `R`; any other root type node, such as `Voucher` under a
  `Money` function, refuses `ill_typed`/`type-mismatch` at the function
  node. A function node with no `body` binding (a clause function) is outside
  this comparison. An `expression` node whose body root is an application
  compares its `semantic_type` with the application's `result_type` the same
  way, with the same refusal at the expression node. The `arm_body` result of a `case` application is the case result type,
  or any arm-body type node that conforms to it when that type is a record type
  node. So `f(): Money { make_taxed() }` and a `case` whose arms yield `Money`
  and `TaxedMoney` are admitted. The `return:0` result form is unchanged: a
  call is typed at the callee's declared result, whatever descendant value it
  returns.
- **Equality.** `quire.op.structural.eq` and `quire.op.structural.ne` take two
  operands of one type node (`same_type`), and a record, or a record inside an
  option, tuple, union or collection of that type, may hold a subtype value;
  the operands are then equal exactly as FR-149 and FR-143 state, and two
  values of different declarations are unequal, a Boolean. The application's
  leaves are those of operand 0's type ("Structural leaf walk"). Two values of
  one declaration compare every effective field of that declaration, so a text
  field that only a descendant declares has no leaf in the application: it is
  compared under the text profile its own declared type gives it (step 4), with
  the package's selected text law, and the leaf check reads operand 0's walk
  only. An application that falls under the event equality exception of
  "Operation families, groups and constraints" refuses.

A call argument, a projection declaration, a slot term or an equality operand
that fails its rule refuses `ill_typed`/`operator-ineligible` at the
application (a union value node: `ill_typed`/`type-mismatch`, FR-440), the
cause of every catalog operand check; QSL FR-359 reports the same case, found
while checking source, as `ill_typed`/`type-mismatch`.

### Reaches over a field

`quire.op.model.reaches_field` is `reaches(a, b, edge)` over a field edge:
operator class `reaches`, operands `reference` and `reference`, result
`boolean`, no laws and no mode, member `field`
(`{kind: "field", declaration, name}`, the same member a field projection
carries) and the `reference_edge` constraint over operands 0 and 1.
`quire.op.model.reaches` is the same predicate over a relationship end, with
operator class `reaches`, operands `reference` and `reference`, result
`boolean`, no laws and no mode, and member `relationship_end`. Its result is
`boolean` whatever the destination end's multiplicity: the destination end
may be single-valued (`[0,1]`, `[1,1]`) or finite multi-valued (any other
finite `[l,u]`, unique or not), and the node is the same shape in every case
(FR-322-AC-57; traversal and charges follow FR-152 "Reachability over a
relationship end").
`quire.op.model.navigate` has operator class `query`, one operand of type
`Reference<T>`, member `relationship_end` and a `result_type` of the
destination end's canonical type (FR-152 "Navigation"). At
the application's `operator-ineligible` check the constraint holds exactly
when, in this order:

1. operand 0 has type `Reference<D>`, where `D`'s model declaration node is
   the member's `declaration`;
2. the member resolves on `D` by steps 2 and 3 of "Model-owned members", to
   a field member owned by the object type declaration `T` (`D` itself or
   one of its supertypes), with those steps' refusals;
3. the field's step 4 type is `Reference<T>`, `Option<Reference<T>>` or
   `Sequence<Reference<T>>[l, u]` (the `bounded_domain`/`collection_bounds`
   node over a `composite_type`/`sequence` node of `Reference<T>`): the
   state contract's `Ref(T)`, `Option(Ref(T))` and `Seq(Ref(T), N)`; and
4. operand 1 has type `Reference<B>` whose object type `B` conforms to `T`
   ("Reference conformance"; step 2 looks up `B`'s declaration with its
   refusal).

Any other operand, and a field of any other type (a set, bag, ordered set,
unbounded sequence, a reference to another object type, or a non-reference
type), refuses `ill_typed`/`operator-ineligible`. Traversal follows
[FR-043](../../functional/foundation/FR-043-evaluate-finite-graph-relations.md).

### Population link of a `reaches`

The `relationship_end` member of a `quire.op.model.reaches` application MAY
carry one further key, `population`, the `NodeId` of a population node. The
key is absent when the application names no population, never `null`, so an
application without it keeps the member, the preimage and the node id it had
before the key existed; the key is part of the application's identity preimage
when present. No other member kind carries `population`, and neither does
`quire.op.model.navigate`: a `population` on a `navigate` application refuses
`ill_typed`/`operator-ineligible` at `member.population`, and on any other
member kind the schema refuses it.

The `population` node SHALL resolve in the package to a `bounded_domain`/
`model_population` node (the `Population<O>[N]` row of "Derived type forms and
forwarding"), whose `semantic_type` is a `Set<Reference<T>>` node over a
`Reference<T>` node over a model declaration node `T`, and `T` SHALL be the
declaration of the type of the end the member's role names (the receiver's end
type `E` in the sense of "Reaches over a field"). The check follows the
destination-type check of FR-322-AC-57 in that order, and refuses at
`member.population`: a `population` that does not resolve, that is not a
`model_population` node, or whose `T` is not `E` refuses
`ill_typed`/`operator-ineligible`. The node's `max` is read as for any
`model_population` node and a negative `max` refuses as it does there; `max` `0`
is admitted, and an absent `max` is the unbounded population (FR-322-AC-74),
which is admitted and states no bound. The application's `dependencies` contain
the `population` node, so a list that omits it is `invalid_semantic_graph` at `/dependencies`. The reader
does not choose an occurrence of the population node: a reader that selects the
population by occurrence takes the `Type` occurrence with the least ordinal.

## Structural leaf walk

The leaf walk of a compared type starts at that type with the empty path
and visits types in declaration pre-order:

- A text type is a **leaf**: it lists one entry at the current path.
- A record, a tuple and each member of a union are **anchors**. A record
  visits each field's type in field declaration order under `field:<name>`.
  A tuple visits each position's type under `position:<i>`. A union visits
  its members in member declaration order, and each member's ordered
  payload types under the two segments `member:<Name>` then `position:<i>`,
  where `<Name>` is the member identifier and `i` is the index in that
  member's payload, even for a member with a single payload position. A
  member with no payload contributes no leaf.
- An `option`, `sequence`, `set`, `bag` or `ordered_set` visits its element
  type under `inner`.
- Every other type contributes no leaf; a float type makes the comparison
  ineligible, as stated above.

The walk SHALL visit every declared union member independently of the
active member of any operand value. `member:<identifier>` contains the
exact declared identifier from FR-440's union binding, followed by a
separate `position:<n>` segment whose index starts at zero and uses
canonical decimal spelling. No member sort, name normalization, union
node key, member ordinal or active-value tag enters the path. The path
names the payload relative to its enclosing union; the application's
operand and result types bind that union's identity. Distinct union types
may therefore have equal leaf lists without becoming the same type.

For the catalog's `operand:0`, `inner:0` and `result_inner` leaf sources,
the selected type starts at the empty path. Selecting a collection's
element through `inner:0` or `result_inner` adds no `inner` segment;
walking the collection type through `operand:0` does. `result_inner`
walks only a `set`, `bag` or `ordered_set` result's element type and yields
no leaves for a `sequence` result. Every text leaf carries its declared
profile and selected text definition, including a leaf beneath a member
that no operand value selects.

The walk keeps each record, tuple and union type node it is walking on the
current path as **open**, with `d`, the number of segments the path had
when the walk entered it. A record, tuple or union type node the walk
reaches while that same node is open is a **recursion leaf**: a cycle
through a `recursion_group`, which
[FR-143](../../functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md)
admits when every recursion edge crosses a record field or a union member
payload position. The walk does not descend into it. When a text type is
reachable from that node, the walk lists the entry
`{path: p + ["recursion:<d>"], laws: [], mode: null}` at the current path
`p`, with `d` written as a canonical decimal string; when no text type is
reachable from it, the walk lists nothing for it. A text type is reachable
from a node when some finite sequence of field, position, member payload and
element steps, without regard to open nodes, leads from it to a text type.
This is the recursion leaf of
QSL FR-093 "Text leaves" rule 3, extended to unions. A recursion leaf
stands for the leaves of the type entered at the first `d` segments of its
path, re-rooted at its path. The
application is admitted: a recursion leaf, through a record, a tuple or a
union, is never `operator-ineligible`. A type reached twice along different
paths, with no cycle, is walked at each.

For `union Label { Named(Text[0,8;nfc]), Tagged(Integer, Text[0,8;nfc]), Empty }`
the text leaves are `["member:Named", "position:0"]` and
`["member:Tagged", "position:1"]`, and `Empty` has none. For
`union List { Cons(Integer, List), Nil }` the walk reaches `List` again at
`["member:Cons", "position:1"]`; no text type is reachable from `List`, so
`quire.op.structural.eq` over two `List` values is admitted with empty
`leaves`.

Each of these types reaches text, so its cycle lists a recursion leaf:

- record: for `record Node { label: Text[0,8;nfc]; next: Option<Node>; }`
  the leaves are `["field:label"]` with the `nfc` law and mode, then
  `["field:next", "inner", "recursion:0"]`;
- tuple: for `record Cell { item: (Text[0,8;nfc], Option<Cell>); }` the
  leaves are `["field:item", "position:0"]` with the `nfc` law and mode,
  then `["field:item", "position:1", "inner", "recursion:0"]`;
- union: for `union Chain { Link(Text[0,8;nfc], Chain), End }` the leaves
  are `["member:Link", "position:0"]` with the `nfc` law and mode, then
  `["member:Link", "position:1", "recursion:0"]`; over `Option<Chain>` they
  are `["inner", "member:Link", "position:0"]` and
  `["inner", "member:Link", "position:1", "recursion:1"]`.

Each recursion leaf carries `laws` `[]` and `mode` `null`.

### Union text-leaf vectors

These vectors settle the same U1–U7 leaf lists as
[QSL FR-093](ix://agent-ix/quire-spec-language/FR-093). They describe the
complete ordered leaves independently of node identity preimages. The
selected text definition is `{authority: "agent-ix", identity:
"quire.value.text.unicode-17.0.0/v1"}`. `N(p)` denotes exactly
`{"path":p,"laws":[{"role":"text_profile","definition":{"authority":"agent-ix","identity":"quire.value.text.unicode-17.0.0/v1"}}],"mode":{"kind":"text_profile","value":"nfc"}}`;
`B(p)` differs only in mode value `"binary-utf8"`; `R(p)` denotes exactly
`{"path":p,"laws":[],"mode":null}`. Each path is an ordered JSON array of
literal UTF-8 segment strings, with no normalization, inserted separator
or escaping of the colon.

| Vector | Declarations and walked type | Complete leaves, in order |
| --- | --- | --- |
| U1 | `union Empty { None, Done }`; `Empty` | `[]` |
| U2 | `union Choice { Right(Text[0, 8; binary-utf8], Text[0, 8; nfc]), Left(Text[0, 8; nfc]), Empty }`; `Choice` | `B(["member:Right","position:0"])`, `N(["member:Right","position:1"])`, `N(["member:Left","position:0"])` |
| U3 | `union Wrapped { Item(Option<Sequence<Text[0, 8; nfc]>[0, 2]>), Empty }`; `Option<Wrapped>` | `N(["inner","member:Item","position:0","inner","inner"])` |
| U4 | `union Chain { End(Text[0, 8; nfc]), Next(Option<Chain>) }`; `Chain` | `N(["member:End","position:0"])`, `R(["member:Next","position:0","inner","recursion:0"])` |
| U5 | U4's `Chain`; `Option<Chain>` | `N(["inner","member:End","position:0"])`, `R(["inner","member:Next","position:0","inner","recursion:1"])` |
| U6 | `union Silent { End, Next(Option<Silent>) }`; `Silent` | `[]` |
| U7 | `union Link { End(Text[0, 8; binary-utf8]), More(Cell) }`; `record Cell { tag: Text[0, 8; nfc]; next: Link?; }`; `Link` | `B(["member:End","position:0"])`, `N(["member:More","position:0","field:tag"])`, `R(["member:More","position:0","field:next","inner","recursion:0"])` |

For U2, renaming `Left` to `Other` changes only `member:Left` to
`member:Other`; declaring `Left` before `Right` moves its leaf before
both `Right` leaves; swapping `Right`'s payload types exchanges the
profiles at `position:0` and `position:1`. Source `case` arm order changes
none of these lists. U4 expands to `End`'s text at every finite chain of
`Next` payloads with its `nfc` profile; U7 expands both profiles through
every finite `More`/`next` chain. The record and tuple examples above
retain their paths, profiles and order.

## Operation families, groups and constraints

This section is normative; the catalog file is its machine-readable copy and a
disagreement between them is a catalog defect, not a reader choice.

A type node's **family** is read from the type the node resolves to through its
`bounded_domain` base chain: `scalar_type` forms `boolean`, `integer`,
`rational`, `decimal`, `float32`, `float64`, `text`, `string`, `uuid` and `timestamp` are their own family, a
`unit` is `quantity`, an enum is `ordered_enum` when its nominal preimage is
ordered and `enum` otherwise; `composite_type` forms `option`, `record`,
`tuple`, `union`, `sequence`, `set`, `bag`, `ordered_set` and `reference` are
their own family; a `model`/`record_value_type` or `model`/`event_type`
declaration is `record`, a `model`/`variant_type` declaration is `enum` (never
`ordered_enum`), and
a model declaration node whose `semantic_form` is `object_type`, `process`,
`state_machine` or `systems_interface` is `object`. A `relation` node and a `model` node of any
other form have no family, so a term they type fits no operand position. An
`object` term is admitted only at a `field_owner` operand position and at
the `object` anchor position of `quire.op.state.clause`: `object` is a
member of no other group, so the `inner:0` result of
`quire.op.model.deref` is read only through a field projection ("Model-owned
members"), and a reference to an object type or to a `state`/
`operation_anchor` node (whose `semantic_type` is its context object type)
is admitted only as a state clause's anchor. The same holds for the node of a
process or a state machine, and for an anchor whose context is one.
The `type_argument` member of `quire.op.reference.upcast` names a
`Reference<T>` node, whose family is `reference`. A term's family is its
type's family, except that a function reference is `function`, a relation reference is `population`,
a `dependency_reference` callee has family `function`,
an aggregate is `aggregate`, a binding is `binder` (whose *bound* is the
descriptor of its value), a clause application is `clause`, a temporal
formula application is `temporal`, and a reference to a `temporal`/`formula`
or `temporal`/`fairness` node has the family of its body's root application
([FR-370](FR-370-checked-package-temporal-clause-body.md)).

| Group | Families | Decision rule |
| --- | --- | --- |
| `any_value` | `boolean`, `integer`, `rational`, `decimal`, `float32`, `float64`, `text`, `string`, `uuid`, `timestamp`, `enum`, `ordered_enum`, `quantity`, `option`, `record`, `tuple`, `union`, `sequence`, `set`, `bag`, `ordered_set`, `reference` | An operand position admits a term whose family is a member. |
| `any_term` | `any_value` plus `function`, `population`, `aggregate`, `binder`, `clause`, `temporal` ([FR-370](FR-370-checked-package-temporal-clause-body.md)) | Used only as the rest position of clause and call operations. |
| `exact_numeric` | `integer`, `rational`, `decimal` | Exact numeric operands of conversions. |
| `rational_promotable` | `integer`, `rational` | Operands promoted exactly by `n ↦ n/1` (FR-146, FR-149). |
| `ieee_float` | `float32`, `float64` | IEEE operands; no `eq`, `ne`, ordering or `negate`. |
| `collection_kind` | `sequence`, `set`, `bag`, `ordered_set` | Collection operands and `element` members. |
| `structural_kind` | `option`, `record`, `tuple`, `union`, `sequence`, `set`, `bag`, `ordered_set` | Structural comparison operands. |
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
`reference`; `arm_body` the type every arm body of a `case` application has
([FR-440](FR-440-checked-package-union-and-case-nodes.md)). The application's `result_type` must have the derived family and,
when the derived result names a type, be that same type node; a clause result
is declared `boolean`. An `expression` or `function` whose body root is the
application has `semantic_type` equal to that `result_type`. A `literal` term's
`type` must have a family its `value_kind` admits (`boolean`, `integer`,
`rational`, `decimal`, `float32_bits`→`float32`, `float64_bits`→`float64`,
`text`, `string`, `uuid`, `timestamp`, `enum`→`enum` or `ordered_enum`, `none`→`option`). Any failure is
`operator-ineligible`.

The fit rule has one exception besides record conformance ("Record
conformance"), **event equality**: `quire.op.structural.eq` and
`quire.op.structural.ne` refuse an application whose operand 0 type node is,
or contains at any depth through option, tuple, union and collection element
positions, a `model`/`event_type` node, though the family is in
`structural_kind` and `same_type` holds. FR-208 gives an event type no
equality, and no record, field or parameter holds an event (FR-208-AC-9), so an
event node reaches a compared type only through an event-invariant clause body.
It is an exception to the fit rule, not a constraint kind: the catalog entries
are unchanged, and the reader applies it at the application's
`operator-ineligible` check.

The native scalar families `string`, `uuid` and `timestamp` each have the
identities `quire.op.<family>.eq` and `quire.op.<family>.ne`:
`quire.op.string.eq`, `quire.op.string.ne`, `quire.op.uuid.eq`,
`quire.op.uuid.ne`, `quire.op.timestamp.eq` and `quire.op.timestamp.ne`. Each
takes two operands of its own family, has result `boolean`, carries no law, no
mode, no member and no leaves, and decides equality by the canonical payload,
so equality is reflexive, symmetric and transitive and `ne` is its negation.
UUID and Timestamp equality is FR-149's (FR-149-AC-14); String equality is QSL
FR-056's "Checked native String and UUID leaves" (FR-056-AC-23): two `string`
values are equal exactly when their Unicode scalar sequences are identical,
with no normalization or case folding. `string` is not `text`: it has no text
profile, so a `text_profile` law or mode on a `string` identity refuses at
the law and mode steps of the reader order above, and a `string` operand of a
`quire.op.text.*` identity pins no text profile and refuses
`invalid_package`/`operation-mode-type-mismatch`. These families have no
ordering, no arithmetic and no other operation identity: an ordering or
arithmetic identity over them refuses `ill_typed`/`operator-ineligible`
(FR-149-AC-16 for UUID and Timestamp).

**Equality-stable types.** A type is *equality-stable* when an equality
application is admitted over two operands of it: a type of family `boolean`,
`integer`, `rational`, `decimal`, `text`, `string`, `uuid`, `timestamp`,
`enum`, `ordered_enum`, `quantity` or `reference` (each through its own
`eq` identity), and a type of a `structural_kind` family through
`quire.op.structural.eq` when no leaf at any depth is `float32`, `float64`
or a `model`/`event_type` node. IEEE values have no `eq`
(`quire.op.ieee.numeric_equal`, `total_order` and `bit_identical` are three
distinct identities), and a `function`, `population`, `object` or
`aggregate` term is not a value type. A requirement that a key, a comparison
or a map position needs a type "over which `=` is admitted" (the FIFO key of
[FR-465](FR-465-checked-package-protocol-declaration-bodies.md)) means
exactly an equality-stable type.

**Scalar literal values.** The `value` of a `string`, `uuid` or `timestamp`
literal enters the node-key preimage, so it has one spelling, the canonical
payload of its native, as [QSL FR-056](ix://agent-ix/quire-spec-language/FR-056)
"K2 canonical native payloads" defines it: a `uuid` is 32 lowercase ASCII
hexadecimal digits grouped `8-4-4-4-12` with four hyphens; a `timestamp` is the
canonical decimal text of a signed 128-bit nanosecond count on the POSIX epoch
scale, `0` or an optional `-` followed by a nonzero digit and further digits,
within `-170141183460469231731687303715884105728` through
`170141183460469231731687303715884105727`; a `string` is its exact Unicode
scalar sequence as a JSON string, with no normalization. A `value` in any other
spelling (a `uuid` with uppercase digits or no hyphens, a `timestamp` of `-0`,
`+1`, `01`, empty text or outside the range, a `string` whose `value` is not a
JSON string) refuses `invalid_package`/`invalid-value` at the literal's
`value`. The spelling is read after the literal's family check, so a literal
whose `type` is not its kind's family refuses `ill_typed`/`operator-ineligible`
even when its value is also non-canonical.

| Constraint | Decision rule |
| --- | --- |
| `same_family` | The two named operands have equal families. |
| `same_type` | The two named operands resolve to the same type node. |
| `conforming_reference` | The two named `reference` operands conform: they resolve to the same type node, or their object types conform one to the other as "Reference conformance" below decides. |
| `reference_edge` | The `field` member is a reference edge between the two named `reference` operands, as "Reaches over a field" above decides. |
| `same_dimension` | Operand 0's unit and the `type_argument` member's unit carry the same `dimension_node_id`. |
| `inner_type` | Operand 1's family is the element family of collection operand 0 and, when operand 1 names a type, the same element type node, or, when that element type is a record type, a record type node that conforms to it ("Record conformance"). |
| `member_of` | Operand 0 resolves to the member's declaring type, or, when that type is a record type, to a record type that conforms to it ("Record conformance"). |
| `member_family` | The member's type family is the named family or a member of the named group. |
| `bound_family` | The bound of binder operand n has the named family or group. |
| `bound_is_member` | The bound of binder operand n resolves to the same type node as the `type_argument` member. |
| `exact_conversion` | FR-149 classifies operand 0's type to the member type as exact. |
| `range_narrowing` | FR-149 classifies it as range narrowing: a checked obligation that the value lies in the target range (`unproved-range` when not proved), never rounding; `mode` is `null`. |
| `rational_narrowing` | FR-149 classifies a rational-to-rational conversion as rational narrowing: a checked obligation that the reduced numerator and denominator lie in the target bounds (`unproved-range`); `mode` is `null`. |
| `scale_reduction` | FR-149 classifies it as scale reduction to a decimal; this is the only conversion that rounds, and it requires a `rounding` mode. |
| `promotes_exact` | Every named operand is in `rational_promotable` and is promoted exactly to its rational value before the operation. |
| `uniform_rest` | Every rest argument has the same family. |
| `union_arms` | The rest arguments are exactly one arm per member of operand 0's union, in member declaration order, each with as many binders as that member's payload arity, each binder typed by its payload position ([FR-440](FR-440-checked-package-union-and-case-nodes.md)). |

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
excluded. A strict reader first validates the `contract_version` document-type tag, closed
wire shape and canonical bytes, then recomputes `package_id` from
`identity_preimage` and each node key from its preimage before graph
admission. A locked source, definition or domain-package digest is an
identity member of the lock: it enters `package_id` through the identity
preimage, and a domain-package digest keys the document the reader reads and
is the one content check the reader makes on that document ("Model-owned
members" step 1). A document whose `contract_version` is not
`quire.checked-package/v2` is not a checked package. Such a document, missing or duplicate
members, out-of-domain values, stale dependencies, cross-domain substitutions,
unknown required features and non-canonical encodings refuse before consumption.
An unknown object member is a V2 refusal. An unrecognized `node_tag` refuses
as `unsupported_node_tag` (QSL ADR-013 O-14, QC-19). QSpec revises the closed
`node_tag` set in place; a reader refuses a `node_tag` outside the set
explicitly with `unsupported_node_tag` rather than admitting or inferring it.
A reader refuses a document with any other `contract_version` as
`unknown_contract_version`, the refusal that the document is not a checked
package, and infers nothing from its content. Exact selected resource limits are admitted;
one-over-limit input returns an incomplete resource disposition without a
partial checked package.

**Integers in a node-key preimage.** A node-key preimage
(`quire.application-node/v1` and QSL FR-092's `quire.structural-node/v1`)
holds integers in two kinds of place, and each kind has one spelling:

- An integer *value*, meaning a literal's `value` of `value_kind`
  `integer` and so every range or cardinality bound, is an `IntegerString`,
  the canonical decimal string, at every magnitude. `Int[0, 9]`'s bounds are
  `"0"` and `"9"`, and `Int[0, 18446744073709551615]`'s upper bound is
  `"18446744073709551615"`.
- A *counter*, meaning `recursion.size`, `recursion.ordinal`, a
  `group_reference` `ordinal` and an `operation.member` `position`, is a JCS
  number when its value is at most 2^53-1 (9007199254740991), and its
  canonical decimal string otherwise. Each counter is decided on its own:
  `{size: "9007199254740992", ordinal: 9007199254740991}` is one recursion
  member.

This is the replay wire's rule for an integer that can exceed 2^53 (QSL
ADR-013 O-09), and it is not a choice: RFC 8785 renders a JSON number
exactly only up to 2^53-1, so a counter beyond that has no exact number
form, and its decimal string is the one spelling that keeps the key exact.
No preimage integer is refused for its magnitude. A counter at most 2^53-1
written as a string, and a counter beyond it written as a number, are not
this form: `node-identity-preimage.schema.json` refuses both. A reader
never reads a counter from the wire: it derives each one from the graph and
writes it in this form, so a node whose retained key was computed from the
other spelling differs from the recomputed digest and refuses
`invalid_package`/`stale-node-key` at that node's `node_id` (the
application-node key stage, or the derived-shape key stage's first pass for
an in-group structural node). Every key published before this rule had
counters below 2^53 and string values, so no published key changes.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-322-AC-1 | Every admitted semantic occurrence maps to at least one original source region. | Analysis |
| FR-322-AC-2 | Changing backend capability availability cannot change package admission or package identity. | Analysis |
| FR-322-AC-3 | Changing a meaning-affecting lock member changes the package identity. | Analysis |
| FR-322-AC-4 | The public reader refuses a document that is not a checked package (`unknown_contract_version`), malformed/duplicate members, stale dependencies, cross-domain digest substitutions, unknown required capabilities and non-canonical bytes before exposing an admitted package. | Test (TC-217) |
| FR-322-AC-5 | The canonical positive, boundary, refusal and mutation vectors distinguish raw source and definition byte digests and domain package `sha256-jcs` digests from checked-semantic and generated-IR identities, even when digest bytes are equal. | Test (TC-217) |
| FR-322-AC-6 | Exact selected limits admit the boundary vector; one-over-byte, node, edge, locus, diagnostic and work inputs return incomplete resource accounting naming the limit, its value and its setting, and no partial checked package. | Test (TC-217) |
| FR-322-AC-7 | A consumer obtains the complete closed semantic graph through the public schema and never through a native parser/CST/package implementation type. | Inspection |
| FR-322-AC-8 | Two conformers derive the same package id from the specified JCS identity preimage; changing an excluded raw artifact/source-map/presentation or reader-limit member does not change it, while changing any named preimage member does. | Test (TC-217) |
| FR-322-AC-9 | The reader result is exactly admitted, refused or incomplete; malformed input and the first exhausted measured limit remain distinguishable and carry their specified typed provenance/accounting. | Test (TC-217) |
| FR-322-AC-10 | Enum, dimension and declared-unit node keys match the closed JCS preimage vectors; stale-body keys, non-normalized rationals, invalid owner joins and every other noncanonical member or term order this artifact refuses -- including an application node's `dependencies` list -- refuse as `invalid_semantic_graph`, except a `state`/`frame` node's own `modifies`/`creates`/`deletes` canonical order, which [FR-340-AC-4](FR-340-checked-package-frame-location.md) owns. | Test |
| FR-322-AC-13 | The published V2 checked-package schema admits its positive fixture carrying all 13 closed node tags with an exact source-map entry for every occurrence, and refuses each authored document-type, digest-domain and node mutation. | Test |
| FR-322-AC-14 | Each published V2 package fixture's `package_id` equals the RFC 8785 JCS SHA-256 of the identity preimage rebuilt from its lock selections and occurrence-free semantic graph; editing a source-map region, node source occurrence, raw source digest, capability disposition or diagnostic catalog leaves that id unchanged, while editing the edition, a profile/definition/model/dependency selection, a required feature, a node member or node order changes it. | Test |
| FR-322-AC-15 | Each published enum, dimension and declared-unit node vector validates against the closed preimage schema and recomputes its recorded JCS SHA-256; each authored semantic mutation is refused by exactly its named schema or semantic check, an absent owner fails the package lock join, and inside the published nominal package a retained preimage with a changed body, type, dependency or unjoined owner is refused by the published cross-field or owner-join constraint. | Test (TC-233) |
| FR-322-AC-17 | The published operation catalog is closed: both V2 schemas carry identical operation definitions and enumerate exactly its identities, law roles, mode kinds and values, member kinds and operator classes; every law-role `DefinitionRef` names a complete-value lock definition by authority and identity; every entry's operands, rest, result form, laws, mode, member, leaf source and constraints use only catalogued families, groups and kinds; integer `div`/`rem` require the integer-division law and `mod` none, IEEE operations require the IEEE profile and have no `eq`, `ne`, ordering or `negate` identity, text operations require the text profile and text mode, `all_instances` takes one population, `lookup` requires an absence mode, `navigate` and `reaches` a relationship end, `reaches_field` a `field` member and the `reference_edge` constraint, `quire.op.state.clause` the `state_clause` member kind and no law, and the record, tuple and index projections, numeric/total-order/bit-identical IEEE comparisons, `deref` and `dispatch_call` are present. | Test (TC-233) |
| FR-322-AC-18 | Each published operation vector validates against the closed node-preimage schema, recomputes its recorded JCS SHA-256 and is distinct from every other vector; every application node of the published operation package is keyed by its application-node preimage and is a published vector, and a vector outside the package refuses as `operation-law-unselected` when placed in it; floor-to-Euclidean division changes the node and package ids; a retained key refuses as `stale-node-key`; a contradictory form, missing member-declaration dependency, stray recursion group or extra dependency is refused; an application without an operation fails the V2 schema; and the all-families function body calls a published function, changes its node id when rewritten as `dispatch_call` and then refuses as `operation-member-mismatch`. | Test (TC-233) |
| FR-322-AC-19 | Each published operation mutation never keeps its stale key and either rekeys to exactly its named counterpart vector and is admitted exactly when its laws are lock-selected, refuses as `operation-law-unselected` until the lock also selects its law and again under the stale selection, or is refused by exactly its named schema or semantic check with its recorded cause and that cause's code; together the mutations exercise every operation refusal cause. | Test (TC-233) |
| FR-322-AC-21 | Every node of each published V2 package carries a `declaration` member exactly when it declares; the schema refuses a declaring node without it and an expression with it; the member enters the application-node preimage and package id; a name duplicated on two nodes refuses as `ambiguous-name` at the node later in node-id digest order; and a nominal node whose declaration differs from its `qualified_declaration` refuses as `declaration-nominal-mismatch` before any ambiguity. | Test (TC-233) |
| FR-322-AC-22 | Every clause application of the published clause package carries exactly one profile law of its catalogued role equal to the lock's `profile_selections` row, and every clause application other than `quire.op.temporal.clause` (FR-370) a `profile_operator` member the profile defines; removing the law refuses as `operation-law-missing`, a protocol profile in the temporal role as `operation-law-mismatch`, a lock without the temporal profile row as `operation-law-unselected`, and an undefined, other-profile or absent operator as `operation-member-mismatch`; for a `quire.op.temporal.clause` law naming no FR-250 profile, the full reader reports FR-370's earlier profile check (`unknown_profile`) before the operation step's `operation-law-mismatch`. | Test |
| FR-322-AC-23 | Every application carries a `result_type` and every literal a `type`; a root `result_type` differing from the node `semantic_type`, a `result_type` the operation cannot produce, and a literal whose `type` family its `value_kind` does not admit each refuse as `operator-ineligible`, while the same literal with its declared type is admitted. | Test (TC-233) |
| FR-322-AC-24 | The FR-149 conversion classification is decided from declared bounds: an integer range into a containing integer or rational range is exact, an integer range whose scaled bounds exceed a decimal range is range narrowing, a decimal of scale 3 into scale 2 is scale reduction and a rational into narrower rational bounds is rational narrowing; `quire.op.rational.div` over integer-range operands carries `promotes_exact` and its published vector is admitted, while a decimal operand refuses as `operator-ineligible`. | Test |
| FR-322-AC-25 | An `operation` member resolves only against a declaring `model` node of `semantic_form` `object_type`, `process`, `state_machine` or `persistence_interface`; every other model form, including `systems_interface`, refuses `operator-ineligible`/`ill_typed`. | Test |
| FR-322-AC-26 | A reference to a dependency package's node is carried as `PackageNodeKey{package, node}`, where `node` is a `WireNodeId`; per R-10 it becomes a `NodeKey` only by lookup in a checked package compiled from digest-addressed source, never minted directly from wire bytes in either the dependency's or the importing package's namespace, and a reader resolves it only against the named dependency's admitted package, refusing a `package` absent from `dependency_selections` as `missing_declaration`/`missing-selection`. | Analysis |
| FR-322-AC-27 | An unrecognized `node_tag` refuses as `unsupported_node_tag`, and QSpec revises the closed `node_tag` set in place. | Analysis |
| FR-322-AC-28 | A `ModelOwner` node-identity preimage carries the declaring domain package's `identity` beside `node` and no version label, so a node id depends only on content; the published `ModelOwner` vectors validate against the closed preimage schema and recompute their recorded JCS SHA-256, and a `ModelOwner` preimage that carries a `version` member is refused by the closed preimage schema. | Test |
| FR-322-AC-29 | Each published model declaration node vector validates against the closed `ModelDeclarationNode` preimage schema and recomputes its recorded JCS SHA-256. A model declaration node carrying `Order`'s `ModelOwner` and keyed by its preimage resolves to `Order`. A node whose owner identity is not lock-selected refuses `missing_declaration`/`missing-selection` in the derived-shape key stage, and a current `model`/`object_type` node whose owner names the relationship `Order-order-Order` refuses `missing_declaration`/`missing-selection` in the owner join. A node whose key was computed under another tag (the relationship's id on a `model` node whose owner names an object type; one whose owner names the relationship is refused by the owner join first, as the second refusal above), whose `semantic_type` is not its own id, that carries a `recursion_group`, or whose body has members refuses `invalid_package`/`stale-node-key` (MW-02, MW-04, MW-05, MW-15: the relationship's id on a `model`/`object_type` node owned by `Order`). A model declaration node that carries a `declaration` beside its `ModelOwner`, or carries no `owner`, fails the V2 schema and refuses `malformed_wire`. | Test |
| FR-322-AC-30 | A model-owned `field` or `operation` member resolves among the declaring type's exposed effective members, own and inherited. A field declared by a supertype resolves on the subtype's node, and a redefined operation resolves to the most derived redefinition. An operation on a `systems_interface` node refuses `ill_typed`/`operator-ineligible`. Two exposed fields of one name refuse `ambiguous_declaration`/`ambiguous-name`. An undeclared name, a name of the other member kind and a relationship declaring node each refuse `ill_typed`/`operator-ineligible`. | Test |
| FR-322-AC-31 | A resolved model member's type follows the declared-type, multiplicity and presence tables. The derived types are `Int[0, 9]` from an integer value type, `Integer` from a constrained native `Integer`, `Option<Boolean>`, `Set<Integer>[0, 3]`, `Sequence<Integer>[0, 5]`, `Set<Integer>[0, 1]`, `Set<Integer>` for `[0, unbounded]`, `String`, `UUID` and `Timestamp` from their native declared types (`String` distinct from `Text`), `Reference<Invoice>` and an operation's `Integer` result. A bare `Text` field (no `Text[min, max; profile]` value type), a `[1, unbounded]` field and an operation with no result each refuse `ill_typed`/`operator-ineligible`. So does a call whose receiver is not a reference to the declaring node, or whose arguments differ in number from the operation's parameters or in type node from them, unless a record argument conforms to its record parameter ("Record conformance"). | Test |
| FR-322-AC-32 | A lock-selected domain package document is checked as evidence before graph admission: a document supplied under the selected digest with matching identity passes the selection admission checks, while a digest domain other than `sha256-jcs` refuses `stale_dependency`/`digest-domain-mismatch`, no document refuses `missing_import`/`missing-selection` and a document whose identity differs from the selection `invalid_model_binding`/`wrong-model-selection`. | Test |
| FR-322-AC-33 | `quire.op.reference.eq` and `quire.op.reference.ne` admit two references to selected object types of which one conforms to the other along declared supertypes: two of one type, a subtype and its supertype in either operand order, a subtype two levels down, and a type with two supertypes against either. A systems interface with no declared supertype against an unrelated object type, two unrelated object types, a reference to a relationship, even compared with itself, and an operand of family `object` refuse `ill_typed`/`operator-ineligible`. An operand whose owner is not selected refuses `missing_declaration`/`missing-selection`, even compared with itself. | Test |
| FR-322-AC-34 | `quire.op.record.project` over a `quire.op.model.deref` result reads a field of a model object: the `deref` result has family `object`, the projection resolves the field on the declaring object type, own or inherited, and its result is the field's member type. A projection over a reference that is not dereferenced, a `field` member naming a node other than the operand's object type, a `deref` whose `result_type` is not the operand's object type, a `deref` result of a relationship, and a projection whose `result_type` is not the field's member type refuse `ill_typed`/`operator-ineligible`. | Test |
| FR-322-AC-35 | A `dependency_selections` entry is a closed `DependencySelection` `{identity, package_id}` whose `package_id` is a `quire.package.semantic/v2` `PackageId`: the published package carrying two such entries, one per identity in ascending identity order in both its lock and identity preimage, is admitted by the V2 schema and its recorded `package_id` recomputes from them, and changing a dependency's `package_id` changes it; an entry with a `Selection` or `DefinitionRef` shape, no `package_id`, a bare-digest `package_id`, a `package_id` in another digest domain or an extra `version` member fails the schema in either member, at the mutated entry, and a `model_selections` `ModelRef` entry carrying an extra `version` member likewise fails it at that entry; a reader refuses either `version`-bearing entry `unknown_member`, since a selection binds by identity and digest and never by version. Entries ordered by UTF-8 bytes where UTF-16 code-unit order differs are admitted; a second entry of one identity refuses `invalid_package`/`conflicting-definition` with both entries as loci, and an entry not strictly after its predecessor refuses `invalid_package`/`invalid-value` at its pointer. | Test (TC-233) |
| FR-322-AC-36 | A `{term: "dependency_reference", package, node}` term whose `package` is a `quire.package.semantic/v2` `PackageId` and whose `node` is a `NodeId` is admitted by the V2 schema as an application argument and by the node-identity preimage schema as a preimage term; a `package` given as a bare digest or in another digest domain, a missing `node` and an extra member each fail both schemas. A `quire.op.function.call` application-node preimage whose callee is a `dependency_reference` has a different JCS SHA-256 when only the reference's `package` digest changes, and a different one again when only its `node` changes. | Test |
| FR-322-AC-37 | A package whose function calls a dependency `function` declaration over `Int[0, 9]` returning `Boolean`, through a `dependency_reference` callee, with the Boolean node as `result_type` and no `dependencies` entry for the target, is admitted when the dependency's admitted package is supplied. With no dependency package supplied it refuses `missing_import`/`missing-selection` at the selection entry; with the callee's `package` absent from `dependency_selections`, `missing_declaration`/`missing-selection`; with `node` naming a node that carries no `declaration`, `missing_declaration`/`missing-name`; with `node` naming a declared record, a function returning a declared record, a function over `Set<R>` or over a tuple holding `R` for a declared record `R`, a function over `Reference<M::T>` for a model type `M::T`, or with the term as a non-callee argument or as a node body root, `ill_typed`/`operator-ineligible`; with the target listed in `dependencies`, `invalid_semantic_graph`. | Test |
| FR-322-AC-38 | `quire.op.model.reaches_field` admits a field edge of the owner's own reference type: `Order.parent` (`Option<Reference<Order>>`), `Order.prior` (`Sequence<Reference<Order>>[0, 3]`) and `Order.origin` (`Reference<Order>`), the inherited `parent` read on `Sub`, and a target of a subtype of `Order`. A set edge, an unbounded sequence edge, an edge to another object type, a non-reference field, a target of an unrelated type, an operand 0 of a type other than the member's declaring node, an undeclared field name and an operand of family `object` refuse `ill_typed`/`operator-ineligible`; an ambiguous field name refuses `ambiguous_declaration`/`ambiguous-name`, and a declaring node keyed under an unselected domain package version `missing_declaration`/`missing-selection`. | Test |
| FR-322-AC-39 | Every node body of every published V2 package and every node-identity vector conforms to the five-stratum body grammar, and the V2 schema and the node-identity preimage schema each admit exactly that grammar. | Test |
| FR-322-AC-40 | A body carrying an application inside an application's arguments, an application or a binding of an aggregate inside an aggregate's members, or an aggregate inside a Group's members fails the V2 schema and is refused `malformed_wire`, and the same meaning with each composite subterm as its own referenced node is admitted. | Test |
| FR-322-AC-41 | A package holding a 100,000-deep expression has the same JSON nesting depth as a package holding a one-level expression, is emitted, read back and admitted under node, edge and byte limits sized for its node count, and no reader limit or effective-limit record names a depth. | Test |
| FR-322-AC-42 | A package whose lock names source and definition digests is admitted with no source or definition bytes and no digest evidence supplied. | Test |
| FR-322-AC-43 | A domain-package document supplied under its selection's digest with the selection's identity, but altered so that the SHA-256 of its RFC 8785 bytes differs from that digest, refuses `stale_dependency`/`content-mismatch` naming the selected and the recomputed digest, before its identity is compared and before any declaration is read; the unaltered document passes. | Test |
| FR-322-AC-44 | A field typed by a value type bound as `Text[min, max; profile]` has type `Text[min, max; profile]` over a `text_bounds` node; a value type bound as `Text` with `min > max` or with a profile outside FR-141's six (a negative `min` is not a document: `minLength` is a non-negative integer) gives the member no type and refuses `ill_typed`/`operator-ineligible`. | Test |
| FR-322-AC-45 | `quire.op.structural.eq` over two `Label` values of `union Label { Named(Text[0,8;nfc]), Tagged(Integer, Text[0,8;nfc]), Empty }` carries exactly the leaves `["member:Named", "position:0"]` and `["member:Tagged", "position:1"]` in that order, each with the `nfc` text law and mode, and none for `Empty`; both V2 schemas admit the `member:<Ident>` segment. | Test |
| FR-322-AC-46 | `quire.op.structural.eq` over two values of `union List { Cons(Integer, List), Nil }`, whose cycle names one `recursion_group`, is admitted with empty `leaves`, and a record whose field's type cycles back to the record through a union is likewise admitted; neither refuses `operator-ineligible`. Over `Node`, `Cell`, `Chain` and `Option<Chain>` of the "Structural leaf walk" examples it is admitted with exactly the listed leaves in that order, each recursion leaf with `laws` `[]` and `mode` `null`; both V2 schemas admit the `recursion:<d>` segment. | Test |
| FR-322-AC-47 | The V2 `Diagnostic` schema admits `type-mismatch` only with `ill_typed`, `missing-selection` only with `missing_import` or `missing_declaration`, and `unsupported-selection` and `wrong-selection-role` only with `unknown_language`, `unknown_edition` or `unknown_profile`. | Test |
| FR-322-AC-48 | A published package whose ungrouped structural node is tampered, with its `node_id` kept, its `identity_projection` entry patched the same way and its `package_id` recomputed, is schema-valid and identity-consistent and refuses `invalid_package`/`stale-node-key` at that node's `node_id`: an `integer_range` `max` narrowed to `2` and widened to `5000`, its `min` raised to `1` and its bounds swapped, a `reference` target re-pointed at the `Integer` node, a `set` element re-pointed at the `Integer` node, and the declared record `Example::Point`'s field `x` re-pointed from the `Integer` node at the `Int[0, 9]` node. The untampered package is admitted. | Test |
| FR-322-AC-49 | The recursive-records package (`record List { next?: List; }` and `record Tree { kids: Sequence<Tree>[0, 3]; }`, each one recursion group keyed by QSL FR-092) is admitted. With one in-group node tampered, its `node_id` kept, its `identity_projection` entry patched and its `package_id` recomputed, it refuses `invalid_package`/`stale-node-key` at that node's `node_id`: `Tree`'s `collection_bounds` `max` widened from `3` to `1000` and its `min` raised to `1`, `Tree`'s `Sequence` element re-pointed from the record at the `collection_bounds` node, and `List`'s `Option` re-pointed at the `Integer` node. The `Tree` group relabelled with a label that is not its group digest, each member rekeyed under that label and every reference rewritten, refuses `invalid_package`/`stale-node-key` at the group member with the lowest node-id digest. | Test |
| FR-322-AC-50 | Every structural node of every published positive fixture, grouped or not, model declaration nodes included, carries the key QSL's FR-092 key function (FR-094's for a model-owned node) mints for it from its own members, its `owner` included, and no placeholder key; the recursive-records fixture's group labels are its groups' digests; and the full reader admits every published positive fixture. | Test |
| FR-322-AC-51 | Every owner-bearing node of every published V2 package carries `owner` with exactly its identity preimage's `owner` value, a `SourceOwner` on a declared structural node and a `ModelOwner` on a model declaration node or clause function, and its `identity_projection` entry carries the same `owner`; no other node carries one. The V2 schema refuses, and a reader refuses `malformed_wire`: a declared record without `owner`, a declared record with a `ModelOwner`, a model declaration node without `owner` or with a `SourceOwner`, an `owner` on the builtin `Integer` node, on a nominal enum node and on an application node, a `DefinitionOwner`, a `SourceOwner` with an extra member and a `ModelOwner` with `version`. A declared record's `owner` changed to a second source that is added to `lock.sources`, with its `declaration` occurrence's regions moved to that source and its node id kept, its projection entry patched the same way and `package_id` recomputed, refuses `invalid_package`/`stale-node-key` at that node. An `identity_projection` entry whose `owner` differs from its node's (the `adverse.json` `projection_owner_mutations` entries `projection-source-owner-differs-from-node` and `projection-model-owner-differs-from-node`, each with `package_id` recomputed) passes the schema and refuses `invalid_package`/`invalid-value` at `/identity_preimage/identity_projection/{i}/owner`. | Test |
| FR-322-AC-52 | The derived-shape key stage joins each node's `owner` to the lock before either pass: a `SourceOwner` whose (`authority`, `identity`) is no `lock.sources` entry's, and a `ModelOwner` whose `identity` is no `model_selections` row's, refuse `missing_declaration`/`missing-selection` at that node's `node_id`, and are reported ahead of a stale key at a node of lower node-id digest. A declared node's `owner` and its source map agree: changing the `source` of the region of a declared node's `declaration` occurrence to another lock source, which leaves every node id and the `package_id` unchanged, refuses `invalid_package`/`invalid-value` at that occurrence's `source_map` entry. | Test |
| FR-322-AC-53 | The two-owner fixtures `positive-two-owners-a.json` and `positive-two-owners-b.json` declare the same records `Point { x: Integer; }` and `List { next?: List; }`, with the qualified names QSL emits for them, `["Point"]` and `["List"]` (QSL FR-092 D1), in both, with identical bodies, under the `SourceOwner`s (`agent-ix`, `example-a`) and (`agent-ix`, `example-b`). Both are admitted. Both also declare `three` (qualified name `["three"]`), a declared `value` whose body root is the application `1 + 2` (`quire.application-node/v1`, no `owner`). Their `Point` node ids differ, their `List` group labels differ and every member of the `List` group (the record and its `Option` node) has a different id in each, while the builtin `Integer` node and the application-keyed `three` node each have one id in both, as intended content addressing; their `package_id`s differ. Each package's in-group keys and group label recompute from that package alone. Package `a` with the `owner` members of `b`, `b`'s source added to `lock.sources`, the declared nodes' `declaration` occurrence regions moved to it, node ids kept, projection patched and `package_id` recomputed, refuses `invalid_package`/`stale-node-key`. | Test |
| FR-322-AC-54 | The derived-shape key stage leaves every `correspondence`/`abstraction_relation` node untouched: `positive-clause-operations.json`'s `correspondence`/`abstraction_relation` node, keyed by FR-451's `quire.abstraction-relation-node/v1` preimage and so not by any `quire.structural-node/v1` preimage, passes the stage and the full reader admits the package; the same node with one `function` segment of its `frames` entry changed, canonical order and `dependencies` kept, node id kept, projection entry patched and `package_id` recomputed, refuses `invalid_semantic_graph` at FR-451's abstraction step 1 (Identity) and not `invalid_package`/`stale-node-key`. | Test |
| FR-322-AC-55 | The owner join resolves every `ModelOwner` against the selected document, whether or not anything reaches the node: a model declaration node whose owner `node` the document does not declare, and a `model`/`object_type` node whose owner names the relationship `Order-order-Order`, each keyed by its own preimage, refuse `missing_declaration`/`missing-selection`; a precondition clause function owned by the operation member `Order/size` (MW-11, and the `clause-precondition-operation-owner` function of `positive-all-families.json`), an invariant owned by the object type `Order` (MW-12) and a body clause function owned by `Order/size` (MW-17) pass the join, while a clause function owned by an undeclared member `Order/missing` (MW-13), one owned by the field `Order/total` (MW-14), a precondition owned by the object type `Order` (MW-16, `clause-precondition-owned-by-object-type`), an invariant owned by the operation member `Order/size` (MW-18, `clause-invariant-owned-by-operation-member`), an invariant owned by the systems interface `Flow` (MW-22, `clause-invariant-owned-by-systems-interface`), a `postcondition` kind (MW-19, `clause-kind-postcondition`), a clause function with two `clause` bindings (MW-20, `clause-two-kind-bindings`) and one with none (MW-21, `clause-without-kind`) refuse `missing_declaration`/`missing-selection` at the clause function's `node_id`. The `clause-*` ids are the `adverse.json` `clause_owner_cases`. | Test |
| FR-322-AC-56 | A `relationship_end` member of a model-owned relationship resolves to one of the relationship's named ends, `sourceEnd.role` or, when the document declares one, `targetEnd.role`, and refuses `missing_declaration`/`missing-name` for any other name. The `Order-invoiced-Invoice` relationship of the published document declares no `targetEnd.role`, so it names only `invoiced`: `invoiced` resolves (`relationship-end-forward-without-inverse` in `adverse.json` `relationship_end_cases`, admitted by the full reader) and `next` refuses (`relationship-end-names-missing-inverse`; RE-ends-4 and RE-ends-5). `Order-order-Order` is `bidirectional` and names `order` and `next`. `Order-follows-Order` is `source-to-target` and names `follows` and `followedBy`: `follows` resolves (RE-ends-7) and `followedBy`, a named inverse traversed from target to source against that direction, refuses `ill_typed`/`operator-ineligible` at `member.name` (`relationship-end-inverse-against-direction`; RE-ends-8). `Order-twin-Order` names `twin` as both its source and its target role, so `twin` refuses `ambiguous_declaration`/`ambiguous-name` at `member.name` (`relationship-end-names-ambiguous-role`; RE-ends-9). A `relationship_end` whose `declaration` is not a relationship refuses `ill_typed`/`operator-ineligible` at `member.declaration`, after owner recovery and before role lookup (RE-ends-6). A missing role refuses at `member.name`. | Test |
| FR-322-AC-57 | `quire.op.model.navigate` has operator class `query`. `quire.op.model.navigate` and `quire.op.model.reaches` over a model-owned relationship end apply the checks in this order, each refusing at its recorded application pointer: declaration kind (`member.declaration`), role lookup (`member.name`: missing-name, ambiguous-name), receiver effective endpoint (`arguments/0`, and for `reaches` `arguments/1`: an operand conforms to the receiver endpoint type directly or through a supertype, else `ill_typed`/`operator-ineligible`), direction (FR-152; `member.name`), destination multiplicity (an unbounded upper or an ordered destination is `unsupported_construct`/`expression-form` at `member.name`), then the operation's own type comparison (`navigate`: `result_type` equals the canonical `Reference<U>`, `Option<Reference<U>>`, `Set<Reference<U>>[l,u]` or `Bag<Reference<U>>[l,u]` of FR-152, else `ill_typed`/`operator-ineligible` at `result_type`; `reaches`: a destination end of the receiver endpoint's own type, whether single-valued (`[0,1]`, `[1,1]`) or finite multi-valued (`Set`/`Bag` shaped, unique or not), else `ill_typed`/`operator-ineligible` at `member.name`; a `reaches` has no result type to compare, so a multi-valued destination end admits exactly as a single-valued one does). The published `relationship_navigation_cases` RN-01 to RN-27 record each. | Test |
| FR-322-AC-58 | Each `integer_encoding_vectors` entry of `node-identity-vectors.json` recomputes its recorded JCS SHA-256 from its preimage: `recursion-size-safe-maximum`, whose `recursion` is `{size: 9007199254740991, ordinal: 9007199254740990}` with both counters JSON numbers, gives `bc6fe35a2c34a7162fe0ed0a20473ffb3284bd70236f96feb75b386c55992874`; `recursion-size-two-pow-53`, whose `size` is the string `"9007199254740992"` and whose `ordinal` is the number 9007199254740991, gives `89ec1ac87a8b82a05d5414f0c3b314f363e49328988ff5d8c3fc9a88014839bc`; and `integer-range-u64-maximum`, the `quire.structural-node/v1` `integer_range` node of `Int[0, 18446744073709551615]` whose `max` is the string `"18446744073709551615"`, gives `f9ea36c52345743ae98ce1c1449610c7e955d986a3ecb08597ca427c6f1b4230`. The two recursion vectors validate against `node-identity-preimage.schema.json`'s `ApplicationNode`, and the same schema refuses the `size` 9007199254740991 written as a string and 9007199254740992 written as a number. Each published `operation_vectors` digest is unchanged. | Test |
| FR-322-AC-59 | Structural equality and inequality over U1–U7 require exactly their complete ordered leaf lists. U2's member rename, member reorder, payload swap and source arm reorder have exactly the effects stated with the vectors; U2 requires all three text leaves even when both operands select `Empty`. A missing final leaf refuses `invalid_package`/`operation-law-missing`; reordered leaves or a recursion leaf with a text law refuse `invalid_package`/`operation-law-mismatch`. | Test |
| FR-322-AC-60 | For each `K` of `set`, `bag` and `ordered_set`, equality over `K<Choice>` (U2) requires U2's leaves with one leading `inner` per path; `contains` over that collection (`inner:0`) and a catalogued `result_inner` operation yielding `K<Choice>` require U2's leaves without that prefix. A `result_inner` operation yielding `sequence<Choice>` requires `[]`, while equality over `sequence<Choice>` requires U2's leaves with one leading `inner`. | Test |
| FR-322-AC-61 | Both V2 schemas admit `member:Right`, `member:_item2`, `position:0`, `position:12`, `recursion:0` and `recursion:12`; they refuse `member:`, `member:1Right`, `position:01`, `position:-1`, `recursion:`, `recursion:01` and `recursion:-1`. The shared catalog includes `union` in `structural_kind`, so equality and inequality admit eligible Union operands under the existing `same_type` constraint and `operand:0` leaf source. | Test |
| FR-322-AC-62 | The settled derived type forms forward aliases to their resolved node without an `alias` node; key `Reference<Order>` as an anonymous self-typed `composite_type`/`reference` over `Order`'s model declaration; key `Population<Order>[3]` as an anonymous `bounded_domain`/`model_population` whose `semantic_type` is `Set<Reference<Order>>` and whose body binds integer `max = 3`; and key a formed `metre / second` quantity as an anonymous self-typed `scalar_type`/`compound_unit` with ordered unit/exponent terms over targetless canonical nominal unit roots whose semantic type is their dimension key. The worked accepted vectors are admitted, including the exact U2 vector. The `RM-REF-TARGET`, `RM-POP-MAX` and `RM-CU-ORDER` exact preimages bind their keys; the corresponding public wire nodes satisfy the `SemanticNode` schema. Retaining each original key exercises stale-key detection, and recomputing each key exercises semantic-shape refusal for a non-model reference target, negative bound, arbitrary set element type and noncanonical term order as `invalid_package`/`invalid-derived-shape`. | Test (`tests/checked_package_v2.rs`, `tc_322_derived_form_key_mutations`, `tc_322_recomputed_derived_forms_refuse_semantic_mutations`) |
| FR-322-AC-63 | A selected domain package with `Address { street: Text[0, 64; nfc] }` and object `Account { address: Address }` admits a self-typed `model`/`record_value_type` declaration node owned by `Address`; `Account.address` derives the `Address` node as its type and a `field` projection on an `Address` value resolves `street` to `Text[0, 64; nfc]`. The record node has family `record`, and an invariant clause function owned by `Address` passes the owner join. | Test |
| FR-322-AC-64 | A record field whose `typeRef` names another record, including a field inherited by a record subtype, derives the named record node. A record-typed field with `optional` presence derives `Option<Address>`, and one with multiplicity `[0, 3]`, ordered and non-unique, derives `Sequence<Address>[0, 3]`; both retain the declared record node at their innermost element. A finite recursive record reached through an optional field retains the same record declaration node at every level. None derives `Reference<Address>` or an object identity. | Test |
| FR-322-AC-65 | A `model`/`record_value_type` node owned by a selected object type or a node absent from the selected document refuses `missing_declaration`/`missing-selection` in the owner join; one with a missing `owner` refuses `malformed_wire`. An unknown field name on a valid record node refuses `ill_typed`/`operator-ineligible`. A record field whose `typeRef` names an event type, state machine, process, persistence interface or namespace refuses `invalid_model_binding`/`malformed-declaration` at FR-154 intake before member derivation; a record node used as an object reference target refuses `invalid_package`/`invalid-derived-shape`. | Test |
| FR-322-AC-66 | A package with a `protocol`/`protocol_reference` carrier to its local, conforming `protocol_clause` target is admitted with one graph collection; its body target, sole dependency and semantic type bind that exact protocol node and its recomputed key changes when the target changes. With the body target and sole dependency changed together and identities recomputed, a missing local target refuses `missing_declaration`/`missing-name` even when a selected dependency contains that key, and a present non-protocol target or unequal semantic type refuses `ill_typed`/`operator-ineligible`. A non-reference body or extra carrier member fails the V2 schema; a dependency target is not admitted as a protocol reference. The same package with two source references to that protocol is admitted with one carrier node carrying two `expression` occurrences, ordinals 0 and 1, each with its own `source_map` entry, and has the same carrier `node_id` and `package_id` as with one occurrence. A carrier with no occurrence or with an occurrence whose role is not `expression` fails the V2 schema; two `source_map` entries with one (`node_id`, `expression`, `ordinal`) key refuse `invalid_source_map` at the later entry's `/source_map/{i}` pointer, and a carrier whose `occurrences` array repeats an `expression` ordinal refuses `invalid_source_map` at the later occurrence's `/semantic_graph/nodes/{n}/occurrences/{j}` pointer (`n` the carrier's node index, `j` the later occurrence's index); two nodes with one `node_id` refuse `invalid_semantic_graph`. | Test |
| FR-322-AC-67 | The six identities `quire.op.string.eq`, `quire.op.string.ne`, `quire.op.uuid.eq`, `quire.op.uuid.ne`, `quire.op.timestamp.eq` and `quire.op.timestamp.ne` are enumerated by both V2 schemas and catalogued as binary `boolean` comparisons with no law or mode, and both schemas admit `string`, `uuid` and `timestamp` as a literal `value_kind`. An application of each identity over two operands of its own family, with result `boolean`, no law, no mode and no member, is admitted. An operand of another family (`string` against `text`, `uuid` against `string`, `timestamp` against `uuid`) and an ordering or arithmetic identity over these families (`quire.op.integer.lt` and `quire.op.integer.add` over `uuid` operands) each refuse `ill_typed`/`operator-ineligible`. A `text_profile` law on a `string`, `uuid` or `timestamp` identity refuses `invalid_package`/`operation-law-mismatch`, a `text_profile` mode `invalid_package`/`operation-mode-mismatch`, and a `string` operand of `quire.op.text.eq` `invalid_package`/`operation-mode-type-mismatch`. A literal of `value_kind` `string`, `uuid` or `timestamp` whose `type` is not its own family refuses `ill_typed`/`operator-ineligible`, and a `string`, `uuid` or `timestamp` literal `value` in a non-canonical spelling, on a literal of the right family, refuses `invalid_package`/`invalid-value`. | Test |
| FR-322-AC-68 | A selected domain package whose document declares the event type `OrderPlaced` (`quire.meaning.model.event-type/v1`, with fields `order_id` and `occurred_at`) admits a self-typed `model`/`event_type` declaration node owned by `OrderPlaced`, with a `ModelOwner` and the closed `ModelDeclarationNode` preimage. The node has family `record`, a `quire.op.record.project` over a value of its type resolves a field of `OrderPlaced` (own or inherited) to its step 4 type, and an invariant clause function owned by `OrderPlaced` passes the owner join. A `quire.op.structural.eq` or `quire.op.structural.ne` over two `OrderPlaced` operands, or over two `Option<OrderPlaced>`, tuple-holding-`OrderPlaced` or `Set<OrderPlaced>` operands, refuses `ill_typed`/`operator-ineligible`; the same applications over `Money` operands are admitted. A `model`/`event_type` node owned by a record value type, an object type or a node absent from the selected document refuses `missing_declaration`/`missing-selection` in the owner join, one with no `owner` or with a `SourceOwner` and no `declaration` fails the V2 schema (`malformed_wire`), and both V2 schemas enumerate the form in the owner-presence rule, the model forms and the `ModelDeclarationNode` preimage. | Test |
| FR-322-AC-69 | With record value types `Money` and `TaxedMoney` (supertype `Money`) and `GiftTaxedMoney` (supertype `TaxedMoney`) of one selected document, a `quire.op.function.call` argument (the parameter read from the callee's body `parameters` binding) or an operation-call argument of type `TaxedMoney` or `GiftTaxedMoney` at a `Money` parameter is admitted with the argument taken by `ref` and no conversion node; a `Money` argument at a `TaxedMoney` parameter, an argument of an unrelated record, and a `Sequence<TaxedMoney>` argument at a `Sequence<Money>` parameter each refuse `ill_typed`/`operator-ineligible`, as does an argument whose count differs from the parameters' (a callee with no `parameters` binding takes none). A `function` node typed `Money` whose `body` binding references a root expression node typed `TaxedMoney`, and a `case` whose arm bodies are typed `Money` and `TaxedMoney` with result type `Money` (FR-440-AC-4), are admitted; a `Money` body root at a `TaxedMoney` function, and an unrelated record (`Voucher`) body root at a `Money` function, each refuse `ill_typed`/`type-mismatch` at the function node. A `quire.op.record.project` over a `GiftTaxedMoney` or `TaxedMoney` operand whose `field` member names `Money` as `declaration` reads `Money`'s field with the type `Money` gives it, and one naming `GiftTaxedMoney` over a `TaxedMoney` operand refuses `ill_typed`/`operator-ineligible`. `quire.op.collection.contains` over a `Set<Money>` and a `TaxedMoney` operand, and `quire.op.collection.set` over `TaxedMoney` rest arguments with a `Set<Money>` result, are admitted, and a `Set<TaxedMoney>` operand of a `Set<Money>` slot refuses. A `value`/`union_value` node whose `Money` payload position holds a `TaxedMoney` term is admitted. A `quire.op.structural.eq` or `quire.op.structural.ne` over two `Money` operands is admitted with `Money`'s leaves, including when the values are `TaxedMoney` values whose subtype-only text field has no listed leaf. | Test |
| FR-322-AC-70 | A `function` node whose body is an `aggregate` with the first binding `parameters`, an `aggregate` of `reference` terms to `value`/`parameter` nodes in parameter order, (QSL FR-092 builds it, including a parameter the body never reads). A `function.call` application over that callee has arguments counted against that binding and typed against the `semantic_type` of its referenced parameter nodes; a callee without the binding takes none. | Test |
| FR-322-AC-71 | A selected domain package that declares the variant type `Tier` (cases `Gold`, `Standard`) and the object `Account` with field `tier: Tier` admits a self-typed `model`/`variant_type` declaration node owned by `Tier`, with a `ModelOwner` and the closed `ModelDeclarationNode` preimage, family `enum`. `Account.tier` derives the `Tier` node as its type, an `optional` field `Option<Tier>` and an ordered non-unique `[0, 3]` field `Sequence<Tier>[0, 3]`, never `Reference<Tier>`. `quire.op.enum.eq` and `quire.op.enum.ne` over two `Tier` operands are admitted; `quire.op.structural.eq` over them, and an ordering application (`quire.op.enum.lt`, `le`, `gt` or `ge`), refuse `ill_typed`/`operator-ineligible`. An `enum_value` node whose `semantic_type` is the `Tier` node and whose case is `Gold` or `Standard` is admitted, and one whose case is not a case of `Tier` in the selected document refuses `invalid_semantic_graph`. The node key is a function of the selected document and the node alone: two units that select the package under different model aliases derive one `Tier` node key and one key for each case. A `model`/`variant_type` node owned by a record value type, an object type or a node absent from the selected document refuses `missing_declaration`/`missing-selection` in the owner join, and one with no `owner` or with a `SourceOwner` and no `declaration` fails the V2 schema (`malformed_wire`); both V2 schemas enumerate the form in the owner-presence rule, the model forms and the `ModelDeclarationNode` preimage. | Test (TC-940) |
| FR-322-AC-72 | A selected domain package that declares the process `Flow` (`quire.meaning.model.process/v1`, field `n`, operation `go`) and the state machine `Gate` (`quire.meaning.model.state-machine/v1`, field `open`, operation `toggle`) admits a self-typed `model`/`process` declaration node owned by `Flow` and a self-typed `model`/`state_machine` declaration node owned by `Gate`, each with a `ModelOwner` and the closed `ModelDeclarationNode` preimage, family `object`; `Flow.n` and `Flow.go` resolve through step 3 as an object type's members do, and `Reference<Flow>` and `Reference<Gate>` derive `composite_type`/`reference` nodes over the two declaration nodes. A `model`/`persistence_interface` node owned by a declared persistence interface passes the owner join. A process node owned by an object type, a state machine owned by a process, or either owned by a node absent from the selected document refuses `missing_declaration`/`missing-selection` in the owner join, and either carrying no `owner` fails the V2 schema (`malformed_wire`); both V2 schemas enumerate the three forms in the owner-presence rule and the `ModelDeclarationNode` preimage. | Test (TC-233, TC-280) |
| FR-322-AC-73 | A `quire.op.model.reaches` application whose `relationship_end` member carries `population`, the id of a `bounded_domain`/`model_population` node over `Set<Reference<Order>>` where `Order` is the type of the end the member's role names, validates against both V2 schemas, is keyed by an application preimage that includes the key, lists the population node in its `dependencies`, and is admitted by the full reader, for a `max` of `3` and of `0`. The same application without the key keeps the node id it has in `positive-operation-identities.json`. `population` naming a `bounded_domain`/`integer_range` node, a node absent from the graph, or a `model_population` node over another model declaration refuses `ill_typed`/`operator-ineligible`; a `dependencies` list without the population node is refused `invalid_semantic_graph`; the published `adverse.json` `reaches_population_cases` record each. | Test |
| FR-322-AC-74 | A `bounded_domain`/`model_population` node over `Set<Reference<Order>>` whose body is the empty `aggregate`, with the `max` member absent, is the unbounded population: it validates against the V2 schema and is admitted by the full reader, as the `population` of a `quire.op.model.reaches` application, with the key PO0 (`f65a8239f4fbc1b6e75bfeed4a578e1c219a27f155a82ddfb7aeeb22657c3b8b`) recomputed from its hand-written RFC 8785 preimage and distinct from the key PO1 of the same node with `max` `3`; the node with `max` `3` stays admitted as a bounded population. A `max` of `-1`, a `max` that is not an integer, and a second member beside `max` each refuse `invalid_package`/`invalid-derived-shape` at the population node, and the key PO0 differs from the key (`0ade8ed9baf129e82dee28096d64f5a0deabca6f3aa06874a4a7e76bb9688e34`) of the same node with `max` `0`. The published `adverse.json` `unbounded_population_cases` record each. | Test |
| FR-322-AC-77 | The lock of a package whose unit has a non-empty refinement internal list carries `refinement_internals` in canonical order and the member is absent when every list is empty; the member is in no identity preimage, and two compilations of the same sources, one with a non-empty list and one without, recompile to different `package_id`s through the refinement nodes' `abstract_internal` alone. A member that is `[]` or `null`, an object with an empty `internal` or a third key fails the schema at that value; objects or spellings out of byte order refuse `invalid_package`/`invalid-value` at the later entry's pointer; a repeated `refinement` or spelling refuses `invalid_package`/`conflicting-definition` with both entries as loci. | Test (TC-233) |
| FR-322-AC-78 | The cause `unsupported-wire` is paired only with `unknown_wire` and `unsupported-feature` only with `unknown_required_feature`, in the FR-272 rows and the `Diagnostic` schema alike, and the FR-272 rows of `unknown_contract_version` and `unknown_required_capability` list no cause. | Test (TC-233) |

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

## References

- Tickets: Linear QSL-290, QSL-638 (the node `owner` member), QSL-642
  (integers in a node-key preimage), QSL-659 (record-valued model fields) and
  QSL-692 (settled derived type forms for StateModel lowering).
