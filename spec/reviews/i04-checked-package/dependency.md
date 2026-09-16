---
id: SR-583
title: "Dependency review of I04 CheckedPackage V1"
type: SpecReview
analysis: dependency
scope: "interface-004, FR-322, FR-195, TC-217"
review_set: all
---
## Summary

The enablement sequence is QSpec schema/vectors, QSL producer, then Contract IR
reader/lowerer. The producer and consumer remain separate owner deliveries.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The future QSL producer must emit this frozen public wire and be qualified by a producer-consumer contract test; its implementation is outside the temporary I04 consumer handoff. | interface-004; FR-322; TC-217 |
