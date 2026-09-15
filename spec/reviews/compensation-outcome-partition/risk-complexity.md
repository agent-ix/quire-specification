---
id: SR-563
title: "Risk-complexity review — FR-236 compensation outcome partition"
type: SpecReview
analysis: risk-complexity
scope: "FR-236, FR-058, TC-058"
review_set: all
---
## Summary

The subject partition reduces downstream mapping risk by keeping duplicate
effects outside attempt outcomes. The remaining moderate risk is ambiguous
same-attempt correction/contradiction treatment.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | A consumer can retain incompatible labels for one attempt because the partition has no selected-evaluation uniqueness rule. | FR-236 Identity | wrong-requirement |

## Disposition

Resolved before acceptance: FR-236 makes the admitted observation explicit in the key and makes conflicting labels an identity contradiction.
