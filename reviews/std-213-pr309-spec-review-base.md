---
id: SR-13638
title: "QSpec PR #309 (STD-213) FR-331 subject-only proof-bound substitution"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@d22f6cab318ce014d8273fa2c2f6cfa02b52e7b9 vs main d080afa; spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/test-cases/TC-431-backend-provider-wire-vectors.md, tests/backend_provider_v1.rs; context proposals/backend-provider-v1/vectors.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---

## Summary

Ticket: STD-213 (PR #309, head d22f6cab318ce014d8273fa2c2f6cfa02b52e7b9). FR-331 now states that a temporal subject's `proof_bounds` are independent of `extent`, which is the STD-213 planner default (flagged as such in the PR body). It is consistent with the existing `ProofBound` bullet (one shape and kind vocabulary for both places) and with the existing `temporal-request` vector, which already has an `unbounded` `infinite-trace` extent beside a `population` `cardinality` subject bound. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (12 + 112).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-331-AC-32 | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331 subject.proof_bounds bullet | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| TC-431 BP-20 | spec/test-cases/TC-431-backend-provider-wire-vectors.md | examined |
| FR-331 ProofBound bullet | spec/objects/interfaces/FR-331-backend-provider-envelope.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "is admitted, with the extent unchanged" has no before/after at the wire boundary: admission does not transform the extent, so "unchanged" can only mean the vector's extent is `unbounded` with no `bounds`. Say that (as BP-20 does), and state that the producer-side rule (the bound moves nothing into `extent.bounds`, changes neither classification nor `finite_bound_available`) is a producer obligation this wire AC does not check | spec/objects/interfaces/FR-331-backend-provider-envelope.md:406 |

## Verdict

Mergeable on the spec text (low only); see SR-13640 for the test.
