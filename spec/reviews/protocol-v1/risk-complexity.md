---
id: SR-032
title: "Risk and complexity review of the v1 protocol and result specification"
type: SpecReview
analysis: risk-complexity
scope: "FR-050..FR-061 and NFR-020..NFR-021"
review_set: all
evaluated_revision: "61a7524"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-059, type: reviews }
---
## Summary

Distributed identity, concurrency, compensation and result monotonicity are the
highest technical risks. Volatility is concentrated in cross-owner producer
interfaces and later projection/realizability algorithms. The selected bounded
global-conformance slice and typed unsupported outcomes isolate those risks.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unresolved specification defect found. High-risk requirements have bounded slices, independent adverse/property cases and explicit producer or research gates. | FR-051..FR-061; NFR-020 |

## Risk register

| Req | Tech Risk | Volatility | Drivers | Mitigation |
| --- | --- | --- | --- | --- |
| FR-050 | medium | medium | Cross-repo identity contracts | IT-020 strict producer/consumer mutations. |
| FR-051 | high | medium | Distributed correlation and identity confusion | One-axis delivery/attempt/effect property cases. |
| FR-052 | high | low | Concurrency and graph topology | Finite graph, bounded iteration, interleaving generation. |
| FR-053 | high | medium | Information visibility and owned choice | Positive communication control plus unobservable-choice mutants. |
| FR-054 | high | medium | Delivery/order assumptions | Explicit per-channel premises and no inferred exactly-once behavior. |
| FR-055 | high | medium | Missing observations and temporal activation | Separate activation/truth and E/F completeness contract. |
| FR-056 | high | low | Effect-dependent state transition | Registration ordering/idempotence state-model cases. |
| FR-057 | high | medium | Commit/recovery semantics | Independent order mutants and typed recovery predicates. |
| FR-058 | high | medium | Retry, duplicates and aggregate membership | Bounded retries and closed-population exact arithmetic. |
| FR-059 | high | medium | Distributed replay/concurrency | Bounded global slice, deterministic simulation, batch/incremental agreement. |
| FR-060 | high | high | Unselected advanced algorithms and external libraries | Independent claims; unsupported/failure results; E03 before adoption. |
| FR-061 | high | medium | Cross-family result algebra | Normative combination table and strict-reader property tests. |
| NFR-020 | high | low | Resource exhaustion on untrusted graphs/observations | Exact limits, mutation tests and resource-incomplete results. |
| NFR-021 | medium | low | Determinism across concurrent presentations | Metamorphic permutation and identity tests. |

## Top hazards

1. FR-061: an inconsistent result can turn missing evidence or inactivity into success.
2. FR-051/FR-058: collapsing transport, attempt and effect identity can duplicate money movement claims.
3. FR-052/FR-059: timestamp or input order can be mistaken for causality.
4. FR-060: terminology can transfer an unqualified projection/realizability theorem.
5. NFR-020: finite syntax can still drive unbounded instance or observation work.

The failure-domain review is [SR-028](./failure-domain.md); its purity, backend
failure and topology findings are resolved.

