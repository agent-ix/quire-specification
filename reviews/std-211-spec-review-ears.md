---
id: SR-13324
title: "EARS review of QSpec PR #287 (STD-211)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@1e274e7; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md, spec/functional/analysis/FR-377-decide-model-refinement.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-376
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-377
    type: reviews
---

## Summary

Ticket: STD-211. The PR edits no FR statement sentence; it adds normative body paragraphs and criteria. `quire validate` reports no EARS diagnostic on the three FRs. The new paragraphs follow FR-375's existing declarative style; two clauses have more than one reading.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-option-rows | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-lifting | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-relationships | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-value-profile | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-377-precedence | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |
| FR-377-option-map | spec/functional/analysis/FR-377-decide-model-refinement.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | "a concrete `?` field is read in a row as `if present(self.f) then Some(value(self.f)) else none`" has no actor and two readings: an implicit rule that types `self.f` (concrete `?`) as `Option<T>` in a row, or guidance that the author must write the `if present` form. Under FR-143 a bare `self.f` of a `?` field is `ill_typed` outside `present`/`value`, so the first reading changes FR-143 inside refinements without saying so; TC-310 mutation 20 tests only the written form. Say which: "A row reads a concrete `?` field only through `present` and `value` (FR-143), for example ...", or "Inside a refinement row, `self.f` of a `?` field has type `Option<T>` ...". | spec/functional/analysis/FR-375-declare-model-refinement.md:270-271; spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md:162-170 |
| FND-002 | low | "`none` takes the option type its use requires" gives no rule when the use requires none (an `if` arm against `Some(e)`, a comparison, an argument), and no rule for `none` against an abstract `Option<Reference<T_A>>` field, which the nested-reference rule below may or may not catch. State the expected-type source (the row's abstract field type, the other `if` arm) and the refusal when there is none. | spec/functional/analysis/FR-375-declare-model-refinement.md:267-268 |

## Verdict

Mergeable on this lens after FND-001; FND-002 is a precision gap. Clean: the support-set refusal sentence ("the checker SHALL refuse ... once for each field row of nested-reference type (at the row's expression) and once per refinement that has an abstract relationship (at the abstract alias of the header)") is determinate with code, cause, locus and count; FR-377's failure precedence is stated as ordered, exclusive conditions; FR-376's sum bound and `only-not-one` sentence are determinate.
