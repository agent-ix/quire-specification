---
id: SR-585
title: "Risk and complexity review of I04 CheckedPackage V1"
type: SpecReview
analysis: risk-complexity
scope: "FR-322, FR-195, checked-package-v1"
review_set: all
---
## Summary

The highest-risk boundary is complete semantic-node correspondence across source
and Contract IR. The closed V1 registry and all-family vector reduce, but do not
eliminate, producer/consumer evolution risk.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Any new semantic node form requires a new graph revision and cross-owner compatibility vector; a v1 consumer must refuse it rather than approximate it. | FR-322; FR-195 |
