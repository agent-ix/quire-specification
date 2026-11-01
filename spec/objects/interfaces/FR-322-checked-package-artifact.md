---
id: FR-322
title: "Immutable checked-package artifact"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_004
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-468
    type: references
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
  - target: ix://agent-ix/quire-specification/FR-481
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
  - target: ix://agent-ix/quire-spec-language/FR-391
    type: references
  - target: ix://agent-ix/quire-specification/FR-451
    type: references
  - target: ix://agent-ix/quire-specification/FR-307
    type: references
  - target: ix://agent-ix/quire-specification/FR-460
    type: references
  - target: ix://agent-ix/quire-specification/FR-461
    type: references
  - target: ix://agent-ix/quire-specification/FR-469
    type: references
  - target: ix://agent-ix/quire-specification/FR-415
    type: references
  - target: ix://agent-ix/quire-specification/FR-368
    type: references
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
  - target: ix://agent-ix/quire-specification/FR-429
    type: references
  - target: ix://agent-ix/quire-specification/FR-417
    type: references
  - target: ix://agent-ix/quire-specification/FR-438
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
  - target: ix://agent-ix/quire-specification/FR-433
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
- `name` — the package identity: a non-empty string, the library identity an
  importer names, the domain package identity or the compilation unit's own
  identity. Two identities are equal only when their UTF-8 bytes are equal. A
  checked package is identified by its `name` and its declared contents compared
  structurally, never by a value derived from those contents: two checked
  packages are the same package exactly when their JSON values are equal member
  by member and, for arrays, element by element in order. No identifier, hash,
  digest or seal of a package exists.
  The lock has no model section and no `ModelRef`: a domain package the source
  selects (FR-154 "Model selection in source") is recorded by its `model_root`
  node alone, whose closed owner is `{kind: "model_root", identity}`. No lock row,
  digest or version exists for a model, and the host supplies
  the document under the identity at read and replay.
- `lock` — exact source-document, edition, ordered profile, dependency and
  required-feature identities. A source-document entry is `{authority, identity}`
  and carries nothing else: no byte digest of the source exists anywhere in the
  package. A definition selection is a `DefinitionRef` `{authority, identity}`.
- `dependency_selections` — the libraries the unit imports directly
  ([FR-307](../../functional/tooling/FR-307-package-reusable-semantic-libraries.md)):
  one `DependencySelection` per directly imported library identity, each
  `{identity}`, in strictly ascending UTF-8 byte order
  of `identity`. `identity` is the library identity an import names. A library
  identity is a non-empty string with no segment structure: `/`, `.` and
  every other character are ordinary bytes, and two identities are equal
  only when their UTF-8 bytes are equal. A second entry whose `identity` equals
  an earlier entry's refuses `invalid_package`/`conflicting-definition`
  (FR-307's diamond cause), with both entries as loci. Otherwise an entry whose
  `identity` is not strictly after its predecessor's in UTF-8 byte order refuses
  `invalid_package`/`invalid-value` at that entry's pointer. A
  `dependency_reference`'s `package` equals one direct entry's `identity`. A
  supplied dependency package binds to that entry when its `name` equals the
  entry's `identity`; the host supplies exactly one package per identity. A
  reader admitting a package with a non-empty `dependency_selections` is supplied
  each selected dependency's admitted package; an entry with none supplied
  refuses `missing_import`/`missing-selection` at that entry's pointer,
  before any `dependency_reference` check. Supply is checked entry by entry in
  lock order (ascending identity). The reader reports the first failing entry in
  lock order and no later one, so one refusal names one entry. The importing
  package's library closure is derived recursively from its direct entries and
  each selected dependency's derived closure. A dependency's derived closure is
  its own direct lock entries plus the derived closures of its supplied
  dependencies; an admitted dependency exposes that closure to its importer. No
  lock records closure entries. If two paths through those locks reach the same
  library identity and the packages supplied under it are not structurally equal,
  they are two definitions under one identity and the reader refuses `invalid_package`/`conflicting-definition` with both
  dependency paths as loci (FR-307). Each locus is one JSON pointer rooted at the
  importing package: `/lock/dependency_selections/{i}` selects a direct entry,
  and each further `/lock/dependency_selections/{j}` traverses the admitted
  package selected by the preceding entry. The final entry is the conflicting
  one. Listing direct imports only is the producer's canonical form; the reader cannot distinguish an over-listed transitive identity
  from a direct import and does not enforce that form.
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
  decided by the recompile (FR-432).
  Its effect on meaning is already in the refinement nodes' `abstract_internal`
  (FR-375).
- `semantic_graph` — one `quire.checked-semantic-graph/v2` closed tagged-node
  union. Its current tags are `scalar_type`, `composite_type`,
  `bounded_domain`, `value`, `expression`, `function`, `model`, `relation`,
  `state`, `temporal`, `protocol`, `claim`, `correspondence`, `extension`
  and `hybrid`. An `extension` node, of the one form `extension_form`, carries
  a selected extension's declaration form, with the extension's node kind as
  body data ([FR-469](FR-469-checked-package-extension-nodes.md)). A `hybrid`
  node, of the form `hybrid_model`, `hybrid_mode` or `hybrid_transition`,
  carries a checked hybrid model, one of its modes or one of its transitions
  ([FR-481](FR-481-checked-package-hybrid-model-nodes.md)). A node's identity is its
  `node_id`, a structural id (`Structural node identity` below); node
  identifiers are unique within the graph. **Graph order** is the order of the
  `semantic_graph.nodes` array, which the producer chooses. A reader neither
  sorts the nodes nor requires them in any sort order, so a graph in any order
  is admitted when every other check passes. Graph order is also the index an
  operand index segment holds ("Operand index segments" below), so reordering
  the nodes rewrites every index segment to its node's new index. Graph order is part of the package
  value that structural equality compares; a node's `ordinal` is producer-assigned
  and is not read from graph order (`Structural node identity` below). Where this
  artifact says that a reader checks nodes "in ascending node-id order", that
  phrase orders the reader's visits for choosing which refusal to report, by the
  componentwise node-id order below; it orders no array of the package. References name a
  compatible declared target by node id; a `state` node with
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
  node id of a `protocol` node in this same
  `semantic_graph.nodes` collection. It has one or more `expression`
  occurrences, one for each source reference to that target, and no other role;
  no `declaration`, `owner` or `nominal_structure`, a `recursion_group` only as
  "Carrier in a cyclic component" below states,
  the same `semantic_type` as its target, and `dependencies` containing
  exactly that target. A reader identifies the carrier by its tag and form,
  never by a name, a generic reference term in another node or a separate
  collection. A `dependency_reference` cannot be its body or its target;
  selected dependency semantics remain limited to the function-call callee
  described below. The reference body and target are part of the carrier's
  structural identity. Occurrences are not, so every source reference to one
  protocol shares the one carrier, identified once per target. The occurrences of a
  carrier have distinct ordinals, and each has its own `source_map` entry,
  keyed (`node_id`, `expression`, `ordinal`), naming that reference's exact
  source regions. A carrier without an `expression` occurrence, with an
  occurrence of any other role, or whose occurrences or `source_map` entries
  repeat an (`node_id`, `role`, `ordinal`) key is refused as
  FR-322-AC-66 states. The closed V2 schema fixes the carrier's body shape; after
  schema and graph-shape checks, the reader resolves its
  target by exact typed node id. An absent target refuses
  `missing_declaration`/`missing-name`; a present non-`protocol` target or
  unequal `semantic_type` refuses `ill_typed`/`operator-ineligible`, at the
  carrier's `/body/target` pointer. At the declaration step, before frame and
  operation checks, the reader visits carriers in ascending node-id
  order and reports the first that fails.
  The earlier graph-shape step compares `dependencies` with the body target
  without resolving that target; an unequal list is
  `invalid_semantic_graph` at `/dependencies`. Target existence, tag and type
  are checked only at the later carrier join. The carrier has no package
  selector: a key absent locally
  fails this join even if a selected dependency contains that key.
  **Carrier in a cyclic component.** A carrier is a node of the graph like any
  other and takes part in the cycle rule of the `structural` node forms
  ("A recursive reference is legal only when ..."). A predicate-join function
  that refers to its own protocol closes a cycle through the carrier (the
  protocol, the function, the carrier and the protocol again, with any
  further nodes between them). The carrier is then a member of that cyclic
  component, and it MUST carry the component's `recursion_group`, the label
  every other member carries: a producer SHALL write it and a reader SHALL require it. A carrier outside every cycle carries none from a producer, and a label
  on one is admitted and means nothing, as for every node outside every cycle.
  The closed V2 schema therefore admits `recursion_group` on the carrier and
  cannot say whether it is required, because only the graph shows a cycle; the
  reader decides, at the cycle check below. A carrier in a group keeps every
  other carrier rule unchanged: its body `{term: "reference", target}`, its
  `dependencies` (exactly the target), its `expression` occurrences and
  `source_map` entries, the one carrier per target, the carrier join and the
  visiting order of the declaration step, and the caller-funded limits. The
  target is written as the target's `node_id`, an ordinary `reference`, as for
  every reference to a member of a recursion group; the wire has no coordinate
  and no separate in-group reference form. Its `node_id` follows the group rule
  of "Structural node identity": the carrier is an anonymous derived member, so
  its `path` is the path of the group's declared member where the group has
  exactly one and a producer-chosen path otherwise, never derived from the
  target's id, and its `ordinal` is producer-assigned and unique among nodes
  sharing its `package` and `path`. The carrier's structural key is therefore its `node_id` with no index segment. The reference body and target remain part of the package value but not of a grouped carrier's id, as for every group member, so a body edit does not change that id. Because the id no longer carries the target, the reader enforces the one carrier per target that a derived id enforces by collision: a second carrier whose body target equals an earlier carrier's refuses `invalid_semantic_graph`, with no cause, at the later carrier's `/semantic_graph/nodes/{n}/node_id`, in the graph-shape step with the node-id checks and before the cycle check below (FR-322-AC-286). A carrier outside every group keeps the derived id below.
  For a worked admitted example, take `positive-all-families.json`; its
  `protocol`/`protocol_clause` node `Example::Handshake` is `P`, with the
  conforming FR-464 body. Add one
  carrier `R` with `node_tag: "protocol"`,
  `semantic_form: "protocol_reference"`, `body: {term: "reference",
  target: P.node_id}`, `semantic_type: P.semantic_type`,
  `dependencies: [P.node_id]` and one `expression` occurrence (a second
  source reference to `P` adds a second `expression` occurrence, ordinal 1, and
  its own `source_map` entry to the same `R`; it adds no node). `R`'s node id
  is the derived-node id `{package, path: ["protocol", "protocol_reference", P.node_id], ordinal: 0}`.
  Add `R` to `semantic_graph.nodes`
  and its occurrences to the source map (in ascending ordinal, which is not a
  requirement); the reader admits this package and resolves `R` to
  `P`. Replacing `R.body.target` and its sole dependency together with a local
  non-protocol node's id, or with an id absent from the graph, gives the respective typed refusals above. Adding the second occurrence to `R`
  leaves `R.node_id` unchanged.
  A `composite_type` node with `semantic_form: "union"`, a `value` node with
  `semantic_form: "union_value"` and an `expression` node with
  `semantic_form: "case"` carry the bodies
  [FR-440](FR-440-checked-package-union-and-case-nodes.md) specifies, and
  each union member is keyed as
  [FR-441](FR-441-union-member-key.md) specifies. A
  `value` node with `semantic_form: "parameter"` stands for a value a binder
  binds (a function parameter; a `let`, query, `count` or `sum` binder; a
  `fold` or `reduce` accumulator or element binder; a `forall` or `exists` binder over a clock
  collection, which binds the element place's slot position (or key), typed by the selector's
  domain ("Presence" below); or the `self`, `result`
  and operation parameters of a state clause; or a continuous variable of a
  hybrid model, [FR-481](FR-481-checked-package-hybrid-model-nodes.md)), and
  its `semantic_type` is the bound value's type. It carries no `declaration` and no `owner`, and its
  body is exactly `aggregate{[binding "name" = literal text, binding "level"
  = literal integer]}`, in that order: `name` is the binder's name as a
  `text` literal typed at the builtin text node, and `level` is the binder's
  level, its position among the binders in scope where it is bound (QSL
  FR-092), as an `integer` literal typed at the builtin Integer node. It is a
  derived node ("Structural node identity" below), so its id
  depends on its name, level and type and not on the function that binds it.
  A model member (field, operation, clause) has no node of
  its own: it is named by its declaring node and its name. A selected model
  operation in `checked_term_data.dispatch_tables` therefore uses the typed
  witness `{declaring: <eligible model NodeRef>, operation: <original member name>}`;
  the `declaring` node is an admitted `model` node whose `semantic_form` is
  `object_type`, `process`, `state_machine` or `persistence_interface`, and
  `operation` is its exact original member name. A relationship declaring node
  is ineligible. This witness is not a
  graph node id, and a reader or producer never infers either component from a
  name, path or synthetic operation node. A recursive reference is legal only
  when every cycle member names the same explicit `recursion_group`; all other
  graph cycles refuse. A `recursion_group` label on a node outside every
  cycle is admitted and means nothing: the label constrains cycles only, and
  no refusal is invented for an acyclic node that carries one.
  **Cycle check.** A **cyclic component** is a set of two or more nodes that
  reach one another, or one node that reaches itself, through the edges a reader
  follows when it builds a node's closure: a non-null `semantic_type` other
  than the node's own, each `dependencies` entry and each `reference` term
  target in the body (a self-typed node's absent `semantic_type` edge is not
  one; an own-path index segment is not an edge, its cycles refuse
  `malformed_wire` earlier, FR-322-AC-280). The reader checks every cyclic
  component in the graph-shape step, after the node-id, dependency-list and
  dangling-`reference` checks and before the owner join, over
  the edges whose two ends are nodes of the graph (an edge to an absent node is
  reported by the later join that owns it, and closes no cycle). The component's
  first member is its member lowest in ascending node-id order. The reader
  refuses `invalid_semantic_graph`, with no cause, at the first member in
  ascending node-id order, across all components, that carries no
  `recursion_group` or whose `recursion_group` differs from that of its
  component's first member (so an unlabelled first member is itself the one
  refused). The locus is `/semantic_graph/nodes/{n}` for a member with no
  label and `/semantic_graph/nodes/{n}/recursion_group` for a member whose
  label differs. The reader finds components with an iterative algorithm whose time is linear in the nodes and edges, with no native call-stack recursion, because a cycle may span any number of nodes. The check adds no limit: a group has no maximum size, the
  caller-funded limits of the reader apply to it as to every other node
  count, and the reader's order after the check, including the carrier join, is
  unchanged. A package whose graph has no cyclic component is read exactly as
  before. Each component is compared with its own first member only, so two
  disjoint cyclic components that reuse one label are both admitted: the
  label's nodes are one group by the rule above, which refuses nothing and
  changes no node id, and the "exactly one declared member" anchor rule of
  "Structural node identity" counts the declared members of all the nodes
  carrying that label, so a producer that reuses a label across components
  chooses the path of an anonymous member as it does for a group of several
  declared members.
  Every node has one or more exact source occurrences.
- `operation` — every `application` semantic term, wherever it occurs in a node
  body, carries `operator`, `operation: {identity, laws, mode, member,
  leaves}`, `result_type` (the node id of the package type the application
  denotes) and `arguments`; every `literal` term carries `type` (the node id of
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
  protocol clause, its declarations and its controls; for a
  `quire.op.state.transition` application the one operator `transition`,
  whose body [FR-341](FR-341-checked-package-state-clause-body.md) fixes; or
  for a claim the temporal operator it applies), or for
  `quire.op.temporal.clause` no member: a temporal clause's body, its
  `quire.op.temporal.*` operator identities and their `temporal_interval` and
  `fairness` members are
  [FR-370](FR-370-checked-package-temporal-clause-body.md)'s,
  or for `quire.op.state.clause` the `{kind: "state_clause", clause}` member
  naming the clause kind (`invariant`, `constraint`, `precondition` or `postcondition`,
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
  entry for every text leaf and every float leaf of the compared type
  (`operand:0`, `inner:0`) or, for `result_inner`, of the element type of a
  `set`, `bag` or `ordered_set` result (`field:<name>`, `member:<name>`,
  `position:<n>` and `inner` segments), a text leaf with the text profile law
  and the mode that leaf's type pins and a float leaf with the `ieee_profile`
  law and mode `null`, and one `{path, laws: [], mode: null}` entry for every
  recursion leaf, whose path ends in a `recursion:<d>` segment. IEEE values have no
  `eq`, `ne` or ordering operation: `quire.op.ieee.numeric_equal`,
  `quire.op.ieee.total_order` and `quire.op.ieee.bit_identical` are three
  distinct identities, and `quire.op.ieee.float32.negate` and
  `quire.op.ieee.float64.negate` are the sign-bit flip of one float of their
  width, with the law `ieee_profile` and mode `null` (the operation rounds
  nothing). A comparison reaching a float leaf is admitted: `=` over a bare
  float is `quire.op.ieee.numeric_equal` and `!=` is `quire.op.boolean.not`
  over it, and `quire.op.structural.eq` and `.ne` over a type holding a float
  at any depth list that float leaf (FR-149 "IEEE relations"). The
  one-occurrence collection forms of FR-145 have three identities:
  `quire.op.collection.sequence.drop_first` and
  `quire.op.collection.ordered_set.drop_first` take one operand of their own
  family (a `set` or `bag` operand has no first occurrence and refuses
  `operator-ineligible`) and have a `result_type` of that family, whose bound
  is the FR-145 derived bound; and `quire.op.collection.remove_one` takes one
  `collection_kind` operand and a value whose type node equals the operand's
  element type node (or, for a record element, conforms to it, by the
  `inner_type` constraint of `quire.op.collection.contains`), has a
  `result_type` of the operand's family, whose bound is the FR-145 derived
  bound, and lists the leaves of `inner:0` as `contains` does. None names a law, a mode
  or a member. A V2 reader refuses the first failing check in the order owned by
  `quire.native.diagnostics/v1`: `unknown-operation`,
  `operation-class-mismatch`, `operation-law-missing`,
  `operation-law-mismatch`, `operation-law-unselected`,
  `operation-mode-mismatch` and `operation-member-mismatch` (code
  `invalid_package`); then each argument application under the same order;
  then `operation-mode-type-mismatch` (`invalid_package`) and
  `operator-ineligible` (arity, operand family, constraint, member or result
  type outside the entry; code `ill_typed`); then the leaves.
- Application nodes — every `value` node with `semantic_form` `union_value` or
  `option_value`, and every other node whose body contains an application except
  a `relation`/`refinement` node, is identified by its structural `node_id`
  like every node. Its operation identity, laws, mode, member and leaves are
  part of its body and are compared structurally with the package value.
  `declaration` is the node's `declaration` member or `null`. For a `state`/`state_clause`,
  `operation.member.execution_point` is part of this body. For a
  `state`/`operation_anchor`, the optional fourth `execution_point` binding is
  part of its body. The schema admits only FR-341/FR-342's closed four-variant
  `ExecutionPoint`; omitting a point is distinct from carrying any variant. The `union_value` selection
  applies even to a nullary construction or a payload consisting only of
  references: its body remains the aggregate
  [FR-440](FR-440-checked-package-union-and-case-nodes.md) specifies, with
  no construction application, operator or operation identity added. This
  exception does not select `record_value` or `tuple_value` nodes whose
  bodies contain no application. A present option value is a `value` node
  with `semantic_form: "option_value"`. Its `semantic_type` is the
  `composite_type`/`option` node; its body is exactly
  `aggregate{[binding "value" = reference(payload)]}`. The referenced payload
  is the checked value of the option's element type. The option type and
  payload reference both determine the node's path. Its `dependencies` contains the
  payload target, not the `semantic_type` edge. It has no operator identity:
  constructing `Some(e)` is a value construction, not an application.
  This `semantic_form` is distinct from FR-207's runtime-value-path step tag
  `option_value`; one identifies a checked package value node and the other
  records traversal through a present optional. Before declarations, the
  reader requires `semantic_type` to resolve to a `composite_type`/`option`
  node, refusing `invalid_semantic_graph` with no cause at
  `/semantic_graph/nodes/{n}/semantic_type`. The `value` reference must resolve
  to a checked value whose type is the option node's element type; otherwise
  the reader refuses `ill_typed`/`type-mismatch` at
  `/semantic_graph/nodes/{n}/body/members/0/value`. Its dependency list is
  exactly the one payload target; a mismatch refuses
  `invalid_semantic_graph` at `/semantic_graph/nodes/{n}/dependencies`. The
  schema admits this body only for `value`/`option_value`.
  `option-value-vectors.json` selects type and payload nodes from
  `positive-integer-literal-references.json`.
  An `expression` whose body root is
  an application has the `semantic_form` fixed by its operator class; the
  node's `dependencies` are exactly the unique node-id-ascending `reference`
  term targets, member declarations and member `population` nodes of its body
  (a member `destination_type` is not one);
  a `dependency_reference` contributes none. A contradictory form or
  dependency list is `invalid_semantic_graph`; for a dependency list the locus
  is the node's `dependencies` member, `/semantic_graph/nodes/{n}/dependencies`.
A frame body refusal, meaning-join or canonical-order
  ([FR-340](FR-340-checked-package-frame-location.md)), is reported after
  every declaration refusal and before any state or operation refusal, at
  the first `state`/`frame` node, visiting them in ascending node-id order, that
  FR-340's own meaning-join/canonical-order precedence selects a defect for.
  A state refusal (the placement of `quire.op.state.clause`, then
  [FR-342](FR-342-checked-package-operation-anchor-body.md)'s
  `operation_anchor` joins, then
  [FR-341](FR-341-checked-package-state-clause-body.md)'s `state_clause`
  joins) is reported after every frame refusal and before any operation
  refusal, in the order FR-341's "Reader order" fixes.
  **Body steps.** After every state refusal and before any operation
  refusal, the reader runs the body steps in this one order, each over its
  own node forms and each reporting before the next starts: the state-graph
  step (FR-475), the temporal step (FR-370), the abstraction step (FR-451),
  the refinement step (FR-375), the terminal step (FR-476),
  the step-relation step (FR-477), the hybrid
  step (FR-481), the timed-model step (FR-230), the protocol step (FR-464)
  and the schedulability step (FR-482). Each body artifact's "Reader order" fixes the
  checks inside its step; its place among the steps is this list's.
- **One total order of the join stages** (FR-322-AC-275). Before
  every declaration, frame, state and operation refusal the reader runs these
  stages in this order, each over every node before the next starts, and reports
  the first refusal: (1) graph shape, the `dependencies` checks above, an
  independent dangling `reference` term and the cycle check of the
  recursion-group rule, last; (2) the owner join of the
  owner-bearing nodes; (3) the derived-shape semantic checks, which refuse a
  derived shape (reference, population, compound unit) whose body is not well
  formed, and a model declaration node whose body is not the empty aggregate,
  as `invalid_package`/`invalid-derived-shape` at the node's `node_id`
  (FR-322-AC-74, `RM-CU-ORDER`, `TW-27`); (4) the edge existence check of a
  `semantic_type` or `dependencies` entry that names no node, for a node of tag
  `scalar_type`, `composite_type`, `bounded_domain`, `value`, `expression`,
  `function` or `model` only. A node of tag `state`, `temporal`, `protocol`,
  `relation`, `claim`, `correspondence`, `extension`, `hybrid` or `timed_model`
  is outside stage 4: this artifact defines no refusal for its `semantic_type`
  or `dependencies` edge. Where that form's own artifact defines one, that
  artifact gives the code, locus and place (a frame whose `semantic_type` names
  no node, and an entry, anchor, subject or clause target that names no node,
  is `missing_declaration`/`missing-name`: FR-340, FR-341, FR-342);
  where it defines none, the edge is left to that family's artifact and this
  artifact adds no semantics to it. No other order is a reading of this artifact. An owner
  defect therefore outranks every dangling `semantic_type` or `dependencies`
  edge. A `semantic_type` or `dependencies` target is not resolved in stage 1
  (that step compares the list and does not resolve its targets), so a package
  with both an owner defect and a dangling edge reports the owner join, and the
  dangling edge is reported by stage 4 only once every owner joins (`TW-28`).
  **Locus of an owner-join refusal.** `missing_declaration`/`missing-selection`
  is at the node's `/semantic_graph/nodes/{n}/node_id`, except a `ModelOwner`
  refusal of a `model`/`tuple_value_type` or `model`/`variant_type` node, which
  is at the node's `/semantic_graph/nodes/{n}/owner` (`TW-06` to `TW-09`). Each
  of those vectors replaces only the node's `owner/node` and keeps its
  `node_id`: a reader derives no `node_id` from an owner and does not compare
  a model node's `node_id.path` with its `owner.node`, so every
  `semantic_type`, `dependencies` and member edge to the node still resolves.
  The kept path still names the original declaration, which breaks the producer
  rule that a model node's path is a function of its owner; no reader check
  sees that, so the owner join is the one defect a reader checks. A
  `ModelRootOwner` refusal of a `model_root` is `missing_import`/`missing-selection`
  at `/semantic_graph/nodes/{n}/owner/identity` (`MR` vectors).
- owner join stage — after graph shape and before any declaration, frame,
  state or operation refusal. A **model declaration node** has the body
  `{term: "aggregate", members: []}` and is its own `semantic_type`, carries no
  `recursion_group`, and an undeclared ungrouped derived shape (`scalar_type`
  `boolean` and `integer`, `composite_type` `reference`, `option`, `set`, `bag`,
  `sequence` and `ordered_set`, `bounded_domain` `integer_range`,
  `collection_bounds` and `model_population`, or `scalar_type`
  `compound_unit`) is its own `semantic_type` for a `scalar_type` or
  `composite_type`; a node that breaks its form's shape refuses
  `invalid_package`/`invalid-derived-shape` at the node's `node_id`. A model
  declaration node whose body is not the empty aggregate refuses it at
  `/semantic_graph/nodes/{n}/node_id` in stage 3 of the join-stage order, after
  every owner joins (`TW-27`). A
  `relation`/`refinement` node carries its authored `name` in its `body`, so
  the name is part of the node's path and two differently named refinements are
  two nodes (FR-375-AC-29), while the node still carries no `declaration`,
  `owner` or `nominal_structure`; when FR-375's `class_catalog` is present it is
  part of that exact `body` in the V2 wire schema. Its local class `node` and
  `binder_types` ids enter the node's exact `dependencies` list; external
  `PackageNodeRef`s do not. The dependency check precedes FR-375's class joins.
  A `protocol`/`protocol_clause` application node carries the identity input
  `owner_model` required by FR-464: its exact local model `NodeRef` is part of
  the node's path and enters the node's exact local `dependencies`. Other application nodes omit it. The reference names the object-type node of the model, owned by a
  selected declaration, over which that protocol clause is declared (the
  object type of its `over` input);
  consumers use this edge as the authoritative clause-to-model association.
  A protocol clause is written as a `protocol`/`protocol_clause` node only
  under a selected protocol profile (FR-464 "Operator catalog"); a unit that
  selects none still has its clause checked and writes no node for it.
  FR-464's protocol step checks its typed target (step 6, the clause's first
  check, after the refinement step; FR-464 "Owner-model edge phase"), and
  no general stage refuses the edge. The target node's own
  `ModelOwner` is still joined by this stage before any body step, so a target
  that is a model declaration owned by no selected declaration refuses
  `missing_declaration`/`missing-selection` at the target's `node_id`, never
  at the edge. Among `correspondence` nodes, only
  the abstraction form is excluded: a node of
  any other form (`source_locus`, `model_correspondence`, `binding_role`,
  `profile_correspondence`) whose body holds no application is a structural
  node. The stage joins the `owner` of every owner-bearing node, visiting them
  ascending by node-id, and refuses the first owner that fails:
  - A `SourceOwner` joins when its (`authority`, `identity`) equals a
    `lock.sources` entry's, else `missing_declaration`/`missing-selection`
    at that node's `node_id` (the owner-join locus paragraph of "One total order of the join stages" states the `/owner` exceptions). It is also coherent with the source map: every
    region of the node's `declaration` occurrence names a `source` whose
    (`authority`, `identity`) equals the owner's, else
    `invalid_package`/`invalid-value` at that occurrence's `source_map`
    entry, because a declared node's owner is the source unit that declares
    it.
  - A `ModelOwner` joins when its `identity` equals the `identity` of a
    `model_root` node's `ModelRootOwner` and the admitted document of that package ("Model-owned
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
    visit.
  - A `ModelRootOwner` joins when its `identity` is that of a domain
    package the host supplies a document for (FR-154). It identifies the selected domain package as
    a whole, not one of that package's declarations. A model root uses
    `node_tag: "model"`, `semantic_form: "model_root"`, an empty aggregate
    body and its own node id as `semantic_type`; its closed owner is
    `{kind: "model_root", identity}`. Its `node_id` is
    `{package: identity, path: ["model", "model_root"], ordinal: 0}`, determined
    by the selected package identity and independent of the source alias. A `model_root` with a `ModelOwner` and a
    `ModelRootOwner` on another form fail the schema (owner-presence table
    below) and a reader refuses them `malformed_wire` at the node's `owner`
    (FR-322-AC-51); a root whose owner does not join a selected model package
    refuses `missing_import`/`missing-selection` at the node's `owner/identity`.
    The root has exactly one `generated` occurrence at ordinal zero, whose
    source-map region is the existing source occurrence that selected this
    model package; the root has no declaration occurrence of its own.
    The root is derived by the checked-package producer from the existing
    model selection; it does not require a new FCD declaration.
    **Completeness and stage.** The V2 schemas check only the closed root
    shape; they cannot join one node to a supplied document, so the join is a
    reader rule. A reader refuses a root whose owner identity names no supplied
    document `missing_import`/`missing-selection` at
    `/semantic_graph/nodes/{i}/owner/identity`. A second root for one identity has the same id; it
    refuses `invalid_semantic_graph` at the second node's
    `/semantic_graph/nodes/{i}/owner`, because the owner is the invalid member. **Order.** The owner join runs in the
    owner-join step above (ascending node-id); the declaration-name checks run
    after it. A package that breaks both reports the owner join. The vectors `MR-04` and
    `MR-09` of `model-root-vectors.json` each carry exactly one defect.
    The root is the producer's complete and final output for the row: a
    producer derives the node from the row alone, and a producer that
    reproduces a published selected-model package
    appends the root after every other node in graph order (graph order is part
    of the package value) and its `source_map` entry after every other entry. The
    published fixtures carry exactly that node and entry.
    **Selection-to-source relation.** The wire carries no separate field
    relating a selection to its source text. The only relation is the root's
    own `generated` ordinal-zero `source_map` entry: the row
    `identity` determines the root id, and that entry holds the
    region or regions the producer asserts for the selecting occurrence, each
    naming a `lock.sources` entry. The relation is producer-asserted: no
    reader checks that a region is the true selecting occurrence, and
    "exactly one region" is not a wire rule (a reader admits two). The
    one-byte regions in the published fixtures are placeholders copied from
    the first source-map entry, not selecting ranges. FR-065-AC-2 source
    correspondence therefore cannot be decided from the wire and stays
    unclaimed in IR-1043; STD-505 closes with this statement, not with a
    decidable relation.
  A structural node carries `semantic_type` `null` exactly when the node is its own type, and `owner`
  exactly when it is owner-bearing. The members of a recursion group are the
  nodes carrying its `recursion_group` label, a non-empty string the producer
  chooses and all members share. Membership is that label alone: the wire has
  no member for a node's position in its group, and a reader derives none. A
  node whose form is not one of the shapes above refuses
  `invalid_package`/`invalid-value` at the node's `node_id`.
- `dependencies` — the nodes a node's definition is built from: a list of
  node ids of the node's own graph, unique and ascending by node-id. A reader follows it beside `semantic_type` and the body's
  references when it builds a node's closure and checks graph cycles. A node
  that is its own `semantic_type` (a self-typed node, `semantic_type` `null`) adds no edge by that type and is not a cycle; only a path through two or more distinct
  nodes, or through a reference, is a cycle under the cycle rule above. A
  producer SHALL write, and a reader SHALL require, exactly the nodes these
  rules name for every node:
  1. each `reference` term target in the node's `body`, at any depth: an
     aggregate member, a binding's value, an application argument or a body
     root. A reference to a member of the node's own recursion group is a
     `reference` naming that member's `node_id` on the wire, so it counts by
     that `node_id`; `{term: "group_reference", ordinal}` is its spelling
     in the written form of a
     group member's reference only;
  2. for each application in the body, the declaration its
     `operation.member` names and that member's `population` node, as the
     application-node rule above states;
  3. for a `bounded_domain` node, its `semantic_type`, the type it bounds;
  4. for a nominal node (an enum, dimension, dimension equivalence or unit),
     the nodes its body names (FR-142 for dimensions and units), and for a `state`/`frame` node the entries of its body
     ([FR-340](FR-340-checked-package-frame-location.md)).

  The rules give the whole list. A literal's `type`, an application's
  `result_type` and the `semantic_type` of every node other than a
  `bounded_domain` are type annotations: a reader reads them through those
  members, and one is in the list only when a rule names the same node. A
  law's `definition` is a `DefinitionRef`, not a node. A
  `dependency_reference` names a node of another package and adds no entry,
  in an application's join as anywhere else. A node form whose body artifact
  states its own list keeps that list, which these rules restate for its
  `reference` terms: [FR-341](FR-341-checked-package-state-clause-body.md),
  [FR-342](FR-342-checked-package-operation-anchor-body.md),
  [FR-370](FR-370-checked-package-temporal-clause-body.md),
  [FR-375](../../functional/analysis/FR-375-declare-model-refinement.md),
  [FR-402](FR-402-hyperproperty-obligation-identity.md),
  [FR-440](FR-440-checked-package-union-and-case-nodes.md),
  [FR-451](FR-451-checked-package-abstraction-relation-body.md),
  [FR-464](FR-464-checked-package-protocol-node-bodies.md) to
  [FR-466](FR-466-checked-package-protocol-control-bodies.md),
  [FR-469](FR-469-checked-package-extension-nodes.md),
  [FR-475](FR-475-checked-package-state-graph-claim-body.md),
  [FR-476](FR-476-checked-package-model-terminal-declaration-body.md),
  [FR-477](FR-477-checked-package-step-relation-claim-body.md),
  [FR-481](FR-481-checked-package-hybrid-model-nodes.md) and
  [FR-482](FR-482-checked-package-schedulability-claim-body.md). For example:

  | Node | `dependencies` |
  | --- | --- |
  | builtin `scalar_type`, `model` or `relation` declaration node, enum declaration | `[]` |
  | `value`/`literal`, `value`/`parameter` | `[]`; the body holds literals only |
  | `value`/`option_value` | the one payload target referenced by its `value` binding; its `semantic_type` is not a dependency |
  | `bounded_domain` (`integer_range`, `collection_bounds`, `model_population` and every other form) | its `semantic_type` only; a bound's literal `type` is not listed |
  | `composite_type` `option`, a collection kind, `reference`; declared `record` or `tuple` | each node the body references |
  | `expression`/`reference`, `correspondence`/`source_locus`, `protocol`/`protocol_reference` | the body's target |
  | `scalar_type`/`compound_unit` | each unit its terms reference |
  | `relation`/`refinement` | the unique node-id-ascending local body keys and term references of FR-375, including local `abstract` and `protocol` keys even with no rows; no external `{package, node}` reference, profile `DefinitionRef` or standalone `semantic_type` edge |
| `extension`/`extension_form` | the unique node-id-ascending targets of its clause bindings |
  | `function`, a clause function included | each node its body references: its parameter nodes, its body's root node and its measure's node |
  | an `expression` holding an application | rules 1 and 2 over the application |

  A list that differs from the one these rules rebuild from the node as
  written refuses `invalid_semantic_graph` at the node's `/dependencies`, a
  graph-shape refusal (FR-322-AC-96).
- `dependency_reference` — a reference to a declaration exported by a
  directly selected `dependency_selections` package:
  `PackageNodeRef{package, node}`, where
  `package` is that dependency's `name`
  and `node` is the `node_id` of the declaration inside that dependency's own
  package; it is resolved only by lookup in the dependency's admitted package
  and never minted from the importing package's namespace. A reader resolves a `dependency_reference` only
  against the named dependency's admitted package. A `dependency_reference`
  naming a `package` absent from the importing package's direct
  `dependency_selections` refuses `missing_declaration`/`missing-selection`,
  even if that package occurs in the derived closure through another selection.
  On the wire it is the term
  `{term: "dependency_reference", package, node}`, where `package` is a dependency identity string and `node` has the `DeclaredNodeId` shape, a `node_id` whose `path` holds only strings ("Operand index segments"). It stands only as the
  callee, argument 0, of a `quire.op.function.call` application, and names
  a declaration node of the dependency, never a node of the importing
  graph. A node's `dependencies` list names nodes of its own graph only, so a
  `dependency_reference` target is never listed there; it joins through
  `dependency_selections` instead. A `node` that is not a node carrying a
  `declaration` in the named dependency's admitted package refuses
  `missing_declaration`/`missing-name`. A `dependency_reference` names a
  `function` declaration whose parameter and result types are
  package-independent. The *signature type nodes* of a function are the
  `semantic_type` of each `value`/`parameter` node its `parameters` binding
  references (the one source of its declared parameter types) and the
  function's own `semantic_type` (its result type); the function's body
  references and its own `dependencies` list are not signature nodes. No node
  in the transitive closure of the signature type nodes' `dependencies` carries a
  `declaration` or a `ModelOwner`. Such types (builtin, bounded-domain and anonymous
  structural types over them) carry no declaration, so each belongs to the package
  that contains it and an imported node keeps its own package name. The
  application's `return:0` result is that function's result type; the
  referencing node's `result_type` names that type's node by that node's
  `node_id`, whose `package` is the dependency's name. A `dependency_reference` that
  names any other declaration, names a function one of whose reachable
  signature type nodes carries a `declaration` or a `ModelOwner` (a
  `Set<R>`, a tuple holding `R`, a quantity over a declared unit, or
  `Reference<M::T>` for a model type `M::T`), or
  stands anywhere other than a `quire.op.function.call` callee refuses
  `ill_typed`/`operator-ineligible`. A reader checks each
  `dependency_reference` in step 7 of the first-refusal order, at its place
  in its node's pre-order term walk wherever it stands (an application
  argument, a binding value, an aggregate member or the body root):
  `missing-selection`, then `missing-name`, then `operator-ineligible`. Each
  refusal's locus is an RFC 6901 pointer into the importing package document:
  `missing-selection` at the term's `package` member, `missing-name` at the
  term's `node` member, and `operator-ineligible` at the term itself, each under the pointer of the term
  in its node's `body`.
- `model_member` — a `field` or `operation` member whose declaring node is a
  model declaration node: a `model` or `relation` node that carries no
  `declaration` and carries its `ModelOwner` (QSL FR-094) as `owner`. Its
  body is `aggregate{[]}` and names
  no member, so the reader derives the member from the domain package the
  lock selects for its owner, in the four steps of
  "Model-owned members" below. Having no node, a field or operation is
  named in a domain key under its declaring type's model declaration node
  (FR-331 "Model member domain keys").
