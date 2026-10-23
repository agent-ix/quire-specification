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

- `identity` — the domain package identity, such as `acme/orders`, named by the source `model` declaration.
- `digest_domain` — exactly `sha256-jcs`.
- `digest` — the lowercase 64-hex SHA-256 over the RFC 8785 JCS bytes of the domain package.
- `declarations` — the original declarations that QSL model intake (FR-154) builds from the package's IR nodes. Each is keyed by `(domain package identity, IR node identity, sha256-jcs)` and keeps the IR node's FCD `origin` (FR-154).
- `effective_view` — the `quire.model.effective-view/v2` identity that QSL derives from those declarations under the selected `quire.model.complete/v1` definition.

## Identity and validation

The key is (`identity`, `digest_domain`, `digest`): the domain package identity and its content digest. Two selections with equal identity and different digests are unequal.
A digest domain other than `sha256-jcs` refuses `stale_dependency`/`digest-domain-mismatch`. Intake reads the package bytes supplied under the selection's digest and does not recompute that digest (FR-154).
A package selects at most one domain package per `identity`; intake
refuses a second selection of an already-selected `identity`, before name
binding, as `duplicate_selection`/`duplicate-identity` (QSL ADR-013 O-01,
QC-5).
Missing required properties, duplicate keys, out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-321-AC-1 | Every effective declaration names its original declaration and ordered derivation path. | Analysis |
| FR-321-AC-2 | A wrong digest domain or a malformed declaration refuses before name binding. | Analysis |
| FR-321-AC-3 | A declaration key is fixed by the domain package identity and the IR node identity; the checker never mints, renames or merges one. | Analysis |
| FR-321-AC-4 | The key components map exactly to the domain package identity and its `sha256-jcs` digest, and an effective identity is derived only in a QSL domain. | Test (TC-195) |
| FR-321-AC-5 | A second selection of an already-selected domain-package `identity` refuses `duplicate_selection`/`duplicate-identity` before name binding, retaining the repeated identity and both selected digests; a second selection of a distinct `identity` is unaffected. | Analysis |

## Dependencies

- [interface_003](./interface_003-model-selection.md) owns the operation boundary.
- FR-150 owns the behavior that produces or consumes this value.
- FR-154 owns model intake.

## Status

Accepted complete-V1 boundary value contract.
