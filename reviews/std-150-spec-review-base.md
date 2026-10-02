---
id: SR-1182
title: "Base review of PR #178: source identity without source revision"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@ef8e9bc557b503ce4eab9a1f78b5449743a85d24; PR #178 diff against origin/main 58f25068, STD-150 part (commit 785b44f): FR-003, FR-004, FR-017, FR-019, FR-030, FR-095, FR-121, FR-134, FR-200, FR-202, FR-259, FR-271, FR-272, FR-288, FR-298, FR-302, FR-310, FR-320, FR-322, interface_001, interface_002, AD-001, TC-002, TC-011, TC-040, TC-233-adjacent tests, complete-v1/tests.md, docs/v1-complete-architecture.md, proposals/checked-package-v2 schema and fixtures, proposals/shared-reference-2-draft, proposals/state-core, proposals/quire-v1, tests/checked_package_v2.rs"
review_set: subset
---
# Base review of PR #178 (STD-150)

## Summary

Ticket: STD-150. Source identity becomes the document identity plus the
content digest, and the source revision is removed. I checked the diff for
dangling revision references, compatibility paths, the untouched domain and
requirement revisions, the recomputed state-core digests, and the deletions.

- No compatibility path: `RawSourceRef`, `ArtifactRef` and the state-core
  shared-reference schema drop `revision` outright, with no optional or legacy
  member and no reader accepting the old shape.
- Requirement revisions are untouched: the state-core subjects and
  invocations keep `"revision": 1` under `authored`/`subject`, and FR-288 keeps
  "requirement revision". No domain revision changed.
- The 23 recomputed state-core digests in the diff each equal the SHA-256 of
  the named fixture file's bytes at this head (34 path-and-digest pairs
  checked in all).
- The deletions are the revision members, the `changed-source-revision`
  reference case, FR-003-AC-4 and its complete-v1 row, and the two catalog
  labels. FR-272-AC-12, AC-13 and `tests/checked_package_v2.rs` follow.
- `cargo test --locked` passes (82 tests).

## Verdict

Changes needed: one medium finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-003-AC-4 ("A missing numeric IR revision mapping refuses executable projection") is deleted together with FR-003's "explicit positive numeric IR revision mapping" sentence. Before this PR, `proposals/state-core/boundary.md` applied that mapping to source revisions and stated "The same applies to requirement revisions". The rewritten boundary.md:44-46 keeps the requirement half: "A requirement's positive authored revision maps explicitly to the IR requirement revision; a missing mapping refuses the executable projection." With FR-003-AC-4 gone, no FR or AC states or tests that refusal, so the requirement-revision rule this PR means to keep exists only in proposal prose. Either give the requirement-revision mapping refusal its own AC (in FR-003 or in the FR that owns authored requirement identity), or drop it from boundary.md if it is no longer wanted. | spec/functional/foundation/FR-003-bind-source-correspondence.md:33; proposals/state-core/boundary.md:44-46 |
