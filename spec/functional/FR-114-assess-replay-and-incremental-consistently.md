---
id: FR-114
title: "Agree replay and incremental settled assessment"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
---
## Description

For the same admitted observations, binding revision, immutable captures, and progress/closure input, the assessment service SHALL produce the same settled per-obligation disposition in batch replay and incremental assessment.

## Behavior

Open prefixes, missing history, unsupported clauses, resource exhaustion, and ambiguous membership SHALL remain explicit non-settled or refused outcomes in both modes. A replay result SHALL NOT claim execution by a production monitor.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-114-AC-1 | Batch and incremental assessment agree for a satisfied and a missed-deadline obligation. | Test (TC-144) |
| FR-114-AC-2 | Open prefix, incomplete history, and ambiguous membership remain distinct from violation in both modes. | Test (TC-144) |
| FR-114-AC-3 | Unsupported capability and exhausted retained state remain explicit and cannot become settled success. | Test (TC-144) |

## Dependencies

- [Shared foundation](../../proposals/quire-v1/shared-foundation.md).
- E owns temporal semantics; B owns protocol assessment semantics.
