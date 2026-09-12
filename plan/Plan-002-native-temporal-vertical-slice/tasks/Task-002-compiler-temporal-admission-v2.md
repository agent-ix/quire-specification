---
id: Task-002
title: "Implement compiler temporal admission and `/2`"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - { target: ix://agent-ix/quire-specification/Task-001, type: depends_on }
  - { target: ix://agent-ix/quire-specification/FR-090, type: references }
  - { target: ix://agent-ix/quire-specification/TC-110, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-113, type: verifies }
---
# Task-002: Implement compiler temporal admission and `/2`

## Scope

Agent A adds native temporal AST/linking admission and the versioned compiled
protocol `/2` envelope while preserving strict `/1` reading.

## Subtasks

- [ ] Carry exact temporal definition identity/revision/digest.
- [ ] Encode tagged event-position, fixed-sample and timestamped clock fields.
- [ ] Refuse absent, conflicting, stale or mismatched selections before lowering.
- [ ] Add strict reader, canonical round-trip and mutation tests.

## Deliverables

- Compiler admission path and `/2` schema with Rust domain newtypes.

## Notes

- `/1` remains readable; it cannot be promoted to authenticated temporal execution.
