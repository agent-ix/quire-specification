---
id: FR-013
title: "Enforce authored operation frames"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: implements
---
# FR-013: Enforce authored operation frames

## Description

When complete operation observations are supplied, the runtime validator shall check their population delta against the selected authored frame.

## Inputs

Exact frame binding, complete pre/post snapshots, created/deleted inventories and changed fields.

## Outputs

A frame-valid invocation or frame/delta diagnostics.

## Behavior

Created/deleted sets are disjoint and equal to actual population differences. The frame grants specific field writes and created/deleted types. A caller cannot enlarge permissions. Missing observations prevent delta comparison rather than implying empty populations.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-013-AC-1 | A change outside the selected frame receives frame_violation. | Test |
| FR-013-AC-2 | An incorrect creation inventory receives population_delta_mismatch. | Test |
| FR-013-AC-3 | An unselected replacement frame refuses the invocation. | Test |
| FR-013-AC-4 | Missing observations do not fabricate a frame violation. | Test |

## Dependencies

- [US-003](../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../proposals/state-core/boundary.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
