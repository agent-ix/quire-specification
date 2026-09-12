---
id: FR-012
title: "Bind operation observations to one invocation"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: traces_to
---
# FR-012: Bind operation observations to one invocation

## Description

When an operation clause is evaluated, the runtime validator shall select observations from the same declared invocation.

## Inputs

Operation identity, parameters, result, pre/post references, context identity and universe.

## Outputs

Validated current/pre/post context or correspondence diagnostics.

## Behavior

Preconditions select pre; postconditions select post. result is post-only. Proposed pre(expression) selects pre for context-dependent reads while preserving already captured lexical values. A deleted required post self is unavailable as a post context even if deletion is permitted.

Operation parameters are immutable values from the selected invocation, checked
against that operation's declared parameter types. Selecting another observation
does not replace a parameter with a similarly named object field. The proposed
initial anchor rule admits pre(expression) only inside a postcondition.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-012-AC-1 | Different pre/post invocation identities receive wrong_snapshot. | Test |
| FR-012-AC-2 | A missing required pre observation receives unavailable_observation. | Test |
| FR-012-AC-3 | A post-only result used in current context receives wrong_snapshot. | Test |
| FR-012-AC-4 | A captured post reference retains its observation inside pre. | Test |
| FR-012-AC-5 | A precondition reading self.versionNumber evaluates against pre even when post contains a different value for the same object. | Test |
| FR-012-AC-6 | An operation parameter retains the invocation-supplied value in both precondition and postcondition evaluation. | Test |
| FR-012-AC-7 | Applying pre to an immutable parameter in a postcondition preserves that parameter's value. | Test |
| FR-012-AC-8 | Using pre in an invariant or precondition receives wrong_snapshot at linking. | Test |

## Examples

These are specification hypotheses for the criteria above, not compiled model
packages or observed evaluator output. Assume one resolved operation declares
an expectedVersion parameter with the same bounded integer type as
self.versionNumber. One invocation supplies expectedVersion = 1. Its pre and
post snapshots contain the same object with versionNumber = 1 and 2 respectively.
Both snapshots satisfy the declared population and frame validation contracts.

| Clause context | Expression | Expected completed value | Criteria |
| --- | --- | --- | --- |
| Precondition | self.versionNumber = expectedVersion | true | FR-012-AC-5, FR-012-AC-6 |
| Postcondition | self.versionNumber = expectedVersion | false | FR-012-AC-6 |
| Postcondition | expectedVersion = pre(expectedVersion) | true | FR-012-AC-7 |

The corresponding precondition containing pre(expectedVersion) refuses at
linking under FR-012-AC-8, so it has no completed Boolean. Existing zero-parameter
producer fixtures do not establish this parameterized operation contract.

## Dependencies

- [US-003](../../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/state-semantics.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
