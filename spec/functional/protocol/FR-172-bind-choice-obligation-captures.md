---
id: FR-172
title: "Bind choice evidence, obligations and captures"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-053
    type: references
  - target: ix://agent-ix/quire-specification/FR-055
    type: references
---

# FR-172: Bind choice evidence, obligations and captures

## Description

When activating a protocol obligation, the binder SHALL preserve its
whole-execution origin, owned-choice visibility evidence and ordered immutable
capture set.

## Inputs

Protocol/workflow instance, control occurrence, guard, choice owner/evidence,
activation boundary and capture expressions.

## Outputs

A uniquely keyed obligation activation with ordered captures, or typed
visibility/identity/definedness refusal.

## Behavior

Choice visibility is discharged only by an explicit communication or declared
shared fact observable by the role before its dependent action. Obligation keys
include the full originating execution/control identity. Capture expressions are
evaluated in authored order at the inclusive activation boundary; duplicates
remain separate occurrences with one value each.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-172-AC-1 | Explicit pre-action choice evidence discharges visibility for exactly the roles and branch it identifies. | Test (TC-203) |
| FR-172-AC-2 | Equal local obligation names in different execution/control origins produce distinct identities. | Test (TC-203) |
| FR-172-AC-3 | Duplicate capture expressions retain authored order and separate occurrence identities with inclusive-boundary values. | Test (TC-203) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
