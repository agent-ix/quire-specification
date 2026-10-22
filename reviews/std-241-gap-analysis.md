---
id: SR-14098
title: "gap analysis of STD-241 new ACs against TC rows (QSpec PR #336)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@7a168cec646cc13f12c56cb48854e3aaedee6351 (diff against std-ra-247-qspec e69cade8); spec/functional/memory/FR-437-explore-release-acquire-memory.md, spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md, spec/test-cases/TC-394-memory-counterexample-wire-and-replay.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-437
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-439
    type: reviews
---
# SR-14098: gap analysis of STD-241 (QSpec PR #336)

## Summary

Ticket: STD-241. Plan completion: not assessed. This is a manual mapping.

- FR-437-AC-14 (new) maps to TC-394 MK-09. TC-394 now also verifies FR-437.
- FR-439-AC-10 (new) maps to TC-394 MK-10.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-437-AC-14's last clause (replay refuses a carried view whose `frontier` count differs, or an `sc-graph` whose edge set differs) is not in MK-09, the row that cites AC-14. It is covered only by MK-10, which cites FR-439-AC-10, so the clause either duplicates FR-439-AC-10 or lacks a row of its own. | spec/functional/memory/FR-437-explore-release-acquire-memory.md:229 |

## Verdict

Every new AC has a TC row. One clause is duplicated across two ACs.

## Dispositions

Round 1. Against QSpec #336 head 7235b5c (fix commit 7235b5c).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-437-AC-14 now ends at the encoded state: "… `left`'s `cur` view has `at` mapping `x` to depth `"0"` and `y` to depth `"0"` and `frontier` mapping `left` to `"1"`, and `right`'s has an empty `frontier`." The replay refusals live only in FR-439-AC-10 and MK-10. |
