---
id: Task-006
title: "Implement exact native-to-TL correspondence"
type: Task
status: in_progress
track: C
priority: P0
relationships:
  - { target: ix://agent-ix/quire-specification/Task-004, type: depends_on }
  - { target: ix://agent-ix/quire-specification/Task-005, type: depends_on }
  - { target: ix://agent-ix/quire-specification/FR-095, type: references }
  - { target: ix://agent-ix/quire-specification/TC-113, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-117, type: verifies }
---
# Task-006: Implement exact native-to-TL correspondence

## Scope

Contract IR implements FR-026's strict correspondence construction and
result-binding path against selected public TL contracts.

## Subtasks

- [ ] Admit only event-position/fixed-sample bounded-future rows with exact equality.
- [ ] Preserve source, clause, predicate, capture, clock, progress and result axes.
- [ ] Refuse finite-window, timestamped and past requests without a formula.
- [ ] Execute independent differential, mutation and strict-reader tests.

## Deliverables

- FR-026 implementation and real TL integration evidence.

## Notes

- Equal formula bytes or truth never repair an unmatched correspondence.
- Qcir #71 is the sole remaining implementation owner. Its qcir predicate/
  correspondence contracts and required public TL owner APIs are merged, so the
  task is active rather than dependency-blocked.
