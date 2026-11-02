---
id: TC-977
title: "Operation-effect carrier and strict reader boundary"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-340
    type: verifies
---

# TC-977: Operation-effect carrier and strict reader boundary

## Description

Verify that the checked package carries each model operation's declared effect
(modifies, creates, deletes, scoped deletes) as an `operation_effects` row keyed
by selected document, declaring node and original operation name, with
provenance; that no effect kind is dropped; and that the reader refuses a
malformed, mis-keyed, mis-attributed or incomplete carrier with exact codes and
loci.

## Test Procedure

Read `proposals/checked-package-v2/operation-effect-vectors.json` and
`proposals/checked-package-v2/schema.json` by reference. Validate every row of
each positive case against `$defs.OperationEffectRow`, then lift each case's
`documents` operation frames (FR-013-AC-13, FR-340 canonical order) and require
the rows to be exactly that lift with one row per operation, keyed and ordered
by `(document, declaring, operation)`. OEC-01 to OEC-03 carry one of modifies,
creates and deletes each, OEC-04 carries only scoped deletes, OEC-05 is the
all-empty effect, OEC-06 two members of one object and OEC-07 the same member
name `settle` under two objects and two documents with distinct effects.

For each `schema_refusals` and `reader_refusals` case, apply its RFC 6902 `patch`
to `{"operation_effects": <rows of base>}`. A schema case fails
`$defs.OperationEffectRow` as `malformed_wire` at exactly the recorded locus. A
reader case passes the schema and the reader, run against the base case's
`documents` as the selected documents, refuses with the recorded code, cause
and locus (`expected_cause` null means no cause, as FR-340's
`invalid_semantic_graph`), and no other. Each case holds one defect.

## Expected Results

All positive cases read back. The 14 schema refusals (OES-01 to OES-14) are
`malformed_wire`. The 26 reader refusals (OER-01 to OER-26) are: OER-01
`missing_declaration`/`missing-selection`; OER-02, OER-10, OER-12 and OER-24
`missing_declaration`/`missing-name`; OER-03, OER-05, OER-08, OER-09, OER-14 to
OER-19 and OER-22 `invalid_package`/`invalid-value` (OER-22 at
`/operation_effects`); OER-20 `invalid_package`/`conflicting-definition`; OER-21
`invalid_package`/`invalid-value`; OER-04, OER-06, OER-07, OER-25 and OER-26
`ill_typed`/`operator-ineligible`; OER-11 `invalid_model_binding`/
`malformed-declaration`; OER-13 and OER-23 `invalid_semantic_graph`. The IR
reader exposes the admitted effect by key and owns no mirror type; QSL owns
emission.
