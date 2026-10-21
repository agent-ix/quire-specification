---
id: SR-13701
title: 'EARS review of QSpec PR #323: FR-322 integer literal operands statement (QSL-940)'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@de9af077e; git diff origin/main...de9af077e (PR #323): spec/objects/interfaces/FR-322-checked-package-artifact.md; ticket QSL-940'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
---

## Summary

Ticket: QSL-940. PR: quire-specification#323, head de9af077e. Method: spec-review with the EARS sub-analysis, over the one new requirement statement. The first sentence is ubiquitous EARS with a named subject: "The producer SHALL write ...". quire validate reports no new EARS warning for it. Criterion-strength analysis was not run, because no Jev client is available. FR-322-AC-92 was judged for testability by mutation in SR-13699.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 "Integer literal operands" paragraph | spec/objects/interfaces/FR-322-checked-package-artifact.md:771-779 | examined |
| FR-322-AC-92 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1840 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "as an inline `literal` term, a singleton domain with `value_kind` `integer`" brings the replay obligation's domain (QSL FR-357's singleton `range`) into the wire grammar. In FR-322 a `literal` term has no domain member, so a reader may look for one. "an inline `literal` term with `value_kind` `integer`" says what is meant. | spec/objects/interfaces/FR-322-checked-package-artifact.md:774-775 |
| FND-002 | low | The exception sentence has no modal ("... are graph children"). Its list "composite, Boolean or non-integer equality" repeats itself, because Boolean equality is a non-integer equality, and the whole list already falls under "a literal in any other position". QSL FR-093 says "composite or non-integer equality". | spec/objects/interfaces/FR-322-checked-package-artifact.md:776-779 |

## Verdict

The statement is well formed. It has one subject, one modal and six named operators, and the exceptions are stated. Two wording nits, both low.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
