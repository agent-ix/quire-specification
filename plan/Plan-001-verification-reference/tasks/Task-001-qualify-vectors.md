---
id: Task-001
title: "Qualify schema and JCS vectors in Rust"
type: Task
status: in_progress
track: A
priority: P1
relationships:
  - target: ix://agent-ix/quire-specification/FR-017
    type: references
  - target: ix://agent-ix/quire-specification/FR-018
    type: references
  - target: ix://agent-ix/quire-specification/TC-001
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-002
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-003
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-004
    type: verifies
---
# Task-001: Qualify vectors

## Scope

Author positive and adverse vectors, then check the schema and canonical bytes
through actual Rust dependencies. Completion covers IT-003-SC-01/02/04 only.

## Subtasks

- [ ] Preserve independently authored canonical byte expectations.
- [ ] Validate the closed schema and one-axis shape mutations.
- [ ] Run B's pinned JCS API with distinct raw-byte and semantic identities.
- [ ] Record bounds, failures, dependency grants and local Rust gates.

## Deliverables

New versioned fixture data, qualification Rust tests/lockfile and reproducible
commands. No new shared reader or inferred B consumer acceptance.
