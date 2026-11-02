---
id: TC-976
title: "Authenticated schedulability claim carrier and reader boundary"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-418
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-482
    type: verifies
---

# TC-976: Authenticated schedulability claim carrier and reader boundary

## Description

Verify that the checked package carries an authenticated typed handle to each
`claim`/`schedulability` occurrence, that the handle joins only to the exact
FR-482 claim node and occurrence, and that the IR typed read is distinct from
the QSL-owned FR-418 successor checker.

## Test Procedure

Read `proposals/checked-package-v2/schedulability-claim-vectors.json` and
`proposals/checked-package-v2/schema.json` by reference. Validate each positive
and schema-refusal row against `$defs.SchedulabilityClaimRef`. For each positive
case, compile the named TC-967 source case and join its `declaration` to the
local `claim`/`schedulability` node. Require one row for each exact `claim`
occurrence; SC-01 has occurrence 0 and the shared-node SC-06 has ordinals 0 and
1. For each `reader_refusals` case, build the schema-valid package named by its
`base_case`, combining the named TC-960 and TC-967 source cases when specified.
Apply the vector's `mutation` to that package's `schedulability_claims` rows;
for SCC-07, replace the row's declaration with the exact `claim` NodeRef from
TC-960/SN-01, whose node has `semantic_form: "state_graph"`. Then run the
FR-322 carrier join before the FR-482 body read. In SCC-10, keep the SC-06
node's two declared occurrences and the one schema-valid row for occurrence 0;
the missing occurrence 1 must refuse coverage at the top-level member. The QSL checker test resolves the same
claim identity through FR-418, then independently recomputes the source and
scheduler successors from the selected timed model. The IR read test returns
the claim body and occurrence without a model-release or successor result.

## Expected Results

Positive rows and packages pass the closed wire schema, and only an exact
local schedulability declaration and owned claim occurrence are admitted. The
schema cases refuse `malformed_wire`; the schema-valid reader cases refuse
`invalid_package`/`invalid-value` or `invalid_package`/`conflicting-definition`
at the exact locus in the vector file. A missing row refuses
`invalid_package`/`invalid-value` at `/schedulability_claims`, and non-canonical
order refuses `invalid_package`/`invalid-value` at the later row. IR returns
the authenticated FR-482 body by reference and does not evaluate source
releases, scheduler successors or task templates. QSL owns those joins and
recomputations. No `TaskTransitionTemplate` or `TaskProductCertificate` field
is added to checked-package V2.