- `model_root` — one self-typed `model` node the producer derives for each
  selected domain-package identity. It has no `declaration`, an
  empty aggregate body and a `ModelRootOwner`; unlike a declaration node, it
  names the selected package as a whole. It is independent of source aliases,
  object-type count and state-machine count.
- `declaration` — `{qualified_name}`, the name a named, source-declared node
  introduces, with the identifier-path grammar. It is carried exactly by
  a `scalar_type`, `composite_type`, `bounded_domain`, `function`, `model`,
  `protocol`, `claim`, `extension` or `hybrid` node, or a `value` node other than an `enum_value`, that
  has a `declaration` source occurrence. A builtin or anonymous type reached
  only through `type` occurrences, an expression, a let value and every other
  node must not carry it, and a producer never invents a name.
  `qualified_name` is the package-local declaration path: its segments name the
  declaring scopes from the package root to the declaration. A consumer
  resolves a qualified reference `a::Name` by first resolving `a` to exactly one
  locked package (itself or a `dependency_selections` entry) and then matching
  `Name`, byte for byte, against the `qualified_name` path of that package's
  declarations; no case folding, normalization, suffix match or search across
  packages applies. A dimension or unit that an admitted domain package
  declares ("Model-owned members" step 4) is a node with no `declaration`, so
  no `qualified_name` names it: a source reaches a package unit only as
  `M::name` in a unit position, where `M` is the model alias of a
  selected domain package and `name` matches, byte for byte, the `name` of a
  unit in that package's document, and it reaches a package dimension
  the same way, as `M::name` in a dimension position (FR-142 "Namespaces"). On a source `enum`, `unit` or `dimension` node, `node_id.path` equals
  `qualified_name` (a structural label comparison; nothing is recomputed or
  hashed), else the reader refuses as `invalid_package` with
  cause `declaration-nominal-mismatch`. The one exception is a nominal node
  whose `owner` is a `ModelOwner` (a dimension or unit an admitted domain
  package declares, "Model-owned members" step 4): it carries no
  `declaration`, its owner is that `ModelOwner`, and its `node_id.path` is
  `[node_tag, semantic_form, <FCD identity>]` like every other node without a
  `declaration` (`["scalar_type", "unit", "ix://acme/rpc/unit/ms"]`), so a unit, a dimension and a
  record of one domain package that share a name have distinct `node_id`s; two nodes carrying the same name in one namespace refuse
  as `ambiguous_declaration` with cause `ambiguous-name`, both before any
  operation refusal. The namespaces are three: dimensions
  (`scalar_type`/`dimension`), units (`scalar_type`/`unit`) and types (every
  other declared node), so a unit, a dimension and a record may carry one name
  (FR-322-AC-81); a model-owned member's `ambiguous-name` is the operation
  refusal of step 3 of "Model-owned members". Library exports resolve only through this member; a
  consumer never recovers a name from source occurrences, display text or a
  node's position.
- `owner` — the owner of a node, carried on the node
  so that a consumer reads it from the package alone. An **owner-bearing node** is a
  structural node (a body whose root is
  not an `application`) that either carries a `declaration` or is a
  **model-owned node** (QSL FR-094) carrying none: a model declaration node
  (`model`/`model_root`, `model`/`object_type`, `model`/`record_value_type`,
  `model`/`tuple_value_type`, `model`/`event_type`,
  `model`/`variant_type`, `model`/`state_machine`, `model`/`process`,
  `model`/`persistence_interface`, `model`/`systems_interface`,
  `relation`/`relationship` or `relation`/`population`), a `scalar_type`/`dimension`
  or `scalar_type`/`unit` node an admitted domain package declares, or a clause function
  (`function`). A node with a `declaration` carries a `SourceOwner`; a node without one
  carries the `ModelOwner` that admitted it (or the `ModelRootOwner` of a `model`/`model_root`). A declared
  owner-bearing node, a source `enum`, `unit` or `dimension` included, carries
  the `SourceOwner` of the source unit that
  declares it, `{kind: "source", authority, identity}`, as does, as the one exception, a
  `scalar_type`/`dimension_equivalence` node, which has no `declaration`; a model-owned node
  carries its `ModelOwner`, `{kind: "model", identity, node}`, with no
  `version` member, or the `ModelRootOwner`
  `{kind: "model_root", identity}` for a `model`/`model_root`.
  The latter's identity is that of a supplied domain package. Every other
  node carries no `owner`: a builtin or anonymous type, an undeclared value,
  parameter or expression, any other undeclared node and an
  application node, a `state`/`state_clause` node included. A
  `DefinitionOwner` is never a node `owner`. A reader reads a
  node's owner only from this member: never from the source map, a source
  occurrence, the lock or the node's position. The V2 schema states the
  rule ("Owner member schema" below), and a node that breaks it refuses
  `malformed_wire` at the node.
- Nominal nodes — each enum declaration/member, dimension,
  dimension-equivalence or declared-unit node is identified by its declaration
  path like any declared node (a dimension equivalence by its structural id,
  which names its two dimension nodes). A source `enum`, `unit` and `dimension`
  node carries its name in `declaration`, its `SourceOwner` and a closed body
  holding its source values (nothing in it is hashed or recomputed):
  an `enum` `{term: "enum", ordered, members}` with `ordered` a boolean and `members` a non-empty
  list of unique identifiers, the enum's cases in declared order; a `dimension`
  `{term: "dimension", terms}` with `terms` a list of `{dimension_node_id, exponent}`
  (`dimension_node_id` a node id, `exponent` an integer string), empty for a base dimension;
  and a `unit` `{term: "unit", dimension_node_id, target_unit_node_id, scale, offset}`
  with `dimension_node_id` a node id, `target_unit_node_id` a node id or `null` for a canonical
  root, and `scale` and `offset` exact rationals `{numerator, denominator}`
  (`numerator` an integer string, `denominator` a positive integer string). An enum is
  `ordered_enum` when its `ordered` is true. A
  model-owned nominal node reads its values from the selected document, and its body is
  `{term: aggregate, members: []}`. A dimension
  equivalence carries its pair and scale in its body and its `SourceOwner`.
  Owner subjects must join an exact lock
  selection. A reader joins enum member declaration/case to `semantic_type`,
  dependency and literal body; dimension terms to self-type/dependencies; a
  dimension equivalence's two dimension ids to self-type/dependencies; and unit
  dimension/target to `semantic_type`/dependencies.
    An `enum_value` node whose `semantic_type` is a
  `model`/`variant_type` node joins the same way, and its `case` SHALL be a case
  of the variant type that the `ModelOwner` selection of its `semantic_type` node declares (the cases of
  `quire.meaning.model.variant-type/v1` in the admitted document, FR-208), else
  the reader refuses `invalid_semantic_graph` at the `enum_value` node, as it
  does for a scalar enum whose case is not among its declaration's members.
  An `enum_value` node whose `semantic_type` is a `model`/`variant_type` node
  of family `union` (one of whose cases carries a `payloadType`) refuses
  `invalid_semantic_graph` at the node, whichever case it names: a member of
  a union is constructed by a `union_value` node, a nullary one included
  ("Tuple value types and payload-bearing variant types").
  The `case` of such a node and of its member id is the declared case
  name, any non-empty string, matched byte for byte as UTF-8; the identifier
  requirement of the member id holds for a `scalar_type`/`enum` member
  only (FR-141).
- `source_map` — the total semantic-occurrence to original source-region
  mapping. An occurrence key is (`node_id`, `role`, `ordinal`), where role is
  `declaration`, `type`, `expression`, `anchor`, `claim` or `generated`; a
  source region is a half-open UTF-8 byte interval `[start,end)` under one exact
  source-document identity. The role `generated` is the role
  of every occurrence of a *compiler-derived node*, a node the compiler adds
  that no source text declares and that derives from exactly one declaration
  node, its *owner*, and of the per-obligation occurrences a refinement owner
  carries on its own node ("Refinement step occurrences" below). Its ordinal is the node's position among the derived
  nodes of that owner, in the owner's total order (for a refinement owner
  deriving one node per step row, FR-375's `steps` declaration order), and its source
  region, when it has one, is the region of the source row it derives from. An occurrence may name
  multiple ordered regions; regions for that one occurrence/document cannot
  overlap, while regions belonging to different occurrences may nest or
  overlap. A path, display name or search result is not a source identity.

  **Refinement step occurrences.** A refinement owner node
  (`node_tag: "relation"`, `semantic_form: "refinement"`,
  [FR-375](../../functional/analysis/FR-375-declare-model-refinement.md)) that
  is *proof-eligible* carries occurrences of role `generated` on its own
  `node_id`, one for each obligation of its per-step simulation proof
  ([FR-468](../../functional/analysis/FR-468-certify-per-step-refinement-obligations.md)).
  A refinement is proof-eligible exactly when FR-468's proof applies to it: it
  has no hidden field, no history field (its `histories` is empty) and no `any`
  step target. A hidden field (FR-375 "Object rows", FR-378) is an abstract
  field with no row: for an entry of the body's `objects`, the abstract fields
  are the fields of its abstract object type in the abstract model node that
  the body's `abstract` key selects in its dependency package, and the field is
  hidden when the entry has no field row of that name. A refinement with an
  abstract field that is hidden in any `objects` entry is not proof-eligible. The initial obligation is at ordinal 0, and the
  obligation of the step row at zero-based position `i` of the node's `steps`
  order (declaration order, FR-375) is at ordinal `i + 1`. The ordinal is the obligation's
  position in the certificate's order, the base part and then the step parts.
  The `steps` order is the order the rows were written, so reordering the rows
  in source reorders the ordinals. The `regions` of a step
  occurrence are exactly the one region of the source row of its step row, and
  the `regions` of the initial occurrence are exactly the one region of the
  node's `declaration` occurrence. This ordinal is the position of an
  obligation of the owner itself, and differs from the ordinal of a node
  derived from a step row (FR-322-AC-87), which counts the owner's derived
  nodes alone.
  - The reader SHALL admit on a proof-eligible refinement owner only a
    `declaration` occurrence at ordinal 0 and `generated` occurrences at
    ordinals 0 to `n` for `n` step rows.
  - The reader SHALL require the `declaration` occurrence at ordinal 0 on every
    refinement owner.
  - The reader SHALL require one `generated` occurrence for each ordinal 0 to
    `n` on a proof-eligible refinement owner only.
  - The reader SHALL admit on a refinement that is not proof-eligible only the
    `declaration` occurrence at ordinal 0, and no `generated` occurrence.
  - The reader SHALL refuse as `invalid_package`/`invalid-value` at the
    offending `source_map` entry: a second entry with one (`node_id`, role,
    ordinal) key, reported at the second; a role and ordinal pair outside the
    admitted pairs (a `generated` ordinal above `n` included); and a `regions`
    that is not the one region its ordinal requires.
  - The reader SHALL examine the owner's entries in `source_map` order and
    refuse the first that fails, and SHALL refuse a missing required occurrence
    at `/source_map` only when every entry passes.
  - The reader SHALL run this check after the owner join and the owner and
    source-map agreement of the owner join stage (FR-322-AC-52), over the
    refinement owner nodes, visiting them in ascending node-id
    order, and report the first failing owner.
- `refinement_step_rows` — an identity-excluded source-provenance sidecar for
  refinement step rows. It is absent exactly when no `relation`/`refinement`
  node has a `steps` entry; otherwise it is a nonempty array of closed
  `{node_id, ordinal, region}` entries. `node_id` is the refinement owner's
  retained `NodeRef`; zero-based `ordinal` is the position of one entry in
  that owner's `body.steps` array, which is in declaration order; `region` is exactly one
  `SourceRegion` for the original checked source row that produced the step.
  QSL SHALL derive this region from the checked source row before writing the
  package, including for a refinement that is not proof-eligible. The sidecar
  has no entry-order rule.
  The sidecar's regions are provenance only; changing a row region alone leaves
  every `node_id` unchanged.

  After the owner join and before refinement step occurrences, the reader
  SHALL visit `refinement_step_rows` in wire order and refuse
  `invalid_package`/`invalid-value` at `/refinement_step_rows/{i}` for an entry
  whose `node_id` is not one retained `relation`/`refinement` node, whose
  ordinal is outside that owner's `steps` array, or whose owner and ordinal
  pair repeats. It then SHALL require
  exactly one entry for every step position of every refinement owner; an
  absent sidecar, a present sidecar with no refinement step, or a missing
  position refuses `invalid_package`/`invalid-value` at
  `/refinement_step_rows`. For each proof-eligible owner, the reader SHALL
  compare the single region of its `generated` `source_map` occurrence at
  ordinal `i + 1` to the region at sidecar ordinal `i`; a mismatch refuses
  `invalid_package`/`invalid-value` at that `source_map` entry. These checks
  compare two wire carriers; the reader does not parse raw QSL source to
  verify that a producer supplied the true original row region.
- `checked_term_data` — an optional, closed producer-captured carrier for facts
  that a later typed read must return without repeating the producer's check.
  When present it has exactly three arrays, `equality_decisions`,
  `protocol_locations` and `dispatch_tables`; any may be empty. When absent it
  means all three arrays are empty. It adds no semantic graph node and does
  not change any node id. Each array is rooted at a local application
  `node_id` and one exact occurrence key (`role`, `ordinal`) belonging to that
  application. The package name binds that key to this checked package; a node
  from another package cannot satisfy it. Each row family adds its own
  discriminator to that root identity.
  `equality_decisions` is keyed by the local equality application `node_id`,
  one of that node's exact `occurrences` keys (`role`, `ordinal`), and an empty
  `term_path`. The root is the equality or inequality expression application
  at that occurrence. `term_path` remains a required wire member as an empty
  array root marker; a non-empty path is refused as `invalid_package`/
  `invalid-value` at its first segment. The reader does not traverse a
  `CheckedTerm` tree, follow a reference to another graph node, or infer an
  equality identity from a path or name. Every composite application is its
  own graph node, so an equality beneath an `and`, `or`, aggregate or binding
  form has a separate application `NodeRef`; the parent body references that
  child node, and the equality row joins directly to the child node with an
  empty `term_path`. Each row's `operator`, ordered operands and source types
  SHALL match that root equality or inequality application (`equal` or
  `not_equal`). Thus multiple equalities beneath one parent have separate rows
  keyed by their respective application nodes, each with an empty path.
  Each row retains the
  `equal` or `not_equal` operator, the two original operand positions in order,
  each operand's source type and producer-selected conversion target (or
  `null`), the selected `text`, `enum`, `quantity` or `plan` schedule, the
  resolved top-level quantity unit facts, and the filtered enum member facts.
  The two operand positions are exactly 0 and 1; the reader joins them to the
  application arguments and checks their source types and the operation
  identity, but does not choose or repeat the conversion or schedule. A unit
  fact records the quantity type, resolved unit, normalized base-dimension
  terms, and exact unit-table form: a declared unit's dimension node, root,
  and ordered conversion edges, or a compound unit's canonical root-unit
  terms with exponents. An enum fact records the enum declaration, exact
  member identity, case, producer-selected ordinal and ordered-enum flag.
  Only units named by a top-level quantity operand and only
  members of the compared enum are retained; irrelevant unit or enum tables
  are empty. An enum schedule carries its complete filtered member index; a
  non-enum schedule carries no enum members. A non-quantity schedule carries
  no unit rows.

  `protocol_locations` maps a local application occurrence to the semantic
  location of each selected protocol term represented there. Its key is the
  application `node_id`, an exact application occurrence key, and a
  producer-assigned zero-based `term_ordinal`, unique within that application
  occurrence. Within each captured application/occurrence group the ordinals
  are consecutive from zero: a row with ordinal `k` witnesses that every
  ordinal below `k` requires its own row. A missing predecessor refuses
  `invalid_package`/`invalid-value` at `/checked_term_data/protocol_locations`.
  Existing row shape, join, duplicate-key and canonical-order refusals SHALL
  precede this array-level missing-predecessor check.
  A lone ordinal-zero row is a complete one-term capture; it does not witness
  an unrecorded successor. This final coordinate distinguishes multiple semantic terms
  represented by one graph application node; unlike equality's always-empty
  `term_path` root marker, it is a producer-assigned row ordinal, not an
  argument path, source-map ordinal
  or source position. `location` preserves the producer's semantic origin and
  child-index path. Its closed origin is either a named type declaration or a
  protocol attempt identified by the protocol and attempt positions in the
  compilation. These values are captured from the checked semantic term and
  are never derived from `source_map` regions, display names, or a package
  selected outside this checked package.
  When a supplied `location.origin.kind` names neither closed origin variant,
  the reader SHALL refuse `malformed_wire` at that `kind` member, not at the
  containing origin object.

  `dispatch_tables` retains the producer's admitted candidate table for a
  selected model-operation dispatch. A table names the exact application and
  occurrence, selected operation witness `{declaring, operation}`, and receiver
  context identity. `declaring` is a `NodeRef` to an admitted `model` node
  whose `semantic_form` is `object_type`, `process`, `state_machine` or
  `persistence_interface`, as required for operation members by FR-322-AC-25.
  `operation` is the original member name as authored in that type. A
  relationship declaring node is ineligible under FR-322-AC-30. Model
  operation members have no semantic graph
  node, so the witness does not contain or imply an operation `NodeRef`. The
  reader checks that the declaring node is admitted and declares an operation
  with exactly that name; a field, absent member or invented operation path
  is refused. Neither the producer nor reader infers the declaring type or
  member name from a graph path, display name or other identity. Its
  `candidates` array is in producer order; the array order is semantic and is
  never sorted by the reader. Each candidate records the receiver's exact
  selection key as the structural `EffectiveId`
  `{owner_effective_type, original}` of FR-150, the candidate operation's body
  handle, and its effective precondition body handle, or `null` when the
  effective precondition has no body. `EffectiveId.original` remains the
  original `{package, node}` declaration key. This selection key is not a
  checked-package `NodeRef`; the reader compares the captured identity
  componentwise with the candidate's admitted effective declaration identity.
  It does not derive the key from a name, graph node id or reverse lookup. A
  present empty array is the producer's captured empty candidate set;
  an absent table is unavailable. Candidate selection keys are unique within
  one table. The reader joins all handles by exact node identity and checks
  that the table names the application and selected operation it accompanies.
  It does not choose a candidate, evaluate a precondition, derive candidates
  by name, or reconstruct the dispatch table from the graph.

  The producer writes these rows from the same checking stage that emits the
  checked package. A row is local to the root package: its application must be
  a local `expression`/application node, the occurrence must be present on
  that node and in `source_map`, and no dependency package may satisfy the
  join. Equality rows are unique by (`application`, `occurrence`, `term_path`);
  because every equality `term_path` is empty, this permits at most one row per
  equality application occurrence. Dispatch tables are unique by
  (`application`, `occurrence`); protocol rows by
  (`application`, `occurrence`, `term_ordinal`). Each array is ordered by
  application node id, then occurrence role and ordinal. Equality ordering ends
  at the occurrence key because the required `term_path` is always empty.
  Protocol rows then order by `term_ordinal`.
  Dispatch candidate order within one table remains the producer order. The
  reader validates these structural joins
  and preserves the captured facts exactly. A missing equality row remains
  unavailable to an IR consumer; it does not cause a schedule, unit table or
  enum index to be fabricated. A producer or reader SHALL NOT rebuild these
  facts by type-checking the graph, projecting value types, consulting an
  operation catalog, searching source text or resolving an arbitrary source
  package. Every inspected row and nested unit or enum fact is charged to the
  caller's finite work limit before inspection. Equality `term_path` is always
  empty, so it contributes zero path-segment work. Exhaustion returns the
  attempted work and first uncharged locus without a partial data view.
- `capability_report` — one closed disposition for every required feature and
  selected model capability. Backend/provider installation is absent.
- `diagnostics` — catalog-qualified, stage-qualified typed causes and exact
  available loci. Message text is not a classification or identity component.

