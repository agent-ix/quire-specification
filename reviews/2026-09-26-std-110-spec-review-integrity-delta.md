---
id: SR-719
title: "Spec review (integrity, delta) — STD-110 catalog 1-draft.8 fix round"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #162 delta 05f6e311..6389506c"
review_set: subset
---
# Spec review (integrity, delta) — STD-110 catalog 1-draft.8 fix round

Transcribed verbatim from https://linear.app/agent-ix/issue/STD-110/catalog-1-draft8-codes-for-the-kernel-refusal-causes-qsl-cannot-record#comment-cbe12673

## Summary

Delta review of the fix round (`05f6e311..6389506c`): only findings the fix introduced or left exposed. The SR-719 file does not exist yet. Following the reviewer skill's step 6, the coder adds `reviews/2026-09-26-std-110-spec-review-integrity-delta.md` with id SR-719 in the next fix commit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-044 (and value-accounting.md:213-217) now make an out-of-domain bounded integer/Rational arithmetic result at evaluation a `refused integer_out_of_domain`/`rational_out_of_domain`, which the catalog says is never undefined. FR-145:94-97 and TC-190 Q12 already classify the same uncharged FR-044 decision on a `sum` prefix under direct kernel evaluation as a located undefined outcome. `sum<Int[0,3]>` over `2, 2` now has two conflicting outcomes. Fix: either align FR-145/TC-190 Q12 (and any other direct-kernel non-member outcome) to the new refusal, or scope the FR-044 rule so `sum` prefixes stay undefined and say why. Choose one outcome. | spec/functional/foundation/FR-044-check-composed-numeric-definedness.md:40-48 |
| FND-002 | low | Three normative rules added by 6389506c have no acceptance criterion and no test: FR-044's evaluation-time refusal (FR-044-AC-1..9 are all checking-time, TC-044/TC-192/TC-193), the blank-label-before-empty-path precedence, and the payload value spellings. Each can change without failing anything. | spec/functional/foundation/FR-044-check-composed-numeric-definedness.md:40-48; proposals/quire-v1/definitions/native-diagnostics.md:102,226-239 |
| FND-003 | low | diagnostic_row_payload_keys drops every backticked name that is not already canonical before comparing, so a row that gains an extra non-canonical payload key passes tc_233_revision_1_draft_8_cause_sets_and_payload_keys_are_closed. FR-272-AC-11 says each row names exactly its keys. Renames and drops are caught (reviewer-verified). Additions are not. | tests/checked_package_v2.rs:4924-4944 |

## Reviewer comment (verbatim)

<!-- reviewer repo=agent-ix/quire-specification visibility=private quoin=0.24.1 module=spec-artifacts-process@v0.26.0 id=SR-719 method=spec-review/integrity lang=spec pr=quire-specification#162 date=2026-09-26 -->

Delta review of the fix round (`05f6e311..6389506c`): only findings the fix introduced or left exposed. The SR-719 file does not exist yet. Following the reviewer skill's step 6, the coder adds `reviews/2026-09-26-std-110-spec-review-integrity-delta.md` with id SR-719 in the next fix commit.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-001 | medium | soundness | FR-044 now says a bounded integer or `Rational[..]` result outside its domain at evaluation is `refused` `integer_out_of_domain`/`rational_out_of_domain`. The catalog adds "None is undefined". FR-145 and TC-190 Q12 already rule that this same uncharged FR-044 decision, on a `sum` prefix under direct kernel evaluation, is a **located undefined outcome**. One input now has two outcomes. |
| FND-002 | low | untestable-ac | The fix added three normative rules with no acceptance criterion or test: FR-044's evaluation-time refusal (every FR-044 AC is static, TC-044), the `blank-label`-before-`empty-path` precedence, and the payload value spellings. |
| FND-003 | low | test-intent | `diagnostic_row_payload_keys` keeps only backticked names that are already canonical. A row that gains a non-canonical key (for example `bits`) still passes, so the test proves renames and drops fail but not additions. |

