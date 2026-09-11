---
id: TC-113
title: "Distinguish event positions, samples and event time"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-090
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-095
    type: verifies
---
## Description

Planned clock-domain and fixed-sample bridge controls.

## Test Procedure

Use one event sequence with two fixed-sample bindings of different periods and
one irregular timestamp binding. Hold the formula and predicate values fixed.
Change epoch, period, unit and sequence authority one axis at a time; omit one
required sample; reorder equal-timestamp events using ingestion order only; and
attempt to reinterpret event counts as elapsed time. Separately omit or corrupt
the fixed-sample clock definition/binding. For the valid sample binding, compare
native and TL truth while preserving the clock correspondence outside the TL
formula.

## Expected Results

Each clock selection retains a distinct subject/result. The valid sample bridge
agrees on truth and retains epoch/period/unit. An absent runtime sample under a
valid binding reports incomplete; a malformed/missing clock binding, changed
period, implicit conversion or unsupported equal-time ordering refuses. Equal
TL formula bytes alone never erase a clock mismatch.
