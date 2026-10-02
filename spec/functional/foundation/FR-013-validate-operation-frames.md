---
id: FR-013
title: "Enforce authored operation frames"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-151
    type: references
  - target: ix://agent-ix/quire-specification/FR-340
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

Created/deleted sets are disjoint and equal to actual population differences. The frame grants specific field writes and created/deleted types. A caller cannot enlarge permissions. Missing observations prevent delta comparison rather than implying empty populations. A `modifies` entry has a scope. An **every-object** entry, written `f` in an operation's `Modifies:` line, grants writes to field `f` on every object of the invocation's pre and post populations whose effective type has the field, or a member that reaches it through `redefines`, as an effective member. A **receiver-scoped** entry, written `self.f`, grants writes to field `f` on the invocation's receiver only; a change to `f` on any other object receives `frame_violation`. One operation may hold both kinds of entry for different fields; an every-object entry and a receiver-scoped entry for the same field refuse the operation's frame `invalid_model_binding`/`conflicting-binding`. The model contract's operation frame, and the semantic IR `Operation.frame` lifted from it, carry each field `modifies` entry's scope as `receiver` or `every-object`. A created object is permitted exactly when its most-specific type conforms to some `creates` entry ([FR-151](../type-model/FR-151-resolve-conformance-redefinition-dispatch.md)). A deleted object is permitted exactly when its most-specific type conforms to a type-wide `deletes` entry, or it is the receiver under `deletes self`, or it is the argument bound to `p` under `deletes p`. A `deletes` entry has a scope: a **type-wide** entry `deletes T` permits deleting any subset of the objects that conform to `T`; a **receiver-scoped** entry `deletes self` permits deleting the receiver only; an **argument-scoped** entry `deletes p`, for a reference parameter `p`, permits deleting the argument bound to `p` only. Deleting any other object under a scoped entry receives `frame_violation`. For each field write the validator charges one `frame.write` before testing the write against the `modifies` grants, then one `frame.redefines-link` before following each `redefines` link, at those points of the [value accounting](../../../proposals/quire-v1/definitions/value-accounting.md); the first unavailable charge ends validation with that record's `incomplete` result and no frame verdict.

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
| FR-013-AC-8 | Under a receiver-scoped entry `self.f`, a write to `f` on the receiver is permitted and a write to `f` on any other object of the same type receives `frame_violation`; under an every-object entry `f`, both writes are permitted; an every-object and a receiver-scoped entry for one field refuse `invalid_model_binding`/`conflicting-binding`. | Test (TC-326) |
| FR-013-AC-9 | Under `deletes self`, deleting the receiver is permitted and deleting any other object receives `frame_violation`; under `deletes p`, deleting the argument bound to `p` is permitted and deleting any other object receives `frame_violation`; under `deletes T`, deleting any subset of the objects conforming to `T` is permitted. | Test (TC-326) |

## Dependencies

- [US-003](../../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/boundary.md) supplies the scoped context.
- [FR-151](../type-model/FR-151-resolve-conformance-redefinition-dispatch.md) owns the conformance rule making a `creates`/`deletes` grant subtype-inclusive under inheritance.
- [FR-340](../../objects/interfaces/FR-340-checked-package-frame-location.md) carries each `modifies` entry's scope in the checked package.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.

## References

- Receiver scope design: quire-spec-language ADR-021 (POR-3, RU-1, QS-4).
- Scoped `deletes`: quire-spec-language SR-1140 FND-004.
- Tickets: Linear STD-134; paired QSL-368.
