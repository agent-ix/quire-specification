---
id: SR-16350
title: "spec-review of FR-478 convert text to a number (quire-specification PR 395, fix round)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@d92ea803; spec/functional/type-model/FR-478-convert-text-to-a-number.md; spec/test-cases/TC-964-text-converts-to-a-number.md; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md; spec/functional/expressions/FR-146-check-total-pure-functions.md; spec/functional/expressions/FR-148-evaluate-ieee-floating-profiles.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; proposals/quire-v1/definitions/value-accounting.md; proposals/quire-v1/shared-grammar.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-478
    type: references
  - target: ix://agent-ix/quire-specification/TC-964
    type: references
  - target: ix://agent-ix/quire-specification/FR-140
    type: references
---

# Spec review of FR-478 convert text to a number (quire-specification PR 395, fix round)

## Summary

Ticket: STD-365. One spec-review pass over `git diff origin/main...d92ea803`, checked for the no-drop/no-narrowing/no-deferral rule, hand-derived vectors, consistency with FR-140, FR-148, FR-149, FR-322 and value-accounting, renamed-file links, id uniqueness across main and all open PR heads, and gates.

Gates: `quire validate --scope . 'spec/**/*.md'`: 86 failure lines (equal to main; none in FR-478 or TC-964). `cargo test --test checked_package_v2 --test backend_provider_v1`: 139 + 24 passed. Merge-tree against current origin/main (7d65c93b, one commit ahead of the branch's merge) is clean.

Examined, clean:

- Scope: the earlier integer-and-decimal cut is removed. Every numeric target (`Integer`/`Int[..]`, `Decimal`/`Decimal[..]`, `Rational`/`Rational[..]`, `Float32[m]`, `Float64[m]`, `Quantity<u>`) and both `Text[..]` and `String` operands are specified with grammar, outcomes, accounting, identities and vectors. Non-number targets keep the existing `ill_typed`/`operator-ineligible`; no existing capability is removed.
- Catalog: FR-478 Dependencies states that the twelve rows go into the `quire-verification-contracts` catalog and that both V2 schemas then enumerate them; the PR body names STD-393 for that work, and STD-393 lists all twelve rows.
- IEEE vectors recomputed with an exact rational rounder: `0.1` nearest-even `0x3fb999999999999a`, toward-zero `0x3fb9999999999999`; binary32 `0x3dcccccd` / `0x3dcccccc`; `1e309` nearest-even `0x7ff0000000000000` and toward-zero `0x7fefffffffffffff`, each `{overflow, inexact}`; `1e-400` nearest-even `+0` and toward-positive `0x0000000000000001`, each `{underflow, inexact}`; `0.25` and `2.5e-1` exact. Specials, `-0`, canonical NaN and the strict-`exact` would-be flags match FR-148.
- Decimal: `1.555` at scale 2 nearest-even gives `156` (tie, even), `1.565` gives `156`, `DecimalLoss` `311/200`; `10.01` coefficient `1001` is outside `[0, 1000]`; sizes `bits(1555)=11`, `digits=4`.
- Rational reduction and domain (`2/4`, `0.25`, `6/4`, `1/10`, `10`) and every undefined spelling follow the EBNF and the sign-of-zero table. Quantity unit suffix rules and vectors are consistent.
- Accounting: `42` sizes `integer_bits` 6 / `decimal_digits` 2; `0.1` into an IEEE target sizes `sbits(1,1)=5` / `sdigits(1,1)=2`; `ieee.round` uses the target width (64), matching value-accounting.
- Refusal code/cause pairs all exist in FR-271/FR-272. The mode design matches FR-322 `type_pinned_modes` (result type alone for a `convert`).
- Renamed files: no reference to the old FR-478 or TC-964 file names remains in either repo. FR-478 and TC-964 are on no other open PR head and not on main.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-140 and FR-322's `scale_reduction` constraint still say a scale reduction (`quire.op.numeric.convert_rounding`) is "the only conversion that rounds". FR-478's `quire.op.text.to_decimal` rounds a decimal under a non-`exact` mode without being a scale reduction. FR-149 was narrowed to "among exact-numeric conversions"; FR-140 and FR-322 were not, so they now contradict FR-478 for a decimal target. Fix: qualify both sentences the same way FR-149 now reads, or name FR-478's decimal rounding as the other rounding conversion. | spec/functional/type-model/FR-140-evaluate-exact-decimals.md:115-117; spec/objects/interfaces/FR-322-checked-package-artifact.md:1906 | wrong-requirement |
| FND-002 | medium | FR-478 keeps a decimal result as spelled (`c`, `s`) whenever `s` is at most the target's maximum scale `s2`, so `convert<Decimal[0, 1000; 0, 2; exact]>` of `1.5` would retain (`15`, `1`). FR-140 says a computed result is retained at (`v × 10^T`, `T`) with `T = smax`, which gives (`150`, `2`). FR-478 cites only FR-140's literal-spelling rule and does not amend the computed-result rule, and no vector has `s < s2` into a bounded decimal, so the retained representation (which later decimal accounting reads) is ambiguous. Fix: either retain at the target scale `s2` (like every other decimal result), or state the spelled retention as an exception in FR-140; add a TC-964 vector such as `1.5` into `Decimal[0, 1000; 0, 2; exact]`. | spec/functional/type-model/FR-478-convert-text-to-a-number.md:185-187; spec/functional/type-model/FR-140-evaluate-exact-decimals.md:78-80 | wrong-requirement |

## Verdict

Not mergeable yet. The scope finding from SR-16349 (FND-001) is fixed and all vectors are correct, but FND-001 and FND-002 (medium) should be fixed in this PR. SR-16349 FND-002 (`spec/spec.md` does not list FR-478) is still open. SR-16349 is not yet committed under `reviews/` in this repo.
