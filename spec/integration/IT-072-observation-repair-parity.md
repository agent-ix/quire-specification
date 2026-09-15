---
id: IT-072
title: "Observation revisions repair temporal and protocol results"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-160
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-173
    type: verifies
---

# IT-072: Observation revisions repair temporal and protocol results

## Objective

Verify that real QObs authority views drive QSL/TL temporal and Protocol repair
with identical partial/uncertain input meaning and retained supersession.

## Target Integration

Pinned QObs Rust owner-view APIs feed the real temporal and protocol Rust
consumers; results re-enter the composed runtime bundle.

## Preconditions

Clock, population, progress and revision contracts use the exact selected wires.

## Inputs

Partial valuations, overlapping clock intervals, late/conflicting observations
and explicit superseding revisions.

## Test Procedure

1. Assess the initial partial/uncertain inputs.
   - IT-072-SC-01: consumers retain matching possible-value and completeness axes.
2. Submit a valid superseding revision.
   - IT-072-SC-02: only the affected suffix/window is recomputed.
3. Compare incremental and full replay results.
   - IT-072-SC-03: settled results and lineage agree exactly.

## Expected Results

No consumer uses ingestion order, midpoint time, quiet time or missing-as-false;
old and replacement results remain linked.
