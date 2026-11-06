---
id: SR-20261104200625
title: "QSL-65 nullable owning-model gap-analysis"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-specification; QSL-65; PR 628 frozen 7db1451d; FR-336-AC-12, TC-970, corpus schema/README/fixture and tests/conformance_corpus_v1.rs"
---

# QSL-65 nullable owning-model gap-analysis

## Summary

Ticket QSL-65; PR quire-specification#628; frozen 7db1451d. Reviewer-only static inspection in the existing ticket worktree. No builds, source edits, commits, pushes, or extra worktrees. The final gate remains outstanding. Author evidence `/tmp/qsl65-qspec-focused-tests.log` reports the two focused tests passed; this is not an independent rerun. Early specification review SR-20261104200622/SR-20261104200623 passed before the Rust test commit and is committed; it was not repeated because the contract did not change after that review.

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-336-AC-12 | examined | The required `obligation_identity.model` member admits `null` and maps it to no owning model (`None`), or admits a non-empty string and maps it unchanged to the present owning model (`Some`). A missing member, an empty string, another JSON type or an unknown obligation-identity member is refused; no model identity is inferred or fabricated. |
| TC-970 | examined | Map the two member values to `Option<String>` and confirm `None` and `Some` of the exact original string, respectively; neither invents a package or function name as the model. |
| ObligationIdentity | examined | required: node_id, model, ordinal; model type: [string, null], minLength: 1; additionalProperties: false |
| ReplayRequest fixture | examined | The function obligation has model: null; the fixture is a schema example, not an executable qualification claim. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

CONDITIONAL for the stated scope.

Plan completion: not assessed

The computed matrix completed successfully: quire CLI 0.36.2 / engine 0.50.2, output `/tmp/qsl65-qspec-matrix-final.json`. FR-336-AC-12 is tagged by both new, non-ignored behavioral tests at tests/conformance_corpus_v1.rs:235 and :271. Both nullable positive arms and each required adverse partition have concrete assertions. Reverse mapping finds the changed schema and tests owned by FR-336-AC-12 and TC-970; no introduced stub or self-attested promotion was found.

This is a scoped PR gap conclusion, not a repository qualification PASS. The repository-wide matrix reports 2,400 untagged, 442 tagged, and 312 method-without-symbol criteria; FR-336-AC-8 is a pre-existing untagged consumer-adapter obligation and AC-9 is retired text still classified untagged. This PR does not supply the QSL consumer implementation or discharge unrelated repository obligations. No plan was supplied. Optional repository-wide semantic review was not requested; ordinary code/test faithfulness for this diff was assessed as code-review. Whole-tree Quire validation has 113 pre-existing structural failures, recorded by the early spec review; changed FR/TC validation passed.

## Coverage

| Test | Criterion | Binding |
| --- | --- | --- |
| replay_obligation_model_preserves_absence_and_present_identity | FR-336-AC-12 | correct |
| replay_obligation_model_refuses_missing_malformed_and_unknown_members | FR-336-AC-12 | correct |
