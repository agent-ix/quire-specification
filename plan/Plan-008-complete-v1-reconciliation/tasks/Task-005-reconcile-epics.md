---
id: Task-005
title: "Reconcile existing epics and tickets"
type: Task
status: not_started
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

- [ ] Audit the full issue graph beneath Research #36 and QSpec #1.
- [ ] Classify each capability as satisfied, existing extension, missing child or
  justified missing child epic.
- [ ] Define exact amendments for Research #65/#66/#67 and bounded parent epics.
- [ ] Replace QSpec #1 closure-by-deferral with total V1 coverage.

## Deliverables

- Inventory-to-epic/ticket routing map.
- Exact existing issue amendments and justified new-child manifest.
