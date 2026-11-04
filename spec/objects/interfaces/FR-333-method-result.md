---
id: FR-333
title: "Verification method result"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_015
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-192
    type: references
---

# FR-333: Verification method result

## Description

When a verification provider returns a method result, the producer SHALL use the immutable MethodResult contract below rather than an untyped map, display string or implementation-local structure.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| contract_version | String | 1..1 |  |
| run_id | String | 1..1 |  |
| subject_claim | JsonObject | 1..1 |  |
| outcome | String | 1..1 |  |
| assumptions_domain | JsonObject | 1..1 |  |
| witness | JsonObject | 1..1 |  |
| coverage | JsonObject | 1..1 |  |
| provenance | JsonObject | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| contract_version | exactly `quire.method-result/v1` |
| run_id | type not stated in source; multiplicity not stated in source; see Field sources |
| subject_claim | record: obligation and claimed strength |
| outcome | string tag: proved, refuted, tested, observed, inconclusive, unsupported, incomplete or failed; multiplicity not stated in source |
| assumptions_domain | record: premises and explored domain |
| witness | record: one of proof reference, test observations, strategy, counterexample; multiplicity not stated in source |
| coverage | record: explored items, transitions, branches, omitted scope; multiplicity not stated in source |
| provenance | record: package, model, observations, timestamps; multiplicity not stated in source |


- `contract_version` — exactly `quire.method-result/v1`.
- `run_id` — request, provider and option identity.
- `subject_claim` — exact obligation and claimed strength.
- `outcome` — proved, refuted, tested, observed, inconclusive, unsupported, incomplete or failed.
- `assumptions_domain` — exact premises and explored domain.
- `witness` — proof reference, test observations, strategy or counterexample.
- `coverage` — explored items/transitions/branches and omitted scope.
- `provenance` — package, model, observations and timestamps as metadata.

## Identity and validation

The result key is (`run_id`, canonical result content). Outcome strength cannot be inferred from provider name or artifact extension.
A document whose `contract_version` is not that identifier, missing required
properties, duplicate keys, out-of-domain values and non-canonical encodings
refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-333-AC-1 | Proved/refuted outcomes retain the exact assumptions and domain required to interpret them. | Analysis |
| FR-333-AC-2 | Heuristic candidates and observations cannot be relabeled as proof. | Analysis |
| FR-333-AC-3 | Malformed, cross-wired or unverifiable provider output becomes provider failure. | Analysis |

## Dependencies

- [interface_015](./interface_015-method-result.md) owns the operation boundary.
- FR-192 owns the behavior that produces or consumes this value.
