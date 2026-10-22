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

## Dispositions

Round 1, checked at head 9f74956915d5c764a6a8b35f227efcc4cdcec7b4 (fix commit 9f74956, contains origin/main). Gates: quire validate --scope . spec/**/*.md fails 116 documents, identical to origin/main, none in NFR-076, TC-953 or spec/spec.md; cargo test --test checked_package_v2 --test backend_provider_v1: 126 + 23 passed. QSL ADR-011 §2.3 (lines 429-485) is unchanged between qsl-lane2 and QSL origin/main.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | TC-953 is now unique among open QSpec PRs: #361 (head 91b3c4e) renumbered its case to spec/test-cases/TC-954-domain-clause-name-resolution.md and no other open PR or origin/main adds TC-953. After: `id: TC-953` / `title: "Proof gate discharge acceptance"` in spec/test-cases/TC-953-proof-gate-discharge-acceptance.md. |
| FND-002 | fixed | FR-331 coupling removed. After (AC-5): "A gate whose pass rests on a stubbed executor, a predetermined verdict or a build-only run fails, and contributes no proof evidence to a qualification claim (NFR-075)." Scope: "Proof evidence from other gates counts toward a qualification claim (NFR-075) only when those gates meet this requirement." and "Does not govern ... how a backend settles a claim or a terminal record, or the `proved` result a backend returns (FR-331)." |
| FND-003 | fixed | After (Behavior item 3): "An expectation for an exact kernel operation, or for a runtime operation that calls one, comes from the QSpec operation vectors or from a checked-in specification model that calls no exact kernel operation, and so shares no kernel helper." AC-6 carries the same "checked-in" wording. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | [minor] TC-953 P09 expects that "the run contributes no proof evidence to a qualification claim", but fixture G holds no qualification claim and no step reads one, so that half of the row (and of AC-5) has no observable in the procedure; the failing gate result is the only thing the row can check. | spec/test-cases/TC-953-proof-gate-discharge-acceptance.md:39; spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md:109 |
| FND-005 | low | [minor] The Scope bullet "Proof evidence from other gates counts toward a qualification claim (NFR-075) only when those gates meet this requirement" copies ADR-011 §2.3's "other gates" (gates other than the #205 gates) but NFR-076 names no set those gates are other than; the first Scope bullet already binds every gate whose result is offered as proof evidence, so a reader cannot tell which gates "other" means. | spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md:30-31 |
