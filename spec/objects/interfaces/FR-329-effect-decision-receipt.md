---
id: FR-329
title: "Authorized effect decision and receipt"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_011
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-182
    type: references
---

# FR-329: Authorized effect decision and receipt

## Description

When a formal decision requests an external workflow effect, the producer SHALL use the immutable EffectDecisionReceipt contract below rather than an untyped map, display string or implementation-local structure.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| contract_version | String | 1..1 |  |
| decision | String | 1..1 |  |
| authority_grant | String | 1..1 |  |
| idempotency_key | String | 1..1 |  |
| effect_request | String | 1..1 |  |
| transport_receipt | String | 1..1 |  |
| effect_observation | String | 0..1 |  |
| lineage | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| contract_version | type not stated in source; see Field sources |
| decision | type not stated in source; see Field sources |
| authority_grant | type not stated in source; multiplicity not stated in source; see Field sources |
| idempotency_key | type not stated in source; see Field sources |
| effect_request | type not stated in source; multiplicity not stated in source; see Field sources |
| transport_receipt | type not stated in source; multiplicity not stated in source; see Field sources |
| effect_observation | type not stated in source; see Field sources |
| lineage | type not stated in source; multiplicity not stated in source; see Field sources |


- `contract_version` — exactly `quire.effect-decision/v1`.
- `decision` — exact semantic result and recommended action identity.
- `authority_grant` — actor, action, subject, scope, expiry and rights.
- `idempotency_key` — caller-owned key bound to the canonical effect request.
- `effect_request` — adapter, operation, typed arguments and request identity.
- `transport_receipt` — delivery/acceptance fact, never business-effect truth.
- `effect_observation` — optional authority-qualified business-effect observation.
- `lineage` — retry, duplicate, supersession and refusal relations.

## Identity and validation

The request key is (`decision`, `authority_grant`, `idempotency_key`, canonical operation/arguments). Reuse with different inputs is a contradiction.
A document whose `contract_version` is not that identifier, missing required
properties, duplicate keys, out-of-domain values and non-canonical encodings
refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-329-AC-1 | A valid fresh key emits at most one external effect request. | Analysis |
| FR-329-AC-2 | A duplicate retry reconciles the same request and receipt history. | Analysis |
| FR-329-AC-3 | Authority failure or key/input mismatch refuses before adapter dispatch. | Analysis |

## Dependencies

- [interface_011](./interface_011-effect-decision-receipt.md) owns the operation boundary.
- FR-182 owns the behavior that produces or consumes this value.
