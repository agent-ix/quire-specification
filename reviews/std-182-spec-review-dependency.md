---
id: SR-13057
title: "Dependency review of PR #284: FR-375 and FR-432 edges"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-specification@04cc1b9; FR-375 and FR-432 frontmatter relationships and Dependencies sections; TC-310 frontmatter"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-432
    type: reviews
---

## Summary

Ticket: STD-182. The PR adds an FR-375 `references` FR-432 edge and an
FR-432 Dependencies bullet. Checks that the new dependencies are recorded
in both directions where a reader needs them.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-432 now takes its abstract-side selection from "the `abstract-subject` production of FR-375", but its frontmatter still lists only US-011, FR-177, FR-425 and FR-426; there is no FR-375 relationship, so a blast-radius query on FR-375 misses FR-432. The body link predates this PR, but the PR makes FR-432's rule depend on FR-375's grammar. | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:5-13,79-82 |

## Verdict

Mergeable on this lens after FND-001 (one frontmatter line). FR-375's new
FR-432 edge and Dependencies bullet are correct; the QSL side (FR-310,
ADR-020 QS-11) points at QSpec FR-375, which now owns the production.
Follow-ups outside this diff: QSL FR-310's note that the protocol side "has
no QSpec production yet" and ADR-020 QS-11's owner list ("QSpec FR-375 and
FR-177", no FR-432) go stale once this merges.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 262aae1 adds an `ix://agent-ix/quire-specification/FR-375` `references` relationship to FR-432's frontmatter. |
