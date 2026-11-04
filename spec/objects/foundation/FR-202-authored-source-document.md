---
id: FR-202
title: "Authored source document"
type: FR
object: entity
relationships:
  - target: ix://agent-ix/quire-specification/FR-003
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-036
    type: specifies
---
# FR-202: Authored source document

## Description

When a clause location is exchanged, the producer SHALL bind it to one authored source document identified by its document identity rather than by its path or its name.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| document_identity | String | 1..1 | identity |
| bytes | Bytes | 1..1 |  |
| segment_map | String | 0..1 |  |
| extracted_body | String | 0..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| document_identity | type not stated in source; see Field sources |
| segment_map | type not stated in source; see Field sources |
| extracted_body | type not stated in source; see Field sources |


- **document_identity** (identity) — the original document identity carried with
  every exchanged clause location
  ([FR-003](../../functional/foundation/FR-003-bind-source-correspondence.md) Inputs).
- **bytes** (content) — the original UTF-8 source bytes and exact identity
  consumed by lexical recognition
  ([FR-036](../../functional/foundation/FR-036-retain-lexical-source-locations.md) Inputs).
- **segment_map** (optional) — may delete only the explicitly selected
  indentation, CRLF and final-newline layout (FR-003). A segment map that
  deletes an interior source token receives `invalid_source_map` (FR-003-AC-3).
- **extracted_body** (optional) — the extractor treats expressions as opaque
  (FR-003).

Comments remain part of source identity and consume input budget even when they
produce no semantic tokens (FR-036).

## Identity and comparison

- **Identity kind** — *authored*. The document is written by a person; its
  identity is carried in the exchange, not computed from its content.
- **Key** — `document_identity`. Equality is exact over this one member. The file path, the display name and
  any rendering of the document never participate: FR-003 binds a location to
  the document identity, and FR-047-AC-4 requires that diagnostics "do not
  reconstruct a location by searching for a repeated name".
- **Key order** — lexicographic over `document_identity`. It
  exists to compare and diff document collections. It carries no precedence
  and no causal meaning.
- **Causal order** — none. A document participates in no causal structure;
  FR-052 forbids deriving causal edges from source order.
- **Absence** — an absent document identity compares only to absence.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-202-AC-1 | Two source documents with identical bytes and different document identities compare unequal. | Analysis |
| FR-202-AC-2 | A document key compares equal only when the document identities compare equal. | Analysis |
| FR-202-AC-3 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-202-AC-4 | Path and display name are absent from the key. | Inspection |

## Dependencies

- [FR-003](../../functional/foundation/FR-003-bind-source-correspondence.md) establishes the
  document identity and the segment-map limits.
- [FR-036](../../functional/foundation/FR-036-retain-lexical-source-locations.md)
  establishes the original-bytes input and comment retention.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
