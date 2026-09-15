---
id: Task-002
title: "Build the atomic master capability inventory"
type: Task
status: in_progress
track: A
priority: P0
relationships:
  - target: ix://agent-ix/quire-specification/Task-001
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-031
    type: references
  - target: ix://agent-ix/quire-specification/FR-032
    type: references
  - target: ix://agent-ix/quire-specification/TC-020
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-032
    type: verifies
---
# Task-002: Build the atomic master capability inventory

## Scope

Replace compound and deferred scope accounting with one evidence-backed atomic
inventory spanning language, runtimes, analyzers, mappings and qualification.

## Subtasks

- [ ] Atomize all 51 current capability groups.
- [ ] Add every reserved construct and prose-only exclusion.
- [ ] Census existing specification artifacts, public code surfaces and tickets.
- [ ] Record independent specification, implementation, verification,
  integration and qualification states.
- [ ] Eliminate forbidden delivery dispositions and unowned work.

## Deliverables

- Maintained atomic capability crosswalk and ledger schema.
- Machine-auditable source, artifact, code and ticket census.
