---
id: SR-031
title: "Evidence review of the v1 protocol and result specification"
type: SpecReview
analysis: evidence
scope: "FR-050..FR-061, NFR-020..NFR-021 and TM-004"
review_set: all
evaluated_revision: "57ac0f3"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/TM-004, type: reviews }
---
## Summary

The mandatory `quoin advise` run completed against the installed catalog after
running outside the managed child-process sandbox tracked by quoin#360. It reports
zero inconclusive obligations. All new FR criteria use the recommended Test
class; one NFR metric intentionally retains Analysis over the advisor's generic
threshold-triggered benchmark recommendation. Eight uncatalogued methods remain
in pre-existing NFR-001..004 and are outside this review scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved: five new NFR metric methods used descriptive uncatalogued labels. They now use catalogued Test/Analysis classes while the Verification prose retains the concrete boundary, mutation, metamorphic and strict-reader methods. | NFR-020; NFR-021 |
| FND-002 | low | Accepted by reviewer judgment: NFR-020-M-1 keeps Analysis for detecting unbounded caller-controlled traversals although the catalog recommends performance benchmarking solely because the threshold is numeric zero. Boundary performance is separately exercised by M-2/TC-062. | NFR-020-M-1 |

## Confirmed evidence strategy

| Obligation group | Catalog signal | Planned evidence |
| --- | --- | --- |
| Identity and result invariants | property/example shapes | Property and adverse Rust tests with one-axis identity mutations. |
| Parallel conformance | concurrency/protocol characteristics | Deterministic simulation of permitted interleavings and model-based generation where the implementation exposes the reviewed state model. |
| Compensation failure/timeout | reliability characteristics | Fault injection through the shared Engineering Assurance interface, with owned failed TechniqueResults. |
| Result algebra | universal/example shapes | Strict-reader property tests across valid and forbidden dimension combinations. |
| Resource boundaries | quantified threshold | Exact-boundary and one-over tests plus limit-check mutation; static analysis for unbounded traversal inventory. |
| Reproduction | stable identity and protocol characteristics | Metamorphic permutation tests and semantic-dependency identity mutation. |

TC artifacts remain planned. Executed evidence requires the future Rust
implementation and Engineering Assurance producer; no local runner, campaign or
passing status is fabricated by this review.

