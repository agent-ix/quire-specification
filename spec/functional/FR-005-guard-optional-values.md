---
id: FR-005
title: "Require snapshot-qualified optional definedness"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: implements
---
# FR-005: Require snapshot-qualified optional definedness

## Description

When an optional value is unwrapped, the typechecker shall require a presence proof for that exact immutable value.

## Inputs

Resolved option types, expression paths, lexical bindings and observation-qualified guard facts.

## Outputs

A defined typed unwrap or undefined_expression.

## Behavior

True and false branch facts propagate in evaluation order. Alternative joins keep only universally valid facts. A stable let alias retains the captured optional value. A pre presence test cannot prove a post option present. Static typing assumes the runtime validation contract; it does not inspect a population.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-005-AC-1 | A guarded unwrap of the same option is defined. | Test |
| FR-005-AC-2 | An unguarded unwrap receives undefined_expression. | Test |
| FR-005-AC-3 | A pre guard used for a post unwrap receives undefined_expression. | Test |
| FR-005-AC-4 | A right-hand guard cannot justify an earlier evaluated unwrap. | Test |

## Dependencies

- [US-003](../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../proposals/state-core/state-semantics.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
