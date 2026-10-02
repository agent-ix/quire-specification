---
id: FR-274
title: "Diagnostic location kind"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-047
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-203
    type: references
  - target: ix://agent-ix/quire-specification/FR-207
    type: references
---
# FR-274: Diagnostic location kind

## Description

When a diagnostic reports where a failure occurred, the reader SHALL resolve the locus under exactly one of the three stated location kinds rather than comparing a locus of one kind against a locus of another.

The proposed catalog states the distinction: "Required context is the affected
request/subject, exact selections that were available, original source region
and typed input location where known. An authored source span, a JSON
field/index path and a runtime object/value path remain different location
kinds"
([native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md),
"Common structured context"). FR-047-AC-4 requires that "original
source/model/clause loci and identities remain attached where available;
diagnostics do not reconstruct a location by searching for a repeated name". The
catalog is proposed, not adopted.

## Values

| Value | Description |
| --- | --- |
| authored source span | A span over authored source. Typed by [FR-203](FR-203-source-region.md), which this artifact references rather than redefining: a half-open byte region over one identified source document. |
| JSON field/index path | A path through a JSON document's fields and indices. Required as the retained payload of the `invalid_package` and `invalid_runtime_input` families — "the actual field/index path", "the actual input path" (proposed catalog). |
| runtime object/value path | A root and an ordered sequence of typed traversal steps through a runtime object or value. Typed by [FR-207](FR-207-runtime-value-path.md), which this artifact references rather than redefining. Held distinct from a JSON field/index path by [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md) and by the same sentence of the proposed catalog. |

The complete-V1 wire discriminators are `source_span`, `json_path` and
`runtime_value`. FR-203 types the authored source span, FR-207 types the runtime
object/value path, and the JSON field/index path is typed below.

## Identity and comparison

- **Identity kind** — *value*. A location kind has no identity of its own; it is
  compared componentwise.
- **Key** — the kind, taken together with the locus it discriminates. Equality
  is componentwise over exactly these and nothing else. Two loci of different
  kinds are never equal and are not compared to one another, because the three
  "remain different location kinds" (proposed catalog). Rendering never
  participates: FR-047-AC-1 requires that "changing/localizing human-readable
  messages leaves the same typed failure's stage, code and cause details
  unchanged", FR-047 states "Diagnostic formatting/localization cannot change
  classification", and the catalog states that changing "display paths" does not
  change a checked clause's static meaning. A display path is therefore not a
  location.
- **Key order** — for the `authored source span` kind, the key order is
  FR-203's: lexicographic over (`document` key, `start`, `end`), for collection
  comparison only. For the `runtime object/value path` kind it is FR-207's: root
  first, then step by step, a proper prefix before its extensions, total across
  authority-qualified selected observations.
  Across kinds the order is lexicographic over the kind first,
  for collection comparison only. It carries no causal meaning and expresses no
  severity or precedence ordering between kinds or between loci.
- **Causal order** — none. FR-203 records that FR-052 constructs the causal
  graph "without deriving causal edges from source order", so an earlier span is
  not a causal predecessor. An upstream diagnostic carrying its own locus is a
  referenced identity, not an ordering: "Related declarations and upstream
  diagnostics retain their own exact source/catalog identities rather than being
  flattened into a string" (proposed catalog).
- **Absence** — an unavailable locus is explicitly absent and compares only to
  absence. The catalog is explicit: "Unavailable context is explicitly
  unavailable; byte zero, an empty path or a name-search match cannot masquerade
  as a located failure." FR-047-AC-4 adds that diagnostics "do not reconstruct a
  location by searching for a repeated name". Byte zero is not an absent span,
  an empty path is not an absent path, and a name match is not a locus.
- **Digest domain** — a location kind carries no digest. Where a span's document
  carries one, its domain rule is
  [FR-201](FR-201-canonical-identity-domain.md); the catalog that
  resolves the diagnostic is itself selected by identity, revision and byte
  digest ([FR-270](FR-270-diagnostic-catalog.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-274-AC-1 | An authored source span, a JSON field/index path and a runtime object/value path are not compared to one another. | Inspection |
| FR-274-AC-2 | An unavailable locus compares only to absence and is not represented as byte zero, an empty path or a name-search match. | Analysis |
| FR-274-AC-3 | Changing a diagnostic's display path leaves its located kind and locus unchanged. | Analysis |
| FR-274-AC-4 | The authored source span kind resolves through FR-203 and is not redefined here. | Inspection |
| FR-274-AC-5 | The runtime object/value path kind resolves through FR-207 and is not redefined here. | Inspection |

## Canonical discriminators and JSON path

The wire discriminators are `source_span`, `json_path` and `runtime_value`.
A JSON path contains a document identity and ordered steps tagged `field` with
an exact member spelling or `index` with a nonnegative integer. Equality and
ordering are componentwise: document, then steps; a proper prefix orders first;
`field` orders before `index`; equal kinds order by spelling or integer. Unknown
discriminators/steps refuse. These wire values never alias display paths.

## Dependencies

- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md) establishes the
  locus-retention obligation and the prohibition on reconstructing a location.
- [Native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)
  is the proposed, non-adopted source of the three distinct location kinds.
- [FR-203](FR-203-source-region.md) types the authored source span
  kind.
- [FR-207](FR-207-runtime-value-path.md) types the runtime object/value path
  kind.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Accepted complete-V1 diagnostic-location contract. Implementation and
qualification remain tracked separately.
