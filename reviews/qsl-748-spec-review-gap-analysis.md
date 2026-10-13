---
id: SR-9004
title: "gap-analysis review of quire-specification PR 246 (variant-typed members, traceability)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@852c515d143bb2fbdce5ed186cd68e686493dfb6; spec/test-cases/TC-939-admit-variant-typed-field-parameter-and-result.md, spec/test-cases/TC-940-variant-type-node-key-is-alias-independent.md, spec/test-cases/TC-941-resolve-variant-type-and-case-names.md, spec/test-cases/TC-235-business-construct-meaning-binding.md, spec/complete-v1/tests.md"
review_set: subset
---

## Summary

Ticket: QSL-748. This is a traceability and test-case review. Each new or amended AC was
checked for a verifying TC that can run as written: FR-208-AC-15, FR-208-AC-16,
FR-208-AC-17, FR-154-AC-11, FR-322-AC-71 and FR-141-AC-7. Each one has a TC. The TC ids
TC-939, TC-940 and TC-941 and the AC ids FR-208-AC-16, FR-208-AC-17, FR-322-AC-71 and
FR-141-AC-7 are unique across all remote branches and open PRs (the highest other TC is
TC-938). TC-235 M28 and M29 now expect admission. Three TC fixture defects follow.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | TC-939 builds on TC-235's K2 and the M24 package, then adds fields to object `Account` and record value type `Settings`. K2 declares neither: its object type is `Order` and its record value type is `Money`. The procedure cannot run as written. Use `Order` and `Money`, or declare `Account` and `Settings`. | spec/test-cases/TC-939-admit-variant-typed-field-parameter-and-result.md:22-29 | correct-requirement-no-evidence |
| FND-002 | medium | TC-940 declares `Level` as a "record value type with cases `Gold` and `Silver`". A record value type has no cases. Expected result 3 compares the key of `Level`'s `Gold` node with `Tier`'s, which needs `Level` to be a variant type, as in TC-941. | spec/test-cases/TC-940-variant-type-node-key-is-alias-independent.md:26-27 | correct-requirement-no-evidence |
| FND-003 | medium | TC-939 step 5's clause, `self.tier = self.tier`, reads a field and does not name the variant type or a case. It cannot fail on the lifted rule it claims to verify: a clause that names a variant type raises no `unsupported_construct`/`declaration-form` (FR-208-AC-15's last sentence). Name the type and a case, for example `self.tier = M::Tier::Gold`, as QSL TC-1007 step 2 does. | spec/test-cases/TC-939-admit-variant-typed-field-parameter-and-result.md:34 | correct-requirement-no-evidence |

## Verdict

Changes requested: three medium findings. The traceability links are otherwise sound:
TC-941 covers FR-208-AC-17, TC-940 covers FR-322-AC-71 and FR-141-AC-7, and TC-939 plus
TC-235 cover FR-208-AC-15, FR-208-AC-16 and FR-154-AC-11. On the complete-v1 matrix:
`spec/complete-v1/tests.md` (TM-009) is a hand-maintained traceability table, and it
already lacked FR-208-AC-14 and AC-15, FR-154-AC-9 to AC-11 and FR-322-AC-68 to AC-70
before this PR. This PR needs no rows there. The computed matrix (`quire matrix`) is the
traceability source. A hand-written copy is tracking ceremony that drifts, so it should be
deleted under its own ticket rather than kept up to date.
