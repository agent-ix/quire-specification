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

Verify that the `temporal`/`monitor_plan` root is admitted only with its seventeen
ordered bindings, the four state-passing entry signatures, the six-case
`Outcome` union and valid premises and buffers, and that each defect
refuses with FR-487's code and locus.

## Test Procedure

Read `proposals/checked-package-v2/monitor-plan-vectors.json` and
`proposals/checked-package-v2/schema.json` by reference. Apply each case's `set`
edits to `base` and decide the plan in FR-487's reader order (shape, reference
kinds, signatures, premise ranges, buffers and state). MP-01, MP-02, MP-33, MP-43, MP-44, MP-45, MP-53, MP-58, MP-81 and MP-82 are
admitted. Every refusal case holds one defect and names its code, cause and
locus. The vectors check the reader order over an abstract description of the plan; the reader itself is quire-contract-ir's (IR-721). The `gap_cases` (MP-54 to MP-57, MP-66 to MP-73) apply FR-487's counter-gap path to abstract live instances and a reading sequence: open instances fail with `ReadingGapExceeded` and leave the state, decided ones stay, the faulting reading is the new baseline and a clock poll does not move it, buffers are dropped with a history-start marker after a gap, and each instance is reported once on settlement with pending ones at watermark steps (MP-70 to MP-72), and event_position counts admitted events only (MP-77). The `limit_cases` (MP-78 to MP-80) fix the closed `LimitReached` kinds: an atom with no admitted projection reports Incomplete(LimitReached{atom_projection}) without changing the Outcome shape. MP-32 names `quire.op.collection.flatten` and
`quire.op.collection.sequence`; both must be in the closed operation catalog, and
the `Operation` identity enum and the `temporal` `semantic_form` enum must be
the published ones (`monitor_plan` present, no new operation identity).

## Expected Results

MP-01, MP-02, MP-33, MP-43, MP-44, MP-45, MP-53, MP-58, MP-81 and MP-82 admitted; every admitted case but
MP-01 yields a root key different from MP-01's (the counter name, the tick unit,
the rate, the uncertainty and a capacity are identity-bearing). MP-03 to MP-09,
MP-23 to MP-28, MP-30, MP-31, MP-35, MP-37, MP-39 to MP-42, MP-48, MP-49, MP-60 to MP-65 and MP-74 to MP-76 refuse
`invalid_semantic_graph`; MP-10 to MP-17, MP-29 and MP-50 to MP-52 refuse
`ill_typed`/`type-mismatch`; MP-18 to MP-22, MP-34, MP-36, MP-38, MP-46, MP-47 and MP-59 refuse
`invalid_package`/`invalid-value`. MP-32 is admitted with no catalog change.

Call closure (FR-487-AC-13): two plans that differ only in the body of a declared helper called by an entry, an atom body or the guard, or called only through another helper, have different root keys while the helper's and the entries' own `node_id`s stay equal; equal helper bodies give an equal root key. The vectors for this check are authored with the producer (QSL-1463) and the strict reader (IR-721); the key is the producer's duty and the reader recomputes none of it.
