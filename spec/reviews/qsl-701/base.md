---
id: SR-624
title: "Base review of the ContractPackage value contract"
type: SpecReview
analysis: base
scope: "FR-330; FR-195-AC-6; TC-932 through TC-937"
review_set: base
---

## Summary

The author addressed the reader-vector and user-story-link findings. Criterion-to-vector links and complete-V1 test-matrix rows are now present, although the computed Quire matrix still reports the criteria untagged because this spec-only change has no source trace binders; I accept that as implementation trace debt for this PR, with trace binding still required before code qualification. The requested semantic digest remains an explicit conflict with the base checklist's digest antipattern and is retained per the user's QSL-701 scope, not cleared by this review.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-5953 | high | Retained at the user's explicit QSL-701 scope, which requires a domain-separated semantic identity. The base checklist's hash/digest antipattern still applies: this is not shown to bind a proof where trust and provenance are the feature. This conflict is recorded, not fixed or cleared. | FR-330 identity; TC-936 | wrong-requirement |
| FND-5954 | medium | Spec-side mapping is now present: criterion-level `verifies` edges and `complete-v1/tests.md` rows map FR-330-AC-1..9 and FR-195-AC-6 to their vectors. The computed matrix still reports all ten criteria `untagged`, because the spec-only slice has no source trace binders. Accept this as implementation trace debt for this spec PR; actual source tests must bind these criteria before code qualification. This does not satisfy the matrix checklist globally. | FR-330-AC-1 through FR-330-AC-9; FR-195-AC-6; TC-931 through TC-937; complete-v1/tests.md | correct-requirement-no-evidence |
| FND-5955 | low | Resolved on b58bcd2: TC-937 now includes a wrong identity domain and noncanonical wire encoding, with the typed domain refusal and canonical-wire refusal stated in its expected results. | FR-330:169-185; FR-330-AC-9; TC-937 | correct-requirement-no-evidence |
| FND-5956 | low | Resolved on b58bcd2: FR-330 now implements US-011 in its frontmatter. | FR-330 frontmatter; FR-195 frontmatter; US-011 | missing-requirement |

## Checks

- ID formats and uniqueness: FR-330, FR-195, FR-330-AC-1..9, FR-195-AC-1..6, and TC-932..937 conform and are unique in this branch. The new TC range follows TC-931 on fresh `origin/main`; no other open PR adds these IDs. Review SR-624 is beyond the fresh-main maximum SR-623 and does not collide with another open PR's review ID.
- Links and structural validation: the changed FR-330, FR-195, and six TC documents passed targeted `quire validate` (exit 0). Repository-wide `quire validate --scope "$PWD" "spec/**/*.md"` exited 3 with 135 documents failing; failures are in unchanged legacy documents. The validator also reported existing module-catalog duplicate-archetype/inverse-edge warnings.
- Coverage: `quire matrix --scope "$PWD" --format tsv` still reports FR-330-AC-1..9 and FR-195-AC-6 `untagged`; its source-trace model has no binders in this spec-only worktree. The authored criterion-to-vector edges and complete-V1 test rows now exist, as recorded in FND-5954; implementation must add source trace tags.
- Hash/digest check: the FR-330 SHA-256 identity is recorded in FND-5953.
- The six checklist coverage rules were checked: there are no option sets or explicit boundary constraints; error-path coverage is incomplete as recorded in FND-5955; per-item and absent-key edge cases are represented in TC-933/TC-934; no state transition applies. Criterion trace coverage remains open under FND-5954.
