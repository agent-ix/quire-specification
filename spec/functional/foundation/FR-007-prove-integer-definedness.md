---
id: FR-007
title: "Prove bounded integer operations"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-206
    type: references
  - target: ix://agent-ix/quire-specification/FR-147
    type: references
---
# FR-007: Prove bounded integer operations

## Description

When an integer expression is linked, the typechecker shall prove its definedness within the selected declared bounds.

## Inputs

Explicit bounded scalar declarations, exact units, literals and guard-refined operand domains.

## Outputs

A defined integer expression or a located typing/definedness refusal.

## Behavior

Values and proof arithmetic are mathematical integers. No host overflow, inferred machine-width domain or implicit widening is admitted. Where the historical bounded integer profile is selected, div truncates toward zero and a nonzero remainder has the dividend sign. Where a complete-V1 integer profile is selected, the frontend shall apply the division law that profile selects, as [FR-147](../expressions/FR-147-evaluate-integer-division-domains.md) states. Divisor zero and signed-minimum/-1 require exclusion under every profile. Like-unit addition/comparison are admitted; multiplication/division/remainder require dimensionless operands.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-007-AC-1 | An unproved out-of-range sum receives undefined_expression. | Test |
| FR-007-AC-2 | A guarded sum proved within its declared bounds is admitted. | Test |
| FR-007-AC-3 | Where the historical bounded integer profile is selected, negative seven divided by three yields negative two. | Test |
| FR-007-AC-4 | Where the historical bounded integer profile is selected, negative seven remainder three yields negative one. | Test |
| FR-007-AC-5 | A possible zero divisor receives undefined_expression. | Test |
| FR-007-AC-6 | Incompatible units receive ill_typed. | Test |
| FR-007-AC-7 | Where a complete-V1 integer profile is selected, negative seven divided by and remainder three take the quotient and remainder of the law that profile selects, as FR-147 specifies. | Test (TC-192) |

## Dependencies

- [US-003](../../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/state-semantics.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
