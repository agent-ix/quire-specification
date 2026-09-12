---
id: FR-009
title: "Compare objects by declared identity"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-204
    type: references
  - target: ix://agent-ix/quire-specification/FR-271
    type: references
---
# FR-009: Compare objects by declared identity

## Description

When identity-bearing objects are compared, the evaluator shall compare their universe, type and object identities.

## Inputs

Validated typed references or object access values with observation correspondence.

## Outputs

Identity equality or a foreign-universe/type refusal.

## Behavior

Equal fields do not imply equal objects. Immutable observations remain attached to field reads. Identity comparison across pre/post is permitted only within the same declared universe. Structural record equality never replaces object identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-009-AC-1 | Equal-valued objects with different identities compare unequal. | Test |
| FR-009-AC-2 | The same object across pre/post of one universe compares equal. | Test |
| FR-009-AC-3 | Coincident object labels from different universes receive foreign_reference. | Test |

## Dependencies

- [US-003](../../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/state-semantics.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
