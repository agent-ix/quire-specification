---
id: NFR-004
title: "Retain review and publication prerequisites"
type: NFR
quality_attribute: compliance
relationships:
  - target: "ix://agent-ix/quire-specification/FR-001"
    type: constrains
  - target: "ix://agent-ix/quire-specification/FR-015"
    type: constrains
  - target: "ix://agent-ix/quire-specification/FR-016"
    type: constrains
---
# NFR-004: Retain review and publication prerequisites

## Statement

When publication of a standard artifact is requested, its owner shall record its accepted specification review and artifact rights before publication.

## Scope

Standard definitions, schemas, reusable fixtures and their promotion records.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Published artifacts without reviewed terms | 0 | 0 | Rights inventory inspection |
| Implementation gates bypassed by draft fixtures | 0 | 0 | Requirement and issue audit |

## Verification

Inspect the chosen artifact grant, inherited notices, definition status and required owner/consumer acceptance. New implementation AGPL approval does not choose a standard-wide prose grant.

## Dependencies

- [FR-001](../functional/FR-001-select-native-profile.md)
- [FR-015](../functional/FR-015-preserve-canonical-domains.md)
- [FR-016](../functional/FR-016-migrate-explicitly.md)
