---
id: FR-322
title: "Immutable checked-package artifact"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface-004
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-132
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---

# FR-322: Immutable checked-package artifact

## Description

When a native complete-V1 package completes linking and checking, the producer
SHALL use the immutable `quire.checked-package/v2` contract below rather than
an untyped map, display string or implementation-local structure. Its normative
closed schema and independent vectors are the I04 public boundary; native
parser/CST and implementation-private package values are not part of that
boundary. The frozen `quire.checked-package/v1` predecessor remains unchanged
and may be read only through an explicitly selected V1 compatibility path.

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
  compiled-model selection and required-feature identities. Every byte digest
  carries its own declared domain; a raw source/definition/model digest cannot
  substitute for a semantic package digest or for another artifact role.
- `semantic_graph` — one `quire.checked-semantic-graph/v2` closed tagged-node
  union. Its current tags are `scalar_type`, `composite_type`,
  `bounded_domain`, `value`, `expression`, `function`, `model`, `relation`,
  `state`, `temporal`, `protocol`, `claim` and `correspondence`. A node key is
  (`node_id.domain`, `node_id.digest`); node identifiers are unique within the
  graph and the `identity_projection` is the typed-key ordered projection of
  every node excluding source occurrences and presentation. References name a
  compatible declared target by node key. A recursive reference is legal only
  when every cycle member names the same explicit `recursion_group`; all other
  graph cycles refuse. Every node has one or more exact source occurrences.
- `operation` — every `application` semantic term, wherever it occurs in a node
  body, carries `{identity, laws, mode, member, leaves}`. `identity` is one
  member of the closed `quire.checked-operation-catalog/v1` published as
  `proposals/checked-package-v2/operation-catalog.json`, whose entry names the
  operation's operator class, operand positions and rest, result form,
  constraints, required law roles, mode kind, member kind and leaf source.
  `laws` is the ordered list of `{role, definition}` pairs whose roles are
  exactly the entry's roles and whose `DefinitionRef` is one of that role's
  catalogued profiles (integer division truncating/floor/Euclidean,
  IEEE 754-2019 default, Unicode 17.0.0 text), each an exact member of the lock's
  `definition_selections`. `mode` is `null` when the entry names no mode kind
  and otherwise one catalogued `rounding`, `text_profile` or `absence` value.
  Types are authoritative for a `rounding` or `text_profile` mode: the value
  equals the single value pinned by the operand and result types, and a
  disagreement is refused rather than resolved in favour of either. `member` is
  `null` or the entry's `field`, `position`, `element`, `relationship_end`,
  `operation` or `type_argument` member naming its declaring node, so a
  projection, navigation or dispatched call never resolves a name by search.
  `leaves` is empty unless the entry compares structured values; it then lists,
  in declaration order, one `{path, laws, mode}` entry for every text leaf of
  the compared type (`field:<name>`, `position:<n>` and `inner` segments), with
  the text profile law and the mode that leaf's type pins. IEEE values have no
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
  targets and member declarations of its body. A retained key or a
  contradictory form or dependency list is `invalid_semantic_graph`.
- `declaration` — `{qualified_name}`, the library-visible name a declaring node
  introduces, with the `qualified_declaration` grammar. It is required on every
  `scalar_type`, `composite_type`, `bounded_domain` and `function` node and on
  a `value` node other than an `enum_value` that has a `declaration`
  occurrence; every other node, including expressions and let values, must not
  carry it. It enters the node identity preimage. On a node carrying a
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
  selected producer/model capability. Backend/provider installation is absent.
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
`invalid_package`, `ill_typed` and `ambiguous_declaration`; the last three carry
exactly the operation and declaration causes above with the code the native
diagnostics catalog pairs with each cause, and no other code carries them.
Before expensive semantic work, the reader measures raw wire bytes; a byte
overage returns `incomplete`. It then performs strict syntax/member validation;
a malformed wire returns `refused`. It charges depth, graph nodes/edges,
occurrences/loci, diagnostics and validation visits as they are traversed; the
first exhausted limit returns `incomplete` with its actual counter and no
admitted package. Equal-limit input is admitted if all later checks succeed.

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
that is not an admitted V2 package and a V2 lowerer refuses it. V1 remains a
separate frozen wire: adding `nominal_identity_preimage` to a V1 node is an
unknown-member refusal, and a V2 reader refuses a V1 contract version rather
than inferring the missing identity evidence. Exact selected
resource limits are admitted; one-over-limit input returns an incomplete
resource disposition without a partial checked package.

## V1 to V2 migration

Migration accepts canonical V1 bytes only as the source package reference. It
requires the exact authored source documents, definition artifacts and
compiled-model documents selected by the V1 lock and reruns the V2 linker and
checker. A successful migration returns the separately identified V2 package
and a `quire.checked-package-migration-correspondence/v1` record containing the
source V1 package key, target V2 package key and a total ordered mapping of
`{source_node_id, target_node_id, basis}` for every V1 node. `basis` is exactly
`relinked-identical` or `relinked-rekeyed`. The correspondence record is
excluded from both package identities.

