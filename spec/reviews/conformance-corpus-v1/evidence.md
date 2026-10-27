---
id: SR-814
title: "Evidence review of conformance corpus v1 wire contract"
type: SpecReview
analysis: evidence
scope: "QSpec FR-336-AC-4/TC-969 and QSL FR-353/TC-881"
review_set: subset
---
# Evidence review of conformance corpus v1 wire contract

## Summary

TC-970 specifies positive schema validation, unknown-member/tag mutations, and relative byte-path checks for replay adapters. QSL implementation tests and trace bindings remain pending the replay code slice.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Schema shape and path-binding requirements now have a dedicated case; consumer decoder semantics must be discharged by QSL tests before implementation completion. | FR-336-AC-7/8; TC-970; FR-350-AC-10 |
