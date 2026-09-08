---
id: IT-001
title: "Preserve the existing canonical consumer contract"
type: IT
relationships:
  - target: "ix://agent-ix/quire-specification/FR-015"
    type: verifies
  - target: "ix://agent-ix/quire-specification/FR-016"
    type: verifies
---
# IT-001: Preserve the existing canonical consumer contract

## Objective

The real existing APIs preserve their selected golden values and refusal behavior. This verifies the legacy contract boundary, not native model binding or evaluation.

## Target Integration

Existing contract-IR public canonicalization/binding APIs and real conformance CLI over pinned files.

## Preconditions

Use the clean existing contract-IR checkout at decc99a430a0894de489102dbab04e83d1fb804f and installed Rust 1.94.1. Build outputs must be inside a writable target directory. No mock reader or hand-reimplemented canonicalizer substitutes for these APIs.

## Inputs

The checked-in canonicalization tests, executable-binding tests and corpus/contract-v0.1/manifest.json with its exact selected fixture digests.

## Test Procedure

1. Run the canonicalization and executable_binding integration test targets (timeout 180 seconds).
   - IT-001-SC-01: All selected tests finish successfully.
2. Run the existing conformance CLI against the unchanged manifest (timeout 180 seconds).
   - IT-001-SC-02: Every emitted fixture row has status match.
3. Compare original repository status after the run (timeout 10 seconds).
   - IT-001-SC-03: No tracked input or canonical fixture was modified.

## Expected Results

The real existing APIs preserve their selected golden values and refusal behavior. This verifies the legacy contract boundary, not native model binding or evaluation.

## Metadata

Priority: High. Automation: real command/file/API execution. Status: draft integration specification; each prerequisite and result requires observed evidence.

## Dependencies

- [FR-015](../functional/FR-015-preserve-canonical-domains.md)
- [FR-016](../functional/FR-016-migrate-explicitly.md)
