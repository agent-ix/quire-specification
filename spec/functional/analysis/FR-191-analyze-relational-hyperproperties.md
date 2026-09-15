---
id: FR-191
title: "Analyze relational and finite-trace-set hyperproperties"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
---

# FR-191: Analyze relational and finite-trace-set hyperproperties

## Description

When a relational or hyperproperty request is checked, the analyzer SHALL bind
each quantified execution/trace variable, correspondence and observation scope
explicitly.

## Inputs

Checked relational syntax, paired/multiple execution subjects, trace domains,
quantifier prefix, alignment relation and finite analysis bounds.

## Outputs

Proved, refuted with paired/trace-set counterexample, inconclusive, unsupported
or failed result.

## Behavior

Relational expressions compare typed values across named executions only.
Finite-trace-set hyperproperties quantify over the declared finite trace domain
and preserve quantifier order. Trace alignment, declassification/assumptions and
population completeness are explicit. No execution is duplicated or correlated
by local name alone.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-191-AC-1 | A two-execution relational property uses explicit input/output correspondences and returns a paired witness/counterexample. | Test (TC-213) |
| FR-191-AC-2 | Reversing a mixed quantifier prefix can change the finite trace-set result and is preserved. | Test (TC-213) |
| FR-191-AC-3 | Missing trace closure/alignment or exceeded domain bounds returns incomplete/requires-bound. | Test (TC-213) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
