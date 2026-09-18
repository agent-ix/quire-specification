---
id: FR-008
title: "Preserve bounded sequence order and multiplicity"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-271
    type: references
  - target: ix://agent-ix/quire-specification/FR-144
    type: references
  - target: ix://agent-ix/quire-specification/FR-145
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

forall and exists visit occurrences in order and stop at the first decisive
result. Empty forall is true; empty exists is false. Missing sequence data is
unavailable. Contextual size typing remains an explicit FS03 review decision.
These four rules hold under every selection that admits a sequence.

This requirement owns sequence order and multiplicity. The set, bag and
ordered-set domain is owned by
[FR-144](../type-model/FR-144-preserve-complete-collection-algebra.md), and the
`map`/`collect`, `filter`, `flatten`, `fold`, `reduce` and `convert` forms are
owned by
[FR-145](../expressions/FR-145-convert-and-query-collections.md).

Where `quire.state.queries/v1` is selected without `quire.value.complete/v1`,
the frontend shall refuse each of those forms with `unsupported_construct` and
its source location.

Where `quire.value.complete/v1` is selected, the frontend shall admit each of
those forms under FR-144 and FR-145 while retaining the sequence order,
multiplicity, empty-quantifier truth and missing-data outcome specified here.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-008-AC-1 | An empty universal predicate yields true. | Test |
| FR-008-AC-2 | An empty existential predicate yields false. | Test |
| FR-008-AC-3 | A two-occurrence sequence containing equal values has size two. | Test |
| FR-008-AC-4 | Missing sequence data receives unavailable_observation. | Test |
| FR-008-AC-5 | Where `quire.state.queries/v1` is selected without `quire.value.complete/v1`, a `collect`, `flatten`, set, bag or ordered-set form receives unsupported_construct with its source location. | Test |
| FR-008-AC-6 | Where `quire.value.complete/v1` is selected, the same forms are admitted under FR-144 and FR-145, and sequence order, multiplicity, empty-quantifier truth and the missing-data outcome remain exactly as specified here. | Test (TC-189, TC-190) |

## Dependencies

- [US-003](../../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/state-semantics.md) supplies the scoped context.
- [FR-144](../type-model/FR-144-preserve-complete-collection-algebra.md) owns the complete collection-kind algebra.
- [FR-145](../expressions/FR-145-convert-and-query-collections.md) owns every collection query and conversion form.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
