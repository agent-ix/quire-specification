---
id: SR-624
title: "Base review of the ContractPackage value contract"
type: SpecReview
analysis: base
scope: "FR-330; FR-195-AC-6; TC-932 through TC-937"
review_set: base
---

## Summary

The QSL-701 ContractPackage contract is structurally valid and resolves the six requested property decisions, but its canonical digest conflicts with the base-review hash rule, and its new acceptance criteria are not connected to test evidence in the computed matrix. The reader vectors also omit two refusal forms required by FR-330, and FR-330 is not linked to its owning user story.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-5953 | high | FR-330 requires a SHA-256 canonical identity digest for ordinary ContractPackage semantic identity. The base checklist permits a canonical digest only when it binds proof to the exact content proved and trust/provenance is the feature; this requirement defines neither proof verification nor such a trust boundary. Remove the digest requirement under this review rule. | FR-330 identity; TC-936 | wrong-requirement |
| FND-5954 | high | The computed coverage matrix has FR-330-AC-1 and AC-2 at `method-without-symbol`, AC-3 through AC-9 `untagged`, and FR-195-AC-6 `untagged`. TC-932 through TC-937 link only to FR-330/FR-195, not individual acceptance criteria. Add criterion-level trace tags to the six TC frontmatters, mapping each criterion to a vector that actually exercises it, then confirm the matrix resolves every criterion. | FR-330-AC-1 through FR-330-AC-9; FR-195-AC-6; TC-932 through TC-937 | correct-requirement-no-evidence |
| FND-5955 | medium | The reader requirement rejects out-of-domain and noncanonical input, but AC-9 and TC-937 do not exercise either case. Add explicit negative vectors for a wrong identity domain and noncanonical wire bytes, and assert their typed FR-330 reader outcomes before consumer access. | FR-330:169-185; FR-330-AC-9; TC-937 | correct-requirement-no-evidence |
| FND-5956 | medium | FR-330 has no relationship to a user story, despite the base checklist requiring FR-to-US links. FR-195 already identifies US-011 as the lowering story; link FR-330 to the owning story or state the correct owner. | FR-330 frontmatter; FR-195 frontmatter; US-011 | missing-requirement |

## Checks

- ID formats and uniqueness: FR-330, FR-195, FR-330-AC-1..9, FR-195-AC-1..6, and TC-932..937 conform and are unique in this branch. The new TC range follows TC-931 on fresh `origin/main`; no other open PR adds these IDs. Review SR-624 is beyond the fresh-main maximum SR-623 and does not collide with another open PR's review ID.
- Links and structural validation: the changed FR-330, FR-195, and six TC documents passed targeted `quire validate` (exit 0). Repository-wide `quire validate --scope "$PWD" "spec/**/*.md"` exited 3 with 135 documents failing; failures are in unchanged legacy documents. The validator also reported existing module-catalog duplicate-archetype/inverse-edge warnings.
- Coverage: `quire matrix --scope "$PWD" --format tsv` reported the criterion states recorded in FND-5954.
- Hash/digest check: the FR-330 SHA-256 identity is recorded in FND-5953.
- The six checklist coverage rules were checked: there are no option sets or explicit boundary constraints; error-path coverage is incomplete as recorded in FND-5955; per-item and absent-key edge cases are represented in TC-933/TC-934; no state transition applies. Criterion trace coverage remains open under FND-5954.
