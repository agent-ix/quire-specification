# Shared model and artifact references — FS02 review amendment

Status: proposed interface contract, `agent-a-fs02/2-draft`. This amendment
resolves A's proposed field meanings for B/C review. It does not register a wire
schema, complete FS02/FS03/FS05, or supply the missing #36/#54 implementation.
This repository is licensed under `AGPL-3.0-or-later`; see LICENSE.

## One owner for each boundary

| Boundary | Existing authority | Native compiler responsibility |
|---|---|---|
| Declaration grammar | Filament semantic-core FieldDecl, TypeRef, OperationDecl and ClauseRef | Parse native clause expressions; do not declare a parallel field/operation grammar. |
| Compiled definitions | Semantic IR `contractVersion: 1.1.0` type/field/operation/relationship identities, units, constraints and source loci | Retain resolved declaration identities; reject unsupported meaning instead of treating names or examples as types. |
| Package exports/imports | Existing package manifest `contractVersion: 1.0.0` | Resolve the explicitly imported public export, not any equal displayName. |
| Exact package closure | Existing package lock `contractVersion: 1.0.0` and its named fingerprint algorithm | Require a validated complete closure, exact input bytes and producer provenance. Do not recompute a different fingerprint under the old name. |
| Typed model view | Existing contract-IR #54, coordinated by C under the delivery owner | Consume its reviewed type environment; A owns native resolution/typechecking against that view. |
| Executable binding | Existing BoundPackage::from_json_bytes / #50 | A lowers qualified nodes and exact source/anchor bindings; the existing binder validates the resulting complete executable clauses. |
| Runtime populations | Identified observation producer plus the admitted state profile | Validate observation/type/universe/reference closure separately from static model validity. |
| Frame/effect contract | Exact model/operation qualification binding, reviewed with the model owner | Require its exact identity and content; runtime callers cannot expand permissions. |
| Execution plan/result/run | B's portable contracts and execution owner | Supply native subjects, dependencies and derivation refs, not another result envelope. |

The inspected Filament Rust reader exposes JSON bundle validation and normalized
IR, not a ready-to-use native typed-model environment. Its `is_valid` also admits
lossy results. A successful schema reader cannot be used as evidence that an
executable frontend mapping is qualified. C's #54 adapter still has explicit
work to perform; A must not create another authoritative model binder.

## Byte identity and semantic interpretation are separate records

Replace the ambiguous single `artifactRef.profile` field in a future registered
schema with two reusable records. B may embed the records in its owned envelopes.
This is the proposed shared seam, not an imposed replacement for B's plan/result.

`ArtifactRef` has exactly these fields:

| Field | Meaning |
|---|---|
| `refVersion` | Exact shared-reference wire identifier/version; proposed `ix.artifact-ref/2-draft`. Unknown versions refuse. |
| `kind` | Explicit artifact kind; source, model-package, linked-package, executable-projection, property, binding, snapshot, invocation, generated-artifact, run-artifact, environment, model-manifest, model-lock or dependency-closure. No implicit conversion between kinds. |
| `authority` | Explicit issuer namespace accepted by the consumer's declared authority map. A matching URI host or repository name does not confer this authority. |
| `identity` | Stable identity within that authority and artifact kind. Exact Unicode scalar sequence; no case folding or Unicode normalization. |
| `revision` | `{namespace, value}` with nonempty opaque strings. Numeric domain revisions remain explicit values in their own namespace. |
| `digest` | `sha256:` plus 64 lowercase hex digits over the exact artifact file bytes. This is always a transport/content digest. |
| `wire` | `{identity, version}` selecting the artifact's exact encoding/schema, independently of this reference record's own wire version. |

`SemanticRef` has exactly these fields:

- `artifact`: the complete ArtifactRef above;
- `language`: `{identity, edition}` for a language subject, or explicit null for
  a model/binding artifact whose meaning is owned by another declared contract;
- `semanticProfile`: `{identity, version, definitionDigest}` or explicit null
  only for a byte-only artifact with no semantic claim;
- `requiredFeatures`: unique feature identifiers interpreted by that profile;
- `canonicalIdentity`: either explicit null when no semantic canonical identity
  is claimed, or `{domain, version, algorithm, digest}` produced by that exact
  domain's established canonicalizer. It never substitutes for `artifact.digest`.

