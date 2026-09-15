---
id: FR-146
title: "Check total pure functions and recursion"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-033
    type: references
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
---

# FR-146: Check total pure functions and recursion

## Description

When admitting a function declaration, the checker SHALL establish typed
purity, total definedness and a well-founded termination argument for every
reachable call cycle.

## Inputs

Function declarations, parameter/result types, call graph, effects declaration,
termination measure and selected type/model closure.

## Outputs

Checked callable identities or located type, effect, definedness or termination
refusals.

## Behavior

Functions may return any complete-V1 value type. Calls bind exact declaration
identity, arity and ordered arguments. Recursion uses a declared well-founded
measure proven to decrease on every cycle. Reflection, mutation, I/O and ambient
lookups are prohibited. Runtime depth/fuel exhaustion remains incomplete.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-146-AC-1 | A pure recursive function with a proven decreasing finite measure is admitted and evaluates its declared result. | Test (TC-191) |
| FR-146-AC-2 | A nondecreasing or unproved call cycle refuses with the cycle and failed obligation. | Test (TC-191) |
| FR-146-AC-3 | I/O, mutation, reflection, ambient access or wrong argument/result typing refuses even in an unreachable branch. | Test (TC-191) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
