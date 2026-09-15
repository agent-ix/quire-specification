---
id: FR-324
title: "Native temporal artifact and result envelope"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface-006
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-160
    type: references
---

# FR-324: Native temporal artifact and result envelope

## Description

When temporal meaning crosses the native temporal boundary, the producer SHALL use the immutable NativeTemporalEnvelope contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.native-temporal/v1`.
- `artifact` — formula graph, typed atoms, interval bounds and semantic-profile identity.
- `clock_binding` — clock authority, epoch, unit, resolution and ordering contract.
- `history_boundary` — execution origin, cutoff and closure facts.
- `valuation` — partial atom values with observation identities and uncertainty sets.
- `progress` — authoritative observed frontier and completeness state.
- `result` — truth/settlement/execution/adequacy/provenance axes plus witness.
- `supersession` — prior result and observation-revision relation when repaired.

## Identity and validation

Artifact, evaluation and result identities are separate. Equal formula text under another profile, clock or source occurrence is unequal.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-324-AC-1 | Unknown or uncertain valuations are retained and never coerced to false. | Analysis |
| FR-324-AC-2 | Finite progress cannot produce proved infinite-trace satisfaction. | Analysis |
| FR-324-AC-3 | A repair preserves the prior result and emits an explicit supersession link. | Analysis |

## Dependencies

- [interface-006](./interface-006-native-temporal.md) owns the operation boundary.
- FR-160 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
