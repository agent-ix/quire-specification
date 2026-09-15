---
id: SR-559
title: "Failure-domain review — FR-236 compensation outcome partition"
type: SpecReview
analysis: failure-domain
scope: "FR-236, FR-058, TC-058"
review_set: all
---
## Summary

The partition covers the material confusion failures: relabelling an attempt
as a duplicate, inferring recovery from success, and converting absence to
pending. The single-outcome amendment is needed to prevent contradictory facts.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | A label-bearing attempt-fact key allows both `failed` and `successful` facts for the same attempt without a declared evaluation discriminator. | FR-236 Identity | wrong-requirement |

## Disposition

Resolved before acceptance: the exact admitted-observation identity is part of the per-attempt fact key and conflicting labels for one pair refuse.
