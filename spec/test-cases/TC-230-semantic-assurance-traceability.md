---
id: TC-230
title: "Semantic assurance traceability"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-310
    type: verifies
---

# TC-230: Semantic assurance traceability

## Description

Link each subsystem result and reject cross-kind, stale and circular evidence relations.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-310. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-310; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
