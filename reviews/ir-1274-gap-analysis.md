---
id: SR-20261103160102
title: 'Gap analysis of QSpec PR #612: TC-970 / FR-336-AC-7 test (IR-1274)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@09aa1225; tests/conformance_corpus_v1.rs, TC-970, FR-336-AC-7; ticket IR-1274'
review_set: subset
date: '2026-11-03'
relationships:
- target: ix://agent-ix/quire-specification/FR-336
  type: reviews
---

## Summary

Ticket: IR-1274. quire matrix shows FR-336-AC-7 tagged only by the new test replay_request_carries_the_proved_checked_package. The test is non-vacuous for its package claim but exercises none of the other AC-7 behaviours.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| replay_request_carries_the_proved_checked_package | tests/conformance_corpus_v1.rs:211 | examined |
| FR-336-AC-7 | spec/objects/interfaces/FR-336-conformance-corpus.md:250 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-336-AC-7 is marked tagged by a test that covers only the package member | tests/conformance_corpus_v1.rs:211 |

## Verdict

Not blocking this PR; pre-existing AC-7 mutation checks and AC-8 (untagged) remain uncovered.