A semantic subject requires its profile; null does not mean “latest” or “default”.
The selected closed FS05 candidate requires `requiredFeatures` explicitly;
`[]` means the empty declared feature set. An omitted member refuses. This
resolves the earlier draft's ambiguous omission wording without changing the
existing fixture payloads. Unknown required features refuse.
Profile definition digests bind locally available approved definitions; a
consumer does not fetch definitions or trust arbitrary executable code implicitly.
No canonical identity is invented for the current syntax-only parser.

Equality is exact over all declared fields, with `requiredFeatures` compared
as a set after rejecting duplicates. Producers sort that set by UTF-8 bytes;
readers must not make feature order semantic. This local rule does not change
any other contract’s collection order. The immutable byte binding key is
`(kind, authority, identity, revision.namespace, revision.value)`; reuse with
changed digest or wire is an identity-content conflict. A changed semantic
interpretation is a different SemanticRef even when source bytes are equal.
An artifact path, retrieval URL, display label or timestamp is locator/provenance
metadata outside the identity record. Different generated artifacts can derive
from one authored clause without adopting that clause's identity.

The existing operation `1-draft` review fixtures retain their original bytes and
explicitly mean fixture encoding by `artifactRef.profile`. They are not silently
reinterpreted as the new record. A future conversion must emit a new wrapper
version, supply an explicit wire mapping, and add semanticProfile only from an
identified definition. B's provisional semantic `profile` likewise requires an
explicit mapping; neither old record is a registered production schema.

## Native import interpretation and model qualification

The source grammar already spells `model Alias = "package" version "version"
digest "digest"`. The proposed exact meaning of its digest is the **transport
SHA-256 of the identified compiled model IR document bytes**. It is not the
manifest digest, lock digest, source contentDigest or canonical lock fingerprint.
An associated compiler input manifest supplies separate exact ArtifactRefs for
the compiled document, manifest, lock, profiles/mappings and every locked package.
Nothing is selected by a network lookup or “latest compatible” fallback.

An import must match the package identity/version inside the compiled IR and
its manifest/lock. Filament's actual package identity is opaque `owner/name`;
semantic declaration URIs are a separate dimension. The #36 issue's illustrative
`ix://<org>/<repo>` package wording needs reconciliation with the existing
manifest contract. A will not change the manifest identity format silently.

Resolution proceeds through the explicitly imported public export name and its
stable typeIdentity. Field and operation names resolve within that declaration.
Aliases are lexical source conveniences. Two candidates with the same exported
name, or repeated stable identities with conflicting definitions, refuse. A
transitively reachable type is usable only through the explicitly permitted
export/reference closure; it is not automatically a directly imported name.

The adapter must preserve, or explicitly refuse, these distinctions:

- required presence versus optional absence versus nullable;
- lower/upper multiplicity versus ordered/unique properties;
- alias constraints and field constraints, including exact inclusive/exclusive
  bounds, rather than only primitive scalar names;
- exact UCUM unit identities; no assumed scaling or dimensional conversion;
- structural record values versus identity-bearing objects and references;
- relationship target/category/composite meaning versus an addressable field;
- operation parameter, return and current/pre/post availability;
- opaque source clause identity versus a parsed native expression or projection.

A missing upper integer bound remains unbounded for this profile. The inspected
ConfigVersion fixture only provides a minimum for VersionNumber. Its illustrative
repeated-digit digests are not verified package bytes; it cannot serve as a real
bound execution fixture. Its parent relationship also does not by itself export
a field named `parent`. Neither an implicit numeric maximum nor a relationship-
to-field mapping may be inferred from that example. A reviewed synthetic producer
may author explicit finite limits and fields, then compile and bind actual bytes.
That experiment does not complete the separate spec-bundle extraction delivery.

A finite execution qualification must identify every domain restriction and its
source. If restrictions are supplied by a separately reviewed profile binding,
retain that binding and its relationship to the authored model; never mutate the
model silently or promote a sample range into an authored constraint.

## Required adapter observations and diagnostics

The #54 interface must return validated declaration handles with their owning
package/import closure, exact stable IDs, model source loci, type/unit/
multiplicity information and supported snapshot contexts. The native compiler
adds its own precise import/name occurrence spans and resolves them to those
handles. Native ExprId values are local syntax handles, never semantic identity.

