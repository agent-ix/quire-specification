---
id: FR-008
title: "Preserve bounded sequence order and multiplicity"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-271
    type: references
---
# FR-008: Preserve bounded sequence order and multiplicity

## Description

When a sequence predicate is evaluated, the evaluator shall retain every occurrence in its authored order.

## Inputs

Typed sequences with explicit maximum lengths and a Boolean element predicate.

## Outputs

A completed quantifier value or an explicit unsupported/incomplete outcome.

## Behavior

forall and exists visit occurrences in order and stop at the first decisive result. Empty forall is true; empty exists is false. Missing sequence data is unavailable. Contextual size typing remains an explicit FS03 review decision. collect, sets, bags and flattening are outside the admitted initial profile.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-008-AC-1 | An empty universal predicate yields true. | Test |
| FR-008-AC-2 | An empty existential predicate yields false. | Test |
| FR-008-AC-3 | A two-occurrence sequence containing equal values has size two. | Test |
| FR-008-AC-4 | Missing sequence data receives unavailable_observation. | Test |
| FR-008-AC-5 | A collect expression receives unsupported_construct. | Test |

## Dependencies

- [US-003](../../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/state-semantics.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
