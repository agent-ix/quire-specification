---
id: SR-16287
title: "Spec-review of FR-095 past-only and mixed-direction correspondence (quire-specification PR 421)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@71f21701; spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md; spec/test-cases/TC-117-native-tl-correspondence.md"
review_set: subset
date: 2026-10-23
relationships:
  - target: ix://agent-ix/quire-specification/FR-095
    type: references
  - target: ix://agent-ix/quire-specification/TC-117
    type: references
---

# Spec-review of FR-095 past-only and mixed-direction correspondence (quire-specification PR 421)

## Summary

Ticket: STD-427 (QSpec half of the past-row change; QSL half quire-spec-language#1370, SR-16268). Head 71f21701. origin/main
is an ancestor of the head.

Gates run by the reviewer: `quire validate --scope . 'spec/**/*.md'` gives exactly 86 failure lines.
`cargo test --test checked_package_v2 --test backend_provider_v1` passes 24 + 134. No vector or fixture changed,
so no QSL conformance run is needed. QSL #1370 at ae1210a5 passes `make check-spec-validation` with QSPEC_DIR
at this head.

Cross-spec: the past-only row, the mixed row and the incomplete history cutoff agree with QSL FR-045's table,
AC-2 and AC-8, and with tl-syntax FR-011 (on tl-syntax origin/main, the past profile rejects future and mixed
nodes) and FR-012. The PR merges clean with every open QSpec PR except stale #393 and #398 (spec.md), and no
other open PR touches FR-095 or TC-117.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | [other] On main the past row read 'Any bounded past operator with origin-complete history ... Supported'. #421 narrows it to past-only and adds a permanent refusal for a formula with both past and future operators ('No TL target'). The reason holds: tl-syntax FR-011-AC-4 rejects every future or mixed-time node under `tl-syntax.past-operators/v1`, so the main row could not be met. It is still a permanent refusal that removes a correspondence the spec promised, and no STD ticket asks for a TL profile that evaluates mixed graphs, so under the standing rule it blocks. Fix: an owner ruling that a mixed formula has no TL correspondence, or a new TL ticket for a mixed-direction profile, with FR-095 stating the correspondence once it exists. This is the reviewer's flag under the binding rule, not an owner answer. | spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md:50-51; spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md:101 | missing-requirement |
| FND-002 | medium | [coverage] The past row and AC-3 say a history-cutoff boundary is 'incomplete' and give no cause. QSL FR-045 (#1370) returns `incomplete` with cause `history-not-origin-complete`, which appears in no QSpec cause vocabulary (FR-272, FR-331 incomplete causes). The owner spec should name the cause, so that the native and TL halves report the same one. Fix: name the cause in FR-095 and add it to the vocabulary. | spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md:50; spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md:101 | missing-requirement |
| FND-003 | low | [trace] FR-095's Inputs cite the closed GitHub issue quire-contract-ir#63 as what governs total-Boolean predicate projection. With #1370, QSL FR-045 owns the projection (its 'Formula and valuation construction'). Fix: cite QSL FR-045, not a closed issue. | spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md:26-29 | wrong-requirement |

## Verdict

Not mergeable under the standing rule (FND-001, a high flagged with low confidence, so it needs an owner ruling or a TL ticket). FND-002 should be fixed with QSL #1370 FND-010. Apart from these, the change is consistent with tl-syntax and QSL FR-045.
