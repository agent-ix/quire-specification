---
id: FR-006
title: "Preserve short-circuit and sharing semantics"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: traces_to
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

An implication records antecedent entry, its Boolean value if completed, and
consequent entry if that branch is entered. Each event identifies the authored
implication and corresponding operand region. A refused entry caused by an
exhausted work ceiling emits no entry event for that operand. Backend probe
availability and B's portable event representation remain separate contracts.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-006-AC-1 | A false implication antecedent yields true. | Test |
| FR-006-AC-2 | A skipped implication consequent has no evaluation event. | Test |
| FR-006-AC-3 | An if expression evaluates only the selected branch. | Test |
| FR-006-AC-4 | A let initializer is evaluated once. | Test |
| FR-006-AC-5 | An unreachable unknown name remains ill_typed. | Test |
| FR-006-AC-6 | A completed false antecedent records antecedent entry followed by value false, with no consequent entry. | Test |
| FR-006-AC-7 | A completed true antecedent followed by consequent evaluation records antecedent entry, value true and consequent entry in that order. | Test |
| FR-006-AC-8 | Exhaustion after a true antecedent but before consequent entry preserves the antecedent events and yields no implication Boolean or consequent-entry event. | Test |
| FR-006-AC-9 | Each reported implication event retains the exact authored source binding and its corresponding operand region. | Test |

## Examples

The following JSON is authored expectation notation for FR-006-AC-6/7/8. It is
not a portable result wire or observed evaluator output. Event tuples name
semantic observations; a consumer mapping must separately bind them to the
selected source artifact, implication and operand spans under FR-006-AC-9.
The incomplete example stops before consequent entry, so absence of that event
is an expected observation rather than a Boolean inference.

```json
[
  {
    "expression": "false implies false",
    "expectedCompletion": "completed",
    "expectedValue": true,
    "expectedEvents": [["antecedent", "reached"], ["antecedent", "value", false]]
  },
  {
    "expression": "true implies false",
    "expectedCompletion": "completed",
    "expectedValue": false,
    "expectedEvents": [["antecedent", "reached"], ["antecedent", "value", true], ["consequent", "reached"]]
  },
  {
    "expression": "true implies false",
    "stopAt": "after antecedent completion, before consequent entry",
    "expectedCompletion": "incomplete",
    "expectedDiagnostic": "resource_exhausted",
    "expectedEvents": [["antecedent", "reached"], ["antecedent", "value", true]]
  }
]
```

## Dependencies

- [US-003](../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../proposals/state-core/state-semantics.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
