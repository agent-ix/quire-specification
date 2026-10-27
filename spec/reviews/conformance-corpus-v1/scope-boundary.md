---
id: SR-816
title: "Scope-boundary review of conformance corpus v1 wire contract"
type: SpecReview
analysis: scope-boundary
scope: "QSpec FR-336/interface_019; QSL FR-350/351/353"
review_set: subset
---
# Scope-boundary review of conformance corpus v1 wire contract

## Summary

PASS. QSpec owns the replay-request harness schema and reusable FR-323 shapes; QSL owns conversion to ReplayRequestWire and execution under FR-071/098. Byte paths remain harness metadata and do not enter the product request.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The replay adapter is explicitly corpus-owned; FR-323 governs reused members, and the typed request remains unchanged. | FR-336; FR-323; FR-350; FR-071 |
