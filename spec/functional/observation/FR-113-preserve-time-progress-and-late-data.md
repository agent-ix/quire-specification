---
id: FR-113
title: "Preserve time, progress, and late-data authority"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-009
    type: implements
  - target: ix://agent-ix/quire-specification/FR-266
    type: references
  - target: ix://agent-ix/quire-specification/FR-267
    type: references
  - target: ix://agent-ix/quire-specification/FR-268
    type: references
  - target: ix://agent-ix/quire-specification/FR-289
    type: references
---
## Description

When assessing a timed obligation, the observation service SHALL retain event
time, ingestion time, selected progress/assertion definition identity, revision,
and byte digest, declared clock interpretation, progress authority, captured
trigger identity, late-data cutoff, and any restoration basis before producing a
settled disposition.

## Behavior

The observation service SHALL NOT infer causality from arbitrary timestamp
order. When silence is used to settle a deadline, the observation service SHALL
do so only when declared progress/closure authority covers that deadline. The
observation service SHALL classify a late event against the selected cutoff rather
than silently rewriting an already settled result. When an assertion definition,
scope, clock selection, source set, or restoration basis does not match the
assessment, the observation service SHALL retain an incomplete or refused input
rather than settle or restart from it.
For a selected half-open window, the observation service SHALL apply
progress/closure only to admitted members with instants `start <= instant < end`.
When that window supplies an inclusive temporal assessment `[a,b]`, the
observation adapter SHALL apply FR-289: retain an explicit same-clock carrier `[a,e)` with `e > b`,
filter participating observations to `a <= instant <= b`, and require progress
covering every required source and subject through `b`. A half-open end or
watermark equal to `b` does not prove inclusive coverage through `b`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-113-AC-1 | A quiet eligible deadline settles only with declared progress authority. | Test (TC-143) |
| FR-113-AC-2 | Event time, ingestion time, causal relation, and clock uncertainty remain distinguishable. | Test (TC-143) |
| FR-113-AC-3 | A late event is retained as late and cannot silently replace the selected settled result. | Test (TC-143) |
| FR-113-AC-4 | A mismatched assertion definition, scope, clock, source set, or restoration basis remains incomplete/refused and cannot settle or restart an assessment. | Test (TC-143) |
| FR-113-AC-5 | Progress/closure includes the selected window start, excludes its end, and cannot settle or contradict a later window through that boundary. | Test (TC-143) |
| FR-113-AC-6 | The FR-289 mapping includes observations at both native inclusive endpoints, filters carrier members above the upper endpoint, uses checked successor only for discrete clocks, and refuses a foreign clock, missing greater end or overflow. | Test (TC-143) |

## Dependencies

- [Shared foundation](../../../proposals/quire-v1/shared-foundation.md).
- [Observation binding and progress contract](../../../proposals/quire-v1/observation-contract.md).
- E owns clock, history, and closure meaning.

## Status

Accepted by FS04 for time/progress authority, exact range correspondence and
late-data preservation. Runtime adoption remains separately tracked.
