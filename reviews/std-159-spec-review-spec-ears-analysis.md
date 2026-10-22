---
id: SR-13801
title: 'EARS review of QSpec PR #326: FR-419 fixed-priority miss evidence (STD-159)'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@b8cbeabdd; git diff origin/main...b8cbeabdd (PR #326, branch std-159-spec): spec/functional/analysis/FR-419-analyze-task-set-schedulability.md, spec/test-cases/TC-372-schedulability.md; ticket STD-159'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-419
  type: reviews
---

## Summary

Ticket: STD-159. PR: quire-specification#326, head b8cbeabdd. Method: spec-review, spec-ears-analysis. FR-419's requirement statement is unchanged and stays an event-driven EARS statement ("When a package declares a task set and a schedulability or response claim over it, the analyzer SHALL decide ... and SHALL settle it only after its evidence is recomputed"). The PR changes Behavior prose and two criteria. This review checks the new Behavior sentences and AC-2 and AC-7 for one trigger, one response, measurable terms and behaviour-only wording.

AC-2 and AC-7 each name one input, one policy and one observable result with exact values; AC-2's EDF clause is unchanged. Neither uses changelog, status or review language. The Behavior sentences are declarative and measurable.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-419 statement | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:24-27 | examined |
| FR-419 Fixed priority section | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:72-86 | examined |
| FR-419 Miss row | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:108 | examined |
| FR-419-AC-2 | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:129 | examined |
| FR-419-AC-7 | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:134 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "(0 when `D_i <= T_i` and the busy period ends with the first job)" reads as two conditions joined by "and", suggesting `q` could exceed 0 with `D_i <= T_i`. With `D_i <= T_i`, job 0 either misses or completes by `T_i − J_i`, so the busy period then ends with it; the second clause is a consequence. Clearer: "0 when `D_i <= T_i`, whose busy period ends with the first job unless that job misses". The phrase is copied from QSL ADR-026 RT-7. | spec/functional/analysis/FR-419-analyze-task-set-schedulability.md:79-80 |

## Verdict

Not clean, one low wording finding. The statement remains conformant EARS, and both changed criteria are single-trigger, single-result, measurable and free of process language. The parenthetical on job 0 is the only ambiguity in the new text that is about wording; the undefined "keeps the busy period open" condition is recorded in SR-13800 FND-002.
