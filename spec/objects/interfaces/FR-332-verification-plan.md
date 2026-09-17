---
id: FR-332
title: "Linked verification plan"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_014
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-192
    type: references
---

# FR-332: Linked verification plan

## Description

When an authored verification plan completes linking, the producer SHALL use the immutable VerificationPlan contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.verification-plan/v1`.
- `plan_id` — identity over canonical source, checked package and resolved selections.
- `obligations` — exact clauses/claims and expected evidence class.
- `methods` — ordered provider-independent method selections.
- `domains` — explicit model/input/trace/time/candidate domains.
- `assumptions` — authored premises with authorities.
- `dependencies` — acyclic prerequisite edges between plan nodes.
- `bounds` — method work and semantic-domain bounds kept distinct.
- `providers` — capability-constrained selections and fallback/refusal policy.

## Identity and validation

The plan key covers all fields except execution results. Source order is retained for diagnostics but dependency order governs execution.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-332-AC-1 | Every obligation has at least one selected method or an explicit unsupported disposition. | Analysis |
| FR-332-AC-2 | A dependency cycle, unresolved provider or absent required domain refuses linking. | Analysis |
| FR-332-AC-3 | Changing a method, assumption, domain, bound or provider selection changes plan identity. | Analysis |

## Dependencies

- [interface_014](./interface_014-verification-plan.md) owns the operation boundary.
- FR-192 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
