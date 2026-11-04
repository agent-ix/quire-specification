---
id: SR-20261103170101
title: 'Spec review of PR #616: decimal vector source forms (QSL-1619), FR-488 (QSL-1541), FR-489 (QSL-1542)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@56ed7477; git diff origin/main(78dcf2a9)...56ed7477 (PR #616): FR-140 operand-membership paragraph, TC-185 D13/D18/D24/D26/D27, FR-488, TC-982, FR-489, TC-983; tickets QSL-1619, QSL-1541, QSL-1542'
review_set: subset
date: '2026-11-04'
relationships:
- target: ix://agent-ix/quire-specification/FR-140
  type: reviews
- target: ix://agent-ix/quire-specification/FR-488
  type: reviews
- target: ix://agent-ix/quire-specification/FR-489
  type: reviews
---

## Summary

Ticket: QSL-1619 (pointer on QSL-1541, QSL-1542). Manual EARS, integrity, failure-domain and consistency passes ran inside this review. The criterion-strength method (Jev) was not run; each new AC was read for whether it can fail. quire validate exits 0 on all six changed files. The repo-wide baseline exits 3 on FR-200, FR-220 and FR-299 only, the same as main. No id collides with main or with open PR #600.

Decimal vectors (QSL-1619): D24, D26 and D27 were re-derived by hand from FR-140 and value-accounting.md and are correct. D26: membership of (100,2) and (3,0) in Decimal[-9,9;0,0] holds (value 1 and 3). Charges are operands 7 bits, 3 digits, 2 occurrences; scale-expansion shift 0 with sbits 7; arithmetic 7+2 = 9 bits and 3+1 = 4 digits; no rounding because discarded digits are 00; result-retain (3,0) with 2 bits. Four work units and one result unit. Limit integer_bits 8 gives consumed 7, next_charge 9 at decimal.arithmetic; work_units 3 gives consumed 3, next_charge 1 at decimal.result-retain. D27: (1200,3) has value 1.2, member as (12,1); shift abs(1+0-3) = 2; N = 1200, D = 7*10^2 = 700, reduced 12/7; exact value 6/35; coefficient 1.714 rounds nearest-even to 2; operands 11 bits and 4 digits; scale-expansion sbits(7,2) = 10; arithmetic max = 11 bits and 4 digits; rounding charged with the same amounts; five work units. scale_expansion 1 gives consumed 0, next_charge 2. D24 numbers are unchanged and still correct, and 3 is not a member of Decimal[0,100;2,2] (c* = 300). The D13 and D18 operands are not members (s* = 2 > 0, s* = 3 > 1), so the evaluator-boundary label is accurate. D26 and D27 reproduce the property each vector pinned (retained versus normalized scale) with member operands: D26 distinguishes bits(100)+bits(3) = 9 from bits(1)+bits(3) = 3 and D27 distinguishes shift 2 from normalized shift 0. The labelling keeps the behavior and is a legitimate disposition. Row counts: TC-185 has 27 D rows, the total of 169 matches, and cargo test --locked --offline exits 0.

FR-488 and FR-489 add typed premise lists, settlement tables and ACs that can fail. Links resolve. Citing FR-360 for the result vocabulary is right: FR-380 and FR-341 cross-references to the same vocabulary are stale in FR-380 itself and are outside this diff.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-140 operand membership paragraph | spec/functional/type-model/FR-140-evaluate-exact-decimals.md:139 | examined |
| TC-185 D13, D18 | spec/test-cases/TC-185-exact-decimal-semantics.md:39 | examined |
| TC-185 D24 | spec/test-cases/TC-185-exact-decimal-semantics.md:50 | examined |
| TC-185 D26, D27 | spec/test-cases/TC-185-exact-decimal-semantics.md:52 | examined |
| FR-488 | spec/functional/ir-proof/FR-488-attach-a-temporal-guarantee-to-implementation-rust.md | examined |
| FR-488-AC-1..7 | spec/functional/ir-proof/FR-488-attach-a-temporal-guarantee-to-implementation-rust.md | examined |
| TC-982 | spec/test-cases/TC-982-implementation-temporal-guarantee-through-checked-correspondence.md | examined |
| FR-489 | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md | examined |
| FR-489-AC-1..6 | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md | examined |
| TC-983 | spec/test-cases/TC-983-composition-carries-liveness-and-fairness-premises.md | examined |
| FR-176 discharge fixed point | spec/functional/protocol/FR-176-compose-assume-guarantee-contracts.md | context_only |
| FR-331 closed inconclusive causes | spec/objects/interfaces/FR-331-backend-provider-envelope.md:678 | context_only |
| FR-380 settlement | spec/functional/analysis/FR-380-transfer-claims-through-refinement.md:102 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-489 and FR-488 emit inconclusive causes premise-missing, premise-mismatch and circular-reasoning that are not in the closed cause vocabulary (FR-331, backend-provider-v1 schema.json) | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md:90 |
| FND-002 | medium | FR-489 settles a guarantee over another subject as premise-mismatch and a premise whose subject differs as invalid-value refusal; AC-2 and AC-6 and TC-983 cases 3 and 6 overlap | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md:99 |
| FND-003 | medium | FR-489 settles an unjustified liveness cycle inconclusive/circular-reasoning and adds a well-founded-measure route, while FR-176 refuses it and accepts only a lattice fixed point; FR-489 says FR-176 is reused unchanged | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md:75 |
| FND-004 | medium | FR-488 settlement table has no row for an unproved model claim or unproved progress item and none for a proved safety clause; the Description requires the progress premise for every settlement | spec/functional/ir-proof/FR-488-attach-a-temporal-guarantee-to-implementation-rust.md:96 |
| FND-005 | medium | TC-982 case 3 accepts premise-missing or premise-unproved, so no outcome can fail it; FR-488-AC-2 names no cause for the per-step-only case | spec/test-cases/TC-982-implementation-temporal-guarantee-through-checked-correspondence.md:46 |
| FND-006 | medium | FR-140 new paragraph states that no source form feeds a non-member operand but gives no disposition for a non-member source literal, no AC, and no vector; the evaluator-boundary contract is stated only by D13 and D18 | spec/functional/type-model/FR-140-evaluate-exact-decimals.md:139 |
| FND-007 | low | FR-489 trusted-assumption row gives label "per rule", which no test can compare | spec/functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md:91 |
| FND-008 | low | FR-140 new paragraph is not wrapped like its neighbours and TC-185 D24 repeats its conversion sentence | spec/test-cases/TC-185-exact-decimal-semantics.md:50 |
