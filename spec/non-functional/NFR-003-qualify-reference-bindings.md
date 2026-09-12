---
id: NFR-003
title: "Reject unqualified identity reuse"
type: NFR
quality_attribute: security
relationships:
  - target: "ix://agent-ix/quire-specification/FR-002"
    type: constrains
  - target: "ix://agent-ix/quire-specification/FR-003"
    type: constrains
  - target: "ix://agent-ix/quire-specification/FR-004"
    type: constrains
---
# NFR-003: Reject unqualified identity reuse

## Statement

When an artifact binding conflicts with selected authority or content, the consumer shall refuse that binding.

## Scope

Imports, source maps, dependency closure and evidence subjects.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Accepted stale-byte mutations | 0 | 0 | Digest mutation tests |
| Accepted identity-content conflicts | 0 | 0 | Independent reference-join tests |

## Verification

Run both stale-digest mutations and changed content with a recomputed digest under the same immutable key. Do not replace semantic validation with schema success.

## Dependencies

- [FR-002](../functional/foundation/FR-002-resolve-owned-declarations.md)
- [FR-003](../functional/foundation/FR-003-bind-source-correspondence.md)
- [FR-004](../functional/foundation/FR-004-separate-artifact-identities.md)
