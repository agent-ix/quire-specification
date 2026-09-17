---
id: SR-583
title: "Dependency review of I04 CheckedPackage V2"
type: SpecReview
analysis: dependency
scope: "interface_004, FR-322, FR-195, TC-217"
review_set: all
---
## Summary

The enablement sequence is QSpec schema/vectors, QSL producer, then Contract IR
reader/lowerer. The producer and consumer remain separate owner deliveries.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The QSL producer emits the selected `quire.checked-package/v2` wire; this record does not review or authorize the Contract IR consumer implementation. | interface_004; FR-322; TC-217 |
