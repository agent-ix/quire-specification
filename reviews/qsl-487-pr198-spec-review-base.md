---
id: SR-1313
title: "Spec review of PR #198 (QSL-487): restore the redefined-operation and text-profile vectors"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@7d119b77d18271b4ad6a9e7bb59d60453cfe8f49; PR #198 diff against origin/main (merge base a73731e, checked as a local merge with main ada3f9eb): FR-322-AC-30, FR-322-AC-44, FR-342-AC-4, TC-280 status note and rows MT-08 to MT-10, MT-15, MA-14, MA-15, AN-02, model-member-type-vectors.json, domain-package-acme-orders.json, the re-keyed fixtures; FCD fixtures/semantic/v1/negative/cases.json as context"
review_set: subset
---
# Spec review of PR #198 (QSL-487)

## Summary

Ticket: QSL-487 (B6 companion). The PR restores the redefined-operation
cases (MT-08, MT-09, MT-10, AN-02) and the text-profile cases (MT-15, MA-14,
MA-15) against the real `acme/orders` document. It removes the "not
qualified until AGE-2226" text from FR-322-AC-30, FR-322-AC-44 and
FR-342-AC-4, and re-keys the fixtures that select the document. MA-16, a
negative Text `min`, is deliberately not restored.

Examined:
- FR-322-AC-30, FR-322-AC-44, FR-342-AC-4 after the qualifier removal (examined)
- TC-280 rows MT-08, MT-09, MT-10, MT-15, MA-14, MA-15, AN-02 and the removed status note (examined)
- model-member-type-vectors.json `domain_package` additions and the restored cases (examined)
- domain-package-acme-orders.json: Code, Inverted, Folded value types; Order sku/alias/slug; Sub and SubSub redefining operations (examined)
- positive-all-families.json, positive-operation-identities.json, dependency-selection-vectors.json re-keys (examined)
- FCD `constraint-negative-min-length` negative case and test/semantic-ir-v1-1.test.ts:601 (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean.

The re-keying changed only identity digests. The three fixtures differ from
main only in the `acme/orders` `model_selections` digest and `package_id`,
plus the `package_id` in dependency-selection-vectors.json. No expected
outcome was hand-edited.

The restored vectors are behavioural:
- `tc_280_the_domain_package_projection_is_derived_from_the_selected_document`
  asserts that the vectors' `domain_package` equals the projection of the
  real document, so the new value types and `redefines` entries cannot drift
  from it.
- Each restored case is derived through the test's FR-322 resolution:
  - MT-08 and MT-09 resolve `Sub`'s redefinitions.
  - MT-10 resolves the most derived `SubSub/size`.
  - MT-15 types `Text[0, 64; nfc]`.
  - MA-14 refuses `[5, 2]`, where min is greater than max.
  - MA-15 refuses the `casefold` profile, which is outside FR-141's six.
  - AN-02 anchors `(Sub, "size")`.
- The AC texts now state what these cases check, without the qualifier.

The MA-16 deviation is sound. FR-322-AC-44 says a negative `min` "is not a
document". That refusal belongs to filament-core-data, which covers it with
the negative case `constraint-negative-min-length`
(fixtures/semantic/v1/negative/cases.json, asserted in
test/semantic-ir-v1-1.test.ts:601). QSpec cannot reach that case with a
valid document, so no QSpec adverse vector is needed.

Base: the branch is off a73731e, and main is now ada3f9eb. The two diffs
touch disjoint files, and a local merge with main is clean. The merged tree
was tested; see SR-1314.
