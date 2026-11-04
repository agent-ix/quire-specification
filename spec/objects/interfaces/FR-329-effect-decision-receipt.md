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
| decision | JsonObject | 1..1 |  |
| authority_grant | JsonObject | 1..1 |  |
| idempotency_key | String | 1..1 |  |
| effect_request | JsonObject | 1..1 |  |
| transport_receipt | JsonObject | 1..1 |  |
| effect_observation | JsonObject | 0..1 |  |
| lineage | JsonObject | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| contract_version | exactly `quire.effect-decision/v1` |
| decision | record: semantic result and recommended action identity |
| authority_grant | record: actor, action, subject, scope, expiry, rights; multiplicity not stated in source |
| idempotency_key | caller-owned key string |
| effect_request | record: adapter, operation, typed arguments, request identity; multiplicity not stated in source |
| transport_receipt | record: delivery or acceptance fact; multiplicity not stated in source |
| effect_observation | record; bullet says optional |
| lineage | record: retry, duplicate, supersession, refusal relations; multiplicity not stated in source |


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
