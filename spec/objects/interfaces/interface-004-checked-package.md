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
  - every V2 application term carries one closed catalogued operation identity with exactly its required lock-selected laws (a clause its lock-selected profile law and one profile operator member), type-pinned mode, member, structural or keyed-collection leaves and declared result type, bound into its application node identity and the package identity
  - every literal term carries its declared type; no application result or literal type is inferred
  - an application node retaining a stale key is refused as invalid_package with cause stale-node-key before any declaration or operation refusal
  - every named, source-declared node carries exactly one package-local declaration path, bound into its node identity; builtin and anonymous nodes carry none; library exports resolve only through it
  - migration never guesses an operation identity, law, mode, member or leaf; it derives only the closed selection-free class table, and a clause only when its profile and operator are unique
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
| `migration_operation_unresolved` | A re-linked application operation is not the operation derived from the same-class V1 application by the FR-322 selection-free class table (every other class is underivable); the subject is `node_correspondences` with the node ordinal. |
| `migration_target_incompatible` | The re-linked result is not the named V2 target package. |

None of these refusals exposes a V2 package or a correspondence.

The `read` operation is a strict, bounded admission operation. It verifies the
declared wire before graph interpretation and returns a typed refusal for an
unknown version, malformed or duplicate member, cross-domain substitution,
stale dependency, unknown capability, or graph/source-map invariant violation.
Exceeding a selected byte, depth, node, edge, locus, diagnostic, or work limit
returns an incomplete resource disposition and no admitted package.

## Operation identity

A V2 `application` term carries its operator class, its exact
`operation: {identity, laws, mode, member, leaves}`, its declared
`result_type` and its arguments: a closed
`quire.checked-operation-catalog/v1` identity from the adjacent
`operation-catalog.json`, the ordered `{role, definition}` laws the catalog
requires for that identity, its `rounding`, `text_profile` or `absence` mode,
its member (record field, tuple position, element, relationship end, dispatched
operation or type argument, each naming its declaring node, or a clause's
`profile_operator`) and one `{path, laws, mode}` leaf for every text leaf of a
structurally compared type or of a keyed collection result's element type. A
`literal` term carries its declared `type`. The operator class alone never
selects floor, truncating or Euclidean division, an IEEE rounding attribute, a
text comparison profile, an absence behaviour, a temporal or protocol operator
or a member, and no result or literal type is inferred. Types are
authoritative: a `rounding` or `text_profile` mode equals the value the operand
and result types pin. Each value law definition joins the package lock's
`definition_selections`; a clause's profile law (`temporal_profile` or
`protocol_profile`) equals the lock's `profile_selections` row of that role, and
its operator is one that profile defines. The families, groups, constraints and
FR-149 conversion classes that decide eligibility are the normative table of
[FR-322](./FR-322-checked-package-artifact.md#operation-families-groups-and-constraints):
range and rational narrowing are checked obligations, and only a decimal scale
reduction rounds.

Every node whose body contains an application is keyed by its
`quire.application-node/v1` preimage (`node_tag`, `semantic_form`,
`semantic_type`, `declaration`, `recursion`, `body`), so a changed operation,
law, mode, member, leaf, result type or declaration changes that node's id and
the package id. `read` refuses with the causes and order owned by
`quire.native.diagnostics/v1` revision `1-draft.4`:

| Order | Cause | Code |
| --- | --- | --- |
| 1 | `unknown-operation` | `invalid_package` |
| 2 | `operation-class-mismatch` | `invalid_package` |
| 3 | `operation-law-missing` | `invalid_package` |
| 4 | `operation-law-mismatch` | `invalid_package` |
| 5 | `operation-law-unselected` | `invalid_package` |
| 6 | `operation-mode-mismatch` | `invalid_package` |
| 7 | `operation-member-mismatch` | `invalid_package` |
| 8 | `operation-mode-type-mismatch` | `invalid_package` |
| 9 | `operator-ineligible` | `ill_typed` |

Causes 1 to 7 of an application precede its argument applications' refusals,
which precede its causes 8 and 9 and then its leaves. Nodes are checked in
ascending node-id digest order and applications in pre-order. Before any
operation refusal the reader checks, in order, the graph shape
(`invalid_semantic_graph`), `stale-node-key` (`invalid_package`) for a node
whose retained key is not its preimage digest, `declaration-nominal-mismatch`
(`invalid_package`) and then `ambiguous-name` (`ambiguous_declaration`) over
the nodes' `declaration` members.

## Declaration

A named, source-declared `scalar_type`, `composite_type`, `bounded_domain`,
`function`, `model`, `protocol` or `claim` node, and a non-enum `value` node,
carries `declaration: {qualified_name}` exactly when it has a `declaration`
source occurrence; builtin and anonymous nodes, expressions and every other node
do not, and no name is invented. `qualified_name` is the package-local
declaration path. It is part of the node identity. It equals a nominal node's
`qualified_declaration`, and no two nodes share it. A qualified reference
`a::Name` resolves `a` to exactly one locked package and then matches `Name`
byte for byte against that package's declaration paths. Library exports
([FR-307](../../functional/tooling/FR-307-package-reusable-semantic-libraries.md))
resolve only through this member.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-004-AC-1 | Every application term of each published V2 package names exactly one catalogued operation identity and explicitly states its laws, mode, member and leaves, all admitted by the catalog and the package lock; the frozen V1 wire refuses an application carrying an operation. | Test (TC-233) |
| interface-004-AC-2 | Changing an application's operation identity, law or absence mode changes its node id and package id and a retained key is refused; the rekeyed package is admitted, refused as `operation-law-unselected` for an unlocked law, or refused as `operator-ineligible` when the mode disagrees with the retained result type. | Test (TC-233) |
| interface-004-AC-3 | For every application node of the published control migration, a target operation the V1 class cannot derive makes `relink_v1_to_v2` refuse with `migration_operation_unresolved` and that node's ordinal, after stale-input checks and before target comparison. | Test (TC-233) |
