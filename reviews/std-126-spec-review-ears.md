---
id: SR-13811
title: "EARS analysis of PR #325 changed statements and criteria"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@3a52c6820e06ce83c3bf888f950292f2c7205332; changed prose and criteria in FR-150, FR-154 (statement paragraph and FR-154-AC-2), FR-321 (validation paragraph and FR-321-AC-2), FR-322 (Model-owned members step 1, Identity and validation, FR-322-AC-32)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
---
# EARS analysis of PR #325

## Summary

Ticket: STD-126. No `SHALL` statement changed. The edits are to explanatory
prose and to three acceptance criteria. The prose states observable
behaviour: intake looks up bytes by the selected digest, and the order of
refusals. FR-322-AC-32 and FR-321-AC-2 each state an observable refusal or
admission. FR-154-AC-2 adds a negative internal claim that no test can
observe.

## Verdict

Pass with one low finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-154-AC-2 ends with "intake never recomputes it", which states an internal step no test can observe. The observable behaviour is the admission clause before it ("a supplied document is admitted whatever its own canonical digest is") | spec/functional/type-model/FR-154-admit-domain-package-model.md:261 |
