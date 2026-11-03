---
id: SR-20261103145403
title: "QSL-1536 r1 EARS analysis of PR #609 (added SHALL statements)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification; PR #609 head cf327420; SHALL statements added or edited in FR-154 (Rational value types) and FR-142 (Carried quantity checker bullet)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
---

# QSL-1536 r1 EARS analysis of PR #609

## Summary

Ticket: QSL-1536. Four statements were examined:

- FR-154 "Intake SHALL read its bounds from its own `constraints` only …" (ubiquitous)
- FR-154 "Otherwise intake SHALL refuse the first defect …" (unwanted-behaviour, with an ordered list)
- FR-154 "A value supplied to execution … SHALL be a rational in lowest terms …"
- FR-142 "The checker SHALL read `Quantity<u, C>` … SHALL refuse … when `C` is not a bounded `Int[..]`, `Decimal[..]` or `Rational[..]`" (unwanted-behaviour)

The quire grammar check raises no EARS warning on FR-154 or FR-142. The only EARS warnings, at FR-322 line 104, are pre-existing.

## Verdict

The statements conform, apart from one low finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "A value supplied to execution at a member of a bounded Rational value type … SHALL be a rational in lowest terms …" makes the input the subject of `SHALL`, not the system. Its response is in the next sentence ("refuses `invalid_runtime_input`"). Rewrite it as an unwanted-behaviour statement: "If a value supplied … is not … then execution SHALL refuse `invalid_runtime_input`/`invalid-value` naming the member path and SHALL NOT reduce, round or convert it." | spec/functional/type-model/FR-154-admit-domain-package-model.md:529-537 |

## Dispositions

| ID | Disposition | Resolution |
| --- | --- | --- |
| FND-001 | fixed | Rewritten as "If a value supplied … is not …, then execution SHALL refuse it `invalid_runtime_input`/`invalid-value` … and SHALL NOT reduce, round or convert it." |
