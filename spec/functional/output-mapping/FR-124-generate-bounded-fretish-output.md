---
id: FR-124
title: "Generate bounded FRETish 3.1 output"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-121
    type: references
  - target: ix://agent-ix/quire-specification/FR-090
    type: references
  - target: ix://agent-ix/quire-specification/FR-094
    type: references
---
## Description

When `quire.output.fretish31/v1` is selected, the FRETish
mapper SHALL generate only the admitted Classic scope/condition/component/
timing/response fragment with explicit clock/completeness conditions.

## Inputs

An admitted common request; total mapped Boolean/scalar signals; an exact
component/variable catalog; and, for temporal output, exact fixed-sample clock,
trigger/capture and decision-window facts.

## Outputs

Deterministic Classic FRETish text regions and complete FR-298 records.

## Behavior

The mapper SHALL use only the timing correspondences selected in the FS06
FRETish profile. Every temporal record retains clock unit, sample period,
trigger/capture, history and completeness conditions not represented in text.

The mapper SHALL NOT treat event positions as elapsed time, timestamps as fixed
samples, open/incomplete windows as complete, FRET finite-trace semantics as
native MLTL correspondence, or timing-free requirements as implicitly temporal.
Unsupported intervals/operators and missing component/signal bindings receive
no substitute sentence.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-124-AC-1 | Selected state, immediate, `[0,b]` fixed-sample eventual and exact `[1,1]` next-timepoint cases generate the stated Classic FRETish fields. | Test (TC-153) |
| FR-124-AC-2 | Every temporal record retains exact clock/unit/period/trigger/capture/completeness conditions, and changing one breaks correspondence. | Test (TC-153) |
| FR-124-AC-3 | Nonselected bounds, past/history, timestamped/open/incomplete/protocol/probabilistic cases are explicit non-preserved mappings with no substitute sentence. | Test (TC-153) |
| FR-124-AC-4 | Timing-free input receives no invented timing, and event count never becomes elapsed time. | Test (TC-153) |
| FR-124-AC-5 | FRET parse/formalization output cannot become native source, truth or FR-095 TL correspondence. | Test (TC-155) |

## Dependencies

- [FRETish profile](../../../proposals/quire-v1/output-mappings/fretish-3.1.md).
- [FR-121](FR-121-account-for-every-output-obligation.md).
- FS04 temporal/observation semantics; qcir #57 owns implementation.

## Status

Accepted by FS06; executable exporter and correspondence evidence remain qcir #57.
