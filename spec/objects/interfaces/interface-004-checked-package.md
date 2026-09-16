---
id: interface-004
title: "I04 immutable checked package"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
---

# [interface-004] I04 immutable checked package

## Contract

```yaml
name: CheckedPackageArtifactV2
associated_types:
  - CheckedPackageId
  - CheckedSemanticGraphV2
  - CheckedSemanticNodePreimageV1
  - CheckedOperationCatalogV1
  - PackageLockV2
  - CheckedPackageArtifactV1
  - CheckedPackageMigrationCorrespondenceV1
  - CheckedPackageMigrationOutcomeV1
  - SourceDocumentRef
  - DefinitionArtifactRef
  - ModelSelectionRef
  - SourceMapV2
  - CapabilityReportV2
  - DiagnosticReportV2
  - CheckedPackageReadLimits
operations:
  - { name: link_and_check, inputs: [ParsedSource, ModelSelection, profile bundle], output: CheckedPackageArtifactV2 or located refusals, semantics: resolves exact definitions types dependencies nominal identities and capabilities }
  - { name: read, inputs: [canonical CheckedPackageArtifactV2 bytes, CheckedPackageReadLimits], output: immutable checked package or typed refusal/incomplete result, semantics: verifies the selected wire, closed structure, canonical bytes, domain-tagged identities and bounded graph before consumption }
  - { name: relink_v1_to_v2, inputs: [CheckedPackageArtifactV1 reference, exact authored source documents, selected definition artifacts, selected compiled-model documents, CheckedPackageReadLimits], output: CheckedPackageArtifactV2 plus a relinked CheckedPackageMigrationOutcomeV1 or a refused CheckedPackageMigrationOutcomeV1 with code and subject or an incomplete result, semantics: reruns V2 linking and checking from authoritative reconstruction inputs; never transforms or fills V1 bytes }
  - { name: inspect, inputs: [CheckedPackageArtifactV2], output: immutable declarations capabilities and diagnostics, semantics: does not execute or lower }
invariants:
  - runtime inputs do not participate in static package identity
  - backend support does not alter admission
  - source bytes, definition bytes, compiled-model documents, checked semantic graph and generated IR occupy distinct typed identity and digest domains
  - the public semantic graph is a closed versioned union owned by this interface; consumers do not receive parser/CST/package implementation objects
  - complete-V1 nominal scalar node keys use the closed node-identity preimages and known vectors published with the I04 schema
  - the frozen V1 wire remains byte-for-byte unchanged and is never reinterpreted as V2
  - V1 to V2 migration emits no V2 package when an authoritative reconstruction input is missing stale ambiguous or incompatible or only V1 bytes are supplied
  - a migration refusal carries exactly one closed code (migration_input_missing, migration_input_stale, migration_input_ambiguous, migration_operation_unresolved, migration_target_incompatible, migration_byte_only_reconstruction) and a typed input subject
  - every V2 application term carries one closed catalogued operation identity with exactly its required lock-selected laws and mode, bound into its expression node identity and the package identity
  - migration never guesses an operation identity, law or mode that the V1 application class and V1 lock do not determine exactly
  - migration never synthesizes nominal identity evidence from V1 checked-package bytes
  - a reader retains invalid unsupported incomplete and failed dispositions; it cannot replace one with an empty graph or a successful package
```

## Public boundary

`quire.checked-package/v2` is the complete-V1 I04 interchange emitted for new
packages. Its normative closed schema and independently authored vectors are
published under
[`proposals/checked-package-v2`](../../../proposals/checked-package-v2/). The
schema defines the transport form; the grammar and the linked normative
requirements retain authority for language meaning. This boundary deliberately
does not expose a parser CST, a filesystem path, a display string, or an
implementation-private native package structure.

The previously frozen
[`quire.checked-package/v1`](../../../proposals/checked-package-v1/) contract
remains unchanged. Its closed node objects cannot carry the nominal preimages
needed to validate enum, dimension and declared-unit identities, so it is not
silently widened. A reader selects V1 or V2 explicitly; a V1 reader refuses V2,
and a V2 reader does not reinterpret V1 bytes as V2.

