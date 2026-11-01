---
id: SR-22503
title: "Integrity review of QSL-585 finite task certificate identities"
type: SpecReview
analysis: integrity
scope: "FR-418-AC-11..13, FR-419-AC-12, TC-371 ZC-19..25, certificate wire schema"
review_set: all
---

## Summary

The finite identity change has one wire-shape criterion, one finite-closure
criterion and a replay criterion, with TC-371 covering canonical bytes, exact
identity refusal, rank/slot mismatch and reuse while another task job remains
live. The exact `TaskTransitionIdentity` path remains available in search
frontiers and timed witnesses. No contradiction, duplicate obligation or
unverified changed criterion found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No completeness, consistency, or atomicity issue found in the changed scope. | FR-418, FR-419, TC-371 |
