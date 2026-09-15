---
id: FR-171
title: "Evaluate joins, proved loops and dynamic roles"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-052
    type: references
  - target: ix://agent-ix/quire-specification/AD-007
    type: references
---

# FR-171: Evaluate joins, proved loops and dynamic roles

## Description

When checking dynamic protocol control, the checker SHALL validate each join,
loop and role-spawn against a closed control and population contract.

## Inputs

Control graph, join policy, branch states, loop invariant/variant, dynamic-role
population/lifetime/closure and resource bounds.

## Outputs

Checked control semantics or typed ambiguity, termination, closure or bound
refusal/incomplete outcome.

## Behavior

Join policies are `all`, `any`, `quorum(n)` or a named pure predicate and define
outstanding-branch cancellation/continuation. Loops require invariant preservation
and a well-founded variant for termination claims. Dynamic roles bind unique
instances to declared populations and lifetimes; finite execution requires a
discharged population bound.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-171-AC-1 | Every join policy settles only under its exact branch condition and applies its declared outstanding-branch behavior. | Test (TC-202) |
| FR-171-AC-2 | A loop with proved invariant/variant is admitted; an unproved termination claim is inconclusive/refused rather than silently bounded. | Test (TC-202) |
| FR-171-AC-3 | Spawn/lifetime/population closure creates and retires exact role instances; missing symbolic bounds return requires-bound for finite execution. | Test (TC-202) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
