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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | high | U11b's expected result contradicts the order FR-142 now states. FR-142 orders units by 'declaration order (qualified declaration name, then source position)' and starts a cycle's edges at 'the cycle's least unit by declaration order'. By name, `cm` sorts before `m`, so the expected edges are `cm -> m`, `m -> cm`, starting at `cm`. U11b expects them to start at 'the least unit `m`', which holds only if 'declaration order' means source order. The phrase 'declaration order' itself suggests source order, and U11i ('`a` ... declared before `b`') and U11k ('Dimension `L` declared before `T`') phrase their orders as source order, but none of them separates the two readings because their name order and source order agree. The ordering of a qualified name (an array of segments) is also not stated. Fix: pick one. For name order, call it 'name order', say how segment arrays compare (segment by segment, bytewise), set U11b's edges to `cm -> m`, `m -> cm`, and add a vector whose name order and source order differ (for example `unit b : L = 0 * m;` written before `unit a : L = 1 * nothing;`, expecting `missing-name` at `a`). For source order, drop 'qualified declaration name' from the parenthesis. | spec/test-cases/TC-187-quantity-and-unit-conversion.md:51 |
| FND-007 | medium | The empty-dimension rule picks the wrong dimension. A compound result rests only on the canonical roots of base dimensions (lines 180-181), but the rule refuses any unitless dimension that a derived dimension with a unit names. The cases it gets wrong: (1) `dimension L; unit m : L = 1; dimension Area = L ^ 2; dimension Vol = Area * L; unit m3 : Vol = 1;` refuses `Area`, although `Vol`'s normalized map `{L: 3}` rests on `m`. (2) `dimension L; dimension N = L / L; unit one : N = 1;` refuses `L`, although `N`'s map is empty. (3) `dimension L; dimension Area = L ^ 2; dimension Vol = Area * Area; unit m4 : Vol = 1;` refuses `Area` naming `Vol`, while the root that is actually missing is `L`'s. The 'a unit's type ... names' alternative never applies, because the dimension a unit's `:` names has that unit. Which derived dimension is retained when several name the dimension is not stated either. This follows the coordinator's literal ruling, so the narrower rule is the owner's call. Fix: refuse a base dimension with no unit that has a nonzero exponent in the normalized map of a dimension that has a unit, retaining that base dimension and the first such dimension in the stated order. Add vectors (1) to (3). | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:83-90 |
| FND-008 | medium | 'SHALL refuse it before the rows above' does not place the empty-dimension check in the dimension loop. Take `dimension Area = L ^ 2; unit m2 : Area = 1 * m2b; unit m2b : Area = 1 * m2; dimension L;`. `Area` comes first by name and has a cycle, and `L` has no unit and is named by `Area`. If the check runs at `L`'s turn in the loop, the result is `definition-cycle` at `Area`; if it runs for every dimension before the loop, the result is `missing-name` at `L`. U11g has a single defect, so it does not decide this. Fix: say the empty-dimension check runs over every dimension, in order, before the per-dimension loop (or as step (0) of each dimension's own turn), and add this two-defect vector. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:83-87 |
| FND-009 | low | In the dimensionless sentence, 'it needs a canonical root unit only when a unit or a quantity names it' says nothing. A quantity names a unit, not a dimension (`Quantity<D>` is refused, FR-142 'Quantity type reference'), and a unit that names `N` is part of `N`'s graph, which the rows already govern. Fix: drop the clause, or say that a dimensionless dimension's units follow rows 1 to 6 like any other dimension's. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:92-95 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | medium | Nothing refuses a dimension cycle, although FR-142 now says 'Dimension terms name any dimension of the package, declared before or after', and the base-dimension rule and step (0) depend on each dimension's normalized map. `dimension A = B; dimension B = A ^ 2; unit a : A = 1;` and `dimension D = D * L;` have no normalized map, so it is undefined whether `L` is 'carried' and which row applies. A term that names no dimension has no stated cause either. QSL FR-091 already refuses a dimension cycle as `invalid_package`/`definition-cycle` with its edges. Fix: before step (0), refuse a dimension whose terms reach it again, `invalid_package`/`definition-cycle`, retaining the term edges from the least dimension by source position, and a term naming no dimension, `missing_declaration`/`missing-name`. Say whether this runs for every dimension before the loop or at the dimension's own turn, and add a vector. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:61-62, 85-94 |
| FND-011 | medium | 'source order (the position of each declaration in the source)' is undefined across documents. A linked package spans more than one unit of compilation (FR-142 'two units of one name in the linked package'), and dimension terms and unit targets are qualified names that can name an imported declaration (`import-decl`). Which of two documents comes first, and whether an imported dimension or unit (already admitted in its own package) takes a turn here, is not stated. So the start of a cycle edge list and the first defect differ between implementations. Fix: define the order as the package's document order (name the list that fixes it), then the byte offset, and say that imported declarations are not re-checked, or where their turn falls. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:65-67 |

