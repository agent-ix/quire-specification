---
id: SR-12896
title: "Base review of PR #282: FR-375 refinement fairness rows admit weak or strong"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@88bf7df; FR-375 Surface syntax, Fairness rows, Checked-package node, AC-6, AC-7, AC-10; TC-310 mutations 8, 11 and 12 and Expected Results"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-310
    type: reviews
---

## Summary

Ticket: STD-178. PR quire-specification#282 (head 88bf7df, diff
`origin/main...HEAD`, base ad2b37e). The fairness row grammar becomes
`fair ( weak | strong ) [ each | whole ]`, the node body's `assume`/`ensure`
entries gain a `kind` field, AC-10 is new and TC-310 gains mutation 11 (old
step 11 is step 12). This base review covers ID format, AC to TC coverage
and link integrity. `quire validate` over both changed files exits 0.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-10 says a `fair strong` row is admitted "with or without `each` or `whole`", but TC-310 mutation 11 writes only `each` (the `ensure` row) and no granularity (`retryA`). An explicit `fair strong whole` row is never exercised, so a parser that mis-orders `strong whole` passes. | spec/functional/analysis/FR-375-declare-model-refinement.md:210; spec/test-cases/TC-310-refinement-declaration-admission.md:57-61 |
| FND-002 | low | FR-375 now gives fairness rows "the meaning of FR-362", but the frontmatter has no FR-362 relationship; the dependency is only a body link, so blast-radius queries on FR-362 miss FR-375. | spec/functional/analysis/FR-375-declare-model-refinement.md:5-19,80 |

## Verdict

Mergeable. No high or medium finding in the base checklist. AC-10 is
falsifiable (kind recorded, node ids differ, kindless and `always` refuse at
the token after `fair`), TC-310 mutation 11 traces it, and the old step 11 is
renumbered consistently (the Expected Results say "step 12"). The weak and
strong forms get different node ids both by AC-7 (any row change changes the
id) and by AC-10 directly.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 847b7a2 adds `assume fair strong whole Impl::Counter::retryA;` and the unmarked strong row to TC-310 mutation 11. The expected results give both kind `strong`, granularity `whole`, and the same entry. |
| FND-002 | fixed | 847b7a2 adds an `ix://agent-ix/quire-specification/FR-362` `references` relationship to FR-375's frontmatter. |
