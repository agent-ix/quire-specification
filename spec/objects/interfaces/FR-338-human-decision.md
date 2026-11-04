---
id: FR-338
title: "Attributed human decision record"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_021
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-311
    type: references
---

# FR-338: Attributed human decision record

## Description

When a human accepts, rejects or limits a qualification or release scope, the producer SHALL use the immutable HumanDecision contract below rather than an untyped map, display string or implementation-local structure.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| contract_version | String | 1..1 |  |
| decision_id | String | 1..1 |  |
| actor | JsonObject | 1..1 |  |
| scope | JsonObject | 1..1 |  |
| evidence | String | 1..1 |  |
| disposition | String | 1..1 |  |
| rationale | String | 1..1 |  |
| supersedes | String | 0..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| contract_version | exactly `quire.human-decision/v1` |
| actor | record: authenticated human identity and authority reference; multiplicity not stated in source |
| scope | record: qualification, release, capability, artifact identities |
| evidence | type not stated in source; multiplicity not stated in source; see Field sources |
| disposition | string tag: accept, reject or accept-with-explicit-limits; multiplicity not stated in source |
| rationale | retained human-authored text; multiplicity not stated in source |
| supersedes | optional prior `decision_id` string (compared as exact string) |


- `contract_version` — exactly `quire.human-decision/v1`.
- `decision_id` — the identity assigned to the decision when it is recorded, compared as an exact string.
- `actor` — authenticated human identity and authority reference.
- `scope` — exact qualification, release, capability and artifact identities.
- `evidence` — immutable evidence set reviewed.
- `disposition` — accept, reject or accept-with-explicit-limits.
- `rationale` — retained human-authored explanation.
- `supersedes` — optional prior decision identity; prior bytes remain immutable.

## Identity and validation

The decision key is `decision_id`. It is not computed from the other members: no identity is derived from the bytes of actor, authority, scope, evidence, disposition, rationale or supersession.
A document whose `contract_version` is not that identifier, missing required
properties, duplicate keys, out-of-domain values and non-canonical encodings
refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-338-AC-1 | An unauthenticated actor or authority/scope mismatch refuses recording. | Analysis |
| FR-338-AC-2 | A decision cannot claim broader scope than its evidence and explicit limits. | Analysis |
| FR-338-AC-3 | Supersession links a new immutable decision and never mutates the prior record. | Analysis |

## Dependencies

- [interface_021](./interface_021-human-decision.md) owns the operation boundary.
- FR-311 owns the behavior that produces or consumes this value.
