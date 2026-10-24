---
id: SR-16359
title: "Failure-domain review of STD-449 timed source identity refusals"
type: SpecReview
analysis: failure-domain
scope: "QSpec PR #431: checked-package timed-model graph source and clock-place validation"
review_set: subset
---

## Summary

The identity boundary is explicit: the timed root selects the base Time node, and the reader follows the source unit's dimension reference and compares node identity. Tick clock places are found through the timed root and fail at the offending place's model binding. The rules do not depend on source text, names, declaration order or dimension-term equivalence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found; the added graph checks name the authority, failing node and pointer for both failure domains. | FR-322-AC-126..127; FR-415-AC-18; TM-23..TM-29 |

## Scope and method

Checked entity identity, source-to-root and place-to-model joins, and graph traversal failure behavior for tick-plus-clock and non-base-Time source units.
