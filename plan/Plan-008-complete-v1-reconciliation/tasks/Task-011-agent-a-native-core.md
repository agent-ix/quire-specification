---
id: Task-011
title: "Agent-A complete native compiler, runtime and tooling campaign"
type: Task
status: not_started
track: E-A
priority: P0
relationships:
  - target: ix://agent-ix/quire-specification/Task-010
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-131
    type: references
  - target: ix://agent-ix/quire-specification/FR-140
    type: references
  - target: ix://agent-ix/quire-specification/FR-150
    type: references
  - target: ix://agent-ix/quire-specification/FR-180
    type: references
  - target: ix://agent-ix/quire-specification/FR-300
    type: references
  - target: ix://agent-ix/quire-specification/FR-339
    type: references
  - target: ix://agent-ix/quire-specification/TC-180
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-185
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-195
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-209
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-220
    type: verifies
---
# Task-011: Agent-A native core campaign

## Scope

Execute QSL #116–123 and WASM #6 end to end: complete source/package and CST,
value/expression/model semantics, reference execution/simulation, lifecycle and
native tooling, then qualification.

## Subtasks

- [ ] Complete each ticket in dependency order with one ticket/PR.
- [ ] Continue to the next unblocked ticket without routine reassignment.
- [ ] Publish only merged pinned interfaces for downstream lanes.
- [ ] Reconcile inventory, matrices and parent tracking after every merge.

## Deliverables

- Complete QSL compiler/reference-runtime/tooling implementation and evidence.
- Exact WASM parity for the bounded pure lifecycle surface.
