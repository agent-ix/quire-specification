---
id: SR-13648
title: "Failure-domain review of QSpec PR #311 (STD-217) total order of protocol-subject steps"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@0bc76ae117f9b5be4dcaa2b5c868218fc06bf64d vs main d080afa; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/test-cases/TC-310-refinement-declaration-admission.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-310
    type: reviews
---

## Summary

Ticket: STD-217 (PR #311, head 0bc76ae117f9b5be4dcaa2b5c868218fc06bf64d). Checks whether the new rule makes the `steps` order total for every row kind FR-432 admits.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-AC-19 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375 steps order | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| TC-310 mutation 23 | spec/test-cases/TC-310-refinement-declaration-admission.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | A `memory` row names no declaration, so it has no primary key ("node-key digest of its first named declaration"). The new text orders memory rows among themselves by spelling but does not place them relative to rows that do carry a digest, so the order is still not total. AC-19's fixture has one memory row, so this is untested. State where rows with no declaration sort (for example before every digest-keyed row) and add a second memory kind to AC-19 | spec/functional/analysis/FR-375-declare-model-refinement.md:398-403 |
| FND-002 | low | The tie list names duplicate/lose, spawn/retire and memory rows, but `fork` and `join` of one `parallel` node and a template's `cattempt` row and its `cend` row also share their first named declaration. The general rule covers them, but the parenthetical reads as exhaustive and AC-19 does not exercise them. Add them to the list or mark it as examples, and add one pair to AC-19 | spec/functional/analysis/FR-375-declare-model-refinement.md:435 |
| FND-003 | low | "written in each of two orders" does not require the two orders to differ in the relative order of each tied pair; two orders that both put `duplicate` before `lose` pass under a stable sort with no tie-break. Name the second order as the reverse of the first | spec/test-cases/TC-310-refinement-declaration-admission.md:158 |

## Verdict

Medium finding FND-001; fix in this PR.

## Dispositions

Disposition pass 1, against head 27b08c16fb75a08b740aed5ca1e3b1727679fb2c.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | verified at 27b08c16fb75 |
| FND-002 | fixed | verified at 27b08c16fb75 |
| FND-003 | fixed | verified at 27b08c16fb75 |
