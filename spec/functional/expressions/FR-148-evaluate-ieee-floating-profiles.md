---
id: FR-148
title: "Evaluate explicit IEEE floating-point profiles"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
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

## IEEE profile

Complete V1 uses the IEEE 754-2019 binary32 and binary64 interchange formats.
Values may be constructed from exact bits, including every NaN payload and sign.
The required operations are addition, subtraction, multiplication, division,
square root and fused multiply-add under `nearest-even`, `nearest-away`,
`toward-zero`, `toward-positive` and `toward-negative` rounding. Fused
multiply-add performs one rounding after the unrounded product and sum.

Numeric equality, IEEE `totalOrder` and bit identity are distinct operations.
When an arithmetic operation must produce a quiet NaN from non-NaN operands,
the profile uses canonical quiet-NaN bits `0x7fc00000` for binary32 and
`0x7ff8000000000000` for binary64. Input NaN bits remain available to bit
identity, and every result retains the selected width, rounding mode and IEEE
exception flags as provenance.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-148-AC-1 | NaN, infinity and signed-zero vectors produce the selected equality and ordering results. | Test (TC-193) |
| FR-148-AC-2 | Changing rounding mode can change the bit-pattern result and therefore changes run provenance. | Test (TC-193) |
| FR-148-AC-3 | A backend lacking the selected width, rounding or exceptional-value policy refuses without decimal/rational substitution. | Test (TC-193) |
| FR-148-AC-4 | A fused multiply-add vector demonstrates its single-rounding result separately from a multiply followed by add. | Test (TC-193) |
| FR-148-AC-5 | The same NaN and signed-zero bit vectors distinguish numeric equality, total ordering and bit identity. | Test (TC-193) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
