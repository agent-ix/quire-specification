---
id: SR-13652
title: 'Code review of QSpec PR #314: tc_303 over-argument schema test (STD-186)'
type: SpecReview
analysis: code-review
scope: agent-ix/quire-specification@77228570f; tests/checked_package_v2.rs tc_303_a_clause_application_has_no_absent_or_null_over_argument; proposals/checked-package-v2/schema.json TemporalClauseBody; ticket STD-186
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-370
  type: reviews
---

## Summary

Ticket: STD-186. PR quire-specification#314, head 77228570f92bd4b83ca2dd51e365fd47e8c5d16e.

The new test was mutation-checked against proposals/checked-package-v2/schema.json `TemporalClauseBody` (line 48). Mutation A: `prefixItems[0]` = anyOf(ReferenceTermShape, null) -> test still passes. Mutation B: `prefixItems[0]` = `{}` (argument 1 unconstrained) -> test still passes. A probe test that puts the clock text literal into argument 1 is refused by the real schema and admitted under mutation B, so that assertion would discriminate. Error dump under mutation A: the nulled package is refused at `/semantic_graph/nodes/3` by the generic node/term shape, not at the over argument.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| tc_303_a_clause_application_has_no_absent_or_null_over_argument | tests/checked_package_v2.rs | examined |
| TemporalClauseBody | proposals/checked-package-v2/schema.json | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The two assertions fail for reasons unrelated to the `over` argument: removal fails `minItems: 6` and the positional shift of the remaining five arguments, and `null` fails the generic term shape at node level. With `prefixItems[0]` replaced by `{}` or made nullable, the test still passes, so it does not guard 'argument 1 is always a reference'. Fix: also replace argument 1 with a non-reference term (for example the clause's own clock text literal, or an empty aggregate) and assert refusal located under `.../body/arguments/0`; that probe is refused by the real schema and admitted under the `{}` mutation. | tests/checked_package_v2.rs:14436-14452 |

## Verdict

Fix FND-001 before merge: as written the test is a regression lock on the term shape and arity, not on FR-370-AC-13's argument-1 rule.

## Dispositions

Round 1, head 13bd4aa05.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed in 13bd4aa05: Test adds a clock-literal-in-argument-1 probe. Re-mutated: prefixItems[0] = {} and prefixItems[0] = anyOf(reference, null, literal) both now fail the test (panic at tests/checked_package_v2.rs:14464). |
