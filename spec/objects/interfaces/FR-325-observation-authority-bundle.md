---
id: FR-325
title: "Observation authority bundle"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_007
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-160
    type: references
---

# FR-325: Observation authority bundle

## Description

When observation facts are supplied to a semantic consumer, the producer SHALL use the immutable ObservationAuthorityBundle contract below rather than an untyped map, display string or implementation-local structure.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| contract_version | String | 1..1 |  |
| authority | String | 1..1 |  |
| revision | String | 1..1 |  |
| records | String | 1..1 |  |
| populations | String | 1..1 |  |
| positions | String | 1..1 |  |
| progress | String | 1..1 |  |
| conflicts | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| contract_version | type not stated in source; see Field sources |
| authority | type not stated in source; multiplicity not stated in source; see Field sources |
| revision | type not stated in source; multiplicity not stated in source; see Field sources |
| records | type not stated in source; multiplicity not stated in source; see Field sources |
| populations | type not stated in source; multiplicity not stated in source; see Field sources |
| positions | type not stated in source; multiplicity not stated in source; see Field sources |
| progress | type not stated in source; multiplicity not stated in source; see Field sources |
| conflicts | type not stated in source; multiplicity not stated in source; see Field sources |


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
A document whose `contract_version` is not that identifier, missing required
properties, duplicate keys, out-of-domain values and non-canonical encodings
refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-325-AC-1 | Every fact names its authority-qualified subject and observation identity. | Analysis |
| FR-325-AC-2 | Unknown membership, progress or availability remains explicit. | Analysis |
| FR-325-AC-3 | A new revision supersedes an old revision only through the declared relation. | Analysis |

## Dependencies

- [interface_007](./interface_007-observation-authority.md) owns the operation boundary.
- FR-160 owns the behavior that produces or consumes this value.
