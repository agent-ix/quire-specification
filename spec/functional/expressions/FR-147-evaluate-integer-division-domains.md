---
id: FR-147
title: "Evaluate mathematical and bounded integer division"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-007
    type: references
  - target: ix://agent-ix/quire-specification/FR-044
    type: references
---

# FR-147: Evaluate mathematical and bounded integer division

## Description

When an integer division or remainder operator is selected, the evaluator SHALL
apply its exact profile-declared quotient/remainder law and result domain.

## Inputs

Integer type, operands, selected Euclidean or truncating division profile and
consumer-domain bounds.

## Outputs

Exact quotient/remainder values or typed zero/domain/refusal outcomes.

## Behavior

Mathematical integers are unbounded language values. `div`/`rem` use the
selected law and satisfy its quotient identity; operator spellings cannot mix
laws. Bounded consumers prove operand/intermediate/result membership. Division
by zero is undefined and no backend may saturate or substitute float division.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-147-AC-1 | Positive and negative operand vectors satisfy the selected quotient/remainder identity exactly. | Test (TC-192) |
| FR-147-AC-2 | Division by zero refuses definedness and produces no numeric value. | Test (TC-192) |
| FR-147-AC-3 | A finite backend lacking a complete derived domain returns requires-bound instead of narrowing mathematical integers. | Test (TC-192) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
