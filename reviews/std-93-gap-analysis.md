---
id: SR-14615
title: 'Gap analysis of PR #348: FR-322-AC-96 against TC-233 DP-01..DP-07 (STD-93)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@c2a9875; git diff origin/main(97bfd65)...c2a9875 (PR #348, branch std-fr322-std-93): FR-322 `dependencies` property, FR-322-AC-96, TC-233 DP-01..DP-07, native-diagnostics.md step 3, checked-package-v2 README, positive-all-families.json, positive-operation-identities.json, dependency-selection-vectors.json; ticket STD-93'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
---

## Summary

Ticket: STD-93. Plan completion: not assessed. This is a manual mapping of new and changed ACs to TC rows. FR-322-AC-96 (new) maps to TC-233 DP-01 (every node of every positive fixture, `package_id` recomputes), DP-02 (`7916…`, `eda3…` list `8a2a…`), DP-03 (`652c…` lists `07f6…`, `1ffe…` lists `0ec8…` alone), and DP-04 to DP-07 (the four refusal mutations at `/dependencies`). No existing AC changed. No test is tagged to FR-322-AC-96 yet. The PR lists it as STD-93 remaining work, and the test reader joins `dependencies` only for application, nominal, frame and carrier nodes today. The fixture claims of DP-01 to DP-03 were checked by independent recompute (see SR-14613).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-AC-96 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1895 | examined |
| TC-233 Node dependency vectors DP-01..DP-07 | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:374 | examined |
| positive fixtures and dependency-selection-vectors.json | proposals/checked-package-v2/fixtures/positive-all-families.json, positive-operation-identities.json, dependency-selection-vectors.json | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. Each clause of FR-322-AC-96 has a DP row, and the fixture-level clauses hold under independent recompute.
