---
id: SR-810
title: "Base review of conformance corpus v1 wire contract"
type: SpecReview
analysis: base
scope: "QSpec FR-336, TC-969/970 and conformance-corpus-v1 schema/fixtures; QSL FR-350/351/353"
review_set: subset
---
# Base review of conformance corpus v1 wire contract

## Summary

PASS. The replay adapter now maps all 15 ReplayRequestWire members, reuses FR-323 source/target/result structures where defined, and closes the uncovered nested Rust types structurally. The example is schema-focused and makes no qualification claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new replay mapping carries digest fields because the owning FR-071/FR-323 contracts require them; they identify inputs and are not used as test or review authority. AC-7/8 link to TC-970; implementation trace evidence remains for the QSL code slice. | FR-336-AC-7/8; TC-970; FR-350 |
