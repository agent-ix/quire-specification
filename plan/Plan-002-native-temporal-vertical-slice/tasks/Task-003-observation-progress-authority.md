---
id: Task-003
title: "Implement observation and progress authority"
type: Task
status: in_progress
track: B
priority: P0
relationships:
  - { target: ix://agent-ix/quire-specification/Task-001, type: depends_on }
  - { target: ix://agent-ix/quire-specification/FR-093, type: references }
  - { target: ix://agent-ix/quire-specification/FR-094, type: references }
  - { target: ix://agent-ix/quire-specification/TC-114, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-115, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-116, type: verifies }
---
# Task-003: Implement observation and progress authority

## Scope

Agent F supplies typed semantic-event, correlation, causal-order, completeness
and monotonic-progress assertions for temporal consumers.

## Subtasks

- [ ] Implement idempotent identical receipt and conflicting identity refusal.
- [ ] Bind progress to its source/clock/subject and inclusive endpoint.
- [ ] Preserve predecessor/replay identity and identify late contradictions.

## Deliverables

- Strict observation/progress contract and executable vectors.

## Notes

- EOF and ingestion order never substitute for closure or causal authority.
