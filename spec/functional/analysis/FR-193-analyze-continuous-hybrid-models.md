---
id: FR-193
title: "Analyze continuous and hybrid models soundly"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
---

# FR-193: Analyze continuous and hybrid models soundly

## Description

When continuous or hybrid analysis is requested, the analyzer SHALL interpret
the declared modes, exact state/time domains, differential/algebraic constraints
and discrete transitions under a selected sound solver contract.

## Inputs

Hybrid model, initial/invariant/guard/reset definitions, time horizon or
unbounded claim, numeric enclosure profile, solver capability and limits.

## Outputs

Proved enclosure, reachable violation witness, inconclusive/unknown enclosure,
unsupported or failed result with soundness provenance.

## Behavior

Continuous evolution is constrained by mode invariants and declared equations;
discrete transitions apply exact guards/resets. Approximate numerical methods
must return sound enclosures under declared error bounds. A point estimate or
failed convergence cannot establish satisfaction.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-193-AC-1 | A selected provider returns a sound reachable enclosure with its numeric/error contract. | Test (TC-215) |
| FR-193-AC-2 | A reachable guard/reset violation returns a source-correspondent hybrid trace witness. | Test (TC-215) |
| FR-193-AC-3 | Unsupported equations, failed convergence or insufficient horizon returns inconclusive/unsupported. | Test (TC-215) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
