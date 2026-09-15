---
id: TC-219
title: "Backend counterexample native replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-197
    type: verifies
---

# TC-219: Backend counterexample native replay

## Description

Round-trip, minimize and replay backend counterexamples and inject decode/domain/verdict mismatches.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-197. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-197; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
