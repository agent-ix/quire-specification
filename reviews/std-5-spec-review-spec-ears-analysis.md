---
id: SR-15214
title: 'EARS analysis of PR #360: NFR-076 proof evidence by discharge (STD-5)'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@2134324; git diff origin/main...2134324 (PR #360, branch std-5-proof-acceptance): spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md (new, NFR-076-AC-1..AC-6); spec/test-cases/TC-953-proof-gate-discharge-acceptance.md (new, P01..P09); spec/spec.md (contains edge); ticket STD-5'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/NFR-076
  type: reviews
---
# EARS analysis of PR #360: NFR-076 proof evidence by discharge (STD-5)

## Summary

Ticket: STD-5. PR #360 at 21343242b1d7d6b528dabbc76e3783c794819b7f, reviewed in a detached worktree; diff scope origin/main...head. Method: spec-review/spec-ears-analysis.

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
| FND-001 | low | [minor] The Statement names only the discharge and mutation conditions; the shared-helper condition (Behavior item 3, AC-4) and the declared-module condition (AC-1) are normative but absent from it, so the Statement is narrower than the requirement its ACs verify. | spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md:17-19 |

## Verdict

Statement is ubiquitous-form EARS with a single SHALL; behaviour-only wording. One minor gap between Statement and Behavior.

## Dispositions

Round 1, checked at head 9f74956915d5c764a6a8b35f227efcc4cdcec7b4 (fix commit 9f74956, contains origin/main). Gates: quire validate --scope . spec/**/*.md fails 116 documents, identical to origin/main, none in NFR-076, TC-953 or spec/spec.md; cargo test --test checked_package_v2 --test backend_provider_v1: 126 + 23 passed. QSL ADR-011 §2.3 (lines 429-485) is unchanged between qsl-lane2 and QSL origin/main.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | After (Statement): "Every proof gate of a Quire runtime or code generation backend SHALL pass only when, for each module it claims, it discharges at least one proposition inside that module, a mutation inside that module turns it red, and every helper shared by its expectation and the function under proof fails the proof when mutated. A gate that declares no claimed module fails." |
