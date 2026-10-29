---
id: Task-001
title: "Qualify schema and JCS vectors in Rust"
type: Task
status: done
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

- [x] Author canonical-form vectors checked against the JCS rules.
- [x] Validate the closed schema and one-axis shape mutations.
- [x] Run B's JCS API with distinct raw-byte and semantic identities.

Observed: five local tests pass, formatting and strict all-target/all-feature
Clippy pass.
Raw duplicate detection is a recorded existing B reader gap; no strict intake
or semantic matching qualification is claimed. IT-003-SC-03 remains planned.

## Deliverables

New versioned fixture data, qualification Rust tests/lockfile and reproducible
commands. No new shared reader or inferred B consumer acceptance.
