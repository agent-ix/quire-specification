---
id: SR-617006
title: 'Gap analysis of PR #618: FR-370-AC-17/AC-18 against TC-303 step 15 (QSL-1644)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@9d8696de; PR #618: FR-370-AC-17, FR-370-AC-18, FR-370 "Window aggregate atom" step 5, TC-303 step 15, temporal-aggregate-vectors.json, tests/checked_package_v2.rs'
review_set: subset
date: '2026-11-04'
relationships:
- target: ix://agent-ix/quire-specification/FR-370
  type: reviews
---

## Summary

Ticket: QSL-1644. Plan completion: not assessed. This is a manual AC-to-vector mapping, scoped to the PR diff.

FR-370-AC-17: the test asserts the catalog row (temporal_formula, `["aggregate"]`, `temporal`, member null, no laws) and the schema enum. The positives are count+filter, sum, min, max, fraction+predicate and quantile+q under positions (AG-01..06), plus quantile over duration under timestamped-event (AG-07). The test asserts that the six kinds are all present.

FR-370-AC-18: every enumerated clause has its own vector. Infinite-trace is AR-01, unknown member AR-02, non-binding member AR-03, duplicate AR-04 and misordered AR-05. Missing kind, window_kind and comparison are AR-06..08, and quantile without q is AR-09. Then q on sum AR-10, value on count AR-11, median AR-12, q=1 AR-14, negative bound AR-15 and min_count 0 on max AR-16. Positions under timestamped-event is AR-17, duration under event-position AR-18, and the two unit mismatches AR-19 and AR-20. Non-Boolean filter is AR-21, rational threshold on count AR-22 and past[5,2] AR-23. min_count 0 on count and sum is admitted by AG-08 and AG-09. AR-13 (kind not text) is extra.

Both ACs are tagged by `#[trace("TC-303", "FR-370-AC-17", "FR-370-AC-18")]`.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-370-AC-17 | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:415 | examined |
| FR-370-AC-18 | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:416 | examined |
| FR-370 Window aggregate atom step 5 | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:224 | examined |
| TC-303 step 15 | spec/test-cases/TC-303-checked-package-temporal-clause-body.md:73 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-370 steps 4-5 state refusals that no AC names and no vector exercises. These are: `window_kind` or `comparison` text outside its set (`invalid-value`); `wrong-value-kind` on `q`, `window_kind`, `comparison`, `lower`, `upper` and `min_count`; a non-numeric `value` and a non-Boolean `predicate` (`type-mismatch`); a missing `value` on sum and a missing `predicate` on fraction; a missing `lower`, `upper`, `min_count` or `threshold`; a negative `min_count`; and an integer threshold on fraction. quire-contract-ir's admission will implement these with no conformance vector. Fix: name them in FR-370-AC-18 and add about 8 reader cases, one per distinct code and locus (for example `comparison "!="`, `value` -> `e.ok`, sum without `value`, `upper` as text, `min_count "-1"`, `threshold` integer on fraction) | proposals/checked-package-v2/temporal-aggregate-vectors.json |

## Verdict

AC-17 and AC-18 are fully backed by vectors and a tagged test. FND-001 is a coverage gap in stated reader behaviour beyond the ACs. It is recommended in this PR because the vectors are the IR admission's conformance input, but it does not block on its own.
