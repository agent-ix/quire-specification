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
| Boolean success after resource exhaustion | 0 | 0 | fault-injection |
| Retained state above declared bound | 0 entries | 0 entries | property-based-testing |

## Verification

Test exact-bound and one-over-bound events, active keys, history, and
population/window members. Independently observe retained state before and
after each control, including attempted allocation/retention on the one-over
path. Verify that every over-bound result remains incomplete/refused with the
selected subject and exceeded bound identity, and that a correct disposition
with excess retained state fails this metric.

## Status

Accepted by FS04 for bounded observation retention. Concrete owner limits and
one-over evidence remain implementation-specific.
