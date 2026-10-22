---
id: SR-723
title: "Spec review (integrity, delta 2) — STD-110 catalog 1-draft.8 fix round 2"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #162 delta 6389506c..b894ba82"
review_set: subset
---
# Spec review (integrity, delta 2) — STD-110 catalog 1-draft.8 fix round 2

Transcribed verbatim from https://linear.app/agent-ix/issue/STD-110/catalog-1-draft8-codes-for-the-kernel-refusal-causes-qsl-cannot-record#comment-d714fdec

## Summary

Delta review of fix round 2 (`6389506c..b894ba82`). The coder adds `reviews/2026-09-26-std-110-spec-review-integrity-delta-2.md` with id SR-723 in the next commit that touches the branch.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | 2cff3d9 made FR-140 (Behavior and FR-140-AC-5) name `decimal_out_of_domain` for every decimal nonmember, with no `sum` carve-out. value-accounting and the catalog make a `Decimal[..]` (and quantity) `sum` seed or running total outside `N` FR-145's located undefined outcome instead. FR-044 carries the carve-out for integer/Rational; FR-140 does not, and tc_233_arithmetic_domain_refusals_leave_sum_prefixes_undefined checks only the integer/Rational documents. A reader of FR-140 alone gets a refusal for `sum<Decimal[..]>` overflow. Fix: add the same one-clause carve-out to FR-140 (the quantity case follows through FR-140/FR-142's target membership), optionally extend the FR-044-AC-10 test to FR-140. | spec/functional/type-model/FR-140-evaluate-exact-decimals.md:96-102,150 |

## Reviewer comment (verbatim)

<!-- reviewer repo=agent-ix/quire-specification visibility=private quoin=0.24.1 module=spec-artifacts-process@v0.26.0 id=SR-723 method=spec-review/integrity lang=spec pr=quire-specification#162 date=2026-09-26 -->

Delta review of fix round 2 (`6389506c..b894ba82`). The coder adds `reviews/2026-09-26-std-110-spec-review-integrity-delta-2.md` with id SR-723 in the next commit that touches the branch.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-001 | low | soundness | FR-140 now states `decimal_out_of_domain` for every decimal nonmember, with no `sum` carve-out. value-accounting and the catalog say a Decimal (or quantity) `sum` running total is FR-145's located undefined outcome "in place of the family's membership refusal". FR-044 got the carve-out; FR-140 did not, and the FR-044-AC-10 test covers only the integer and `Rational[..]` families. |

Not findings (verified at b894ba82):
- **Lock digests:** all 49 digest entries in complete-value-lock.json, complete-model-lock.json and value-complete-rules.json equal the raw `sha256` of the current bytes. None of the 7 digests replaced this round (`e7cf850f`, `6fdab8c8`, `3787474a`, `4d0dcb64`, `5438519e`, `644e325d`, `69780c29`) or the prior model-lock `533c35d7` appears outside reviews/. Revisions are unchanged, which is correct.
- **Tests and mutations:** full `cargo test` passes (66 tests across 4 targets). The reviewer ran 3 mutations in a scratch worktree (drop the FR-044 carve-out, swap flag order, add a `bits`: key) and each failed its test. The scratch worktree and target dir are deleted.
- **Sum-prefix sweep:** FR-044, FR-145, TC-190 Q12, value-accounting (both places), the catalog, FR-041-AC-6 and state-contract all agree for the integer and `Rational[..]` families. IEEE sums have no domain membership.
- **Transcribed review files:** the five transcribed `## Dispositions` sections and the SR-719 file match the Linear comments byte for byte, apart from blank lines. `quire validate` on reviews passes. One nit: the SR-719 wrapper's FND-002 Refs cell reads `...native-diagnostics.md:FR-044:40-48; native-diagnostics:102,226-239` (garbled).

+++ [reviewer data]