A row's basis is `relinked-identical` exactly when the source and target node
digests are equal and `relinked-rekeyed` otherwise.

Missing, stale, ambiguous or incompatible reconstruction inputs refuse before
any V2 package is exposed. A migrator cannot derive absent owner, qualified
declaration, enum-member order, dimension-term or unit-conversion evidence from
V1 graph bytes. Byte-only conversion, version relabeling and silent filling are
not migration. A V1 application carries only its operator class and no law,
mode, member or leaf, so the migrator derives a V2 operation only from this
closed table, whose rows are exactly the catalogued classes with one operation
and no law, mode, member or leaf selection:

| V1 class | Derived operation |
| --- | --- |
| `conditional` | `quire.op.control.if` |
| `let` | `quire.op.control.let` |
| `pre` | `quire.op.state.pre` |
| `present` | `quire.op.option.present` |
| `value` | `quire.op.option.value` |
| `deref` | `quire.op.model.deref` |
| `temporal` | `quire.op.temporal.clause` |
| `protocol_control` | `quire.op.protocol.control` |
| `state_transition` | `quire.op.state.transition` |
| `claim` | `quire.op.claim.clause` |

Each derived operation is `{identity, laws: [], mode: null, member: null,
leaves: []}`. Every other class (`call`, `unary`, `binary`, `quantify`,
`collection`, `query`, `convert`, `reaches`) is unresolved even when the V1 lock
selects a law it could use. A target node resolves only when its application
terms, in pre-order, pair one-to-one with the same-ordinal V1 node's
application terms of the same class and each target operation equals the
derived one. Otherwise the migrator refuses with
`migration_operation_unresolved` and a `node_correspondences` subject carrying
that node's ordinal; it never guesses an operation, division law, rounding mode,
text profile, member or leaf.

The migration result is the closed `MigrationOutcome` sum: exactly
`relinked { correspondence }` or `refused { code, subject }`. `subject` names the
offending reconstruction input and, for a source, its authority and identity,
or, for an unresolved operation, `node_correspondences` and the node ordinal.
The refusal code vocabulary is exactly, in evaluation order,
`migration_byte_only_reconstruction` (no reconstruction inputs; only V1 bytes),
`migration_input_missing` (an absent input member, an empty source list or any
V1-locked source absent), `migration_input_ambiguous`,
`migration_input_stale`, `migration_operation_unresolved` and
`migration_target_incompatible`. Every check of one
cause precedes every check of the next, so an absent locked source is reported
as missing even when another supplied source is ambiguous or stale; the first
failing check is reported and no other code exists.

