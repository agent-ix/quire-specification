---
id: FR-160
title: "Evaluate uncertain, partial and revisioned temporal inputs"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-002
    type: references
  - target: ix://agent-ix/quire-specification/FR-113
    type: references
---

# FR-160: Evaluate uncertain, partial and revisioned temporal inputs

## Description

When temporal inputs are partial, clock-uncertain or superseded, the evaluator SHALL propagate their exact possibility and revision sets into the temporal result.

## Inputs

Checked formula, known/missing/conflicting atom valuations, exact clock intervals,
observation revisions, progress/closure and resource limits.

## Outputs

Definite true/false, pending/indeterminate, incomplete, refused or failed result
with revision lineage.

## Behavior

Partial atoms carry possible truth values and reasons. Clock intervals use exact
unit arithmetic and establish order only when intervals require it. A revisioned
repair invalidates the affected suffix/window, recomputes under the same package
and links retained superseded and replacement results. Missing values and
midpoint guesses never become false facts.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-160-AC-1 | Known, missing and conflicting valuations produce their sound multi-valued temporal outcomes. | Test (TC-199) |
| FR-160-AC-2 | Overlapping clock intervals retain ordering uncertainty while disjoint intervals establish the required order. | Test (TC-199) |
| FR-160-AC-3 | An out-of-order revision recomputes exactly the affected result region and preserves old/new lineage. | Test (TC-199) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
