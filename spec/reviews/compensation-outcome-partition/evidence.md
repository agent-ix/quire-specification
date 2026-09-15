---
id: SR-562
title: "Evidence review — FR-236 compensation outcome partition"
type: SpecReview
analysis: evidence
scope: "FR-236, FR-058, TC-057, TC-058, TM-004"
review_set: all
---
## Summary

TC-058 plans independent mutation for every allowed and forbidden pair, while
TC-057 retains the non-inference rule. The test oracle needs a declared
single-outcome boundary for a selected evaluation.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The planned mutation oracle cannot decide whether two labels for one attempt are a forbidden contradiction or two valid facts. | FR-236-AC-5, TC-058 | correct-requirement-no-evidence |

## Disposition

Resolved before acceptance: FR-236-AC-5 and TC-058 now make the conflicting-label oracle explicit: the exact pair refuses.