The QSpec qualification crate proves only the published schemas, vectors and
receipt classification (FR-322-AC-13 through AC-21). The reader, linker and
lowering criteria remain owned by the Contract IR and QSL producer evidence
named by TC-217.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-322-AC-1 | Every admitted semantic occurrence maps to at least one original source region. | Analysis |
| FR-322-AC-2 | Changing backend capability availability cannot change package admission or package identity. | Analysis |
| FR-322-AC-3 | Changing a meaning-affecting lock member changes the package identity. | Analysis |
| FR-322-AC-4 | The public reader refuses unknown versions, malformed/duplicate members, stale dependencies, cross-domain digest substitutions, unknown required capabilities and non-canonical bytes before exposing an admitted package. | Test (TC-217) |
| FR-322-AC-5 | The canonical positive, boundary, refusal and mutation vectors distinguish raw source, definition and compiled-model byte digests from checked-semantic and generated-IR identities, even when digest bytes are equal. | Test (TC-217) |
| FR-322-AC-6 | Exact selected limits admit the boundary vector; one-over-byte, depth, node, edge, locus, diagnostic and work inputs return incomplete resource accounting and no partial checked package. | Test (TC-217) |
| FR-322-AC-7 | A consumer obtains the complete closed semantic graph through the public schema and never through a native parser/CST/package implementation type. | Inspection |
| FR-322-AC-8 | Two conformers derive the same package id from the specified JCS identity preimage; changing an excluded raw artifact/source-map/presentation or reader-limit member does not change it, while changing any named preimage member does. | Test (TC-217) |
| FR-322-AC-9 | The reader result is exactly admitted, refused or incomplete; malformed input and the first exhausted measured limit remain distinguishable and carry their specified typed provenance/accounting. | Test (TC-217) |
| FR-322-AC-10 | Enum, dimension and declared-unit node keys match the closed JCS preimage vectors; stale-body keys, non-normalized rationals, invalid owner joins and noncanonical member/term order refuse as `invalid_semantic_graph`. | Test (TC-217) |
| FR-322-AC-11 | The published V1 schema and vectors remain byte-for-byte unchanged; V1 refuses V2 members/version and V2 refuses V1 rather than silently upgrading either closed wire. | Test (TC-217) |
| FR-322-AC-12 | V1-to-V2 migration re-links and re-checks exact authoritative reconstruction inputs and returns the closed `MigrationOutcome`: `relinked` with a total typed node correspondence whose basis is `relinked-identical` exactly when node digests are equal, or `refused` with exactly one of `migration_input_missing`, `migration_input_stale`, `migration_input_ambiguous`, `migration_operation_unresolved`, `migration_target_incompatible` or `migration_byte_only_reconstruction` and its typed subject, without exposing a V2 package. | Test (TC-217) |
| FR-322-AC-13 | Each published checked-package schema admits its positive fixture carrying all 13 closed node tags with an exact source-map entry for every occurrence and refuses each authored version, digest-domain and node mutation; the frozen V1 publication bytes match independent SHA-256 locks, each schema refuses the other version, a consistently relabelled V1 graph fails the V2 schema, and each published V1 package fixture (the frozen all-families fixture and the V2-directory nominal migration source) validates against the frozen V1 schema with a `package_id` equal to the RFC 8785 JCS SHA-256 of its stored identity preimage. | Test (TC-233) |
| FR-322-AC-14 | Each published V2 package fixture's `package_id` equals the RFC 8785 JCS SHA-256 of the identity preimage rebuilt from its lock selections and occurrence-free semantic graph; editing a source-map region, node source occurrence, raw source digest, capability disposition or diagnostic catalog leaves that id unchanged, while editing the edition, a profile/definition/model/dependency selection, a required feature, a node member or node order changes it. | Test (TC-233) |
| FR-322-AC-15 | Each published enum, dimension and declared-unit node vector validates against the closed preimage schema and recomputes its recorded JCS SHA-256; each authored semantic mutation is refused by exactly its named schema or semantic check, an absent owner fails the package lock join, and inside the published nominal package a retained preimage with a changed body, type, dependency or unjoined owner is refused by the published cross-field or owner-join constraint. | Test (TC-233) |
| FR-322-AC-16 | The published migration schema admits exactly the closed `MigrationOutcome` forms; each published relinked receipt is total and ordered with basis `relinked-identical` exactly when node digests are equal, and each published refusal receipt classifies to its recorded code and subject as the first failing check in the order byte-only, missing, ambiguous, stale, operation, target. | Test (TC-233) |
| FR-322-AC-17 | The published operation catalog is closed: both V2 schemas carry identical operation definitions and enumerate exactly its identities, law roles, mode kinds and values, member kinds and operator classes; every law-role `DefinitionRef`, digest included, is a complete-value lock definition; every entry's operands, rest, result form, laws, mode, member, leaf source and constraints use only catalogued families, groups and kinds; integer `div`/`rem` require the integer-division law and `mod` none, IEEE operations require the IEEE profile and have no `eq`, `ne`, ordering or `negate` identity, text operations require the text profile and text mode, `all_instances` takes one population, `lookup` requires an absence mode, `navigate` and `reaches` a relationship end, and the record, tuple and index projections, numeric/total-order/bit-identical IEEE comparisons, `deref` and `dispatch_call` are present. | Test (TC-233) |
| FR-322-AC-18 | Each published operation vector validates against the closed node-preimage schema, recomputes its recorded JCS SHA-256 and is distinct from every other vector; every application node of the published operation package is keyed by its application-node preimage and is a published vector, and a vector outside the package refuses as `operation-law-unselected` when placed in it; floor-to-Euclidean division changes the node and package ids; a retained key, contradictory form, missing member-declaration dependency, stray recursion group or extra dependency is refused; an application without an operation fails the V2 schema; and the all-families function body calls a published function, changes its node id when rewritten as `dispatch_call` and then refuses as `operation-member-mismatch`. | Test (TC-233) |
| FR-322-AC-19 | Each published operation mutation never keeps its stale key and either rekeys to exactly its named counterpart vector and is admitted exactly when its laws are lock-selected, refuses as `operation-law-unselected` until the lock also selects its law and again under the stale selection, or is refused by exactly its named schema or semantic check with its recorded cause and that cause's code; together the mutations exercise every operation refusal cause. | Test (TC-233) |
| FR-322-AC-20 | Migration derives a V2 operation from a V1 application class exactly when the catalog has one operation for that class with no law, mode, member or leaf selection, and the derived operation equals that entry; every other class is unresolved. | Test (TC-233) |
| FR-322-AC-21 | Every node of each published V2 package carries a `declaration` member exactly when it declares; the schema refuses a declaring node without it and an expression with it; the member enters the application-node preimage and package id; a name duplicated on two nodes refuses as `ambiguous-name` at the node later in node-id digest order; and a nominal node whose declaration differs from its `qualified_declaration` refuses as `declaration-nominal-mismatch` before any ambiguity. | Test (TC-233) |

## Dependencies

- [interface-004](./interface-004-checked-package.md) owns the operation boundary.
- FR-132 owns the behavior that produces or consumes this value.
- [FR-195](../../functional/ir-proof/FR-195-lower-complete-contract-ir.md)
  consumes this boundary for exact Contract IR lowering.
- [FR-201](../foundation/FR-201-canonical-identity-domain.md) owns comparison of
  the distinct identity/digest domains.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
