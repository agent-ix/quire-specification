---
id: SR-13670
title: 'Spec review of QSpec PR #319: FR-142 unit-graph topology causes, failure domain'
type: SpecReview
analysis: failure-domain
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

Failure-domain pass over the unit-graph refusal rule: each vector built by hand from the `unit-decl` grammar over TC-187's `L` graph (`m`, `cm`, `in`, `rev`; `s` of `T`), two-defect graphs under the stated order, and the missing-root claim over every finite shape (no units, a root beside a self-target, a root beside a separate cycle, no root with an unknown target). `quire validate --scope . 'spec/**/*.md'` (quire 0.36.2, engine 0.50.2) reports 134 failed documents on main and on the PR head, the same set; FR-142 and TC-187 validate clean. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (114 and 14 tests).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-142 cause table | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142 root row | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142 missing-root sentence | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142 check-order sentence | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| TC-187 U06 | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11a | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11b | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11c | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11d | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11e | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-187 U11f | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
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
| FND-001 | high | The check order cannot be computed for the graphs it has to order. A unit's identity is its node key, whose preimage contains `target_unit_node_id`, and FR-142 says 'Acyclic unit targets make these keys recomputable in root-to-leaf order' (lines 140-143). A unit on a target cycle, a unit whose target is unknown, and every unit whose target chain reaches one of them has no key. So 'each unit in ascending unit identity' is undefined exactly when rows 3 and 6 can fire, and so is the choice between two cycles, such as a self-target `u_self` and the `m`/`cm` cycle in one dimension. The QSL assembler (qsl-semantics src/check/assemble/units.rs) keys units only after the name and cycle checks pass, for this reason. Fix: order units by something that exists before keys do, such as the qualified declaration and then the source position, and say that the cycle reported is the one that contains the least unit. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:78-81 |
| FND-002 | high | The 'no root means a cycle' claim is false for a dimension with no units. `dimension L;` with no unit has no targetless unit, no unknown target, no cross-dimension target and no cycle, so no row refuses it, yet lines 51-52 require 'exactly one canonical root'. The gap reaches evaluation: `dimension L; dimension Area = L^2; unit m2 : Area = 1;` admits `m2`, but `2 m2 * 3 m2` needs a compound term over the canonical root unit of `L` (line 160, 'admitted I04 canonical root-unit node IDs'), and `L` has none. The claim holds for every nonempty dimension: a nonempty finite graph in which every unit names a known in-dimension target has a cycle, and a root beside a self-target or a separate cycle is caught by row 6. Fix: say 'a nonempty dimension' in the claim; then either refuse a dimension that has no unit with a named cause, or admit it as rootless and say what a compound result over a rootless base dimension is (or refuse a derived dimension whose base dimension has no root). | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:75-78 |
| FND-003 | medium | The loop nesting of the check order is not fixed. 'check the defects in the table's order, for each dimension in ascending dimension identity and each unit in ascending unit identity, and SHALL report the first' reads defect-major (row 1 over every dimension, then row 2) or dimension-major (every row in the first dimension, then the next), and inside one dimension defect-major or unit-major. Two-defect graphs give different results: with dimension L before T, a self-targeting unit in L and a zero-scale unit in T give `definition-cycle` (dimension-major) or `invalid-value` (defect-major); in one dimension, unit `a` before `b`, `a` targeting `nothing` and `b` with scale zero give `missing-name` (unit-major) or `invalid-value` (defect-major). Some cases do not depend on the reading: one unit with scale zero and an unknown target is `invalid-value`, and a self-target beside a second targetless unit in one dimension is `conflicting-definition`. Fix: state the nesting, for example 'for each dimension in order; inside it, each row in table order; inside a unit row, each unit in order'. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:78-81 |
| FND-004 | medium | 'the canonical root' has no referent when a dimension has two targetless units, and row 2 is checked before row 5. With `m` (scale 1) and `m_alt` (scale 2, targetless), it is unclear whether `m_alt` is a root whose scale is not one (`invalid-value`) or a second root (`conflicting-definition`). The vectors leave out the values that decide this. U11a and the new AC do not give `m_alt`'s scale or offset, U11c does not give `x`'s scale and U11f does not give `u_self`'s, so writing `unit m_alt : L = 2;` makes U11a `invalid-value` and `unit x : L = 0 * nothing;` makes U11c `invalid-value`. Every vector can be built from the `unit-decl` grammar. Fix: say that row 2 applies to every targetless unit, and give each vector its source declaration: `unit m_alt : L = 1;`, `unit x : L = 1 * nothing;`, `unit u_self : L = 1 * u_self;`. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:69 |
| FND-005 | medium | The table leaves out the fifth interim fault of the blocked consumer. QSL FR-091-AC-35 refuses a derived dimension whose normalized terms are empty (`dimension L; dimension N = L / L;`) with the interim `unit-graph-topology` cause, and FR-091-OQ-12 says it 'needs one as well, and STD-112 does not list it yet'. After this PR, QSL-275 still has one fault with no QSpec cause. QSpec does not say anywhere whether such a dimension is refused: FR-142 lines 46-49 only remove zero exponents. Fix: rule it here (for example `invalid_package`/`invalid-value`, retaining the dimension and its written terms) or state that a derived dimension that cancels to no terms is the dimensionless dimension and is admitted, so that QSL drops the fault. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:61-74 |

## Verdict

Not mergeable: the stated order is uncomputable for the graphs it must order (FND-001), and the missing-root claim fails for an empty dimension (FND-002). The claim holds for every nonempty dimension. A root beside a self-target or a separate cycle is caught by row 6, and a rootless dimension with an unknown target by row 3. One unit with scale zero and an unknown target is `invalid-value` under any reading of the order.
