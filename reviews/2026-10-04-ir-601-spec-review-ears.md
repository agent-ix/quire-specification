---
id: SR-1294
title: "EARS review of PR #188: division checks only the exposed member"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@f0002b92095dee386759dd17530db0f757f285d6; PR #188 edited statements and criteria: FR-147 Outputs, Behavior, Division profiles, FR-147-AC-1, AC-6, AC-9, AC-10; FR-007 Behavior; FR-271 values row and AC-6; FR-272 row and AC-11"
review_set: subset
---
# EARS review of PR #188 (IR-601)

## Summary

Ticket: IR-601. I checked the edited requirement text and criteria for one
behaviour per criterion, a stated trigger and response, and a result a test
can check.

- FR-147-AC-9 names the three operators, their codes and causes, under each
  profile, with no member exposed. Each part can be tested.
- FR-147-AC-10 gives concrete domains, operands and expected values for the
  member-only rule, so a pair-checking implementation fails it.
- FR-147-AC-1 and AC-6 now name `div` and `rem` as separate single-result
  expressions with their four charges.
- The FR-271, FR-272 and FR-007 statements are declarative and do not
  contradict FR-147.

## Verdict

Clean: no EARS defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
