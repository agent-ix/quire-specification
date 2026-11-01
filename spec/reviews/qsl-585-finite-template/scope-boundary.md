---
id: SR-22507
title: "Scope-boundary review of QSL-585 finite task certificate identities"
type: SpecReview
analysis: scope-boundary
scope: "FR-418 certificate wire, FR-419 task semantics, FR-364 replay, TC-371"
review_set: all
---

## Summary

QSpec owns the finite certificate template and its closed wire shape. The
checked claim remains the authority for task policy and task parameters; source
operation edges retain source identities; search evidence and timed witnesses
retain exact task identities; and the replay layer reconstructs concrete
occurrences and generations. No responsibility is duplicated or left
unassigned in the changed contract.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No scope allocation or external-boundary gap found in the changed contract. | FR-418, FR-419, FR-364 |
