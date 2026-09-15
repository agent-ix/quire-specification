---
id: FR-325
title: "Observation authority bundle"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface-007
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-160
    type: references
---

# FR-325: Observation authority bundle

## Description

When observation facts are supplied to a semantic consumer, the producer SHALL use the immutable ObservationAuthorityBundle contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.observation-authority/v1`.
- `authority` — producer, scope and contract identity.
- `revision` — immutable revision and explicit predecessor/supersession relation.
- `records` — typed subject, occurrence, value and availability facts.
- `populations` — membership scope, members and completeness authority.
- `positions` — semantic order/clock coordinates distinct from ingest order.
- `progress` — observed frontier, closure and late-data policy.
- `conflicts` — duplicate, contradiction and unresolved-correlation records.

## Identity and validation

The bundle key is (`authority`, `scope`, `revision`). Timestamp, filename and ingestion position never substitute.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-325-AC-1 | Every fact names its authority-qualified subject and observation identity. | Analysis |
| FR-325-AC-2 | Unknown membership, progress or availability remains explicit. | Analysis |
| FR-325-AC-3 | A new revision supersedes an old revision only through the declared relation. | Analysis |

## Dependencies

- [interface-007](./interface-007-observation-authority.md) owns the operation boundary.
- FR-160 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
