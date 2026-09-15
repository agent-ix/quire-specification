---
id: TC-222
title: "Lossless CST round trip"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-302
    type: verifies
---

# TC-222: Lossless CST round trip

## Description

Round-trip valid trivia-rich source, recover invalid source and apply an incremental edit.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-302. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-302; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
