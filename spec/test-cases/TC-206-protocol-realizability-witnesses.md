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

## Derived-artifact identity cases (FR-175-AC-6)

1. Analyze one realizable protocol; record the witness identity and the carried key members.
2. Repeat, changing in turn the FR-288 definition key, the strategy profile, the role visibility, the channel premises, the environment premises, the bounds, and the analyzer identity and then its version. Each yields a different identity.
3. Repeat with only the display name, path, span and producing run changed. The identity is equal.
4. Run the same analyzer identity and version twice on one key. Both runs return the same witness.
5. Omit one key member at a time. Each is refused `invalid_runtime_input`/`missing-member` naming the member.

Expected: case 1 carries all key members; case 2 differs in each variation; cases 3 and 4 equal; case 5 refused as stated.
