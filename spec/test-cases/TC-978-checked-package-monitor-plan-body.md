---
id: TC-978
title: "Tick-monitor plan root node"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-487
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-978: Tick-monitor plan root node

## Description

Verify that the `temporal`/`monitor_plan` root is admitted only with its twelve
ordered bindings, the three state-passing entry signatures, the six-case
`Outcome` union and valid integer premises and buffers, and that each defect
refuses with FR-487's code and locus.

## Test Procedure

Read `proposals/checked-package-v2/monitor-plan-vectors.json` and
`proposals/checked-package-v2/schema.json` by reference. Apply each case's `set`
edits to `base` and decide the plan in FR-487's reader order (shape, reference
kinds, signatures, premise ranges, buffers and state). MP-01 and MP-02 are
admitted. Every refusal case holds one defect and names its code, cause and
locus. MP-28 names `quire.op.collection.flatten` and
`quire.op.collection.sequence`; both must be in the closed operation catalog, and
the `Operation` identity enum and the `temporal` `semantic_form` enum must be
the published ones (`monitor_plan` present, no new operation identity).

## Expected Results

MP-01 and MP-02 admitted. MP-03 to MP-09 and MP-23 to MP-27 refuse
`invalid_semantic_graph`; MP-10 to MP-17 refuse `ill_typed`/`type-mismatch`;
MP-18 to MP-22 refuse `invalid_package`/`invalid-value`. MP-28 is admitted with
no catalog change.
