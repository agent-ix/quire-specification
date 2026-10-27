---
id: SR-2026102706
title: "Risk review of QSL-1168 tuple and union payload contract"
type: SpecReview
analysis: risk-complexity
scope: "FR-154, FR-208, TC-234, TC-235"
review_set: subset
---

## Summary

The new type forms are technically medium risk because they add ordered tuple positions and typed union payloads to package intake. Volatility is low: the accepted shape and refusal boundary are explicit, while the separately owned FCD/IR representation is a tracked dependency.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unmitigated risk gap found; the downstream implementation has direct positive, boundary, and ordering cases. | FR-154-AC-25, FR-154-AC-33, FR-208-AC-15, FR-208-AC-19, TC-234, TC-235 |
