---
id: SR-16354
title: "spec-review of FR-352 clause-run record carries the extraction member (quire-specification PR 424)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@cf057e31; spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-352
    type: references
---

# Spec review of FR-352 clause-run record carries the extraction member (quire-specification PR 424)

## Summary

Ticket: STD-344. One spec-review pass over `git diff origin/main...cf057e31` (one file, FR-352): standing rule, AC realizability, id uniqueness across open QSpec heads, QSL name cross-check, mergeability, gates.

Examined, clean:

- No deferral or narrowing language. FR-352-AC-13 is on no other open head; AC-4/AC-5 gaps predate the PR.
- The member list sentence, the AC and the Dependencies note agree with QSL FR-031/FR-267 (original, body, binding, outcome, map, original_regions) as read at QSL #1313 15a7ff35.
- Gates: `quire validate` 86 failure lines (same as main); QSpec cargo tests 24 and 139 passed with #424 and #393 merged onto main; QSL conformance `checked_v2` 49 passed; no QSL-parsed file is touched.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-352 makes `extraction` a member of the closed clause-run record and requires the strict reader to refuse an unknown, missing or null member of it, but defines no members: "QSL FR-031 owns its fields; this requirement adds none". A reader built from QSpec alone cannot decide which member is unknown or missing, so AC-13's three refusal vectors have no oracle in this repository. List the member set (original, body, binding, outcome, map, optional original_regions) in FR-352, as QSL FR-267 does, or cite the QSL requirement as the normative member list with its name. | spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md | missing-requirement |

## Verdict

Mergeable after FND-001 is dispositioned. Merge-tree against main is clean. QSL #1313 depends on this PR.
