---
id: SR-13645
title: "QSpec PR #311 (STD-217 items 3-4) FR-375 steps order and FR-322 generated role"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@0bc76ae117f9b5be4dcaa2b5c868218fc06bf64d vs main d080afa; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-310-refinement-declaration-admission.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-310
    type: reviews
---

## Summary

Ticket: STD-217 (PR #311, head 0bc76ae117f9b5be4dcaa2b5c868218fc06bf64d). Partial PR: STD-217 items 3 and 4 only; items 1, 2 and 5 are left open on purpose and are not reviewed here. Item 4 adds a tie-break by alias-free class spelling to FR-375's `steps` order; item 3 defines the `generated` occurrence role (planner default, flagged in the PR body). No schema or vector changes.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-AC-19 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375 steps order | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-322-AC-84 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 generated role | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-310 mutation 23 | spec/test-cases/TC-310-refinement-declaration-admission.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The tie-break uses "the row's alias-free class spelling (FR-432)", but FR-432 and FR-375 define alias-free spellings only for abstract classes (`<A>::` removed); `steps` rows are concrete-side left sides spelled with `<C>`. Say that the concrete alias `<C>::` is removed | spec/functional/analysis/FR-375-declare-model-refinement.md:399-403 |

## Verdict

Low only in this lens; see SR-13647 and SR-13648 for the blocking findings.

## Dispositions

Disposition pass 1, against head 27b08c16fb75a08b740aed5ca1e3b1727679fb2c.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | verified at 27b08c16fb75 |
