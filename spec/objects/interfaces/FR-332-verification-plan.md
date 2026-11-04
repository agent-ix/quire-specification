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

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| contract_version | String | 1..1 |  |
| plan_id | String | 1..1 |  |
| obligations | JsonObject | 0..* |  |
| methods | JsonObject | 0..* |  |
| domains | JsonObject | 1..1 |  |
| assumptions | JsonObject | 1..1 |  |
| dependencies | JsonObject | 1..1 |  |
| bounds | JsonObject | 1..1 |  |
| providers | JsonObject | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| contract_version | exactly `quire.verification-plan/v1` |
| obligations | list of obligation records: clause or claim plus expected evidence class |
| methods | ordered list of method selections |
| domains | record: model, input, trace, time, candidate domains; multiplicity not stated in source |
| assumptions | record: authored premises with authorities; multiplicity not stated in source |
| dependencies | record: prerequisite edges between plan nodes; multiplicity not stated in source |
| bounds | record: method work bounds and semantic-domain bounds; multiplicity not stated in source |
| providers | record: capability-constrained selections and fallback or refusal policy; multiplicity not stated in source |


- `contract_version` — exactly `quire.verification-plan/v1`.
- `plan_id` — the identity assigned to the plan, compared as an exact string. It is not computed from the other members.
- `obligations` — exact clauses/claims and expected evidence class.
- `methods` — ordered provider-independent method selections.
- `domains` — explicit model/input/trace/time/candidate domains.
- `assumptions` — authored premises with authorities.
- `dependencies` — acyclic prerequisite edges between plan nodes.
- `bounds` — method work and semantic-domain bounds kept distinct.
- `providers` — capability-constrained selections and fallback/refusal policy.

## Identity and validation

The plan key is `plan_id`; no identity is derived from the other fields. Source order is retained for diagnostics but dependency order governs execution.
A document whose `contract_version` is not that identifier, missing required
properties, duplicate keys, out-of-domain values and non-canonical encodings
refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-332-AC-1 | Every obligation has at least one selected method or an explicit unsupported disposition. | Analysis |
| FR-332-AC-2 | A dependency cycle, unresolved provider or absent required domain refuses linking. | Analysis |
| FR-332-AC-3 | Changing a method, assumption, domain, bound or provider selection changes plan identity. | Analysis |

## Dependencies

- [interface_014](./interface_014-verification-plan.md) owns the operation boundary.
- FR-192 owns the behavior that produces or consumes this value.
