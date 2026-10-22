---
id: SR-15215
title: 'Gap analysis of PR #360: NFR-076-AC-1..AC-6 against TC-953 P01..P09 (STD-5)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@2134324; git diff origin/main...2134324 (PR #360, branch std-5-proof-acceptance): spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md (new, NFR-076-AC-1..AC-6); spec/test-cases/TC-953-proof-gate-discharge-acceptance.md (new, P01..P09); spec/spec.md (contains edge); ticket STD-5'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/NFR-076
  type: reviews
---
# Gap analysis of PR #360: NFR-076-AC-1..AC-6 against TC-953 P01..P09 (STD-5)

## Summary

Ticket: STD-5. PR #360 at 21343242b1d7d6b528dabbc76e3783c794819b7f, reviewed in a detached worktree; diff scope origin/main...head. Method: gap-analysis.

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
| FND-001 | low | [minor] Behavior requires the report to name each mutant run and whether it turned the gate red, and each shared helper with its mutation result; AC-1 requires only status and discharged check count, while TC-953 P01 asserts 'both mutants are reported red', a check with no AC basis. | spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md:73-76,103; spec/test-cases/TC-953-proof-gate-discharge-acceptance.md:31 |
| FND-002 | medium | [minor] Behavior item 3 requires an expectation for an exact-kernel operation, or a runtime operation that calls one, to come from the QSpec vectors or a specification model calling no kernel operation; no AC states it and no TC-953 row offers a kernel-operation expectation built another way (P07/P08 test only helper mutation). | spec/non-functional/NFR-076-accept-proof-evidence-by-discharge.md:67-70,106 |

## Verdict

Every AC-1..AC-5 has TC-953 rows (P01-P09) with correct trace tags; AC-6 is Inspection. Two minor coverage gaps.
