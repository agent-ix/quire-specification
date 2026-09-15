---
id: Task-003
title: "Audit existing specification coverage"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - target: ix://agent-ix/quire-specification/Task-002
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-020
    type: references
  - target: ix://agent-ix/quire-specification/FR-030
    type: references
  - target: ix://agent-ix/quire-specification/TC-020
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-030
    type: verifies
---
# Task-003: Audit existing specification coverage

## Scope

Read the complete existing QSpec corpus and classify exact coverage for every
inventory row before proposing any new or amended specification artifact.

## Subtasks

- [ ] Map every capability to current architecture, requirements, objects and tests.
- [ ] Identify complete, trace-only, bounded, compound, conflicting and missing coverage.
- [ ] Record exact amendment or reuse actions and affected matrices.
- [ ] Reject unsupported coverage claims based only on titles or indexes.

## Deliverables

- Inventory-to-existing-spec coverage map.
- Evidence-backed specification delta manifest.
