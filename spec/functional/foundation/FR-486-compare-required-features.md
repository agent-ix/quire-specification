---
id: FR-486
title: "Compare required features of a shared semantic reference"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-004"
    type: traces_to
  - target: "ix://agent-ix/quire-specification/FR-017"
    type: depends_on
---
# FR-486: Compare required features of a shared semantic reference

## Description

When a shared semantic reference carries required features, the reader shall compare them as a duplicate-free decoded set.

## Inputs

A shared semantic reference whose `requiredFeatures` member is selected by the
[wire amendment](../../../proposals/shared-reference-2-draft/README.md).

## Outputs

A checked feature set, an invalid-wire refusal for a duplicate decoded member, or
an unknown-required-feature refusal.

## Behavior

requiredFeatures is validated as a duplicate-free decoded set and compared
independently of the order it was written in. No global array sort applies.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-486-AC-1 | Reordered unique requiredFeatures compare equal; duplicate decoded features refuse before comparison and an unknown required feature refuses interpretation. | Test (TC-975) |

## Dependencies

- [US-004](../../usecase/US-004-retain-evidence-meaning.md) and [FR-017](FR-017-select-verification-artifact-roles.md).
- [NFR-002](../../non-functional/NFR-002-classify-resource-incompleteness.md) preserves incomplete outcomes.
- [IT-003](../../integration/IT-003-verification-reference-amendment.md) separates producer structure from a fully qualified shared reader.

## Status

Accepted.
