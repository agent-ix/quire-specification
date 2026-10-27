---
id: SR-813
title: "Dependency review of conformance corpus v1 wire contract"
type: SpecReview
analysis: dependency
scope: "QSpec FR-336/interface_019; QSL FR-350/351/353"
review_set: subset
---
# Dependency review of conformance corpus v1 wire contract

## Summary

PASS. The ownership direction remains QSpec FR-336 to QSL FR-350. QSpec references the existing FR-323 and FR-071/072 contracts; QSL binds the corpus-owned adapter to the typed request and replay operation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-336 owns the corpus adapter; FR-323 owns reused envelope shapes; FR-071/072 own request and result behavior. The dependency direction remains acyclic. | FR-336; FR-323; FR-350; FR-071; FR-072 |
