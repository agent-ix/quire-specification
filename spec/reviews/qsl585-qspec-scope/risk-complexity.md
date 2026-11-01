---
id: SR-12006
title: "Risk-complexity review of QSL-585 task automaton specification"
type: SpecReview
analysis: risk-complexity
scope: "FR-181, FR-331, FR-364, FR-415, FR-418, FR-419"
review_set: all
---

## Summary

The bounded-subtraction region construction and queue-growth continuation checker are high technical risk because they combine symbolic clock reasoning, scheduler policy, identities, and feedback. The current specification makes the replay proof obligations explicit and leaves implementation verification to QSL.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No additional requirement-level mitigation gap found; the downstream QSL implementation must exercise the finite-product closure and queue-growth replay cases in TC-372. | FR-418, FR-364, FR-419 |
