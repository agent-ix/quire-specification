---
id: SR-14744
title: "EARS analysis of STD-188 (QSpec PR #350)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@a8be66647bfb9a32fd2b546d467a1cbe54bc121c; spec/functional/analysis/FR-405-declare-probabilistic-models.md, spec/functional/analysis/FR-407-state-probabilistic-claims.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-405
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-407
    type: reviews
---
# SR-14744: EARS analysis of STD-188 (QSpec PR #350)

## Summary

Ticket: STD-188. The PR changes no FR statement line. Each new rule in
FR-405 and FR-407 is one unwanted-behaviour clause ("a ... refuses with
`invalid_probabilistic_model`/`<cause>`"), and it names the subject, the
condition and the cause. FR-405-AC-9 and FR-407-AC-10..15 each state one
input and one outcome. No new text uses changelog, status or review
language.

The trigger "a claim that reaches the frontend by any other route" has no
defined input. It is recorded once, as a buildability defect, in SR-14743
FND-001, and is not repeated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean for EARS shape. The `ears:` warnings that quire reports on FR-405 and
FR-407 are the same as on main.
