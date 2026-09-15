---
id: TC-214
title: "Verification plan dispatch"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-192
    type: verifies
---

# TC-214: Verification plan dispatch

## Description

Link/execute an acyclic plan and inject missing provider, unsupported sibling and dependency cycle.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-192. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-192; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