Not findings (verified): every changed lock digest equals raw `sha256sum` of the current bytes: value-accounting.md `6fdab8c8…`, FR-148 `69780c29…` (in both complete-value-lock.json and value-complete-rules.json), value-complete-rules.json `3787474a…`, complete-value-lock.json `e7cf850f…`. No old digest (`1d8b15f8`, `8b8500fa`, `012e66aa`, `96e20d5a`, or the old model-lock `565d67ab`) appears anywhere in the tree. No lock pins any other file this PR changed. Revisions were rightly left alone: the definitions README says an edited draft needs no successor revision. `tc_195` recomputes the model lock's pin of complete-value-lock.json and passes. Nothing in tests recomputes complete-value-lock.json's own entries, so those digests are checked only by this review. The FR-272 list, the catalog paragraph and `REVISION_1_DRAFT_8_ROWS` agree. No file other than historical review records names `1-draft.7` as current. Full `cargo test` passes at 6389506c (4 targets, 63 tests). The reviewer ran 3 mutations in a scratch worktree (rename `flags`->`flag_set`, drop `label` from the canonical list, delete FR-271's `integer_out_of_domain` row) and each failed its test. The `quire validate` failures on FR-270/271/272 (missing `id`, `fields`) also occur on `origin/main` and predate this PR.

+++ [reviewer data]

```yaml
scope:
  - {id: FR-044, path: "spec/functional/foundation/FR-044-check-composed-numeric-definedness.md"}
  - {id: FR-145, path: "spec/functional/expressions/FR-145-convert-and-query-collections.md"}
  - {id: FR-146, path: "spec/functional/expressions/FR-146-check-total-pure-functions.md"}
  - {id: FR-148, path: "spec/functional/expressions/FR-148-evaluate-ieee-floating-profiles.md"}
  - {id: FR-270, path: "spec/objects/foundation/FR-270-versioned-diagnostic-catalog.md"}
  - {id: FR-271, path: "spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md"}
  - {id: FR-272, path: "spec/objects/foundation/FR-272-typed-diagnostic-cause.md"}
  - {id: TC-190, path: "spec/test-cases/TC-190-collection-query-and-conversion-algebra.md"}
  - {id: TC-191, path: "spec/test-cases/TC-191-total-pure-function-checking.md"}
  - {id: TC-193, path: "spec/test-cases/TC-193-ieee-exceptional-and-rounding-profiles.md"}
  - {id: TC-233, path: "spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md"}
  - {id: native-diagnostics, path: "proposals/quire-v1/definitions/native-diagnostics.md"}
  - {id: value-accounting, path: "proposals/quire-v1/definitions/value-accounting.md"}
  - {id: complete-value-lock, path: "proposals/quire-v1/definitions/complete-value-lock.json"}
  - {id: value-complete-rules, path: "proposals/quire-v1/definitions/value-complete-rules.json"}
  - {id: complete-model-lock, path: "proposals/quire-v1/definitions/complete-model-lock.json"}
  - {id: definitions-readme, path: "proposals/quire-v1/definitions/README.md"}
  - {id: complete-v1-matrix, path: "spec/complete-v1/tests.md"}
  - {id: tests/checked_package_v2.rs, path: "tests/checked_package_v2.rs"}
findings:
  - fnd: FND-001
    severity: medium
    method: spec-review/integrity
    check: soundness
    artifact_id: FR-044
    path: spec/functional/foundation/FR-044-check-composed-numeric-definedness.md
    lines: "40-48"
    excerpt: |-
      At evaluation, the evaluator decides a bounded integer or `Rational[..]`
      arithmetic result's membership in its result domain after normalization and
      before retaining it (`quire.value.accounting/v1`). A result outside that domain
      is refused with no value, as
      `refused { code: integer_out_of_domain, cause: outside-domain }` or
      `refused { code: rational_out_of_domain, cause: outside-domain }`,
      respectively.
    conflicts_with:
      - path: spec/functional/expressions/FR-145-convert-and-query-collections.md
        lines: "88-97"
        excerpt: |-
          one addition `t_k = t_(k-1) + e_k` of `N`'s numeric family is evaluated, followed by the
          uncharged FR-044 decision that `t_k` is a member of `N`'s domain. ...
          Under direct kernel evaluation, a non-member seed or prefix is a located undefined outcome
          at that summand or addition; it makes no later charge and exposes no total.
      - path: spec/test-cases/TC-190-collection-query-and-conversion-algebra.md
        lines: "41"
        excerpt: |-
          a located undefined outcome at the addition after `integer-arithmetic.arithmetic` because `4` is outside `[0,3]`, with four work units, no result unit and no total
      - path: proposals/quire-v1/definitions/native-diagnostics.md
        lines: "216-218"
        excerpt: |-
          Each is category refusal: the operation is defined, and the ... declared target domain does not
          admit its result. None is undefined, incomplete or a type error
    finding: "FR-044 (and value-accounting.md:213-217) now make an out-of-domain bounded integer/Rational arithmetic result at evaluation a `refused integer_out_of_domain`/`rational_out_of_domain`, which the catalog says is never undefined. FR-145:94-97 and TC-190 Q12 already classify the same uncharged FR-044 decision on a `sum` prefix under direct kernel evaluation as a located undefined outcome. `sum<Int[0,3]>` over `2, 2` now has two conflicting outcomes. Fix: either align FR-145/TC-190 Q12 (and any other direct-kernel non-member outcome) to the new refusal, or scope the FR-044 rule so `sum` prefixes stay undefined and say why. Choose one outcome."
  - fnd: FND-002
    severity: low
    method: spec-review/integrity
    check: untestable-ac
    artifact_id: FR-044
    path: spec/functional/foundation/FR-044-check-composed-numeric-definedness.md; proposals/quire-v1/definitions/native-diagnostics.md
    lines: "FR-044:40-48; native-diagnostics:102,226-239"
    excerpt: |-
      At evaluation, the evaluator decides a bounded integer or `Rational[..]` arithmetic result's membership ...
      The label check runs first, over the labels in that order, and the first blank label decides `blank-label` ...
      Each payload value is written as one exact ASCII string: ...
    finding: "Three normative rules added by 6389506c have no acceptance criterion and no test: FR-044's evaluation-time refusal (FR-044-AC-1..9 are all checking-time, TC-044/TC-192/TC-193), the blank-label-before-empty-path precedence, and the payload value spellings. Each can change without failing anything."
  - fnd: FND-003
    severity: low
    method: spec-review/integrity
    check: test-intent
    artifact_id: tests/checked_package_v2.rs
    path: tests/checked_package_v2.rs
    lines: "4924-4944"
    excerpt: |-
      backticked(&payload)
          .into_iter()
          .filter(|name| canonical.contains(name))
          .collect()
    finding: "diagnostic_row_payload_keys drops every backticked name that is not already canonical before comparing, so a row that gains an extra non-canonical payload key passes tc_233_revision_1_draft_8_cause_sets_and_payload_keys_are_closed. FR-272-AC-11 says each row names exactly its keys. Renames and drops are caught (reviewer-verified). Additions are not."
```

+++


## Dispositions

Transcribed verbatim from https://linear.app/agent-ix/issue/STD-110/catalog-1-draft8-codes-for-the-kernel-refusal-causes-qsl-cannot-record#comment-3b6d96f8

<!-- reviewer-dispositions repo=agent-ix/quire-specification visibility=private quoin=0.24.1 module=spec-artifacts-process@v0.26.0 id=SR-719 date=2026-09-26 -->



| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2cff3d9 + b894ba8: FR-044, value-accounting (the refusal sentence and the sum schedule) and the catalog all carve out a `sum` seed or running total and hand it to FR-145's located undefined outcome. FR-145:94-97 and TC-190 Q12 are unchanged. The integer and `Rational[..]` families now give one outcome. `tc_233_arithmetic_domain_refusals_leave_sum_prefixes_undefined` (FR-044-AC-10) pins all four documents. The reviewer dropped the FR-044 carve-out in a scratch worktree and the test failed (checked_package_v2.rs:6872). The Decimal and quantity families are not closed in FR-140: see SR-723 FND-001. |
| FND-002 | fixed | 2cff3d9: the three rules now have criteria and traced tests. FR-044-AC-10 is traced by `tc_233_arithmetic_domain_refusals_leave_sum_prefixes_undefined`, FR-272-AC-12 by `tc_233_invalid_source_identity_checks_labels_before_the_path` and FR-272-AC-13 by `tc_233_payload_values_have_one_spelling`. All are in the matrices (composed-foundation, complete-v1). The reviewer swapped `invalid` and `inexact` in the catalog's flag order and the spelling test failed (checked_package_v2.rs:7014). |
| FND-003 | fixed | 2cff3d9: payload keys are now written `key`: in both documents, and `diagnostic_row_payload_keys` takes every backticked name followed by `:`, canonical or not. The reviewer added `` `bits`: the would-be bits `` to the `rational_out_of_domain` rows and the test failed with left `{bits, expected}`, right `{expected}` (checked_package_v2.rs:6803). An added key written without the colon would still slip through. The catalog now states the `key`: convention, so that is a convention breach, not a gap in the test. |

+++ [reviewer data]

```yaml
dispositions:
  - fnd: FND-001
    outcome: fixed
    fix_sha: 2cff3d9
    path: spec/functional/foundation/FR-044-check-composed-numeric-definedness.md
    lines: "40-52"
    after_excerpt: |-
      before retaining it (`quire.value.accounting/v1`). A result outside that domain,
      other than a `sum` seed or prefix, is refused with no value, as
      `refused { code: integer_out_of_domain, cause: outside-domain }` or
      `refused { code: rational_out_of_domain, cause: outside-domain }`,
      respectively. ... A `sum` seed or prefix is not the expression's result: FR-145
      defines `sum` as a fold whose every running total lies in `N`'s domain, ...
      and FR-145 makes that a located undefined outcome, not a refusal.
    verification: "Mutation: remove 'other than a `sum` seed or prefix,' from FR-044 -> tc_233_arithmetic_domain_refusals_leave_sum_prefixes_undefined panics at tests/checked_package_v2.rs:6872."
  - fnd: FND-002
    outcome: fixed
    fix_sha: 2cff3d9
    path: spec/objects/foundation/FR-272-typed-diagnostic-cause.md; spec/functional/foundation/FR-044-check-composed-numeric-definedness.md
    lines: "FR-272:173-174; FR-044:67"
    after_excerpt: |-
      | FR-272-AC-12 | Under catalog revision `1-draft.8`, the native-diagnostics and FR-272 `invalid_source_identity` rows each check the labels ... in that order before the path, the first blank label deciding `blank-label`, and each reports `empty-path` only when every label is non-blank. | Test (TC-233) |
      | FR-272-AC-13 | Under catalog revision `1-draft.8`, the catalog writes each payload value as one exact ASCII string: ... | Test (TC-233) |
    verification: "Mutation: swap `invalid` and `inexact` in the catalog flag-set order -> tc_233_payload_values_have_one_spelling fails at tests/checked_package_v2.rs:7014 (left [inexact,...,invalid], right [invalid,...,inexact])."
  - fnd: FND-003
    outcome: fixed
    fix_sha: 2cff3d9
    path: tests/checked_package_v2.rs
    lines: "4921-4950"
    after_excerpt: |-
      let pieces = payload.split('`').collect::<Vec<_>>();
      pieces
          .iter()
          .enumerate()
          .skip(1)
          .step_by(2)
          .filter(|(index, _)| {
              pieces
                  .get(index + 1)
                  .is_some_and(|after| after.starts_with(':'))
          })
    verification: "Mutation: append '; and `bits`: the would-be bits.' to the native-diagnostics rational_out_of_domain row -> fails at tests/checked_package_v2.rs:6803, left {bits, expected} right {expected}."
```

+++

