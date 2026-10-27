---
id: SR-2026102702
title: "Failure-domain review of QSL-1168 tuple and union payload contract"
type: SpecReview
analysis: failure-domain
scope: "FR-154, FR-208, TC-234, TC-235"
review_set: subset
---

## Summary

The contract names malformed tuple shapes, position conflicts, missing references, and unsupported payloads. It distinguishes nullary union members from typed payload members and leaves recursive-value rules to FR-143.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unstated failure-domain or identity cases found in the changed contract. | FR-154-AC-11, FR-154-AC-25, FR-154-AC-33, FR-208-AC-15, FR-208-AC-19 |
