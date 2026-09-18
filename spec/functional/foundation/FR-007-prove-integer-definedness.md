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

Values and proof arithmetic are mathematical integers. No host overflow, inferred machine-width domain or implicit widening is admitted. The `quire.value.integer-division.*` identity the checked package retains selects the quotient and remainder law, as [FR-147](../expressions/FR-147-evaluate-integer-division-domains.md) states: under `quire.value.integer-division.truncating/v1` div truncates toward zero and a nonzero remainder has the dividend sign; under `quire.value.integer-division.floor/v1` and `quire.value.integer-division.euclidean/v1` the quotient and remainder are those of the retained law. Divisor zero and signed-minimum/-1 require exclusion under every retained law. Like-unit addition/comparison are admitted; multiplication/division/remainder require dimensionless operands.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-007-AC-1 | An unproved out-of-range sum receives undefined_expression. | Test |
| FR-007-AC-2 | A guarded sum proved within its declared bounds is admitted. | Test |
| FR-007-AC-3 | Where the checked package retains `quire.value.integer-division.truncating/v1`, negative seven divided by three yields negative two. | Test |
| FR-007-AC-4 | Where the checked package retains `quire.value.integer-division.truncating/v1`, negative seven remainder three yields negative one. | Test |
| FR-007-AC-5 | A possible zero divisor receives undefined_expression. | Test |
| FR-007-AC-6 | Incompatible units receive ill_typed. | Test |
| FR-007-AC-7 | Where the checked package retains `quire.value.integer-division.floor/v1`, negative seven divided by three yields negative three and negative seven remainder three yields two; where it retains `quire.value.integer-division.euclidean/v1`, the same operands yield negative three and two, as FR-147 specifies. | Test (TC-192) |

## Dependencies

- [US-003](../../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/state-semantics.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
