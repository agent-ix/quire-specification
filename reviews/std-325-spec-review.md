---
id: SR-16185
title: "Spec review of quire-specification PR #385: STD-325, timed refinement, dense-time reductions and strong fairness over time"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@0a34b1b18; PR #385 (git diff origin/main...0a34b1b18): spec/functional/analysis/FR-377-decide-model-refinement.md, spec/functional/analysis/FR-382-select-state-space-reductions.md, spec/functional/temporal/FR-416-evaluate-timed-temporal-claims.md, spec/test-cases/TC-963-timed-refinement-reductions-and-strong-fairness.md"
review_set: subset
evaluated_revision: "0a34b1b18"
relationships:
  - target: ix://agent-ix/quire-specification/FR-377
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-382
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-416
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-963
    type: reviews
---
# SR-16185: Spec review of quire-specification PR #385: STD-325, timed refinement, dense-time reductions and strong fairness over time

## Summary

Ticket: STD-325. Reviewed `git diff origin/main...HEAD` at 0a34b1b18 against QSL FR-430 at main 2c5d82d37 (Timed refinement, Reductions over a dense-time subject, Strong fairness over time, FR-430-AC-1..4). Each rule is QSL's, restated in QSpec terms ('the symbolic provider' for EN-6, 'the step function' for `check_step`) without QSL type names; QSL FR-144's routing rule is correctly left out. FR-377-AC-14/15, FR-382-AC-7 and FR-416-AC-9 carry FR-430-AC-1..4's vectors unchanged. Checked AC-15 by hand: under `when self.trap { x <= 0 }`, `enter` resets `x`, so the only `undo` pre-states have `x = 0` with time frozen, and after `undo` no operation is enabled (`send` and `enter` need `x >= 1`, `undo` needs `sent`), so no time-divergent continuation exists and the model time-locks, as the AC says. In AC-9's Blink, `fire` is enabled only inside delays, so weak fairness refutes and strong fairness holds. TC-963 is free in main, in every open QSpec PR and has no QSL collision (separate id space). Gates at the head: validate 86 failure lines (main 86); cargo 23 + 119 passed; no test-parsed file changes. origin/main is an ancestor; no textual conflict with the other six PRs. Lead defaults are flagged.

Method: quoin spec-review (base), with integrity, EARS, criterion-strength (by hand: no Jev client) and failure-domain checks folded in; code-review of the `tests/` change where present is folded in here. Gap-analysis does not apply (spec, vectors and trace tags only).

Units examined:

- `FR-377 Timed concrete subject` (examined): spec/functional/analysis/FR-377-decide-model-refinement.md
- `FR-377-AC-14` (examined): spec/functional/analysis/FR-377-decide-model-refinement.md
- `FR-377-AC-15` (examined): spec/functional/analysis/FR-377-decide-model-refinement.md
- `FR-382 Dense-time subjects` (examined): spec/functional/analysis/FR-382-select-state-space-reductions.md
- `FR-382-AC-7` (examined): spec/functional/analysis/FR-382-select-state-space-reductions.md
- `FR-416 Strong fairness over time` (examined): spec/functional/temporal/FR-416-evaluate-timed-temporal-claims.md
- `FR-416-AC-9` (examined): spec/functional/temporal/FR-416-evaluate-timed-temporal-claims.md
- `TC-963` (examined): spec/test-cases/TC-963-timed-refinement-reductions-and-strong-fairness.md

## Verdict

**MERGEABLE**, with one low finding to fix or accept.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-382 says negotiation names 'the first such reduction in the order partial-order, constraint', but AC-7 and TC-963 TR-06 select partial-order reduction and a state constraint only one at a time, so the order rule has no vector that can fail. Add the item selecting both, naming partial-order reduction. (QSL FR-430-AC-3 has the same gap.) | spec/functional/analysis/FR-382-select-state-space-reductions.md:144-149, spec/functional/analysis/FR-382-select-state-space-reductions.md:163, spec/test-cases/TC-963-timed-refinement-reductions-and-strong-fairness.md |
