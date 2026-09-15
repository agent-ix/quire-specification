---
id: FR-321
title: "Authoritative model-selection artifact"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface-003
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-150
    type: references
---

# FR-321: Authoritative model-selection artifact

## Description

When a model export is selected for native linking, the producer SHALL use the immutable ModelSelectionArtifact contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exact selected producer interface and adapter contract versions.
- `authority` — producer authority, repository/package identity and revision.
- `export` — model-export identity, canonical bytes and digest domain.
- `closure` — ordered declarations, types, relationships and dependencies admitted by the producer.
- `correspondence` — original declaration to ordered effective-declaration derivation paths.
- `feature_report` — exact available, absent and unsupported producer capabilities.

## Identity and validation

The key is (`authority`, `export`, `contract_version`). Equal local labels or equal declaration bytes under another authority are unequal.
Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-321-AC-1 | Every effective declaration names its original declaration and ordered derivation path. | Analysis |
| FR-321-AC-2 | An unknown producer version, stale digest or incomplete closure refuses before name binding. | Analysis |
| FR-321-AC-3 | A consumer cannot mint, rename or merge producer identities. | Analysis |

## Dependencies

- [interface-003](./interface-003-model-selection.md) owns the operation boundary.
- FR-150 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
