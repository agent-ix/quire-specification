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

PASS. The ownership direction is QSpec contract to QSL consumer, and QSL
consumes QSpec's declared applicability and result fields without adding a
reverse dependency.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unresolved dependency issue found: FR-336 owns the wire contract; FR-350/351/353 depend on it for execution, scope and preflight settlement. | FR-336; FR-350; FR-351; FR-353 |
