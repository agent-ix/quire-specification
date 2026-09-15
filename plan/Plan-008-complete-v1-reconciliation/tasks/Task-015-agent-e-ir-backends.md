---
id: Task-015
title: "Agent-E Contract IR, codegen, Kani and mapping campaign"
type: Task
status: not_started
track: E-E
priority: P0
relationships:
  - target: ix://agent-ix/quire-specification/Task-010
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/TC-217
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-218
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-219
    type: verifies
---
# Task-015: Agent-E IR and backend campaign

## Scope

Execute Contract IR #99–101/#55–58/#40, Contract Runtime #16 and Codegen
#48–50: complete IR, exact runtime/oracles, bounded Kani, native replay, output
mappings and backend qualification.

## Subtasks

- [ ] Land producer-before-consumer repositories in dependency order.
- [ ] Preserve closed bounded-Kani work and extend exact node families.
- [ ] Refuse unsupported or unbounded items per item without emitted substitutes.
- [ ] Pin and record every Kani/tool/dependency option in evidence.

## Deliverables

- Complete target-neutral IR and exact proof/mapping backend implementations.
