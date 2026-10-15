---
id: SR-304
title: "Evidence review of native temporal lifecycle correspondence"
type: SpecReview
analysis: evidence
scope: "FR-090 through FR-095, NFR-040, TM-006, TC-110 through TC-118 and IT-040/041"
review_set: all
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/TM-006, type: reviews }
---
## Summary

The planned methods now match the kind of claim: independent finite truth-table
and differential oracles for semantics, integration cases for identity and
lifecycle handoffs, property generation for checked bounds, and mutation tests
for forbidden fallback or axis substitution. None is represented as executed
implementation evidence.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Resolved: the former unbounded "every exhaustive" claim now declares finite AST, interval, proposition, trace, anchor and lifecycle limits plus population/cardinality reporting. | FR-095-AC-1; TC-117 | correct-requirement-no-evidence |
| FND-002 | high | Resolved: TC-116 and TC-117 now mutate both closure axes, settlement basis, exact support and activation/execution states independently. | TC-116; TC-117; IT-040/041 | correct-requirement-no-evidence |
| FND-003 | medium | Resolved: TC-115 now falsifies progress monotonicity with rollback and same-revision conflict cases. | FR-094-AC-8; TC-115 | correct-requirement-no-evidence |
| FND-004 | high | Open F evidence gate: TC-145/146 and IT-051 do not yet prove preservation of the revised B/E result dimensions. | TC-145/146; IT-051 | correct-requirement-no-evidence |
| FND-005 | low | Tool prerequisite: Quire 0.31.0 strict validation ran successfully, but installed Quoin 0.23.1 could not parse the selected Quire version probe and therefore emitted no method-adviser result. This review does not substitute a fabricated adviser pass. | local tool output; TM-006 | correct-requirement-no-evidence |

## Method check

| Obligation kind | Planned method | Reason |
| --- | --- | --- |
| Bounded operator meaning | Finite exhaustive property oracle and boundary mutation | Directly compares every declared member to an independent definition. |
| Native/TL correspondence | Rust integration differential plus identity/result-axis mutation | Tests semantic equality and refuses lost provenance/lifecycle dimensions. |
| Progress/closure | Stateful integration transition test | Exercises monotonic and contradictory histories, not only snapshots. |
| Resource bounds | Property and mutation tests | Detects overflow/exhaustion Boolean fallbacks. |
| Definition adoption | Exact-byte review and checksum census | Prevents a changed candidate from aliasing an old revision. |
