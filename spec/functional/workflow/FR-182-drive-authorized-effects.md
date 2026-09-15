---
id: FR-182
title: "Drive authorized idempotent workflow effects"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-012
    type: implements
  - target: ix://agent-ix/quire-specification/AD-008
    type: references
---

# FR-182: Drive authorized idempotent workflow effects

## Description

When an application accepts a typed formal decision, the workflow boundary SHALL require explicit authority and an idempotency identity before requesting an external effect.

## Inputs

Decision/result identity, action parameters, authority grant, idempotency key,
adapter manifest and current receipt state.

## Outputs

Rejected recommendation, effect request plus attributed receipt, or typed
adapter failure/incomplete outcome.

## Behavior

Evaluators emit recommendations only. The adapter validates authority scope,
binds one decision to one idempotency key and records request/receipt identities.
Retries reconcile the same request. A transport receipt is not a business effect;
effect observations re-enter through the observation authority contract.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-182-AC-1 | A valid authority and fresh idempotency key issue one typed request and retain an attributed receipt. | Test (TC-211) |
| FR-182-AC-2 | Duplicate delivery with the same decision/key does not request a second external effect. | Test (TC-211) |
| FR-182-AC-3 | Missing/mismatched authority, reused key with different inputs or receipt/effect substitution refuses. | Test (TC-211) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
