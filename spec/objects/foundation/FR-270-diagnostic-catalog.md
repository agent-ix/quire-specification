---
id: FR-270
title: "Diagnostic catalog"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-047
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-271
    type: contains
  - target: ix://agent-ix/quire-specification/FR-272
    type: contains
---
# FR-270: Diagnostic catalog

## Description

When a producer publishes a machine-readable classification, the producer SHALL resolve it through one selected diagnostic catalog rather than through an ad-hoc literal or a match on English message text.

FR-047 requires "a typed cause in its diagnostic catalog rather than
diagnostic message text", and FR-047-AC-2 requires that "every emitted machine
code resolves to the selected catalog; an uncatalogued code cannot be
published as a stable producer outcome". This artifact types the catalog that
requirement selects. The only catalog so far offered is the **proposed** native
diagnostic interpretation
([native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)),
which states of itself: "**Proposed, not adopted or implemented.**" It is source
material here, not an adopted selection.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| interpretation_identity | String | 1..1 | identity |
| code_vocabulary | DiagnosticCodeVocabulary | 1..1 |  |
| closed_cause_sets | TypedDiagnosticCause | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| interpretation_identity | string identity such as `quire.native.diagnostics/v1`; exactly one (proposed catalog header; Key) |


| Property | Source | Note |
| --- | --- | --- |
| Interpretation identity | Proposed catalog, header | "Interpretation identity: `quire.native.diagnostics/v1`". |
| Code vocabulary | [FR-271](FR-271-diagnostic-code-vocabulary.md) | The catalogued code spellings. |
| Closed cause sets | [FR-272](FR-272-typed-diagnostic-cause.md) | "A cause is a closed discriminated value with the payload required by its selected variant, not a free-form string map." |

Selection is separate from every other selection a response makes: the catalog
is selected "separately from its language/profile, payload wire and backend
selections", and "No clause `using` alias can select this catalog or acquire
source permissions from it" (proposed catalog). Changing the catalog "does not
change a checked clause's static meaning"; it "does change the response's
interpretation/provenance".

Extension is closed against the implementation: "A compiler enum or its
`non_exhaustive` annotation cannot extend this selection." An unknown selection, code or required variant "refuses
interpretation of that diagnostic while retaining its original bytes and
available provenance. It cannot be recast as a known failure, an empty
diagnostic list or a successful assessment" (proposed catalog).

## Identity and comparison

- **Identity kind** — *authored*. The catalog states its own interpretation
  identity in its header; it is not computed from its bytes.
- **Key** — (`interpretation identity`). Equality is exact equality of that
  identity, because the response "selects this catalog's exact identity"
  (proposed catalog). No rendering participates: FR-047-AC-1 requires that "changing/localizing
  human-readable messages leaves the same typed failure's stage, code and cause
  details unchanged", and FR-047 states "diagnostic formatting/localization
  cannot change classification". Display wording, display paths and locale are
  therefore outside the key — "Changing diagnostic wording, display paths or
  this response catalog does not change a checked clause's static meaning."
- **Key order** — by the interpretation identity's UTF-8 bytes, for collection
  comparison only. It carries no causal meaning
  and expresses no precedence, recency or preference between catalogs.
- **Causal order** — none. A catalog participates in no causal structure. Where
  a diagnostic names an upstream diagnostic, that upstream diagnostic retains
  its own identity rather than an ordering: "Related declarations and upstream
  diagnostics retain their own exact source/catalog identities rather than being
  flattened into a string" (proposed catalog).
- **Absence** — an absent identity compares only to absence. A response
  that names no catalog selection has an absent selection; it is never defaulted
  to the proposed native catalog. "Unavailable context is explicitly
  unavailable; byte zero, an empty path or a name-search match cannot masquerade
  as a located failure" (proposed catalog).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-270-AC-1 | Two catalog selections compare equal exactly when their interpretation identities are equal. | Analysis |
| FR-270-AC-2 | A response with no named catalog selection has an absent selection and receives no default catalog. | Analysis |
| FR-270-AC-3 | Changing or localizing the human-readable wording carried with a diagnostic leaves the selected catalog key unchanged. | Analysis |
| FR-270-AC-4 | A code not resolvable in the selected catalog is not published as a stable producer outcome. | Inspection |

## Resolved catalog identity

A catalog is an externally selected definition entity, not an authored native
clause. Its key is (`catalog authority`, `catalog identity`).

## Dependencies

- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md) establishes the
  catalogued-classification obligation.
- [Native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)
  is the proposed, non-adopted catalog this artifact extracts from.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Accepted complete-V1 diagnostic-catalog contract.
