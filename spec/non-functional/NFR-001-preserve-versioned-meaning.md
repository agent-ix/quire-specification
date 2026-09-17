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

Fixture and canonical bytes pinned by digest; explicit definition/mapping adoption.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Pinned digest mismatches | 0 | 0 | Byte comparison |
| Unmapped cross-version equivalence grants | 0 | 0 | Adverse consumer tests |

## Verification

Verify the `proposals/state-core` definition and fixture bytes against the digests pinned in `typing-cases.json`, `reference-cases.json` and `SHA256SUMS`. Exercise changed profile, dependency, wire and canonical-domain controls under independent consumers.

## Dependencies

- [FR-001](../functional/foundation/FR-001-select-native-profile.md)
- [FR-014](../functional/foundation/FR-014-read-exact-wire.md)
- [FR-015](../functional/foundation/FR-015-preserve-canonical-domains.md)
- [FR-016](../functional/foundation/FR-016-migrate-explicitly.md)

## Status

Accepted by FS05 for versioned interchange and explicit migration. Fixture and
canonical bytes match their pinned digests; executable evidence stays in its
own matrices.
