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
  - target: ix://agent-ix/quire-spec-language/FR-019
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
  `proposals/checked-package-v2/operation-catalog.json`, whose entry names the
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
  projection, navigation or dispatched call never resolves a name by search, or
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
  operation refusal. Library exports resolve only through this member; a
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

`read` receives limits from its caller; they are not artifact data and never
participate in package identity. Its closed `ReadResult` is exactly one of:
`admitted { package }`; `refused { code, stage, cause, loci }`; or
`incomplete { limit_kind, limit, consumed, stage, loci }`. The v2 refusal code
vocabulary is `unknown_contract_version`, `malformed_wire`, `duplicate_member`,
`unknown_member`, `noncanonical_wire`, `stale_dependency`,
`digest_domain_mismatch`, `unknown_required_capability`,
`invalid_semantic_graph`, `invalid_source_map`, `unsupported_node_tag`,
`invalid_package`, `ill_typed`, `ambiguous_declaration`, `missing_declaration`
and `invalid_model_binding`; the last five carry exactly the `stale-node-key`,
operation and declaration causes above, and the `missing-name` and
`malformed-declaration` frame-entry causes
[FR-340](FR-340-checked-package-frame-location.md) defines, with the code the
native diagnostics catalog pairs with each cause, and no other code carries
them.
Before expensive semantic work, the reader measures raw wire bytes; a byte
overage returns `incomplete`. It then performs strict syntax/member validation;
a malformed wire returns `refused`. It charges depth, graph nodes/edges,
occurrences/loci, diagnostics and validation visits as they are traversed; the
first exhausted limit returns `incomplete` with its actual counter and no
admitted package. Equal-limit input is admitted if all later checks succeed.

## Operation families, groups and constraints

This section is normative; the catalog file is its machine-readable copy and a
disagreement between them is a catalog defect, not a reader choice.

A type node's **family** is read from the type the node resolves to through its
`bounded_domain` base chain: `scalar_type` forms `boolean`, `integer`,
`rational`, `decimal`, `float32`, `float64` and `text` are their own family, a
`unit` is `quantity`, an enum is `ordered_enum` when its nominal preimage is
ordered and `enum` otherwise; `composite_type` forms `option`, `record`,
`tuple`, `sequence`, `set`, `bag` and `ordered_set` are their own family, and a
model entity type is `reference`. A term's family is its type's family, except
that a function reference is `function`, a relation reference is `population`,
an aggregate is `aggregate`, a binding is `binder` (whose *bound* is the
descriptor of its value) and a clause application is `clause`.

| Group | Families | Decision rule |
| --- | --- | --- |
| `any_value` | `boolean`, `integer`, `rational`, `decimal`, `float32`, `float64`, `text`, `enum`, `ordered_enum`, `quantity`, `option`, `record`, `tuple`, `sequence`, `set`, `bag`, `ordered_set`, `reference` | An operand position admits a term whose family is a member. |
| `any_term` | `any_value` plus `function`, `population`, `aggregate`, `binder`, `clause` | Used only as the rest position of clause operations. |
| `exact_numeric` | `integer`, `rational`, `decimal` | Exact numeric operands of conversions. |
| `rational_promotable` | `integer`, `rational` | Operands promoted exactly by `n ↦ n/1` (FR-146, FR-149). |
| `ieee_float` | `float32`, `float64` | IEEE operands; no `eq`, `ne`, ordering or `negate`. |
| `collection_kind` | `sequence`, `set`, `bag`, `ordered_set` | Collection operands and `element` members. |
| `structural_kind` | `option`, `record`, `tuple`, `sequence`, `set`, `bag`, `ordered_set` | Structural comparison operands. |
| `enum_kind` | `enum`, `ordered_enum` | Enum operands. |

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
An unknown object member or node tag is a V2 refusal. A transport relay may
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
| FR-322-AC-10 | Enum, dimension and declared-unit node keys match the closed JCS preimage vectors; stale-body keys, non-normalized rationals, invalid owner joins and noncanonical member/term order among an enum's members, a dimension's terms or a declared unit's own fields refuse as `invalid_semantic_graph` (a `state`/`frame` node's own `modifies`/`creates`/`deletes` canonical order is [FR-340-AC-4](FR-340-checked-package-frame-location.md), not this criterion). | Test (TC-217) |
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

## Dependencies

- [interface_004](./interface_004-checked-package.md) owns the operation boundary.
- FR-132 owns the behavior that produces or consumes this value.
- [FR-195](../../functional/ir-proof/FR-195-lower-complete-contract-ir.md)
  consumes this boundary for exact Contract IR lowering.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) owns comparison of
  the distinct identity/digest domains.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
