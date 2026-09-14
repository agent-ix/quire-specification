---
id: Task-003
title: "Implement observation and progress authority"
type: Task
status: done
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

- [x] Implement idempotent identical receipt and conflicting identity refusal.
- [x] Bind progress to its source/clock/subject and inclusive endpoint.
- [x] Preserve predecessor/replay identity and identify late contradictions.

## Deliverables

- Strict observation/progress contract and executable vectors.

## Notes

- EOF and ingestion order never substitute for closure or causal authority.
- Completed by `quire-observation` Plan-001 / PR #19 (`9ac80e9`). Observation
  #18/#13 retain the separate producer-qualified subject/topology adoption.
