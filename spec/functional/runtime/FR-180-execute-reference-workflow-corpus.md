---
id: FR-180
title: "Execute the complete reference workflow corpus"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-008
    type: references
  - target: ix://agent-ix/quire-specification/FR-114
    type: references
---

# FR-180: Execute the complete reference workflow corpus

## Description

When the reference corpus is executed, the composed runtime SHALL bind each
exact package, input revision and population/grouping rule to its declared
multidimensional expected results.

## Inputs

Version-locked ConfigVersion, timed-refund and fan-out/join/compensation packages;
snapshots/events/progress/populations; run limits.

## Outputs

Ordered native execution bundles and exact corpus comparison records.

## Behavior

Cross-workflow aggregates declare grouping keys and duplicate rules. The corpus
includes healthy, violating, boundary, missing/unknown, contradictory and
resource-incomplete cases. ConfigVersion includes 0, 1000 and just-outside
values. Timed-refund binds order/payment/refund identities and inclusive time
boundaries. Choreography covers choice, fan-out, every join policy, retry,
compensation and partial recovery.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-180-AC-1 | ConfigVersion native, generated-oracle, property and proof consumers agree on every bounded-corpus verdict. | Test (TC-209) |
| FR-180-AC-2 | Timed-refund cases distinguish wrong identity/amount, silence, progress, timeout, failure and achieved compensation. | Test (TC-209) |
| FR-180-AC-3 | Fan-out/join/compensation cases preserve branch, queue, attempt/effect and full outcome-partition semantics. | Test (TC-209) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
