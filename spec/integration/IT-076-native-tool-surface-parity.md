---
id: IT-076
title: "Native API, CLI, WASM and authoring tools preserve parity"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-300
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-308
    type: verifies
---

# IT-076: Native API, CLI, WASM and authoring tools preserve parity

## Objective

Verify the real Rust lifecycle library, CLI, WASM boundary, lossless CST,
formatter and language server preserve identical subjects and dispositions.

## Target Integration

QSL/CLI/WASM/editor Rust components run locally through their public APIs
and command/process boundaries.

## Preconditions

Machine wire identifiers and resource limits are explicit.

## Inputs

Valid/invalid/recovered source, formatting/idempotency corpus, lifecycle requests,
unknown-feature and resource-boundary variants.

## Test Procedure

1. Parse/check through API, CLI and WASM.
   - IT-076-SC-01: package identities, diagnostics and dispositions agree.
2. Format and incrementally edit the corpus.
   - IT-076-SC-02: formatting is idempotent and full/incremental results agree.
3. Exercise wire/limit/ambient-authority failures.
   - IT-076-SC-03: each boundary refuses safely and identically where applicable.

## Expected Results

All surfaces preserve source and semantic identity; no tool silently executes a
different profile or gains ambient effects.
