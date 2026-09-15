---
id: SR-560
title: "Integrity review — FR-236 compensation outcome partition"
type: SpecReview
analysis: integrity
scope: "FR-236, FR-058, TC-058, TM-004"
review_set: all
---
## Summary

Exact subject identity, authority retention, absence, and no-fallback rules are
defined. The per-attempt fact must retain one explicit evaluation identity to
avoid two incompatible labels becoming separate valid facts.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Per-attempt outcome comparison lacks an exact evaluation identity and therefore cannot distinguish a correction from a contradiction. | FR-236 Identity | wrong-requirement |

## Disposition

Resolved before acceptance: the exact attempt/admitted-observation pair is unique, and distinct offered labels for that pair refuse.
