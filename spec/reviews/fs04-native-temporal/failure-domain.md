---
id: SR-411
title: "Failure-domain review of native temporal evaluator semantics"
type: SpecReview
analysis: failure-domain
scope: "FR-091, FR-093, FR-094, TC-111, TC-114 and TC-116"
review_set: all
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-091, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-093, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-094, type: reviews }
---
## Summary

The review exercised absent or conflicting guards, duplicate receipt identity,
missing decision support and resource exhaustion during concurrent obligations.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | No Boolean is fabricated from an unestablished guard, missing decision support or exhausted semantic work. | FR-091; FR-093; FR-094 |
| FND-002 | medium | Identical duplicate receipt is idempotent only when its payload agrees; unequal reuse remains a typed contradiction. | FR-093; TC-114 |
| FND-003 | medium | A ceiling applies to the affected obligation after preserving independently settled siblings. | FR-094; TC-116 |

