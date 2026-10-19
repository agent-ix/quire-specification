---
id: SR-12898
title: "Failure-domain review of PR #282: identity and refusal edges of strong fairness rows"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@88bf7df; FR-375 Fairness rows, Checked-package node, AC-3, AC-7, AC-10; TC-310 mutation 11"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-310
    type: reviews
---

## Summary

Ticket: STD-178. PR quire-specification#282. Looks for unstated failure modes
and identity confusion in the kind-bearing fairness rows: weak and strong rows
on one operation, duplicate rows, sort order and refusal location.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-375 says `fair weak each op` and `fair strong each op` "are different rows", but it does not say what happens to two identical fairness rows. AC-3 lists conflicting-binding duplicates for every other row kind, not for fairness rows. QSL FR-137 de-duplicates them ("two equal rows check as one constraint"). A QSpec checker that refuses `conflicting-binding` and one that de-duplicates are both conformant, and only the de-duplicating one agrees with QSL. The gap predates this PR, but the PR is the point where the row's identity tuple is defined. | spec/functional/analysis/FR-375-declare-model-refinement.md:164-171,203 |

## Verdict

Mergeable. Kind is part of the emitted entry, so the identity projection
separates weak from strong. Sort order is unaffected because arrays sort by
the operation's node key. The refusal location for a kindless or unknown kind
is pinned to the token after `fair`. Mutation 11 places a strong `whole` row
on `retryA` beside the positive unit's weak `each` row; FR-362 keeps both
kinds on one operation, so that is admitted.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 847b7a2 identifies a fairness row by its (kind, operation, granularity) triple, with an unwritten granularity read as `whole`. Two rows of one set with the same triple refuse `invalid_model_binding`/`conflicting-binding` naming both, and FR-375-AC-3 lists this. TC-310 mutation 11 adds the identical-pair and `weak`/`weak whole` vectors. QSL FR-137 was changed to the same refusal in aea8326f7. |
