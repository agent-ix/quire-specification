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
  - target: ix://agent-ix/quire-specification/FR-440
    type: references
  - target: ix://agent-ix/quire-specification/FR-441
    type: references
  - target: ix://agent-ix/quire-specification/FR-043
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
  - target: ix://agent-ix/quire-spec-language/FR-093
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
  `composite_type` node with `semantic_form: "union"`, a `value` node with
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
  member naming one operator the law's profile defines (the FR-052 bounded
  control constructs, or for a claim the temporal operator it applies), or for
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
  node's `dependencies` are exactly the unique digest-ascending `reference`
  term targets and member declarations of its body; a `dependency_reference`
  contributes none. A contradictory form or
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
  re-derives the keys of two sets of nodes by QSL FR-092's
  `quire.structural-node/v1` preimage, so that a consumer reading a range,
  bound or element type from such a node reads the content its key names. The
  **derived shapes** are the undeclared nodes outside every recursion group
  of `scalar_type` `boolean` and `integer`, `composite_type` `reference`,
  `option`, `set`, `bag`, `sequence` and `ordered_set`, and `bounded_domain`
  `integer_range` and `collection_bounds`. Such a node is current when its
  key is the JCS SHA-256 of its preimage (`owner` absent, `recursion` `null`,
  `semantic_type` `null` exactly when the node is its own type) and, for a
  `scalar_type` or `composite_type`, when its `semantic_type` is the node
  itself. The **in-group structural nodes** are every node that carries a
  `recursion_group`, has no `nominal_identity_preimage` and whose body holds
  no application, declared or not and of any shape. The members of a group
  are the nodes carrying its label, and a member's `ordinal` is its position
  among them in graph order. Such a node is current when its key is the JCS
  SHA-256 of its QSL FR-092 group-local preimage (`recursion`
  `{size, ordinal}`, each body `reference` to a member and a `semantic_type`
  naming a member written `{term: "group_reference", ordinal}`, `owner` from
  the node's `declaration` source occurrence when it is declared) with
  `recursion.group` set to its retained `recursion_group`. The stage runs two
  passes. The first visits both sets together, ascending by retained node-id
  digest, and refuses the first node that is not current. The second
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
  fails the first pass at a member whose ordinal changed. A conformant package carries
  on every node of both sets the key QSL's FR-092 key function mints.
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
`unsupported_construct` and `wrong_snapshot`. The `stale-node-key`, operation and declaration causes
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
   `quire.model.complete/v1` rules. The read's refusal is the first refusal
   in FR-154's report order, with its own code, cause and loci, at stage
   `package_read`. The caller supplies `ModelNormalizationLimitsV1` among the
   reader's limits, and an exhausted one is the read's `incomplete` result.
2. **Owner recovery.** For each admitted selection's `identity` and
   each object type and relationship declaration of its document, the reader
   computes the declaration's model declaration node key: the JCS SHA-256 of
   the closed `ModelDeclarationNode` preimage of
   `node-identity-preimage.schema.json`, `{version:
   "quire.structural-node/v1", node_tag, semantic_form, semantic_type: null,
   declaration: null, recursion: null, owner: {kind: "model", identity,
   node}, body: {term: "aggregate", members: []}}`, where `node` is
   the declaration's IR node identity. The key depends only on content: the
   domain package's declared version label is not part of it. `node_tag`/`semantic_form` is
   `model`/`object_type` for an object type with no `interfaceFeatures`,
   `model`/`systems_interface` for an object type with `interfaceFeatures` and
   `relation`/`relationship` for a relationship. Each key computed charges
   one validation visit. The declaring node's `ModelOwner` is the owner whose
   key equals the node's `node_id` digest and whose preimage `node_tag` and
   `semantic_form` equal the node's, and its declaration `T` is the one that
   key was computed from. The preimage's (`identity`, `node`,
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
   | a value type of the same domain package bound as `Text[min, max; profile]`, `0 <= min <= max`, with `profile` one of FR-141's `unicode-scalars`, `nfc`, `nfd`, `nfkc`, `nfkd` and `binary-utf8` | `Text[min, max; profile]`, whose length, bounds and comparison are FR-141's under `profile` | `bounded_domain`/`text_bounds` with bounds `min` and `max` and `text_profile` `profile` over the `scalar_type`/`text` node |
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

### Reaches over a field

`quire.op.model.reaches_field` is `reaches(a, b, edge)` over a field edge:
operator class `reaches`, operands `reference` and `reference`, result
`boolean`, no laws and no mode, member `field`
(`{kind: "field", declaration, name}`, the same member a field projection
carries) and the `reference_edge` constraint over operands 0 and 1.
`quire.op.model.reaches` is the same predicate over a relationship end. At
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
   ("Reference conformance"; step 2 recovers `B`'s owner with its
   refusals).

Any other operand, and a field of any other type (a set, bag, ordered set,
unbounded sequence, a reference to another object type, or a non-reference
type), refuses `ill_typed`/`operator-ineligible`. Traversal follows
[FR-043](../../functional/foundation/FR-043-evaluate-finite-graph-relations.md).

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
from a node when some sequence of field, position, member payload and
element steps leads from it to a text type. This is the recursion leaf of
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

## Operation families, groups and constraints

This section is normative; the catalog file is its machine-readable copy and a
disagreement between them is a catalog defect, not a reader choice.

A type node's **family** is read from the type the node resolves to through its
`bounded_domain` base chain: `scalar_type` forms `boolean`, `integer`,
`rational`, `decimal`, `float32`, `float64` and `text` are their own family, a
`unit` is `quantity`, an enum is `ordered_enum` when its nominal preimage is
ordered and `enum` otherwise; `composite_type` forms `option`, `record`,
`tuple`, `union`, `sequence`, `set`, `bag`, `ordered_set` and `reference` are
their own family, and a model declaration node whose `semantic_form` is `object_type` or
`systems_interface` is `object`. A `relation` node and a `model` node of any
other form have no family, so a term they type fits no operand position. An
`object` term is admitted only at a `field_owner` operand position and at
the `object` anchor position of `quire.op.state.clause`: `object` is a
member of no other group, so the `inner:0` result of
`quire.op.model.deref` is read only through a field projection ("Model-owned
members"), and a reference to an object type or to a `state`/
`operation_anchor` node (whose `semantic_type` is its context object type)
is admitted only as a state clause's anchor. The `type_argument` member of `quire.op.reference.upcast` names a
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
| `any_value` | `boolean`, `integer`, `rational`, `decimal`, `float32`, `float64`, `text`, `enum`, `ordered_enum`, `quantity`, `option`, `record`, `tuple`, `union`, `sequence`, `set`, `bag`, `ordered_set`, `reference` | An operand position admits a term whose family is a member. |
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
`text`, `enum`→`enum` or `ordered_enum`, `none`→`option`). Any failure is
`operator-ineligible`.

| Constraint | Decision rule |
| --- | --- |
| `same_family` | The two named operands have equal families. |
| `same_type` | The two named operands resolve to the same type node. |
| `conforming_reference` | The two named `reference` operands conform: they resolve to the same type node, or their object types conform one to the other as "Reference conformance" below decides. |
| `reference_edge` | The `field` member is a reference edge between the two named `reference` operands, as "Reaches over a field" above decides. |
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
| FR-322-AC-10 | Enum, dimension and declared-unit node keys match the closed JCS preimage vectors; stale-body keys, non-normalized rationals, invalid owner joins and every other noncanonical member or term order this artifact refuses -- including an application node's `dependencies` list -- refuse as `invalid_semantic_graph`, except a `state`/`frame` node's own `modifies`/`creates`/`deletes` canonical order, which [FR-340-AC-4](FR-340-checked-package-frame-location.md) owns. | Test (TC-217) |
| FR-322-AC-13 | The published V2 checked-package schema admits its positive fixture carrying all 13 closed node tags with an exact source-map entry for every occurrence, and refuses each authored document-type, digest-domain and node mutation. | Test (TC-233) |
| FR-322-AC-14 | Each published V2 package fixture's `package_id` equals the RFC 8785 JCS SHA-256 of the identity preimage rebuilt from its lock selections and occurrence-free semantic graph; editing a source-map region, node source occurrence, raw source digest, capability disposition or diagnostic catalog leaves that id unchanged, while editing the edition, a profile/definition/model/dependency selection, a required feature, a node member or node order changes it. | Test (TC-233) |
| FR-322-AC-15 | Each published enum, dimension and declared-unit node vector validates against the closed preimage schema and recomputes its recorded JCS SHA-256; each authored semantic mutation is refused by exactly its named schema or semantic check, an absent owner fails the package lock join, and inside the published nominal package a retained preimage with a changed body, type, dependency or unjoined owner is refused by the published cross-field or owner-join constraint. | Test (TC-233) |
| FR-322-AC-17 | The published operation catalog is closed: both V2 schemas carry identical operation definitions and enumerate exactly its identities, law roles, mode kinds and values, member kinds and operator classes; every law-role `DefinitionRef` names a complete-value lock definition by authority and identity; every entry's operands, rest, result form, laws, mode, member, leaf source and constraints use only catalogued families, groups and kinds; integer `div`/`rem` require the integer-division law and `mod` none, IEEE operations require the IEEE profile and have no `eq`, `ne`, ordering or `negate` identity, text operations require the text profile and text mode, `all_instances` takes one population, `lookup` requires an absence mode, `navigate` and `reaches` a relationship end, `reaches_field` a `field` member and the `reference_edge` constraint, `quire.op.state.clause` the `state_clause` member kind and no law, and the record, tuple and index projections, numeric/total-order/bit-identical IEEE comparisons, `deref` and `dispatch_call` are present. | Test (TC-233) |
| FR-322-AC-18 | Each published operation vector validates against the closed node-preimage schema, recomputes its recorded JCS SHA-256 and is distinct from every other vector; every application node of the published operation package is keyed by its application-node preimage and is a published vector, and a vector outside the package refuses as `operation-law-unselected` when placed in it; floor-to-Euclidean division changes the node and package ids; a retained key refuses as `stale-node-key`; a contradictory form, missing member-declaration dependency, stray recursion group or extra dependency is refused; an application without an operation fails the V2 schema; and the all-families function body calls a published function, changes its node id when rewritten as `dispatch_call` and then refuses as `operation-member-mismatch`. | Test (TC-233) |
| FR-322-AC-19 | Each published operation mutation never keeps its stale key and either rekeys to exactly its named counterpart vector and is admitted exactly when its laws are lock-selected, refuses as `operation-law-unselected` until the lock also selects its law and again under the stale selection, or is refused by exactly its named schema or semantic check with its recorded cause and that cause's code; together the mutations exercise every operation refusal cause. | Test (TC-233) |
| FR-322-AC-21 | Every node of each published V2 package carries a `declaration` member exactly when it declares; the schema refuses a declaring node without it and an expression with it; the member enters the application-node preimage and package id; a name duplicated on two nodes refuses as `ambiguous-name` at the node later in node-id digest order; and a nominal node whose declaration differs from its `qualified_declaration` refuses as `declaration-nominal-mismatch` before any ambiguity. | Test (TC-233) |
| FR-322-AC-22 | Every clause application of the published clause package carries exactly one profile law of its catalogued role equal to the lock's `profile_selections` row, and every clause application other than `quire.op.temporal.clause` (FR-370) a `profile_operator` member the profile defines; removing the law refuses as `operation-law-missing`, a protocol profile in the temporal role as `operation-law-mismatch`, a lock without the temporal profile row as `operation-law-unselected`, and an undefined, other-profile or absent operator as `operation-member-mismatch`; for a `quire.op.temporal.clause` law naming no FR-250 profile, the full reader reports FR-370's earlier profile check (`unknown_profile`) before the operation step's `operation-law-mismatch`. | Test (TC-233) |
| FR-322-AC-23 | Every application carries a `result_type` and every literal a `type`; a root `result_type` differing from the node `semantic_type`, a `result_type` the operation cannot produce, and a literal whose `type` family its `value_kind` does not admit each refuse as `operator-ineligible`, while the same literal with its declared type is admitted. | Test (TC-233) |
| FR-322-AC-24 | The FR-149 conversion classification is decided from declared bounds: an integer range into a containing integer or rational range is exact, an integer range whose scaled bounds exceed a decimal range is range narrowing, a decimal of scale 3 into scale 2 is scale reduction and a rational into narrower rational bounds is rational narrowing; `quire.op.rational.div` over integer-range operands carries `promotes_exact` and its published vector is admitted, while a decimal operand refuses as `operator-ineligible`. | Test (TC-233) |
| FR-322-AC-25 | An `operation` member resolves only against a declaring `model` node of `semantic_form` `object_type`, `process`, `state_machine` or `persistence_interface`; every other model form, including `systems_interface`, refuses `operator-ineligible`/`ill_typed`. | Test (TC-233) |
| FR-322-AC-26 | A reference to a dependency package's node is carried as `PackageNodeKey{package, node}`, where `node` is a `WireNodeId`; per R-10 it becomes a `NodeKey` only by lookup in a checked package compiled from digest-addressed source, never minted directly from wire bytes in either the dependency's or the importing package's namespace, and a reader resolves it only against the named dependency's admitted package, refusing a `package` absent from `dependency_selections` as `missing_declaration`/`missing-selection`. | Analysis |
| FR-322-AC-27 | An unrecognized `node_tag` refuses as `unsupported_node_tag`, and QSpec revises the closed `node_tag` set in place. | Analysis |
| FR-322-AC-28 | A `ModelOwner` node-identity preimage carries the declaring domain package's `identity` beside `node` and no version label, so a node id depends only on content; the published `ModelOwner` vectors validate against the closed preimage schema and recompute their recorded JCS SHA-256, and a `ModelOwner` preimage that carries a `version` member is refused by the closed preimage schema. | Test (TC-279) |
| FR-322-AC-29 | Each published model declaration node vector validates against the closed `ModelDeclarationNode` preimage schema and recomputes its recorded JCS SHA-256. A reader recovers a declaring node's `ModelOwner` only from a lock-selected domain package with the node's own tag and form, so `Order`'s node keyed under an unselected domain package, and the relationship node's id carried by a `model` node, refuse `missing_declaration`/`missing-selection`. A matched node whose `semantic_type` is not its own id, or that carries a `declaration` or a `recursion_group`, or whose body has members, refuses `invalid_package`/`stale-node-key`. | Test (TC-280) |
| FR-322-AC-30 | A model-owned `field` or `operation` member resolves among the declaring type's exposed effective members, own and inherited. A field declared by a supertype resolves on the subtype's node, and a redefined operation resolves to the most derived redefinition. An operation on a `systems_interface` node refuses `ill_typed`/`operator-ineligible`. Two exposed fields of one name refuse `ambiguous_declaration`/`ambiguous-name`. An undeclared name, a name of the other member kind and a relationship declaring node each refuse `ill_typed`/`operator-ineligible`. | Test (TC-280) |
| FR-322-AC-31 | A resolved model member's type follows the declared-type, multiplicity and presence tables. The derived types are `Int[0, 9]` from an integer value type, `Integer` from a constrained native `Integer`, `Option<Boolean>`, `Set<Integer>[0, 3]`, `Sequence<Integer>[0, 5]`, `Set<Integer>[0, 1]`, `Set<Integer>` for `[0, unbounded]`, `Reference<Invoice>` and an operation's `Integer` result. A `Text` field, a `[1, unbounded]` field and an operation with no result each refuse `ill_typed`/`operator-ineligible`. So does a call whose receiver is not a reference to the declaring node, or whose arguments differ in number or type node from the operation's parameters. | Test (TC-280) |
| FR-322-AC-32 | A lock-selected domain package document is checked as evidence before graph admission: a document supplied under the selected digest with matching identity passes the selection admission checks, while a digest domain other than `sha256-jcs` refuses `stale_dependency`/`digest-domain-mismatch`, no document refuses `missing_import`/`missing-selection` and a document whose identity differs from the selection `invalid_model_binding`/`wrong-model-selection`. | Test (TC-280) |
| FR-322-AC-33 | `quire.op.reference.eq` and `quire.op.reference.ne` admit two references to selected object types of which one conforms to the other along declared supertypes: two of one type, a subtype and its supertype in either operand order, a subtype two levels down, and a type with two supertypes against either. A systems interface with no declared supertype against an unrelated object type, two unrelated object types, a reference to a relationship, even compared with itself, and an operand of family `object` refuse `ill_typed`/`operator-ineligible`. An operand whose owner is not selected refuses `missing_declaration`/`missing-selection`, even compared with itself. | Test (TC-281) |
| FR-322-AC-34 | `quire.op.record.project` over a `quire.op.model.deref` result reads a field of a model object: the `deref` result has family `object`, the projection resolves the field on the declaring object type, own or inherited, and its result is the field's member type. A projection over a reference that is not dereferenced, a `field` member naming a node other than the operand's object type, a `deref` whose `result_type` is not the operand's object type, a `deref` result of a relationship, and a projection whose `result_type` is not the field's member type refuse `ill_typed`/`operator-ineligible`. | Test (TC-281) |
| FR-322-AC-35 | A `dependency_selections` entry is a closed `DependencySelection` `{identity, package_id}` whose `package_id` is a `quire.package.semantic/v2` `PackageId`: the published package carrying two such entries, one per identity in ascending identity order in both its lock and identity preimage, is admitted by the V2 schema and its recorded `package_id` recomputes from them, and changing a dependency's `package_id` changes it; an entry with a `Selection` or `DefinitionRef` shape, no `package_id`, a bare-digest `package_id`, a `package_id` in another digest domain or an extra `version` member fails the schema in either member, at the mutated entry, and a `model_selections` `ModelRef` entry carrying an extra `version` member likewise fails it at that entry; a reader refuses either `version`-bearing entry `unknown_member`, since a selection binds by identity and digest and never by version. Entries ordered by UTF-8 bytes where UTF-16 code-unit order differs are admitted; a second entry of one identity refuses `invalid_package`/`conflicting-definition` with both entries as loci, and an entry not strictly after its predecessor refuses `invalid_package`/`invalid-value` at its pointer. | Test (TC-233) |
| FR-322-AC-36 | A `{term: "dependency_reference", package, node}` term whose `package` is a `quire.package.semantic/v2` `PackageId` and whose `node` is a `NodeId` is admitted by the V2 schema as an application argument and by the node-identity preimage schema as a preimage term; a `package` given as a bare digest or in another digest domain, a missing `node` and an extra member each fail both schemas. A `quire.op.function.call` application-node preimage whose callee is a `dependency_reference` has a different JCS SHA-256 when only the reference's `package` digest changes, and a different one again when only its `node` changes. | Test (TC-233) |
| FR-322-AC-37 | A package whose function calls a dependency `function` declaration over `Int[0, 9]` returning `Boolean`, through a `dependency_reference` callee, with the Boolean node as `result_type` and no `dependencies` entry for the target, is admitted when the dependency's admitted package is supplied. With no dependency package supplied it refuses `missing_import`/`missing-selection` at the selection entry; with the callee's `package` absent from `dependency_selections`, `missing_declaration`/`missing-selection`; with `node` naming a node that carries no `declaration`, `missing_declaration`/`missing-name`; with `node` naming a declared record, a function returning a declared record, a function over `Set<R>` or over a tuple holding `R` for a declared record `R`, a function over `Reference<M::T>` for a model type `M::T`, or with the term as a non-callee argument or as a node body root, `ill_typed`/`operator-ineligible`; with the target listed in `dependencies`, `invalid_semantic_graph`. | Test (TC-233) |
| FR-322-AC-38 | `quire.op.model.reaches_field` admits a field edge of the owner's own reference type: `Order.parent` (`Option<Reference<Order>>`), `Order.prior` (`Sequence<Reference<Order>>[0, 3]`) and `Order.origin` (`Reference<Order>`), the inherited `parent` read on `Sub`, and a target of a subtype of `Order`. A set edge, an unbounded sequence edge, an edge to another object type, a non-reference field, a target of an unrelated type, an operand 0 of a type other than the member's declaring node, an undeclared field name and an operand of family `object` refuse `ill_typed`/`operator-ineligible`; an ambiguous field name refuses `ambiguous_declaration`/`ambiguous-name`, and a declaring node keyed under an unselected domain package version `missing_declaration`/`missing-selection`. | Test (TC-281) |
| FR-322-AC-39 | Every node body of every published V2 package and every node-identity vector conforms to the five-stratum body grammar, and the V2 schema and the node-identity preimage schema each admit exactly that grammar. | Test (TC-427) |
| FR-322-AC-40 | A body carrying an application inside an application's arguments, an application or a binding of an aggregate inside an aggregate's members, or an aggregate inside a Group's members fails the V2 schema and is refused `malformed_wire`, and the same meaning with each composite subterm as its own referenced node is admitted. | Test (TC-427) |
| FR-322-AC-41 | A package holding a 100,000-deep expression has the same JSON nesting depth as a package holding a one-level expression, is emitted, read back and admitted under node, edge and byte limits sized for its node count, and no reader limit or effective-limit record names a depth. | Test (TC-427) |
| FR-322-AC-42 | A package whose lock names source and definition digests is admitted with no source or definition bytes and no digest evidence supplied. | Test (TC-427) |
| FR-322-AC-43 | A domain-package document supplied under its selection's digest with the selection's identity, but altered so that the SHA-256 of its RFC 8785 bytes differs from that digest, refuses `stale_dependency`/`content-mismatch` naming the selected and the recomputed digest, before its identity is compared and before any declaration is read; the unaltered document passes. | Test (TC-280) |
| FR-322-AC-44 | A field typed by a value type bound as `Text[min, max; profile]` has type `Text[min, max; profile]` over a `text_bounds` node; a value type bound as `Text` with a negative `min`, with `min > max` or with a profile outside FR-141's six gives the member no type and refuses `ill_typed`/`operator-ineligible`. | Test (TC-280) |
| FR-322-AC-45 | `quire.op.structural.eq` over two `Label` values of `union Label { Named(Text[0,8;nfc]), Tagged(Integer, Text[0,8;nfc]), Empty }` carries exactly the leaves `["member:Named", "position:0"]` and `["member:Tagged", "position:1"]` in that order, each with the `nfc` text law and mode, and none for `Empty`; both V2 schemas admit the `member:<Ident>` segment. | Test (TC-395) |
| FR-322-AC-46 | `quire.op.structural.eq` over two values of `union List { Cons(Integer, List), Nil }`, whose cycle names one `recursion_group`, is admitted with empty `leaves`, and a record whose field's type cycles back to the record through a union is likewise admitted; neither refuses `operator-ineligible`. Over `Node`, `Cell`, `Chain` and `Option<Chain>` of the "Structural leaf walk" examples it is admitted with exactly the listed leaves in that order, each recursion leaf with `laws` `[]` and `mode` `null`; both V2 schemas admit the `recursion:<d>` segment. | Test (TC-395) |
| FR-322-AC-47 | The V2 `Diagnostic` schema admits `type-mismatch` only with `ill_typed`, `missing-selection` only with `missing_import` or `missing_declaration`, and `unsupported-selection` and `wrong-selection-role` only with `unknown_language`, `unknown_edition` or `unknown_profile`. | Test (TC-233) |
| FR-322-AC-48 | A published package whose undeclared, ungrouped derived-shape node is tampered, with its `node_id` kept, its `identity_projection` entry patched the same way and its `package_id` recomputed, is schema-valid and identity-consistent and refuses `invalid_package`/`stale-node-key` at that node's `node_id`: an `integer_range` `max` narrowed to `2` and widened to `5000`, its `min` raised to `1` and its bounds swapped, a `reference` target re-pointed at the `Integer` node and a `set` element re-pointed at the `Integer` node. The untampered package is admitted. | Test (TC-233) |
| FR-322-AC-49 | The recursive-records package (`record List { next?: List; }` and `record Tree { kids: Sequence<Tree>[0, 3]; }`, each one recursion group keyed by QSL FR-092) is admitted. With one in-group node tampered, its `node_id` kept, its `identity_projection` entry patched and its `package_id` recomputed, it refuses `invalid_package`/`stale-node-key` at that node's `node_id`: `Tree`'s `collection_bounds` `max` widened from `3` to `1000` and its `min` raised to `1`, `Tree`'s `Sequence` element re-pointed from the record at the `collection_bounds` node, and `List`'s `Option` re-pointed at the `Integer` node. The `Tree` group relabelled with a label that is not its group digest, each member rekeyed under that label and every reference rewritten, refuses `invalid_package`/`stale-node-key` at the group member with the lowest node-id digest. | Test (TC-233) |
| FR-322-AC-50 | Every node of every published positive fixture that is a derived shape or an in-group structural node carries the key QSL's FR-092 key function mints for it, and no placeholder key; the recursive-records fixture's group labels are its groups' digests; and the full reader admits every published positive fixture. | Test (TC-233) |

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

- Tickets: Linear QSL-290.
