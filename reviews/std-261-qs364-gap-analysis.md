---
id: SR-15453
title: "gap analysis of STD-261 FR-323 state_clause ACs and TC-278 vectors (QSpec PR #364)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@b5a2d15c0155235a398ec330d8892e19f4919973; spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-278-replay-envelope-members.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-278
    type: reviews
---
# SR-15453: gap analysis of STD-261 FR-323 state_clause ACs and TC-278 vectors (QSpec PR #364)

## Summary

Ticket: STD-261. This is a spec-only PR. Plan completion: not assessed. AC-to-vector trace: AC-18 is
traced by RE-37; AC-19 by RE-38, RE-39 and RE-40; AC-21 by RE-37 and RE-41. AC-6 is still traced by TC-278's
existing vectors. Each new criterion has a negative case. Gates at b5a2d15c0: `quire validate --scope .
'spec/**/*.md'` printed 86 failure lines (baseline 86). `cargo test --test checked_package_v2 --test
backend_provider_v1` passed 23 and 118 tests. The executor code for `state_clause` is outside this repo and
is the QSL code lane's work.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. Every new or changed AC has a TC-278 vector.
