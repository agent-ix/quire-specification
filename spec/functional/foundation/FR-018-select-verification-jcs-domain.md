---
id: FR-018
title: "Compare required features of a shared semantic reference"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-004"
    type: traces_to
  - target: "ix://agent-ix/quire-specification/FR-017"
    type: depends_on
---
# FR-018: Compare required features of a shared semantic reference

## Description

When a shared semantic reference carries required features, the reader shall compare them as a duplicate-free decoded set.

The former verification JCS identity domain (`sha256-jcs`), the semantic `canonicalIdentity` field and the stored automated-core digest are withdrawn. A reference carries no canonical identity and no digest; the benefit they offered did not outweigh the churn of keeping a stored value in step with its source, and identity plus version already separate the artifacts.

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
| FR-018-AC-1 | Withdrawn: the verification JCS identity domain and every stored or compared digest are removed; no failure it caught is separate from identity and version. | Withdrawn |
| FR-018-AC-2 | Withdrawn: the verification JCS identity domain and every stored or compared digest are removed; no failure it caught is separate from identity and version. | Withdrawn |
| FR-018-AC-3 | Withdrawn: the verification JCS identity domain and every stored or compared digest are removed; no failure it caught is separate from identity and version. | Withdrawn |
| FR-018-AC-4 | Reordered unique requiredFeatures compare equal; duplicate decoded features refuse before comparison and an unknown required feature refuses interpretation. | Test (TC-003) |
| FR-018-AC-5 | Withdrawn: the verification JCS identity domain and every stored or compared digest are removed; no failure it caught is separate from identity and version. | Withdrawn |
| FR-018-AC-6 | Withdrawn: the verification JCS identity domain and every stored or compared digest are removed; no failure it caught is separate from identity and version. | Withdrawn |

## Dependencies

- [US-004](../../usecase/US-004-retain-evidence-meaning.md) and [FR-017](FR-017-select-verification-artifact-roles.md).
- [NFR-002](../../non-functional/NFR-002-classify-resource-incompleteness.md) preserves incomplete outcomes.
- [IT-003](../../integration/IT-003-verification-reference-amendment.md) separates producer structure from a fully qualified shared reader.

## Status

FR-018-AC-1, -2, -3, -5 and -6 are withdrawn; their numbers are not reused. FR-018-AC-4 remains accepted.
