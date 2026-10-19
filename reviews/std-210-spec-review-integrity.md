---
id: SR-13311
title: "Integrity review of QSpec PR #286 (STD-210)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@5f25fd2; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md, spec/test-cases/TC-194-complete-equality-matrix.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-149
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-194
    type: reviews
---

## Summary

Ticket: STD-210. Checks that every QSpec statement of the ordering rule agrees after the change. Repo-wide grep for ordering-operator typing ("ordering comparison", "supplies/supply no expected type", `n + 1 <=`, ordering expressions paired with `ill_typed`/`type-mismatch`) finds the rule only in FR-149 prose (line 221-222), FR-149-AC-21 and TC-194 E35. No other FR, AC or TC (FR-044, FR-146, FR-206, FR-375/376/377, TC-189, TC-397) restates or contradicts it. QSL FR-104 and FR-381 on origin/main state the same acceptance.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-149 ordering prose | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | examined |
| FR-149-AC-21 | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | examined |
| TC-194 E35 | spec/test-cases/TC-194-complete-equality-matrix.md | examined |
| FR-376-AC-5 (open PR #287) | spec/functional/analysis/FR-376-evaluate-population-valued-mapping-expressions.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | E35's closing clause says "an over-extension of the equality expected type to ordering would give `unproved-range`", which holds for the two arithmetic vectors (`n + 1`, `earlier.tally + 1`) but not for `n < later.tally` and `n >= later.tally`: a path operand gets no expected type under the equality rule either, so an implementation that applied the equality's refusal to ordering would give `ill_typed`/`type-mismatch` there, not `unproved-range`. Scope the clause to the arithmetic vectors and say the path vectors guard against a `type-mismatch` over-extension. | spec/test-cases/TC-194-complete-equality-matrix.md:70 |

## Verdict

Mergeable on this lens. The prose, AC-21 and E35 agree on all four operators, both operand orders and the absence of coercion. AC-21's new acceptance clause matches the prose sentence it restates, and E35's count ("the four ordering comparisons") matches its four added vectors.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | b2aebb1 adds a fifth ordering vector (`later.tally > n`) to E35's input, but the expected result still says "the four ordering comparisons are accepted". The input now lists five (`n + 1 <= later.tally`, `earlier.tally + 1 <= later.tally`, `n < later.tally`, `n >= later.tally`, `later.tally > n`). Change "four" to "five". | spec/test-cases/TC-194-complete-equality-matrix.md:70 |

## Dispositions

Round 1, against head b2aebb1.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | b2aebb1: E35 now says the over-extension "would give `unproved-range` for the two arithmetic vectors and `ill_typed`/`type-mismatch` for the path vectors `n < later.tally`, `n >= later.tally` and `later.tally > n`" |
