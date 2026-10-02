---
id: SR-1196
title: "Spec review of quire-specification PR #179 (QSL-290): Text[min, max; profile] model-owned member type"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@3ec9a087b5aae49dc34bae7ff8114272a6a8bade; PR #179 diff against main 5e85e890; FR-322 step 4, FR-322-AC-44, TC-280, model-member-type-vectors.json, tests/model_member_types.rs; checked against quire-spec-language FR-056-AC-11 on spec/381-depth"
review_set: subset
---
# Spec review of quire-specification PR #179

## Summary

Ticket: QSL-290. The PR adds a FR-322 step 4 row typing a field whose value type is bound as `Text[min, max; profile]`, FR-322-AC-44, TC-280 vectors MT-15, MA-14 and MA-15, and the Rust derivation of the row.

Examined:
- FR-322 step 4 Text row (examined)
- FR-322-AC-44 (examined)
- TC-280 MT-15, MA-14, MA-15 (examined)
- tests/model_member_types.rs `element_type` (examined)
- FR-141 profiles (context_only)
- QSL FR-056-AC-11 on spec/381-depth (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The step 4 row admits a Text value type only when `0 <= min <= max`, and the Rust derivation checks `lower >= 0`. But AC-44 names only `min > max` and a foreign profile as refusals, and no vector has a negative `min`. Add "or `min < 0`" to AC-44 and a vector (for example `[-1, 4; nfc]`) refusing `ill_typed`/`operator-ineligible`, so the row's lower-bound rule is tested. | spec/objects/interfaces/FR-322-checked-package-artifact.md:760 |

## Verdict

The row is right. `text_bounds` with `min`, `max` and `text_profile` bindings is the form schema.json's `DomainNode` and the positive-operation-identities fixture already use. The six profiles are FR-141's, and the Rust derivation implements the row exactly. The refusals are unchanged: native `Text` stays MA-06 under AC-31, and inverted bounds and `casefold` refuse `ill_typed`/`operator-ineligible`. Against QSL #577's FR-056-AC-11 (spec/381-depth at adf724a5), the six profiles agree. The bounds differ slightly: QSpec requires `0 <= min <= max`, while QSL FR-056 says only `min <= max`, so QSL #577 should add `0 <=`. `cargo test --locked` passes (61 + 12 + 7 + 3 tests, 0 failed). `quire validate` passes TC-280. On FR-322 it reports two structural errors (a `value_object` `id` field and a `fields` record), but main 5e85e890 reports the same two, so #179 did not cause them. One low finding: AC-44 does not test the `min >= 0` rule. **Mergeable once FND-001 is fixed.**
