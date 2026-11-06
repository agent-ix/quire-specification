---
id: SR-20261104200624
title: "QSL-65 nullable owning-model code-review"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-specification; QSL-65; PR 628 frozen 7db1451d; FR-336-AC-12, TC-970, corpus schema/README/fixture and tests/conformance_corpus_v1.rs"
---

# QSL-65 nullable owning-model code-review

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

PASS for the stated scope.

Rust review is folded into code-review. The schema change adds only the null arm, retains the required member and closed obligation object, and leaves string minLength in force. The Rust tests execute the real authored ReplayRequest schema through the existing definition_validator and registry; accepted values decode through serde_json into Option<String>, asserting None and exact Some text including decomposed Unicode. Each adverse mutation starts from an admitted fixture and changes only the targeted dimension. There are no new production carrier types, copied contracts, unsafe code, nondeterministic resources, new dependencies, modified CI, or narrowed gates. The two tests carry paired TC-970 / FR-336-AC-12 traces. The fixture and README explicitly avoid claiming executable replay qualification; actual QSL consumer qualification remains a separate leg.

## Coverage

Plan completion: not assessed

| Test | Criterion | Binding |
| --- | --- | --- |
| replay_obligation_model_preserves_absence_and_present_identity | FR-336-AC-12 | correct |
| replay_obligation_model_refuses_missing_malformed_and_unknown_members | FR-336-AC-12 | correct |
