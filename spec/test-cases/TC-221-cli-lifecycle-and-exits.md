---
id: TC-221
title: "CLI lifecycle and exits"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-301
    type: verifies
---

# TC-221: CLI lifecycle and exits

## Description

Invoke every command and assert machine output, diagnostics and outcome-specific exit statuses.

## Test Procedure

Use the subject and inputs named by FR-301. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-301; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
