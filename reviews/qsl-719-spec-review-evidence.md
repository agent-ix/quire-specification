---
id: SR-7837
title: "Evidence analysis of allInstances coverage accounting"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-specification@2923f8a0debaee0102d8889aed52925c9174bf2f; proposals/quire-v1/definitions/value-accounting.md"
review_set: subset
---

## Summary

Ran the declared-method advisor for FR-153 and FR-151 obligations. FR-153-AC-1 through AC-9 and FR-151-AC-1 through AC-16 author `Test`; the advisor reports no authored-method mismatch or inconclusive recommendation for these criteria. FR-153-AC-5 directly covers complete subtype populations and withholding partial results. No evidence-method defect found for this prose clarification.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found | - |

## Scope

- {id: FR-153-AC-1..9, path: "spec/functional/type-model/FR-153-query-closed-model-environments.md", role: examined, excerpt: "Each FR-153 acceptance criterion authors Test; quoin advise reports no mismatch or inconclusive recommendation for AC-1 through AC-9."}
- {id: FR-151-AC-1..16, path: "spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md", role: examined, excerpt: "Each FR-151 acceptance criterion authors Test; quoin advise reports no mismatch or inconclusive recommendation for AC-1 through AC-16."}
- {id: FR-153-AC-5, path: "spec/functional/type-model/FR-153-query-closed-model-environments.md", lines: "97-99", role: examined, excerpt: "All-instances includes closed subtype populations exactly once and emits no partial population when closure is unknown."}
