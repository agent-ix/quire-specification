---
id: FR-336
title: "Independent complete-V1 conformance corpus"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_019
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-311
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-018
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: references
---

# FR-336: Independent complete-V1 conformance corpus

## Description

When the complete-V1 conformance corpus is published or executed, the producer SHALL use the immutable ConformanceCorpus contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.conformance-corpus/v1`.
- `inventory_revision` — exact 188-row scope identity.
- `vectors` — stable vector ID, capability ID, source/model/input and expected typed disposition.
- `classes` — positive, lower/upper boundary, just-outside, refusal, incomplete and metamorphic mutations.
- `oracle` — independent expected value/verdict derivation and version.
- `applicability` — required consumer types and supported semantic profiles.
- `results` — subject/run/vector outcomes and artifact references.
- `coverage` — exactly one primary capability entry plus criterion/consumer rollups.

## Identity and validation

Vector identity covers its complete input and oracle; corpus identity covers the ordered vector set and inventory revision.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-336-AC-1 | Every inventory capability has positive and adverse/boundary vectors where applicable. | Analysis |
| FR-336-AC-2 | Removing or mutating a required vector makes qualification incomplete. | Analysis |
| FR-336-AC-3 | A producer cannot qualify itself solely with an oracle sharing its implementation. | Analysis |

## Dependencies

- [interface_019](./interface_019-conformance-corpus.md) owns the operation boundary.
- FR-311 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
