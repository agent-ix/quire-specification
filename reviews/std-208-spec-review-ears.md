---
id: SR-13325
title: "EARS review of QSpec PR #288 (STD-208)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@07eb2ff; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md; ticket STD-208"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-193
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
---

## Summary

Ticket: STD-208. PR: quire-specification#288 at 07eb2ff. Requirement-statement grammar of the new normative prose. The PR does not change FR-193's or FR-331's statement sentence. The Solver budgets bullets use SHALL except where noted.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-193-budgets-steps | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-time | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-identity | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-331-hybrid-solver-budgets | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The rounding rule that FR-193-AC-9 tests is stated in the indicative with no responsible system: "A caller that supplies a fractional duration has it rounded up ... when the request is admitted". Write it as an event-driven SHALL naming who rounds, e.g. "When a request with a fractional `max_solver_time` is admitted, the <request writer\|provider> SHALL round it up to the next whole second". The same applies to "Two requests that differ only in `max_solver_time` have equal request identity" in the next bullet. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:87-97 |

## Verdict

One low EARS finding on the rounding sentence; the remaining new bullets are well-formed SHALL statements.

## New findings (disposition pass 2)

Against head 607417f.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The new defaults bullet opens with a garbled trigger: "When the caller sets a budget none". Write it as "When the caller sets no value for a budget, the request writer SHALL omit the member ...". | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:103-108 |

## Dispositions

Round 1, against head 5a9fc8d.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 5a9fc8d: event-driven SHALL naming the request writer; identity sentence uses SHALL |
| FND-002 | fixed | fbab2f2: trigger reads "When the caller sets no value for a budget" |
