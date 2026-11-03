---
id: SR-20261103160101
title: 'Spec review of QSpec PR #612: PL-R3 vector role, ReplayRequest package member (IR-1274)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@09aa1225; git diff origin/main...HEAD (PR #612): checked-term-data-vectors.json PL-R3-leaf-reference, conformance-corpus-v1 schema.json/README/fixtures/replay-request.json, FR-336 AC-7, TC-970, tests/conformance_corpus_v1.rs; ticket IR-1274'
review_set: subset
date: '2026-11-03'
relationships:
- target: ix://agent-ix/quire-specification/FR-336
  type: reviews
---

## Summary

Ticket: IR-1274 (also QSL-1592 context). PR: quire-specification#612. The PL-R3-leaf-reference occurrence role is corrected and matches its siblings and the FR-322 carrier text. ReplayRequest gains required package, 14 members match the FR-336 table. The fixture's node ids do not resolve in its embedded package. Tests: conformance_corpus_v1 3 pass, checked_package_v2 pass, quire validate on FR-336 and TC-970 clean.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| PL-R3-leaf-reference | proposals/checked-package-v2/checked-term-data-vectors.json:2913 | examined |
| FR-336-AC-7 | spec/objects/interfaces/FR-336-conformance-corpus.md:250 | examined |
| TC-970 procedure | spec/test-cases/TC-970-replay-request-corpus-adapter-is-closed.md:22 | examined |
| ReplayRequest schema | proposals/conformance-corpus-v1/schema.json:1113 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | replay-request.json embeds a package that its own node ids and dependencies do not match | proposals/conformance-corpus-v1/fixtures/replay-request.json |
| FND-002 | low | TC-970 and AC-7 do not state the package refusal the new test asserts; TC-970 sentence missing a comma | spec/test-cases/TC-970-replay-request-corpus-adapter-is-closed.md:24 |

## Verdict

Mergeable with FND-001 acknowledged or fixed.
