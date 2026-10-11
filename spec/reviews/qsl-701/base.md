---
id: SR-5658
title: "Base review of the ContractPackage value contract"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@b96f2064015e7f6d20b70686c36830c11f8a45ac; FR-330; FR-195-AC-6; TC-932 through TC-937"
review_set: base
---

## Summary

The author addressed the reader-vector and user-story-link findings. Criterion-to-vector links and complete-V1 test-matrix rows are present, although the computed Quire matrix still reports the criteria untagged because this spec-only change has no source trace binders; trace binding remains required before code qualification. On final head `b96f206`, independent Opus review rejected FND-5953: the requested digest is a canonical identity over semantic content, not file or version tracking, and it does not replace provenance checks. FND-5954 is deferred as implementation trace debt.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-5953 | high | Retained at the user's explicit QSL-701 scope, which requires a domain-separated semantic identity. The base checklist's hash/digest antipattern still applies: this is not shown to bind a proof where trust and provenance are the feature. This conflict is recorded, not fixed or cleared. | FR-330 identity; TC-936 | wrong-requirement |
| FND-5954 | medium | Spec-side mapping is now present: criterion-level `verifies` edges and `complete-v1/tests.md` rows map FR-330-AC-1..9 and FR-195-AC-6 to their vectors. The computed matrix still reports all ten criteria `untagged`, because the spec-only slice has no source trace binders. Accept this as implementation trace debt for this spec PR; actual source tests must bind these criteria before code qualification. This does not satisfy the matrix checklist globally. | FR-330-AC-1 through FR-330-AC-9; FR-195-AC-6; TC-931 through TC-937; complete-v1/tests.md | correct-requirement-no-evidence |
| FND-5955 | low | Resolved on b58bcd2: TC-937 now includes a wrong identity domain and noncanonical wire encoding, with the typed domain refusal and canonical-wire refusal stated in its expected results. | FR-330:169-185; FR-330-AC-9; TC-937 | correct-requirement-no-evidence |
| FND-5956 | low | Resolved on b58bcd2: FR-330 now implements US-011 in its frontmatter. | FR-330 frontmatter; FR-195 frontmatter; US-011 | missing-requirement |

## Checks

- ID formats and uniqueness: FR-330, FR-195, FR-330-AC-1..9, FR-195-AC-1..6, and TC-932..937 conform and are unique in this branch. The new TC range follows TC-931 on fresh `origin/main`; no other open PR adds these IDs. A repository-wide scan of main and fetched remote branches found SR-5657 as the maximum; SR-5658 is unused.
- Links and structural validation: the changed FR-330, FR-195, and six TC documents passed targeted `quire validate` (exit 0). On current main `c385e69a` and final PR head `b96f206`, repository-wide `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' --summary` exited 3 on both trees: main 142 failed documents, PR 141. The PR has no PR-only failed names; main-only is `spec/objects/interfaces/interface_012-contract-package.md`, whose duplicate `Features` table the PR removes. The validator also reports existing module-catalog duplicate-archetype/inverse-edge warnings.
- Coverage: `quire matrix --scope "$PWD" --format tsv` still reports FR-330-AC-1..9 and FR-195-AC-6 `untagged`; its source-trace model has no binders in this spec-only worktree. The authored criterion-to-vector edges and complete-V1 test rows now exist, as recorded in FND-5954; implementation must add source trace tags.
- Hash/digest check: the FR-330 SHA-256 identity is recorded in FND-5953.
- The six checklist coverage rules were checked: there are no option sets or explicit boundary constraints; error-path coverage is incomplete as recorded in FND-5955; per-item and absent-key edge cases are represented in TC-933/TC-934; no state transition applies. Criterion trace coverage remains open under FND-5954.

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-5953 | rejected | The canonical semantic identity binds a proof to the exact semantic content proved. It contains no file path, commit SHA, or tool version; provenance remains separately checked. |
| FND-5954 | deferred | Source trace binding requires implementation tests and is outside this spec-only PR; it remains required before code qualification. |
| FND-5955 | fixed | b58bcd2 |
| FND-5956 | fixed | b58bcd2 |

```yaml
dispositions:
  - fnd: FND-5953
    outcome: rejected
    reason: "The RFC 8785 digest identifies the exact semantic ContractPackage projection; it does not track files or versions, and FR-330 separately validates source provenance."
  - fnd: FND-5954
    outcome: deferred
    reason: "Source trace binding requires implementation tests and is outside this spec-only PR; it remains required before code qualification."
  - fnd: FND-5955
    outcome: fixed
    fix_sha: b58bcd2
    after_excerpt: |-
      The wrong identity domain is refused as `invalid_package`/`invalid-value` at `/identity/domain`, using FR-322's existing typed pair; `digest_domain_mismatch` remains for a wrong domain on an input `PackageId`. Noncanonical wire bytes are refused as `noncanonical_wire` at `package_read`, with no cause or pointer, before identity recomputation.
  - fnd: FND-5956
    outcome: fixed
    fix_sha: b58bcd2
    after_excerpt: |-
      relationships:
        - target: ix://agent-ix/quire-specification/US-011
          type: implements
```

## Rebase review

Independent Opus review of the rebased branch and final I12 cleanup found no new defect. The original six PR patches were preserved on current main `c385e69a`; the only additional source change removes the duplicate `Features` table added by main, retaining the single `lower` operation feature.
