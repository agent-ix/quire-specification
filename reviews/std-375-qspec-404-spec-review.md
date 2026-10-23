---
id: SR-16330
title: "spec-review of FR-482 schedulability claim node, grammar widening and FR-419 conditions (quire-specification PR 404)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@fcbf602c; spec/objects/interfaces/FR-482-checked-package-schedulability-claim-body.md; spec/functional/analysis/FR-419-analyze-task-set-schedulability.md; spec/test-cases/TC-967-checked-package-schedulability-claim-node.md; spec/test-cases/TC-372-schedulability.md; proposals/quire-v1/shared-grammar.md; proposals/checked-package-v2/schema.json; spec/objects/interfaces/FR-322-checked-package-artifact.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-482
    type: references
  - target: ix://agent-ix/quire-specification/FR-419
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/TC-967
    type: references
---

# Spec review of FR-482 schedulability claim node, grammar widening and FR-419 conditions (quire-specification PR 404)

## Summary

Ticket: STD-375. One spec-review pass over `git diff origin/main...fcbf602c`, checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across all open PR heads, cross-repo names, mergeability and gates.

Gate: `quire validate --scope . 'spec/**/*.md'`: 86 failure lines (origin/main also 86). `cargo test --test checked_package_v2 --test backend_provider_v1`: 23 + 132 passed. QSL conformance `cargo test -p qsl-package --lib checked_v2` with QSPEC_DIR at this head (QSL worktree at #1349 head): 46 passed, 0 failed.

Examined, clean:

- FR-482-AC-1 to AC-3 and SC-01 to SC-19 by hand against the reader order: shape, literals, form fit, task order, unit, then task-set conditions. SC-11 (`kind` response, empty `task`) fails form fit at `task`; SC-12 at `t1`'s `wcet_hi`; SC-13 at `tasks`; SC-15 at `t2`'s `period` (the first time value in body order is `t1`'s period, typed `ms`); SC-16 and SC-17 at the task aggregate; SC-18: `t1` and `t3` share priority 1 and the second of the two is `t3`; the same edit of `CtlEdf`'s node is admitted, as `edf` ignores priorities. All hold. The `Ctl` fixture values (priorities 1, 2, 3, periods 4, 6, 12, wcet 1, 2, 5) match QSL FR-454-AC-1.
- FR-419-AC-10 and TC-372 RT-15 agree with QSL FR-246-AC-3 on the four-refusal order (`t1`, `t2` task-set defects, then `t1`, `t2` for `edf`). The grammar widening (`signed-quantity`, empty task body, `lo` with optional `wcet_hi`) is consistent with the refusals the PR gives to the checker; `schema.json` gains `schedulability` in `ClaimNode.semantic_form`; FR-322 names FR-482 in both lists.
- No deferral language; the inline task set and the first-quantity unit are stated lead defaults, not narrowings. Ids: FR-482, FR-482-AC-1 to AC-3, TC-967 and FR-419-AC-10 appear on no other open PR head or on main.
- The author's QSL conformance result (46 tests) reproduced in this review: `cargo test -p qsl-package --lib checked_v2` with QSPEC_DIR at head fcbf602c passed 46 of 46.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | Both links to FR-142 use `../type-model/FR-142-evaluate-quantities-and-units.md`, which from `spec/objects/interfaces/` resolves to `spec/objects/type-model/`. FR-142 lives at `spec/functional/type-model/`, so the link is broken. The validate count is unchanged because the validator does not follow the link. Fix: `../../functional/type-model/FR-142-evaluate-quantities-and-units.md`. | spec/objects/interfaces/FR-482-checked-package-schedulability-claim-body.md:72, 174 | wrong-requirement |

## Verdict

Mergeable: no defect that blocks the PR.

Conflicts with main: none (clean merge-tree). Conflicts with open #398 (spec/spec.md) and #417 (shared-grammar.md), both textual list/section edits; resolve as main plus both changes. Clean with the other open heads touching schema.json, FR-322 and the grammar.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Fix 006819eb. Both FR-142 links in FR-482 now read `../../functional/type-model/FR-142-evaluate-quantities-and-units.md`, which resolves to the existing file. |

Round 1 verdict at 006819eb: Mergeable: the only change since fcbf602c is the two links (plus the SR file); `quire validate` still gives 86 failure lines and merge-tree with main is clean. The cargo and QSL conformance runs (23 + 132, 46 of 46 at fcbf602c) were not rerun because no vector or schema changed.
