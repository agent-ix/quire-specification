---
id: Task-007
title: "Prove end-to-end native temporal conformance"
type: Task
status: not_started
track: A
priority: P0
relationships:
  - { target: ix://agent-ix/quire-specification/Task-006, type: depends_on }
  - { target: ix://agent-ix/quire-specification/FR-090, type: references }
  - { target: ix://agent-ix/quire-specification/FR-091, type: references }
  - { target: ix://agent-ix/quire-specification/FR-092, type: references }
  - { target: ix://agent-ix/quire-specification/FR-093, type: references }
  - { target: ix://agent-ix/quire-specification/FR-094, type: references }
  - { target: ix://agent-ix/quire-specification/FR-095, type: references }
  - { target: ix://agent-ix/quire-specification/NFR-040, type: references }
  - { target: ix://agent-ix/quire-specification/TC-110, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-111, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-112, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-113, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-114, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-115, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-116, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-117, type: verifies }
  - { target: ix://agent-ix/quire-specification/TC-118, type: verifies }
---
# Task-007: Prove end-to-end native temporal conformance

## Scope

Run cross-repository differential, replay-versus-incremental and fault-injection
evidence over the admitted vertical slice.

## Subtasks

- [ ] Compare independent native and selected TL oracle outcomes for the finite corpus.
- [ ] Compare offline replay and incremental settlement from equal admitted history.
- [ ] Inject overflow, resource and retention failures and prove no Boolean fallback.
- [ ] Publish exact versions, corpus cardinality and unsupported-domain disposition.

## Deliverables

- Reproducible conformance evidence and release-readiness review.

## Notes

- A resource stop prevents an exhaustive claim; it is not a passing result.
