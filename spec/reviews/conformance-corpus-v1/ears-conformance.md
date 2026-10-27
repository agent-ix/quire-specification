---
id: SR-817
title: "EARS conformance review of conformance corpus v1 wire contract"
type: SpecReview
analysis: ears-conformance
scope: "QSpec FR-336; QSL FR-350/351/353"
review_set: subset
---
# EARS conformance review of conformance corpus v1 wire contract

## Summary

The new ACs name the replay-request actor, closed schema, mapping obligations, refusal conditions and byte-path boundary; they are test-linked through TC-970.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-7/8 state observable structural and path behavior and are verified by the replay adapter case. | FR-336-AC-7/8; TC-970 |
