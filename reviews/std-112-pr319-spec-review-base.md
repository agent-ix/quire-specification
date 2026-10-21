---
id: SR-13667
title: 'Spec review of QSpec PR #319: FR-142 unit-graph topology causes, base checklist'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@820027e65; PR #319 diff origin/main...HEAD: spec/functional/type-model/FR-142-evaluate-quantities-and-units.md,
  spec/test-cases/TC-187-quantity-and-unit-conversion.md; ticket STD-112'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-142
  type: reviews
- target: ix://agent-ix/quire-specification/TC-187
  type: reviews
---

## Summary

Ticket: STD-112. PR quire-specification#319, head 820027e656308de9cd1b790583c8cea0e3b38ac1.

Base checklist over the FR-142 unit-graph refusal rule, FR-142-AC-5, the new AC row and TC-187 U06 and U11 to U11f. The cause names exist in FR-272 and native-diagnostics with the stated codes (`invalid-value`, `incompatible-definition`, `conflicting-definition` and `definition-cycle` under `invalid_package`; `missing-name` under `missing_declaration`); AC-5 correctly drops the topology cases; no status or owner wording; no new hash, digest or pin. `quire validate --scope . 'spec/**/*.md'` (quire 0.36.2, engine 0.50.2) reports 134 failed documents on main and on the PR head, the same set; FR-142 and TC-187 validate clean. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (114 and 14 tests).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-142 unit-graph refusal statement | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142 missing-root sentence | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142 check-order sentence | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142-AC-5 | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142-AC-13 (new, line 248) | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| TC-187 U06 | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11 | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11a | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11b | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11c | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11d | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11e | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11f | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| PR #319 body | PR #319 description | examined |
| FR-142 Dimension and conversion algebra | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | context_only |
| FR-142 unit node keys | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | context_only |
| FR-142-AC-13 (existing, line 249) | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | context_only |
| FR-272 invalid_package and missing_declaration rows | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | context_only |
| native-diagnostics invalid_package / invalid_semantic_graph rows | proposals/quire-v1/definitions/native-diagnostics.md | context_only |
| checked-package V2 unit reader constraints | proposals/checked-package-v2/README.md | context_only |
| unit-decl grammar | proposals/quire-v1/shared-grammar.md | context_only |
| QSL FR-091 Dimension and unit declarations, OQ-11, OQ-12, AC-35 | quire-spec-language spec/functional/FR-091-produce-value-forms-and-assemble-package-declarations.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No criterion or vector exercises the fixed check order (the two SHALLs on lines 78-81). Every vector in the new AC and in U06 and U11a-U11f has exactly one defect, so any order passes them. Fix: add to the renumbered AC and to TC-187 at least one unit with scale zero and target `nothing` (`invalid-value`), `m_alt` beside `u_self` (`conflicting-definition`), and a two-dimension case once the nesting is fixed (failure-domain FND-003). | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:248 |
| FND-002 | low | 'none of which needs a catalog revision (FR-272)' records this change, not system behavior. Fix: 'with these FR-272 causes'. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:62-63 |
| FND-003 | low | U11 now only restates U11a-U11f. It has no operation of its own, so it is a vector that cannot be run by itself. Fix: delete U11, or move its sentence into the procedure prose above the table. | spec/test-cases/TC-187-quantity-and-unit-conversion.md:50 |

## Verdict

Not mergeable as a set (see integrity FND-001 and failure-domain FND-001 and FND-002). This method's findings are one coverage gap and two wording nits. What passes: the reuse of existing causes, `incompatible-definition` for a cross-dimension target, and dropping the topology cases from AC-5. U06, U11b, U11d and U11e can be built and give the stated cause under any reading of the order.

## Dispositions

Round 1, bdacb1420 (head 8ba797093).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed in bdacb1420: / U11h / `unit z : L = 0 * nothing;` / `invalid_package`/`invalid-value` (row 1 before row 2 within one unit) / / U11i / In dimension `L`, `unit a : L = 1 * nothing;` declared before `unit b : L = 0 * m;` / `missing_declaration`/`missing-name` naming `a` (the earliest unit is processed first) / |
| FND-002 | fixed | Fixed in bdacb1420: When the checker admits a package's unit and dimension declarations, the checker SHALL refuse a topology defect before any quantity value exists, with these FR-272 causes; each refusal names the dimension node and the unit or |
| FND-003 | fixed | Fixed in bdacb1420: U11 row removed; TC-187 now goes from U10 to U11a: / U11a / Add `unit m_alt : L = 1;` beside `unit m : L = 1;` / `invalid_package`/`conflicting-definition` naming `L`, `m` and `m_alt` (FR-142-AC-15) / |
