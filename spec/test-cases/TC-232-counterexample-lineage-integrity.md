---
id: TC-232
title: "Counterexample lineage integrity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-312
    type: verifies
---

# TC-232: Counterexample lineage integrity

## Description

Validate original/shrunk/replayed lineage and reject domain, verdict and replay-material loss.

## Test Procedure

Use the subject and inputs named by FR-312. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-312; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
