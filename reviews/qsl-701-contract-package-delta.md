---
id: SR-5658
title: QSL-701 ContractPackage reader clarification delta review
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@c1cffe5c16d5ef5f8505d1dbdc57262b3e0a845a; FR-330-contract-package.md; TC-933-contract-package-accounting.md; TC-937-contract-package-reader.md"
review_set: subset
---

# QSL-701 ContractPackage reader clarification delta review

## Summary

PASS. All six medium findings and the final low finding are resolved on the exact reviewed head; no blockers remain.

Reviewed head: `c1cffe5c16d5ef5f8505d1dbdc57262b3e0a845a`.

Reviewer: Opus 5.5 (`claude-opus-5-5`), independent fresh read-only review. The review covered the full diff from `ac0b37afc3dda6e7b81814561b8d114be91b4f73` (three specification files, +112/−48) and the final delta from `0369ba6` to this head. No build or tests were run as part of the review.

## Findings

All six medium findings are resolved:

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-5658 | medium | Resolved (M1): FR-330 permits `""` and forbids `null`; vectors distinguish the expected code and cause. | `FR-330-contract-package.md` |
| FND-5659 | medium | Resolved (M2): own digest mismatch maps to `invalid_package` / `invalid-value`; dependency mismatch maps to `stale_dependency` / `content-mismatch`, aligned with FR-322. | `FR-330-contract-package.md` |
| FND-5660 | medium | Resolved (M3): TC-933 retains the source-map accounting bijection refusal. | `TC-933-contract-package-accounting.md` |
| FND-5661 | medium | Resolved (M4): FR-330 requires a known FR-272 cause in an FR-322-admitted pair; TC-937 covers each case. | `FR-330-contract-package.md`; `TC-937-contract-package-reader.md` |
| FND-5662 | medium | Resolved (M5) within this repository: FR-330 specifies decimal-string grammar; `ReadResult` has no wire form. | `FR-330-contract-package.md` |
| FND-5663 | medium | Resolved (M6): the specification consistently names Contract IR FR-038 and links QSL FR-096 for relevant counter and path rules. | `FR-330-contract-package.md` |
| FND-5664 | low | Resolved: FR-330 names the declared-versus-recomputed identity case and explains why self-digest mismatch differs from dependency mismatch. | `FR-330-contract-package.md` |

## Residual note

No blockers remain. One optional wording nit remains non-blocking: FR-330 calls the declared-`package_id` case "compile-time", while FR-272 says "recomputed from the compiled source." The reviewer identified this as an interpretation of FR-272, not an explicit timing term in that clause; the mismatch mapping is unaffected.

The review confirms M5 and M6 against this repository's text only; the reviewer could not inspect the separate QSL and Contract IR repositories. No status above depends on QSL-701 ticket prose.
