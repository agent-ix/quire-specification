---
id: NFR-001
title: "Preserve historical definition and artifact meaning"
type: NFR
quality_attribute: compatibility
relationships:
  - target: "ix://agent-ix/quire-specification/FR-001"
    type: constrains
  - target: "ix://agent-ix/quire-specification/FR-014"
    type: constrains
  - target: "ix://agent-ix/quire-specification/FR-015"
    type: constrains
  - target: "ix://agent-ix/quire-specification/FR-016"
    type: constrains
---
# NFR-001: Preserve historical definition and artifact meaning

## Statement

When a definition or wire contract changes, the standard shall retain the identity of the historical interpretation.

## Scope

Historical fixture and canonical bytes; explicit definition/mapping adoption.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Historical golden byte drift | 0 changed bytes | 0 changed bytes | Byte comparison |
| Unmapped cross-version equivalence grants | 0 | 0 | Adverse consumer tests |

## Verification

Compare selected historical fixtures before and after the amendment. Exercise changed profile, dependency, wire and canonical-domain controls under independent consumers.

## Dependencies

- [FR-001](../functional/foundation/FR-001-select-native-profile.md)
- [FR-014](../functional/foundation/FR-014-read-exact-wire.md)
- [FR-015](../functional/foundation/FR-015-preserve-canonical-domains.md)
- [FR-016](../functional/foundation/FR-016-migrate-explicitly.md)
