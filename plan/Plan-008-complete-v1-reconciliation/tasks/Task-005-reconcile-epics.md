---
id: Task-005
title: "Reconcile existing epics and tickets"
type: Task
status: complete
track: B
priority: P0
relationships:
  - target: ix://agent-ix/quire-specification/Task-002
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-031
    type: references
  - target: ix://agent-ix/quire-specification/TC-020
    type: verifies
---
# Task-005: Reconcile existing epics and tickets

## Scope

Route every capability through the existing program graph before proposing new
children, preserving completed leaf evidence and eliminating closure by deferral.

## Subtasks

- [x] Audit the full issue graph beneath Research #36 and QSpec #1.
- [x] Classify each capability as satisfied, existing extension, missing child or
  justified missing child epic.
- [x] Define exact amendments for Research #65/#66/#67 and bounded parent epics.
- [x] Replace QSpec #1 closure-by-deferral with total V1 coverage.

## Deliverables

- Inventory-to-epic/ticket routing map.
- Exact existing issue amendments and justified new-child manifest.

## Result

`docs/v1-program-routing.md` assigns all 175 atomic rows to exactly one existing
primary epic and classifies their retained evidence and missing child work.
Twenty-one rows retain direct implementation/verification leaves; 154 need
semantic delivery children; all 175 still need qualification completion. No new
top-level epic is justified. Live parent tracking was repaired across Research,
QSpec, QSL, Protocol, Verification and Contract IR; QSL #1 was reopened without
reopening its completed children, Research #65/#66/#67 were promoted to required
delivery epics, and QSpec #63 was attached natively beneath QSpec #1.
