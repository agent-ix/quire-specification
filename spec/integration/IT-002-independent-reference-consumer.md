---
id: IT-002
title: "Qualify the shared reference consumer"
type: IT
relationships:
  - target: "ix://agent-ix/quire-specification/FR-003"
    type: verifies
  - target: "ix://agent-ix/quire-specification/FR-004"
    type: verifies
  - target: "ix://agent-ix/quire-specification/FR-014"
    type: verifies
  - target: "ix://agent-ix/quire-specification/FR-016"
    type: verifies
---
# IT-002: Qualify the shared reference consumer

## Objective

Independent consumer observations agree with the selected contract. Missing consumer adoption is an unmet prerequisite, not an empty successful test.

## Target Integration

A native producer packet consumed through B/C-owned existing-repository adapters using real files and validators.

## Preconditions

The exact FS02/FS05 candidate and consumer revisions must be accepted and pinned. The real consumer must be available. Current adoption is unresolved; this test remains planned and cannot pass using only A's producer audit.

## Inputs

Versioned source/property/model/frame/snapshot/invocation/run references; valid and malformed source maps; one-axis stale, duplicate, unknown-version and migration mutations.

## Test Procedure

1. Build the selected producer and consumer (timeout 180 seconds).
   - IT-002-SC-01: Both selected executable versions are recorded.
2. Pass the exact reference packet to the actual consumer (timeout 30 seconds).
   - IT-002-SC-02: The admitted positive references resolve to their intended roles.
3. Pass the adverse controls through the same consumer (timeout 30 seconds).
   - IT-002-SC-03: Each adverse case returns its specified refusal or incompleteness category.
4. Inspect historical artifact bytes (timeout 10 seconds).
   - IT-002-SC-04: The consumer has not rewritten historical evidence.

## Expected Results

Independent consumer observations agree with the selected contract. Missing consumer adoption is an unmet prerequisite, not an empty successful test.

## Metadata

Priority: High. Automation: real command/file/API execution. Status: draft integration specification; each prerequisite and result requires observed evidence.

## Dependencies

- [FR-003](../functional/foundation/FR-003-bind-source-correspondence.md)
- [FR-004](../functional/foundation/FR-004-separate-artifact-identities.md)
- [FR-014](../functional/foundation/FR-014-read-exact-wire.md)
- [FR-016](../functional/foundation/FR-016-migrate-explicitly.md)
