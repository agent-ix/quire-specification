---
id: SR-2026102811
title: "Integrity review of stale identity wire contract"
type: SpecReview
analysis: integrity
scope: "FR-331; TC-431; backend-provider-v1 schema and vectors"
review_set: all
---

## Summary

The normative distinction, strict schema alternatives, and positive/negative vectors agree. Each stale identity has a discriminator and exactly one domain payload.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found in the scoped identity contract. | FR-331-AC-49; TC-431 BP-48 |
