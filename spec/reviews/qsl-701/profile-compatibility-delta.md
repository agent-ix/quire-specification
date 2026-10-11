---
id: SR-5660
title: "QSL-701 lowering-profile compatibility delta review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@20e2ae1fade110df88f8984164d7be4a53657f85; I12; FR-330; TC-932; TC-935"
review_set: subset
---

## Summary

Final delta review **PASS**. The previous review's concrete findings were
resolved: the unsupported lowering-profile/source-selection mismatch behavior
is removed; the explicit lowering profile is represented by a closed reference
shape; the reader validates that shape without claiming a profile registry or
compatibility predicate; and the provenance and semantic-preservation vectors
now state grounded outcomes.

The accepted boundary is identity-only. `LoweringProfileRef` reuses the closed
`{authority, identity}` form of FR-322 `DefinitionRef`. Neither I12 nor FR-322
defines a mapping from that reference to required source selections. The IR
`CompleteLoweringProfileV2` fields inspected by the reviewer are
`supported_tags`, `require_bounds`, and `work_limit`; there is no typed
source-selection requirement. QSL-706 tracks that separate feature gap.

Independent reviewer: Claude Opus, read-only delta review. Final review
confirmed the changed FR-330, I12, TC-932 and TC-935 against FR-322 and FR-195.
No build was run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-1 | high | Resolved: TC-932 no longer asserts an ungrounded profile/source-selection mismatch refusal; stale identity mutations fail at `/identity/digest`. | TC-932; FR-330-AC-5 |
| FND-2 | medium | Resolved: I12 declares `LoweringProfileRef` with the closed `{authority, identity}` shape reused from FR-322 `DefinitionRef`; FR-330 validates shape only. | I12; FR-330 |
| FND-3 | medium | Resolved: reader ordering checks the closed reference shape and does not assert registry or compatibility validation. | FR-330 |
| FND-4 | low | Resolved: AC-5 and TC-935 retain positive, testable provenance, shape and identity checks. | FR-330-AC-5; TC-935 |
| FND-5 | medium | Resolved: AC-1 grounds meaning in the originating checked-package node under the exact FR-322 lock, and TC-935 compares the represented node. | FR-330-AC-1; TC-935 |
| FND-6 | low | Resolved: TC-932 expects `invalid_package`/`invalid-value` at `/identity/digest` for stale identity. | TC-932; FR-330 |

## Validation

Targeted `quire validate` passed for FR-330, I12, TC-932 and TC-935. The
validator emitted existing module-catalog warnings; no document errors were
reported. `git diff --check` passed. No Cargo or other build was run for this
specification-only change.
