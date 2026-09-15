---
id: FR-161
title: "Analyze native infinite-trace temporal properties"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-002
    type: references
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
---

# FR-161: Analyze native infinite-trace temporal properties

## Description

When an infinite-trace temporal property is requested, the analyzer SHALL apply
the selected unbounded semantic profile and report only conclusions justified
by its declared soundness premises.

## Inputs

Checked unbounded formula, transition/model subject, fairness assumptions,
provider capability/profile, limits and exact tool pins.

## Outputs

Proved, refuted, inconclusive, unsupported or failed result with premises,
witness/counterexample and provenance.

## Behavior

Unbounded always/eventually/until/release and past duals have infinite-trace
meaning independent of one solver encoding. A valid finite counterexample may
refute an applicable property. Finite prefixes, absence of a counterexample or
resource exhaustion cannot prove infinite satisfaction.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-161-AC-1 | A provider proves or refutes only under the exact transition/fairness premises and returns replayable evidence. | Test (TC-200) |
| FR-161-AC-2 | A finite-prefix monitor returns pending/inconclusive rather than proved for an unresolved liveness property. | Test (TC-200) |
| FR-161-AC-3 | Unsupported operators, missing fairness/model closure or exhausted analysis return distinct non-conclusive dispositions. | Test (TC-200) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