`read` receives limits from its caller, the domain package documents its
the domain-package documents of the `model_root` nodes, keyed by identity (FR-154), and the extension definitions the reader holds (FR-469); they are not artifact data and never participate in package identity. Its closed `ReadResult` is exactly one of:
`admitted { package }`; `refused { code, stage, cause, loci }`; or
`incomplete { limit_kind, limit, consumed, stage, loci }`. The v2 refusal code
vocabulary is `unknown_contract_version`, `malformed_wire`, `duplicate_member`,
`unknown_member`, `noncanonical_wire`, `unknown_required_capability`,
`invalid_semantic_graph`, `invalid_source_map`, `unsupported_node_tag`,
`invalid_package`, `ill_typed`, `ambiguous_declaration`, `missing_declaration`,
`invalid_model_binding`, `missing_import`, `unknown_profile`,
`unsupported_construct` and `wrong_snapshot`. A V2 reader SHALL pair the cause
`unsupported-wire` only with `unknown_wire` and `unsupported-feature` only
with `unknown_required_feature`, as FR-272 and the `Diagnostic` schema's
`DiagnosticCausePairing` do; `unknown_contract_version` and
`unknown_required_capability` carry no cause (FR-272); the ContractPackage
reader refuses a nested refusal record that pairs either with a cause, as
FR-330 states (FR-322-AC-78 covers the pairing). The operation and declaration causes
above, and the `missing-name` and `malformed-declaration` frame-entry causes
[FR-340](FR-340-checked-package-frame-location.md) defines, are carried by
`invalid_package`, `ill_typed`, `ambiguous_declaration`,
`missing_declaration` or `invalid_model_binding`, each with the code the
native diagnostics catalog pairs with it, and by no other code; this
exclusivity is over those (code, cause) pairs. The (code, cause) pairs of
"Model-owned members" form a separate set, each a pairing the native
diagnostics catalog lists: step 1 reports the
FR-154 intake and `quire.model.complete/v1` normalization causes under their
own codes (`missing_import`, `invalid_model_binding`,
`missing_declaration`, `unsupported_construct` and `wrong_snapshot`), and
steps 3 and 4 report `ambiguous_declaration`/`ambiguous-name` and
`ill_typed`/`operator-ineligible` (a model declaration node whose owner names no
selected declaration of its kind was refused earlier by the owner join stage,
and step 2 refuses nothing).
The remaining (code, cause) pairs a V2 reader reports are each the pairing
[FR-272](../foundation/FR-272-typed-diagnostic-cause.md) lists, and the V2
`Diagnostic` schema admits each cause only with its code:
`ill_typed`/`type-mismatch` (FR-440's union value join, and FR-481's rate, reset and call terms); `ill_typed`/`non-boolean-root` (a Boolean root that is not Boolean: FR-464 to FR-466, FR-476, FR-477 and FR-481's invariants, guards, initial conditions and activation guards);
`missing_import`/`missing-selection` and `missing_declaration`/`missing-selection`;
and `unknown_profile`/`unsupported-selection` and
`unknown_profile`/`wrong-selection-role`
([FR-370](FR-370-checked-package-temporal-clause-body.md)'s profile check),
the schema admitting those two causes with any of FR-272's
`unknown_language`, `unknown_edition` and `unknown_profile`; and
`unsupported_construct`/`declaration-form`, `unsupported_construct`/`expression-form`
(such as the destination-multiplicity refusal of FR-322-AC-57) and
`unsupported_construct`/`not-yet-implemented`, the three causes FR-272 lists
for `unsupported_construct`.
Before expensive semantic work, the reader measures raw wire bytes; a byte
overage returns `incomplete`. It then performs strict syntax/member validation,
including the body grammar below; a malformed wire returns `refused`. It
charges graph nodes/edges, occurrences/loci, diagnostics and validation visits
as they are traversed; the first exhausted limit returns `incomplete` with its
actual counter, its configured bound and its setting name
([FR-461](../../functional/foundation/FR-461-name-the-limit-value-and-setting.md)),
and no admitted package. Equal-limit input is admitted if all later checks
succeed. The reader has no depth limit: the body grammar and the index
segments of node ids ("Operand index segments") fix the JSON depth of every
package
([FR-460](../../functional/foundation/FR-460-bound-every-stage-by-resource-limits.md)).

## Structural node identity

A node's identity is its *structural key*, the node's structural position,
compared componentwise and never hashed. A node's `node_id` is the wire form of
that key: a closed object with exactly these members:

| Member | Value |
| --- | --- |
| `package` | The `name` of the package the node belongs to (for a `model_root` node and for a node a domain package owns, that domain package's identity). |
| `path` | A non-empty array of segments, each a string or an *index segment* `{"node": i}` ("Operand index segments" below). A declared node's path is its declaration path, the declaring scopes' names outermost first, all strings. A derived node (no `declaration`, for example `Set<Integer>`, an application or a literal) outside every recursion group has the path `[node_tag, semantic_form, ...operands]`; each operand is a string (a literal's text, an operator or member name) or an operand node, which the path holds as the index segment naming that operand node in `semantic_graph.nodes`. A path never holds a `node_id` object. |
| `ordinal` | A non-negative JSON integer from `0` to 9007199254740991 (2^53-1, the I-JSON safe integer), assigned by the producer. It tells apart nodes that share one `package` and `path`; the reader's only rule is that no two nodes sharing a `package` and `path` have the same `ordinal`. It need not be `0`, need not be contiguous and need not follow sibling or graph order, and a reader never derives one. A recursion-group member's `ordinal` is no exception: it is producer-assigned and unique among nodes sharing its `package` and `path`, and it is not a position in the group (`recursion_group`, below). |

Ordinals compare numerically; `package` and `path` strings compare as UTF-8 bytes. An `ordinal` that is negative, above 9007199254740991, not a JSON integer (a string, a fraction or an exponent form, for example) is a malformed integer and is refused `malformed_wire` at `/semantic_graph/nodes/{i}/node_id/ordinal`, `i` the node's graph index; the same bound and the same refusal apply to every `node_id` held in a reference, dependency, `semantic_type` or `source_map` entry, and to the `node` member of every index segment, at that member's pointer.

A recursion-group member is identified by `{package, path, ordinal}`, and its `path` is never derived from its operands. A member that carries a `declaration` keeps its declaration path (the declaring scopes' names outermost first), as every declared node does, so the members of one group need not share one `path`: a group may hold several declared members, each with its own declaration path. An anonymous derived member carries the path of the group's declared member where the group has exactly one; where the group has none, or several, there is no external anchor for it, and the producer chooses its path, which is admitted, with nothing invented or refused. Nodes sharing a `package` and `path`, members or not, are told apart by `ordinal`, which is producer-assigned like any other (unique among nodes sharing `package` and `path`; a producer may use graph order, but need not); membership of a group is the shared `recursion_group` label alone, and the reader derives no position in the group. Only a derived node outside every recursion group derives its path from its operands, each operand node written as an index segment. A producer disambiguates other nodes sharing a package and path by distinct ordinals of its choosing; body content is not part of the path.

A node belongs to the package that contains it, and an imported node keeps its
own package name. Two nodes with equal `package`, `path` and `ordinal` have the
same `node_id`, and node ids are unique within a graph: a second node with an
equal `node_id` refuses `invalid_semantic_graph` at the second node's `node_id`
(`NO-07`, `NO-08`); sibling nodes at an ordinal other than `0`, at non-contiguous
ordinals or out of graph order are admitted (`NO-05`, `NO-06`). The
node order used wherever this artifact says "ascending node-id order" is
`package` in UTF-8 byte order, then `path` element by element (a string sorts
before an operand; two operands compare by their structural keys, recursively
by this same order; a proper prefix sorts first), then `ordinal`. It is an
order of structural keys, never of graph indexes: the index an index segment
holds takes no part in it. A reader recomputes no
identifier: it checks that every `node_id` is unique, that every reference,
dependency, `semantic_type` and recursion-group member resolves, and that a
declared node's `path` equals its `declaration.qualified_name`. A derived
node's `path` is an identity label: the reader does not compare it with the
node's body or operands, and a `path` that differs from what a producer would
write for that body is not a defect. Within one package a `{package, path,
ordinal}` is simply unique or a duplicate. When the same `{package, path}`
reaches a reader in two places with differing bodies (the diamond of
FR-322-AC-75, where two dependency closures reach one identity as unequal
packages), the reader refuses `invalid_package`/`conflicting-definition`; no
other comparison of a recorded value with a recomputed one exists. A
`scalar_type`/`compound_unit` node is an ordinary derived node whose operands
are, for each unit term in turn, the term's root-unit node followed by its
exponent's text ("Derived type forms and forwarding"), so its path names every
exponent and two compound units never share a path; the evaluator's runtime compound-unit value
(`quire.value.compound-unit/v1`, FR-142) is a different thing and is never a
node id.

### Operand index segments

An operand node is never written inside another node's id. The wire holds it
as an *index segment*, the closed object `{"node": i}` whose one member `node`
is a JSON integer from `0` to 9007199254740991 naming `semantic_graph.nodes[i]`
by its graph index. Any other segment object (a `node_id` object, an extra
member, a missing `node`, a string, negative, fractional or exponent-form
`node`) fails the V2 schema and refuses `malformed_wire`. So a `node_id` has a
fixed JSON depth (the object, its `path` array and an index segment), whatever
the depth of the expression it names, and its size is linear in its path length.
A package of *n* nodes whose paths hold *s* segments in all has node ids of
O(*n* + *s*) bytes, and a left-nested chain of 100,000 applications has one
index segment per operand rather than 100,000 nested ids.

- **Scope.** Every index segment in a checked package document names a node of
  that same document's `semantic_graph.nodes`. This holds for a node's own
  `node_id` and for every `node_id` the document carries anywhere else: a
  body `reference` `target`, a literal's `type`, an application's
  `result_type`, a `dependencies` entry, a `semantic_type`, an `owner.node`, a
  `source_map` entry's `node_id`, the node ids of a nominal, timed, protocol,
  refinement or extension body, and every other `NodeRef`. An imported node
  is present in the importing graph (it keeps its own `package`), so its operands
  are local nodes too, and its index segments name the importing graph.
  An index segment always resolves against the `semantic_graph.nodes` of the
  checked-package document the id belongs to. A `node_id`'s own `package`
  member names the node's owner, not a graph: an imported derived node sits
  in the importing graph under the dependency's `package` (node `2` of
  `fixtures/positive-dependency-call.json`, `Int[0, 9]` of `test/predicates`,
  whose `{"node": 0}` is the importer's index), so its id never resolves
  against the owner's graph.
  - Inside a package document, the id belongs to that document's graph.
  - A document outside any package holds the id exactly as the checked package
    it is about writes it, and resolves it against that package's graph. That
    package is named by one member of the document: a backend request's
    `request.package` ([FR-331](FR-331-backend-provider-envelope.md); an
    in-process request, which has no `package` member, by the checked package
    handed to the provider), a conformance-corpus replay
    request's top-level `package` and any other corpus vector's compiled
    `input` ([FR-336](FR-336-conformance-corpus.md)), and a report or
    diagnostic by the checked package it reports on. The importer's id of an
    imported node, copied into such a document about the importer, resolves
    against the importer's graph.
  - A `ContractPackage` ([FR-330](FR-330-contract-package.md)) is
    self-contained. An id naming an output semantic node resolves in the
    ContractPackage's own output `semantic_graph`. A source-frame id (a
    requested item, a refusal's requested id, an accounting payload id or a
    source-map occurrence `node_id`) is written `{"source_key": i}` and
    resolves in the ContractPackage's own `source_keys` table, whose entries
    are FR-322 node ids whose index segments name other `source_keys` entries.
    No ContractPackage id resolves against the source package's graph.
  - An external `{package, node}` refinement key of
    [FR-375](../../functional/analysis/FR-375-declare-model-refinement.md)
    resolves its `node` against the graph of the dependency named by the key's
    `package` member, not by the `node`'s own `package`.
- **Dependency references.** A `dependency_reference` `node` names a declared
  node of another package's graph, so it holds no index segment: its `path` is
  the declaration path, all strings, and the V2 schema gives `node` the shape of
  a `node_id` whose `path` items are strings. An index segment there fails the
  schema and refuses `malformed_wire`.
- **Order.** Graph order stays the producer's choice. An index segment may name
  a node earlier or later in graph order, and a reader requires no topological
  or sorted order of the nodes. The relation in which node *k* names node *j*
  through an index segment of its own `node_id.path` is acyclic: no node's key
  is defined in terms of itself.
- **Reader checks.** After the V2 schema check and before every
  `semantic_graph` check (so before the duplicate-`node_id` refusal), the reader
  checks index segments and refuses `malformed_wire`, with no cause, at the
  segment's `node` member pointer, such as
  `/semantic_graph/nodes/{k}/node_id/path/{s}/node` or
  `/source_map/{j}/node_id/path/{s}/node`:
  1. an index not below the length of `semantic_graph.nodes`, the first such
     segment in the order of the document's canonical bytes;
  2. otherwise a cycle: the first segment, by the graph index *k* of the node
     whose own `node_id.path` holds it and then by its position *s* in that
     path, whose named node reaches node *k* through own-path index segments.
     A node that names itself is such a cycle.
- **Rebuilding the key.** The structural key of a node is its `node_id` with
  each index segment replaced by the structural key of the node it names; the
  structural key of a `node_id` held anywhere else in the document is built the
  same way. A reader rebuilds every key after the index checks, resolving each
  named node once and sharing its key wherever it is named, so the rebuilt keys
  form a directed acyclic graph of size linear in the package and no key is
  copied into another. Identity, equality, uniqueness, resolution and the node
  order above are defined on structural keys. Within one admitted document
  the rebuilt keys of the nodes are distinct, so two node ids of that document
  are equal exactly when they are equal as wire values; a reader may compare the
  wire values there once the duplicate-`node_id` check has passed. Across two
  documents (an importer and the dependency it imports, or the two packages of
  FR-322-AC-75's diamond), the same node can carry different index segments,
  and only the structural keys are compared.
- **Producing.** A producer interns each operand node in the graph before it
  writes the user's `node_id`, and writes each operand as the index segment of
  that node. Two derived nodes have one structural key exactly when they have
  the same `node_tag`, `semantic_form`, string operands and operand keys, in
  order, so a producer that interns by structural key emits each one once.
- **Notation.** Where this artifact writes an operand of a path as `X's
  node_id`, `X.node_id` or "the `node_id` of `X`", the structural key holds
  `X`'s key and the wire holds `X`'s index segment. The FR-201 `NodeId`
  identity value is the structural key; FR-201 states it with nested operand
  ids because it is a value, not a wire encoding.

### Derived request items

The deadlock-freedom item
([FR-366](../../functional/temporal/FR-366-check-deadlock-freedom.md) over a
model subject,
[FR-429](../../functional/protocol/FR-429-report-protocol-deadlocks.md) over a
protocol subject), the time-lock-freedom item
([FR-417](../../functional/temporal/FR-417-settle-time-lock-freedom.md)) and
the race-freedom item
([FR-438](../../functional/memory/FR-438-report-data-races.md)) are request
items that the request writer derives
([FR-368](../../functional/temporal/FR-368-negotiate-temporal-model-check-items.md)).
They are not package content, so none of them has a node. `semantic_graph`
holds no node for any of them. They have no `node_tag`, no `semantic_form` and
no derived path, and a producer mints no `node_id` for them. This is
checkable: no `semantic_form` of any `node_tag` names a derived item kind,
neither `deadlock-freedom`, `time-lock-freedom` and `race-freedom` nor
`deadlock_freedom`, `time_lock_freedom` and `race_freedom`. The closed V2
schema therefore refuses a node of such a form, and the only node a derived
item's claim names is its anchor. A string in a node's `path` is an identity
label ("Structural node identity" above) and does not make that node a node of
a derived item.

Each derived item is keyed by the triple of its `subject`, its anchor node's
`node_id` and its `item_kind`. That is the obligation identity of
[FR-368](../../functional/temporal/FR-368-negotiate-temporal-model-check-items.md)
"Request members", with the anchor in the place of a claim. The anchor node is
the `claim.node` that FR-368 "Claim and anchor of a derived or refinement
item" writes, and it is an ordinary node of the graph:

| Item | Subject | Anchor node (`claim.node`) | `node_tag`/`semantic_form` | Anchor `node_id` | `claim.occurrence` |
| --- | --- | --- | --- | --- | --- |
| `deadlock-freedom` | model subject | the `model_root` node of the one model selection the subject reads | `model`/`model_root` | `{package: <selected identity>, path: ["model", "model_root"], ordinal: 0}` ("Model root") | `generated`, 0 |
| `time-lock-freedom` | timed model subject | the same `model_root` node | `model`/`model_root` | as above | `generated`, 0 |
| `deadlock-freedom` | protocol subject | the subject's protocol clause node | `protocol`/`protocol_clause` | its declaration path, the protocol's qualified name, at the ordinal the producer gave the node | `declaration`, 0 |
| `race-freedom` | protocol subject with a weak `parallel` | the same protocol clause node | `protocol`/`protocol_clause` | as above | `declaration`, 0 |

The anchor is never embedded as an operand of a derived path, because no
derived path exists for these items. Two derived items of one subject share
an anchor and are told apart by `item_kind`: the deadlock-freedom and
time-lock-freedom items of one model subject, and the deadlock-freedom and
race-freedom items of one protocol subject. Items of one kind over two
subjects with one anchor, for example two sets of initial states over one
model, share the anchor `node_id` and the `item_kind` and are told apart by
`subject`. The anchor and kind alone are therefore not a key; the triple is. No key
is derived from the package's `name` or contents for these items. A key of
that kind would be the package identity, which no item has.

A reader admits a derived item by resolving its `claim.node` by exact
`node_id` in `semantic_graph`. For a `deadlock-freedom` or `time-lock-freedom`
item over a model subject, that must be the `model`/`model_root` node of the
model selection that the subject reads. For a `deadlock-freedom` or
`race-freedom` item over a protocol subject, it must be the subject's
`protocol`/`protocol_clause` node. The reader also checks that
`claim.occurrence` is the occurrence FR-368 chooses: the anchor's
`declaration` occurrence at ordinal 0, else its `expression` occurrence at
ordinal 0, else its `generated` occurrence at ordinal 0. A claim node that the
graph does not hold, a node of another tag, form or subject, or a correct
node with any other occurrence is refused before consumption
([FR-331](FR-331-backend-provider-envelope.md) "Items").
Replay ([FR-364](../../functional/temporal/FR-364-replay-temporal-counterexamples.md),
[FR-433](../../functional/protocol/FR-433-replay-protocol-counterexamples.md),
[FR-438](../../functional/memory/FR-438-report-data-races.md) "Race replay")
resolves the item by the same triple: it resolves `claim.node` to the anchor
in the recompiled package and takes `subject` and `item_kind` from the request
item that the counterexample's `request_index` names. It learns which derived
item it replays from that `item_kind`, never from a node. FR-368
gives some subjects no anchor (FR-368-AC-22, FR-368-AC-24). Such a subject has
no key. The producer settles its derived items `unsupported` and writes no
request item, so nothing is left to key or replay.

## Body grammar

A node's `body` is drawn from a closed grammar of five strata. No production
names itself or a higher stratum, and a `node_id` holds its operands as index
segments, never as nested node ids ("Operand index segments"), so the JSON depth
of every V2 package is fixed by the schema, whatever the model's size.

| Stratum | Productions |
| --- | --- |
| Leaf | `literal`, `reference`, `dependency_reference` |
| Group | `aggregate` whose members are each a Leaf or a `binding` whose value is a Leaf |
| Tuple | `aggregate` whose members are each a Leaf or a Group |
| Member | a Leaf, a Group, or a `binding` whose value is a Leaf, a Group or a Tuple |
| Body | a Leaf; an `application` whose `arguments` are each a Member; an `aggregate` whose members are each a Member; a `frame` body ([FR-340](FR-340-checked-package-frame-location.md)); an `abstraction_relation` body ([FR-451](FR-451-checked-package-abstraction-relation-body.md)); a `refinement` body with the closed row and target grammar of [FR-375](../../functional/analysis/FR-375-declare-model-refinement.md); or an `extension_form` body, whose clause values are each a `reference` ([FR-469](FR-469-checked-package-extension-nodes.md)) |

**Integer literal operands.** The producer SHALL write an integer literal
operand of `quire.op.integer.add`, `quire.op.integer.sub`,
`quire.op.integer.mul`, `quire.op.integer.negate`, `quire.op.integer.eq` and
`quire.op.integer.ne` as an inline `literal` term `{term, type, value,
value_kind}` with `value_kind` `integer` and `type` the builtin Integer node,
and no `value`/`literal` node SHALL stand for it. A literal under a narrowing
conversion (`quire.op.numeric.narrow`), a literal in any other position, and
an operand of composite or non-integer equality SHALL be a graph child, its own
node reached by `reference`. The rule binds the producer: a reader SHALL admit
the reference form of such an operand as well, which yields a different node
identity, and SHALL NOT refuse it for that form.

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
profile fit"). A reader
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

`schema.json` gives `NodeBase` the optional property
`owner`, `{"$ref": "#/$defs/NodeOwner"}`, where `NodeOwner` is
`{"oneOf": [SourceOwner, ModelOwner, ModelRootOwner]}` over these closed
owner definitions. `ModelRootOwner` is `{kind: "model_root", identity,
}` and its `identity` is that of a supplied domain package.
The node definition carries one
further rule, `OwnerPresenceRule`, in its `allOf`. A node is
**structural** for this rule when it is not a node of a recursion group and its `body.term` is not `application` (an application
stands only at a body root, so this is "its body holds no application"). This
includes a `correspondence`/`abstraction_relation` node, which the derived-shape
semantic checks exclude, but the rule never asks it for an `owner`: `DeclarationTagRules`
forbids `declaration` on every `correspondence` node and the form is in
neither model-owned row, so it falls in the last row and carries none.

| Node | `owner` |
| --- | --- |
| structural, carries `declaration` | required, `SourceOwner` |
| structural, no `declaration`, `node_tag`/`semantic_form` `scalar_type`/`dimension_equivalence` | required, `SourceOwner` |
| structural, no `declaration`, `node_tag`/`semantic_form` `model`/`model_root` | required, `ModelRootOwner` |
| structural, no `declaration`, `node_tag`/`semantic_form` `scalar_type`/`dimension`, `scalar_type`/`unit`, `model`/`object_type`, `model`/`record_value_type`, `model`/`tuple_value_type`, `model`/`event_type`, `model`/`variant_type`, `model`/`state_machine`, `model`/`process`, `model`/`persistence_interface`, `model`/`systems_interface`, `relation`/`relationship` or `relation`/`population` | required, `ModelOwner` |
| structural, no `declaration`, `node_tag` `function` | required, `ModelOwner` |
| every other node | absent |

A node that breaks the rule, or whose `owner` is a `DefinitionOwner`, has a
member outside its owner's closed definition, a `ModelOwner` carrying
`version`, or a `ModelRootOwner` carrying an extra member, fails the schema,
and a reader refuses it `malformed_wire` at
`/semantic_graph/nodes/{i}/owner`, or at `/semantic_graph/nodes/{i}` when a
required `owner` is absent (FR-322-AC-51).

## Model-owned members

A `model_member` is resolved in four steps. Step 1 runs before graph
admission. Steps 2 to 4 run at the application's `operator-ineligible`
check, and their refusals take that check's place in the refusal order.
Steps 1 and 2 also serve the `conforming_reference` and `conforming_upcast`
constraints ("Reference conformance" below).

1. **Selection evidence.** After strict syntax and member validation and
   before graph admission, the reader admits the document of each
   `model_root` node, in node order, under FR-154's admission checks in
   FR-154's order: the caller supplies a
   document under the root's identity, else `missing_import`/`missing-selection`;
   and the document's own identity equals the root's, else
   `invalid_model_binding`/`wrong-model-selection`. The reader then reads the
   document's declarations under FR-154 and normalizes them under the
   `quire.model.complete/v1` rules. The selected document is an FCD semantic IR
   2.0.0 domain package (FR-154 step 3; filament-core-data FR-091 and FR-095):
   `contractVersion`, `package`, `constructs`, `types`, `populations`,
   `dimensions` and `units`, each type carrying its
   `fields`, `operations`, `supertypes` and `relationships`, each population its
   members of "Population references" ("Owner join"; filament-core-data FR-141),
   and each dimension and unit node the members filament-core-data FR-146
   states (`dimensions` and `units` may be absent, which is the empty list).
   A field typed `ix://quire/native/Clock` is a clock field of its owning type
   (FR-154). A relationship's IR
   node identity is FCD's, `ix://<org>/<name>/relationship/<Name>-<verb>-<Target>`,
   and it is declared in its source type's `relationships` with `sourceEnd` and
   `targetEnd`. The declarations steps 2 to 4 read are a derived view of that
   document, which holds no member the document does not: the
   published `domain-package-acme-orders.json` is the document and `domain_package`
   in `model-member-type-vectors.json` is that view; likewise
   `domain-package-acme-shapes.json` is the document and `domain_package` in
   `model-tuple-union-vectors.json` its view (FR-322-AC-151). A
   selected document is named by its identity alone: the `model_root` node of
   a published fixture names the document (the
   published `acme/orders` and `acme/shapes` fixtures alike). Admission is as
   FR-322-AC-32 states. The read's refusal is the first refusal
   in FR-154's report order, with its own code, cause and loci, at stage
   `package_read`. The caller supplies `ModelNormalizationLimitsV1` among the
   reader's limits, and an exhausted one is the read's `incomplete` result.
2. **Declaration lookup.** A declaring node carries its `ModelOwner` as
   `owner`, and the owner join has already joined that owner
   to a `model_root` node. The declaration `T` is the one that the
   admitted document of the selection whose `identity` equals the owner's
   `identity` declares under the IR node identity equal to the owner's
   `node`, and of the kind the node's `node_tag`/`semantic_form` names: an
   object type with no `interfaceFeatures` for `model`/`object_type`, a
   record value type of FR-208's `quire.meaning.model.record-value-type/v1`
   for `model`/`record_value_type`, a tuple value type of FR-208's
   `quire.meaning.model.tuple-value-type/v1` for `model`/`tuple_value_type`, an
   event type of FR-208's
   `quire.meaning.model.event-type/v1` for `model`/`event_type`, a variant type of
   FR-208's `quire.meaning.model.variant-type/v1`, with or without case
   payloads, for `model`/`variant_type`, a
   state machine of FR-208's `quire.meaning.model.state-machine/v1` for
   `model`/`state_machine`, a process of FR-208's
   `quire.meaning.model.process/v1` for `model`/`process`, a persistence
   interface of FR-208's `quire.meaning.model.persistence-interface/v1` for
   `model`/`persistence_interface`, an object type with `interfaceFeatures` for
   `model`/`systems_interface`, a relationship for `relation`/`relationship` and
   a population of FR-208's `quire.meaning.model.population/v1` for
   `relation`/`population` ("Population references").
   A process and a state machine declare fields, supertypes of their own meaning
   and operations, so step 3 resolves their members as it does an object
   type's. FR-321 admits one selection
   per `identity` and a document declares each IR node identity once, so at
   most one declaration matches. The owner join
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
   (`quire.model.complete/v1` phases 3 and 4). A field or an operation
   redefines exactly the member its `redefines` member names (FR-154), so
   an operation redefinition hides the operation it names as a field
   redefinition hides a field. A member that only shares an inherited
   member's name hides nothing. A `field` member considers the
   field members and an `operation` member the operation members. A member
   matches when the last `/` segment of its IR node identity equals `name`,
   byte for byte. A `position` member of a `model`/`tuple_value_type` node
   resolves among the positions of `T` and matches when its `position`
   equals the zero-based index `i` of a position of `T`, whose identity is
   `<T's node identity>/p<i>` ("Tuple value types and payload-bearing variant
   types"); positions have no names, so there is no ambiguity, and a
   `position` at or above the number of positions of `T` matches none.
   Exactly one match resolves the member; two or more refuse
   `ambiguous_declaration`/`ambiguous-name`, including a supertype's
   operation and a subtype's operation of one name with no `redefines`
   between them, which FR-150's effective view
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
   | `ix://quire/native/Bytes` | `Bytes`, an octet sequence of any length (the empty sequence included), compared by its canonical payload under FR-149's Bytes row; operations `length` and `concat`, no ordering and no other arithmetic | `scalar_type`/`bytes` |
   | `ix://quire/native/JsonObject` | `JsonObject`, a JSON value of any kind (object, array, string, number, boolean or `null`), compared by its canonical payload under FR-149's JsonObject row; opaque: equality only, no member access, no ordering and no arithmetic | `scalar_type`/`json_object` |
   | `ix://quire/native/Duration` | `Duration`, an amount of elapsed time: the type `Quantity<s>` where `s` is the unit named `s` that the member resolves, and `s` is the 1-second unit of its own owner's `Time`. A member declared by a selected package resolves the unit `s` that its own selection declares (`ix://quire/unit/<p>/s`, `<p>` that selection's identity), and only when that selection declares no `s` the unit `s` the unit of compilation declares; a member declared by the unit of compilation resolves the unit `s` the unit of compilation declares (`ix://quire/unit/s`). A source `s` and a package `s` are two distinct units (FR-142), so a unit of compilation and a package that both declare `s` never make a `Duration` ambiguous. The resolved `s` SHALL be typed by a base dimension whose declared name is `Time` and that its own owner declares (the unit of compilation's `Time` for a source `s`, the selection's `Time` for a package `s`, never another owner's `Time` of the same name), and SHALL have exact scale 1 and offset 0 to the root unit of that dimension: the root unit itself, or a unit whose composed edges to the root have scale 1 and offset 0. A `Duration` member and a `Quantity<s>` member have one type, with the exact rational magnitude in seconds that "Scalar literal values" spells; a quantity in another unit of `Time` reaches `Duration` only through FR-142's explicit conversion to `s`. If no `s` is resolved the member refuses `missing_declaration`/`missing-name`; if the resolved `s` is typed by a dimension that is not such a base dimension named `Time`, or has a scale other than 1 or an offset other than 0 to the root, the member has no type and the application refuses `ill_typed`/`operator-ineligible` | the `scalar_type`/`unit` node of the resolved `s` (family `quantity`); no node is introduced |
   | `ix://quire/native/Float32` | `Float32`, an IEEE binary32 value (FR-148); a field names no rounding mode, so the type is strict `exact`; a field value is compared by `=` under numeric equality (FR-149) and by its bits in a collection (FR-144) | `scalar_type`/`float32` |
   | `ix://quire/native/Float64` | `Float64`, an IEEE binary64 value (FR-148), with the same rules as `Float32` | `scalar_type`/`float64` |
   | `ix://quire/native/Clock`, or an alias of it, on a field, an operation parameter or an operation result with FR-154's clock member shape (presence `optional`, or a result of multiplicity `{0, 1}`, is `Option<Clock>`) | `Clock`, the clock type of FR-415: a clock field is one clock per object of its owning type (and per record or tuple path that holds it), holds no data value, is read only as the clock operand of an atomic clock constraint (in a guard or a time invariant) and is written only by a reset (FR-415); a clock parameter or result is a clock place passed or returned by place (FR-415 "Clock places") | none: no node stands for a clock. This artifact carries no clock constraint, reset or timed-model member; the checker reads a clock operand there by (declaring node, field path) and records the timed model (FR-415, QSL FR-230). An operation member whose parameter or result is a clock member has that member's type from the row and no node for the parameter or result; the operation's other members are typed as usual. A record value type with a clock field has the `model`/`record_value_type` node of its data fields only, and its clock fields are read by (declaring node, field name) as an object type's are. The multiplicity table below applies to a clock member: `{0, 3, true, false}` makes it a `Sequence<Clock>[0, 3]`, and a `sequence` or `map` type whose `items` or `values` is `Clock` is a `Sequence<Clock>` or `Map<Text, Clock>` (FR-415 "Clock collections"). A clock collection whose element holds data (a `Sequence<Timer>`) is typed by the `composite_type` node of its kind over the element's **data part** node, the `record_value_type` node that lists the data fields only, with the `collection_bounds` node of its bounds when it has them; a clock collection whose element holds no data (a `Set<Clock>`, a `Bag<Clock>`, a `Sequence<Option<Clock>>`) has no node, and the timed model records its kind, bounds and key domain with the clock place, so that the shape reads (`size`, `present`, an index or key being in range) are recorded data predicates of the timed model and read no clock. A `field` application whose member resolves to a clock field or a clock collection in any other position, a value position, refuses `invalid_timed_model`/`clock-read-outside-constraint` |
   | a value type (FR-208 `quire.meaning.model.value-type/v1`) of the same domain package bound as `Int[lo, hi]`, `lo <= hi`, as TC-196's `Count` and `Small` and TC-197's `Count` are | `Int[lo, hi]` | `bounded_domain`/`integer_range` with bounds `lo` and `hi` over the `Integer` node |
   | a value type of the same domain package bound as `Text[min, max; profile]`, `0 <= min <= max`, with `profile` one of FR-141's `unicode-scalars`, `nfc`, `nfd`, `nfkc`, `nfkd` and `binary-utf8` | `Text[min, max; profile]`, whose length, bounds and comparison are FR-141's under `profile` | `bounded_domain`/`text_bounds` with bounds `min` and `max` and `text_profile` `profile` over the `scalar_type`/`text` node |
   | a record value type `R` of the same domain package, admitted under FR-208 | `R`, a value with no object identity | the self-typed `model`/`record_value_type` declaration node owned by `R`; no `composite_type`/`reference` is introduced |
   | a tuple value type `P` of the same domain package, admitted under FR-208 | `P`, an identity-free ordered product of the types of its positions `p0` to `pN-1` | the self-typed `model`/`tuple_value_type` declaration node owned by `P`; no `composite_type`/`tuple` and no `composite_type`/`reference` is introduced |
   | a variant type `V` of the same domain package, admitted under FR-208, none of whose cases carries a `payloadType` | `V`, an unordered enumeration whose cases are the case names of `V` | the self-typed `model`/`variant_type` declaration node owned by `V`, family `enum`; no `composite_type`/`reference` and no unit-owned `scalar_type`/`enum` node is introduced |
   | a variant type `V` of the same domain package, admitted under FR-208, at least one of whose cases carries a resolving `payloadType` | `V`, a union whose members are the cases of `V` in ascending UTF-8 byte order of case name, a case with a `payloadType` being a member of one payload position and a case without one a nullary member | the same self-typed `model`/`variant_type` declaration node owned by `V`, family `union` ("Tuple value types and payload-bearing variant types"); no `composite_type`/`union` node is introduced |
   | `ix://quire/unit/<name>`, naming either a unit the unit of compilation declares, by its qualified declaration name with its segments joined by `/` and never a dimension segment (`unit ms : Time` is `ix://quire/unit/ms`), or a unit an admitted domain package declares, by its FCD identity's package identity and `name` (`ms` in the `units` of `acme/rpc`, FCD identity `ix://acme/rpc/unit/ms`, is `ix://quire/unit/acme/rpc/ms`) | `Quantity<u>`, the type of a quantity in unit `u`, whether the unit of compilation or an admitted domain package declares `u` | the `scalar_type`/`unit` node of `u` (family `quantity`); no node is introduced, and a `min` or `max` constraint on the unit (FR-154 "Unit bounds") adds none: the member's node stays this node, its bounded type `Quantity<u>[min, max]` is the checker's refinement, and the checker reads the bound from the admitted document |
   | a `reference` type whose `target` is `ix://quire/native/Clock` or an alias of it | `Reference<Clock>`, a data value naming one clock place (FR-415 "Clock places"); compared by place, never by clock value | none: no node stands for the clock or for the reference; the checker reads `ref` and `deref` over it and the state key holds the place id (FR-415 "Clock places") |
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
   multiplicity gives. The unit graph of the checked package holds the
   dimensions and units the unit of compilation declares and those of every
   admitted domain package (FR-154). Each dimension and unit an admitted
   domain package declares is one checked-package node: a
   `scalar_type`/`dimension` or `scalar_type`/`unit` node that carries no
   `declaration` and carries as `owner` the `ModelOwner` `{kind: "model",
   identity, node}` of its selection, `identity` the selection's identity and
   `node` its FCD identity (`ix://acme/rpc/unit/ms`). Its `node_id.path` is
   `[node_tag, semantic_form, <FCD identity>]`, the rule of every node without a
   `declaration` (`["scalar_type", "unit", "ix://acme/rpc/unit/ms"]`, and
   `["scalar_type", "dimension", "ix://acme/rpc/dimension/Time"]` for a dimension), and its body is
   `{term: aggregate, members: []}`; its values are read from the selected document.
   The node carries no `declaration`, under
   the `ModelOwner` exception of the `declaration` rule. Its `node_id` stays
   distinct from a source unit `ms`'s (a different `package` and `path`), whose `declaration` carries the same
   `qualified_name` under a `SourceOwner`; a dimension's `terms` are
   its FCD terms and a unit's `dimension`, `target`, `scale` and
   `offset` are its FCD `dimension`, `target`, `scale` and
   `offset`, each read from the document and resolved to those nodes (a source dimension and unit
   hold them as `terms`, `dimension_node_id`, `target_unit_node_id`, `scale` and `offset` in their bodies). A dimension node is self-typed and
   depends exactly on the dimension nodes of its terms; a unit node's
   `semantic_type` is its dimension node and it depends exactly on that node
   and its non-null target. A `typeRef` `ix://quire/unit/<p>/<name>` whose
   leading segments `<p>` equal the identity of a `model_root` owner
   names that package's unit `name`; any other `typeRef` names a source unit by
   its qualified declaration name. A `typeRef` that names both a source unit
   and a package unit refuses `ambiguous_declaration`/`ambiguous-name`. A
   source names a package unit as `M::ms` through the row's model alias
   (`Quantity<M::ms>` after `model M = "acme/rpc"`) and a package dimension
   as `M::name` (FR-142 "Namespaces"). Each package dimension is its own node under its
   selection's `ModelOwner`, so two packages that each declare a dimension
   `Time` hold two nodes with distinct `node_id`s; a source dimension term or unit
   dimension that names one lowers to that node's `node_id` as `terms` entry or
   `dimension_node_id`, and the qualified spelling leaves no other trace in the
   package.

   A dimension equivalence (FR-142 "Dimension equivalence", the source
   declaration `dimension R::Time ~ F::Time by 1 R::s = 1000 F::ms;`) is one `scalar_type`/
   `dimension_equivalence` node, appended to the unit graph beside the
   dimension nodes and never changing one: the nodes it relates keep their
   `node_id`s. It is self-typed and carries no `declaration`. It carries the
   `SourceOwner` of the source that declares it, and its
   `body` is the closed `{term: "dimension_equivalence", dimension_node_ids, root_scale}`, where `dimension_node_ids` lists the `node_id`s of the two
   related dimension nodes, distinct and in ascending node-id order, which may
   belong to different packages of a consistent multi-package graph (the order compares `package`, then
   `path` element by element, then `ordinal`, and each id resolves to a `scalar_type`/`dimension`
   node of the admitted package set), and
   `root_scale` is the exact positive rational (`numerator`, `denominator`
   decimal strings, reduced, the numerator at least 1) for which one canonical
   root of the first listed node equals `root_scale` canonical roots of the
   second (the root ratio FR-142 "Dimension equivalence" reduces the `by`
   clause to; a declaration written in the other node order stores the
   reciprocal), so `R::Time ~ F::Time by 1 R::s = 1000 F::ms` and
   `F::Time ~ R::Time by 1000 F::ms = 1 R::s` write one body. The node's
   `node_id.path` is an assigned identity label the producer chooses; no rule derives it
   from the body. The reader checks only the structural shape of the id, the
   uniqueness of its {`package`, `path`, `ordinal`}, that `dimension_node_ids`
   resolve to two distinct dimension nodes in ascending node-id order, and
   path equality with a `declaration` only where a declaration exists, which it
   does not here (the published
   fixture's path `["scalar_type", "dimension_equivalence", <first id>, <second id>]`
   is an example convention only). It depends on exactly those two nodes, and its one occurrence has role
   `anchor` and the ordinal of the declaration among the equivalence
   declarations of its source in source order, with the declaration's region.
   A reader recomputes the equivalence classes from these nodes alone: the
   connected components of the pairs `dimension_node_ids`, with the base classes
   the components of the pairs whose two nodes are both `terms`-empty
   `dimension` nodes, and applies FR-142's effect rules to the `node_id`s it
   reads, so no package member records a class, a root or a conversion edge.
   Each pair is a pair of `scalar_type`/`dimension` nodes, else the reader
   refuses `invalid_semantic_graph`; a pair repeated by two nodes, a pair of one
   node twice, a pair of two nodes that are not both base dimensions and
   whose normalized maps differ (under the base classes) or whose `root_scale`
   differs from the ratio the base roots fix, and a cycle of base pairs whose
   `root_scale` values do not multiply to exactly one (a pair read against its
   listed order counting as its reciprocal) refuse
   `invalid_semantic_graph` at the later node in node-id order, as FR-142
   refuses the source declaration. Wire precedes graph: the closed V2 schema
   requires `root_scale`, bounds `dimension_node_ids` to exactly two entries,
   admits only a positive-integer `numerator` and `denominator`, and forbids a
   `declaration` member on this node (its one occurrence has role `anchor`), so
   a missing `root_scale`, a zero or negative `numerator` or `denominator`, a
   `dimension_node_ids` list of other than two entries and a `declaration`
   member refuse `malformed_wire` at the node, never `invalid_semantic_graph`.
   A schema-valid but unreduced `root_scale` (`2000/2`) refuses
   `invalid_semantic_graph` at the node. A package holding no equivalence node
   reads as before. A unit `typeRef` that names no unit of the checked package refuses
   `missing_declaration`/`missing-name`. A declared type outside the table, an `unbounded`
   upper bound with a lower bound above `0` and an operation with no result
   give the member no type, and the application refuses
   `ill_typed`/`operator-ineligible`.

   Each node the tables name has no `declaration`; the record value type, the
   tuple value type and the variant type are model-owned, while the other
   derived types are anonymous structural nodes,
   exactly as QSL FR-092 and FR-094 identify them. The node that
   stands for a type, and every application node id that references it, is
   canonical.

### Tuple value types and payload-bearing variant types

FR-154 admits a tuple value type (FR-208 `quire.meaning.model.tuple-value-type/v1`)
and a variant case with a resolving `payloadType` (FR-154-AC-11, FR-154-AC-25,
FR-154-AC-33; FR-208-AC-15, FR-208-AC-19). This section is their V2 wire shape.
It adds one node form, `model`/`tuple_value_type`, and changes the meaning of
one, `model`/`variant_type`; it adds no wire member, no `composite_type` node and
no operation identity, and the published operation catalog is unchanged.

- **Tuple declaration node.** `node_tag: "model"`, `semantic_form:
  "tuple_value_type"`. It is a model declaration node ("Model-owned members"
  step 2) in every respect of `record_value_type`: no `declaration`
  or `recursion_group`; `semantic_type` its own
  `node_id`; the body `{term: "aggregate", members: []}`; empty `dependencies`;
  and an `owner`, a `ModelOwner` `{kind: "model", identity, node}` whose
  `identity` is the identity of the domain package that
  declares the tuple and whose `node` is the tuple's owner-nested IR node
  identity `ix://<package identity>/<tuple name>`. Its family is `tuple`. The node
  holds no position: the positions `p0` to `pN-1` (`N` at least one), the
  position identities `<tuple node identity>/p<i>` and each position's
  `typeRef` are read from the selected document, as a record's fields are. A
  position is not a node: it has no `owner`, no `node_id`, no occurrence and no
  `source_map` key (an occurrence key is `node_id`, role, ordinal), and a
  tuple node's occurrences and `source_map` entries follow the rule that
  governs every other model declaration node.
- **Tuple identity.** The node's `node_id` is `{package, path, ordinal}`
  with `package` the domain package's identity and `path` the tuple's
  declaration path, so it is a function of the `owner` alone: the same value for
  every alias, and two tuples of one shape and two names have two `node_id`s.
  FR-208-AC-19's tuple type identity, the ordered ordinal/type pairs, is carried
  by the selected document, not by the `node_id`. Reordering a tuple's
  position/type pairs changes the document and leaves the tuple's `node_id`
  (FR-208's declaration identity) unchanged. A consumer SHALL therefore take the
  type identity of a tuple node as the pair of its `node_id` and its `owner`'s
  `identity` (FR-321 admits one selection per `identity`).
- **Position member.** A position is read by `quire.op.tuple.project`, whose
  operand has family `tuple` and whose `member` is `{kind: "position",
  declaration, position}`, `declaration` naming the tuple's node and `position`
  a `KeyOrdinal`. `member_of` holds when operand 0 resolves to that node. Step 3
  resolves the position and step 4 derives its type from the position's
  `typeRef` as a field of presence `required` and multiplicity `[1, 1]` (a
  position has no presence, multiplicity or name, so a position type is never an
  `Option`, a collection or a `Reference`): `p0: Boolean`, `p1: Integer` of the
  tuple `Pair` give `quire.op.tuple.project` over a `Pair` value with position
  `0` the `Boolean` node and with position `1` the `Integer` node. A `position`
  that is not a position of the tuple, and a `declaration` that is not the
  operand's tuple node, refuse `ill_typed`/`operator-ineligible` at the
  application node, the RFC 6901 pointer `/semantic_graph/nodes/<i>` of the
  `quire.op.tuple.project` application (`/semantic_graph/nodes/31` in
  `fixtures/positive-model-tuple-union.json`), and never at a member of it:
  not `/semantic_graph/nodes/<i>/body/operation/member`, nor its `/position` or
  `/declaration`. A `position` that is no `KeyOrdinal` fails the V2 schema and is
  `malformed_wire` at the same node (FR-322-AC-152). In FR-272's terms the
  refusal's `locus` is the `CheckedNodeId` of that node and its `document_pointer`
  is the offending field, `/semantic_graph/nodes/<i>/body/operation/member/position`;
  the vector's `locus` string is an RFC 6901 pointer into the package, not a
  `CheckedNodeId`, and here names the node, where the field-level cases of a
  tuple node (TW-02 to TW-04) name the field. A `typeRef` that names another tuple or a variant
  type gives that type's declaration node, so a tuple may nest tuples and
  unions. A `composite_type`/`tuple` node and a `model`/`tuple_value_type` node
  are two different type nodes even when their positions have equal types.
- **Payload-bearing variant types.** `model`/`variant_type` remains the one node
  form of a FR-208 variant type, with the unchanged declaration node, `node_id` and
  owner join. Its family is read from the selected document: `enum` when no case
  carries a `payloadType`, `union` when at least one does (the mixed type
  `U { a, b(Boolean) }` is a union with a nullary member `a` and a one-position
  member `b`). The members of the union are the cases in ascending UTF-8 byte
  order of case name. This is the member declaration order that FR-440 and the
  structural leaf walk use for a model union, because FR-208 gives the order of
  `variants` no meaning. A member has payload arity 0 without a `payloadType`
  and 1 with one, its payload type at position `0` being the step 4 type of the
  `payloadType` as a member of presence `required` and multiplicity `[1, 1]`.
  The `node_id` does not name a case or a payload, so a type that gains or loses
  its first payload case changes the selected document and not the `node_id` of
  the `variant_type` node.
- **Union reads over a model union.** Where FR-440 reads the bindings of a
  `composite_type`/`union` body, the reader reads the members, arities and
  payload types of a `model`/`variant_type` node of family `union` from the
  selected document as above, and applies FR-440's joins otherwise unchanged: a
  `value`/`union_value` node whose `semantic_type` is such a node names one of its
  cases with one payload term of the payload type for a payload case and none for
  a nullary case, else it refuses `ill_typed`/`type-mismatch`; a `union_value`
  whose `semantic_type` is a `variant_type` node of family `enum` refuses
  `ill_typed`/`type-mismatch`; an `expression`/`case` over such a union has one
  arm per case in member order, each arm binder having the payload type; and
  `quire.op.structural.eq` and `quire.op.structural.ne` are admitted for two
  values of one such node, while the `enum` operations refuse
  `ill_typed`/`operator-ineligible`, as they refuse a `composite_type`/`union`
  operand. A nullary case of a union is a `union_value` node with the empty
  payload `aggregate`, never an `enum_value` node. The text leaves of a model
  union are those of its payload types under `member:<Name>` then `position:0`.
- **Admitted payload and position types.** A tuple position `typeRef` and a variant
  case `payloadType` are typed by the step 4 table as a field of presence `required`
  and multiplicity `[1, 1]`, and each is admitted exactly when a row of that table gives
  its `typeRef` a type (a `Reference<O>` row for an object type `O` of the package, a
  `Quantity<u>` row for a unit `u`, or a row for a native or value type): the natives `Boolean`, `Integer`, `String`, `UUID`, `Timestamp`,
  `Bytes`, `JsonObject`, `Duration`, `Float32` and `Float64`, a unit, an object type of the package (typed `Reference<O>`), and the package
  record, tuple and variant types and bounded value types. Four declared types have no
  row, and each is refused (not mapped, and never inferred): the native
  `ix://quire/native/Rational`, the native `ix://quire/native/Decimal`, the native
  `ix://quire/native/Text` with no `Text[min, max; profile]` bound (a bare `Text`, or a
  package value type bound as `Text` without `min`, `max` and `profile`), and any other
  package value type that no row binds (so a value type bound as a `Decimal[...]` or
  `Rational` domain is refused too until a row maps it; a later row is additive). An alias
  is read through to its target before this test. `Bytes` and `JsonObject` are unbounded
  octet and JSON values but have rows, so they are admitted; the refused set is the four
  types above, not "unbounded" as a class. The refusal is raised in step 1, at stage
  `package_read`, before any graph refusal and with FR-154's codes for the same site, and
  FR-154 itself admits these natives (FR-154-AC-17), so the refusal is this reader's: a
  `payloadType` of one is `unsupported_construct`/`declaration-form` at
  `Variant.payloadType`, a position `typeRef` of one is
  `invalid_model_binding`/`malformed-declaration` at the position, exactly as for
  `ix://quire/native/Clock`. A consumer of the selected document (the semantic IR intake,
  the FCD reader) that admits such a `typeRef` as a payload or position refuses at its
  intake with the same code and locus; it never carries the type on to an operation
  lookup. A *member* (field, parameter or result) of one of the four types is not an
  intake refusal: it gives the member no type and the application refuses
  `ill_typed`/`operator-ineligible` ("A declared type outside the table"), as a bare `Text`
  field does (FR-322-AC-31).
- **Refusals.** In FR-322's order: the closed V2 schema refuses (`malformed_wire`)
  a `tuple_value_type` node with no `owner`, with a `SourceOwner` and no
  `declaration`, with a `declaration` beside a `ModelOwner`, or with a
  `ModelOwner` carrying `version`. Step 1 refuses every intake defect with
  FR-154's code at `package_read`: no position, a gap, an absent or unsupported
  position `typeRef`, or a tuple that declares fields, identity fields,
  operations, supertypes or clauses is `invalid_model_binding`/`malformed-declaration`;
  an unresolved position `typeRef` is `missing_declaration`/`missing-name`; two
  positions of one ordinal are `invalid_model_binding`/`conflicting-binding`; an
  unresolved or unsupported `payloadType` is `unsupported_construct`/`declaration-form`
  at `Variant.payloadType`. A model declaration node that breaks its form's shape (a
  non-empty body, a `semantic_type` that is not its own `node_id`, or a
  `recursion_group`) refuses `invalid_package`/`invalid-derived-shape` at the
  node's `node_id`. The owner join refuses a `tuple_value_type` node whose
  owner `node` the document does not declare as a tuple value type (a record value
  type, an object type or an absent node) `missing_declaration`/`missing-selection`
  at the node.
- **Intake refusal order.** Step 1 reports the first refusal in this order: each
  tuple of `tuple_types` in document order, taking (1) two positions of one
  ordinal, `conflicting-binding` at the later position; then (2) no position, or a
  tuple member other than `node` and `positions`, `malformed-declaration` at the
  tuple; then (3) the first position whose ordinal is not its index or whose
  `typeRef` is absent, `ix://quire/native/Clock` or one of the four refused types above,
  `malformed-declaration` at the position; then (4) the first position whose `typeRef` names no package type or
  native, `missing-name` at the position; then each variant of `variant_types` in
  document order, the first case whose `payloadType` is unresolved,
  `ix://quire/native/Clock` or one of the four refused types above, `unsupported_construct`/`declaration-form` at
  `payloadType`. The tuples are all read before any variant type, so a document
  with a defect of each kind reports the tuple's.
- **Recursion.** Model types may refer to one another and to themselves through
  position `typeRef`s and `payloadType`s: `Cell(Integer, List)` with
  `List { Nil, Cons(Cell) }` is admitted by intake, as FR-440 admits a recursive
  union and FR-322-AC-46 a recursive record. A model declaration node never
  carries a `recursion_group`, and its `node_id` is a function of `owner` alone, so a
  cycle among model types has no group to resolve: the cycle exists only in the
  selected document, where a reference is the other type's declaration identity. A reader does not decide whether a recursive
  type has a finite value. A type with no nullary or base case has no
  `union_value` constructible from finite terms and the package carries none; it
  is not refused. The structural leaf walk out of a model tuple or union follows
  the positions and the members above, ends a branch that re-enters a model tuple
  or union already open on the current path, and contributes the `recursion:<d>`
  leaf of FR-440, `d` the path length at which the re-entered type was entered,
  exactly when a text leaf is reachable from that type; a recursive type that
  reaches no text, such as `List` above, has no leaf (FR-322-AC-147).
- **Occurrence roles.** The schema's `DeclarationOccurrenceRule` governs: a
  `declaration` member is present if and only if a `declaration`-role occurrence is
  present, with `enum_value` exempt as it is for every nominal member node. A
  model declaration node has no `declaration` member, so its occurrence takes the
  role `expression` and its `source_map` region is the region of the declaring
  construct; a `tuple_value_type` node follows the rule of every other model
  declaration node. The `model_root` node's occurrence is `generated`
  (FR-322-AC-132). An `enum_value` node over a model variant type takes the role
  `declaration`, as the `enum_value` nodes of `positive-nominal-identities.json` do.
- **Model root.** A tuple, a variant type or a payload adds no `model_root` node
  and changes none: a selection still has exactly one root, determined by its
  identity alone (FR-322-AC-132).

The published vectors are `model-tuple-union-vectors.json` (the declarations,
the position, family, member, union-read, recursion and intake cases and the
wire cases), and the
package fixture `fixtures/positive-model-tuple-union.json`, whose application
nodes are a `union_value` over the body-less `Shape` and `List` nodes, an
`enum_value`, two `tuple.project`, a `case` and three equalities.

### Derived type forms and forwarding

The following forms are part of the closed V2 semantic vocabulary. They are
anonymous structural nodes: each has `declaration: null`, no `owner`
and no `recursion_group`. A source type alias
introduces no node. After resolving an alias, every type annotation and every
application result type names the resolved type's node, so an alias cannot
create a second `composite_type`, `bounded_domain` or `scalar_type` identity. A
`Quantity<u>` annotation likewise introduces no node: it names the
`scalar_type`/`unit` node of `u`, the type of a quantity in that unit (family
`quantity`), under the `node_id` of that unit's node (FR-142).
A quantity literal `n u` lowers to a `literal` term of `value_kind` `quantity`
whose `type` is that node and whose `value` is the magnitude spelled as "Scalar
literal values" says.

| Source type | Node and semantic type | Body | Structural `path` |
| --- | --- | --- | --- |
| `Reference<O>` | `composite_type`/`reference`, self-typed | `aggregate{[reference(M_O)]}`, where `M_O` is `O`'s model declaration node. A derived reference target SHALL be a `model`/`object_type`, `model`/`process`, `model`/`state_machine`, `model`/`systems_interface` or `relation`/`relationship` declaration; scalar, value, function and other composite nodes are refused. | `["composite_type", "reference", M_O's node_id]`: `Ref = Reference<Order>` and a parameter typed `Ref` both name the one node with that path |
| `Population<O>[N]`; `Population<O>` with no maximum | `bounded_domain`/`model_population`, semantic type `S_O`, where `S_O` is `composite_type`/`set` over `R_O` | `aggregate{[binding "max" = literal integer N]}`; for `Population<O>` with no maximum, `aggregate{[]}`, the `max` member absent | `["bounded_domain", "model_population", S_O's node_id, "N"]` with `N` the maximum's decimal text, and `["bounded_domain", "model_population", S_O's node_id]` for the unbounded population: `Bucket = Population<Order>[3]` and `Population<Order>[5]` name two nodes, and `Open = Population<Order>` a third whose `semantic_type` is the same `S_O` |
| a formed quantity over a compound unit | `scalar_type`/`compound_unit`, self-typed | `aggregate` of one `aggregate{[binding "unit" = reference(U), binding "exponent" = literal integer e]}` per root unit, in `quire.value.compound-unit/v1` order; dimensionless is `aggregate{[]}`. Each `U` is a targetless canonical nominal root (a `scalar_type`/`unit` node with an empty body and `semantic_type` equal to its dimension node). | `["scalar_type", "compound_unit", U_1's node_id, "e_1", ..., U_n's node_id, "e_n"]`, one node id and one exponent string per term, the terms in the body's order: ascending node-id order of their root units, the order of `quire.value.compound-unit/v1` and of FR-201's `UnitId` `terms`. `e_i` is the exponent's canonical decimal-integer text: an optional leading `-`, then ASCII digits with no leading zero, and no `+`. `metre / second` has the path `["scalar_type", "compound_unit", U_metre's node_id, "1", U_second's node_id, "-1"]` when `U_metre` sorts first; `metre^2` has `["scalar_type", "compound_unit", U_metre's node_id, "2"]` and `metre^-1` has `["scalar_type", "compound_unit", U_metre's node_id, "-1"]`, two paths, so neither relies on an `ordinal` to differ; the dimensionless unit has `["scalar_type", "compound_unit"]`. Equal terms give one path and one node, which a producer keys at `ordinal` `0`. |

A derived shape is identified by its structural path and by nothing else,
independently of a package version and of a source alias spelling.

A self-typed structural `scalar_type`/`unit` is not an alternative root: the
published V2 schema does not admit a structural unit node, which is always a
nominal node with its declaration path.

The corresponding checks are closed, with the stage that owns each check made
explicit. A source alias that names no type refuses
`missing_declaration`/`missing-name` before lowering. The derived-shape
semantic checks examine the complete body.

The named refusal vectors are:

| Mutation vector | Mutation | Refusal |
| --- | --- | --- |
| `RM-REF-TARGET` | Change R1's `reference(M1)` target to a scalar node; the schema remains valid. | `invalid_package`/`invalid-derived-shape` at R1. |
| `RM-POP-MAX` | Change PO1's integer `max` to negative `-1`; the schema remains valid. | `invalid_package`/`invalid-derived-shape` at PO1. |
| `RM-CU-ORDER` | Swap U2's two unit terms; the schema remains valid. | `invalid_package`/`invalid-derived-shape` at U2. |

The executable reader matrix covers the accepted and refused cells below:

| Derived shape | Accepted form | Refusal mutations exercised |
| --- | --- | --- |
| `composite_type`/`reference` | body target is a model declaration node | scalar, function and value targets |
| `bounded_domain`/`model_population` | `semantic_type` is `set<reference<T>>`; either one canonical nonnegative integer `max` binding (a bounded population) or no member (the unbounded population) | negative `max`; non-integer `max`; a member beside `max`; arbitrary set element |
| `scalar_type`/`compound_unit` | self-typed node; targetless canonical nominal unit roots whose semantic type is the dimension node; nonzero exponents in strict node-id order | non-self-typed node; non-unit root; reordered terms |

The required-`max` rule applies to a bounded population only. A
`model_population` node whose `max` member is absent is the unbounded
population, the node a source `Population<O>` with no maximum derives. It is
admitted, with the same `semantic_type` rule and the empty `aggregate` body, and
its path differs from that of every bounded `Population<O>[N]` and from that of
the unbounded `Set<Reference<O>>` (`composite_type`/`set` alone, with no
`collection_bounds`). No `max` is invented for the unbounded node: the absent
`max` is not `0` (its path differs from the path of the node with `max` `0`) and
is not any other bound, and a `max` that is present is a bounded population read
as before. A `population` link of a `quire.op.model.reaches` application that
names an unbounded population node is admitted and states no bound for the
application.

The reader checks the semantic shape: a
`composite_type`/`reference` whose body target is not a model declaration
node, a `bounded_domain`/`model_population` whose `semantic_type` is not the
`set<reference<T>>` node or whose `max` is negative or not an integer
literal, or which has any member other than a single `max`, and a
`scalar_type`/`compound_unit` whose root is not a
targetless canonical nominal unit, whose exponent is zero, whose terms are
duplicated or not in canonical order, or whose body is not the stated
aggregate is refused by the semantic shape check. Adding a declaration, owner
or nominal body to any of these
anonymous forms fails the V2 schema or the owner/declaration rules. An alias
that resolves to a valid form has no separate refusal
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
constraints"). The projection's operand is that `deref` application, its
`member` names the field and declaring node and carries `read_form`, and its
`result_type` is the step 4 type of the resolved field. The closed `read_form`
values are `bare` for a source field read such as `view.total`, and
`explicit_deref` for a source read such as `deref(view).total`. A model-object
projection SHALL carry exactly one of those values; the reader refuses a
missing or inapplicable `read_form` as `malformed_wire`, and the schema refuses
an unrecognized value. The reader preserves `read_form` in the admitted graph.
This member-level distinction keeps the existing `quire.op.model.deref`
operation and its catalog mode unchanged while making the source read form
available to charge-producing consumers. It extends the already-catalogued
`field` member without adding an operation identity or changing FR-466's
protocol operator list. Encoding the distinction as `Operation.mode` on
`model.deref` would instead require a new catalogued mode kind and a wider
`OperationMode` schema. The `field_owner` operand group
admits the `object` operand, and `member_of` then holds exactly as stated
above. A `Reference<O>` operand has family `reference`, which `field_owner`
does not admit, so a projection over a reference that is not dereferenced
refuses `ill_typed`/`operator-ineligible`.

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

`quire.op.reference.upcast` carries the `conforming_upcast` constraint over
operand 0, of type `Reference<S>`, and its `type_argument` member, which
names `Reference<T>`. At the application's `operator-ineligible` check, the
reader looks up the declarations of `S`'s and `T`'s model declaration nodes
by step 2, with step 2's refusal, including when both name one type node;
the operand's declaration is the source and the member's is the target. The
constraint holds exactly when both are object type declarations, with or
without `interfaceFeatures`, of one selected document and the source
conforms to the target under FR-151's conformance relation as above: they
are the same effective type, or a chain of declared supertypes leads from
the source to the target. Unlike `conforming_reference`, the direction
matters: an upcast whose target conforms to its source and not the reverse
is a downcast, which no catalog operation admits (FR-149). A downcast, two
object types with no supertype chain from the source to the target, a
systems interface with no declared supertype against an object type, a
relationship declaration at either end
refuse `ill_typed`/`operator-ineligible`, so a reader admits an upcast only
between the pair FR-149's `Reference<S>` to `Reference<T>` row admits.

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
  `declaration` naming a node that `A` does not conform to refuses. The
  `read_form` member is reserved for a model-object projection over a
  `quire.op.model.deref` result. An ordinary record projection SHALL omit
  `read_form`; a record projection that carries it refuses `malformed_wire`.
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
application without it keeps the member and the path it had before the key
existed; the key is part of the application's structural path when present. No other member kind carries `population`, and neither does
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

### Destination type of a relationship end

The `relationship_end` member of `quire.op.model.reaches` and
`quire.op.model.navigate` carries one further required key, `destination_type`,
the `NodeId` `{package, path, ordinal}` of the destination end's type node
(the semantic IR's `relationship_end`, FR-038, names the destination type). The
producer supplies it; the reader recomputes no node id, never derives the value
from the relationship or the selected document, and returns it verbatim. The
reader checks the shape of the value only, a structural `NodeId`, and does not
resolve it: the destination type node MAY be absent from the package's graph
(for example a type of a package the graph does not carry), and the id is not a
`dependencies` entry of the application, because a `dependencies` entry names a
node the graph carries. A `destination_type` that is missing, is not a
structural `NodeId` or carries a member beyond `package`, `path` and `ordinal`
fails the V2 schema (`malformed_wire`).

### Population references

A state clause names a population its domain package declares by the
qualified name `M::P` and reads it only as the population operand of
`allInstances<T>(M::P)` or `lookup<T>(M::P, r)` (QSL FR-391,
[FR-153](../../functional/type-model/FR-153-query-closed-model-environments.md)).
The clause has no binder to reference, and the `Population<O>[N]` type node of
"Derived type forms and forwarding" is identified by its member type and bound
alone, so with populations `counters` and `archive` of `Counter` the two reads
would check to one node. A **population reference** is the node that carries
the declared population itself.

- **Node.** `node_tag: "relation"`, `semantic_form: "population"`. It is a
  model declaration node ("Model-owned members" step 2): no `declaration`,
  or `recursion_group`; `semantic_type` its own
  `node_id`; the body `{term: "aggregate", members: []}`; empty
  `dependencies`; and an `owner`, a `ModelOwner` `{kind: "model", identity,
  node}` whose `identity` is the identity of the domain
  package that declares the population and whose `node` is the population's IR
  node identity. A `relation`/`population` node with no `owner` fails the V2
  schema (`malformed_wire`).
- **Declaration key.** The node's `owner` is the population's declaration key
  (FR-154): `identity` (package) then `node`, each as UTF-8 bytes. The node holds nothing else of the declaration: no member
  types, no extent, no maximum and no observation.
- **Identity.** The node's `node_id` is `{package, path, ordinal}` with
  `package` the owner's `identity` and `path` the population's declaration path.
  It is a function of the declaration key alone. It does not depend on the
  member type `U`, on a maximum, on the clause or on the operation that reads it.
  The populations `ix://acme/runs/counters` and `ix://acme/runs/archive` of
  package `acme/runs` have two distinct `node_id`s; every read of `counters` in
  a package, by an `allInstances` or a `lookup` in any clause, references its one
  node.
- **Owner join.** The owner join admits the node
  when its `identity` equals that of a `model_root` node and that package's admitted
  document declares `owner.node` as a population
  (`quire.meaning.model.population/v1`,
  [FR-208](../foundation/FR-208-quire-meaning-vocabulary.md)); otherwise it
  refuses `missing_declaration`/`missing-selection` at the node's `node_id`.
  The document the join reads is the one step 1 of "Model-owned members"
  admits, whose `populations` member lists one FCD semantic IR 2.0.0
  population per declared population: `identity`, `displayName`, `kind`,
  `members`, `extent`, `origin` and, optionally, `symmetric` (filament-core-data
  FR-141 and its semantic-ir population schema). A population's `identity` is
  its IR node identity, as FR-154 takes a declaration's IR node identity from
  FCD (FR-095).
  The owner join SHALL compare `owner.node` with the `identity` of each
  `populations` entry of the selected document. The reader SHALL build the
  population's node from that `identity`, and the node's `node_id` SHALL be the one
  "Identity" above derives from the declaration key. A selected document whose
  `populations` entry lacks an `identity` string, or carries a `node` member,
  SHALL refuse at step 1 as `invalid_model_binding`/`malformed-declaration` at
  stage `package_read` (FR-154), reported at the owner of the `model_root` node
  (`/semantic_graph/nodes/<i>/owner`) of the package whose document is read, before
  the owner join runs. The join reads no
  other population member (FR-154 and FCD own where the IR document declares a
  population and what else it carries). That covers a population owned by a node the document declares as another
  kind or does not declare, and a node of another model form owned by a
  population. A population declares no field, operation or relationship end,
  so a `field`, `operation` or `relationship_end` member whose declaring node
  is a population reference refuses `ill_typed`/`operator-ineligible`.
- **Use.** The node has family `population`. A `reference` term to it is
  operand 0 of `quire.op.model.all_instances` and `quire.op.model.lookup`,
  the positions the operation catalog gives the family, and the application
  lists the node in its `dependencies` as for any reference target. The
  application is otherwise as for a parameter operand: the `type_argument`
  member names the queried `T`, and the `result_type` is `Set<Reference<T>>`,
  or the `collection_bounds` `[0, N]` node over it when the unit authors a
  maximum `N` for the population (QSL FR-380). `N` belongs to the unit and not
  to the declaration, so it enters the package through the `result_type` and
  never through the population reference; two units that bound one population
  differently share its one reference node. `U`, the population's canonical
  member type (FR-153), is read from the selected document and is not carried.
- **Quantifier domain.** A population is a binding and not a collection. The
  `collection_kind` operand of `quire.op.collection.forall`, `exists`,
  `count`, `filter` and every other operation, and every other operand
  position, refuses a population-family term `ill_typed`/`operator-ineligible`.
  The domain of a clause `forall(o in allInstances<T>(M::P) : c)` is
  therefore three nodes: the population reference, the `all_instances`
  application over it, and the quantifier application whose operand 0
  references that application's node; the domain of the quantifier is the set
  the application returns.
- **Relation to `Population<O>[N]`.** The anonymous type form of
  FR-322-AC-62 types a population *binding*, a `value`/`parameter`
  node that a binder introduces and a caller supplies, and it names no
  declaration; it is also the node the `population` link of a
  `quire.op.model.reaches` application names, and a population reference
  there refuses `ill_typed`/`operator-ineligible` at `member.population`. A
  population reference is the operand when a clause names a declared
  population. Both have family `population` and fit the same operand
  position; the reader tells them by the referenced node, a
  `relation`/`population` node or a `value`/`parameter` node typed by a
  `bounded_domain`/`model_population`. The two forms are distinct: the
  `Population<O>[N]` form types a binding and names no declaration, and a
  declared population is a population reference.
- **Abstraction relations.** A `populations` entry of an
  `abstraction_relation` body
  ([FR-451](FR-451-checked-package-abstraction-relation-body.md)) targets this
  node, so the entry names the declaration key.

### Refinement row expressions

A `relation`/`refinement` body ([FR-375](../../functional/analysis/FR-375-declare-model-refinement.md))
holds each row expression (a field row, a step receiver or argument, a history
update) as a `SemanticTerm`: a `literal`, or a `reference` to a node whose body
root is an application, every composite subterm being its own node ("Body
grammar"). A **row node** of a refinement is a node that a chain of `reference`
terms, through application arguments and binding values, reaches from one of
that refinement's row expressions. The three forms below are application nodes
(`node_tag: "expression"`) they add no
member to the refinement body and no node form, and the
refinement node's `dependencies` already list their row expression targets. The
forms below are read over the concrete model
`acme/counter` (object type `Counter`, population `counters`, and the history
row `history Counter.seen: Int[0, 9] = 0`).

- **History read.** A history field is declared by one entry of the body's
  `histories` (`concrete_type`, `name`, `type`). It is no member of the concrete
  type: the type's declaration node keeps the body `aggregate{[]}` and declares
  no field of that name ([FR-375](../../functional/analysis/FR-375-declare-model-refinement.md)
  "History rows"), and no `field` member names it, so FR-378's "one field to
  every object" is a field of the refinement's analysis state and not of the
  concrete declaration. A read of it is the application with `operator` `query`
  (`semantic_form` `query`), identity `quire.op.refinement.history`,
  `laws` `[]`, `mode` `null`, `leaves` `[]`, and `member`
  `{kind: "history", declaration, name}`, where `declaration` is the model
  declaration node of the history's `concrete_type` and `name` its name.
  `arguments` is `[ref(r)]` for one term `r` of family `reference`, the
  receiver (the `self` parameter node of the row, or any other `Reference<T>`
  term), and `result_type` is the node of the history's declared `type`. The
  pre-state value of the field, `pre(seen)` of an update, is
  `quire.op.state.pre` (`pre_read`) over the read, as for any value. Its structural
  path is the application's alone: the operation identity, the member's
  declaration key and name, the receiver and the result type enter it, and
  neither the refinement node nor the history's initial value or updates
  does, because the refinement's body reaches this node and a path that held it
  would be circular. Two refinements that declare the same history of the same
  type therefore refer to one node id, and the graph holds that node once.
- **Population value.** A concrete population `C::p` that a row names is read
  as `allInstances<T>(C::p)`, the node `quire.op.model.all_instances` over the
  `relation`/`population` node of "Population references", whose owner is the
  population of the selected concrete model package; it is not a
  population-family term in any other position (a population is a binding, not
  a collection, "Quantifier domain"). `member` is `type_argument{declaration: T's
  model node}`. In a refinement row the `result_type` SHALL be the
  `collection_bounds` `[0, n]` node over `Set<Reference<T>>`, `n` the maximum the
  unit authors for the population (QSL FR-380): FR-376 types the form
  `Set<Reference<T>>[0, n]` and derives the bounds of `count` and `sum` from `n`,
  so the unbounded `Set<Reference<T>>` of "Population references" is not a
  population value of a row and refuses `ill_typed`/`operator-ineligible` at the
  application. The other forms of
  [FR-376](../../functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md)
  lower over that node, each binder `x` being a `value`/`parameter` node of type
  `Reference<T>` and an omitted `where` the `boolean` literal `true`:

  | FR-376 form | Nodes |
  | --- | --- |
  | `count(x in P where c)` | `quire.op.collection.count` over the population value, binder body `c`, `type_argument` and `result_type` the `Int[0, n]` node |
  | `exists(x in P where c)` | `quire.op.collection.exists` over the population value, binder body `c` |
  | `all(x in P where c)` | `quire.op.collection.forall` over the population value, binder body `c` |
  | `sum(x in P where c: e)` | `quire.op.collection.sum.<family>` over `F`, binder body `e`, `type_argument` and `result_type` the bounded type FR-376 derives |
  | `only(x in P where c: e)` | `quire.op.collection.only` over `F`, binder body `e`, `type_argument` and `result_type` the type node of `e` |

  where `F` is the population value when `where` is omitted and otherwise the
  `quire.op.collection.filter` application over it with binder body `c`.
  `quire.op.collection.only` takes one `collection_kind` operand and one binder,
  has a `result_type` equal to its `type_argument` member and to the bound of its
  binder (the `bound_is_member` constraint), and is defined at a collection of
  exactly one element; for any other collection the value is undefined with the
  reason `only-not-one` (FR-376). It does not lower through `map`, because
  `map` over a set collapses equal values and would count two objects with one
  value as one. A member other than the bound's type refuses
  `ill_typed`/`operator-ineligible`.
- **seq.** `seq(i in lo .. hi: e)` is the application with `operator` `collection`
  (`semantic_form` `collection`), identity `quire.op.collection.seq`, `member`
  `type_argument{declaration: E}` for `E` the type node of `e`, and `arguments`
  `[ref(lo), ref(hi), binding{name: i, value: ref(e)}]`, where `lo` and `hi`
  are terms of family `integer` whose type nodes are `integer_range` nodes with
  bounds `[lo_min, lo_max]` and `[hi_min, hi_max]`, and the binder `i` is a
  `value`/`parameter` node of type the `integer_range` node `[lo_min, hi_max]`
  (FR-376). The `result_type` is the `collection_bounds` `[0, m]` node over
  `Sequence<E>` with `m = max(0, hi_max - lo_min + 1)`. A bound whose type is not
  an `integer_range` node (an unbounded `Integer` has no `lo_min` or `hi_max`),
  or a `result_type` that is another node, refuses `ill_typed`/`operator-ineligible`
  at the application. For `seq(i in 0 .. 3: i)`, `m` is `4`.
- **Operation catalog.** The forms need three entries of
  `quire.checked-operation-catalog/v1` and one member kind, published exactly as
  follows:
  `quire.op.refinement.history` (operator `query`, one `reference` operand,
  result `member`, member kind `history`), `quire.op.collection.only` (operator
  `collection`, operands `collection_kind` and `binder`, result `member`, member
  `type_argument`, constraint `bound_is_member` on operand 1) and
  `quire.op.collection.seq` (operator `collection`, operands `integer`,
  `integer` and `binder`, result `sequence`, member `type_argument`, constraint
  `bound_is_member` on operand 2); the `history` member is `{kind: "history",
  declaration, name}` beside the `field` member of the closed `OperationMember`
  schema. The catalog, both V2 schemas' `OperationIdentity` and `OperationMember`
  enumerate them together or not at all
  (FR-322-AC-17), so a reader that does not yet hold the entries refuses these
  identities `unknown-operation` and no earlier.
- **Refusals.** In the refinement step's row check
  ([FR-375](../../functional/analysis/FR-375-declare-model-refinement.md)
  "Reader order" step 5), a row node holding a `quire.op.refinement.history`
  application whose member (`declaration`, `name`) equals no entry of this
  refinement's `histories` (`concrete_type`, `name`) refuses
  `missing_declaration`/`missing-name` at the application's
  `/semantic_graph/nodes/{n}/body/operation/member/name`, and one whose
  `result_type` is not that entry's `type` refuses `ill_typed`/`type-mismatch` at
  `/semantic_graph/nodes/{n}/body/result_type`; both precede every operation
  refusal. In the operation stage a history read whose receiver is not a
  `reference` to the member's declaring type (or a type that conforms to it,
  "Reference conformance"), or that no refinement reaches as a row node,
  refuses `ill_typed`/`operator-ineligible` at the application. The
  operation-stage refusals of `quire.op.collection.only`, `quire.op.collection.seq`
  and the population value are at the application node
  too, `/semantic_graph/nodes/{n}`, even when the operand they examine is the
  `result_type` or the member, which a generic result-type check would locate at
  `/body/result_type`.
- **Relation to QSL FR-093 and FR-376.** FR-093's lowering table already has
  rows for `count`, `sum` and the population value (`AllInstances`), so those
  need no new row; it lacks exactly three: the history read, `only` and `seq`
  (an addition). The one new requirement on a row it already has is that the
  `result_type` of a population value in a refinement row is bounded `[0, n]`.
  Three statements conflict with the forms and are ruled here: FR-376's type `Set<Reference<T>>[0, n]`
  with `n` the universe size against "Population references"' optional authored
  bound (ruled: required in a row, and `n` is the authored bound, so the universe
  of FR-377 is the authored maximum); "Quantifier domain"'s population that is a
  binding and not a collection against FR-376's population name that denotes a set
  (ruled: the name is the `all_instances` application, never a population term in
  an operand position); and FR-378's history field "of every object" against a
  model declaration that holds no field of that name (ruled: a history is
  declared by the refinement alone, and read only through the history read).

## Structural leaf walk

The leaf walk of a compared type starts at that type with the empty path
and visits types in declaration pre-order:

- A text type is a **leaf**: it lists one entry at the current path.
- A record, a tuple and each member of a union are **anchors**; a
  `model`/`record_value_type`, `model`/`tuple_value_type` and a
  `model`/`variant_type` of family `union` are walked as the record, tuple and
  union they stand for, from the selected document ("Tuple value types and
  payload-bearing variant types"). A record
  visits each field's type in field declaration order under `field:<name>`.
  A tuple visits each position's type under `position:<i>`. A union visits
  its members in member declaration order, and each member's ordered
  payload types under the two segments `member:<Name>` then `position:<i>`,
  where `<Name>` is the member identifier and `i` is the index in that
  member's payload, even for a member with a single payload position. A
  member with no payload contributes no leaf.
- An `option`, `sequence`, `set`, `bag` or `ordered_set` visits its element
  type under `inner`.
- A float type (`float32`, `float64`) is a **leaf**: it lists one entry at the
  current path, with the law `ieee_profile` (the selected IEEE definition) and
  mode `null`. The walk lists a float leaf for every leaf source, so a set,
  bag or ordered set result whose element type holds a float lists it as well.
- Every other type contributes no leaf.

The walk SHALL visit every declared union member independently of the
active member of any operand value. `member:<identifier>` contains the
exact declared identifier from FR-440's union binding, followed by a
separate `position:<n>` segment whose index starts at zero and uses
canonical decimal spelling. No member sort, name normalization, union
node id, member ordinal or active-value tag enters the path. The path
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
complete ordered leaves independently of node identity. The
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
`unit` is `quantity`, an enum is `ordered_enum` when its body's `ordered` is
true and `enum` otherwise; `composite_type` forms `option`, `record`,
`tuple`, `union`, `sequence`, `set`, `bag`, `ordered_set` and `reference` are
their own family; a `model`/`record_value_type` or `model`/`event_type`
declaration is `record`, a `model`/`tuple_value_type` declaration is `tuple`, a
`model`/`variant_type` declaration is `enum` (never `ordered_enum`) when none of
its cases carries a `payloadType` and `union` when one does, and
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
type's family, except that a function reference is `function`, a reference to a `relation`/`population` node is `population`,
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
| `ieee_float` | `float32`, `float64` | IEEE operands; `negate` per width, and no `eq`, `ne` or ordering identity. |
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
`text`, `string`, `uuid`, `timestamp`, `bytes`, `json_object`, `enum`→`enum` or `ordered_enum`, `none`→`option`, `quantity`→`quantity`, `reference`→`reference`). Any failure is
`operator-ineligible`.

The Option literals `none` and `absent` (FR-143 "The `absent` Option literal") lower
to one `literal` term with `value_kind` `none` whose `type` is a node of family
`option`, the `composite_type`/`option` node of the `Option<T>` the position
expects; the spelling is not carried. No type node stands for a clock, so for an
`Option<Clock>` place no such literal exists in the package: `self.v = absent` over
`v: Option<Clock>` has no package form: it is the excluded drop ("Forms the package
excludes", `CW-03`), refused by the producer and carried by no node.

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
`invalid_package`/`operation-mode-type-mismatch`. `string` also has the six
text-to-number conversions `quire.op.string.to_integer`, `to_decimal`,
`to_rational`, `to_float32`, `to_float64` and `to_quantity`, catalogued as
FR-478 states beside their `quire.op.text.to_*` counterparts; a
text-to-number conversion is a `convert`, so its `rounding` mode, when its
target has one, is pinned by the result type alone. These families have no
ordering, no arithmetic and no other operation identity: an ordering or
arithmetic identity over them refuses `ill_typed`/`operator-ineligible`
(FR-149-AC-16 for UUID and Timestamp).

The native scalar families `bytes` and `json_object` have the identities
`quire.op.bytes.eq`, `quire.op.bytes.ne`, `quire.op.json_object.eq` and
`quire.op.json_object.ne`, each with two operands of its own family, result
`boolean`, no law, no mode, no member and no leaves, deciding equality by the
canonical payload, so equality is reflexive, symmetric and transitive and `ne`
is its negation. The family `bytes` also has `quire.op.bytes.length`, one
`bytes` operand and result `integer`, the number of octets of the operand, and
`quire.op.bytes.concat`, two `bytes` operands and result `bytes`, the octets of
operand 0 followed by those of operand 1, so that
`length(concat(a, b)) = length(a) + length(b)` and `concat` is associative with
the empty sequence as identity. Both V2 schemas enumerate these six identities in
`OperationIdentity`, so a package that applies them validates. `json_object` has no operation identity beyond
`eq` and `ne`, and `bytes` none beyond these four: an ordering, arithmetic,
member-access or any other identity over either family refuses
`ill_typed`/`operator-ineligible`. A `bytes` operand of a `quire.op.string.*` or
`quire.op.text.*` identity, and a `string` or `text` operand of a
`quire.op.bytes.*` identity, refuses the same, because there is no conversion
between the families. `Duration` introduces no family and no identity: its
operations are the quantity identities of FR-142 over `Quantity<s>`.

**Equality-stable types.** A type is *equality-stable* when an equality
application is admitted over two operands of it: a type of family `boolean`,
`integer`, `rational`, `decimal`, `text`, `string`, `uuid`, `timestamp`,
`bytes`, `json_object`, `enum`, `ordered_enum`, `quantity` or `reference` (each through its own
`eq` identity), a type of family `float32` or `float64` (through
`quire.op.ieee.numeric_equal`, with `!=` its `quire.op.boolean.not`), and a
type of a `structural_kind` family through `quire.op.structural.eq` when no
leaf at any depth is a `model`/`event_type` node. IEEE values have no `eq`
or `ne` identity (`quire.op.ieee.numeric_equal`, `total_order` and
`bit_identical` are three distinct identities), and a `function`, `population`, `object` or
`aggregate` term is not a value type. A requirement that a key, a comparison
or a map position needs a type "over which `=` is admitted" (the FIFO key of
[FR-465](FR-465-checked-package-protocol-declaration-bodies.md)) means
exactly an equality-stable type.

**Scalar literal values.** The `value` of a `string`, `uuid`, `timestamp` or
`quantity` literal has one spelling, the canonical
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
`value`. A `bytes` literal's `value` is a JSON string of lowercase ASCII
hexadecimal digits, two per octet in sequence order, the empty string for the
empty sequence (`""`, `00ff`; not `00FF`, `0`, `0x00` or `00 ff`). A
`json_object` literal's `value` is a JSON string holding the RFC 8785 (JCS)
serialization of one JSON value whose text is already canonical: the string
parses as a JSON value with no repeated member name at any depth, and
serializing the parsed value under JCS (members sorted by UTF-16 code unit
order of their names, no insignificant whitespace, strings and numbers as
ECMAScript serializes them) gives the string again, so a number is written as
the shortest ECMAScript spelling of its IEEE binary64 value (`{"a":1,"b":[true,null]}`;
not `{"b":[true,null],"a":1}`, `{"a": 1}`, `{"a":1.0}`, `{"a":1e2}`, a number
that is not the shortest spelling of the binary64 it rounds to such as
`9007199254740993`, text that is not one JSON value, or a repeated name; `[]`, `3`,
`"x"` and `null` are admitted).
A `bytes` or `json_object` `value` in any other spelling refuses
`invalid_package`/`invalid-value` at the literal's `value`. A `Duration` literal is a
`quantity` literal over the `scalar_type`/`unit` node of `s`, its `value` the exact
rational number of seconds, spelled as any quantity magnitude is. A `quantity` literal's `type` is the `scalar_type`/`unit` node of its
unit (family `quantity`) and its `value` is the exact rational magnitude: the
canonical decimal integer when the denominator is 1, otherwise
`numerator/denominator` in lowest terms with a denominator above 1 and a sign
only on the numerator (`3`, `-5`, `1/1000`; not `03`, `+3`, `2/4`, `3/1`, `2.5`). A `rational` literal's `type` is a `scalar_type`/`rational` node and
its `value` is its exact value in the same canonical magnitude spelling
(`0`, `1`, `-3`, `1/2`), so one rational has one spelling: `2/2`, `1/1`,
`02`, `+1`, `2/4` and `1/0` refuse `invalid_package`/`invalid-value` at the
literal's `value`. The spelling is read after the literal's family check, so a literal
whose `type` is not its kind's family refuses `ill_typed`/`operator-ineligible`
even when its value is also non-canonical.

A `decimal` literal's `type` is a `scalar_type`/`decimal` node or a
`bounded_domain`/`decimal_range` node (family `decimal`) and its `value` is the
decimal string of its coefficient `c` and scale `s` as written, keeping the
scale: the digits of `|c|` with the decimal point placed `s` digits from the
right, padded on the left with zeros so that at least one digit precedes the
point, no point when `s` is `0`, and `-` only before a nonzero `c`
(`1.25` for `(125, 2)`, `0.05`, `10`, `0.00`, `-0.5`; not `.5`, `1.`, `+1.5`,
`01.5`, `-0.0`, `1e2` or `1/4`). Two literals of one type with one magnitude
and different scales (`1.0` and `1.00`) are two values and two nodes. For a
`decimal_range` type, membership of the literal is decided by its value under
FR-140, not by the written coefficient or scale: the value is normalized, its
membership scale is `s* = max(s, smin)`, and it is a member exactly when
`s* <= smax` and its membership coefficient `v x 10^s*` lies in `[lo, hi]`.
`5` is therefore a member of `Decimal[-1000, 1000; 1, 2]` (`s* = 1`, `c* = 50`),
and `1000.00` is a member of `Decimal[0, 1000; 0, 2]` (normalized `1000`,
`s* = 0`); the written spelling stays as written and is the literal's
identity. A `value` that is not a member, like a `value` in any other spelling,
refuses `invalid_package`/`invalid-value` at the literal's `value`, after the
family check.

A `float32_bits` literal's `value` is the IEEE 754 binary32 bit pattern of its
value and a `float64_bits` literal's `value` the binary64 bit pattern, each as a
JSON string of lowercase ASCII hexadecimal digits with no `0x` prefix and no
separators: 8 digits for `float32_bits` and 16 for `float64_bits`, the most
significant octet first and leading zeros kept (`3f000000`, `00000000`;
`3fe0000000000000`, `8000000000000000`). Every such string is a bit pattern, so
the infinities, signed zeros and every NaN payload are distinct values with
distinct spellings. A `value` in any other spelling (an uppercase digit, a `0x`
prefix, 7 or 9 digits for `float32_bits`, 15 or 17 for `float64_bits`, a
decimal numeral such as `0.5`, or a value that is not a JSON string) refuses
`invalid_package`/`invalid-value` at the literal's `value`, after the family
check as for every scalar literal.

**Reference literal values.** A `reference` literal is a reference literal
([FR-480](../../functional/type-model/FR-480-check-and-resolve-reference-literals.md)).
Its `type` is the `composite_type`/`reference` node over a `model`/`object_type`
declaration node, and its `value` is the object identity as lowercase
hexadecimal, two digits per byte, at least one byte. A `value` with an
uppercase digit, an odd number of digits, a non-hexadecimal character or no
digit refuses `invalid_package`/`invalid-value` at the literal's `value`. A
`reference` literal typed by a reference node over any other declaration kind
refuses `ill_typed`/`operator-ineligible`. The literal carries no universe and
no object type component: a subject binding fixes both (FR-480).

| Constraint | Decision rule |
| --- | --- |
| `same_family` | The two named operands have equal families. |
| `same_type` | The two named operands resolve to the same type node. |
| `conforming_reference` | The two named `reference` operands conform: they resolve to the same type node, or their object types conform one to the other as "Reference conformance" below decides. |
| `conforming_upcast` | The named `reference` operand and the `type_argument` member resolve to the same type node, or the operand's object type conforms to the member's object type, in that direction, as "Reference conformance" below decides. |
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
| `scale_reduction` | FR-149 classifies it as scale reduction to a decimal; among exact-numeric conversions this is the only one that rounds, and it requires a `rounding` mode. An FR-478 `quire.op.text.to_decimal` or `quire.op.string.to_decimal` conversion also rounds, under its target's non-`exact` mode. |
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

`quire.op.quantity.negate` is the unary quantity negation of FR-142 "Quantity
negation": one `quantity` operand, result `operand:0` (the operand's own unit
type), and no law, mode, member, constraint or leaves. It takes no `rounding`
mode, because negation is exact, and its operand is a non-affine unit's
quantity.

## Identity and validation

A checked package is identified by its `name` and its contents compared
structurally (`name` above). Runtime inputs, backend availability, filesystem
paths, display ordering and diagnostic rendering are not part of the package. A
strict reader first validates the `contract_version` document-type tag, closed
wire shape and canonical bytes, then admits the graph. A domain package's
identity names the document the reader reads and is recorded by its `model_root`
node. A model carries no digest and the reader makes no content check of that
document ("Model-owned members" step 1). A document whose `contract_version` is not
`quire.checked-package/v2` is not a checked package. Such a document, missing or duplicate
members, out-of-domain values, cross-domain substitutions,
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

**Integers.** An integer *value*, meaning a literal's `value` of `value_kind`
`integer` and so every range or cardinality bound, is an `IntegerString`, the
canonical decimal string, at every magnitude. A *counter*, meaning an
`operation.member` `position`, is a JSON number when its value is at most
2^53-1 (9007199254740991), and its canonical decimal string otherwise
(QSL ADR-013 O-09). A `node_id.ordinal` is always a JSON number and has the one
bound 2^53-1 ("Structural node identity").

### Temporal clock bindings

`temporal_bindings` carries the typed clock selection for each temporal
declaration whose selected profile requires a clock configuration. It is a
top-level checked-package member, not a semantic-graph node. When no
`temporal`/`temporal_clause` node selects one of the clock-configured profiles,
the member is absent; when present, it is a non-empty array. Absence never
supplies a default clock or configuration for a temporal declaration that
requires one.

Each row is exactly:

```json
{
  "declaration": {"package": "<package>", "path": ["<segment>"], "ordinal": 0},
  "clock": {"kind": "event_position", "sequence_authority": "<name>"},
  "clock_name": "<name>"
}
```

`declaration` is a structural `NodeRef` equal to the `node_id` of exactly one
`temporal`/`temporal_clause` declaration node (FR-370), whose occurrence has
role `claim`. It is not the referenced `temporal`/`formula` expression node,
the model root, or a `timed_model` node: those identities do not identify the
temporal declaration's selected clock. The row is read by that exact node id;
a reader never selects it by array position, clock name, formula contents, or
path-name inference. Rows are in ascending structural `NodeRef` order and each
declaration occurs at most once.

`clock_name` is the exact non-empty text value of the clause body's `clock`
argument (the second argument of `quire.op.temporal.clause`, FR-370). A reader
uses this typed member directly; it never parses a legacy `clock:` prefix or
derives the name from the profile identity.

`clock` is exactly one closed alternative, with no omitted or extra members:

| `kind` | Remaining members | Selected `temporal_profile` definition |
| --- | --- | --- |
| `event_position` | `sequence_authority`: non-empty name | `quire.temporal.event-position.false-extension/v1` |
| `fixed_sample` | `epoch` and `period`: exact typed numbers; `period` is positive; `unit`: non-empty name | `quire.temporal.fixed-sample.false-extension/v1` |
| `timestamped_event` | `timestamp_unit`: non-empty name | `quire.temporal.timestamped-event.finite-window/v1` |

`epoch` and `period` use the closed tagged exact-number form `{kind:
"integer", decimal}` or `{kind: "rational", numerator, denominator}`;
integer components use canonical decimal strings and rational denominators are
positive. The rational components are in lowest terms. `period` is a strictly
positive exact number. The authority and unit values are non-empty strings
compared byte-for-byte; no trimming, normalization or alias resolution occurs.

The profile is not repeated in a row. The reader resolves `declaration`, reads
the clause's `temporal_profile` law, and requires that law to equal the matching
`lock.profile_selections` entry. The selected definition determines exactly the
`clock.kind` above. A row for an absent or wrong-kind declaration, a duplicate
row, a missing row for a clock-configured temporal declaration, a mismatching
clock name, or a kind inconsistent with its selected profile is refused; no
clock, parameter or profile is substituted. The reader exposes the admitted
row by reference as typed clock configuration data, without reparsing the
wire or rechecking the profile selection during later reads. After aggregate
order validation and before per-node joins, the reader requires exactly one row
for every clock-configured temporal declaration. A missing row refuses at the
first such declaration's own `/semantic_graph/nodes/{N}/body`, where `N` is its
graph index. A row with an absent or wrong-kind declaration refuses at that
row's `/temporal_bindings/{i}/declaration`; a repeated declaration refuses at
the second row `/temporal_bindings/{i}`. Each of these semantic refusals is
`invalid_package`/`timed-node-reference`. After aggregate order validation and
declaration `NodeRef` resolution, the reader checks rows in ascending structural
`NodeRef` order. For each row it compares `clock_name` before checking the
configuration kind against the selected profile: a name mismatch refuses
`invalid_package`/`timed-node-reference` at
`/temporal_bindings/{i}/clock_name`; a profile/configuration kind mismatch
refuses with the same code and cause at `/temporal_bindings/{i}/clock/kind`.

`temporal-clock-binding-vectors.json` publishes one full checked-package
positive for each configuration kind and the schema-invalid unknown-kind,
missing-member and wrong-type cases. Its schema-valid semantic refusals cover a
missing required row, absent and wrong-kind declaration references, and a
duplicate row. Further semantic refusals cover a clock-name mismatch and a
profile/configuration kind mismatch, with their exact row loci and check order.
The multi-clause positive proves distinct declaration `NodeRef`s and ascending
row order. The selected profile, configuration kind, and clause clock name
agree in every single- and multi-clause positive. The vector file is the
machine-readable form consumed by the QSpec qualification test.

### Timed-model nodes

CheckedPackage V2 carries admitted timed models in `semantic_graph.nodes`.
The node tag is `timed_model`; its closed structural forms are `timed_model`,
`time_source`, `clock_place`, `clock_reset`, `clock_guard`, `time_invariant`,
`urgency` and `clock_constraint`. The `timed_model` root is owned by the
declared state model and references, in canonical order, its one source and
all places and timed clauses. It also carries the exact nominal dimension node
that authorizes time. A source carries dense/tick kind, that dimension's
resolved unit node, and for tick the declaring model, operation anchor and
exact positive rational period. A `timed_model` body has exactly the ordered
bindings `model`, `time_dimension`, `source`, `clock_places`, `resets`,
`guards`, `invariants`, `urgencies`, `constraints`;
the five clause bindings are ordered reference aggregates. A `time_source`
body is exactly `kind`, `unit` for `dense`, and exactly `kind`, `unit`,
`model`, `operation`, `period` for `tick`; the operation reference is to a
`state`/`operation_anchor`, and that anchor's `context` is the tick model. The
source model equals the root model. A `clock_place` body is exactly `model`,
`path`, `resolved_path`: it has no `guards` field, the guards that constrain a
place being the `clock_guard` nodes of the root and the predicate expressions of
their parts, which a place cannot list without a cycle in a graph whose nodes
depend on their dependencies; `resolved_path` is an ordered table with
one row for each path step, carrying the declaring object/record type,
resolved member identity, declared member type, collection kind and bounds
where applicable, and the terminal `Clock` marker. It is checked package data,
not information recovered by reopening the selected model document. The
reader joins every row to its path step and to the root model's selected
declaration; absence, disagreement or an undeclared member is a malformed
clock place. Its nonempty path has ordered `field(name)`,
`tuple(index)`, `union(case)` or `slot(collection, selector, minimum,
maximum)` entries.

A `clock_place` is a **template**: one node for each distinct clock path of
the model, however many elements a collection holds in a state, and its identity is
that of its complete body. The `selector` of a `slot` step is therefore the
**symbolic element selector**, a `reference` to the type node of the selector's
domain: the package's `Integer` type node for a `Sequence`, `Set`, `Bag` or
`OrderedSet` (the element's zero-based position, FR-415 "Element places"), and
the node of the key type for a `Map`. Only a scalar key type is ruled here: a tuple or
record key is out of scope until a vector exercises it, and a reader that cannot resolve it
refuses the place `malformed-clock-place`.
A `Map` that is the element of a `Sequence`, `Set`, `Bag` or `OrderedSet` adds
a second slot step, so a path may carry several. A slot step whose selector is
an integer literal, a reference to any other node, or a reference to the type
node of another domain refuses `invalid_package`/`malformed-clock-place` at that
path step; the integer literal arm of the `selector` binding stays in both
closed schemas for wire compatibility and is not admitted in a `clock_place`.
The concrete position or key exists only on a place instantiated at analysis or
run time, which no node represents: every instance of a template shares its
node, and no node field depends on how many elements a state holds. Identity is the
complete body and a place has no field but `model`, `path` and `resolved_path`,
and the rows of `resolved_path` are a function of the path and the selected
declaration (FR-322-AC-124), so two `clock_place` nodes of one model and path
whose rows all join are one body and one node. A second node of a path
therefore exists only with a row that differs from the declared one, a forged
row. The timed-model step joins the rows of every `clock_place` node to its
path and to the selected declaration, in ascending node-id order,
after the aggregate order check (FR-322-AC-260) and before the repeated-path
determination (`duplicate-clock-place`), so the second node is refused
`invalid_package`/`malformed-clock-place` at its first differing row
`/semantic_graph/nodes/{i}/body/members/2/value/members/{j}` (`TM-12`,
`GD-02`) and is never admitted as a second place. Precedence: the order check
comes first, so a twin whose order key equals its predecessor's (`AO-10`) is
`invalid_semantic_graph` by order and never reaches its row. This reverses the
earlier rule (STD-559, QSpec #496) that a second place node of a path is
`invalid_package`/`duplicate-clock-place` before any row is joined: that rule
cannot hold, because no valid second node of a path exists. `duplicate-clock-place` is
kept for a repeated path whose every row joins, which a conforming package
cannot contain: it would repeat the body of the earlier node, and one
node listed twice repeats its order key and is the FR-322-AC-260 order refusal
(`TM-30`). The `guards` aggregate of the `timed_model` root may be empty; it
is empty exactly when the model has no guard to list. A guard that exists is
never absent from its aggregate: a `clock_guard` node that the root's `guards`
does not list refuses `invalid_package`/`timed-node-reference`, and a producer
that cannot write the guards of a model refuses to emit that model rather
than write an empty aggregate.

The refusal locus of an unlisted guard is the unlisted node itself: the
`clock_guard` node's `body`, `/semantic_graph/nodes/{i}/body` with `i` the
graph index of the first unlisted `clock_guard` node in ascending node-id
order. Like a missing timed root, a node the root omits has no reference to
point at, and the node is the defect. `GD-03` of the slot vectors carries the
`locus` and the `locus_suffix` `body`.

**Aggregate order.** Each reference aggregate of a timed root (`clock_places`,
`resets`, `guards`, `invariants`, `urgencies`, `constraints`) lists its members in **declared order**: strictly
ascending by the member node's **order key**: the minimum of the triples
(`source.identity`, `start`, `end`) over every region of every `source_map`
entry of the node, compared in that order (the identity as UTF-8 bytes, the
positions as integers). A node with one entry and one region has that region
as its key; the minimum makes the key a total order when a node has several
entries or regions, and two members that share a start are ordered by their
end. A `clock_place` that has no region of its own (the model document of a
`model_root` node is a host-supplied document, not a source artifact of
`source_map`: its bytes are not a source and no region
points into it) carries the region of its model's declaration, which a
`source_map` entry must give it, so several places can have one minimum
triple, and the order key of a `clock_place` is the pair (that triple, its
**place path key**). The path key compares the steps of `path` in order and
ends the comparison at the shorter path when it is a prefix of the other: a
step is ordered first by its kind (`field`, `tuple`, `union`, `slot`), then a
`field` step by the UTF-8 bytes of its `name`, a union step by those of its `case`, a `tuple` step by its
index, and a `slot` step by the UTF-8 bytes of its `collection`, then its
`minimum`, then its `maximum`, `unbounded` above every integer. The `selector`
of a slot step is a reference to a node and the `resolved_path` is derived from
the path and the document, so neither enters the key; a member that is not a
`clock_place` is ordered, when it shares its minimum triple with a neighbour,
by a member key and never by a source region of a shared term. A
**term** (an owner, a slot, a binder, a condition, a predicate) is keyed by its
**structure**: its semantic form, then the shape of its body, a reference
replaced by the key of its target (a `self` or a binder by its name and level, a
deref or alias chain, a field, union case, tuple index or slot by the rules of
the place path key), a literal by its kind and value; the `type` reference of a
literal and every region are not read, so a term shared by many
nodes keys each of them the same wherever else it is used and a node's position
never changes when another member uses the term. The member keys, compared
element by element, are: a `clock_constraint` of kind `place`, (0, the place
path key, owner, slots in order, comparison, bound); of kind `parameter`, (1,
operation name, parameter name, comparison, bound); a `clock_reset` of kind
`place`, (0, the place path key, operation name, condition, binders in order,
owner, slots in order, value); of kind `parameter`, (1, operation name,
parameter name, value); a `time_invariant`, (every bound's key in order, then
its predicate); a `clock_guard`, its operation name; an `urgency`, its kind and
operation name. The comparison ranks `lt`, `le`, `eq`, `ge`, `gt`, bounds and
values compare as rationals, and a shorter list is below a longer one it is a
prefix of. The key is a
function of the source region, the declared path and the member's own structure (the structure of its terms, its operation and parameter names, comparison, bound and value). Two places with one triple and one path
are one repeated path and are refused as out of order, before
`duplicate-clock-place`. The same strict order holds in the lists the clause
forms nest: the `constraints` of each `parts` entry of a `clock_guard` and the
`bounds` of a `time_invariant`, whose members are `clock_constraint` nodes
ordered by the same order key, the minimum triple of their regions and then the
member key above. No node
id enters the order, so a node's position
in an aggregate does not change when another member is added or changed and
two compilations of one source list a model's members identically; a
one-member or empty aggregate is in order. A member whose order key is not
strictly above its predecessor's (out of order, or equal) refuses
`invalid_semantic_graph`, with no cause, at that member,
`/semantic_graph/nodes/{i}/body/members/{b}/value/members/{j}` for a root
(`b` the index of the binding, `j` the index of the first such member) and
`/semantic_graph/nodes/{i}/body/members/1/value/members/{p}/members/{j}`
for the constraints of the part `p` of a guard (`j` the index in the part, from
1) and `/semantic_graph/nodes/{i}/body/members/1/value/members/{j}` for the
`bounds` of an invariant. It is the noncanonical order that FR-322-AC-10 refuses and is
reported before `duplicate-clock-place` and the unlisted-guard refusal of the
same node. A producer writes this order and a reader refuses every other, so a
node's aggregate has one encoding whatever the order its members were
collected in. The second place of `GD-02` starts after every published place,
so `GD-02`, which appends it last, is in order and refuses only at its forged
row.

**Selected timed package.** The slot vectors do not stand alone: they name a
complete selected package (`selected_package`), its selected document
(the one `selected-documents.json` pairs with that package, a full FCD semantic IR 2.0.0 document of `acme/orders`
built as in FR-322-AC-151, carrying no digest member) and the identity of the package's `model_root` owner
(`selection_identity`). The package is the dense timed fixture with the places
of `SL-01` to `SL-07`, the places `v` and `q` of the presence section, their
parameters and `PR-01` and `PR-03` added before the model root, every place
listed by the timed root. Each resolved row of a place is derived from the
document alone. A collection whose element is a collection is declared by an FCD
type definition of kind `sequence` (`items`) or `map` (`values`) that names the
element type; the field names that definition as its `typeRef`, its
multiplicity gives the outermost level, and the type renders as the definition
wraps it (`Sequence<Sequence<ix://quire/native/Clock>>` for `g` below). A
`sequence` or `map` definition carries no bounds, so the level it declares is
unbounded.

**Presence.** `present(p)` over a clock place is a derived Boolean predicate
and is the application of `quire.op.option.present` (form `presence_read`,
result type Boolean). The existing form over one Option-typed operand is
unchanged, with the same body and so the same node; the place form has the
arguments, in order: a reference to the
`clock_place` template node of `p`; the owner term, an expression of the
place's owner object type (`self`, another object such as `other`, a parameter
or a bound variable); and one term for each `slot` step of the path in path
order, of the selector's domain (an Integer position, or a key for a `Map`).
The place identity (owner, `clock_place` node id, slots; QSL FR-230 "Place
identity") is the whole operand, so no record, tuple, model, type or
`clock_place` node carries a presence bit or an `Option<Boolean>`, and the
`object_type` node of the owner, a function of its `owner` and its empty body, is the
same node with or without an `Option<Clock>` member. A quantifier over a clock
collection (`forall(e in p: ...)`, `exists`) binds `e` as a `parameter` of the
selector's domain (the one case in which a parameter's `semantic_type` is the
slot's selector domain and not the element's value type), and `present(e)` and every other read of the element place
take that parameter as the slot term. The indexed read of an element (`self.q[i]`) is not a presence form in this version: no vector or criterion publishes it. The reader refuses
`ill_typed`/`operator-ineligible` at `/body/arguments` an argument count other
than two plus the number of slot steps, and at `/body/arguments/0` a first
argument that is neither a `clock_place` node nor an Option-typed term; and
`ill_typed`/`type-mismatch` at the argument an owner term not of the owner type
or a slot term not of the selector's domain. An `Option` or `?` wrapper adds no
path step: the `resolved_path` row of the field carries the rendered declared
type `Option<T>`, and the terminal row is the one whose type is `Clock` once
the wrapper is removed.

**Clause nodes.** The five clause forms are closed, and every field is
authored on the wire: no member is derived from another node and no node
carries a field that only a reader can fill. A `time_invariant` is exactly
`predicate`, `bounds`; a `clock_guard` is exactly `operation`, `parts`; an
`urgency` is exactly `kind`, `predicate` for `when`, or `kind`, `model`,
`operation_name` for `operation` (an urgency of kind `operation` keeps its
published layout and is not changed here: it names its operation by the model
and the operation's name because it reads no frame, while the guard, the reset
and the parameter constraint read the frame or the operation's parameters
through the anchor; reconciling the two names is open); a `clock_reset` and a `clock_constraint` each
have two kinds, selected by their first binding `kind` (a `text` literal):

| form | `kind` | exact bindings, in order |
| --- | --- | --- |
| `clock_constraint` | `place` | `kind`, `place`, `owner`, `slots`, `comparison`, `bound` |
| `clock_constraint` | `parameter` | `kind`, `operation`, `parameter`, `comparison`, `bound` |
| `clock_reset` | `place` | `kind`, `operation`, `place`, `owner`, `slots`, `binders`, `condition`, `value` |
| `clock_reset` | `parameter` | `kind`, `operation`, `parameter`, `value` |
| `clock_guard` | none | `operation`, `parts` |
| `time_invariant` | none | `predicate`, `bounds` |

`comparison` is a `text` literal, one of `lt`, `le`, `eq`, `ge`, `gt` (one
word for each relation of a clock constraint; `eq` is the relation FR-415
writes `=`, admitted in a guard because a guard may test equality, and excluded from an invariant, which bounds a delay from above and refuses an equality as `time-invariant-shape`), and `bound` and `value` are quantity literals of the root's unit
node, nonnegative and in lowest terms. `operation` is a `reference` to a
`state`/`operation_anchor` node of the root's model with no execution point
(FR-342-AC-11), the one node that stands for the operation in every clause
that names it. `place` is a `reference` to a `clock_place` template; `owner`
and `slots` are the owner term and the slot terms of the place identity
exactly as `present(p)` carries them (Presence, above), so a constraint or a
reset of an element place names its element by terms, not by a position stored
in the template. `binders` is the ordered aggregate of `parameter` references
that the reset quantifies (`for e in p`), each one of the reset's own `slots`,
in slot order; `condition` is a Boolean `expression` reference, the
condition under which the reset happens.
`parameter` is a `text` literal naming a parameter of the referenced
operation whose declared type is `Clock`: a `Clock` parameter is bound by
place (FR-415 "Clock parameters"), so the `parameter` constraint bounds the
clock the call supplies and the `parameter` reset sets it to `value`, over the
places of the clock fields the operation's frame names (a frame that names no
clock field is the source refusal `clock-write`, and a package that carries
such a reset refuses as below). The `Clock` parameter is the only parameter
form: it is an inline `text` name, not a node, and the root gains no member. `parts` is a nonempty aggregate whose members
are aggregates, each exactly the binding `predicate` (a Boolean `expression`
reference) followed by one or more `reference` members to
`clock_constraint` nodes, the constraints that hold in the operation when the
predicate does, in order (a member of a part after `predicate` is a bare reference, not a
binding: the generic `SemanticTerm` that every consumer of the V2 schema shares
admits an aggregate in a binding value only one level deep, and a nested
`constraints` binding would be a third, so the part is flat instead of that
shared schema being widened); `bounds` is a nonempty
aggregate of references to `clock_constraint` nodes of kind `place` whose
`comparison` is `lt` or `le`: an invariant is an upper bound, which FR-415
states with `<` and `<=` only.

**Clause membership is derived on the clause side.** A `clock_place` carries
no clause node and no `guards` field: a place cannot list the `clock_guard`
nodes that reach it through their constraints, which would be a cycle in a graph
whose nodes depend on their dependencies, and a list of their predicate expressions
would only repeat what the parts already say, unchecked. The field is removed
from the place, and the root's `guards` (FR-322-AC-243) is the only list of
`clock_guard` nodes. A clause is a member of
the model by the root aggregate that lists it, a `clock_constraint` is a member
through the guard part or invariant that names it, and the root's
`constraints` lists exactly the constraints that a part or an invariant names,
no other: a listed constraint that nothing names refuses
`invalid_package`/`timed-node-reference` at its member of the root,
`/semantic_graph/nodes/{i}/body/members/8/value/members/{j}/target`. **Every
timed node that the root does not list**, a reset, an invariant, a constraint or
a place as much as a guard, refuses `invalid_package`/`timed-node-reference` at
that node's own `body`, `/semantic_graph/nodes/{i}/body`, `i` the graph index of
the first such node in ascending node-id order (the rule of the unlisted
guard, FR-322-AC-243). A model has at most one `clock_guard` for each operation
anchor, every `parts` entry of the operation being one member of it: a second
guard for one anchor refuses at that second guard node's own `operation`
binding, `/semantic_graph/nodes/{i}/body/members/0/value/target`, `i` the graph
index of the second guard in the root's `guards`. A guard and a reset name the
same anchor node, and a reset of the place `p` in the operation `o` requires
`o`'s frame (FR-340) to modify the object field that begins `p`'s path, so
`size()` cannot reset a clock it does not write. A reset of kind `parameter` has
no `condition`: a conditional reset of a `Clock` parameter does not exist in
this version, and a `condition` binding on one is a schema refusal (`CL-28`).

**Quantified clauses are out of scope.** A guard or an invariant whose body is
`forall(e in p: B)` or `exists(e in p: B)` over clock places (FR-415 "Reading")
has no binder form in a `clock_guard`, a `clock_guard` part or a
`time_invariant`; only a `clock_reset` carries `binders`. A producer that meets
one refuses with `unsupported_construct`/`not-yet-implemented` at the
quantifier and names STD-554, the ticket that defines the binder form; it does
not write an unquantified approximation. The vectors `QN-01` and `QN-02` of
`timed-clause-node-vectors.json` publish it; TC-367 CK-37 and FR-415-AC-24 state it.

**Forms the package excludes.** Three forms are excluded from the checked
package by rule. For each, the producer refuses `unsupported_construct`/`expression-form`
at the form, naming the construct (a guard disjunct that reads no clock, a clock
write that is not a reset to a constant, a selection reset whose condition reads
element data); it writes no clause node for the operation and no approximation
of the form, and a reader never meets one. The
reader rules do not change: each exclusion is the closed layout or the join
written above, pinned by a package row. The producer-level cases are the
`excluded_forms` of `timed-clause-node-vectors.json`.
- **A guard disjunct that reads no clock** (`GJ-01`:
  `self.x >= 1 ms or self.versionNumber > 0`). A part pairs a predicate with at
  least one constraint (FR-322-AC-261, `CL-20`), so a disjunct is a part only
  when it holds a clock constraint, and a part with no constraint is not a
  form. Every disjunct that reads a clock is built, one part
  each (`GJ-02`, the guard of node 47), and a disjunct whose source writes the
  constraint `self.x >= 0 ms` builds it as written (`GJ-03`): the producer
  never inserts a constraint to give a data-only disjunct one.
- **A clock write that is not a reset to a constant.** The one write node is
  the `clock_reset`, of kind `place` or `parameter`, whose `value` is a required
  quantity. A drop is not a reset (`CW-03`: `self.v = absent`), so a reset
  without a `value` is refused by both schemas (`CL-55`). A whole-collection write
  (`CW-01`: `self.ts = sequence[Timer { x: 0 ms, n: 1 }]`; `CW-02`:
  `self.ts = filter(e in self.ts: e.n > 1)`) creates, carries and removes slots,
  and a reset names one place by its owner and exactly the slot terms its path
  has slot steps for, so no reset names a collection as a whole (`CL-56`, the
  slot count). The class is a write whose value is a collection of clock-bearing
  elements: a `sequence[...]`, `set[...]`, `bag[...]` or map literal, `filter`,
  `flatten`, concatenation, slicing, reordering, and a collection element
  written as a literal (a map entry `self.m[k] = Timer { ... }` or an ordered
  element `self.ts[i] = Timer { ... }`). Every member of the class but the
  ordered element creates, carries or removes slots (the map entry creates a slot
  when its key is absent). The ordered element (`CW-06`) is excluded for a
  different reason:
  an element write to a slot that exists creates, carries and removes none, but
  it replaces the element at a slot chosen by a dynamic term `i`, data part and
  clocks together, and the package has no indexed element-write form. A reset
  names a place and writes only a constant to its clock, carries no data
  component, and FR-415 gives the ordered element write no language rule (no
  criterion or vector of FR-415-AC-12, FR-415-AC-14 or TC-367 CK-15 to CK-17
  and CK-20 publishes one).
  The producer therefore refuses `self.ts[i] = Timer { ... }` instead of
  lowering it to a slot reset and a data write; the built forms that reach the
  same slot are the reset `self.ts[i].x = 0 ms` (FR-415-AC-12) beside ordinary
  data writes. A write of a non-collection composite is not in the
  class: `self.o = Timer { x: 0 ms, n: 3 }` is the reset of the place `o.x` to
  `0 ms` (making an `Option<Timer>` place present) together with an ordinary data
  write of `n`, and is built. The drop is the write `absent` to an `Option<Clock>`
  place (or a slot of `Sequence<Option<Clock>>`), whose presence is a runtime
  bit; `self.o = absent` over `o: Option<Timer>` is not that drop, because the
  presence of `o.x` is derived from the data case of `o` (Presence, above), so it
  is an ordinary data write that builds no clause node and ends the presence of
  `o.x` with it. A constant write to a guarded place is a reset and makes the
  place present (`CW-04`, node 82). A presence requirement is not a write
  (`CW-05`): `present(p)` in a postcondition is a `presence_read` expression of
  the postcondition and not a clause, so the package carries no node for it; a
  place absent in the pre-state meets it only when a reset of the operation
  writes the place (FR-415 "No implicit reset on a presence change"; `FP-01`,
  `FP-09`, `FP-11`), and the source refusal `clock-presence-unwritten` is as
  FR-415 states. An unwritten clock keeps its value, so a frame that names a
  clock field imposes no write.

  These exclusions concern what a package-producing frontend builds. The
  language rules of FR-415 (AC-7, AC-11, AC-12, AC-14) are unchanged: they say
  what the source forms mean, and a frontend that produces a checked package
  refuses the forms listed here instead of building them.
- **A selection reset whose condition reads element data** (`SD-01`:
  `forall(e in self.ts: e.n > 1 implies e.x = 0 ms)`). A binder is a
  `parameter` node among the reset's slots, bound by slot position only (the
  index of a `Sequence` or `OrderedSet`, the key of a `Map`, the canonical
  position of a `Set` or `Bag`) and typed by the selector's domain (`CL-10`,
  `CL-12`; no published row pins a read of element data, because no package can
  carry one). The package has no expression that reads the data part of the
  element at a binder, and the package defines none, so a `condition` reads no element data: it is a Boolean
  expression over the owner, the operation's parameters, the state data of the
  owner and the presence of the binder's place (`SD-02`, node 84:
  `present(e)`), evaluated in the pre-state at each live slot.

**Reader joins.** The reader checks, in this order: the aggregate order below
for every aggregate of the package; the unlisted timed nodes; then each timed
node in ascending node-id order, and within a node the joins of its form
in the order written here; then the model-wide rules (one guard for each
anchor, each listed constraint named). A refusal is
`invalid_package`/`timed-node-reference` at the target of the binding named
(`/body/members/{k}/value/target`, `k` the index of that binding; a `parameter`
name, a literal, refuses at the literal itself, `/body/members/2/value`)
unless another code is written.
- `clock_place`: the rows of `resolved_path` join the path and the selected document (FR-322-AC-148); it has no clause to join.
- `clock_constraint` of kind `place`: `place` is a `clock_place`; then an
  `owner` that is not of the owner type refuses `ill_typed`/`type-mismatch` at
  `/body/members/2/value`; a `slots` count other than the number of `slot` steps
  refuses `ill_typed`/`operator-ineligible` at `/body/members/3/value`; a slot
  term not of the selector's domain refuses `ill_typed`/`type-mismatch` at that
  slot (the rules of `present(p)`).
- `clock_constraint` of kind `parameter`: `operation` is a `state`/`operation_anchor`
  of the root's model with no execution point; then `parameter` names a
  parameter the operation declares, then one declared with type `Clock`.
- `clock_reset` of kind `place`: `operation` as above; `place` and the owner and
  slot typing of the constraint, at binding indexes 2, 3 and 4; then the
  operation's frame modifies the field that begins the place's path; then each
  `binders` member is a `parameter` node among the reset's `slots`, in slot
  order; then `condition` is a Boolean `expression`.
- `clock_reset` of kind `parameter`: `operation` as above; then `parameter` as
  for a constraint; then the operation's frame modifies a clock field of the model.
- `clock_guard`: `operation` as above; then for each part, `predicate` is a
  Boolean `expression`; then each constraint is a `clock_constraint`, and a
  constraint of kind `parameter` names the guard's own anchor.
- `time_invariant`: `predicate` is a Boolean `expression`; then each bound is a
  `clock_constraint` of kind `place` whose `comparison` is `lt` or `le` (`eq`,
  `ge` and `gt` are not upper bounds).
A constraint that is not above its predecessor in a part, or a bound not above
its predecessor in an invariant, refuses `invalid_semantic_graph` with no cause
by the aggregate order, and that check comes first. The `urgency` of kind
`operation` and the `Clock` parameter named by a constraint or a reset are the
only places a clause names an operation other than by its anchor, and the
urgency keeps its published layout (above).

Closed layouts, counts and orders are in both schemas; all terms have their declared reference or literal type.
These closed layouts reject omitted, extra, reordered, wrongly typed and
wrong-count children before semantic graph checks (an empty `parts`, a part
without a constraint, an empty `bounds` and a `comparison` outside the five
words are schema refusals). Reset nodes carry the normalized nonnegative
quantity; guard nodes carry the constraints they bound and invariant nodes
their upper bounds; urgency carries either its data predicate or operation
reference; each clock constraint carries its comparator, normalized quantity
and unit reference. `Clock` is not a
value or scalar type node, and
clock fields do not appear as data fields in record/model nodes.

The `TimedModelNode` variant in `schema.json` fixes each node's closed outer
shape, tag and semantic form. Timed nodes occur once in graph
order and each has a structural `node_id`. The timed root's `time_dimension` is the selected base
dimension `Time`; the source unit's `dimension_node_id` MUST equal that exact
node identity. A base `Length` dimension, a derived non-Time dimension and a
nominal derived-Time dimension are not interchangeable with that root even
when their dimension terms are equivalent. The reader validates node shape,
ownership, dependency closure, path canonicality, unit dimension and exact
rational encoding in ascending node-id order. A tick source cannot own any clock
place. It refuses missing members, unknown members, dangling
references, wrong node kinds, duplicate places, malformed paths and invalid
quantities with these paired diagnostic causes: a missing root is
`invalid_package`/`missing-timed-model`, an unresolved or wrong-kind target is
`invalid_package`/`timed-node-reference`, a repeated place whose
rows all join is `invalid_package`/`duplicate-clock-place` (a second node of a
path carries a forged row and is `malformed-clock-place`, `GD-02`), a malformed path is
`invalid_package`/`malformed-clock-place`, a clock place under a tick source
is `invalid_package`/`clock-without-dense-time` at that place's `model`
reference, and a source unit whose dimension node does not equal the root's
base `Time` node is `ill_typed`/`invalid-time-quantity` at the source's `unit`
reference. The source-unit rule applies to both dense and tick sources. An
invalid or non-time timed quantity is also `ill_typed`/`invalid-time-quantity`.
The `TimedModelNotEmitted` condition is not a valid package-read
outcome for an S3-admitted model. I2 reconstructs the checked timed model from
the graph and verifies the same normalized constraints.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-322-AC-1 | Every admitted semantic occurrence maps to at least one original source region. | Analysis |
| FR-322-AC-2 | Changing backend capability availability cannot change package admission or the contents of the package. | Analysis |
| FR-322-AC-3 | Changing a meaning-affecting lock member changes the package: the recompiled value is not structurally equal to the one before the change. | Analysis |
| FR-322-AC-4 | The public reader refuses a document that is not a checked package (`unknown_contract_version`), malformed/duplicate members, cross-domain substitutions, unknown required capabilities and non-canonical bytes before exposing an admitted package. | Test (TC-217) |
| FR-322-AC-5 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-6 | Exact selected limits admit the boundary vector; one-over-byte, node, edge, locus, diagnostic and work inputs return incomplete resource accounting naming the limit, its value and its setting, and no partial checked package. | Test (TC-217) |
| FR-322-AC-7 | A consumer obtains the complete closed semantic graph through the public schema and never through a native parser/CST/package implementation type. | Inspection |
| FR-322-AC-8 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-9 | The reader result is exactly admitted, refused or incomplete; malformed input and the first exhausted measured limit remain distinguishable and carry their specified typed provenance/accounting. | Test (TC-217) |
| FR-322-AC-10 | Non-normalized rationals, invalid owner joins and every other noncanonical member or term order this artifact refuses -- including an application node's `dependencies` list -- refuse as `invalid_semantic_graph`, except a `state`/`frame` node's own `modifies`/`creates`/`deletes` canonical order, which [FR-340-AC-4](FR-340-checked-package-frame-location.md) owns. | Test |
| FR-322-AC-13 | The published V2 checked-package schema admits its positive fixture carrying all 15 closed node tags with an exact source-map entry for every occurrence, and refuses each authored document-type and node mutation. | Test |
| FR-322-AC-14 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-15 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-17 | The published operation catalog is closed: both V2 schemas carry identical operation definitions and enumerate exactly its identities, law roles, mode kinds and values, member kinds and operator classes; every law-role `DefinitionRef` names a complete-value lock definition by authority and identity; every entry's operands, rest, result form, laws, mode, member, leaf source and constraints use only catalogued families, groups and kinds; integer `div`/`rem` require the integer-division law and `mod` none, IEEE operations require the IEEE profile and have no `eq`, `ne` or ordering identity, and `negate` per width takes one float and returns the operand's type; text comparisons and size require the text profile law and mode; the six FR-478 text conversions carry the `text_profile` law, the six string conversions do not, IEEE conversion targets also carry `ieee_profile`, each conversion names its target with a `type_argument` member and `member_family` constraint, and `rounding` is pinned by decimal, IEEE and quantity result types while integer and rational conversions have no mode; `all_instances` takes one population, `lookup` requires an absence mode, `navigate` and `reaches` a relationship end, `reaches_field` a `field` member and the `reference_edge` constraint, `quire.op.state.clause` the `state_clause` member kind and no law, and the record, tuple and index projections, numeric/total-order/bit-identical IEEE comparisons, `deref` and `dispatch_call` are present. | Test (TC-233) |
| FR-322-AC-18 | Withdrawn: the published operation vectors and their mutation file it checked are removed. | Withdrawn |
| FR-322-AC-19 | Withdrawn: the published operation mutations it checked are removed. | Withdrawn |
| FR-322-AC-21 | Every node of each published V2 package carries a `declaration` member exactly when it declares; the schema refuses a declaring node without it and an expression with it; a name duplicated on two nodes refuses as `ambiguous-name` at the node later in node-id order; and a source nominal node whose `node_id.path` differs from its `declaration.qualified_name` refuses as `declaration-nominal-mismatch` before any ambiguity. | Test (TC-233) |
| FR-322-AC-22 | Every clause application of the published clause package carries exactly one profile law of its catalogued role equal to the lock's `profile_selections` row, and every clause application other than `quire.op.temporal.clause` (FR-370) a `profile_operator` member the profile defines; removing the law refuses as `operation-law-missing`, a protocol profile in the temporal role as `operation-law-mismatch`, a lock without the temporal profile row as `operation-law-unselected`, and an undefined, other-profile or absent operator as `operation-member-mismatch`; for a `quire.op.temporal.clause` law naming no FR-250 profile, the full reader reports FR-370's earlier profile check (`unknown_profile`) before the operation step's `operation-law-mismatch`. | Test |
| FR-322-AC-23 | Every application carries a `result_type` and every literal a `type`; a root `result_type` differing from the node `semantic_type`, a `result_type` the operation cannot produce, and a literal whose `type` family its `value_kind` does not admit each refuse as `operator-ineligible`, while the same literal with its declared type is admitted. | Test (TC-233) |
| FR-322-AC-24 | The FR-149 conversion classification is decided from declared bounds: an integer range into a containing integer or rational range is exact, an integer range whose scaled bounds exceed a decimal range is range narrowing, a decimal of scale 3 into scale 2 is scale reduction and a rational into narrower rational bounds is rational narrowing; `quire.op.rational.div` over integer-range operands carries `promotes_exact` and its published vector is admitted, while a decimal operand refuses as `operator-ineligible`. | Test |
| FR-322-AC-25 | An `operation` member resolves only against a declaring `model` node of `semantic_form` `object_type`, `process`, `state_machine` or `persistence_interface`; every other model form, including `systems_interface`, refuses `operator-ineligible`/`ill_typed`. | Test |
| FR-322-AC-26 | A reference to a dependency package's node is carried as `PackageNodeRef{package, node}`, `package` the dependency's identity and `node` the node's structural `node_id`; a reader resolves it only against the named dependency's admitted package, refusing a `package` absent from `dependency_selections` as `missing_declaration`/`missing-selection`. | Analysis |
| FR-322-AC-27 | An unrecognized `node_tag` refuses as `unsupported_node_tag`, and QSpec revises the closed `node_tag` set in place. | Analysis |
| FR-322-AC-28 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-29 | A model declaration node carrying `Order`'s `ModelOwner` resolves to `Order`. A node whose owner identity is not lock-selected refuses `missing_declaration`/`missing-selection` in the owner join, and a current `model`/`object_type` node whose owner names the relationship `Order-order-Order` refuses `missing_declaration`/`missing-selection` in the owner join. A node whose `semantic_type` is not its own id, that carries a `recursion_group`, or whose body has members refuses `invalid_package`/`invalid-derived-shape` (MW-04, MW-05). A model declaration node that carries a `declaration` beside its `ModelOwner`, or carries no `owner`, fails the V2 schema and refuses `malformed_wire`. | Test |
| FR-322-AC-30 | A model-owned `field` or `operation` member resolves among the declaring type's exposed effective members, own and inherited. A field declared by a supertype resolves on the subtype's node, and an operation redefined through `Operation.redefines` (FR-154) resolves to the most derived redefinition. An operation on a `systems_interface` node refuses `ill_typed`/`operator-ineligible`. Two exposed fields of one name refuse `ambiguous_declaration`/`ambiguous-name`, and so do a supertype's operation and a subtype's operation of one name when the subtype's operation has no `redefines` naming it. An undeclared name, a name of the other member kind and a relationship declaring node each refuse `ill_typed`/`operator-ineligible`. | Test |
| FR-322-AC-31 | A resolved model member's type follows the declared-type, multiplicity and presence tables. The derived types are `Int[0, 9]` from an integer value type, `Integer` from a constrained native `Integer`, `Option<Boolean>`, `Set<Integer>[0, 3]`, `Sequence<Integer>[0, 5]`, `Set<Integer>[0, 1]`, `Set<Integer>` for `[0, unbounded]`, `String`, `UUID`, `Timestamp`, `Float32` and `Float64` from their native declared types (`String` distinct from `Text`), `Reference<Invoice>` and an operation's `Integer` result. A bare `Text` field (no `Text[min, max; profile]` value type), a `[1, unbounded]` field and an operation with no result each refuse `ill_typed`/`operator-ineligible`. So does a call whose receiver is not a reference to the declaring node, or whose arguments differ in number from the operation's parameters or in type node from them, unless a record argument conforms to its record parameter ("Record conformance"). | Test |
| FR-322-AC-32 | A domain package document named by a `model_root` node is checked as evidence before graph admission: a document supplied under the root's identity with that identity passes the selection admission checks, no document refuses `missing_import`/`missing-selection` and a document whose identity differs from the root's `invalid_model_binding`/`wrong-model-selection`. | Test |
| FR-322-AC-33 | `quire.op.reference.eq` and `quire.op.reference.ne` admit two references to selected object types of which one conforms to the other along declared supertypes: two of one type, a subtype and its supertype in either operand order, a subtype two levels down, and a type with two supertypes against either. A systems interface with no declared supertype against an unrelated object type, two unrelated object types, a reference to a relationship, even compared with itself, and an operand of family `object` refuse `ill_typed`/`operator-ineligible`. An operand whose owner is not selected refuses `missing_declaration`/`missing-selection`, even compared with itself. | Test |
| FR-322-AC-34 | `quire.op.record.project` over a `quire.op.model.deref` result reads a field of a model object: the `deref` result has family `object`, the projection resolves the field on the declaring object type, own or inherited, and its result is the field's member type. Its field member carries `read_form: bare` for `view.total` and `read_form: explicit_deref` for `deref(view).total`; otherwise identical operand, field and result data yield distinct member values, and a reader preserves each value. A missing `read_form`, an inapplicable `read_form` (including one on an undereferenced reference operand or an ordinary record projection), and an unrecognized value refuse `malformed_wire`. A projection over a reference that is not dereferenced, a `field` member naming a node other than the operand's object type, a `deref` whose `result_type` is not the operand's object type, a `deref` result of a relationship, and a projection whose `result_type` is not the field's member type refuse `ill_typed`/`operator-ineligible`. | Test |
| FR-322-AC-35 | A `dependency_selections` entry is a closed `DependencySelection` `{identity}`: the published package carrying two such direct entries, one per identity in ascending identity order in its lock, is admitted by the V2 schema; an entry with a `Selection` or `DefinitionRef` shape, no `identity`, any member beside `identity` or an extra `version` member fails the schema at the mutated entry, and a lock with a `model_selections` member fails the schema; a reader refuses a `DependencySelection` carrying `version` `unknown_member`, since a library binds by its `name` and a model by the identity of its `model_root`. Entries ordered by UTF-8 bytes where UTF-16 code-unit order differs are admitted; a second entry of one identity refuses `invalid_package`/`conflicting-definition` with both entries as loci, and an entry not strictly after its predecessor refuses `invalid_package`/`invalid-value` at its pointer. | Test (TC-233) |
| FR-322-AC-36 | A `{term: "dependency_reference", package, node}` term whose `package` is a dependency identity string and whose `node` is a `NodeId` whose `path` holds only strings is admitted by the V2 schema as an application argument; an empty `package`, a missing `node`, an extra member and a `node` whose `path` holds an index segment (`NI-07`) each fail the schema. | Test |
| FR-322-AC-37 | A package whose function calls a dependency `function` declaration over `Int[0, 9]` returning `Boolean`, through a `dependency_reference` callee, with the Boolean node as `result_type` and no `dependencies` entry for the target, is admitted when the dependency's admitted package is supplied. With no dependency package supplied it refuses `missing_import`/`missing-selection` at the selection entry; with the callee's `package` absent from `dependency_selections`, `missing_declaration`/`missing-selection`; with `node` naming a node that carries no `declaration`, `missing_declaration`/`missing-name`; with `node` naming a declared record, a function returning a declared record, a function over `Set<R>` or over a tuple holding `R` for a declared record `R`, a function over `Reference<M::T>` for a model type `M::T`, or with the term as a non-callee argument or as a node body root, `ill_typed`/`operator-ineligible`; with the target listed in `dependencies`, `invalid_semantic_graph` at the node's `dependencies`. The published `fixtures/positive-dependency-library.json`, `fixtures/positive-dependency-call.json` and `dependency-call-vectors.json` carry the library, the importer and each mutation with its refusal and locus. | Test (TC-233) |
| FR-322-AC-38 | `quire.op.model.reaches_field` admits a field edge of the owner's own reference type: `Order.parent` (`Option<Reference<Order>>`), `Order.prior` (`Sequence<Reference<Order>>[0, 3]`) and `Order.origin` (`Reference<Order>`), the inherited `parent` read on `Sub`, and a target of a subtype of `Order`. A set edge, an unbounded sequence edge, an edge to another object type, a non-reference field, a target of an unrelated type, an operand 0 of a type other than the member's declaring node, an undeclared field name and an operand of family `object` refuse `ill_typed`/`operator-ineligible`; an ambiguous field name refuses `ambiguous_declaration`/`ambiguous-name`, and a declaring node owned by a domain package identity the lock does not select `missing_declaration`/`missing-selection`. | Test (TC-281) |
| FR-322-AC-39 | Every node body of every published V2 package conforms to the five-stratum body grammar, and the V2 schema admits exactly that grammar. | Test |
| FR-322-AC-40 | A body carrying an application inside an application's arguments, an application or a binding of an aggregate inside an aggregate's members, or an aggregate inside a Group's members fails the V2 schema and is refused `malformed_wire`, and the same meaning with each composite subterm as its own referenced node is admitted. | Test |
| FR-322-AC-41 | A package holding a 100,000-deep expression has the same JSON nesting depth as a package holding a one-level expression, is emitted, read back and admitted under node, edge and byte limits sized for its node count, and no reader limit or effective-limit record names a depth. | Test |
| FR-322-AC-42 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-44 | A field typed by a value type bound as `Text[min, max; profile]` has type `Text[min, max; profile]` over a `text_bounds` node; a value type bound as `Text` with `min > max` or with a profile outside FR-141's six (a negative `min` is not a document: `minLength` is a non-negative integer) gives the member no type and refuses `ill_typed`/`operator-ineligible`. | Test |
| FR-322-AC-45 | `quire.op.structural.eq` over two `Label` values of `union Label { Named(Text[0,8;nfc]), Tagged(Integer, Text[0,8;nfc]), Empty }` carries exactly the leaves `["member:Named", "position:0"]` and `["member:Tagged", "position:1"]` in that order, each with the `nfc` text law and mode, and none for `Empty`; both V2 schemas admit the `member:<Ident>` segment. | Test |
| FR-322-AC-46 | `quire.op.structural.eq` over two values of `union List { Cons(Integer, List), Nil }`, whose cycle names one `recursion_group`, is admitted with empty `leaves`, and a record whose field's type cycles back to the record through a union is likewise admitted; neither refuses `operator-ineligible`. Over `Node`, `Cell`, `Chain` and `Option<Chain>` of the "Structural leaf walk" examples it is admitted with exactly the listed leaves in that order, each recursion leaf with `laws` `[]` and `mode` `null`; both V2 schemas admit the `recursion:<d>` segment. | Test |
| FR-322-AC-47 | The V2 `Diagnostic` schema admits `type-mismatch` and `non-boolean-root` only with `ill_typed`, `missing-selection` only with `missing_import` or `missing_declaration`, `unsupported-selection` and `wrong-selection-role` only with `unknown_language`, `unknown_edition` or `unknown_profile`, and `declaration-form`, `expression-form` and `not-yet-implemented` only with `unsupported_construct`. | Test (TC-233) |
| FR-322-AC-48 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-49 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-50 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-51 | Every owner-bearing node of every published V2 package carries `owner`: a `SourceOwner` on a node with a `declaration` and on a `dimension_equivalence` node, and the `ModelOwner` that admitted it on a model declaration node, clause function or model-owned nominal unit or dimension; no other node carries one. The V2 schema refuses, and a reader refuses `malformed_wire`: a declared record without `owner`, a declared record with a `ModelOwner`, a model declaration node without `owner` or with a `SourceOwner`, an `owner` on the builtin `Integer` node and on an application node, a `DefinitionOwner`, a `SourceOwner` with an extra member and a `ModelOwner` with `version`. | Test |
| FR-322-AC-52 | The owner join joins each node's `owner` to the lock before the derived-shape semantic checks: a `SourceOwner` whose (`authority`, `identity`) is no `lock.sources` entry's, and a `ModelOwner` whose `identity` is that of no `model_root` owner, refuse `missing_declaration`/`missing-selection` at that node's `node_id` (`/owner` for a `ModelOwner` on a tuple or variant model type, FR-322-AC-169), and are reported ahead of a derived-shape refusal at a node of lower node-id order. A declared node's `owner` and its source map agree: changing the `source` of the region of a declared node's `declaration` occurrence to another lock source refuses `invalid_package`/`invalid-value` at that occurrence's `source_map` entry. | Test |
| FR-322-AC-53 | The two-owner fixtures `positive-two-owners-a.json` and `positive-two-owners-b.json` declare the same records `Point { x: Integer; }` and `List { next?: List; }`, with the qualified names QSL emits for them, `["Point"]` and `["List"]` (QSL FR-092 D1), in both, with identical bodies, under the `SourceOwner`s (`agent-ix`, `example-a`) and (`agent-ix`, `example-b`). Both are admitted. Both also declare `three` (qualified name `["three"]`), a declared `value` whose body root is the application `1 + 2` (no `owner`). Their `Point` and `List` nodes have different `node_id`s, since their `package` differs, and the two packages are structurally unequal. | Test |
| FR-322-AC-54 | The derived-shape semantic checks leave every `correspondence`/`abstraction_relation` node untouched: `positive-clause-operations.json`'s `correspondence`/`abstraction_relation` node passes the stage and the full reader admits the package. | Test |
| FR-322-AC-55 | The owner join resolves every `ModelOwner` against the selected document, whether or not anything reaches the node: a model declaration node whose owner `node` the document does not declare, and a `model`/`object_type` node whose owner names the relationship `Order-order-Order`, each refuse `missing_declaration`/`missing-selection`; a precondition clause function owned by the operation member `Order/size` (MW-11, and the `clause-precondition-operation-owner` function of `positive-all-families.json`), an invariant owned by the object type `Order` (MW-12) and a body clause function owned by `Order/size` (MW-17) pass the join, while a clause function owned by an undeclared member `Order/missing` (MW-13), one owned by the field `Order/total` (MW-14), a precondition owned by the object type `Order` (MW-16, `clause-precondition-owned-by-object-type`), an invariant owned by the operation member `Order/size` (MW-18, `clause-invariant-owned-by-operation-member`), an invariant owned by the systems interface `Flow` (MW-22, `clause-invariant-owned-by-systems-interface`), a `postcondition` kind (MW-19, `clause-kind-postcondition`), a clause function with two `clause` bindings (MW-20, `clause-two-kind-bindings`) and one with none (MW-21, `clause-without-kind`) refuse `missing_declaration`/`missing-selection` at the clause function's `node_id`. The `clause-*` ids are the `adverse.json` `clause_owner_cases`. | Test |
| FR-322-AC-56 | A `relationship_end` member of a model-owned relationship resolves to one of the relationship's named ends, `sourceEnd.role` or, when the document declares one, `targetEnd.role`, and refuses `missing_declaration`/`missing-name` for any other name. The `Order-invoiced-Invoice` relationship of the published document declares no `targetEnd.role`, so it names only `invoiced`: `invoiced` resolves (`relationship-end-forward-without-inverse` in `adverse.json` `relationship_end_cases`, admitted by the full reader) and `next` refuses (`relationship-end-names-missing-inverse`; RE-ends-4 and RE-ends-5). `Order-order-Order` is `bidirectional` and names `order` and `next`. `Order-follows-Order` is `source-to-target` and names `follows` and `followedBy`: `follows` resolves (RE-ends-7) and `followedBy`, a named inverse traversed from target to source against that direction, refuses `ill_typed`/`operator-ineligible` at `member.name` (`relationship-end-inverse-against-direction`; RE-ends-8). `Order-twin-Order` names `twin` as both its source and its target role, so `twin` refuses `ambiguous_declaration`/`ambiguous-name` at `member.name` (`relationship-end-names-ambiguous-role`; RE-ends-9). A `relationship_end` whose `declaration` is not a relationship refuses `ill_typed`/`operator-ineligible` at `member.declaration`, after owner recovery and before role lookup (RE-ends-6). A missing role refuses at `member.name`. | Test |
| FR-322-AC-57 | `quire.op.model.navigate` has operator class `query`. `quire.op.model.navigate` and `quire.op.model.reaches` over a model-owned relationship end apply the checks in this order, each refusing at its recorded application pointer: declaration kind (`member.declaration`), role lookup (`member.name`: missing-name, ambiguous-name), receiver effective endpoint (`arguments/0`, and for `reaches` `arguments/1`: an operand conforms to the receiver endpoint type directly or through a supertype, else `ill_typed`/`operator-ineligible`), direction (FR-152; `member.name`), destination multiplicity (an unbounded upper or an ordered destination is `unsupported_construct`/`expression-form` at `member.name`), then the operation's own type comparison (`navigate`: `result_type` equals the canonical `Reference<U>`, `Option<Reference<U>>`, `Set<Reference<U>>[l,u]` or `Bag<Reference<U>>[l,u]` of FR-152, else `ill_typed`/`operator-ineligible` at `result_type`; `reaches`: a destination end of the receiver endpoint's own type, whether single-valued (`[0,1]`, `[1,1]`) or finite multi-valued (`Set`/`Bag` shaped, unique or not), else `ill_typed`/`operator-ineligible` at `member.name`; a `reaches` has no result type to compare, so a multi-valued destination end admits exactly as a single-valued one does). The published `relationship_navigation_cases` RN-01 to RN-27 record each. | Test |
| FR-322-AC-58 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-59 | Structural equality and inequality over U1–U7 require exactly their complete ordered leaf lists. U2's member rename, member reorder, payload swap and source arm reorder have exactly the effects stated with the vectors; U2 requires all three text leaves even when both operands select `Empty`. A missing final leaf refuses `invalid_package`/`operation-law-missing`; reordered leaves or a recursion leaf with a text law refuse `invalid_package`/`operation-law-mismatch`. | Test |
| FR-322-AC-60 | For each `K` of `set`, `bag` and `ordered_set`, equality over `K<Choice>` (U2) requires U2's leaves with one leading `inner` per path; `contains` over that collection (`inner:0`) and a catalogued `result_inner` operation yielding `K<Choice>` require U2's leaves without that prefix. A `result_inner` operation yielding `sequence<Choice>` requires `[]`, while equality over `sequence<Choice>` requires U2's leaves with one leading `inner`. | Test |
| FR-322-AC-61 | Both V2 schemas admit `member:Right`, `member:_item2`, `position:0`, `position:12`, `recursion:0` and `recursion:12`; they refuse `member:`, `member:1Right`, `position:01`, `position:-1`, `recursion:`, `recursion:01` and `recursion:-1`. The shared catalog includes `union` in `structural_kind`, so equality and inequality admit eligible Union operands under the existing `same_type` constraint and `operand:0` leaf source. | Test |
| FR-322-AC-62 | The settled derived type forms forward aliases to their resolved node without an `alias` node; `Reference<Order>` is an anonymous self-typed `composite_type`/`reference` over `Order`'s model declaration; `Population<Order>[3]` is an anonymous `bounded_domain`/`model_population` whose `semantic_type` is `Set<Reference<Order>>` and whose body binds integer `max = 3`; and a formed `metre / second` quantity is an anonymous self-typed `scalar_type`/`compound_unit` with ordered unit/exponent terms over targetless canonical nominal unit roots whose semantic type is their dimension node, and whose path embeds each term's root-unit node id and exponent text, so a formed `metre^2` and a formed `metre^-1` are two nodes with different paths. | Test
| FR-322-AC-63 | A selected domain package with `Address { street: Text[0, 64; nfc] }` and object `Account { address: Address }` admits a self-typed `model`/`record_value_type` declaration node owned by `Address`; `Account.address` derives the `Address` node as its type and a `field` projection on an `Address` value resolves `street` to `Text[0, 64; nfc]`. The record node has family `record`, and an invariant clause function owned by `Address` passes the owner join. | Test |
| FR-322-AC-64 | A record field whose `typeRef` names another record, including a field inherited by a record subtype, derives the named record node. A record-typed field with `optional` presence derives `Option<Address>`, and one with multiplicity `[0, 3]`, ordered and non-unique, derives `Sequence<Address>[0, 3]`; both retain the declared record node at their innermost element. A finite recursive record reached through an optional field retains the same record declaration node at every level. None derives `Reference<Address>` or an object identity. | Test |
| FR-322-AC-65 | A `model`/`record_value_type` node owned by a selected object type or a node absent from the selected document refuses `missing_declaration`/`missing-selection` in the owner join; one with a missing `owner` refuses `malformed_wire`. An unknown field name on a valid record node refuses `ill_typed`/`operator-ineligible`. A record field whose `typeRef` names an event type, state machine, process, persistence interface or namespace refuses `invalid_model_binding`/`malformed-declaration` at FR-154 intake before member derivation; a record node used as an object reference target refuses `invalid_package`/`invalid-derived-shape`. | Test |
| FR-322-AC-66 | A package with a `protocol`/`protocol_reference` carrier to its local, conforming `protocol_clause` target is admitted with one graph collection; its body target, sole dependency and semantic type bind that exact protocol node. With the body target and sole dependency changed together, a missing local target refuses `missing_declaration`/`missing-name` even when a selected dependency contains that node, and a present non-protocol target or unequal semantic type refuses `ill_typed`/`operator-ineligible`. A non-reference body or extra carrier member fails the V2 schema; a dependency target is not admitted as a protocol reference. The same package with two source references to that protocol is admitted with one carrier node carrying two `expression` occurrences, ordinals 0 and 1, each with its own `source_map` entry, and has the same carrier `node_id` as with one occurrence. A carrier with no occurrence or with an occurrence whose role is not `expression` fails the V2 schema; two `source_map` entries with one (`node_id`, `expression`, `ordinal`) key refuse `invalid_source_map` at the later entry's `/source_map/{i}` pointer, and a carrier whose `occurrences` array repeats an `expression` ordinal refuses `invalid_source_map` at the later occurrence's `/semantic_graph/nodes/{n}/occurrences/{j}` pointer (`n` the carrier's node index, `j` the later occurrence's index); two nodes with one `node_id` refuse `invalid_semantic_graph`. | Test |
| FR-322-AC-67 | The six identities `quire.op.string.eq`, `quire.op.string.ne`, `quire.op.uuid.eq`, `quire.op.uuid.ne`, `quire.op.timestamp.eq` and `quire.op.timestamp.ne` are enumerated by both V2 schemas and catalogued as binary `boolean` comparisons with no law or mode, and both schemas admit `string`, `uuid` and `timestamp` as a literal `value_kind`. An application of each identity over two operands of its own family, with result `boolean`, no law, no mode and no member, is admitted. An operand of another family (`string` against `text`, `uuid` against `string`, `timestamp` against `uuid`) and an ordering or arithmetic identity over these families (`quire.op.integer.lt` and `quire.op.integer.add` over `uuid` operands) each refuse `ill_typed`/`operator-ineligible`. A `text_profile` law on a `string`, `uuid` or `timestamp` identity refuses `invalid_package`/`operation-law-mismatch`, a `text_profile` mode `invalid_package`/`operation-mode-mismatch`, and a `string` operand of `quire.op.text.eq` `invalid_package`/`operation-mode-type-mismatch`. A literal of `value_kind` `string`, `uuid` or `timestamp` whose `type` is not its own family refuses `ill_typed`/`operator-ineligible`, and a `string`, `uuid` or `timestamp` literal `value` in a non-canonical spelling, on a literal of the right family, refuses `invalid_package`/`invalid-value`. | Test |
| FR-322-AC-68 | A selected domain package whose document declares the event type `OrderPlaced` (`quire.meaning.model.event-type/v1`, with fields `order_id` and `occurred_at`) admits a self-typed `model`/`event_type` declaration node owned by `OrderPlaced`, with a `ModelOwner`. The node has family `record`, a `quire.op.record.project` over a value of its type resolves a field of `OrderPlaced` (own or inherited) to its step 4 type, and an invariant clause function owned by `OrderPlaced` passes the owner join. A `quire.op.structural.eq` or `quire.op.structural.ne` over two `OrderPlaced` operands, or over two `Option<OrderPlaced>`, tuple-holding-`OrderPlaced` or `Set<OrderPlaced>` operands, refuses `ill_typed`/`operator-ineligible`; the same applications over `Money` operands are admitted. A `model`/`event_type` node owned by a record value type, an object type or a node absent from the selected document refuses `missing_declaration`/`missing-selection` in the owner join, one with no `owner` or with a `SourceOwner` and no `declaration` fails the V2 schema (`malformed_wire`), and both V2 schemas enumerate the form in the owner-presence rule and the model forms. | Test |
| FR-322-AC-69 | With record value types `Money` and `TaxedMoney` (supertype `Money`) and `GiftTaxedMoney` (supertype `TaxedMoney`) of one selected document, a `quire.op.function.call` argument (the parameter read from the callee's body `parameters` binding) or an operation-call argument of type `TaxedMoney` or `GiftTaxedMoney` at a `Money` parameter is admitted with the argument taken by `ref` and no conversion node; a `Money` argument at a `TaxedMoney` parameter, an argument of an unrelated record, and a `Sequence<TaxedMoney>` argument at a `Sequence<Money>` parameter each refuse `ill_typed`/`operator-ineligible`, as does an argument whose count differs from the parameters' (a callee with no `parameters` binding takes none). A `function` node typed `Money` whose `body` binding references a root expression node typed `TaxedMoney`, and a `case` whose arm bodies are typed `Money` and `TaxedMoney` with result type `Money` (FR-440-AC-4), are admitted; a `Money` body root at a `TaxedMoney` function, and an unrelated record (`Voucher`) body root at a `Money` function, each refuse `ill_typed`/`type-mismatch` at the function node. A `quire.op.record.project` over a `GiftTaxedMoney` or `TaxedMoney` operand whose `field` member names `Money` as `declaration` reads `Money`'s field with the type `Money` gives it, and one naming `GiftTaxedMoney` over a `TaxedMoney` operand refuses `ill_typed`/`operator-ineligible`. `quire.op.collection.contains` over a `Set<Money>` and a `TaxedMoney` operand, and `quire.op.collection.set` over `TaxedMoney` rest arguments with a `Set<Money>` result, are admitted, and a `Set<TaxedMoney>` operand of a `Set<Money>` slot refuses. A `value`/`union_value` node whose `Money` payload position holds a `TaxedMoney` term is admitted. A `quire.op.structural.eq` or `quire.op.structural.ne` over two `Money` operands is admitted with `Money`'s leaves, including when the values are `TaxedMoney` values whose subtype-only text field has no listed leaf. | Test |
| FR-322-AC-70 | A `function` node whose body is an `aggregate` with the first binding `parameters`, an `aggregate` of `reference` terms to `value`/`parameter` nodes in parameter order, (QSL FR-092 builds it, including a parameter the body never reads). A `function.call` application over that callee has arguments counted against that binding and typed against the `semantic_type` of its referenced parameter nodes; a callee without the binding takes none. | Test |
| FR-322-AC-71 | A selected domain package that declares the variant type `Tier` (cases `Gold`, `Standard`) and the object `Account` with field `tier: Tier` admits a self-typed `model`/`variant_type` declaration node owned by `Tier`, with a `ModelOwner`, family `enum`. `Account.tier` derives the `Tier` node as its type, an `optional` field `Option<Tier>` and an ordered non-unique `[0, 3]` field `Sequence<Tier>[0, 3]`, never `Reference<Tier>`. `quire.op.enum.eq` and `quire.op.enum.ne` over two `Tier` operands are admitted; `quire.op.structural.eq` over them, and an ordering application (`quire.op.enum.lt`, `le`, `gt` or `ge`), refuse `ill_typed`/`operator-ineligible`. An `enum_value` node whose `semantic_type` is the `Tier` node and whose case is `Gold` or `Standard` is admitted, and one whose case is not a case of `Tier` in the selected document refuses `invalid_semantic_graph`. The `node_id` is a function of the selected document and the node alone: two units that select the package under different model aliases derive one `Tier` node and one node for each case. A `model`/`variant_type` node owned by a record value type, an object type or a node absent from the selected document refuses `missing_declaration`/`missing-selection` in the owner join, and one with no `owner` or with a `SourceOwner` and no `declaration` fails the V2 schema (`malformed_wire`); both V2 schemas enumerate the form in the owner-presence rule and the model forms. | Test (TC-940) |
| FR-322-AC-72 | A selected domain package that declares the process `Flow` (`quire.meaning.model.process/v1`, field `n`, operation `go`) and the state machine `Gate` (`quire.meaning.model.state-machine/v1`, field `open`, operation `toggle`) admits a self-typed `model`/`process` declaration node owned by `Flow` and a self-typed `model`/`state_machine` declaration node owned by `Gate`, each with a `ModelOwner`, family `object`; `Flow.n` and `Flow.go` resolve through step 3 as an object type's members do, and `Reference<Flow>` and `Reference<Gate>` derive `composite_type`/`reference` nodes over the two declaration nodes. A `model`/`persistence_interface` node owned by a declared persistence interface passes the owner join. A process node owned by an object type, a state machine owned by a process, or either owned by a node absent from the selected document refuses `missing_declaration`/`missing-selection` in the owner join, and either carrying no `owner` fails the V2 schema (`malformed_wire`); both V2 schemas enumerate the three forms in the owner-presence rule. | Test (TC-233, TC-280) |
| FR-322-AC-73 | A `quire.op.model.reaches` application whose `relationship_end` member carries `population`, the id of a `bounded_domain`/`model_population` node over `Set<Reference<Order>>` where `Order` is the type of the end the member's role names, validates against both V2 schemas, lists the population node in its `dependencies`, and is admitted by the full reader, for a `max` of `3` and of `0`. `population` naming a `bounded_domain`/`integer_range` node, a node absent from the graph, or a `model_population` node over another model declaration refuses `ill_typed`/`operator-ineligible`; a `dependencies` list without the population node is refused `invalid_semantic_graph`; the published `adverse.json` `reaches_population_cases` record each. | Test |
| FR-322-AC-74 | A `bounded_domain`/`model_population` node over `Set<Reference<Order>>` whose body is the empty `aggregate`, with the `max` member absent, is the unbounded population: it validates against the V2 schema and is admitted by the full reader, as the `population` of a `quire.op.model.reaches` application, as a node distinct from the same node with `max` `3`; the node with `max` `3` stays admitted as a bounded population. A `max` of `-1`, a `max` that is not an integer, and a second member beside `max` each refuse `invalid_package`/`invalid-derived-shape` at the population node, and the unbounded node is distinct from the same node with `max` `0`. The published `adverse.json` `unbounded_population_cases` record each. | Test |
| FR-322-AC-75 | A diamond with two direct dependencies whose recursively derived closures reach one library identity as structurally unequal packages refuses `invalid_package`/`conflicting-definition`, with both importer-rooted selection-path JSON pointers as loci. A `dependency_reference` naming a transitive-only package refuses `missing_declaration`/`missing-selection` even when that package is in the derived closure. | Test |
| FR-322-AC-76 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-77 | The lock of a package whose unit has a non-empty refinement internal list carries `refinement_internals` in canonical order and the member is absent when every list is empty; two compilations of the same sources, one with a non-empty list and one without, recompile to structurally unequal packages through the refinement nodes' `abstract_internal` alone. A member that is `[]` or `null`, an object with an empty `internal` or a third key fails the schema at that value; objects or spellings out of byte order refuse `invalid_package`/`invalid-value` at the later entry's pointer; a repeated `refinement` or spelling refuses `invalid_package`/`conflicting-definition` with both entries as loci. | Test (TC-233) |
| FR-322-AC-78 | The cause `unsupported-wire` is paired only with `unknown_wire` and `unsupported-feature` only with `unknown_required_feature`, in the FR-272 rows and the `Diagnostic` schema alike, and the FR-272 rows of `unknown_contract_version` and `unknown_required_capability` list no cause. | Test (TC-233) |
| FR-322-AC-79 | A `literal` of `value_kind` `quantity` typed by a `scalar_type`/`unit` node is admitted with family `quantity`, both V2 schemas enumerate `quantity` as a literal `value_kind`, a quantity literal typed by another family and an integer literal typed by a unit node each refuse `ill_typed`/`operator-ineligible`, and a magnitude not in the canonical spelling refuses `invalid_package`/`invalid-value`. | Test (TC-233) |
| FR-322-AC-80 | A field, operation parameter or operation result `typeRef` of the form `ix://quire/unit/<name>` has the type of the named unit of the checked package, under the multiplicity and presence tables; a name that matches no unit refuses `missing_declaration`/`missing-name`, and an argument of another unit or family refuses `ill_typed`/`operator-ineligible`. | Test (TC-280) |
| FR-322-AC-81 | A unit, a dimension and a record that carry one `declaration` name are admitted, and two units, or two types, that carry one name refuse `ambiguous_declaration`/`ambiguous-name` at the later node in node-id order. A unit, a dimension and a record of one admitted domain package that share a name are admitted with distinct `node_id`s, the paths of the model-owned unit and dimension being `[node_tag, semantic_form, <FCD identity>]`. | Test (TC-233) |
| FR-322-AC-82 | A selected domain package that declares the populations `counters` and `archive` (`quire.meaning.model.population/v1`) admits a self-typed `relation`/`population` declaration node owned by each, with a `ModelOwner`, an empty `aggregate` body; the two `node_id`s differ, and a changed `owner.identity` changes the node's `package`. The owner join admits the node when the document declares a population whose `identity` equals `owner.node` and refuses `missing_declaration`/`missing-selection` for a population owned by an object type, by an undeclared node, and for a `model`/`object_type` or `relation`/`relationship` node owned by a population. A `relation`/`population` node with no `owner` fails the V2 schema (`malformed_wire`) Both V2 schemas enumerate `population` in the owner-presence rule. A selected document whose `populations` entry lacks an `identity` string, or carries a `node` member, refuses `invalid_model_binding`/`malformed-declaration` at `package_read`, at the owner of the package's `model_root` node, before any graph check, and no population of it is joined; each condition refuses alone (`population-entry-without-an-identity-refused`, `population-entry-with-a-node-member-refused` and `population-entry-lacking-only-an-identity-refused`). The published `adverse.json` `selected_document_populations` holds the `counters` and `archive` entries. | Test (TC-233, TC-280) |
| FR-322-AC-83 | A `quire.op.model.all_instances` and a `quire.op.model.lookup` application whose operand 0 references a population reference are admitted by the full reader, list the node in their `dependencies` and carry no maximum or member type of the population; the reads of `counters` and of `archive` have different population nodes and application nodes, and one population read by both operations is one node. A population reference as the `collection_kind` operand of `quire.op.collection.contains` and as the declaring node and as the `population` link of a `quire.op.model.reaches` `relationship_end` member each refuse `ill_typed`/`operator-ineligible`. The published operation fixture's operand 0 is a `value`/`parameter` binding typed by `Population<Order>[2]`, and the full reader admits it. A population reference as the declaring node of a `quire.op.record.project` `field` member and of a `quire.op.model.dispatch_call` `operation` member each refuse `ill_typed`/`operator-ineligible`. The published `adverse.json` `population_reference_cases` record each case. | Test (TC-233) |
| FR-322-AC-84 | Supply of `dependency_selections` is checked entry by entry in lock order, and the first failing entry is reported alone: two unsupplied entries refuse `missing_import`/`missing-selection` at entry 0; entry 0 supplied and entry 1 unsupplied refuse at entry 1; entry 0 supplied and entry 1 unsupplied refuse at entry 1 alone, never at entry 0. | Test (TC-233) |
| FR-322-AC-85 | A `dependency_reference` refusal's locus is the term's `package` member for `missing-selection`, its `node` member for `missing-name` and the term itself for `operator-ineligible`, each under the pointer of the term in its node's `body`. | Test (TC-233) |
| FR-322-AC-86 | The signature type nodes of a dependency function are the `semantic_type` of each node its `parameters` binding references and the function's own `semantic_type`: a dependency function with a parameter of a declared record type is ineligible (`ill_typed`/`operator-ineligible`), while one whose body only references a declared record is eligible. | Inspection (TC-233) |
| FR-322-AC-87 | A refinement owner with step rows `R0`, `R1` (in `steps` declaration order) from which the compiler derives one node per row has, for the node derived from `R1`, an occurrence with role `generated`, ordinal 1 and a `source_map` region equal to the region of `R1`'s source row; swapping the source order of the two rows swaps their `steps` positions, and so their ordinals. | Test (TC-233) |
| FR-322-AC-88 | A proof-eligible refinement owner with step rows `R0` and `R1` in `steps` order, whose source regions are `[10,20)` and `[30,40)` and whose `declaration` region is `[0,50)`, is admitted when its `source_map` holds a `declaration` occurrence at ordinal 0 and `generated` occurrences on its own node at ordinal 0 with region `[0,50)`, at ordinal 1 with region `[10,20)` and at ordinal 2 with region `[30,40)`, and entries of other nodes do not affect it (`RS-01`). | Test (TC-233) |
| FR-322-AC-89 | With the same two rows written in the other source order (`RS-02`), `steps` lists them in that order, so ordinal 1 holds the region of the row now written first and ordinal 2 that of the row written second. | Test (TC-233) |
| FR-322-AC-90 | A proof-eligible refinement owner with two step rows refuses `invalid_package`/`invalid-value` at the offending `source_map` entry for a `generated` occurrence at ordinal 3, for a second entry with the key of an existing occurrence, for a `generated` occurrence at ordinal 1 that carries `R1`'s region, for an `expression` occurrence at ordinal 1 and for a `declaration` occurrence at ordinal 1; and at `/source_map` for a package that omits the `declaration` occurrence, the `generated` ordinal 0 or the `generated` ordinal 2. | Test (TC-233) |
| FR-322-AC-91 | A refinement owner that is not proof-eligible, with a history field, with an `any` step target or with an abstract field that has no field row, is admitted with its `declaration` occurrence alone and refuses `invalid_package`/`invalid-value` at the first `generated` entry on its node when one is present (`RS-12` to `RS-15`). The hidden field is derived from the abstract model's fields against the `objects` entries' field rows, whatever their order (`RS-14`, `RS-16`, `RS-17`). | Test (TC-233) |
| FR-322-AC-92 | In every published producer-output package (every published positive package except the reader-admission fixture `positive-integer-literal-references.json` of FR-322-AC-93), each operand of a `quire.op.integer.add`, `sub`, `mul`, `negate`, `eq` or `ne` application that is an integer literal typed at the builtin Integer node is an inline `literal` term with `value_kind` `integer` and that type, and no such operand is a `reference` to a `value`/`literal` node of `value_kind` `integer`, whatever the node's type. In `positive-two-owners-a.json` and `-b.json` the `three` application holds the inline operands `1` and `2` and has empty `dependencies`; in `positive-operation-identities.json` the `integer.mul` and `integer.add` hold `7` and `-2` inline, and the `value`/`literal` nodes `7` and `-2` are present for the nodes that reference them. | Test (TC-233) |
| FR-322-AC-93 | The reader admits the reference form of an integer literal operand under the six operators: `positive-integer-literal-references.json`, whose `three` application is `quire.op.integer.add` over references to two `value`/`literal` nodes, validates against the V2 schema and passes every reader check, and its `three` node has a different `node_id` from that of `positive-two-owners-a.json`, which holds the inline form. | Test (TC-233) |
| FR-322-AC-94 | `quire.op.reference.upcast` admits an operand `Reference<S>` with `type_argument` `Reference<T>` exactly when `S` and `T` are selected object types of one document and `S` conforms to `T` along declared supertypes: one type to itself, a subtype to its supertype, a subtype two levels down to its root, and a type with two supertypes to either. A supertype to its subtype (a downcast), two unrelated object types, a systems interface with no declared supertype to an object type, a reference to a relationship at either end and an operand of family `object` refuse `ill_typed`/`operator-ineligible`. An operand or member whose owner is not selected refuses `missing_declaration`/`missing-selection`. | Test |
| FR-322-AC-95 | The full reader admits each published positive fixture whose graph holds no `recursion_group` (thirteen fixtures, `positive-dependency-call.json` read with `positive-dependency-library.json` supplied; `positive-timed-clause-nodes.json` and `positive-timed-slot-nodes.json` are excluded because their clock-clause and slot nodes are admitted by the timed-model checks of TC-367, not by the shared reader this criterion runs), both as published, where all but `positive-nominal-identities.json` list their nodes out of ascending node-id order, and with its `semantic_graph.nodes` reversed and with them sorted ascending by node-id order, each index segment of the document rewritten to its node's new graph index. A reversed graph names later nodes through its index segments, and it is admitted. Each reorder that changes the node sequence yields a structurally unequal package whose nodes have the same structural keys, and the reader reports no refusal for the order of the nodes. | Test (TC-233) |
| FR-322-AC-96 | Every node of every published positive fixture carries exactly the `dependencies` list that FR-322 `dependencies` rules 1 to 4 rebuild from the node as written, or, for a node form whose body artifact states its own list, that list: in `positive-all-families.json` the `expression`/`reference` node and the `correspondence`/`source_locus` node each list their body target, and the `bounded_domain`/`integer_range` node lists its `semantic_type`; in `positive-operation-identities.json` the `bounded_domain`/`model_population` node lists its `semantic_type` alone, not its `max` literal's `type`. The full reader refuses `invalid_semantic_graph` at the node's `/dependencies` for the `expression`/`reference` node with `[]`, the `integer_range` node with `[]`, the `model_population` node with the Integer node added, and a `value`/`parameter` node listing its `semantic_type`. | Test (TC-233) |
| FR-322-AC-97 | The V2 schema admits a `value`/`parameter` node whose body binds `name` then `level`, and a `scalar_type`/`compound_unit` node; a parameter body with `level` before `name`, or with a third member, fails the V2 schema (`malformed_wire`). The full reader admits every published positive fixture, and each `value`/`parameter` node of one is distinct by `name` and `level`. | Test (TC-233) |
| FR-322-AC-98 | Step 1 over a selected document that carries `dimensions` and `units` (FR-154-AC-19's `Time`, `Rate`, `s` and `ms`) and an object type with a clock field reads every dimension and unit node and the clock field into the declarations steps 2 to 4 read, in authored order; the same document with `dimensions` and `units` absent reads as one with both lists empty; a selected document whose `units` holds a unit with `scale` `0` refuses at stage `package_read` with FR-154's `invalid_model_binding`/`malformed-declaration` and its loci. | Test (TC-280) |
| FR-322-AC-99 | With the unit of compilation declaring `dimension Time; unit s : Time = 1; unit ms : Time = 1/1000 * s;` and the admitted domain package `acme/rpc` declaring base dimension `Rate`, root unit `us` of `Rate` and its own unit `ms` of `Rate` (`target` `us`, `scale` `1000`), a field typed `ix://quire/unit/ms` has type `Quantity<ms>` over the source unit's node and a field typed `ix://quire/unit/acme/rpc/ms` has type `Quantity<M::ms>` (with `model M = "acme/rpc"`) over the package unit's node, two distinct `scalar_type`/`unit` nodes with no `ambiguous_declaration`; the package unit node carries no `declaration`, its `owner` is `{kind: "model", identity: "acme/rpc", node: "ix://acme/rpc/unit/ms"}`, its `node_id.path` is `["scalar_type", "unit", "ix://acme/rpc/unit/ms"]` and its `node_id` is distinct from the source `ms` node's, and it depends exactly on the package `Rate` node and the package `us` node; a field typed `ix://quire/unit/rpc/ms` refuses `missing_declaration`/`missing-name`. A field typed `ix://quire/native/Clock` resolves as a `field` member to the clock type with no checked-package node; a `field` application over it in a value position refuses `invalid_timed_model`/`clock-read-outside-constraint`, while the same clock as the operand of an atomic clock constraint in a guard or time invariant raises no such refusal. | Test (TC-280) |
| FR-322-AC-100 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-101 | A `literal` of `value_kind` `reference` typed by the `composite_type`/`reference` node over a `model`/`object_type` node, with `value` `7331`, is admitted with family `reference`, and both V2 schemas enumerate `reference` as a literal `value_kind`. The same literal with `value` `7G31`, `733`, `73A1` or empty text refuses `invalid_package`/`invalid-value` at `value`; typed by the reference node over a `model`/`process` node, or by an integer node, it refuses `ill_typed`/`operator-ineligible`. | Test (TC-965) |
| FR-322-AC-114 | A `rational` literal typed by a `scalar_type`/`rational` node with value `0`, `1`, `-3`, `1/2` or `-7/2` is admitted with family `rational`, and one with value `2/2`, `1/1`, `02`, `+1`, `2/4`, `1/0`, `0/5`, `1/-2`, `2.5` or empty text refuses `invalid_package`/`invalid-value`. | Test (TC-233) |
| FR-322-AC-102 | A field typed `ix://quire/native/Bytes` has the type `Bytes` over a `scalar_type`/`bytes` node, a field typed `ix://quire/native/JsonObject` has the type `JsonObject` over a `scalar_type`/`json_object` node, and a field with a `maxLength` constraint typed `Bytes` has the type `Bytes`; with multiplicity `[0, unbounded]` and unordered unique they give `Set<Bytes>`. Two fields typed `Bytes` name one node. `Bytes` and `JsonObject` are equality-stable types, and a `Bytes` is distinct from a `String` and from a `Text`. | Test (TC-280) |
| FR-322-AC-103 | An application of `quire.op.bytes.eq`, `quire.op.bytes.ne`, `quire.op.json_object.eq` or `quire.op.json_object.ne` over two operands of its own family has result `boolean` with no law, mode or member; `quire.op.bytes.length` over one `bytes` operand has result `integer`; `quire.op.bytes.concat` over two `bytes` operands has result `bytes`. A `bytes` against a `string`, `text` or `json_object` operand, `quire.op.bytes.concat` over a `json_object` operand, an ordering identity (`quire.op.integer.lt`) over `bytes`, and any `quire.op.integer.add` or member-access identity over `json_object` each refuse `ill_typed`/`operator-ineligible`. A literal of `value_kind` `bytes` with `value` `""`, `"00ff"` or `"deadbeef"` is admitted, and with `value` `"00FF"`, `"0"`, `"0x00"` or `"00 ff"` refuses `invalid_package`/`invalid-value`; a literal of `value_kind` `json_object` with `value` `"{}"`, `"[]"`, `"3"`, `"null"` and `"{\"a\":1,\"b\":[true,null]}"` is admitted, and with `value` `"{\"b\":1,\"a\":1}"`, `"{\"a\": 1}"`, `"{\"a\":1.0}"`, `"{\"a\":9007199254740993}"`, `""`, `"{"` or `"{\"a\":1,\"a\":2}"` refuses `invalid_package`/`invalid-value`. A `bytes` or `json_object` literal whose `type` is not its own family refuses `ill_typed`/`operator-ineligible` even when its `value` is also non-canonical. Both V2 schemas list `quire.op.bytes.eq`, `.ne`, `.length`, `.concat`, `quire.op.json_object.eq` and `.ne` in `OperationIdentity`, and a package applying each validates against either schema. | Test (TC-280) |
| FR-322-AC-104 | With the unit of compilation declaring `dimension Time; unit s : Time = 1; unit ms : Time = 1/1000 * s;` and no selected package declaring `s`, a field typed `ix://quire/native/Duration` has the type `Quantity<s>` over the node of `s`, the same node as a field typed `ix://quire/unit/s`, and `Duration` against `Quantity<s>` is one type; a `Duration` against a `Quantity<ms>` compares only after FR-142's explicit conversion, and `1500 ms` converted to `s` equals the `Duration` `3/2`. A `Duration` field of an object type of an admitted domain package `acme/rpc` that declares base dimension `Time` and root unit `s`, with a unit of compilation declaring no `s`, has the type `Quantity<s>` over that package unit's node. With neither declaring `s` the field refuses `missing_declaration`/`missing-name`; with `unit s : Length = 1` and no dimension `Time`, or with `s` typed by a derived dimension named `Time`, the member has no type and the application refuses `ill_typed`/`operator-ineligible`. | Test (TC-280) |
| FR-322-AC-105 | With the unit of compilation declaring `dimension Time; unit s : Time = 1; dimension Speed = R::Time ^ -1; unit tenth : R::Time = 1/10 * R::s;` and the admitted domain packages `acme/rpc` (alias `R`) and `acme/flow` (alias `F`) each declaring base dimension `Time` and root unit `s` of it, the unit graph holds three `scalar_type`/`dimension` nodes named `Time` with pairwise distinct `node_id`s: the source node under a `SourceOwner`, and two package nodes with no `declaration`, `owner` `{kind: "model", identity: "acme/rpc", node: "ix://acme/rpc/dimension/Time"}` and `{kind: "model", identity: "acme/flow", node: "ix://acme/flow/dimension/Time"}`, each `node_id.path` `["scalar_type", "dimension", <its FCD identity>]`, with no `ambiguous_declaration` among them; each package root `s` depends on exactly its own package `Time` node; the source `Speed` node is self-typed and its `terms` hold the `acme/rpc` `Time` node id with exponent `-1` and depend on exactly that node; `tenth` has `dimension_node_id` the `acme/rpc` `Time` node id and `target_unit_node_id` the `acme/rpc` `s` node id; the strict reader admits the package. | Test (TC-280) |
| FR-322-AC-106 | A selected document whose object type `Call` has a clock field `x`, a record value type `Timer` with a clock field `x` and a data field `n`, a field `t` typed `Timer`, an optional clock field `w`, an operation `reset` with a parameter `c` typed `ix://quire/native/Clock` and an operation `clk` with a result typed `ix://quire/native/Clock` reads each as a clock member; the `Timer` `record_value_type` node lists `n` only, no node stands for `c`, the result of `clk`, `x`, `t.x` or `w`, and `w` is `Option<Clock>`. A `field` application that resolves to `x`, `t.x` or `w` in a value position refuses `invalid_timed_model`/`clock-read-outside-constraint`. | Test (TC-280) |
| FR-322-AC-107 | With the unit `ms` of `acme/rpc`, an alias `D` whose `target` is `ix://quire/unit/acme/rpc/ms`, a sequence type whose `items` is that reference, a map type whose `values` is that reference, and a field typed `D` have the types `Quantity<M::ms>`, `Sequence<Quantity<M::ms>>`, `Map<Text, Quantity<M::ms>>` and `Quantity<M::ms>`, over the one `scalar_type`/`unit` node of `ms`, and introduce no node of their own. | Test (TC-280) |
| FR-322-AC-108 | A literal of `value_kind` `float32_bits` typed by a `float32` type with `value` `3f000000`, `00000000`, `80000000` or `7fc00000` and a literal of `value_kind` `float64_bits` typed by a `float64` type with `value` `3fe0000000000000`, `0000000000000000`, `8000000000000000` or `7ff8000000000001` is admitted; a `float32_bits` literal with `value` `3F000000`, `0x3f000000`, `3f00000`, `3f0000000`, `0.5` or the JSON number `1056964608`, and a `float64_bits` literal with `value` `3FE0000000000000`, `0x3fe0000000000000`, `3fe000000000000`, `3fe00000000000000`, `0.5` or the JSON number `4602678819172646912`, each refuses `invalid_package`/`invalid-value` at the literal's `value`; a `float32_bits` literal typed by a `float64` type and a `float64_bits` literal typed by a `string` type refuse `ill_typed`/`operator-ineligible` even when the `value` is also non-canonical. The `value` of every `float32_bits` and `float64_bits` literal in `positive-operation-identities.json` is in this spelling. | Test (TC-233) |
| FR-322-AC-109 | A selected document whose object type `Call` has a field `s` typed `ix://quire/native/Clock` with multiplicity `{0, 3, true, false}`, a field `b` typed `Clock` with `{0, 3, false, false}`, a record value type `Timer` with a clock field `x` and a data field `n`, a field `ts` typed `Timer` with `{0, 3, true, false}`, a sequence type `CS` whose `items` is `Clock` and a map type `CM` whose `values` is `Clock` reads `s`, `b`, `CS` and `CM` as clock collections with no node, and `ts` as the `bounded_domain`/`collection_bounds` node with bounds `0` and `3` over the `composite_type` node of form `sequence` over the `Timer` node that lists `n` only; a `field` application over `s`, `b` or `ts` in a value position refuses `invalid_timed_model`/`clock-read-outside-constraint`, and a clock constraint over `ts`'s elements raises no such refusal. | Test (TC-280) |
| FR-322-AC-110 | The `Timer` data part node of AC-109 is the same node whether `Timer` is held by a field of multiplicity `{1, 1}`, `{0, 3, true, false}` or `{0, 3, false, true}`; a clock collection written in two models with equal element types, bounds and kinds has equal types, and a model that changes only a clock collection's declared maximum yields a structurally unequal package. | Test (TC-280) |
| FR-322-AC-116 | With the unit of compilation declaring `dimension Time; unit s : Time = 1;` and the admitted domain package `acme/rpc` declaring base dimension `Time` and root unit `s`, a `Duration` field of an object type of `acme/rpc` has the type `Quantity<s>` over the package unit's node (`ix://quire/unit/acme/rpc/s`), never ambiguous with the source `s`, and a `Duration` field declared by the unit of compilation has the type `Quantity<s>` over the source unit's node; the two types are different (FR-142), so comparing them without conversion refuses `ill_typed`. A package declaring base dimension `Time` and units `s` (root) and `min` (scale 60 to `s`) gives `Duration` the root `s`; a package declaring `Time` with root `sec` and `s` of scale 60 to `sec` (a minute) refuses the `Duration` field `ill_typed`/`operator-ineligible`, as does `s` with scale 1 and offset 5 to its root; `s` that reaches the root through `t` with scale 60 from `s` to `t` and 1/60 from `t` to the root (composed scale 1) is admitted. A package that declares `s` while the unit of compilation declares its own base dimension `Time` and `s` gives the package member the package's own `s` of the package's own `Time`; the source `Time` is never the package member's `Time`. A package declaring no `s` with the unit of compilation declaring `s` gives its `Duration` member the source `s`. | Test (TC-280) |
| FR-322-AC-111 | `quire.op.ieee.float64.negate` over one `float64` operand typed `Float64[nearest-even]` has `result_type` the operand's type node, the law `ieee_profile` selecting the lock's IEEE definition, mode `null`, no member and no leaves; the same over `float32` with `quire.op.ieee.float32.negate`. A `float32` operand of the `float64` identity, a `float64` operand of the `float32` identity, two operands, a `result_type` of another type node (`Float64[toward-zero]` over a `Float64[nearest-even]` operand), an `integer` or `decimal` operand, a `rounding` mode on the node and a missing `ieee_profile` law refuse as the reader order states (`operator-ineligible` for the operand, arity and result, `operation-mode-mismatch` for the mode, `operation-law-missing` for the law); a lock that selects no `ieee_profile` refuses `operation-law-unselected`. The identities `quire.op.ieee.float64.eq`, `quire.op.ieee.float64.ne` and `quire.op.ieee.float64.lt` refuse `unknown-operation`. | Test (TC-233) |
| FR-322-AC-112 | `quire.op.structural.eq` and `.ne` over `record R { x: Float64[nearest-even]; t: Text[0,8;nfc]; }` are admitted with the leaves `["field:x"]` (laws `[ieee_profile]`, mode `null`) and `["field:t"]` (the `nfc` law and mode) in field order; over `Option<Float64[nearest-even]>` with the leaf `["inner"]`; over `union U { V(Float32[exact]), N }` with `["member:V", "position:0"]`; over `Sequence<Float64[nearest-even]>` with `["inner"]`; over a record holding no float and no text, none. A `quire.op.collection.set` whose result is `Set<Float64[nearest-even]>` lists the leaf with no path segment under `result_inner`. A float leaf missing from `leaves`, one listing no law, one listing the text profile law or a mode, and a lock without the `ieee_profile` selection refuse as a text leaf does. The occurrences `quire.op.ieee.numeric_equal` over two `float64` operands, and `quire.op.boolean.not` over it, are admitted as the lowering of `=` and `!=`. | Test (TC-233) |
| FR-322-AC-113 | A literal of `value_kind` `decimal` typed by `Decimal[0, 1000; 0, 2; nearest-even]` with `value` `"1.25"`, `"0.05"`, `"10"`, `"0.00"`, `"1000"`, `"0.5"`, `"1000.00"` and `"1.250"` is admitted because each is a member by value, and with `value` `".5"`, `"1."`, `"+1.5"`, `"01.5"`, `"-0.0"`, `"1e2"`, `"1/4"` and `""` (not canonical spellings), and `"1.255"` (membership scale 3 above the scale bound), `"1001"`, `"1000.01"` (membership coefficient above the bound) and `"-1"` refuses `invalid_package`/`invalid-value`; `"1.0"` and `"1.00"` are two literals. Typed by `Decimal[-1000, 1000; 1, 2; nearest-even]`, `"-0.5"`, `"5"` (membership scale 1, coefficient 50) and `"100"` are admitted and `"1001"` refuses `invalid-value`. A `decimal` literal typed by an `integer` or `string` node refuses `ill_typed`/`operator-ineligible` even when its value is also malformed. | Test (TC-233) |
| FR-322-AC-117 | `quire.op.collection.sequence.drop_first` over a `Sequence<Integer>[0,9]` operand and `quire.op.collection.ordered_set.drop_first` over an `OrderedSet<Integer>[0,9]` operand are admitted with a `result_type` of the operand's family (`Sequence<Integer>[0,8]` and `OrderedSet<Integer>[0,8]`), no law, no mode, no member and no leaves; `quire.op.collection.remove_one` over `Bag<Text[0,8;nfc]>[0,9]` and a `Text[0,8;nfc]` value is admitted with the one leaf of the empty path (`inner:0` adds no `inner` segment) carrying the `nfc` law and mode as `contains` lists it, over `Set<Float64[nearest-even]>[0,9]` and a `Float64[nearest-even]` value with the empty-path float leaf carrying `ieee_profile` and mode `null`, and over `Sequence<Integer>[0,9]` with no leaf. A `drop_first` identity over a `Set` or `Bag` operand, a `drop_first` whose family differs from its operand's, a `remove_one` whose value is not the element type, one operand, or a `result_type` of another family, and a `drop_first` or `remove_one` listing a law, a mode or a member refuse as the reader order states (`operator-ineligible` for the operand family, arity, constraint and result type; `operation-law-mismatch`, `operation-mode-mismatch` and `operation-member-mismatch` for the rest). | Test (TC-233) |
| FR-322-AC-118 | Withdrawn: the dimension-equivalence vectors it checked are removed. | Withdrawn |
| FR-322-AC-119 | An `application` of `quire.op.quantity.negate` with one `quantity` operand typed by the unit node of `m` is admitted with result type the unit node of `m`, no laws, `mode` `null` and no member; a second operand or a non-`quantity` operand (an `integer`) refuses `ill_typed`/`operator-ineligible`, a `mode` value refuses `invalid_package`/`operation-mode-mismatch` and a `laws` entry refuses `invalid_package`/`operation-law-mismatch`; both V2 schemas enumerate `quire.op.quantity.negate` in `OperationIdentity` directly after `quire.op.quantity.sub`, and the catalog entry of `quire-verification-contracts` carries it. | Test (TC-187) |
| FR-322-AC-120 | A timed state model has one owned `timed_model` root and one `timed_model`-tagged node per source, clock place, reset, guard, invariant, urgency and clock constraint; their references resolve to correctly typed nodes, their required data enter each node's body, and the reader reconstructs the producer's checked timed model and normalized constraints. The semantic refusal vectors identify the first defect at these JSON pointers (`i` is the indexed graph node and `j` the indexed aggregate member): a missing root is `invalid_package`/`missing-timed-model` at `/semantic_graph/nodes`; a dangling or wrong-kind tick operation is `invalid_package`/`timed-node-reference` at `/semantic_graph/nodes/{i}/body/members/3/value/target`; a dangling clock-place reference is `invalid_package`/`timed-node-reference` at `/semantic_graph/nodes/{i}/body/members/3/value/members/{j}/target`; a second place node of one path (`TM-12`) can carry only a forged `resolved_path` row, so its row join, which follows the aggregate order check (FR-322-AC-260) and precedes the repeated-path determination (`duplicate-clock-place`), refuses it `invalid_package`/`malformed-clock-place` at the first differing row `/semantic_graph/nodes/{i}/body/members/2/value/members/{j}` (the repeated-path cause `duplicate-clock-place` is reachable only for a repeated path whose every row joins, which a conforming package cannot contain), while one place node listed twice (`TM-30`) repeats its order key and refuses `invalid_semantic_graph`, with no cause, at the repeated member (FR-322-AC-260); an unlisted `clock_guard` node is `invalid_package`/`timed-node-reference` at `/semantic_graph/nodes/{i}/body` (FR-322-AC-243); a noncanonical path order is `invalid_package`/`malformed-clock-place` at `/semantic_graph/nodes/{i}/body/members/1/value/members/{j}`; and a non-time unit or nonpositive period is `ill_typed`/`invalid-time-quantity` at `/semantic_graph/nodes/{i}/body/members/1/value` or `/semantic_graph/nodes/{i}/body/members/4/value/value`, respectively. Reader checks use ascending node-id order. | Test (TC-233, TC-367; semantic vectors TM-09..TM-14) |
| FR-322-AC-121 | The reader runs body steps in the order state-graph, temporal, abstraction, refinement, terminal, step-relation, hybrid, timed-model, protocol, schedulability, after state checks and before operation checks. In a schema-valid package with independent abstraction, refinement and terminal defects and no earlier defect, the abstraction step reports first; repairing only that defect reports refinement, and then repairing only the refinement defect reports terminal. When a schema-valid package has a defect in its timed-model graph and protocol graph but no earlier defect, the timed-model step reports first, when that timed defect is repaired the protocol step reports, and when a hybrid defect is added the hybrid step reports before the timed-model step. | Test (TC-233) |
| FR-322-AC-122 | A checked package containing each of the eight `timed_model` structural forms is admitted with each node's self `semantic_type`; removing, adding, reordering or changing the type/count of any required child is refused by the published wire schema. | Test (TC-367; `proposals/checked-package-v2/fixtures/positive-timed-model-nodes.json`, `timed-model-node-vectors.json`) |
| FR-322-AC-123 | The timed root's `time_dimension` reference and source unit's `dimension_node_id` name the same nominal node, and that node is the selected base dimension `Time`. A dimension-name lookup, dimension equivalence, derived dimension or declaration-order inference never substitutes a different node. A mismatched root/unit reference or a unit of another dimension refuses `ill_typed`/`invalid-time-quantity`; changing the explicit binding changes the timed root's body. | Test (TC-367; TM-06, TM-07) |
| FR-322-AC-124 | Every clock-place path step has exactly one matching `resolved_path` row; rows identify owner, member identity and declared type, and collection rows carry kind and bounds. The final row is the `Clock` terminal. The reader resolves rows against the selected model declaration without reopening source; an absent row, mismatched path, undeclared member, coherently forged member chain, wrong terminal type, or wrong collection bound refuses `invalid_package`/`malformed-clock-place` at the affected path step `/semantic_graph/nodes/{i}/body/members/1/value/members/{j}` or resolved-path row `/semantic_graph/nodes/{i}/body/members/2/value/members/{j}`. The positive `Order.clockMap["key"]` vector records the selected `Order` field as `Map<Text, Clock>`, its map-value slot, and terminal Clock. | Test (TC-367; TM-07, TM-15..TM-19, TM-22) |
| FR-322-AC-125 | A tick source's `operation` reference targets a `state`/`operation_anchor`; its anchor context equals both the tick source's `model` and the timed root's `model`, and its operation resolves as a declared operation of that model. A function node or an anchor of another model refuses `invalid_package`/`timed-node-reference` in the timed-model step. An unresolved name on a model-owned operation anchor is refused earlier by FR-342 as `missing_declaration`/`missing-name` at the operation member. A valid anchor advances time by the exact positive period per occurrence. | Test (TC-367; TM-08, TM-11, TM-20, TM-21) |
| FR-322-AC-131 | `ExecutionPoint` has exactly `initialization{name: AnchorName}`, `handler{name: AnchorName}`, `pre{operation: AnchorName}` and `post{operation: AnchorName}` variants; each rejects unknown tags, missing/extra members and empty names. The state-clause application and operation-anchor body carry it at the locations FR-341/FR-342 specify, so a point is part of that node's body; omitting it is distinct from every point. Both schemas agree. | Test (TC-233) |
| FR-322-AC-126 | I2 refuses a tick source that owns any clock place as `invalid_package`/`clock-without-dense-time` at the offending place node's `model` binding pointer `/semantic_graph/nodes/{place}/body/members/0/value/target`. A tick source with no places remains admissible; TM-23 starts from the positive tick fixture, adds the positive dense fixture's valid clock-place node to its root, and reports the appended place node's model pointer. | Test (TC-367; TM-23) |
| FR-322-AC-127 | The timed root names the exact base `Time` dimension node and the source unit's `dimension_node_id` equals that node by identity. I2 refuses a source unit in base `Length`, a derived non-Time dimension, or a nominal derived-Time dimension as `ill_typed`/`invalid-time-quantity` at `/semantic_graph/nodes/{source}/body/members/1/value/target`, for both dense and tick sources. Dimension equivalence, matching dimension terms, names and declaration order do not substitute for this identity equality. | Test (TC-367; TM-24..TM-29) |
| FR-322-AC-128 | A checked package with refinement steps carries one closed `refinement_step_rows` entry for every step of every refinement owner, keyed by that owner's retained node id and zero-based step position in declaration order; its regions are part of no node body. The reader admits the `RS-01` and `RS-02` row regions, each keyed by (owner, ordinal) for its own source order, and refuses an absent or incomplete sidecar at `/refinement_step_rows`, and a wrong owner, out-of-range ordinal or duplicate pair at the offending entry. | Test (TC-233) |
| FR-322-AC-129 | A proof-eligible refinement owner with source rows `R0` and `R1` is admitted only when generated occurrence ordinals 1 and 2 each carry exactly the region of the corresponding `refinement_step_rows` entry; changing either generated region or its row-region carrier alone leaves every node unchanged but refuses `invalid_package`/`invalid-value` at that `source_map` entry. A refinement that is not proof-eligible still carries every row-region entry and no generated occurrence. | Test (TC-233, TC-310) |
| FR-322-AC-130 | Both closed V2 schemas admit a refinement `class_catalog` with ordered concrete and abstract declared classes (local `NodeRef` versus local-or-dependency `RefinementKey`), typed binders and source-default memory classes, and refuse a missing or extra class-entry member or a wrong key shape. A `memory drain` entry carries `drain: store-buffer-flush`; the catalog includes `memory drain` for each of `sc`, `tso` and `ra` that occurs as a source default, and `memory flush` for `tso` when it occurs, without a `drain` member. In an admitted protocol refinement, removing a local class node or binder type from `dependencies` refuses in graph shape before class joins. A model-only refinement retains its nine-member body with no catalog. Every V2 `protocol_clause` node carries the identity-bound `owner_model` reference in its path and exact dependencies; other nodes omit it. | Test (TC-233, TC-310, TC-946) |
| FR-322-AC-132 | For each selected domain-package identity, a package contains exactly one self-typed `model`/`model_root` node with the closed `{kind: "model_root", identity}` owner, one `generated` occurrence at ordinal zero mapped to the existing model-selection source region; this form exists regardless of declaration count and its `node_id` is determined by the selected identity, never by a model alias or representative declaration. The V2 schema rejects a root without `ModelRootOwner`, a `ModelRootOwner` on another node form and an extra owner member; the join of the owner's identity to a supplied document is a reader rule, not a schema rule (FR-322-AC-135). The reader admits it without a new FCD declaration or selection field. | Test |
| FR-322-AC-133 | A refinement `class_catalog` is unique and ordered by catalog key (FR-375): a declared class by its alias-free `class` spelling, a memory class by (`class`, `memory_model`), compared by `class` bytes then `memory_model` bytes. Both closed V2 schemas admit three `memory drain` entries for `sc`, `tso` and `ra` (schemas cannot state order or uniqueness); the reader refuses a drain entry reordered from `ra`, `sc`, `tso`, or a repeated (`class`, `memory_model`) pair, as `invalid_semantic_graph` at the offending array entry in refinement step 2. A concrete or abstract declared entry `activate <protocol>` is admitted as an ordinary declared class (`node` and `binder_types`), with no schema change. | Test (TC-233, TC-310) |
| FR-322-AC-134 | Every published positive package that selects a domain package carries exactly one `model`/`model_root` node per selected identity, appended after its other nodes, equal to the node derived from the identity alone, with its `node_id`, owner, empty aggregate body, empty dependencies and self `semantic_type`, and one `source_map` entry for it (role `generated`, ordinal zero) appended after the other entries; packages with equal rows carry the same root node. | Test (TC-310) |
| FR-322-AC-135 | The V2 schemas reject a root without an owner, with an extra owner member and a `ModelRootOwner` on an object type, and admit a root whose owner names no supplied document. The reader refuses the first three `malformed_wire`, an owner naming no supplied document `missing_import`/`missing-selection` at `/semantic_graph/nodes/{i}/owner/identity` and a repeated root `invalid_semantic_graph` at the second node's `owner`. | Test (TC-310) |
| FR-322-AC-136 | Each root has one source-map entry, role `generated`, ordinal zero, with at least one non-empty region, each naming a `lock.sources` entry. The entry is the producer-asserted selection-to-source relation; no reader verifies that a region is the selecting occurrence and the published one-byte regions are placeholders. | Test (TC-310) |
| FR-322-AC-137 | Every published selected-model package contains the node and source-map entry a producer derives from each selected identity, last in graph order and last in the source map; a producer conformance run compares its emitted root for an identity with that node. | Test (TC-310) |
| FR-322-AC-138 | `model-root-vectors.json` publishes the controls `MR-00` (unmutated, admitted), `MR-01` to `MR-03` (schema-invalid, `malformed_wire`), `MR-04` (unjoined owner, one defect) and `MR-09` (repeated root) as RFC 6902 patches with schema validity and the refusal code, cause and pointer; the QSpec test runs them from the file. | Test (TC-310) |
| FR-322-AC-139 | Withdrawn: the dimension-equivalence wire vectors it checked are removed. | Withdrawn |
| FR-322-AC-140 | A selected domain package that declares the tuple value type `Pair` (positions `p0: Boolean`, `p1: Integer`) admits a self-typed `model`/`tuple_value_type` declaration node owned by `Pair`, with a `ModelOwner` whose `node` is `ix://acme/shapes/Pair`, no `declaration`, the empty aggregate body and empty `dependencies`; its family is `tuple`, and no position is a node, an owner or a `source_map` key. Both V2 schemas enumerate `tuple_value_type` in the owner-presence rule and the model forms. A `tuple_value_type` node with no `owner`, with a `SourceOwner` and no `declaration`, with a `declaration` beside a `ModelOwner` or with a `version` in its owner fails the V2 schema (`malformed_wire`), one with a non-empty body refuses `invalid_package`/`invalid-derived-shape` at `/semantic_graph/nodes/{n}/node_id` as any model declaration node does (`TW-27`), and one owned by a record value type, an object type, a variant type or a node the document does not declare refuses `missing_declaration`/`missing-selection` in the owner join. | Test (TC-940) |
| FR-322-AC-141 | `quire.op.tuple.project` over a value of the `Pair` node with the member `{kind: "position", declaration: Pair, position: 0}` has result type the `Boolean` node and with position `1` the `Integer` node; a position equal to or above the number of positions, and a `declaration` that is not the operand's tuple node, refuse `ill_typed`/`operator-ineligible`. A position whose `typeRef` names a tuple, a variant type or a record value type has that type's declaration node as its type (a tuple `Nest` with positions `Pair` and `Shape` projects the `Pair` node and the `Shape` node), a position is never typed `Option`, a collection or a `Reference`, and a `field` field typed `Pair` derives the `Pair` node, never a `composite_type`/`tuple` node. | Test (TC-940) |
| FR-322-AC-142 | The tuple `node_id` is a function of `owner` alone. Swapping the position types of `Pair` (positions `Integer`, `Boolean`) leaves the `node_id` of the `Pair` node unchanged and changes the selected document, and leaves the `model_root` node unchanged; two tuples with equal positions and different names have different `node_id`s; the same package selected under two model aliases derives one `Pair` node. The type identity of a tuple node is the pair of its `node_id` and its owner's `identity`. | Test (TC-940) |
| FR-322-AC-143 | A variant type `Shape` with the nullary case `Empty` and the payload cases `Flag(Boolean)` and `Both(Pair)` is admitted as the same self-typed `model`/`variant_type` declaration node and owner join as an enumeration, and its family is `union`: its members are `Both`, `Empty`, `Flag` in ascending UTF-8 byte order of case name regardless of the order of `variants`, with payload arities 1, 0, 1 and payload types the `Pair` node, none and the `Boolean` node. A `case` over `Shape` has the arms `Both`, `Empty`, `Flag`; the same arms in the order `Flag`, `Empty`, `Both` refuse `ill_typed`/`operator-ineligible`. A variant type none of whose cases carries a `payloadType` keeps family `enum`. A field, parameter, result, tuple position or payload typed `Shape` derives the `Shape` node. The `Shape` node's `node_id` is the same whether `Shape` declares only `Empty` or also `Flag`; the two documents differ. | Test (TC-940) |
| FR-322-AC-144 | The package fixture carries a `value`/`union_value` over the body-less `Shape` node for `Empty`, `Flag` and `Both`, a `union_value` over `List`, an `enum_value` over `Color`, two `quire.op.tuple.project` over `Pair`, a `case` over `Shape` and the equalities over `Shape`, `List` and `Color`, and the full reader enforces the rules below on them. A `value`/`union_value` node whose `semantic_type` is the `Shape` node is admitted with the case `Empty` and the empty payload `aggregate`, with `Flag` and one `Boolean` payload term, and with `Both` and one `Pair` payload term; the case `Flag` with an `Integer` payload term, with no payload term or with two, and a case `Missing`, refuse `ill_typed`/`type-mismatch`. A `union_value` over the enumeration `Color` refuses `ill_typed`/`type-mismatch`, and an `enum_value` over `Shape` naming `Empty` or `Flag` refuses `invalid_semantic_graph` at the node. `quire.op.structural.eq` over two `Shape` operands is admitted and `quire.op.enum.eq` refuses `ill_typed`/`operator-ineligible`, as does `quire.op.structural.eq` over two `Color` operands; a `case` over a `Shape` operand has one arm per member in the member order above, and an arm missing refuses `ill_typed`/`operator-ineligible`. A `quire.op.tuple.project` over `Pair` at position `2`, at the beyond-safe `KeyOrdinal` string `9007199254740993` or with the tuple `Swap` as its `declaration` refuses `ill_typed`/`operator-ineligible`, and one whose position is a string that is no `KeyOrdinal` fails the V2 schema. | Test (TC-940) |
| FR-322-AC-145 | A selected document with a tuple that has no position, a gap in its position ordinals, a position without `typeRef`, or a field, identity field, operation, supertype or clause refuses `invalid_model_binding`/`malformed-declaration` at stage `package_read`; a position `typeRef` naming no package type refuses `missing_declaration`/`missing-name`; two positions with one ordinal refuse `invalid_model_binding`/`conflicting-binding`; a variant case whose `payloadType` is unresolved or is `ix://quire/native/Clock` refuses `unsupported_construct`/`declaration-form`. Each is the first refusal in FR-154's report order, in the order "Intake refusal order" states, and precedes every graph refusal. | Test (TC-940) |
| FR-322-AC-146 | `model-tuple-union-vectors.json` publishes the declarations, the cases of FR-322-AC-141 to FR-322-AC-145, the recursion cases of FR-322-AC-147 and the wire cases of FR-322-AC-140 and FR-322-AC-144; and `fixtures/positive-model-tuple-union.json` is a schema-valid package with one `model_root` per selection and a tuple node and a payload-bearing variant node whose owners join the published document. The QSpec test runs all three from the files. | Test (TC-940) |
| FR-322-AC-147 | A selected document in which the tuple `Cell` (positions `Integer`, `List`) and the variant type `List` (cases `Nil`, `Cons(Cell)`) refer to one another is admitted by intake. The `Cell` and `List` nodes carry no `recursion_group`, each `node_id` is a function of its owner alone, and `List` has family `union` and `Cell` family `tuple`. The structural walk out of `List` has the one step `member:Cons`, `position:0` to `Cell`, and out of `Cell` the steps `position:0` to `Integer` and `position:1` to `List`; `quire.op.structural.eq` over two `List` operands, which re-enters `List` and reaches no text, has no leaf and is admitted with the empty `leaves`, and one that supplies a leaf refuses `invalid_package`/`operation-law-mismatch`. A `union_value` of `List` with the case `Cons` and a `Cell` payload term, and with `Nil` and none, is admitted. | Test (TC-940) |
| FR-322-AC-148 | The slot selector of a template step is symbolic. For `ts: Sequence<Timer>[0, 3]` with `Timer { x: Clock; n: Int }`, the place `ts.0.x` is the one `clock_place` whose path is `field(ts)`, `slot(sequence, <Integer type node>, 0, 3)`, `field(x)`; `Set`, `Bag` and `OrderedSet` differ only in the `collection` literal, the declared field (`c`, `b` and `o`: one model declares one field of each name once) and the resulting path, `Sequence<Sequence<Clock>>` carries one Integer-typed slot step for each level, and `Map<Text, Clock>` carries the Text type node (the published fixture place). A slot selector that is an integer literal, a reference to the Text type node under a `Sequence`, a reference to an expression node, or a reference to the Integer type node under a `Map<Text, _>` refuses `invalid_package`/`malformed-clock-place` at the path step `/semantic_graph/nodes/{i}/body/members/1/value/members/{j}`. | Test (TC-367; CK-26; SL-01..SL-06, SR-01..SR-04) |
| FR-322-AC-149 | One template node serves every instance. The places `ts.0.x`, `ts.1.x` and `ts.2.x` of AC-148 are instances of the one node of `SL-01`; no node carries an index and no node field depends on how many elements a state holds. Two `clock_place` nodes with one path can differ only in the derived `resolved_path`, and a row that differs from the selected declaration is forged (FR-322-AC-124), so the second node refuses `invalid_package`/`malformed-clock-place` at its first differing row, after the order check and before the repeated-path determination (this replaces the earlier `duplicate-clock-place` refusal of a second node, STD-559); `duplicate-clock-place` is kept for a repeated path whose every row joins, which a conforming package cannot contain. | Test (TC-367; CK-26; SL-01, SL-07, GD-02) |
| FR-322-AC-150 | A `clock_place` has no `guards` field (`SL-07` is exactly `model`, `path` and `resolved_path`), and an empty `guards` aggregate is admitted on a `timed_model` root (the published tick fixture) when the model has no guard. A `clock_guard` node that the root's `guards` does not list refuses `invalid_package`/`timed-node-reference` and is never read as absent (`GD-03`), and a second place node of one path refuses `invalid_package`/`malformed-clock-place` at its forged `resolved_path` row (`GD-02`); the vectors publish both as replayable package mutations, RFC 6902 patches (FR-322-AC-242), and `GD-03` refuses at the locus of FR-322-AC-243. | Test (TC-367; CK-26; GD-01..GD-03) |
| FR-322-AC-151 | The selected document of the published `acme/shapes` package is `domain-package-acme-shapes.json`, a full FCD semantic IR 2.0.0 document (`contractVersion`, `package.identity`, `constructs`, `types`) with `Holder` an object type, `Pair`, `Swap`, `Nest` and `Cell` tuple value types with ordered `positions` (the tuple construct uses FCD's `tuple` shape and each position carries its schema-required `origin`, per filament-core-data FR-147-AC-2 and FR-147-AC-3), and `Shape`, `Color` and `List` variant types with `variants`; `domain_package` in `model-tuple-union-vectors.json` is exactly the view of it that intake reads (types in document order, lists in declared order, multiplicity bounds as decimal strings, a tuple position's ordinal its index) and is not versioned. `selection_identity` is the identity of the fixture's `model_root` owner. Swapping the position types of `Pair` in the document, or reordering two declarations in its `types`, changes the document and leaves the `model_root` node unchanged. | Test (TC-940) |
| FR-322-AC-152 | A wire case of `model-tuple-union-vectors.json` whose `quire.op.tuple.project` application names a position that is not a position of the tuple, a `declaration` that is not the operand's tuple node, or a beyond-safe `KeyOrdinal` string refuses `ill_typed`/`operator-ineligible` at the locus of the application node, `/semantic_graph/nodes/<i>`, and not at `/semantic_graph/nodes/<i>/body/operation/member`, `/member/position` or `/member/declaration`; one whose position is no `KeyOrdinal` is `malformed_wire` at the same application node. | Test (TC-940) |
| FR-322-AC-153 | A wire case of `model-tuple-union-vectors.json` states a `cause` exactly when its refusal code carries one: a case refused `invalid_package`, `ill_typed`, `ambiguous_declaration`, `missing_declaration` or `invalid_model_binding` states its cause, and an admitted case and a case refused `malformed_wire` or `invalid_semantic_graph`, codes that carry no cause (FR-272), state none. | Test (TC-940) |
| FR-322-AC-154 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-200 | A `relation`/`refinement` node's closed `RefinementBody` requires `name`, an `Identifier`, in both V2 schemas; the node's body, and so its path, includes it. Two refinement nodes that differ only in `name` are two nodes; a body without `name` fails both schemas; two nodes of one package with an equal `name` refuse `ambiguous_declaration`/`ambiguous-name` at the later node's `body/name`. | Test (TC-310) |
| FR-322-AC-155 | `present(other.v)` over `v: Option<Clock>`, and the body `present(e)` of `forall(e in self.q: ...)` over `q: Sequence<Option<Clock>>[0, 2]` are each one `presence_read` application of `quire.op.option.present`, typed Boolean, with the arguments the place template, the owner term (`other`, `self`) and, for `q`, one slot term (the binder parameter `e`); the two nodes are distinct, the place nodes are the same templates for every owner, and no place, parameter or model node carries a `presence` member or an `Option<Boolean>`. | Test (TC-367; CK-27; PR-01, PR-03) |
| FR-322-AC-156 | A `presence_read` with no slot term over `q` refuses `ill_typed`/`operator-ineligible` at `/body/arguments`; one whose owner term is an Integer parameter refuses `ill_typed`/`type-mismatch` at `/body/arguments/1`; one whose first argument is no `clock_place` node refuses `ill_typed`/`operator-ineligible` at `/body/arguments/0`. | Test (TC-367; CK-27; PR-04..PR-06) |
| FR-322-AC-157 | The `clock_place` of `v: Option<Clock>` has the single path step `field(v)` and one `resolved_path` row whose member type is `Option<ix://quire/native/Clock>`, and the `Option` wrapper adds no step. The row joins the declared field `v` of the `Order` declaration by identity, declared presence `optional` and type `Clock`, and terminates at `Clock` once the wrapper is removed; the `object_type` node of `Order` is the same node with and without `v`, and its declared members are exactly the declared fields. The place of `q: Sequence<Option<Clock>>` has a `field` and a `slot` step whose row type is that `Option` type and is checked for its schema and symbolic selector only. | Test (TC-367; CK-27; PR-01..PR-03, model_join) |
| FR-322-AC-160 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-161 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-162 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-322-AC-163 | Withdrawn: the `refinement-row-node-vectors.json` file it checked is removed. | Withdrawn |
| FR-322-AC-164 | Withdrawn: the `refinement-row-node-vectors.json` file it checked is removed. | Withdrawn |
| FR-322-AC-165 | Withdrawn: the `refinement-row-node-vectors.json` file it checked is removed. | Withdrawn |
| FR-322-AC-166 | Withdrawn: the `refinement-row-node-vectors.json` file it checked is removed. | Withdrawn |
| FR-322-AC-167 | Withdrawn: the `refinement-row-node-vectors.json` file it checked is removed. | Withdrawn |
| FR-322-AC-168 | Withdrawn: the `catalog_delta` vector file it checked is removed. | Withdrawn |
| FR-322-AC-169 | Withdrawn: the nominal rekey vectors and join-stage ordering vectors it checked are removed. | Withdrawn |
| FR-322-AC-170 | Withdrawn: the nominal rekey vectors it checked are removed. | Withdrawn |
| FR-322-AC-171 | A `node_id.ordinal` of 0 and of 9007199254740991 is admitted and compares numerically (9007199254740991 after 9); an `ordinal` of 9007199254740992, of -1, of `1.5`, of `1e2` or the string `"1"` is refused `malformed_wire` at `/semantic_graph/nodes/{i}/node_id/ordinal`. | Test (TC-233) |
| FR-322-AC-180 | `contract-package-selection-vectors.json` publishes the typed source-selection rows `T`, `D` and `U` of TC-935 (`selections`, each an exact FR-322 `Selection` with its `role` and closed `{authority, identity}` `definition`) and the four compatibility cases `matching`, `missing`, `differing identity` and `unrelated selection` (`compatibility_cases`, each stating its `required_source_selections` and `lock_profile_selections` as ordered label lists). It is the authoritative machine-readable form: a consumer reads it (for example through `QUIRE_SPECIFICATION_DIR`) and neither copies nor redefines the rows. The fenced JSON block and the case table of TC-935 equal it exactly, and the QSpec test fails when either differs in any row, role, authority, identity, case name or label order. | Test (TC-935) |
| FR-322-AC-181 | A variant case whose `payloadType` is the native `Rational`, the native `Decimal`, the native `Text` (bare), or a package value type bound as `Text` without `min`, `max` and `profile`, refuses `unsupported_construct`/`declaration-form` at `Variant.payloadType` in step 1, at stage `package_read`, with FR-154's code and no graph refusal; a package value type bound to `Decimal[0, 9]`, to `Rational` or to `Text` without bounds refuses the same (`TK-14`, `TK-21`, `TK-22`), and so does an FR-154 alias declaration whose `target` is one of the four types, directly or through a chain of aliases that each resolve (`TK-27` to `TK-29`, `TK-33`). The cases whose `payloadType` is an alias of `Float64`, also through a chain of aliases (`TK-34`, `TK-35`), `Float32`, `Float64`, `Bytes`, `JsonObject`, `String`, `UUID`, `Timestamp`, `Duration`, a `Text[0, 8; nfc]` value type, or a declared tuple or variant type are admitted, so unbounded `Bytes` and `JsonObject` are not refused (`TK-20`, `TK-26`). | Test (TC-940) |
| FR-322-AC-182 | A tuple position whose `typeRef` is the native `Rational`, the native `Decimal`, a bare native `Text`, or a package value type bound as `Text` without bounds refuses `invalid_model_binding`/`malformed-declaration` at the position in step 1, at stage `package_read`, as `ix://quire/native/Clock` does and ahead of the `missing-name` check of a later position; a position typed `Float32`, `Float64`, `String`, `UUID`, `Timestamp`, `Duration`, `Bytes` or a `Text[0, 8; nfc]` value type is admitted, a package value type bound to `Decimal[0, 9]` or `Rational` is refused (`TK-18`, `TK-23`, `TK-24`), and so is an FR-154 alias whose `target` is `Rational`, `Decimal` or a bare `Text` (`TK-30` to `TK-32`); an alias of `Float64` is admitted (`TK-34`, `TK-35`). The other refused positions are `TK-15` to `TK-17` and `TK-19`, and `TK-25` is admitted. | Test (TC-940) |
| FR-322-AC-183 | A field, an operation parameter or an operation result typed `Rational`, `Decimal` or bare `Text` is admitted by FR-154 intake and gives the member no type: the application that resolves it refuses `ill_typed`/`operator-ineligible`, never a mapping to a `scalar_type` node, whether the declared type is the native or a package value type bound as `Decimal[...]` or `Rational`. The four types of FR-322-AC-181 have no node mapping and no identity rule, and no operation lookup is reached for them. | Test (TC-280) |
| FR-322-AC-240 | The slot vectors name a complete selected package. `domain-package-acme-timed-slots.json` is a full FCD semantic IR 2.0.0 document of `acme/orders` (built as FR-322-AC-151: the whole document, with no view substituted) declaring the object type `Order` with the fields `sku`, `v`, `ts`, `c`, `b`, `o`, `g`, `s`, `q` and `clock`, the record value type `Timer` (clock field `x`, data field `n`), the `sequence` type `ClockRow` and the `map` type `ClockMap`. `Order` also declares, copied unchanged from `domain-package-acme-orders.json`, the relationship `ix://acme/orders/relationship/Order-order-Order` and the operation `ix://acme/orders/Order/size`, so every model-owned node of the package (the relationship node, the `size` clause function and the `Order` object type) is declared by the selected document (the strict FR-322 owner join; no node is added or removed). `fixtures/positive-timed-slot-nodes.json` is the dense timed fixture with `SL-01` to `SL-05`, `SL-06` (the published place) and `SL-07`, the places `v` and `q`, the parameters `other`, `i` and `e` and `PR-01` and `PR-03` added before the model root and every place listed by the timed root. The vectors' `selection_identity` and the owner identity of its `model_root` are the document's identity; each `package_node` index of a vector names the published node; the rows of every place are derived from the document alone. The Bag and OrderedSet places `SL-03` and `SL-04` declare the fields `b` and `o`. | Test (TC-367; CK-28; SL-01..SL-07, PR-01, PR-03) |
| FR-322-AC-241 | The nested collection `g: Sequence<Sequence<Clock>>` of `SL-05` is declared by FCD members alone: the `Order` field `g` has `typeRef` `ix://acme/orders/ClockRow`, presence `required` and multiplicity `{lower: 0, ordered: true, unique: false}` (no `upper`), and `ix://acme/orders/ClockRow` is a type definition of kind `sequence` whose `items` is `ix://quire/native/Clock`. The `sequence` definition gives the inner `Sequence<ix://quire/native/Clock>` and the multiplicity the outer level, so the member type renders `Sequence<Sequence<ix://quire/native/Clock>>` and the rows 0, 1 and 2 of the place are `g` (kind `sequence`, bounds `0` and `unbounded`), `Sequence<Sequence<ix://quire/native/Clock>>[slot]` (declaring type `Sequence<ix://quire/native/Clock>`, member type `Sequence<ix://quire/native/Clock>`, kind `sequence`, `0`, `unbounded`) and `Sequence<ix://quire/native/Clock>[slot]` (declaring type and member type `ix://quire/native/Clock`, kind `none`). The map place `clock` is declared the same way by the `map` definition `ClockMap` (`values` `Clock`) under multiplicity `{1, 1}`. A `typeRef` that is a rendered string, a type definition without `items` or `values`, and a bound on the definition are not FCD members and no encoding is read from them. | Test (TC-367; CK-28; SL-05, SL-06) |
| FR-322-AC-242 | `GD-02` and `GD-03` publish `patch`, an RFC 6902 array of `add`, `remove` and `replace` operations on `fixtures/positive-timed-slot-nodes.json`. The patched package is valid under the V2 schema; the reader refuses it with the vector's `expected_code`, `expected_cause` and `locus` and no other. The unmutated package is the control and is admitted. | Test (TC-367; CK-29; GD-02, GD-03) |
| FR-322-AC-243 | A `clock_guard` node that no timed root's `guards` lists refuses `invalid_package`/`timed-node-reference` at `/semantic_graph/nodes/{i}/body`, `i` the graph index of the unlisted node (the first in ascending node-id order when several are unlisted), not at the root's `guards` aggregate; `GD-03` refuses at `/semantic_graph/nodes/47/body` with `locus_suffix` `body`. The second place of `GD-02` (a twin of `SL-07`'s path whose last `resolved_path` row names a member the selected declaration lacks) refuses `invalid_package`/`malformed-clock-place` at `/semantic_graph/nodes/72/body/members/2/value/members/1`, its forged row. The aggregate order check (FR-322-AC-260) runs first, the row join of each place node (FR-322-AC-124) second and the repeated-path determination (`duplicate-clock-place`) third. The `GD-02` patch appends the twin last in the root's `clock_places` and is in order; this criterion asserts no aggregate order. | Test (TC-367; CK-29; GD-02, GD-03) |
| FR-322-AC-260 | The slot vectors' `aggregate_order` section publishes the order rule and the cases `AO-01`..`AO-10` over `fixtures/positive-timed-slot-nodes.json`: `AO-01`, the selected package, lists `clock_places`, `resets`, `guards`, `invariants`, `urgencies` and `constraints` in ascending order key (the nine places in the order of their source regions, which is not their node-id order); `AO-02` swaps two adjacent places, `AO-03` lists the places in ascending node-id order (an order by node id is not canonical) and `AO-04` swaps the two urgencies; `AO-05` and `AO-06` give two places one start and order them by their end (in order, and out of order), `AO-07` gives the second place a further `source_map` entry that starts earlier, so its minimum key falls below its predecessor's, and `AO-08`..`AO-10` give two places one region and order them by the place path key (FR-322-AC-265): `AO-08` lists them in path order (admitted), `AO-09` out of it, `AO-10` gives a twin of a path the same region as the place it repeats (equal keys, refused as out of order, because the order check runs before the row join of the twin, which would otherwise refuse it `malformed-clock-place`). The order key is the minimum triple (`source.identity`, `start`, `end`) over every region of every `source_map` entry of the node and, for a `clock_place`, its place path key after it (FR-322-AC-265). Each of `AO-02`..`AO-04`, `AO-06` and `AO-07` is an RFC 6902 `patch`, is schema-valid, and refuses `invalid_semantic_graph`, with no cause, at the first member whose order key is not above its predecessor's (`locus`), and no other refusal; `AO-05` is admitted. A repeated path is a distinct place node in order (`TM-12`, `GD-02`), never one node listed twice (`TM-30`), which would repeat its order key and refuse `invalid_semantic_graph`, with no cause, at the repeated member. `GD-02`, which appends its second place last, is in order. | Test (TC-367; CK-30; AO-01..AO-10) |
| FR-322-AC-261 | The five clause forms are closed layouts with every field authored on the wire (`proposals/checked-package-v2/timed-clause-node-vectors.json`, `forms`): a `clock_constraint` of kind `place` is exactly `kind`, `place`, `owner`, `slots`, `comparison`, `bound` and of kind `parameter` exactly `kind`, `operation`, `parameter`, `comparison`, `bound`; a `clock_reset` of kind `place` is exactly `kind`, `operation`, `place`, `owner`, `slots`, `binders`, `condition`, `value` and of kind `parameter` exactly `kind`, `operation`, `parameter`, `value`; a `clock_guard` is exactly `operation`, `parts`, each part the binding `predicate` followed by at least one constraint reference; a `time_invariant` is exactly `predicate`, `bounds` with at least one bound; a `clock_place` is exactly `model`, `path`, `resolved_path` (a `guards` binding is refused, `CL-27`); `comparison` is one of `lt`, `le`, `eq`, `ge`, `gt`. An empty `parts` (`CL-17`), an empty `bounds` (`CL-18`), a comparison outside the five words (`CL-19`) a part with no constraint (`CL-20`) and a place with a `guards` binding (`CL-27`) are refused by both schemas at the node's `body`, before any semantic check. | Test (TC-367; CK-32; CL-01, CL-17..CL-20, CL-27) |
| FR-322-AC-262 | The references of a clause are joined in ascending node-id order and refuse `invalid_package`/`timed-node-reference` at the member: an `operation` that is a frame and not an `operation_anchor` (`CL-02`) or an anchor with an execution point (`CL-03`, see below), an invariant bound compared with anything but `lt` or `le` (`CL-07`), a `parameter` the operation does not declare (`CL-14`) or declares with a type other than `Clock` (`CL-15`), refused at the literal, `/body/members/2/value`; an operation anchor of another model whose own context, frame and `semantic_type` agree, so that every state-step join holds (`CL-25`; the same rebinding that leaves the frame on the first model is refused earlier by the state step, `CL-54`); a guard part that names a `parameter` constraint of another operation (`CL-21`); a predicate that is not Boolean, in a guard part (`CL-22`) or an invariant (`CL-23`); an invariant bound compared with `eq` (`CL-24`); the `Clock` parameter form is the inline `kind: parameter` of a reset and a constraint, with no new node form and no new member of the root. A conditional reset of a `Clock` parameter does not exist: a `parameter` reset has no `condition` binding and `CL-28` adds one and is refused by both schemas. `CL-03` is carved out of the schema-validity assertion of this criterion, of `CK-33` and of FR-342-AC-11 for this change only: the V2 schema's generic body rule rejects any node with an `execution_point` binding, so the patched package is not schema-valid, a gap that STD-555 closes; until then the vector is a reader-level vector (`schema`: `anchor-point-schema-gap`) and its refusal is asserted by the reader oracle alone. | Test (TC-367; CK-33; CL-02, CL-03, CL-07, CL-14, CL-15, CL-21..CL-25, CL-28) |
| FR-322-AC-263 | Clause membership is derived on the clause side. The root lists every guard, reset, invariant and constraint and is the only list of `clock_guard` nodes; a constraint is a member through the guard part or invariant that names it; a `clock_place` has no `guards` field (it could only list the `clock_guard` nodes that reach it, a cycle in the graph, or repeat their predicates unchecked; `CL-27`). Every timed node the root does not list refuses `invalid_package`/`timed-node-reference` at that node's own `body`, first in node-id order (`CL-05`, a constraint; `CL-26`, a reset). A model has at most one `clock_guard` for each operation anchor (`CL-04` refuses the second at that guard node's own `operation` binding), and a constraint the root lists that no part and no invariant names refuses at the root's `constraints` member (`CL-06`). Quantified guards and invariants are out of scope (STD-554, `QN-01`, `QN-02`). | Test (TC-367; CK-34, CK-37; CL-04..CL-06, CL-26, QN-01, QN-02) |
| FR-322-AC-264 | The terms of a place clause are typed as the terms of `present(p)`: an `owner` that is not of the place's owner type refuses `ill_typed`/`type-mismatch` at `/body/members/{k}/value` (`CL-08`), a slot term of another type than the selector's domain `ill_typed`/`type-mismatch` at that slot (`CL-10`), a `slots` count other than the number of `slot` steps `ill_typed`/`operator-ineligible` (`CL-09`); a reset of a place whose first field the operation's frame does not modify (`CL-11`), a `binders` member that is not a slot term or is out of slot order (`CL-12`) and a non-Boolean `condition` (`CL-13`) refuse `invalid_package`/`timed-node-reference`. | Test (TC-367; CK-35; CL-08..CL-13) |
| FR-322-AC-265 | The strict aggregate order of FR-322-AC-260 holds in the lists the clause forms nest, the constraints of a guard part and the bounds of an invariant, under the same order key (`CL-16` swaps two constraints of one part and refuses `invalid_semantic_graph` with no cause at the second). Members that share a minimum region triple are ordered by a structural key and never by a source region of a shared term: a `clock_place` by its place path key (field by name bytes, union by case bytes, tuple by index, slot by collection bytes then minimum then maximum with `unbounded` last; neither the selector nor `resolved_path` is read), a `clock_constraint` by (the place path key, owner term, slot terms, comparison `lt`, `le`, `eq`, `ge`, `gt`, bound), a `clock_reset` by (the place path key, operation name, condition, binders in order, owner, slots, value), a `parameter` constraint or reset by operation name, parameter name, then the same, a `time_invariant` by (every bound's key in order, predicate), a `clock_guard` by its operation name and an `urgency` by kind and operation name; a term is keyed by its structure (its form and the shape of its body, a reference replaced by the key of its target). `AO-08` lists two places of one region in path order and is admitted, `AO-09` lists them out of path order and `AO-10` repeats a path with its region, and both refuse `invalid_semantic_graph` at the member; `CL-29` lists three constraints of one region in key order (admitted) and `CL-30` in region order only (refused), `CL-31` and `CL-32` do the same for two invariants. `CL-33` and `CL-34` give two constraints of one place and region the comparisons `le` and `eq`, `CL-35` and `CL-36` one comparison and the bounds `9/1` and `10/1` (numeric, not text, order), `CL-37` and `CL-38` differ only in the owner term (`other`, `self`), `CL-39` and `CL-40` only in the slot term (`e`, `i`), `CL-41` and `CL-42` two resets only in the operation (`arm`, `touch`), `CL-43` and `CL-44` only in the condition, `CL-48` and `CL-49` only in the value, `CL-50` and `CL-51` only in the binders, `CL-45` and `CL-46` three invariants of one first bound only in the second bound, `CL-52` and `CL-53` two invariants only in the predicate; the first id of each pair is admitted in key order and the second, the reverse, refuses at the second member. `CL-47` is `CL-37` with an earlier region added to the shared term `other`, so that the order of the two terms' minimum regions flips: the package stays admitted, because a node's position does not change when another node uses a term it shares. The model document of a `model_root` node is not a source artifact, so no location in it orders a place. | Test (TC-367; CK-30, CK-36; AO-08..AO-10, CL-16, CL-29..CL-53) |
| FR-322-AC-266 | The clause fixture `fixtures/positive-timed-clause-nodes.json` selects `domain-package-acme-timed-clauses.json` (which declares the entity `Gadget` with the one operation `ping()` only so that `CL-25` can name an anchor of another model) and carries the clause forms in both kinds; its dependencies are the sorted references of its body. The dense timed fixtures `positive-timed-model-nodes.json` and `positive-timed-slot-nodes.json` carry the same forms. The vectors `CL-01`..`CL-56` are replayed as RFC 6902 patches on the clause fixture. | Test (TC-367; CK-32..CK-37; CL-01..CL-56, QN-01, QN-02) |
| FR-322-AC-267 | The checked package excludes three source forms by rule, each refused by the producer `unsupported_construct`/`expression-form` at the form and naming the construct (`proposals/checked-package-v2/timed-clause-node-vectors.json`, `excluded_forms`): a guard disjunct that reads no clock (`GJ-01`; every disjunct that reads a clock is built, `GJ-02`, `GJ-03`), a clock write that is not a reset to a constant, namely a whole-collection write (`CW-01`, `CW-02`), the drop `self.v = absent` (`CW-03`) and the ordered element write `self.ts[i] = Timer { ... }` (`CW-06`), and a selection reset whose condition reads element data (`SD-01`). A constant write to a guarded place is a built reset (`CW-04`), a presence requirement is a postcondition predicate with no clause node (`CW-05`) and a condition over the binder's presence is built (`SD-02`). The reader rows are `CL-20` (a part with no constraint, refused by both schemas), `CL-55` (a place reset with no `value`, refused by both schemas at the node's `body`) and `CL-56` (a reset of a collection place with no slot term, `ill_typed`/`operator-ineligible` at `/semantic_graph/nodes/84/body/members/4/value`); no reader rule is added or relaxed. | Test (TC-367; CK-32, CK-38; CL-20, CL-55, CL-56, GJ-01..GJ-03, CW-01..CW-06, SD-01, SD-02) |
| FR-322-AC-268 | The two dimension nodes of a dimension equivalence may belong to different packages: a consistent graph of equivalences over the `Time` dimensions of three packages is admitted, with the root ratio of each pair fixed by the base roots and the product around the cycle exactly one. A pair in descending order, a pair of one node twice, a pair repeated by a second equivalence node, a pair naming a node that is not a dimension, a cycle whose root-scale product is not one and a schema-valid unreduced `root_scale`, an equivalence of derived dimensions whose normalized maps are unequal (`XP-08`) and one whose scale is inconsistent with the base roots (`XP-09`) each refuse `invalid_semantic_graph` at the later equivalence node in node-id order (`proposals/checked-package-v2/dimension-equivalence-vectors.json`, `XP-01` to `XP-10`; the reader that decides them is quire-contract-ir's, FR-063, and the vectors are authoritative for it). | Test (TC-280) |
| FR-322-AC-269 | A source `enum`, `dimension` and `unit` node carries the closed body of FR-322 "Nominal nodes" (`enum` `{term, ordered, members}`, `dimension` `{term, terms}`, `unit` `{term, dimension_node_id, target_unit_node_id, scale, offset}`), and the V2 schema refuses, as `malformed_wire`, an empty `aggregate` body on one of them (`NB-01`), an enum with no members (`NB-02`), a unit without `scale` (`NB-03`), an extra member on a unit body (`NB-04`) and a non-integer dimension exponent (`NB-05`); an enum with `ordered` true (`NB-06`) is admitted and is `ordered_enum`. | Test (TC-233) |
| FR-322-AC-270 | The `relationship_end` member carries `destination_type`, a producer-supplied structural `NodeId` that the reader returns verbatim without resolving or deriving it (FR-322 "Destination type of a relationship end"). `fixtures/positive-operation-identities.json` names a destination type node absent from the graph, is admitted by the full reader, and lists no `destination_type` among the node's `dependencies`; `DT-05` of `adverse.json` `destination_type_mutations`, a destination naming a node present in the graph, is admitted and kept verbatim. | Test (TC-233) |
| FR-322-AC-271 | A `relationship_end` member with no `destination_type` (`DT-01`), a `destination_type` that is a string (`DT-02`), one with a member beyond `package`, `path` and `ordinal` (`DT-03`) or one whose `ordinal` is a string (`DT-04`) fails the V2 schema, which a reader reports `malformed_wire`. | Test (TC-233) |
| FR-322-AC-272 | A node `ordinal` is producer-assigned. Nodes sharing one `package` and `path` at ordinals that are not `0` (`NO-05`: one sibling at `7`), not contiguous and in neither ordinal nor graph order (`NO-06`: siblings at `3` then `2` placed before the node at `0`) are admitted by the full reader; the reader neither requires `0`, contiguity or sibling order nor derives an ordinal. | Test (TC-233) |
| FR-322-AC-273 | Two nodes of an equal `{package, path, ordinal}` refuse `invalid_semantic_graph` at the second node's `node_id`, `/semantic_graph/nodes/{i}/node_id`: a sibling at the original's ordinal `0` (`NO-07`) and two siblings both at ordinal `4` (`NO-08`, refused at the second). A recursion-group member's `ordinal` is held to the same uniqueness rule. The reader that decides these refusals is quire-contract-ir's, and the vectors (`proposals/checked-package-v2/node-ordinal-vectors.json`, `NO-07`, `NO-08`) are authoritative for it; the test here checks them with a standalone duplicate-id helper, not with the shared model oracle's own refusal. | Test (TC-233) |
| FR-322-AC-274 | A recursion-group member never carries a path derived from operands. A member with a `declaration` keeps its declaration path, so a group may hold several declared members with distinct paths (the Flow, joinPred and Holder cycle of TC-946 K14); an anonymous derived member carries the path of the group's declared member where there is exactly one, and a producer-chosen path, admitted, where there is none or several. Nodes sharing `package` and `path` are told apart by their producer-assigned `ordinal`, unique among nodes sharing them. Group membership is the shared `recursion_group` label alone: no node carries a group-position member. The `positive-recursive-records`, `positive-two-owners-a`, `positive-two-owners-b` and `positive-integer-literal-references` fixtures, each group of which has one declared member, hold this for every group member, and the K14 node set holds it for three declared members. | Test (TC-233) |
| FR-322-AC-275 | The join stages run in the order "One total order of the join stages" states, each over every node before the next starts: a package whose `model`/`tuple_value_type` node `Pair` has a `ModelOwner` naming a node its selected document does not declare, and whose `Pair`-typed parameter carries a `semantic_type` naming a node the graph lacks, refuses `missing_declaration`/`missing-selection` at `/semantic_graph/nodes/0/owner` (stage 2, the owner join), never the dangling edge of stage 4 (`TW-28`). | Test (TC-940) |
| FR-322-AC-276 | The derived deadlock-freedom, time-lock-freedom and race-freedom items add no node to `semantic_graph` and have no `node_tag`, `semantic_form` or derived path. Each is keyed by its `subject`, its anchor `node_id` and its `item_kind`. In the `derived-items-request` vector, the model subject's deadlock-freedom and time-lock-freedom items both have claim node `{package: "acme/orders", path: ["model", "model_root"], ordinal: 0}` (`model`/`model_root`, occurrence `generated` 0). The protocol subject's deadlock-freedom and race-freedom items both have claim node `{package: "example", path: ["Example", "Handshake"], ordinal: 0}` (`protocol`/`protocol_clause`, occurrence `declaration` 0). Each node resolves in the package the request carries. The two items that share a node have one `subject` and differ only in `item_kind`. No node of the package other than the anchor has a `semantic_form` that names the item's kind. A copy of the package that adds a `model`/`deadlock_freedom` node over the `model_root` anchor fails the checked-package schema. If the model subject's deadlock-freedom claim is retargeted to the protocol clause node, to item 0's `temporal`/`formula` node or to a `node_id` absent from the graph, the reader refuses the item before consumption. | Test (TC-431 BP-51) |
| FR-322-AC-277 | A derived item whose claim names its correct anchor node with an occurrence other than the one FR-368 picks is refused before consumption: the model subject's deadlock-freedom item of `derived-items-request` with `claim.occurrence` `{role: "declaration", ordinal: 0}` in place of `{role: "generated", ordinal: 0}` is admitted by the schema and refused by the reader. | Test (TC-431 BP-51) |
| FR-322-AC-278 | No published V2 package, fixture or vector holds a `node_id` object inside a `node_id.path`: every operand is an index segment `{"node": i}`. Over `fixtures/positive-dependency-call.json`, the V2 schema refuses as `malformed_wire`, at the patched segment, a path segment that is a nested `node_id` object (`NI-01`), `{"node": -1}` (`NI-02`), `{"node": "0"}` (`NI-03`), `{"node": 1.5}` (`NI-04`), `{"node": 0, "ordinal": 0}` (`NI-05`) and `{}` (`NI-06`), and a `dependency_reference` `node` whose path holds an index segment (`NI-07`). | Test (TC-233; NI-01..NI-07) |
| FR-322-AC-279 | A schema-valid index segment whose `node` is not below the length of `semantic_graph.nodes` refuses `malformed_wire`, with no cause, at that segment's `node` member, before every `semantic_graph` check: in a node's own `node_id` (`NI-08`, index `4` of four nodes, at `/semantic_graph/nodes/2/node_id/path/3/node`), in a literal's `type` (`NI-09`, index 9007199254740991) and in a `source_map` entry's `node_id` (`NI-10`, at `/source_map/2/node_id/path/3/node`). | Test (TC-233; NI-08..NI-10) |
| FR-322-AC-280 | A cycle through own-path index segments refuses `malformed_wire`, with no cause, at the first segment by holding node's graph index and then path position whose named node reaches the holding node: a node that names itself (`NI-11`, at `/semantic_graph/nodes/2/node_id/path/3/node`) and two nodes that name each other (`NI-12`, at node `0`'s segment `/semantic_graph/nodes/0/node_id/path/2/node`). An index segment that names a later node without a cycle is admitted (the reversed fixtures of FR-322-AC-95). | Test (TC-233; NI-11, NI-12) |
| FR-322-AC-281 | A graph holding the Integer node and a left-nested chain of 100,000 derived `expression`/`application` nodes, each naming the previous one through one index segment (`NI-13`), writes its node ids in at most 200 bytes per node, at the same maximum JSON nesting depth as a chain of 100, and its text is parsed by a JSON reader with a nesting limit of 128. The index checks admit it, and its rebuilt structural keys are 100,001 distinct keys, the key of each chain node holding its predecessor's key as its last path element. | Test (TC-233; NI-13) |
| FR-322-AC-282 | An index segment resolves against the graph of the document the id belongs to, never the graph of the id's `package`: in `fixtures/positive-dependency-call.json` with its nodes reversed (FR-322-AC-95), the importer's id of the imported `Int[0, 9]`, `package` `test/predicates` with `{"node": 3}` naming the importer's Integer node, copied into a document about the importer (`NI-14`), rebuilds against the importer's graph to the structural key of `fixtures/positive-dependency-library.json`'s `Int[0, 9]`; resolved against that library's own graph, where index `3` is `Int[0, 9]` itself, it would name itself and refuse as a cycle. | Test (TC-233; NI-14) |
| FR-322-AC-283 | A cyclic predicate join that includes its `protocol_reference` carrier is admitted when every member, the carrier included, carries one `recursion_group` (TC-946 `K16`): the Flow, joinPred and Holder cycle of `K14` closed through a carrier `R` whose body target is `Flow`'s `node_id`, whose only dependency is `Flow`, and whose single `expression` occurrence keeps its `source_map` entry; `R` is an anonymous derived member whose `path` is a producer-chosen path (the group holds three declared members; `K16` uses `["R"]`) and whose `ordinal` is producer-assigned, so changing `R`'s ordinal to another unused value, or its path to another producer-chosen path, is admitted and no other node id changes. The V2 schema admits `recursion_group` on `R`. | Test (TC-946; K16) |
| FR-322-AC-284 | An acyclic carrier keeps its derived id `{package, path: ["protocol", "protocol_reference", target node_id], ordinal: 0}` and the single-target behaviour of FR-322-AC-66 (TC-946 `K17`). A lone `recursion_group` label on it is admitted, with the same package otherwise unchanged and no refusal, because the label constrains cycles only; the closed V2 schema no longer refuses it `malformed_wire`. | Test (TC-946; K17) |
| FR-322-AC-285 | Malformed membership of a cyclic component refuses `invalid_semantic_graph`, with no cause, in the graph-shape step, before the owner join and before the carrier join (TC-946 `K18` to `K21`, over the `K16` node set, whose members in node-id order are `Flow`, `Holder`, the carrier `R` and `joinPred`): `R` with no label while the others carry one (`K18`) is refused at `/semantic_graph/nodes/{n}` of `R`; `R` with a label other than the others' (`K19`) at `R`'s `/recursion_group`; `Holder` with no label while `R` and the rest carry one (`K20`) at `Holder`'s node, the first deviating member; and every member unlabelled (`K21`) at `Flow`, the component's first member. A component with no cycle through the carrier, and an acyclic carrier alone, are not refused (FR-322-AC-284). No group is refused for its size, and two disjoint cyclic components that reuse one label are both admitted (`K22`, with each component labelled consistently). | Test (TC-946; K18 to K22) |
| FR-322-AC-286 | Two `protocol_reference` carriers with one body target refuse `invalid_semantic_graph`, with no cause, at the later carrier's `/semantic_graph/nodes/{n}/node_id`, in node-id order, in the graph-shape step before the cycle check: `K23`, the `K16` node set plus a second carrier of `Flow` at path `["R"]`, ordinal `1`, is refused at that second carrier even though its `node_id` differs from the first's. A grouped carrier and an acyclic carrier to different targets are not refused. | Test (TC-946; K23) |
| FR-322-AC-244 | TC-971 resolves OV-01 and OV-02's option element type and checked-payload selectors against the published integer-literal fixture. The closed body has exactly one `value` binding to a payload reference; changing the selected type or payload changes the node. OV-SR1..OV-SR4 (empty members, wrong binding name, inline literal instead of a reference, and multiple bindings) fail the closed body schema. The strict reader admits each positive package and refuses OV-R1's non-composite semantic type and OV-R2's non-option composite semantic type as `invalid_semantic_graph` at `semantic_type`; OV-R3's boolean payload and OV-R4's option-type node used as a payload as `ill_typed`/`type-mismatch` at `body/members/0/value`; and OV-R5's dependency list with an extra target as `invalid_semantic_graph` at `dependencies`. | Test (TC-971) |
| FR-322-AC-287 | A source that declares the dimension equivalences `E0` = `B ~ C` then `E1` = `A ~ B` (so source order is the reverse of the order of their node ids) is emitted with two `scalar_type`/`dimension_equivalence` nodes, each with exactly one occurrence, role `anchor`: `E0`'s at ordinal `0` and `E1`'s at ordinal `1`, each with its own declaration's region. A third equivalence declared between them in the source moves the later one's ordinal to `2`. | Test (TC-233) |
| FR-322-AC-288 | `checked_term_data` is absent when no facts are retained and otherwise has exactly `equality_decisions`, `protocol_locations` and `dispatch_tables` arrays; each may be empty. Each equality record identifies the exact equality application `NodeRef` and occurrence; its `term_path` is empty because every composite application is a separate graph node. When an equality occurs beneath an `and`, `or`, aggregate or binding form, that parent body references the equality application node by its `NodeRef`, and the equality row is keyed to that child node. Positive equality vectors include the four TC-1070 Fixture E expressions at rows 8, 12, 15 and 17 as four separate equality rows. Records preserve operator, ordered operand positions, source types, nullable conversion targets and selected schedule. The enum record carries the compared enum's exact declaration and member identities, case, ordinal and ordered flag. Quantity records carry normalized dimension terms and either a declared unit's exact root and conversion-edge path or a compound unit's canonical root terms and exponents. A dispatch record preserves the selected operation witness `{declaring, operation}`, receiver context and candidates in producer order, with each candidate's selection key, operation-body handle and nullable effective-precondition body handle. A protocol location preserves its application/occurrence, per-term ordinal, named type-declaration or protocol-attempt origin and child-index path. These records do not alter semantic graph nodes or ids, and readers do not infer identity from path or name. | Test |
| FR-322-AC-289 | The reader joins each equality row to exactly one local equality application `NodeRef` and source occurrence. The row has an empty `term_path`, which selects that application node as the equality root; a non-empty path refuses `invalid_package`/`invalid-value` at the first path segment. The reader does not follow a `reference` term to another graph node and does not infer an equality identity from a path or name. It checks the selected operator, both original argument positions and source types. A row targeting an absent or foreign node, non-application root, missing/wrong occurrence, non-empty path, different operator, or changed operand/type refuses `invalid_package`/`invalid-value` at the first mismatching sidecar member. Duplicate (`application`, `occurrence`, `term_path`) keys refuse `invalid_package`/`conflicting-definition` at the later row with both loci. An equality application without a captured row remains admitted and has no fabricated decision. | Test |
| FR-322-AC-290 | A dispatch record joins to exactly one local call application and its exact occurrence, selected-operation witness and receiver context; the witness is `{declaring: <eligible model NodeRef>, operation: <original member name>}`, where `declaring` resolves to an admitted `model` node whose `semantic_form` is `object_type`, `process`, `state_machine` or `persistence_interface` (FR-322-AC-25), and `operation` exactly names an operation member exposed by that type under FR-322-AC-30 and FR-154. A relationship declaring node, absent or foreign declaring type, field or absent operation member, or invented operation-path `NodeRef` is refused at the operation witness with the applicable typed outcome; identity is never inferred from a name or path. Every candidate body and effective-precondition handle joins to the admitted package, and each candidate selection key is the exact FR-150 structural `EffectiveId` (`owner_effective_type`, `original`) supplied by the producer and matches the candidate's admitted effective declaration identity. It is not a `NodeRef` and is never inferred from a name or reverse-mapped from a graph node. Selection keys are unique. The reader preserves candidate array order. A foreign context/body/effective identity, mismatched effective identity, wrong call application or occurrence, duplicate dispatch key or duplicate selection key refuses with a named typed outcome at the first mismatching row; it neither selects a candidate nor evaluates a precondition. An absent table is unavailable, while a present empty table is the captured empty candidate set. | Test |
| FR-322-AC-291 | Each protocol location row joins to one local expression/application node and one of its exact source occurrences; two terms sharing that application and occurrence have distinct `term_ordinal` values and preserve their different semantic locations. Each captured application/occurrence group has consecutive ordinals starting at zero. PL-01-root's lone ordinal-zero row admits; PL-R2's lone ordinal-one row witnesses the missing ordinal-zero mapping and refuses `invalid_package`/`invalid-value` at `/checked_term_data/protocol_locations`. A foreign or absent node, non-application, missing occurrence, duplicate per-term key, duplicate term ordinal or malformed origin refuses at the first mismatching row; no location is inferred from a source region, display name or selected dependency. | Test |
| FR-322-AC-292 | `checked-term-data-vectors.json` publishes positive top-level text, enum, declared-quantity, compound-quantity and plan equality records, plus two equalities beneath one `and` parent, an equality beneath `or`, aggregate-member and binding-value forms. The parent forms reference each child equality application by its exact `NodeRef`; each equality record names that child node and has an empty `term_path`. The four TC-1070 Fixture E expressions at rows 8, 12, 15 and 17 remain four separate equality rows. Positive metadata demonstrates these parent-to-child references without adding a wire member. The vectors also include two protocol terms at one application occurrence and ordered and empty dispatch candidate sets whose selected operations use the typed declaring-node/name witness and whose selection keys use the structural FR-150 `EffectiveId`. Schema-invalid cases include a checked-package `NodeRef` used as an effective identity, an invented operation-path `NodeRef` (`DT-S5`), missing identity members, a missing equality `term_path`, an unrecognized path selector kind and extra identity fields. Schema-valid reader mutations cover wrong operator/operand/type/occurrence, a non-empty equality path ending at a reference to a distinct non-equality application, foreign application and effective identity, mismatched effective identity, a non-member selected operation (`DT-R6`), duplicate equality application/occurrence key, protocol key and dispatch key, duplicate selection key, missing protocol-term mapping and absent precondition join. Reader-level mutations have the expected refusal code, cause and exact JSON pointer. Read work charges every sidecar row and each nested unit, enum and candidate entry before inspection. The vectors contain 17 sidecar rows, 3 unit facts, 2 enum facts and 2 dispatch candidates, with zero equality `term_path` segments, for an exact 24-unit boundary; admission succeeds at 24 units. A 23-unit limit refuses `resource_exhausted` at `/checked_term_data/dispatch_tables/0/candidates/1`, the final charged candidate, with no partial data view. | Test (TC-233) |
| FR-322-AC-293 | A `fairness` member carries a closed `target` union: model operation (`declaration` node id and `name`), protocol root, attempt node or compensation template (each an exact structural `node` id), parallel branch (`parallel` node id and `branch` label), or hybrid transition (`declaration` node id and `name`). `temporal-fairness-target-vectors.json` publishes one positive member per variant and malformed members with a missing/unknown discriminant, missing required identity, extra identity field and invalid node ordinal; each positive satisfies the V2 `OperationMember` schema and every malformed member fails it. Eleven schema-valid reader cases distinguish absent, foreign-package and wrong-kind parallel ids, a missing branch label, wrong-kind protocol-root, attempt and compensation targets, and root, attempt, parallel and compensation targets owned by another protocol clause in the same package; each records the required refusal code, cause and exact target or label JSON pointer from FR-370-AC-15. | Test (TC-303) |
| FR-322-AC-294 | Each clock-configured `temporal`/`temporal_clause` has exactly one `temporal_bindings` row keyed by its structural clause `NodeRef`; each row has the exact non-empty clause clock name and one closed `ClockConfiguration` whose kind and parameter set match the clause's selected `temporal_profile` law. The clause identity distinguishes declarations even when their model, profile and clock names agree; the multi-clause positive proves two distinct clause keys and ascending structural `NodeRef` row order. The three matching profile/configuration pairs (event-position, fixed-sample and timestamped-event) are complete checked-package positives; an unknown kind, a missing parameter or wrong-typed parameter is `malformed_wire`. A schema-valid row whose declaration `NodeRef` is absent or is not a temporal clause refuses `invalid_package`/`timed-node-reference` at `/temporal_bindings/{i}/declaration`; a duplicate row refuses with that code/cause at its second row. A missing row for a clock-configured clause refuses with that code/cause at the first such clause's `/semantic_graph/nodes/{N}/body`, where `N` is its graph index. These row-coverage checks follow aggregate order validation and precede per-node joins. After declaration resolution, rows are checked in ascending structural `NodeRef` order: clock-name mismatch refuses `invalid_package`/`timed-node-reference` at `/temporal_bindings/{i}/clock_name`, before a profile/configuration kind mismatch refuses with the same code and cause at `/temporal_bindings/{i}/clock/kind`. No profile identity is repeated in the row: the reader joins the clause law to `lock.profile_selections`. When no clock-configured temporal clause exists, `temporal_bindings` is absent; its absence never defaults a required clock configuration. | Test (TC-134; TC-233) |

## Dependencies

- [interface_004](./interface_004-checked-package.md) owns the operation boundary.
- FR-132 owns the behavior that produces or consumes this value.
- [FR-468](../../functional/analysis/FR-468-certify-per-step-refinement-obligations.md)
  owns the per-step proof obligations a refinement owner's `generated`
  occurrences number.
- [FR-195](../../functional/ir-proof/FR-195-lower-complete-contract-ir.md)
  consumes this boundary for exact Contract IR lowering.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) owns comparison of
  the distinct identity domains.
- [FR-154](../../functional/type-model/FR-154-admit-domain-package-model.md)
  owns domain package admission, and
  [FR-150](../../functional/type-model/FR-150-preserve-model-normalization-provenance.md)
  owns the effective view a model-owned member resolves in.
- [FR-153](../../functional/type-model/FR-153-query-closed-model-environments.md)
  owns the population binding a population reference names, and QSL FR-391
  the state-clause population name that lowers to it.

## Status

Accepted complete-V1 boundary value contract.

## References

- Tickets: Linear STD-613, STD-614 and STD-615 (the shared checked-term-data
  carrier for equality decisions, protocol semantic locations and dispatch
  candidate tables).
- Tickets: Linear QSL-290, QSL-638 (the node `owner` member), QSL-659 (record-valued model fields) and
  QSL-692 (settled derived type forms for StateModel lowering).
