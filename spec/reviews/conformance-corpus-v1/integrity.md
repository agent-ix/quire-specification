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

PASS. The replay request fixture has all 15 required members; nested request types, target members and source arms align across the schema, FR-336 mapping table and FR-350 adapter binding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-323 target/source forms and the structural RequestTargetWire fields remain distinct, and declared-domain kind/null and bound fields are represented separately. | FR-336-AC-7; TC-970; FR-350 |
