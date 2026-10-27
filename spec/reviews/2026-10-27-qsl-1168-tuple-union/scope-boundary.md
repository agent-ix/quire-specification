---
id: SR-2026102707
title: "Scope-boundary review of QSL-1168 tuple and union payload contract"
type: SpecReview
analysis: scope-boundary
scope: "FR-154, FR-208, TC-234, TC-235"
review_set: subset
---

## Summary

QSpec owns the meaning and acceptance contract for tuple types and variant payloads. FCD/IR owns the representation support, and QSL owns intake, finite-domain behavior, and its end-to-end tests; the amendment keeps those boundaries distinct.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No responsibility leakage or unstated external guarantee found in the changed artifacts. | FR-154, FR-208 |
