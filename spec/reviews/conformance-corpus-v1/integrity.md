---
id: SR-812
title: "Integrity review of conformance corpus v1 wire contract"
type: SpecReview
analysis: integrity
scope: "QSpec FR-336, interface_019, TC-969; QSL FR-350/351/353"
review_set: subset
---
# Integrity review of conformance corpus v1 wire contract

## Summary

PASS. The vector, capability, rollup, subject and expected-disposition shapes
are consistent across the schema, fixture, interface and QSL consumer rules.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unresolved integrity issue found: the `incomplete` vector class is not confused with a result disposition, capability applicability is separate from per-vector applicability, and QSL reads only the primary capability rows for scope. | FR-336; FR-350; FR-351 |
