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

Created/deleted sets are disjoint and equal to actual population differences. The frame grants specific field writes and created/deleted types. A caller cannot enlarge permissions. Missing observations prevent delta comparison rather than implying empty populations. A `modifies` entry naming a field grants writes to that field on every object of the invocation's pre and post populations whose effective type has the field, or a member that reaches it through `redefines`, as an effective member. A created or deleted object is permitted exactly when its most-specific type conforms to some `creates` or `deletes` entry, respectively ([FR-151](../type-model/FR-151-resolve-conformance-redefinition-dispatch.md)). The validator charges each field write's grant test and each `redefines` link it follows at the `frame.write` and `frame.redefines-link` points of the [value accounting](../../../proposals/quire-v1/definitions/value-accounting.md), before that work runs; the first unavailable charge ends validation with that record's `incomplete` result and no frame verdict.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-013-AC-1 | A change outside the selected frame receives frame_violation. | Test |
| FR-013-AC-2 | An incorrect creation inventory receives population_delta_mismatch. | Test |
| FR-013-AC-3 | An unselected replacement frame refuses the invocation. | Test |
| FR-013-AC-4 | Missing observations do not fabricate a frame violation. | Test |
| FR-013-AC-5 | A write to a `modifies`-granted field on an object of the invocation's pre or post populations, including an object reachable from no parameter or one whose effective type reaches the field only through `redefines`, is permitted; a write to a field the frame does not grant receives `frame_violation`. | Test (TC-236) |
| FR-013-AC-6 | Creating an instance of a subtype under a `creates` entry naming its supertype, or deleting one under a `deletes` entry naming its supertype, is permitted; creating or deleting an instance of a type unrelated to any `creates`/`deletes` entry receives `frame_violation`. | Test (TC-236) |
| FR-013-AC-7 | A write whose member reaches its granted field through `k` `redefines` links consumes exactly `1 + k` work units at one `frame.write` and `k` `frame.redefines-link` charges; with `work_units` limited to `k`, validation returns `incomplete` at the last `frame.redefines-link` with `consumed = k` and `next_charge = 1`, and reports no frame verdict. | Test (TC-236) |

## Dependencies

- [US-003](../../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/boundary.md) supplies the scoped context.
- [FR-151](../type-model/FR-151-resolve-conformance-redefinition-dispatch.md) owns the conformance rule making a `creates`/`deletes` grant subtype-inclusive under inheritance.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
