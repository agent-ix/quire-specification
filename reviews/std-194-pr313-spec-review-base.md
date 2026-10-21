---
id: SR-13649
title: 'Spec review of QSpec PR #313: invalid-fairness-kinds registration cause (STD-194)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@9f0d0c865; PR #313 diff origin/main...HEAD: FR-368, FR-272, FR-290, TC-301, TC-431, native-diagnostics.md, backend-provider-v1/vectors.json; ticket STD-194'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-368
  type: reviews
- target: ix://agent-ix/quire-specification/FR-272
  type: reviews
- target: ix://agent-ix/quire-specification/FR-290
  type: reviews
- target: ix://agent-ix/quire-specification/TC-301
  type: reviews
- target: ix://agent-ix/quire-specification/TC-431
  type: reviews
---

## Summary

Ticket: STD-194. PR quire-specification#313, head 9f0d0c86581ca8eb93d5f6d270180349f33c3cc1.

One review pass with the base lens and the EARS, integrity and failure-domain lenses folded in (proportionate; no separate sub-analysis files). Gates run in an own clone: `cargo test --test checked_package_v2 --test backend_provider_v1` pass (12 + 113); `quire validate --scope . 'spec/**/*.md'` gives 134 failures as on main, and the FR-272/FR-290 messages are byte-identical to main's (pre-existing), the other touched files clean. Discrimination: each of the four new adverse vectors was shown to fail `tc_431_adverse_vectors_are_refused` when its schema constraint was removed (drop `type: array` -> not-a-list vector, drop the `enum` -> unknown-label vector, drop `uniqueItems` -> duplicate-label vector, drop the `else` branch -> non-temporal vector); the absent case is the pre-existing `manifest-temporal-without-fairness-kinds` vector (fails when the `then` branch is emptied). The single cause was checked against QSL FR-134 (qsl-lane2 origin/main 5c4a96692): QSL's Remaining work anticipates a QSpec rename and asks whether one cause like `invalid-domains` is preferred, so the planner default (single cause, QSL follows) is consistent; QSL FR-134 AC-3..AC-5 and QSL-877 must move to `invalid-fairness-kinds` with the defect retained. AC ids FR-368-AC-8..10 and BP-22 collide with nothing on main or in the open PRs. EARS: the new rule is a state-driven SHALL with a named subject; no EARS finding.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-368 invalid-fairness-kinds rule | spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md | examined |
| FR-368-AC-8 | spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md | examined |
| FR-368-AC-9 | spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md | examined |
| FR-368-AC-10 | spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md | examined |
| FR-272 invalid_capability row | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| native-diagnostics invalid_capability row | proposals/quire-v1/definitions/native-diagnostics.md | examined |
| FR-290 registration causes | spec/objects/protocol/FR-290-protocol-claim-kind.md | examined |
| TC-301 step 5 | spec/test-cases/TC-301-temporal-negotiation-and-fairness-kinds.md | examined |
| TC-301 expected 5 | spec/test-cases/TC-301-temporal-negotiation-and-fairness-kinds.md | examined |
| TC-431 BP-22 | spec/test-cases/TC-431-backend-provider-wire-vectors.md | examined |
| vectors.json fairness adverse vectors | proposals/backend-provider-v1/vectors.json | examined |
| QSL FR-134 Manifest fairness_kinds admission | quire-spec-language:spec/functional/FR-134-advertise-the-fairness-kinds-en-1-decides.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The branch does not merge into current main (fd7a694, #312 merged): `git merge` conflicts in proposals/backend-provider-v1/vectors.json (both append to the end of `adverse`) and spec/test-cases/TC-431-backend-provider-wire-vectors.md (BP-21 from #312 sits where BP-22 is appended). BP-22 itself is still free. Fix: rebase on main, keep BP-21 then BP-22 and both vector groups, and rerun `cargo test --test backend_provider_v1` and quire validate on the result. | spec/test-cases/TC-431-backend-provider-wire-vectors.md:37 |
| FND-002 | medium | FR-368-AC-10 is verified through TC-431 BP-22, but no test carries its tag: `tc_431_adverse_vectors_are_refused` (tests/backend_provider_v1.rs:292) traces only FR-331-AC-19/20/23/28, and `grep FR-368 tests/` is empty, so the matrix shows AC-10 uncovered although the four vectors do run. TC-431's frontmatter verifies only FR-331. Fix: add `#[trace("TC-431", "FR-368-AC-10")]` to that test (or a dedicated test over the five fairness vectors) and a `verifies` FR-368 relationship on TC-431. | spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md:155 |
| FND-003 | medium | FR-368 retains the offending value only 'for the last two' defects (`unknown-label`, `duplicate-label`), while the FR-272 row and native-diagnostics.md say the cause retains 'the defect (...) with the offending value' for all four, which reads as also retaining the supplied non-list value for `not-a-list`. QSL FR-134 names 'the member as supplied, such as null' for a non-list. Two implementers would retain different fields. Fix: pick one rule (retaining the supplied member for `not-a-list` matches QSL) and state it identically in FR-368, FR-272 and native-diagnostics. | spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md:119-120 |
| FND-004 | low | BP-22 lists 'fairness_kinds removed' and says '[weak, strong] stays admitted', but this PR adds no removal vector and no [weak, strong] positive: the absent case is the pre-existing `manifest-temporal-without-fairness-kinds` vector and the `temporal-request` positive carries [strong, weak]. Fix: name the existing vector and the positive's actual value in the row. | spec/test-cases/TC-431-backend-provider-wire-vectors.md:37 |
| FND-005 | low | The rule orders the four fairness defects among themselves but not against the other registration causes on the same registration (`unknown-mode`, `invalid-domains`, ...). QSL FR-134-AC-6 reports the pair or `domains` cause first. QSpec FR-290 has no cross-cause order for any registration cause, so this is an existing gap the new cause inherits. Fix: state that `invalid-fairness-kinds` is judged after the pair and `domains` checks, or record the gap for FR-290. | spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md:114-129 |
| FND-006 | low | The diff removes the final newline of proposals/backend-provider-v1/vectors.json ('\ No newline at end of file'). Fix: restore it. | proposals/backend-provider-v1/vectors.json:3441 |

## Verdict

Not mergeable as is: FND-001 (does not merge into main) and FND-002 (AC-10 untagged) block. FND-003 should be settled in this PR because QSL FR-134 is about to copy the retention rule. The vectors discriminate (mutation evidence in Summary), the single cause is consistent with QSL FR-134's open question, and the non-temporal 'not read' rule matches QSL.
