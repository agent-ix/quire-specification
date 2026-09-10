---
id: NFR-050
title: "Bound retained observation state"
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/quire-specification/FR-112
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-114
    type: constrains
---
## Statement

The observation service SHALL enforce declared event-size, active-key, retained-history, and population/window bounds; exhaustion reports retain the exceeded-bound cause.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Boolean success after resource exhaustion | 0 | 0 | Boundary tests |
| Retained state above declared bound | 0 entries | 0 entries | Resource tests |

## Verification

Test exact-bound and one-over-bound events, active keys, history, and population members. Verify that every over-bound result remains incomplete/refused with the selected subject and exceeded bound identity.
