---
id: SR-16366
title: "Risk-complexity review of QSL-1093 hyperproperty placement"
type: SpecReview
analysis: risk-complexity
scope: "FR-402, FR-370, TC-303, TC-346"
review_set: all
---

## Summary

The main implementation risk is keeping the reader's claim-member and node-kind checks aligned with FR-370's closed placement rules. The positive, negative, finite-domain, and alias-mismatch vectors mitigate that risk directly; no external or volatile dependency is introduced.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unmitigated high-risk or high-volatility issue found in the changed scope | FR-402, FR-370, TC-303, TC-346 |
