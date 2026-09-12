---
id: FR-203
title: "Source region"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-003
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-036
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-047
    type: specifies
---
# FR-203: Source region

## Description

When a producer reports a location in authored source, the producer SHALL express it as a half-open byte region over one identified source document rather than as a reconstructed position.

## Properties

- **document** (`FR-202` reference) — the authored source document the region
  indexes into. A region is meaningless without it.
- **start** (`byte offset`) — zero-based
  ([FR-003](../../functional/FR-003-bind-source-correspondence.md)).
- **end** (`byte offset`) — zero-based and exclusive; regions are half-open
  (FR-003, and [FR-036](../../functional/FR-036-retain-lexical-source-locations.md)
  "original half-open byte spans").
- **column** (`scalar column`, where reported) — one-based (FR-003).

A region must not split a UTF-8 scalar: a span that does receives
`invalid_source_map` (FR-003-AC-2). Decoding JSON string escapes does not
replace original spelling or offsets (FR-036).

## Identity and comparison

- **Identity kind** — *value*. A region has no identity of its own and is
  compared componentwise.
- **Key** — (`document`, `start`, `end`). Equality is componentwise over exactly
  these. The rendered text inside the region, the message that reports it, its
  locale and its formatting never participate: FR-047-AC-1 requires that
  changing or localizing human-readable messages leaves the same typed failure's
  stage, code and cause details unchanged.
- **Key order** — lexicographic over (`document` key, `start`, `end`), for
  collection comparison only. It carries no causal meaning: FR-052 constructs the
  causal graph "without deriving causal edges from source order", so an earlier
  region is not a causal predecessor.
- **Causal order** — none.
- **Absence** — a region that is not available is explicitly absent. Byte zero,
  an empty path or a name-search match cannot stand in for it
  ([native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)),
  and FR-047-AC-4 requires that diagnostics do not reconstruct a location by
  searching for a repeated name.
- **Digest domain** — not applicable; the region carries no digest. Its document
  does ([FR-202](FR-202-authored-source-document.md)).

## Location kinds are distinct

An authored source span, a JSON field or index path, and a runtime object or
value path remain different location kinds and are not interchangeable
([native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)).
This artifact types the first kind only.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-203-AC-1 | Two regions with equal offsets over different documents compare unequal. | Analysis |
| FR-203-AC-2 | An unavailable region compares only to absence and is not represented as offset zero. | Analysis |
| FR-203-AC-3 | Changing the human-readable message that reports a region leaves the region's key unchanged. | Analysis |
| FR-203-AC-4 | A source span, a JSON field/index path and a runtime value path are not compared to one another. | Inspection |

## Open gap

Two requirements state the span convention and they do not state the same
amount, and neither cross-references the other:

- FR-003: "Regions use zero-based half-open UTF-8 byte offsets and one-based
  scalar columns", and "A map may delete only the explicitly selected
  indentation/CRLF/final-newline layout."
- FR-036: "Recognized token values with original half-open byte spans", and "No
  normalization, newline conversion or implicit semicolon insertion changes
  source interpretation."

FR-036 states half-openness but neither the zero-basing nor the column
convention. Whether FR-003's permitted CRLF and indentation deletion is inside
or outside FR-036's prohibition on newline conversion is not stated anywhere.
This artifact records both readings and resolves neither.

## Dependencies

- [FR-003](../../functional/FR-003-bind-source-correspondence.md) and
  [FR-036](../../functional/FR-036-retain-lexical-source-locations.md) establish
  the region conventions.
- [FR-202](FR-202-authored-source-document.md) supplies the document identity.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity discipline.
