---
id: SR-413
title: "Dependency review of native temporal evaluator semantics"
type: SpecReview
analysis: dependency
scope: "FR-091, FR-093 and FR-094"
review_set: all
evaluated_revision: "b78fccb"
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-091, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-093, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-094, type: reviews }
---
## Summary

The amendment is semantically complete in its own boundary but deliberately
does not claim an executable cross-repository bridge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Implementation depends on strict `quire.compiled-protocol/2` definition and clock binding; `/1` cannot authenticate the selected temporal meaning. | FR-090; L5 #38 |
| FND-002 | high | Predicate truth requires FR-025 and temporal lowering requires FR-026; a finite-window profile cannot be silently mapped to false-extension TL semantics. | FR-091; contract-IR #63/#64 |
| FND-003 | medium | Progress and completeness authority remain an observation-owner input, not a property of EOF or receipt order. | FR-094 |

