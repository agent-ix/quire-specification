---
id: FR-151
title: "Resolve conformance, redefinition and closed dispatch"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-150
    type: references
---

# FR-151: Resolve conformance, redefinition and closed dispatch

## Description

When checking specialization or a dispatched call, the model checker SHALL
apply the selected conformance, subsetting, redefinition and method-precedence
rules over a closed declaration set.

## Inputs

Original/effective declarations, inheritance graph, variance rules,
subsetting/redefinition edges and complete method set.

## Outputs

Validated effective declarations and selected method, or located cycle,
incompatibility, ambiguity or incomplete-closure refusal.

## Behavior

Substitutability checks value, multiplicity, pre/postcondition and effect
variance under the selected profile. Redefinition retains correspondence and
cannot weaken inherited invariants. Dispatch selects the unique most-specific
applicable method; registration or source order never resolves ambiguity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-151-AC-1 | A compatible specialization/redefinition yields one effective member with complete provenance. | Test (TC-196) |
| FR-151-AC-2 | An inheritance cycle, weakened invariant or incompatible variance refuses with the contributing edges. | Test (TC-196) |
| FR-151-AC-3 | Zero or multiple most-specific methods return no dispatch result and identify missing closure or ambiguity. | Test (TC-196) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
