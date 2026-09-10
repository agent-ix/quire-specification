---
id: SR-028
title: "Failure-domain review of the v1 protocol and result specification"
type: SpecReview
analysis: failure-domain
scope: "FR-050..FR-061, NFR-020..NFR-021 and the protocol contract"
review_set: all
evaluated_revision: "61a7524"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-059, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
---
## Summary

Identity, purity, extension failure and graph termination were checked across
the protocol family. Two initial gaps are resolved; all entity classes remain
distinct, graph work is finite, predicates are pure and backend failure cannot
silently select a fallback.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: the initial draft did not explicitly constrain guards, triggers and recovery predicates to total read-only evaluation. The contract now forbids callbacks, I/O, mutation, reflection and user-code execution and types evaluation failure. | protocol-contract.md; FR-053; FR-055; FR-057 |
| FND-002 | medium | Resolved: an advanced analysis backend failure now yields a failed per-claim result, invokes no unreviewed fallback and preserves independent results. | FR-060-AC-5; TC-060 |

## Failure-domain checks

| Domain | Disposition |
| --- | --- |
| Extension points | Optional analysis backends have explicit supported/unsupported/refused/failed outcomes; no callback failure is suppressed. |
| Entity identity | Protocol/workflow/role/message/send/receive/delivery/attempt/effect/registration/commit/result identities are explicit and substitution refuses. |
| Evaluation purity | Guards, triggers and recovery relations are total and read-only; the evaluator executes no business action. |
| Graph topology | Causal cycles, missing joins, zero-progress loops and missing finite iteration bounds refuse. NFR-020 bounds structures and traversal. |
| Incomplete observations | Open/incomplete/unknown states remain distinct; a finite witnessed violation is the only specified incomplete-input truth exception. |

