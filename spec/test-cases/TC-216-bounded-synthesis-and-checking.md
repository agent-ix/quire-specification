---
id: TC-216
title: "Bounded synthesis and checking"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-194
    type: verifies
---

# TC-216: Bounded synthesis and checking

## Description

Find/check a candidate, replay a failed candidate, and distinguish no-candidate from incomplete search.

## Test Procedure

Use the subject and inputs named by FR-194. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-194; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
