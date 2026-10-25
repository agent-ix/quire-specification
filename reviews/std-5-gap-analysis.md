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

## Dispositions

Round 1, checked at head 9f74956915d5c764a6a8b35f227efcc4cdcec7b4 (fix commit 9f74956, contains origin/main). Gates: quire validate --scope . spec/**/*.md fails 116 documents, identical to origin/main, none in NFR-076, TC-953 or spec/spec.md; cargo test --test checked_package_v2 --test backend_provider_v1: 126 + 23 passed. QSL ADR-011 §2.3 (lines 429-485) is unchanged between qsl-lane2 and QSL origin/main.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | After (AC-1): "... passes, and its report gives each module the status `discharged` and its discharged check count, names each mutant run in the module with whether it turned the gate red, and names each shared helper with the result of its mutation." TC-953 P01's "the report names `m1` and `m2` and states each turned the gate red, and names no shared helper (NFR-076-AC-1)" now has an AC basis. |
| FND-002 | fixed | New AC-6: "An expectation for an exact kernel operation, or for a runtime operation that calls one, that is derived from anything other than the QSpec operation vectors or a checked-in specification model that calls no exact kernel operation fails the gate, and the report names the operation." Covered by TC-953 P10 (expectation calling f1, calling another kernel operation: fail naming f1; checked-in model: pass). Former Inspection AC is now AC-7 and cites AC-1 to AC-6. |
