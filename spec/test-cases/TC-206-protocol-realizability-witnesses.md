---
id: TC-206
title: "Protocol realizability witnesses"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-175
    type: verifies
---

# TC-206: Protocol realizability witnesses

## Description

Analyze realizable and hidden-choice-unrealizable graphs plus missing-premise and bound cases.

## Test Procedure

Use the subject and inputs named by FR-175. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-175; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
