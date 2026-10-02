---
id: SR-1171
title: "Integrity review of quire-specification PR #177: FR-377 routes no per-step obligation for a history-field declaration"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@fa24716c9f3effcff5f4368850bb637cb1718da2; PR #177 diff against origin/main; FR-377, TC-312; checked against quire-spec-language FR-146 on main"
review_set: subset
---
# Integrity review of PR #177

## Summary

Ticket: STD-133. This PR is the QSpec half of quire-spec-language SR-1150
FND-006: FR-377 now routes no per-step simulation obligation for a
declaration with a history field, as QSL FR-146 writes no
`StepSimulationRecord` for one.

Examined:
- FR-377 Engines prose: "A per-step simulation engine discharges the safety
  half of a declaration with no hidden field, no history field and no `any`
  row", and the added reason that a per-step obligation cannot see history
  values. This matches QSL FR-146 "Which records" (no hidden field, no
  history field and no `any` row; otherwise no record).
- FR-377-AC-7, which now names the history field.
- TC-312 case 7. Its history row
  `history Impl::Counter.commits: Int[0, 4] = 0 { on Impl::Counter::commitA:
  pre(self.commits) + 1; on Impl::Counter::commitB: pre(self.commits) + 1; }`
  is written exactly as TC-314 case 1 writes it, and fits FR-375's history
  row rules: a new field, a bounded type, an initial literal, and one update
  per operation. The expected verdict is right. Each commit raises `value`
  by one and `value` stops at 2, so `commits` equals `value` at every
  reachable state and never leaves `Int[0, 4]`. The history values are a
  function of the concrete state, so the explored state count and every
  verdict are case 1's (FR-378-AC-2), and the item settles `proved`.

`quire validate` on both changed files exits 0, and `cargo test --locked`
passes (82 tests, 0 failed).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-377-AC-7 says a per-step obligation "is never routed for a declaration with a hidden field or a history field, or for an `any` row". The Engines prose, and QSL FR-146, put the `any` condition on the declaration: one `any` row means no per-step obligation for any row. "For an `any` row" can be read as suppressing only that row's own obligation, which is the reading SR-1150 FND-006 asked the two specs to settle. Write "for a declaration with a hidden field, a history field or an `any` row". | spec/functional/analysis/FR-377-decide-model-refinement.md:161 |

## Verdict

The history-field rule now agrees with QSL FR-146, and TC-312 case 7 is
well-formed with the right expected verdict. One low wording finding
remains on AC-7's `any` clause. Mergeable once it is fixed.
