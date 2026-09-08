---
id: FR-012
title: "Bind operation observations to one invocation"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: implements
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

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-012-AC-1 | Different pre/post invocation identities receive wrong_snapshot. | Test |
| FR-012-AC-2 | A missing required pre observation receives unavailable_observation. | Test |
| FR-012-AC-3 | A post-only result used in current context receives wrong_snapshot. | Test |
| FR-012-AC-4 | A captured post reference retains its observation inside pre. | Test |

## Dependencies

- [US-003](../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../proposals/state-core/state-semantics.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
