---
id: SR-14612
title: 'Gap analysis of PR #351: FR-201-AC-10 against TC-952 (STD-91)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@f26c632; git diff origin/main(97bfd65)...f26c632 (PR #351, branch std-fr322-std-91): FR-201 Values row and FR-201-AC-10, FR-201 Dependencies, AD-016 arrow 5 and seed vector, TC-952 (new), TC-272 precondition; ticket STD-91'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-201
  type: reviews
- target: ix://agent-ix/quire-specification/TC-952
  type: reviews
---

## Summary

Ticket: STD-91. Plan completion: not assessed. This is a manual mapping of new and changed ACs to TC rows. FR-201-AC-10 (new) maps to TC-952 OI-01 to OI-07. TC-952 `verifies` FR-201 and FR-201-AC-10. No existing AC changed. No test is tagged to FR-201-AC-10 yet; the PR lists the CG test as STD-91 remaining work (CG TK-05).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-201-AC-10 | spec/objects/foundation/FR-201-canonical-identity-domain.md:111 | examined |
| TC-952 | spec/test-cases/TC-952-obligation-identity-digest-domain.md | examined |
| FR-201 value quire.ir-proof.obligation-identity/v1 | spec/objects/foundation/FR-201-canonical-identity-domain.md:47 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-201 row places clause, operation-application, function and per-step-record obligations in the domain, but FR-201-AC-10 and TC-952 test only a clause obligation (`O1`, a `claim` occurrence). No vector computes a function-contract identity (`declaration` occurrence) or an application identity, and none computes a per-step record. For the per-step record O-09 excludes the occurrence key, so OI-04's "another ordinal gives a different digest" does not hold for it. The other forms are either outside the AC, which should then say so, or need one vector each. | spec/objects/foundation/FR-201-canonical-identity-domain.md:111 |

## Verdict

Every new AC has a TC row, and OI-01 to OI-07 cover each clause of FR-201-AC-10 for a clause subject: source-span only, node id, ordinal, kind, parameter node id, domain and cross-domain labels. One coverage gap: the other subject forms the row puts in the domain have no vector (FND-001).
