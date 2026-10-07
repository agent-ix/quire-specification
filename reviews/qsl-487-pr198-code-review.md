---
id: SR-1314
title: "Code review of PR #198 (QSL-487): tests/model_member_types.rs text value types and redefines"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@7d119b77d18271b4ad6a9e7bb59d60453cfe8f49; PR #198 diff against origin/main; tests/model_member_types.rs (TEXT_PROFILES, element_type Text arm, project_document redefines and string value types, tc_280_text_value_type_members_carry_their_bounds_and_profile); tests run on a local merge with origin/main ada3f9eb"
review_set: subset
---
# Code review of PR #198 (QSL-487)

## Summary

Ticket: QSL-487. The test changes do four things:
- `element_type` gains a Text arm, which admits `Text[min, max; profile]`
  only when min ≤ max and the profile is one of FR-141's six.
- `project_document` reads operation `redefines` from object-type
  `extensions[].payload.redefines`.
- `project_document` projects `string` value types through their
  `minLength`/`maxLength` and `format` (`text-profile:`) constraints.
- A new test runs the FR-322-AC-44 member cases.

Examined:
- element_type Text arm and TEXT_PROFILES (examined)
- project_document redefines map and string value-type projection (examined)
- tc_280_text_value_type_members_carry_their_bounds_and_profile (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The projection is read from the real document, and
`tc_280_the_domain_package_projection_is_derived_from_the_selected_document`
compares it with the vectors. The new code therefore checks the document's
content, not hand-written values. An unexpected scalar or a missing bound
panics instead of being skipped. The trace tags match the TC-280 rows.

Focused tests ran in one build through locked-build.sh, on a local merge of
the branch with origin/main ada3f9eb: model_member_types, checked_package_v2,
backend_provider_v1, checked_package_model and shared_reference. All passed: 17, 88, 9, 3 and 7 tests respectively.