## Dispositions

Round 1, bdacb1420 (head 8ba797093).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed in bdacb1420: units shown. The order of the defects is the order of the table: the checker SHALL process the dimensions in declaration order (qualified declaration name, then source position, since a unit's node key is not computable before the target chain is known to be acyclic), and for each dimension SHALL (a) check each of its units in declaration order against the first three rows, (b) then check the dimension against the next three rows, and (c) report the first defect found, so a dimension is refused by the earliest row that applies to its earliest unit, and a later dimension is not examined once... |
| FND-002 | fixed | Fixed in bdacb1420: A dimension that has no unit, and that a unit's type or a term of a derived dimension names where that derived dimension has a unit, has no canonical root for a compound result to rest on; the checker SHALL refuse it before the rows above, `missing_declaration`/`missing-name`, retaining the dimension and the declaration that names it. For a nonempty dimension no separate missing-root cause exists: a finite graph in which every unit names a known target within the dimension is cyclic, so such a dimension is refused by row 2, 3 or 6. |
| FND-003 | fixed | Fixed in bdacb1420: units shown. The order of the defects is the order of the table: the checker SHALL process the dimensions in declaration order (qualified declaration name, then source position, since a unit's node key is not computable before the target chain is known to be acyclic), and for each dimension SHALL (a) check each of its units in declaration order against the first three rows, (b) then check the dimension against the next three rows, and (c) report the first defect found, so a dimension is refused by the earliest row that applies to its earliest unit, and a later dimension is not examined once... |
| FND-004 | fixed | Fixed in bdacb1420: / 4 / the dimension has two or more targetless units / `invalid_package`/`conflicting-definition` / the dimension and every targetless unit, in declaration order / / 5 / the dimension's one targetless unit (its canonical root) has a scale other than one or an offset other than zero / `invalid_package`/`invalid-value` / the root, its scale and its offset / |
| FND-005 | fixed | Fixed in bdacb1420: A derived dimension whose normalized terms cancel to nothing, such as `dimension N = L / L;`, is the dimensionless dimension: the checker SHALL admit it with the empty normalized map, and it needs a canonical root unit only when a unit or a quantity names it. |
| FND-006 | still-open | New this round; not yet fixed. |
| FND-007 | still-open | New this round; not yet fixed. |
| FND-008 | still-open | New this round; not yet fixed. |
| FND-009 | still-open | New this round; not yet fixed. |

Round 2, abe176f9e.

| FND | outcome | reason |
| --- | --- | --- |
| FND-006 | fixed | Fixed in abe176f9e: / U11b / Declare `unit m : L = 1 * cm;` and `unit cm : L = 1/100 * m;` (no unit is targetless) / `invalid_package`/`definition-cycle` retaining the edges `m -> cm`, `cm -> m`, starting at `m`, the least unit by source position (name order would start at `cm`); there is no missing-root cause for a no / U11l / The U11b cycle with `unit cm : L = 1/100 * m;` declared before `unit m : L = 1 * cm;` / `invalid_package`/`definition-cycle` retaining `cm -> m`, `m -> cm` from the least unit by source position, `cm` / |
| FND-007 | fixed | Fixed in abe176f9e: The base-dimension rule: a base dimension (declared with no terms) that has no unit has no canonical root for a compound result to rest on when a dimension that has a unit carries it in its normalized map. At the base dimension's own turn the checker SHALL refuse it, `missing_declaration`/`missing-name`, retaining the base dimension and the first unit, in source order, whose dimension has a nonzero exponent of it in its normalized map; a derived dimension without a unit is not refused, since only base dimensions anchor a compound result. For a nonempty dimension no separate missing-root cau... |
| FND-008 | fixed | Fixed in abe176f9e: / U11m / `dimension Area = L ^ 2;` `dimension L;` `unit a : Area = 1 * b;` `unit b : Area = 1 * a;`; then `dimension L;` `dimension Area = L ^ 2;` and the same units / `invalid_package`/`definition-cycle` at `Area` retaining `a -> b`, `b -> a` (`Area` is processed first, `L` is not examined); then `missing_declaration`/`missing-name` at `L` (`L` is processed first) / |
| FND-009 | fixed | Fixed in abe176f9e: A derived dimension whose normalized map is empty, such as `dimension N = L / L;`, is the dimensionless dimension: the checker SHALL admit it. Its units form N's own graph under rows 1 to 6, and its empty map names no base dimension, so it creates no base-dimension obligation. |
| FND-010 | still-open | New this round; not yet fixed. |
| FND-011 | still-open | New this round; not yet fixed. |
