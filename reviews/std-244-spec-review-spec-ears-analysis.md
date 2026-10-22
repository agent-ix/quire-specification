---
id: SR-14717
title: "EARS analysis of STD-244 (QSpec PR #345)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@073ee8c1bd971d53c52138ad16f2b50e9c7e5e87 (diff against origin/main 97bfd65); spec/functional/analysis/FR-381-check-symmetric-populations.md, spec/functional/expressions/FR-145-convert-and-query-collections.md, spec/test-cases/TC-320-symmetric-population-authoring.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-381
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-145
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-320
    type: reviews
---
# SR-14717: EARS analysis of STD-244 (QSpec PR #345)

## Summary

Ticket: STD-244. PR: quire-specification#345, head 073ee8c.

FR-381's SHALL statement is unchanged. The changed AC-2 and new AC-5 are concrete: they name the code, the cause, the node and the population. There is no status, changelog or review wording in the normative text.

Units examined:

- `FR-381-AC-2` (examined)
- `FR-381-AC-5` (examined)
- `FR-381` (examined)
- `FR-381-breaking-forms` (examined)
- `FR-381-precedence` (examined)
- `FR-145` (examined)
- `FR-145-convert` (context_only)
- `TC-320` (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Minor: the new precedence rule is normative (it decides which refusal a checker reports) but is written as passive prose ("is refused", "the refusal is") with no subject and no SHALL. State it as "When ..., the checker SHALL report ...", as QSL FR-150 does. | spec/functional/analysis/FR-381-check-symmetric-populations.md:93-101 |

## Verdict

One minor EARS finding.
