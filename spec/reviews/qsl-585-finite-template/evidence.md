---
id: SR-22506
title: "Evidence review of QSL-585 finite task certificate identities"
type: SpecReview
analysis: evidence
scope: "FR-418-AC-11..13, FR-419-AC-12, TC-371 ZC-19..25 and zone certificate wire tests"
review_set: all
---

## Summary

Quoin advice recommends `Test` for the changed criteria and reports no authored
method mismatch. The QSpec wire test reads the published positive and
malformed vectors, checks canonical bytes, ensures semantic rank/slot
mutations remain schema-valid, refuses exact identities in certificate
positions, and confirms exact identities remain valid in search evidence.
TC-371 assigns recomputation and concrete replay outcomes to the QSL reader
boundary. No evidence-method mismatch found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No verification-method mismatch or missing vector obligation found in the changed scope. | FR-418, FR-419, TC-371 |
