---
id: TC-223
title: "Formatter LSP incremental parity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-303
    type: verifies
---

# TC-223: Formatter LSP incremental parity

## Description

Format twice and compare full versus incremental diagnostics, symbols and stale-edit refusal.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-303. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-303; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
