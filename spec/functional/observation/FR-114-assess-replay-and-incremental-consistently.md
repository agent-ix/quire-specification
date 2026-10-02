---
id: FR-114
title: "Agree replay and incremental settled assessment"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
  - target: ix://agent-ix/quire-specification/FR-161
    type: references
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
---
## Description

For the same admitted observations, binding selection, immutable captures, and
progress/closure input, the assessment service SHALL produce the same settled
per-obligation disposition in batch replay and incremental assessment.

## Behavior

An open prefix that lacks decisive support, missing history, unsupported clauses,
resource exhaustion, and ambiguous membership SHALL remain explicit non-settled
or refused outcomes in both modes. An open subject with decisive witness or
decisive-counterexample support MAY retain its settled truth while its
progress/closure remains open; that result SHALL retain the exact settlement
basis and decision support. A replay result SHALL NOT claim execution by a
production monitor.

When the temporal analysis reports settlement basis `unsettled` (FR-243,
[FR-161](../temporal/FR-161-analyze-infinite-trace-properties.md)
`inconclusive`, [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)),
the QObs assessment SHALL agree with that non-conclusive disposition in both
batch and incremental modes and SHALL NOT resolve it to a settled truth in
either mode; agreement on `unsettled` requires the same progress identity, not
merely the same label.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-114-AC-1 | Batch and incremental assessment agree for a decisive-witness satisfied obligation, a decisive-counterexample violated obligation, and a missed-deadline obligation. | Test (TC-144) |
| FR-114-AC-2 | An open prefix without a recorded decisive witness or counterexample, incomplete history, and ambiguous membership remain distinct from violation in both modes. | Test (TC-144) |
| FR-114-AC-3 | Unsupported capability and exhausted retained state remain explicit and cannot become settled success. | Test (TC-144) |
| FR-114-AC-4 | For a temporal result carrying settlement basis `unsettled`, QObs batch and incremental assessment agree on the non-conclusive disposition and its exact progress identity, and neither mode resolves it to a settled truth. | Test (TC-239) |

## Dependencies

- [Shared foundation](../../../proposals/quire-v1/shared-foundation.md).
- E owns temporal semantics; B owns protocol assessment semantics.

## Status

Accepted by FS04 for replay/incremental equivalence and explicit non-settled
outcomes. Executable full-stack assessment remains Planned.
