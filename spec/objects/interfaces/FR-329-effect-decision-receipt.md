---
id: FR-329
title: "Authorized effect decision and receipt"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface-011
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-182
    type: references
---

# FR-329: Authorized effect decision and receipt

## Description

When a formal decision requests an external workflow effect, the producer SHALL use the immutable EffectDecisionReceipt contract below rather than an untyped map, display string or implementation-local structure.

## Properties

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
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-329-AC-1 | A valid fresh key emits at most one external effect request. | Analysis |
| FR-329-AC-2 | A duplicate retry reconciles the same request and receipt history. | Analysis |
| FR-329-AC-3 | Authority failure or key/input mismatch refuses before adapter dispatch. | Analysis |

## Dependencies

- [interface-011](./interface-011-effect-decision-receipt.md) owns the operation boundary.
- FR-182 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
