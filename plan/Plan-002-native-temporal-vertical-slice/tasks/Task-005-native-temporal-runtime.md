---
id: Task-005
title: "Implement native temporal runtime and result model"
type: Task
status: in_progress
track: A
priority: P0
relationships:
  - { target: ix://agent-ix/quire-specification/Task-002, type: depends_on }
  - { target: ix://agent-ix/quire-specification/Task-003, type: depends_on }
  - { target: ix://agent-ix/quire-specification/FR-091, type: references }
  - { target: ix://agent-ix/quire-specification/FR-092, type: references }
  - { target: ix://agent-ix/quire-specification/FR-093, type: references }
  - { target: ix://agent-ix/quire-specification/FR-094, type: references }
  - { target: ix://agent-ix/quire-specification/NFR-040, type: references }
  - { target: ix://agent-ix/quire-specification/TC-111, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-112, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-114, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-115, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-116, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-118, type: verifies }
---
# Task-005: Implement native temporal runtime and result model

## Scope

Implement native future/past evaluation, trigger instances and closed result
axes for valid truth, pending, incomplete and refusal outcomes.

## Subtasks

- [ ] Use pointwise connectors and exact inclusive bounds.
- [ ] Preserve captures and distinguish false from unestablished guards.
- [ ] Settle silence only from validated progress and preserve sibling results.
- [ ] Charge checked work/retention before expansion; retain required state.

## Deliverables

- Offline and incremental evaluator with shared typed result envelope.

## Notes

- Do not lower to TL here; current past and finite-window correspondence stays unsupported.
- The strict TL syntax, parser, evaluator and rewrite owner APIs are merged. Qcir #71 owns the remaining
  native composition; this task is not complete until that executable path lands.
