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

- `contract_version` — exactly `quire.method-result/v1`.
- `run_id` — request, provider, implementation, toolchain and option identity.
- `subject_claim` — exact obligation and claimed strength.
- `outcome` — proved, refuted, tested, observed, inconclusive, unsupported, incomplete or failed.
- `assumptions_domain` — exact premises and explored domain.
- `witness` — proof reference, test observations, strategy or counterexample.
- `coverage` — explored items/transitions/branches and omitted scope.
- `provenance` — package, model, observations, ecosystem lock and timestamps as metadata.

## Identity and validation

The result key is (`run_id`, canonical result content). Outcome strength cannot be inferred from provider name or artifact extension.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-333-AC-1 | Proved/refuted outcomes retain the exact assumptions and domain required to interpret them. | Analysis |
| FR-333-AC-2 | Heuristic candidates and observations cannot be relabeled as proof. | Analysis |
| FR-333-AC-3 | Malformed, cross-wired or unverifiable provider output becomes provider failure. | Analysis |

## Dependencies

- [interface_015](./interface_015-method-result.md) owns the operation boundary.
- FR-192 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
