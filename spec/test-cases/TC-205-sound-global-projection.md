---
id: TC-205
title: "Sound global projection"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-174
    type: verifies
---

# TC-205: Sound global projection

## Description

Project a valid graph and report hidden-choice, incompatible-merge, unsupported and exhausted cases.

## Test Procedure

Use the subject and inputs named by FR-174. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-174; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.

## Derived-artifact identity cases (FR-174-AC-6)

1. Project one global definition for one role; record the identity and the carried key members.
2. Repeat, changing in turn the FR-288 definition key, the selected role, the projection profile, the channel premises (a FIFO versus an unordered ordering policy), the choice premises, the visibility premises and the bounds. Each yields a different identity.
3. Repeat, changing only the display name, source path, span and producing run. The identity is equal.
4. Omit one key member at a time from the key or from a carried artifact. Each is refused `invalid_runtime_input`/`missing-member` naming the member; no default is used.

Expected: case 1 carries all five key members; case 2 differs in each variation; case 3 equal; case 4 refused as stated.
