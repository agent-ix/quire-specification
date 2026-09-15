---
id: SR-581
title: "Base review — FR-236 compensation outcome partition"
type: SpecReview
analysis: base
scope: "FR-236, FR-058, TC-058, and TM-004 outcome-partition amendments"
review_set: all
---
## Summary

The partition makes the attempt and aggregate subjects explicit and provides
planned property evidence for every admitted and forbidden pair. One identity
ambiguity requires amendment before acceptance.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | Keying a per-attempt outcome fact by its label permits conflicting outcomes for one attempt/evaluation; require one outcome slot whose label is its value, not an identity component. | FR-236 Identity, FR-236-AC-5 | wrong-requirement |

## Disposition

Resolved before acceptance: FR-236 now keys the fact by exact attempt and admitted-observation identities, treats the label as its value, and requires refusal on conflicting labels.
