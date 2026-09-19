---
id: FR-180
title: "Execute the complete reference workflow corpus"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
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

## Reference verdict contract

Each corpus case's source/package bytes are emitted by I04 `link_and_check`
as `quire.checked-package/v2` with its `state` nodes; the corpus pins those
exact bytes and the case's `package_id`, and the runtime admits them only
through I04 `read`. ConfigVersion cases replay on the complete-V1 value
runtime only.

Each corpus case fixes source/package bytes, model and observation inputs,
closure/progress, limits and expected per-clause disposition. ConfigVersion uses
the inclusive integer domain `0..=1000`; `-1` and `1001` are refusal vectors.
Timed refund keys every payment and refund to the same order identity and checks
the inclusive deadline against exact event-time intervals. Choreography cases
enumerate every join policy and independently vary request receipt, business
effect, compensation attempt and achieved recovery. Native execution is the
semantic reference; generated consumers must agree on the complete typed
disposition, not merely a Boolean projection.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-180-AC-1 | ConfigVersion native, generated-oracle, property and proof consumers agree on every bounded-corpus verdict. | Test (TC-209, IT-075) |
| FR-180-AC-2 | Timed-refund cases distinguish wrong identity/amount, silence, progress, timeout, failure and achieved compensation. | Test (TC-209) |
| FR-180-AC-3 | Fan-out/join/compensation cases preserve branch, queue, attempt/effect and full outcome-partition semantics. | Test (TC-209) |
| FR-180-AC-4 | ConfigVersion includes 0, 1000, -1 and 1001 and all consumers agree on value or refusal and source locus. | Test (TC-209, IT-075) |
| FR-180-AC-5 | Corpus identity fixes every input, semantic profile and expected disposition so a replay cannot consult ambient state. | Test (TC-209) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
