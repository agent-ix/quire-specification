---
id: SR-13640
title: "Code review of QSpec PR #309 (STD-213) tc_431_a_subject_bound_leaves_the_extent_unbounded"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@d22f6cab318ce014d8273fa2c2f6cfa02b52e7b9 vs main d080afa; tests/backend_provider_v1.rs; context proposals/backend-provider-v1/vectors.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---

## Summary

Ticket: STD-213 (PR #309, head d22f6cab318ce014d8273fa2c2f6cfa02b52e7b9). One new test, traced `TC-431`/`FR-331-AC-32`. Mutation run on a copy of the vectors file: changing the subject bound's `kind` to `integer`, adding `extent.bounds`, or setting `extent.classification` to `bounded` each fail the test (good). Changing the extent domain kind `infinite-trace` to `integer`, setting `finite_bound_available` to `true`, or changing the bound's `maximum` all leave it green: the schema admits those documents and the test does not assert them.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| tc_431_a_subject_bound_leaves_the_extent_unbounded | tests/backend_provider_v1.rs | examined |
| FR-331-AC-32 | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The test does not pin AC-32's precondition: it never asserts the extent domain kind is `infinite-trace`, that `finite_bound_available` is `false`, or that the population bound is a `cardinality` bound. Mutating the vector's domain kind to `integer` or `finite_bound_available` to `true` keeps the test green, so the vector can drift out of the case AC-32 names without any failure. Add `assert_eq!(item["extent"]["domains"][0]["kind"], "infinite-trace")`, `assert_eq!(item["extent"]["finite_bound_available"], false)` and a `bound.type == "cardinality"` check on the population entry | tests/backend_provider_v1.rs:701-720 |

## Verdict

Not mergeable until FND-001 (three extra assertions) is fixed; the binding is correct.
