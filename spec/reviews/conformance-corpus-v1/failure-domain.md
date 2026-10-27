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

PASS. The replay adapter closes source, target, witness-value and request objects; unsafe/unlisted byte paths refuse before request construction, while readable-path and digest errors remain assigned to their existing per-vector and FR-071 owners.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Structural unknowns, unsupported source arms and unsafe byte paths have explicit refusal points; ordinary missing vector files remain isolated tool failures. | FR-336-AC-7/8; TC-970; FR-350 |
