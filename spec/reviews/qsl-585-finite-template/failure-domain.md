---
id: SR-22504
title: "Failure-domain review of QSL-585 finite task certificate identities"
type: SpecReview
analysis: failure-domain
scope: "FR-418 finite task-transition identity; FR-419 replay identity; TC-371 ZC-19..25"
review_set: all
---

## Summary

The reader binds each template to a checked task and recomputed successor,
checks the task-local live-job rank and slot, and closes states only under a
task-preserving bijection that retains phase, queue order, clocks and
activation order. Concrete replay restores exact occurrence and generation
values, including after reuse. Malformed wire and semantic mismatch cases are
separate; no uncovered identity or termination failure mode found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No identity-confusion, refusal-path, or finite-closure gap found in the changed scope. | FR-418, FR-419, TC-371 |
