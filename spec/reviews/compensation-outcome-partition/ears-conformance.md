---
id: SR-565
title: "EARS conformance review — FR-236 compensation outcome partition"
type: SpecReview
analysis: ears-conformance
scope: "FR-236 and FR-058 outcome-partition clauses"
review_set: all
---
## Summary

The subject and refusal rules are concrete, but the per-attempt identity clause
does not state the selected-evaluation condition needed for a deterministic
response to conflicting labels.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Add an explicit condition: when two different labels are offered for one attempt and selected evaluation, the evaluator shall refuse rather than retain both. | FR-236 Identity, FR-236-AC-5 | missing-requirement |

## Disposition

Resolved before acceptance in FR-236 Identity and FR-236-AC-5: two different labels for one exact attempt/admitted-observation pair SHALL refuse.
