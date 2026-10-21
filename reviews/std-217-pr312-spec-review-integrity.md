---
id: SR-13637
title: 'Spec review of QSpec PR #312: FR-468, integrity'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@3fa26ceda; PR #312 diff origin/main...HEAD: spec/functional/analysis/FR-377-decide-model-refinement.md, spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md, spec/spec.md, spec/test-cases/TC-950-per-step-refinement-queries-and-certificates.md; ticket STD-217'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-468
  type: reviews
- target: ix://agent-ix/quire-specification/FR-377
  type: reviews
---

## Summary

Ticket: STD-217. PR quire-specification#312, head 3fa26cedad15da7e78f66ccf84060f4912ea408d.

Integrity lens: consistency of FR-468 with FR-377 Engines, FR-375's `steps` order, FR-290's per-step row, QSL ADR-020 RE-2 and QSV FR-123; link and edge integrity. FR-468's relationships (US-011, FR-377, FR-379, FR-375, FR-290) resolve; TC-950 `verifies` FR-468; spec.md has the row and the `contains` edge. FR-468 does not reference FR-354/FR-331 (base FND-003).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-468 | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Inputs | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-468 Step query | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md | examined |
| FR-377 Dependencies/FR-468 | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| spec.md FR-468 row | spec/spec.md | examined |
| TC-950 | spec/test-cases/TC-950-per-step-refinement-queries-and-certificates.md | examined |
| FR-377 Engines | spec/functional/analysis/FR-377-decide-model-refinement.md | context_only |
| QSL ADR-020 RE-2 | quire-spec-language spec/decisions/ADR-020-refinement-mappings-between-qsl-models.md | context_only |
| QSV FR-123 Use case | quire-semantic-value spec/functional/FR-123-step-simulation-record-and-obligation.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The Inputs put the exclusion on the wrong subject: 'whose concrete subject has no hidden field, history field or `any` row'. Hidden fields are abstract fields that the declaration leaves unmapped (FR-378), and history fields and `any` rows are rows of the declaration. The Description and FR-377 Engines both say 'a declaration with no hidden field, no history field and no `any` row'. Fix: 'An admitted refinement declaration with no hidden field, history field or `any` row'. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:26 |
| FND-002 | low | FR-377 now lists FR-468 under Dependencies but has no `relationships` edge to it. Every other FR in that list (FR-375, FR-376, FR-378, FR-379, FR-181, FR-013, FR-161, FR-290) has one, and FR-377 and FR-379 already reference each other, so a cycle is no reason to leave it out. Fix: add `- target: ix://agent-ix/quire-specification/FR-468` with `type: references` to FR-377's frontmatter. | spec/functional/analysis/FR-377-decide-model-refinement.md:245 |
| FND-003 | low | It is not clear which invariants the step queries assume. FR-468 says 'the concrete invariants' and, in Inputs, 'the concrete subject's invariant clauses'. QSL ADR-020 RE-2 and QSV FR-123 (PR #59, same ticket) are narrower: the invariant clauses whose context is an object type of the concrete model, because the declaring package can also hold the abstract model (ADR-020 RM-1). The abstract model's invariants are not hypotheses. Fix: use the RE-2 wording in the Step query bullet and in Inputs. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:39 |

## Verdict

Mergeable on this lens alone; three low wording and link findings.

## Dispositions

Round 1, head a56c80c5c.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed in a56c80c5c: An admitted refinement declaration ([FR-375](FR-375-declare-model-refinement.md)) with no hidden field, history field or `any` row, the invariant clauses whose context is an object type of the concrete model, and a proof backend's outcome and refutation certificates for the queries. ## Outputs For e |
| FND-002 | fixed | Fixed in a56c80c5c: relationships: ... - target: ix://agent-ix/quire-specification/FR-468 type: references |
| FND-003 | fixed | Fixed in a56c80c5c: Every query carries the *hypotheses*: the invariant clauses whose context is an object type of the concrete model (the abstract model's invariants are not hypotheses), the same list the per-step simulation record carries (quire-semantic-value FR-123). - **Initial query.** The negation of: for every concrete initial state satisfying the hypotheses, the state maps to an abstract initial state. It ca |
