---
id: SR-13651
title: 'Failure-domain review of QSpec PR #314: required over parameter (STD-186)'
type: SpecReview
analysis: failure-domain
scope: 'agent-ix/quire-specification@77228570f; PR #314: FR-361 over binding, FR-370 argument 1, FR-361-AC-8 context; ticket STD-186'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-361
  type: reviews
---

## Summary

Ticket: STD-186. PR quire-specification#314, head 77228570f92bd4b83ca2dd51e365fd47e8c5d16e.

Failure-domain lens on option (b): what a model-wide claim becomes when it must name an `over` parameter.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-361 over binding paragraph | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| FR-361-AC-8 | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Under (b) a claim that reads no one object must still name `over (x: T)`, so its decidability depends on the universe of `T`: with no conforming object (or an empty universe) FR-361-AC-8 settles it `inconclusive`/`no-instance` before exploring, although the formula never reads `x`; with N objects it is checked N times and a refutation is reported against an arbitrary instance. The new FR-361 text and the PR's cost analysis omit this. Fix: state the consequence in FR-361 (an author must pick a type with at least one object), so the owner decision between (a) and (b) weighs it. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:128-131 |

## Verdict

One medium failure-domain gap; it informs the owner-level (a)/(b) choice and does not block if recorded.
