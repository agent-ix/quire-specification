---
id: FR-144
title: "Preserve complete collection-kind semantics"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-008
    type: references
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
---

# FR-144: Preserve complete collection-kind semantics

## Description

When constructing or comparing a collection, the evaluator SHALL preserve the
selected sequence, set, bag or ordered-set algebra.

## Inputs

Element type, collection kind, declared cardinality bound and occurrences.

## Outputs

A typed bounded collection/equality result or refusal.

## Behavior

Sequences preserve order and duplicates. Sets preserve membership without
duplicates and expose no semantic iteration order. Bags preserve multiplicity
without semantic order. Ordered sets preserve first-occurrence order and
uniqueness. Equality follows those rules and all materialization checks the
declared maximum before allocation.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-144-AC-1 | Permuting a set or bag does not change equality, while permuting a sequence or ordered set can. | Test (TC-189) |
| FR-144-AC-2 | Duplicate insertion is preserved by sequence/bag and eliminated by set/ordered set according to their defined order rule. | Test (TC-189) |
| FR-144-AC-3 | Missing or exceeded cardinality bounds refuse without partial materialization. | Test (TC-189) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
