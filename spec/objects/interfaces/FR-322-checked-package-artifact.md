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
`invalid_semantic_graph`, `invalid_source_map` and `unsupported_node_tag`.
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
not migration.

The migration result is the closed `MigrationOutcome` sum: exactly
`relinked { correspondence }` or `refused { code, subject }`. `subject` names the
offending reconstruction input and, for a source, its authority and identity.
The refusal code vocabulary is exactly, in evaluation order,
`migration_byte_only_reconstruction` (no reconstruction inputs; only V1 bytes),
`migration_input_missing` (an absent input member, an empty source list or any
V1-locked source absent), `migration_input_ambiguous`,
`migration_input_stale` and `migration_target_incompatible`. Every check of one
cause precedes every check of the next, so an absent locked source is reported
as missing even when another supplied source is ambiguous or stale; the first
failing check is reported and no other code exists.

The QSpec qualification crate proves only the published schemas, vectors and
receipt classification (FR-322-AC-13 through AC-16). The reader, linker and
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
| FR-322-AC-12 | V1-to-V2 migration re-links and re-checks exact authoritative reconstruction inputs and returns the closed `MigrationOutcome`: `relinked` with a total typed node correspondence whose basis is `relinked-identical` exactly when node digests are equal, or `refused` with exactly one of `migration_input_missing`, `migration_input_stale`, `migration_input_ambiguous`, `migration_target_incompatible` or `migration_byte_only_reconstruction` and its typed subject, without exposing a V2 package. | Test (TC-217) |
| FR-322-AC-13 | Each published checked-package schema admits its positive fixture carrying all 13 closed node tags with an exact source-map entry for every occurrence and refuses each authored version, digest-domain and node mutation; the frozen V1 publication bytes match independent SHA-256 locks, each schema refuses the other version, a consistently relabelled V1 graph fails the V2 schema, and each published V1 package fixture (the frozen all-families fixture and the V2-directory nominal migration source) validates against the frozen V1 schema with a `package_id` equal to the RFC 8785 JCS SHA-256 of its stored identity preimage. | Test (TC-233) |
| FR-322-AC-14 | Each published V2 package fixture's `package_id` equals the RFC 8785 JCS SHA-256 of the identity preimage rebuilt from its lock selections and occurrence-free semantic graph; editing a source-map region, node source occurrence, raw source digest, capability disposition or diagnostic catalog leaves that id unchanged, while editing the edition, a profile/definition/model/dependency selection, a required feature, a node member or node order changes it. | Test (TC-233) |
| FR-322-AC-15 | Each published enum, dimension and declared-unit node vector validates against the closed preimage schema and recomputes its recorded JCS SHA-256; each authored semantic mutation is refused by exactly its named schema or semantic check, an absent owner fails the package lock join, and inside the published nominal package a retained preimage with a changed body, type, dependency or unjoined owner is refused by the published cross-field or owner-join constraint. | Test (TC-233) |
| FR-322-AC-16 | The published migration schema admits exactly the closed `MigrationOutcome` forms; each published relinked receipt is total and ordered with basis `relinked-identical` exactly when node digests are equal, and each published refusal receipt classifies to its recorded code and subject as the first failing check in the order byte-only, missing, ambiguous, stale, target. | Test (TC-233) |

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
