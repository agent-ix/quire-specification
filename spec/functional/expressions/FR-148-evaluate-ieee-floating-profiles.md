---
id: FR-148
title: "Evaluate explicit IEEE floating-point profiles"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
---

# FR-148: Evaluate explicit IEEE floating-point profiles

## Description

When a source selects an IEEE floating-point profile, the evaluator SHALL use
its declared width, rounding mode and exceptional-value comparison policy.

## Inputs

Binary width, rounding mode, bit-pattern values and arithmetic/comparison
operations.

## Outputs

Exact profile bit-pattern result, Boolean comparison or typed refusal.

## Behavior

NaN payloads, infinities and signed zero remain representable. Numeric equality,
total ordering and bitwise identity are distinct selected operations. Each
arithmetic step rounds once under the declared mode. Cross-width or exact-number
conversion is explicit and reports loss.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-148-AC-1 | NaN, infinity and signed-zero vectors produce the selected equality and ordering results. | Test (TC-193) |
| FR-148-AC-2 | Changing rounding mode can change the bit-pattern result and therefore changes run provenance. | Test (TC-193) |
| FR-148-AC-3 | A backend lacking the selected width, rounding or exceptional-value policy refuses without decimal/rational substitution. | Test (TC-193) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
