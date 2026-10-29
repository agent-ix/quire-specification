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
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
---
# FR-202: Authored source document

## Description

When a clause location is exchanged, the producer SHALL bind it to one authored source document identified by its document identity and digest rather than by its path or its name.

## Properties

- **document_identity** (identity) — the original document identity carried with
  every exchanged clause location
  ([FR-003](../../functional/foundation/FR-003-bind-source-correspondence.md) Inputs).
- **digest** (identity component) — the original document digest. It is a raw
  artifact digest, which hashes the complete supplied bytes including layout
  ([FR-018](../../functional/foundation/FR-018-select-verification-jcs-domain.md)), and it
  carries the domain in which it was derived
  ([FR-201](FR-201-canonical-identity-domain.md)).
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
- **Key** — (`document_identity`, `digest`). Equality is
  componentwise over exactly these two. The file path, the display name and
  any rendering of the document never participate: FR-003 binds a location to
  the document identity, and FR-047-AC-4 requires that diagnostics "do not
  reconstruct a location by searching for a repeated name".
- **Key order** — lexicographic over (`document_identity`, `digest`). It
  exists to compare and diff document collections. It carries no precedence
  and no causal meaning.
- **Causal order** — none. A document participates in no causal structure;
  FR-052 forbids deriving causal edges from source order.
- **Absence** — an absent digest compares only to absence. It is never
  defaulted to the empty digest. A changed
  definition digest prevents reuse of the earlier semantic subject
  ([FR-001](../../functional/foundation/FR-001-select-native-profile.md)-AC-3).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-202-AC-1 | Two source documents with identical bytes and different document identities compare unequal. | Analysis |
| FR-202-AC-2 | A document key compares equal only when identity and digest both compare equal. | Analysis |
| FR-202-AC-3 | An absent digest compares only to absence and receives no default digest. | Analysis |
| FR-202-AC-4 | Path and display name are absent from the key. | Inspection |

## Dependencies

- [FR-003](../../functional/foundation/FR-003-bind-source-correspondence.md) establishes the
  identity and digest pair and the segment-map limits.
- [FR-036](../../functional/foundation/FR-036-retain-lexical-source-locations.md)
  establishes the original-bytes input and comment retention.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
