---
id: TC-111
title: "Evaluate bounded future truth tables"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-091
    type: verifies
---
## Description

Planned independent truth-table coverage for bounded future semantics.

## Test Procedure

Enumerate Boolean formulas using `eventually`, `always`, `until` and `release`
for every interval through `[3,3]`, two propositions, traces through length five
and anchors through two. Compute the false-extension oracle directly from the
definitions in [FR-091](../functional/temporal/FR-091-evaluate-bounded-future.md), not by
calling the candidate evaluator. Include zero-width windows, short/empty traces,
nesting and `p until[1,2] q` with false `p` at the anchor and true `q` at offset
one. Mutate inclusive endpoints and the `until` lower-bound range independently.
For a one-position closed trace with `p` true, independently evaluate
`always[0,1] not holds(p)` and `eventually[0,1] not holds(p)`; mutate the
candidate to an untimed Boolean-connective reading.

## Expected Results

Every candidate outcome equals the independent oracle. Endpoint or lower-bound
mutations change at least one named vector. `release` remains the exact Boolean
dual of `until`; no separately implemented boundary rule can drift.
The pointwise-connective vectors distinguish the selected results from the
untimed mutation.
