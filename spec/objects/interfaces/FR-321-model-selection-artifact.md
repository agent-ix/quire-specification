---
id: FR-321
title: "Authoritative model-selection artifact"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_003
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-150
    type: references
---

# FR-321: Authoritative model-selection artifact

## Description

When a package imports a domain package for native linking, the checker SHALL use the immutable ModelSelectionArtifact contract below rather than an untyped map, display string or implementation-local structure.

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| identity | String | 1..1 |  |
| declarations | JsonObject | 1..1 |  |
| effective_view | String | 1..1 |  |

## Field sources

Type and Multiplicity above were derived from the retained text below. Where that text does not state a value, or states a form the table cannot hold, the row is listed here.

| Field | Derivation note |
|---|---|
| identity | domain package identity string, e.g. `acme/orders` |
| declarations | collection of declaration records keyed by (identity, IR node identity); multiplicity not stated in source |
| effective_view | exactly one `quire.model.effective-view/v2` identity |


- `identity` — the domain package identity, such as `acme/orders`, named by the source `model` declaration and recorded by the package's `model_root` node only. There is no lock row, digest, digest domain or version. The source names the identity alone (FR-154 "Model selection in source").
- `declarations` — the original declarations that QSL model intake (FR-154) builds from the package's IR nodes. Each is keyed by `(domain package identity, IR node identity)` and keeps the IR node's FCD `origin` (FR-154).
- `effective_view` — the `quire.model.effective-view/v2` identity that QSL derives from those declarations under the selected `quire.model.complete/v1` definition.

## Identity and validation

The key is `identity`. Nothing is recorded in the lock and nothing is gated.
A package selects at most one domain package per `identity`; intake
refuses a second selection of an already-selected `identity`, before name
binding, as `duplicate_selection`/`duplicate-identity` (QSL ADR-013 O-01,
QC-5).
Missing required properties, duplicate keys, out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-321-AC-1 | Every effective declaration names its original declaration and ordered derivation path. | Analysis |
| FR-321-AC-2 | A domain package document with a malformed declaration refuses before name binding. | Analysis |
| FR-321-AC-3 | A declaration key is fixed by the domain package identity and the IR node identity; the checker never mints, renames or merges one. | Analysis |
| FR-321-AC-4 | The key components map exactly to the domain package identity and IR node identity, and an effective identity is derived only in a QSL domain. | Test (TC-195) |
| FR-321-AC-5 | A second selection of an already-selected domain-package `identity` refuses `duplicate_selection`/`duplicate-identity` before name binding, retaining the repeated identity and both selected documents; a second selection of a distinct `identity` is unaffected. | Analysis |
| FR-321-AC-6 | A ModelSelection is its `identity` alone: the lock has no `model_selections` member and a lock carrying one fails the V2 schema. | Test (TC-234) |

## Dependencies

- [interface_003](./interface_003-model-selection.md) owns the operation boundary.
- FR-150 owns the behavior that produces or consumes this value.
- FR-154 owns model intake.

## Status

Accepted complete-V1 boundary value contract.
