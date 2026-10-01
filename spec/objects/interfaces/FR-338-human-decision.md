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

- `contract_version` — exactly `quire.human-decision/v1`.
- `decision_id` — domain-separated identity over all fields except itself.
- `actor` — authenticated human identity and authority reference.
- `scope` — exact qualification, release, capability and artifact identities.
- `evidence` — immutable evidence set reviewed.
- `disposition` — accept, reject or accept-with-explicit-limits.
- `rationale` — retained human-authored explanation.
- `supersedes` — optional prior decision identity; prior bytes remain immutable.

## Identity and validation

The decision key is `decision_id`; its preimage includes actor, authority, scope, evidence, disposition, rationale and supersession.
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
