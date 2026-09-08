---
id: FR-006
title: "Preserve short-circuit and sharing semantics"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: implements
---
# FR-006: Preserve short-circuit and sharing semantics

## Description

When a control expression is evaluated, the evaluator shall follow its specified branch order.

## Inputs

Well-typed Boolean operands, value conditionals and immutable let expressions.

## Outputs

The selected value and source activation events, or incompleteness.

## Behavior

and, or and implies evaluate their left operand once and skip the right operand on false, true and false respectively. Value if evaluates only its selected branch. Let evaluates its initializer once. Implication antecedent and consequent events retain authored correspondence. Unknown names and unsupported constructs remain errors in unreachable branches.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-006-AC-1 | A false implication antecedent yields true. | Test |
| FR-006-AC-2 | A skipped implication consequent has no evaluation event. | Test |
| FR-006-AC-3 | An if expression evaluates only the selected branch. | Test |
| FR-006-AC-4 | A let initializer is evaluated once. | Test |
| FR-006-AC-5 | An unreachable unknown name remains ill_typed. | Test |

## Dependencies

- [US-003](../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../proposals/state-core/state-semantics.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
