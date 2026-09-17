---
id: SR-585
title: "Risk and complexity review of I04 CheckedPackage V2"
type: SpecReview
analysis: risk-complexity
scope: "FR-322, FR-195, checked-package-v2"
review_set: all
---
## Summary

The highest-risk boundary is complete semantic-node correspondence across source
and Contract IR. The closed node registry and all-family vector reduce, but do not
eliminate, producer/consumer evolution risk.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Any new semantic node form requires a new graph revision and cross-owner compatibility vector; a consumer pinned to an earlier schema revision must refuse it rather than approximate it. | FR-322; FR-195 |
