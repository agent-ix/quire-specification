---
id: SR-561
title: "Dependency review — FR-236 compensation outcome partition"
type: SpecReview
analysis: dependency
scope: "FR-236, FR-057, FR-058, FR-235, FR-238, TC-057, TC-058"
review_set: all
---
## Summary

The ruling consumes existing attempt, relation, effect, and population
identities and introduces no Protocol or producer dependency. The outcome slot
needs its evaluation identity stated before a consumer can safely bind it.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The new per-attempt outcome fact omits the evaluation identity that separates one observation revision from another. | FR-236 Identity | missing-requirement |

## Disposition

Resolved before acceptance: FR-260 admitted-observation identity is now an exact component of the per-attempt outcome fact key.
