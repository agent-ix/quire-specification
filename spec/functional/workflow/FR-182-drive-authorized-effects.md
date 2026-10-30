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

## Effect-adapter protocol

The evaluator emits an immutable recommendation. A human or policy decision
binds its identity, actor, authority grant, decision time and approved effect
parameters. The adapter derives one idempotency key from adapter identity,
decision identity and effect identity, persists the pending request before
transport, and reconciles retries against that record. A reused key is valid
only for byte-identical canonical request content. Transport receipts update
delivery state; separately observed domain events establish business effect and
re-enter evaluation as new evidence. No evaluator or analyzer invokes an
adapter directly.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-182-AC-1 | A valid authority and fresh idempotency key issue one typed request and retain an attributed receipt. | Test (TC-211) |
| FR-182-AC-2 | Duplicate delivery with the same decision/key does not request a second external effect. | Test (TC-211) |
| FR-182-AC-3 | Missing/mismatched authority, reused key with different inputs or receipt/effect substitution refuses. | Test (TC-211) |
| FR-182-AC-4 | Crash-and-retry after pending-request persistence reconciles the same request without issuing a distinct effect identity. | Test (TC-211) |
| FR-182-AC-5 | Only an attributed domain observation can establish the business effect; recommendation, decision and transport receipt cannot. | Test (TC-211) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
