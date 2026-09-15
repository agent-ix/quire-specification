---
id: Task-006
title: "Design the complete modular V1 architecture"
type: Task
status: complete
track: C
priority: P0
relationships:
  - target: ix://agent-ix/quire-specification/Task-003
    type: depends_on
  - target: ix://agent-ix/quire-specification/Task-004
    type: depends_on
  - target: ix://agent-ix/quire-specification/Task-005
    type: depends_on
  - target: ix://agent-ix/quire-specification/StR-002
    type: references
  - target: ix://agent-ix/quire-specification/FR-020
    type: references
  - target: ix://agent-ix/quire-specification/TC-040
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-049
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-170
    type: verifies
---
# Task-006: Design the complete modular V1 architecture

## Scope

Design all language, semantic, runtime, analysis, mapping and qualification
subsystems together against the full inventory and existing implemented seams.

## Subtasks

- [x] Define the ten subsystem views and their ownership boundaries.
- [x] Define shared identities, types, package, failure and evolution contracts.
- [x] Define typed cross-subsystem interfaces and backend capability negotiation.
- [x] Exercise extension scenarios without architectural replacement.
- [x] Preserve exact refusal and prohibit approximation.

## Deliverables

- Complete root and subsystem architecture delta ready for `/specify`.
- Interface, dependency and migration decisions covering the full inventory.

## Result

`docs/v1-complete-architecture.md` defines four dependency planes, ten complete
subsystems, shared identity/type/package/evolution/failure contracts, 21 typed
interfaces, the acyclic Rust component graph, exact backend negotiation, ten
extension scenarios and the ordered migration. It assigns the composed runtime
facade to the already-planned `quire-integration` repository so QSL, Protocol,
Verification and the small generated-oracle runtime keep their authorities.
`docs/v1-architecture-allocation.md` maps every one of the corrected 176 atomic
rows to exactly one subsystem, one AD-002..AD-011 view, required interfaces and
its Task-005 epic. The audit added V1-TEMP-023 so the finite-evidence refusal is
paired with a positive sound unbounded-temporal capability.
