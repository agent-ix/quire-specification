---
id: SR-13812
title: "Gap analysis of PR #325 changed and deleted criteria against test-case rows"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@3a52c6820e06ce83c3bf888f950292f2c7205332; FR-150-AC-3, FR-154-AC-2, FR-321-AC-2, FR-322-AC-32, deleted FR-322-AC-43; TC-195 N05, TC-234 I02, TC-280 MS-01..MS-06, IT-010-SC-08, model-member-type-vectors.json selection_cases, tests/model_member_types.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-154
    type: reviews
---
# Gap analysis of PR #325

## Summary

Ticket: STD-126. Plan completion: not assessed. The PR deletes FR-322-AC-43
together with its TC-280 MS-03 row, vector MS-03 and the
`tc_280_an_altered_document_under_the_selected_digest_is_refused` test. No
reference to any of them remains outside historical reviews/. It also changes
FR-154-AC-2 and FR-322-AC-32 to say that a document supplied under the
selected digest is admitted whatever its own canonical digest is. The PR
deleted the altered-document rows that would now test that, and added no row
that does: TC-234 I02's "K1 with one field value changed" case and vector
MS-03 are removed, not turned into admission cases. The tests pass.
`tc_280_selected_domain_package_documents_are_admitted_by_digest_key_and_identity`
is still bound to FR-322-AC-32 and checks the refusal cases and the MS-01
admission.

## Verdict

Not mergeable as written. Two medium coverage gaps. The FR-150-AC-3 gap is
reported in SR-13810 FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-154-AC-2's new clause "a supplied document is admitted whatever its own canonical digest is" has no TC-234 row. I02's altered-K1 case was deleted instead of being changed to expect admission | spec/functional/type-model/FR-154-admit-domain-package-model.md:261; spec/test-cases/TC-234-domain-package-model-intake.md:44 |
| FND-002 | medium | FR-322-AC-32's new clause "whatever its own canonical digest is" has no vector or TC-280 row. MS-03 was deleted instead of being changed to expect `admission: passed`, so no selection case supplies a document whose digest differs from its key | spec/objects/interfaces/FR-322-checked-package-artifact.md:1780; spec/test-cases/TC-280-model-owned-member-types.md:188-191; proposals/checked-package-v2/model-member-type-vectors.json:486-491 |

## Dispositions

Disposition pass 1, against PR head f14d1d8b11d8b8f40694c248777fbde513773202
(fix commit f14d1d8). `cargo test --test model_member_types` passes (23
tests); `tc_280_selected_domain_package_documents_are_admitted_by_digest_key_and_identity`
(bound to FR-322-AC-32) now runs the restored MS-03 case through `select`,
which applies its patch and returns `admission: passed`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | TC-234 I02 now includes "K1 with one field value changed, supplied under S1's digest" expecting "the changed K1 admitted with its changed field value (FR-154-AC-2)", and the closing clause reads "no refused run charges `normalize.record` or builds a declaration". |
| FND-002 | fixed | Vector MS-03 is restored with patch `/types/5/constraints/1/operands/value` = 8 and `"expected": {"admission": "passed"}`; TC-280 row MS-03 reads "Passes the selection admission checks: the document's own canonical digest is not compared with the selected digest", bound to FR-322-AC-32. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | TC-280 MS-03's input says "`OrderTotal`'s upper bound changed to 8", but vector MS-03 patches `/types/5/constraints/1/operands/value`, which in domain-package-acme-orders.json is `ix://acme/orders/Inverted`'s `maxLength` (OrderTotal is `/types/8`, with `min`/`max`). The admission outcome is unaffected. The mislabel is carried over verbatim from main's deleted row; this PR re-adds it | spec/test-cases/TC-280-model-owned-member-types.md:190; proposals/checked-package-v2/model-member-type-vectors.json:489; proposals/checked-package-v2/domain-package-acme-orders.json |
