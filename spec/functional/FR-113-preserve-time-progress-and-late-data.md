---
id: FR-113
title: "Preserve time, progress, and late-data authority"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
---
## Description

When assessing a timed obligation, the observation service SHALL retain event time, ingestion time, declared clock interpretation, progress authority, captured trigger identity, and late-data cutoff before producing a settled disposition.

## Behavior

Arbitrary timestamp order SHALL NOT establish causality. Silence SHALL settle a deadline only when declared progress/closure authority covers that deadline. The observation service SHALL classify a late event against the selected cutoff rather than silently rewriting an already settled result.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-113-AC-1 | A quiet eligible deadline settles only with declared progress authority. | Test (TC-143) |
| FR-113-AC-2 | Event time, ingestion time, causal relation, and clock uncertainty remain distinguishable. | Test (TC-143) |
| FR-113-AC-3 | A late event is retained as late and cannot silently replace the selected settled result. | Test (TC-143) |

## Dependencies

- [Shared foundation](../../proposals/quire-v1/shared-foundation.md).
- E owns clock, history, and closure meaning.
