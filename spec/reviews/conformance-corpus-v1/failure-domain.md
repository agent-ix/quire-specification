---
id: SR-811
title: "Failure-domain review of conformance corpus v1 wire contract"
type: SpecReview
analysis: failure-domain
scope: "QSpec FR-336 and schema; QSL FR-350/351/353"
review_set: subset
---
# Failure-domain review of conformance corpus v1 wire contract

## Summary

PASS. The wire distinguishes schema/preflight refusal from a missing but
otherwise valid vector input, which remains an isolated per-vector tool failure.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No unresolved failure-domain issue found in this change: closed object shapes, typed expected dispositions and explicit reader-side uniqueness/reference checks preserve refusal boundaries; a valid unreadable input does not abort sibling vectors. | FR-336; FR-350; FR-353 |
