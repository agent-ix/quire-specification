---
id: SR-15452
title: "failure-domain review of STD-261 FR-323 decode and resolution order (QSpec PR #364)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@b5a2d15c0155235a398ec330d8892e19f4919973; spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-278-replay-envelope-members.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-278
    type: reviews
---
# SR-15452: failure-domain review of STD-261 FR-323 decode and resolution order (QSpec PR #364)

## Summary

Ticket: STD-261. Examined: rule 1 decode refusals (none, more than one, and a non-function request
with `arguments` or an occurrence key) and their order before the byte-provision rule; rule 2 resolution
after dependency rule 5; how resolution staleness differs from envelope staleness; and what happens when a
state clause's name is a function's name.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | minor: the prose's rule for more than one member also covers a request carrying all three, which gives `/replay/state_clause`. AC-19 and RE-39 state only pairs. Add the all-three case to AC-19 and RE-39 so that an implementation that only checks pairs fails a test. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:529 |

## Verdict

Mergeable. The decode order is total and every refusal pointer is determined. One minor coverage gap.

## Dispositions

Round 1, reviewed QSpec #364 at 415f9dd0e.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed at 415f9dd0e (AC-19 and RE-39 add the all-three case) |
