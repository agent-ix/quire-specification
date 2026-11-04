---
id: SR-20261103170102
title: 'Gap analysis of PR #616: TC-185 D26/D27, TC-982, TC-983 against FR-140, FR-488, FR-489 (QSL-1619, QSL-1541, QSL-1542)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@56ed7477; git diff origin/main(78dcf2a9)...56ed7477 (PR #616): TC-185, TC-982, TC-983, tests/qualification_vectors.rs, FR-140-AC-6, FR-140-AC-7, FR-488-AC-1..7, FR-489-AC-1..6; tickets QSL-1619, QSL-1541, QSL-1542'
review_set: subset
date: '2026-11-04'
relationships:
- target: ix://agent-ix/quire-specification/TC-185
  type: reviews
- target: ix://agent-ix/quire-specification/TC-982
  type: reviews
- target: ix://agent-ix/quire-specification/TC-983
  type: reviews
---

## Summary

Ticket: QSL-1619. Plan completion: not assessed. Manual mapping of changed ACs to TC rows and vectors. quire matrix lists every FR-488 and FR-489 AC as untagged, the same as FR-140 and every other spec-only TC in this repo (no test in tests/ is tagged to TC-317 or TC-982); this is the repo convention for downstream-implemented vectors and is not a finding. tests/qualification_vectors.rs counts D rows only: TC-185 has 27 D rows (25 plus D26 and D27), the total 169 equals the sum of the five counts, and cargo test --locked --offline exits 0.

FR-140-AC-6 (charge positions, all-zero discarded digits) maps to D13 and D26. FR-140-AC-7 (Rational to Decimal conversion follows the division schedule) maps to D24, now through a QSL source form. FR-488-AC-1 maps to TC-982 case 1, AC-2 to cases 2 and 3, AC-3 to cases 4 and 5, AC-4 to case 6, AC-5 to case 7, AC-6 to case 8 and AC-7 to case 9. FR-489-AC-1 maps to TC-983 case 2, AC-2 to case 3, AC-3 to case 1, AC-4 to case 4, AC-5 to case 5 and AC-6 to case 6. Every AC has a row. One gap: the FR-140 paragraph on non-member source operands has no AC and no vector (FND-001). TC-982 case 3 expected result is a disjunction (FND-002).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| TC-185 D26, D27 | spec/test-cases/TC-185-exact-decimal-semantics.md:52 | examined |
| FR-140-AC-6 | spec/functional/type-model/FR-140-evaluate-exact-decimals.md | examined |
| FR-140-AC-7 | spec/functional/type-model/FR-140-evaluate-exact-decimals.md | examined |
| tests/qualification_vectors.rs ROWS and totals | tests/qualification_vectors.rs:37 | examined |
| TC-982 cases 1-9 | spec/test-cases/TC-982-implementation-temporal-guarantee-through-checked-correspondence.md | examined |
| TC-983 cases 1-6 | spec/test-cases/TC-983-composition-carries-liveness-and-fairness-premises.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No AC or vector covers a non-member decimal literal supplied in source (for example decimal(150,2) into a scale-0 type) | spec/functional/type-model/FR-140-evaluate-exact-decimals.md:139 |
| FND-002 | medium | TC-982 case 3 expected result is a disjunction that no outcome can fail | spec/test-cases/TC-982-implementation-temporal-guarantee-through-checked-correspondence.md:46 |
