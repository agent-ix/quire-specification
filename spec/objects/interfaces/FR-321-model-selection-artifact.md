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

- `contract_version` — the FCD FR-126 `interfaceVersion` and `wireSchema` of the admitted producer document; under the selected `quire.model.complete/v1` definition the interface version is exactly `1.3.0`.
- `authority` — the correspondence `ProducerObject.authority` (FCD FR-116) of the export's records.
- `export` — the FCD FR-117 static bundle key: bundle `identity`, namespaced `revision` and `digest` with domain `filament-canonical-json-1`.
- `closure` — ordered declarations, types, relationships and dependencies admitted by the producer, including every generalization, subsetting and redefinition record and the bundle's `generalizationClosure`, `subsettingClosure` and `redefinitionClosure` header statements.
- `correspondence` — the producer supplies original declarations and their edges; QSL derives the effective declarations and ordered derivation paths as the `quire.model.effective-view/v1` identity defined by the selected `quire.model.complete/v1` definition. The producer never supplies an effective identity.
- `feature_report` — exact available, absent and unsupported producer capabilities.

## Identity and validation

The key is (`authority`, `export`, `contract_version`). Equal local labels or equal declaration bytes under another authority are unequal.
An unknown `interfaceVersion` refuses `unknown_wire`/`unsupported-wire`. Unknown contract versions, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-321-AC-1 | Every effective declaration names its original declaration and ordered derivation path. | Analysis |
| FR-321-AC-2 | An unknown producer version, stale digest or incomplete closure refuses before name binding. | Analysis |
| FR-321-AC-3 | A consumer cannot mint, rename or merge producer identities. | Analysis |
| FR-321-AC-4 | The key components map exactly to the FCD correspondence authority, FR-117 bundle key and FR-126 interface version, and an effective identity is derived only in a QSL domain. | Test (TC-195) |

## Dependencies

- [interface-003](./interface-003-model-selection.md) owns the operation boundary.
- FR-150 owns the behavior that produces or consumes this value.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
