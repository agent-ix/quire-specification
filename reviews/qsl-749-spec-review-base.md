---
id: SR-9011
title: "base review of quire-specification PR 248 (delete the TM-009 traceability matrix)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@a62e9757c324457354eba1cd0458053363f581bd; spec/complete-v1/tests.md (deleted), spec/spec.md, docs/v1-capability-inventory.md"
review_set: subset
---

## Summary

Ticket: QSL-749. Base checklist review of PR 248, which deletes the hand-maintained
TM-009 matrix `spec/complete-v1/tests.md`, its `contains` relationship and index row in
`spec/spec.md`, and the sentence pointing at it in `docs/v1-capability-inventory.md`.
The PR head is based on current `main` (13f0d75, #246) and merges with no conflict. The
diff is three files, deletions only, with no new matrix, ledger, count or check. The
deletion itself is clean. The tracking-record rule of the checklist also finds seven
sibling hand-maintained TestMatrix files that nothing consumes, outside this diff.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | The repo still holds seven hand-maintained TestMatrix documents of the same kind as TM-009: TM-001, TM-002, TM-003, TM-004, TM-006, TM-007 and TM-008, each a hand-written `Functional Req / Acceptance Criteria / Test Cases / Status` table. Nothing under `src/`, `tests/`, `Cargo.toml`, AGENTS.md or CLAUDE.md reads them. 28 TC files and several plan files repeat their status in prose ("TM-004 remains Planned"). This is the file-tracking pattern QSL-749 removes, outside this PR's diff. Delete them and their status prose in QSL-749 or a ticket the lead names. | spec/verification-reference/tests.md; spec/native-source/tests.md; spec/composed-foundation/tests.md; spec/protocol/tests.md; spec/temporal/tests.md; spec/observation/tests.md; spec/output-mapping/tests.md; spec/test-cases/TC-110-select-temporal-profile.md:12 | wrong-requirement |

## Verdict

PR 248's own change passes the base checklist:

- `spec/complete-v1/` is gone, and nothing outside historical `reviews/` and
  `spec/reviews/` names `TM-009`, `complete-v1/tests.md` or `complete-v1/` (the one
  `complete-v1` hit is the unrelated id `complete-v1-subsystem-index`).
- `tests/*.rs`, `src/`, `Cargo.toml` and AGENTS.md/CLAUDE.md do not read the path. The
  repo has no Makefile, xtask, scripts or `.github` workflow that reads it.
- The `spec/spec.md` relationship to TM-009 and its index row are removed together, so no
  edge dangles.
- No replacement record or check is added.
- Historical `reviews/` and `spec/reviews/` files are untouched.

FND-001 is outside the diff and does not make this PR's deletion wrong. It needs a
disposition from the lead.
