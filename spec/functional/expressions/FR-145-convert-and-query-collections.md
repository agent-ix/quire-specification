---
id: FR-145
title: "Convert and query every collection kind explicitly"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-041
    type: references
  - target: ix://agent-ix/quire-specification/FR-144
    type: references
---

# FR-145: Convert and query every collection kind explicitly

## Description

When applying a collection conversion or query, the evaluator SHALL derive the
result kind, order, uniqueness, multiplicity and bound from the selected
operation contract.

## Inputs

Bounded collection, typed pure function/predicate, target kind and result bound.

## Outputs

A typed collection/scalar result, explicit loss record, refusal or incomplete
outcome.

## Behavior

`map` and `collect` are one operation. Filter preserves the source kind where
closed under filtering. Flatten defines source/element kind combinations and
combines occurrence order/multiplicity without implicit deduplication. Fold and
reduce select a catalog operation, empty identity, associativity requirement and
intermediate domain. Conversions that lose order or duplicates report that loss.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-145-AC-1 | Map/filter/flatten over each collection kind returns the specified kind and deterministic semantic occurrences. | Test (TC-190) |
| FR-145-AC-2 | Fold on an empty collection uses only a declared identity; a reduction without one refuses. | Test (TC-190) |
| FR-145-AC-3 | A conversion that loses order or multiplicity requires explicit acceptance and records the exact loss. | Test (TC-190) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
