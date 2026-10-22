---
id: SR-15213
title: 'Spec integrity analysis of PR #360: NFR-076 proof evidence by discharge (STD-5)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@2134324; git diff origin/main...2134324 (PR #360, branch std-5-proof-acceptance): spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md (new, NFR-076-AC-1..AC-6); spec/test-cases/TC-953-proof-gate-discharge-acceptance.md (new, P01..P09); spec/spec.md (contains edge); ticket STD-5'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/NFR-076
  type: reviews
---
# Spec integrity analysis of PR #360: NFR-076 proof evidence by discharge (STD-5)

## Summary

Ticket: STD-5. PR #360 at 21343242b1d7d6b528dabbc76e3783c794819b7f, reviewed in a detached worktree; diff scope origin/main...head. Method: spec-review/spec-integrity-analysis.

Examined:
- NFR-076 (examined): spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md
- NFR-076-AC-1 (examined): spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md
- NFR-076-AC-2 (examined): spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md
- NFR-076-AC-3 (examined): spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md
- NFR-076-AC-4 (examined): spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md
- NFR-076-AC-5 (examined): spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md
- NFR-076-AC-6 (examined): spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md
- TC-953 (examined): spec/test-cases/TC-953-proof-gate-discharge-acceptance.md
- spec/spec.md NFR-076 contains edge (examined): spec/spec.md

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | [major] TC-953 is not unique: open PR #361 (std-21-domain-clause-names, head 8a0ac91) adds spec/test-cases/TC-953-domain-clause-name-resolution.md with id TC-953 and links it from FR-470. Whichever PR lands second duplicates the id; the PR body's 'No open PR uses either' no longer holds. | spec/test-cases/TC-953-proof-gate-discharge-acceptance.md:2 |
| FND-002 | high | [major] AC-5's second clause and the Scope bullet bind FR-331 `proved` results to proof gates, but an FR-331 `proved` result is a backend's settlement of a claim (proof basis plus certification, including `uncertified` solver results), not output of a CI proof gate over RT or CG code; ADR-011 §2.3 scopes the rule to proof evidence the #205 gates count and names no FR-331 coupling. No test can observe that a gate run 'contributes no proof evidence to a proved result', and read literally the clause makes FR-331 `proved` depend on RT/CG gate runs, which FR-331 does not state. | spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md:30-31,107 |
| FND-003 | low | [minor] ADR-011 §2.3 requires the alternative expectation source to be a checked-in specification model that calls no quire-exact operation; NFR-076 drops 'checked-in', so an untracked model would satisfy it. | spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md:67-70 |

## Verdict

NOT MERGEABLE: TC-953 id collision with open PR #361, and an FR-331 coupling that cannot be tested and has no ADR basis. Otherwise faithful to QSL ADR-011 §2.3 (lines 429-485) and T-11 (:1489): SUCCESS-or-proved only, no allow-listed mutant, shared-helper mutation over the built call graph, unreached modules fail, stubbed executor/verdict/build-only never evidence. NFR-076 is unique. quire validate: 134 failing documents, identical to merge-base 0a11d84, none in new files; cargo tests 126 + 23 passed. PR body carries no review-finding paraphrase.