```yaml
scope:
  - {id: FR-044, path: "spec/functional/foundation/FR-044-check-composed-numeric-definedness.md"}
  - {id: FR-140, path: "spec/functional/type-model/FR-140-evaluate-exact-decimals.md"}
  - {id: FR-141, path: "spec/functional/type-model/FR-141-evaluate-text-and-enumerations.md"}
  - {id: FR-145, path: "spec/functional/expressions/FR-145-convert-and-query-collections.md"}
  - {id: FR-147, path: "spec/functional/expressions/FR-147-evaluate-integer-division-domains.md"}
  - {id: FR-148, path: "spec/functional/expressions/FR-148-evaluate-ieee-floating-profiles.md"}
  - {id: FR-272, path: "spec/objects/foundation/FR-272-typed-diagnostic-cause.md"}
  - {id: TC-185, path: "spec/test-cases/TC-185-exact-decimal-semantics.md"}
  - {id: TC-186, path: "spec/test-cases/TC-186-text-and-enum-identity-semantics.md"}
  - {id: TC-190, path: "spec/test-cases/TC-190-collection-query-and-conversion-algebra.md"}
  - {id: TC-192, path: "spec/test-cases/TC-192-integer-division-profiles.md"}
  - {id: TC-193, path: "spec/test-cases/TC-193-ieee-exceptional-and-rounding-profiles.md"}
  - {id: TC-233, path: "spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md"}
  - {id: native-diagnostics, path: "proposals/quire-v1/definitions/native-diagnostics.md"}
  - {id: value-accounting, path: "proposals/quire-v1/definitions/value-accounting.md"}
  - {id: complete-value-lock, path: "proposals/quire-v1/definitions/complete-value-lock.json"}
  - {id: value-complete-rules, path: "proposals/quire-v1/definitions/value-complete-rules.json"}
  - {id: complete-model-lock, path: "proposals/quire-v1/definitions/complete-model-lock.json"}
  - {id: complete-v1-matrix, path: "spec/complete-v1/tests.md"}
  - {id: composed-foundation-matrix, path: "spec/composed-foundation/tests.md"}
  - {id: tests/checked_package_v2.rs, path: "tests/checked_package_v2.rs"}
  - {id: reviews, path: "reviews/2026-09-26-std-110-*.md"}
findings:
  - fnd: FND-001
    severity: low
    method: spec-review/integrity
    check: soundness
    artifact_id: FR-140
    path: spec/functional/type-model/FR-140-evaluate-exact-decimals.md
    lines: "96-102,150"
    excerpt: |-
      Every result
      is kept at its retained representation (`v × 10^T`, `T`) after any rounding
      step and then checked for membership by value; a
      nonmember refuses as
      `refused { code: decimal_out_of_domain, cause: outside-domain }` with no value
      and is never re-rounded to a coarser scale,
    conflicts_with:
      - path: proposals/quire-v1/definitions/value-accounting.md
        lines: "428-431,437-438"
        excerpt: |-
          A seed or running total that is not a member of `N`'s domain ends the sum with FR-145's located
          undefined outcome at that summand or addition, in place of the family's membership refusal
      - path: proposals/quire-v1/definitions/native-diagnostics.md
        lines: "219-221"
        excerpt: |-
          A `sum` seed or running total outside `N`'s domain is none of these refusals: FR-145 makes it a located undefined outcome
    finding: "2cff3d9 made FR-140 (Behavior and FR-140-AC-5) name `decimal_out_of_domain` for every decimal nonmember, with no `sum` carve-out. value-accounting and the catalog make a `Decimal[..]` (and quantity) `sum` seed or running total outside `N` FR-145's located undefined outcome instead. FR-044 carries the carve-out for integer/Rational; FR-140 does not, and tc_233_arithmetic_domain_refusals_leave_sum_prefixes_undefined checks only the integer/Rational documents. A reader of FR-140 alone gets a refusal for `sum<Decimal[..]>` overflow. Fix: add the same one-clause carve-out to FR-140 (the quantity case follows through FR-140/FR-142's target membership), optionally extend the FR-044-AC-10 test to FR-140."
```

+++

