---
id: SR-20261104200622
title: "QSL-65 model-free corpus obligation specification review"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-specification; QSL-65; PR 628 frozen 3b20dbae; FR-336 replay adapter mapping and AC-12; TC-970 owning-model procedure; corpus README, ReplayRequest/ObligationIdentity schema and replay-request fixture"
---

# QSL-65 corpus owning-model specification review

## Summary

PASS for proceeding to the Rust conformance-test phase. This is the repository-required specification review before implementation, not the final mixed-PR review or a qualification claim. The selected review set is base plus failure-domain. No active AssuranceProfile document occurs in this repository; the installed catalog contains that type, but there is no applicable authored profile requiring an enforced run or human acceptance.

The changed FR-336 and TC-970 validate. The explicit full-tree command, `quire validate --scope /home/peter/dev/worktrees/titan-gate-qspec-main 'spec/**/*.md'`, exits 3 with 113 structural document failures outside these changed documents; its output is retained at `/tmp/qsl65-qspec-spec-review/full-tree-validation.log`. The two edited specification documents do not occur among those failures. No unrelated specifications were edited. Computed implementation coverage was not assessed: new Rust tests are intentionally pending this review.

## Examined scope

| Unit | Role | Excerpt or exact schema constraint |
| --- | --- | --- |
| FR-336 replay adapter mapping | examined | The corpus adapter SHALL map `obligation_identity.model: null` to the existing typed obligation identity's absent owning model (`None`). |
| FR-336-AC-12 | examined | The required `obligation_identity.model` member admits `null` and maps it to no owning model (`None`), or admits a non-empty string and maps it unchanged to the present owning model (`Some`). A missing member, an empty string, another JSON type or an unknown obligation-identity member is refused; no model identity is inferred or fabricated. |
| TC-970 owning-model procedure | examined | Map the two member values to `Option<String>` and confirm `None` and `Some` of the exact original string, respectively; neither invents a package or function name as the model. |
| ObligationIdentity schema | examined | `required` retains `node_id`, `model`, `ordinal`; `model` has `type: [string, null]` and `minLength: 1`; `additionalProperties: false` remains. |
| ReplayRequest schema | examined | Only the request's obligation identity refers to the changed `$defs.ObligationIdentity`; the required request members and unrelated nested schemas remain unchanged. |
| Corpus README and replay-request fixture | examined | The fixture carries `model: null` and remains explicitly a schema example, not an executable qualification vector. |
| FR-201 absence | context_only | An absent component compares only to absence and is never defaulted. |
| QSL public ObligationIdentity | context_only | `model: Option<ModelIdentity>` represents an absent owning model; the function call-site constructs `ObligationIdentity::new(function, None, 0)`. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

The change repairs the corpus encoding of an existing optional identity component. JSON Schema applies `minLength` to strings and accepts null through the new type arm. Keeping `model` in `required` distinguishes a deliberate absence from an omitted member; the adapter must validate that distinction before mapping to an optional Rust value. The nonempty string path preserves the exact identity text, and neither path introduces model inference, digest comparison, a carrier, a package pin, or a FR-071 product-wire change.

FR-336-AC-12 is unique, method-only verified by Test, and TC-970 assigns both positive arms plus missing, empty, Boolean, number, array, object, and unknown-member adverse cases to it. The criterion can fail independently for schema closure, optional-component mapping, or identity substitution. Existing unrelated criteria and relationships are unchanged. No state-transition or graph-algorithm coverage applies to this scalar sum encoding. Rust coverage, source execution, and the final gate remain obligations of the next phase.
