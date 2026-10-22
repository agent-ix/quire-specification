---
id: SR-415
title: "Risk and complexity review of native temporal evaluator semantics"
type: SpecReview
analysis: risk-complexity
scope: "FR-091, FR-093 and FR-094"
review_set: all
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-091, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-093, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-094, type: reviews }
---
## Summary

The highest risk is semantic substitution across layers, not evaluator syntax.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Clock conversion and closure encoding are identity-bearing selections and must be rejected when unmatched. | FR-091; FR-094 |
| FND-002 | high | Captured trigger state and live obligation retention need explicit resource accounting, including bounds on active instances and retained bytes. | FR-093; FR-094 |
| FND-003 | medium | Rust implementation must use closed result enums, domain newtypes and checked conversions rather than stringly typed or saturating state. | L5 #38 |

