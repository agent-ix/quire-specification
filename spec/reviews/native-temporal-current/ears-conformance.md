---
id: SR-307
title: "EARS review of native temporal lifecycle correspondence"
type: SpecReview
analysis: ears-conformance
scope: "Normative statements in FR-090 through FR-095 and NFR-040"
review_set: all
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-091, type: reviews }
---
## Summary

The temporal requirements use event, state, unwanted-behavior and ubiquitous
clauses with named subjects and observable responses. Scoped strict Quire
validation reports no EARS or acceptance-criterion warning after remediation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved during review: missing facts now name `the evaluator` and their exact unavailable/retained outcomes rather than relying on an implied subject. | FR-091 Behavior |
| FND-002 | medium | Resolved: fixed-sample definition refusal and runtime incompleteness are separate observable clauses. | FR-090 Behavior |
| FND-003 | low | No remaining EARS grammar finding exists in the E-owned scope; grammar conformance does not establish implementation or profile adoption. | Quire 0.31.0 strict output |

## Clause check

| Clause family | Result |
| --- | --- |
| Event | Admission, activation, evaluation, lowering and later contradiction triggers name the responding frontend/evaluator/bridge. |
| State | Open/closed scope, missing fact/history/sample, progress and settlement states name exact outcomes. |
| Unwanted behavior | Inference, false padding, false closure, rollback, coercion, substitution and byte rewriting are explicitly prohibited. |
| Verifiability | Every changed normative boundary maps to an acceptance criterion and planned TC-113, TC-115, TC-116 or TC-117 mutation. |
