---
id: FR-013
title: "Enforce authored operation frames"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-151
    type: references
---
# FR-013: Enforce authored operation frames

## Description

When complete operation observations are supplied, the runtime validator shall check their population delta against the selected authored frame.

## Inputs

Exact frame binding, complete pre/post snapshots, created/deleted inventories and changed fields.

## Outputs

A frame-valid invocation or frame/delta diagnostics.

## Behavior

Created/deleted sets are disjoint and equal to actual population differences. The frame grants specific field writes and created/deleted types. A caller cannot enlarge permissions. Missing observations prevent delta comparison rather than implying empty populations. A `modifies` entry naming a field grants writes to that field on every object of the invocation's pre and post populations whose effective type has the field, or a member that reaches it through `redefines`, as an effective member. A created or deleted object is permitted exactly when its most-specific type conforms to some `creates` or `deletes` entry, respectively ([FR-151](../type-model/FR-151-resolve-conformance-redefinition-dispatch.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-013-AC-1 | A change outside the selected frame receives frame_violation. | Test |
| FR-013-AC-2 | An incorrect creation inventory receives population_delta_mismatch. | Test |
| FR-013-AC-3 | An unselected replacement frame refuses the invocation. | Test |
| FR-013-AC-4 | Missing observations do not fabricate a frame violation. | Test |
| FR-013-AC-5 | A write to a `modifies`-granted field on an object of the invocation's pre or post populations, whether or not it is reachable from any parameter, is permitted; a write to a field the frame does not grant receives `frame_violation`. | Test (TC-236) |
| FR-013-AC-6 | Creating an instance of a subtype under a `creates` entry naming its supertype is permitted; creating an instance of a type unrelated to any `creates` entry receives `frame_violation`. | Test (TC-236) |

## Dependencies

- [US-003](../../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/boundary.md) supplies the scoped context.
- [FR-151](../type-model/FR-151-resolve-conformance-redefinition-dispatch.md) owns the conformance rule making a `creates`/`deletes` grant subtype-inclusive under inheritance.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
