---
id: SR-1298
title: "Spec review of PR #189 fix round: FR-322 derived-shape key stage"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@4bf7e9c834a6e29695621be5233aec8c19359d67; spec/objects/interfaces/FR-322-checked-package-artifact.md (derived-shape key stage, FR-322-AC-48..50), spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/complete-v1/tests.md"
review_set: subset
---
# Spec review of PR #189 fix round (QSL-635)

## Summary

Ticket: QSL-635. Spec text first changed in fix commit 9630eb4: FR-322
gains a normative derived-shape key stage, ACs 48-50, TC-233 steps and
results, and three Test Matrix rows. The stage's place in the reader order,
its two passes, its refusal locus and its in-group preimage match QSL FR-092
and the test reader (`stale_derived_shape_node`). AC-48, AC-49 and AC-50 are
each testable and name concrete mutations, and the three new tests trace them.
The matrix rows are present. `quire validate` adds no new failure class: the
145 failed documents on main are the pre-existing TestMatrix column
mismatches.

## Verdict

**Request changes.** One high finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The stage re-keys in-group structural nodes "declared or not and of any shape", but outside a group it re-keys only the undeclared derived shapes. An ungrouped declared structural node is never re-derived. QSL emits declared records (FR-092 D1), so all-families `Example::Point` qualifies. Re-pointing Point's `x` from `Integer` to the `Int[0, 9]` node, keeping its node id and recomputing projection and `package_id`, is admitted, and a consumer reads a narrowed field type: the IR-627 class one hop away. The test reader mirrors the spec (`is_derived_shape` requires no `declaration`), and no other stage keys a declared non-nominal node. Fix: re-key every ungrouped non-nominal, non-application structural node, owner from its declaration source occurrence, as the in-group set already does. The positive-fixture key test already shows every such fixture node is current. Add a declared-record tamper row. | spec/objects/interfaces/FR-322-checked-package-artifact.md:245-280, tests/checked_package_v2.rs:2061-2084, tests/checked_package_v2.rs:2216, proposals/checked-package-v2/fixtures/positive-all-families.json:16 |
