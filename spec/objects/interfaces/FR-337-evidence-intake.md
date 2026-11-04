---
id: FR-337
title: "Immutable evidence intake record"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_020
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-310
    type: references
---

# FR-337: Immutable evidence intake record

## Description

When a producer result enters the assurance evidence store, the producer SHALL use the immutable EvidenceIntake contract below rather than an untyped map, display string or implementation-local structure.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| contract_version | String | 1..1 |  |
| evidence_id | String | 1..1 |  |
| subjects | String | 1..1 |  |
| results | String | 1..1 |  |
| artifacts | String | 1..1 |  |
| traceability | String | 1..1 |  |
| rights | String | 1..1 |  |
| limitations | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| contract_version | type not stated in source; see Field sources |
| subjects | type not stated in source; see Field sources |
| results | type not stated in source; multiplicity not stated in source; see Field sources |
| artifacts | type not stated in source; multiplicity not stated in source; see Field sources |
| traceability | type not stated in source; multiplicity not stated in source; see Field sources |
| rights | type not stated in source; multiplicity not stated in source; see Field sources |
| limitations | type not stated in source; multiplicity not stated in source; see Field sources |


- `contract_version` — exactly `quire.evidence-intake/v1`.
- `evidence_id` — the evidence's identifier, assigned at intake and compared as an exact string.
- `subjects` — exact requirement, semantic, implementation and run identities.
- `results` — immutable native/method/qualification result references.
- `artifacts` — retained bytes, media type and availability.
- `traceability` — typed verifies/implements/qualifies/supersedes relationships.
- `rights` — provenance, license, disclosure and retention metadata.
- `limitations` — missing/redacted/non-replayable material and its claim impact.

## Identity and validation

Evidence identity never substitutes for any subject, result, artifact or run identity. Intake is append-only; corrections supersede.
A document whose `contract_version` is not that identifier, missing required
properties, duplicate keys, out-of-domain values and non-canonical encodings
refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-337-AC-1 | Unknown types or cross-kind identity reuse refuses intake. | Analysis |
| FR-337-AC-2 | Missing replay material remains an explicit limitation and cannot support a broader claim. | Analysis |
| FR-337-AC-3 | Intake validates producer bytes but does not execute or reinterpret the producer. | Analysis |

## Dependencies

- [interface_020](./interface_020-evidence-intake.md) owns the operation boundary.
- FR-310 owns the behavior that produces or consumes this value.
