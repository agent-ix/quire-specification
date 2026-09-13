---
id: Task-001
title: "Ratify cross-repository native-temporal architecture"
type: Task
status: done
track: A
priority: P0
relationships:
  - { target: ix://agent-ix/quire-specification/FR-090, type: references }
  - { target: ix://agent-ix/quire-specification/FR-095, type: references }
  - { target: ix://agent-ix/quire-specification/TC-110, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-113, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-117, type: verifies }
---
# Task-001: Ratify cross-repository native-temporal architecture

## Scope

Review and publish one boundary table joining native source, compiled `/2`,
observation/progress, Contract-IR projection/correspondence, TL and result
envelope contracts.

## Subtasks

- [x] Name exact owners, versions, identity fields and strict readers.
- [x] Freeze the support table and the finite-window false-extension discriminator.
- [x] Record Rust wire/newtype/resource constraints and refusal code ownership.

## Deliverables

- Reviewed architecture artifact and resolved cross-repo acceptance record.

## Notes

- Blocks all implementation work because a boundary omission would require an
  incompatible rework later.
