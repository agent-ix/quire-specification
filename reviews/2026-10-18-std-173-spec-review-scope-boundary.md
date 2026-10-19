---
id: SR-12824
title: "Scope-boundary review of the FR-331 no-instance wire cause"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-specification; PR #280 (STD-173) at d9780f2: FR-331, FR-361, FR-391, FR-399 (unchanged), FR-395 (unchanged), TC-294, TC-334, TC-431; QSL main 49bcf9a FR-125, FR-171..FR-174"
review_set: subset
review_date: "2026-10-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-399, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-361, type: reviews }
---
## Summary

Ticket: STD-173. This review checks the author's scope decisions.

- **(3) Zero-instance behaviour in FR-361 with AC-8, not a new FR.** Accepted.
  FR-361 owns the `over` binding and already holds the sibling
  `no-initial-state` rule in AC-7.
- **(4) TC-431 as the only test of FR-331-AC-24.** Accepted for a
  wire-spelling AC, since the semantics are covered by TC-294 step 7
  (FR-361-AC-8) and TC-334 PR-13 (FR-391-AC-5). TC-431 does not yet cite
  AC-24, which is base FND-001.
- **(2) ADR-018 QS-6.** QS-6 lists no causes. The ADR's V-6 enumeration is a
  QSL follow-up (dependency FND-002).
- **(1) FR-399's hyperproperty row left unchanged.** This leaves a real but
  pre-existing gap; see FND-001.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | Hyperproperty items can have zero instances, and neither repository says how they settle. QSpec FR-395 gives `behaviours (v: V::T)` "one instance per key of that population's universe" (product across aliases), and QSL FR-172 checks to one instance per key. An empty universe therefore yields zero instances. FR-399's inconclusive row lists `no-initial-state` and not `no-instance`, and QSL FR-125's Instancing rule covers only temporal clauses and state-graph claims. Leaving FR-399 out of STD-173 is defensible scope, because QSL defines `NoInstance` only for FR-125 items. But a zero-instance hyper item has no stated outcome, and an implementation could report it as vacuously holding. Recommend a follow-up ticket covering FR-399 (QSpec) and FR-172 (QSL). | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:81; spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md:63-69; qsl-lane2 spec/functional/FR-172-check-hyper-and-relation-clauses-over-model-subjects.md:90-93 | missing-requirement |

## Verdict

The scope decisions hold. FND-001 should go to a follow-up ticket and does
not block this PR.
