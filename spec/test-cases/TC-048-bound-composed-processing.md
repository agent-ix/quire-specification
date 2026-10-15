---
id: TC-048
title: "Stop composed work at its actual resource boundary"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/NFR-010
    type: verifies
---
## Description

Accepted portable resource controls for the state producer/query/graph
boundaries. Accounting/version contracts and
runnable harnesses remain implementation work.

## Test Procedure

For each implemented boundary, use an input with known declared accounting and
run it with zero, sufficient, exact-sufficient and one-less limits. Include
valid per-wrapper nested sequences with excessive aggregate work, acyclic
predicate fan-out, rational normalization and long/cyclic graphs. Observe charged
allocation/expansion and the affected request's result; retain an independent
request to verify that its existing result is not overwritten.

## Expected Results

Sufficient runs complete only their actual checked work. Insufficient limits
produce typed incomplete outcomes before uncharged expansion or publication of
a completed affected result. Zero never means unlimited, and required inputs
are never silently discarded. Independently completed requests retain their
scope. Missing harness/boundary coverage remains explicitly planned.
