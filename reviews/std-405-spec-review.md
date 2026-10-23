---
id: SR-16278
title: "spec-review of nonzero quantity literal divisor (quire-specification PR 405)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@eaed9b73; spec/functional/expressions/FR-146-check-total-pure-functions.md; spec/test-cases/TC-191-total-pure-function-checking.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
---

# Spec review of nonzero quantity literal divisor (quire-specification PR 405)

## Summary

Ticket: STD-405. One spec-review pass over the PR's own delta (`git diff origin/main...eaed9b73`), checked for the no-drop/no-narrowing/no-deferral rule, testable vectors (key vectors derived by hand), id uniqueness across open PR heads, and schema/vector consistency.

Examined, clean:

- Hand-derived: (14) `d / 1 s` with d = 1/2 m gives 1/2 m/s; (15) -1/2 m/s; (17) `t / 1 s` cancels to dimensionless 1; (16) and (18) `unproved-nonzero`. These match. The `let` restriction matches the Decimal literal rule.
- Gates: quire validate 86 (checked at the #403 head, which includes this head unchanged); cargo green there too.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The ticket's case, `d / 3 ms`, has no vector. Every admitted divisor has scale 1 and is the root unit, so the sentence "the divisor's composed scale applied exactly" is never tested. Neither is a decimal magnitude (`0.5 s`) or the zero decimal `0.0 s` named in the prose. Fix: add `d / 3 ms` with d = 3/2 m, which gives 500 m/s in root units, plus a decimal divisor and `0.0 s`. | spec/test-cases/TC-191-total-pure-function-checking.md | missing-requirement |
| FND-002 | medium | The interval rule assumes quantity types with a bounded magnitude (P12's `magnitude type Q`). FR-142 "Quantity type reference" says `Quantity<u>` admits "no bound on the magnitude; a bounded magnitude is not a complete-V1 type form". So the range obligation of an admitted quotient has no defined interval. P12 (12) and (13) already use the form; this PR adds admitted cases on top of it. Fix: either state the bounded-magnitude quantity form in FR-142, or define the quotient's range obligation without magnitude bounds. | spec/functional/expressions/FR-146-check-total-pure-functions.md | wrong-requirement |
| FND-003 | low | The text does not say whether `d / -1 s` (unary minus on an integer magnitude) is a quantity literal divisor or a negation, which is not a literal. State which. | spec/functional/expressions/FR-146-check-total-pure-functions.md | wrong-requirement |

## Verdict

Mergeable after FND-001, FND-002 (medium) are fixed or dispositioned by the lead.
