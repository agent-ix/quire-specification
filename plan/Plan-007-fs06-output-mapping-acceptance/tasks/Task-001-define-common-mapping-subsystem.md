---
id: Task-001
title: "Define the common output-mapping subsystem"
type: Task
status: done
track: A
priority: P0
relationships:
  - target: ix://agent-ix/quire-specification/FR-120
    type: references
  - target: ix://agent-ix/quire-specification/FR-121
    type: references
  - target: ix://agent-ix/quire-specification/FR-125
    type: references
  - target: ix://agent-ix/quire-specification/FR-297
    type: references
  - target: ix://agent-ix/quire-specification/FR-299
    type: references
---
# Task-001: Define the common output-mapping subsystem

## Scope

Accept one target-selection, per-obligation loss-record and immutable-package
boundary shared by all three output mappers without making target text source.

## Subtasks

- [x] Define exact request/profile selection and whole-request refusal.
- [x] Require one non-approximate record per requested source obligation.
- [x] Close mapping identity and qualified-adequacy gaps in FR-269.
- [x] Define deterministic package identity, checked limits and no partial output.
- [x] Keep parser/tool observations downstream and non-semantic.

## Deliverables

- AD-004; FR-120/121/125; FR-297–299; NFR-060/061.
- Common output-mapping profile `quire.output.mapping/v1` `1-draft.1`.
- Portable controls TC-150, TC-154 and TC-155.