| Trigger | Required result |
|---|---|
| Required package/artifact/export absent | `missing_import`, with exact reference locus |
| Multiple export/name candidates | `ambiguous_declaration`, with every candidate locus |
| Identity/revision/version selected but differing expected content or closure | `stale_dependency` or `identity_content_conflict`, before typechecking |
| Undeclared authority, import or object universe/type | `foreign_reference` |
| Incompatible units, parameter/field type or invalid declaration constraints | `ill_typed`, with reference and declaration loci |
| Missing authored finite bound | `unbounded_domain` |
| Valid declaration construct outside qualified mapping | `unsupported_construct`, preserving source obligations |
| Wrong current/pre/post anchor or unavailable operation context | `wrong_snapshot` / `unavailable_observation` as appropriate |
| Input size, graph or validation budget exhausted | incomplete `resource_exhausted`, with no partially accepted model |

Known invalid bindings refuse. An unavailable observation stays incomplete;
missing static type authority never yields a healthy/violating value. Unknown
versions and required features refuse before an interpretation is selected.

## Review requests and acceptance evidence

B: confirm composition of ArtifactRef and SemanticRef covers property subject,
model, binding, dependency and run-artifact references without redefining your
result envelope. Define run/result selection in your owned contract; equal-time
results need an explicit unique rule or ambiguity, never incidental input order.

C: confirm the owner map, import digest meaning and package identity reconciliation
against actual #36/#54 delivery constraints. Existing Quire, IR, Quoin and TL
wire/canonical profiles keep their current bytes and version names. Existing
collection order rules stay local to each contract; trace_ids are not globally
sorted just because dependency references are a set.

Acceptance requires reviewed agreement plus concrete producer/independent-consumer
controls for each refusal row. Current parser/source and fixture-digest checks
are useful inputs, not substitutes for those tests. FS03 inference/evaluation and
FS05 canonicalization/migration vectors remain separate owned requirements.

Inspected current sources: Filament `3b75e01`, including
`docs/semantic-data-system/contracts-v1.md`,
`schema/semantic/v1/package-manifest.schema.json`, `package-lock.schema.json`,
`src/compiler/packages/lock.mjs`, `crates/semantic-ir/src/lib.rs` and
`fixtures/semantic/v1/positive/config-version-v1-1.json`; live #36/#54 issue bodies
on 2026-09-07; contract-IR `decc99a` binding API. These are pinned evidence,
not a claim that their pending delivery tickets are complete.

Concrete serialized expectations are in [reference-cases.json](fixtures/reference-cases.json):
an actual digest-bound source/profile pair, feature-set reorder equivalence and
one-axis refusal controls. These vectors do not yet represent independent
consumer execution.


## Current kernel package qualification

At Filament `3b75e01`, `packages/semantic-kernel/bundle.json` declares generation
of a semantic-core schema kernel, with publication gate not taken. Its provenance
says `published: false`; the checked-in generated TypeScript package and schema
index do not establish the planned Rust/Python targets or a domain type environment.
`losses.json` explicitly records `DefaultDecl.value` narrowed from any JSON value
to an object and required-but-possibly-empty `OperationDecl.params` collapsed to
optional presence. These losses must remain visible and cannot qualify lossless
native model binding. This package is not #36's domain extraction or #54's typed
model adapter. The TypeSpec frontend, manifest/lock logic, shared schemas and Rust
semantic-IR reader are unchanged between the earlier inspected `65ea7fa` and
`3b75e01`; A's new producer reproduction uses the actual current pinned checkout.

C's independent review accepted the corrected operation reference baseline at
`693e5c4`. The broader production reference seam remains in review. B's existing
provisional reader must explicitly adopt the new wrapper version and structured
revision and compare requiredFeatures as a duplicate-free set. A does not change
B's reader or silently treat its older exact-array comparison as agreement.


The role amendment is `agent-a-fs02/2-draft` / `ix.artifact-ref/2-draft` because
its closed artifact-kind set now admits environment, model-manifest, model-lock
and dependency-closure. The earlier `reference-cases.json` remains byte-stable
at 1-draft. New `role-compositions.json` demonstrates the complete role graph;
`reference-migrations.json` explicitly relates common-kind records and refuses
implicit conversion. B must select the new version deliberately. New kinds
have no representation in the earlier closed set.

The composition packet includes an actually generated TypeSpec model, manifest
and lock from Filament 3b75e01. Fresh and selected-lock builds produce identical
IR bytes; a stale lock refuses with no output/repair. The model is still an
unqualified input to #54. Existing operation fixtures retain their original
synthetic declaration IDs, which differ from the producer's actual field and
operation IDs; no slash/hyphen normalization or silent adoption is permitted.
Property compositions retain authored identity plus exact source regions and
positive numeric IR revision mappings. Run/native-output/environment examples
illustrate reference placement without defining B's portable runner/result schema.
