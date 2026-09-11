---
id: Task-004
title: "Implement total-Boolean predicate projection"
type: Task
status: not_started
track: B
priority: P0
relationships:
  - { target: ix://agent-ix/quire-specification/Task-001, type: depends_on }
  - { target: ix://agent-ix/quire-specification/FR-095, type: references }
  - { target: ix://agent-ix/quire-specification/TC-117, type: verifies }
---
# Task-004: Implement total-Boolean predicate projection

## Scope

Contract IR implements FR-025's strict native predicate to TL proposition/signal
mapping and its non-value decisions.

## Subtasks

- [ ] Select accepted native/result/availability contracts and strict TL readers.
- [ ] Build canonical proposition and valuation correspondence records.
- [ ] Reject partial, non-Boolean, stale, unavailable and resource-exhausted inputs.

## Deliverables

- FR-025 implementation and its real-reader/property suite.

## Notes

- No hardcoded proposition or Boolean default is permitted before this lands.
