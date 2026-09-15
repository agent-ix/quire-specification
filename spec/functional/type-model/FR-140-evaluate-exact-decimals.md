---
id: FR-140
title: "Evaluate exact decimal values"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
---

# FR-140: Evaluate exact decimal values

## Description

When checking a decimal expression, the semantic kernel SHALL represent every
value as an exact signed coefficient and declared nonnegative scale.

## Inputs

Decimal types with coefficient/scale bounds, literals, operands and explicit
conversion requests.

## Outputs

An exact decimal, typed loss/refusal, or incomplete resource outcome.

## Behavior

Literal spelling determines an exact coefficient/scale before normalization.
Equality compares mathematical values; declared representation constraints
remain validation conditions. Arithmetic derives exact intermediate scales and
requires an explicit rounding rule only when the result type cannot represent
the exact value. No binary float is used as an intermediate.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-140-AC-1 | `1.0` and `1.00` compare mathematically equal while retaining their declared representation provenance. | Test (TC-185) |
| FR-140-AC-2 | Exact representable arithmetic returns the mathematical result without rounding. | Test (TC-185) |
| FR-140-AC-3 | An unrepresentable result without a selected rounding rule refuses; an explicit rule returns the rounded value plus loss record. | Test (TC-185) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