The adjacent `node-identity-preimage.schema.json` and
`node-identity-vectors.json` are normative parts of this version. They close the
JCS preimages for enum declarations/members, dimensions and declared units
without making a whole package identity part of a nominal type identity.

The adjacent `migration-correspondence.schema.json` and
`migration-correspondence-vectors.json` define the separate V1-to-V2 migration
receipt. Migration accepts a V1 package only as the selected source reference;
it requires the exact source, definition and compiled-model artifacts named by
that package's lock and reruns V2 linking/checking. Success returns a separately
identified V2 package plus a correspondence record containing both package keys
and one ordered mapping for every source V1 node. Each mapping records either
`relinked-identical` or `relinked-rekeyed`. The correspondence is evidence about
the two packages and is excluded from both identities. A row's basis is
`relinked-identical` exactly when its source and target node digests are equal;
the canonical `nominal-relinked-rekeyed` vector re-links a V1 package whose
enum, dimension and unit keys differ from their preimage-derived V2 keys.
V1 package bytes alone never authorize inference of nominal owner, qualified
declaration, enum-member order, dimension terms or unit conversion.

`relink_v1_to_v2` returns the closed `MigrationOutcome` sum defined in
`migration-correspondence.schema.json`: `relinked { correspondence }` or
`refused { code, subject }`, never both. `subject` names the offending
reconstruction `input` (`reconstruction_inputs`, `sources`, `edition`,
`profile_selections`, `definition_selections`, `model_selections`,
`dependency_selections`, `node_correspondences` or `target_package_id`) plus, for a source, its
authority and identity. The refusal `code` vocabulary is closed and each cause
has exactly one code; a migrator evaluates them in this order, completing every
check of one cause before any check of the next, and reports the first:

| Code | Cause |
| --- | --- |
| `migration_byte_only_reconstruction` | Only V1 checked-package bytes are supplied; no reconstruction inputs exist. |
| `migration_input_missing` | A reconstruction input member is absent, the source list is empty, or any V1-locked source is absent. |
| `migration_input_ambiguous` | Two supplied sources share authority and identity. |
| `migration_input_stale` | A supplied input differs from the exact V1 lock selection. |
| `migration_operation_unresolved` | A re-linked application operation (class pairing or law) is not exactly derivable from the V1 node and V1 lock; the subject is `node_correspondences` with the node ordinal. |
| `migration_target_incompatible` | The re-linked result is not the named V2 target package. |

None of these refusals exposes a V2 package or a correspondence.

The `read` operation is a strict, bounded admission operation. It verifies the
declared wire before graph interpretation and returns a typed refusal for an
unknown version, malformed or duplicate member, cross-domain substitution,
stale dependency, unknown capability, or graph/source-map invariant violation.
Exceeding a selected byte, depth, node, edge, locus, diagnostic, or work limit
returns an incomplete resource disposition and no admitted package.

## Operation identity

A V2 `application` term carries its exact `operation`: a closed
`quire.checked-operation-catalog/v1` identity from the adjacent
`operation-catalog.json`, the ordered `{role, definition}` laws the catalog
requires for that identity and its `rounding` or `text_profile` mode. The
operator class alone never selects floor, truncating or Euclidean division, an
IEEE rounding attribute or a text comparison profile. Each law definition must
join the package lock's `definition_selections`. An expression node whose body
is an application is keyed by its `quire.expression-application-node/v1`
preimage, so a changed operation, law or mode changes that node's id and the
package id. `read` refuses an unknown operation, a class, law or mode that
contradicts the catalog, a law the lock does not select, or operands outside the
catalogued positions as `invalid_semantic_graph` with the catalogued cause named
by [FR-322](FR-322-checked-package-artifact.md).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-004-AC-1 | Every V2 application term names exactly one catalogued operation identity with its required lock-selected laws and mode, and the V1 wire cannot carry one. | Test (TC-233) |
| interface-004-AC-2 | Changing an expression application's operation identity, law or mode changes its node id and package id; retaining the old key or selecting an unlocked law refuses. | Test (TC-233) |
| interface-004-AC-3 | `relink_v1_to_v2` refuses with `migration_operation_unresolved` after stale-input checks and before target comparison whenever a re-linked operation is not exactly derivable from the V1 node and lock. | Test (TC-233) |
