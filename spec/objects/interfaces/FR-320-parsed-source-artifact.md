---
id: FR-320
title: "Lossless parsed-source artifact"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_002
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-302
    type: references
---

# FR-320: Lossless parsed-source artifact

## Description

When a parsed source crosses the parser boundary, the producer SHALL use the immutable ParsedSourceArtifact contract below rather than an untyped map, display string or implementation-local structure.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| contract_version | String | 1..1 |  |
| source | AuthoredSourceDocument | 1..1 |  |
| edition | String | 1..1 |  |
| tokens | JsonObject | 0..* |  |
| trivia | JsonObject | 0..* |  |
| nodes | JsonObject | 0..* |  |
| diagnostics | DiagnosticCatalog | 1..1 |  |
| recoveries | JsonObject | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| contract_version | exactly `quire.parsed-source/v1` |
| edition | identity string (retained text: selected language edition identity) |
| tokens | ordered list of token records: kind, byte region, original bytes |
| trivia | ordered list of byte-region records |
| nodes | ordered list of CST node records: kind, byte region, child indices, recovery state |
| diagnostics | multiplicity not stated in source; see Field sources |
| recoveries | list of recovery records with source loci; multiplicity not stated in source |


- `contract_version` — exactly `quire.parsed-source/v1`.
- `source` — exact FR-202 source-document identity.
- `edition` — the selected language edition identity.
- `tokens` — ordered token kind, exact byte region and original bytes.
- `trivia` — ordered whitespace/comment byte regions retained without normalization.
- `nodes` — ordered CST nodes with kind, byte region, child indices and recovery state.
- `diagnostics` — FR-270 catalog-qualified diagnostics.
- `recoveries` — inserted/deleted/reclassified tokens with exact source loci.

## Identity and validation

The artifact key is (`source`, `edition`, canonical tokens/trivia/nodes/recoveries). Node indices are artifact-local and never source or semantic identities.
A document whose `contract_version` is not that identifier, missing required
properties, duplicate keys, out-of-domain values and non-canonical encodings
refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-320-AC-1 | Serializing an unrecovered artifact reproduces the exact source bytes. | Analysis |
| FR-320-AC-2 | A recovery records every synthetic or discarded token and cannot claim semantic admission. | Analysis |
| FR-320-AC-3 | Changing trivia changes the parsed artifact while unchanged mapped nodes may retain only correspondence scoped to the unchanged source. | Analysis |

## Dependencies

- [interface_002](./interface_002-parsed-source.md) owns the operation boundary.
- FR-302 owns the behavior that produces or consumes this value.
