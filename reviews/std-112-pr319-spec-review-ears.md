---
id: SR-13668
title: 'Spec review of QSpec PR #319: FR-142 unit-graph topology causes, EARS conformance'
type: SpecReview
analysis: ears-conformance
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

EARS conformance of the new FR-142 obligations: the refusal statement, the check-order sentence, the refused-dimension sentence and the new AC row. `quire validate --scope . 'spec/**/*.md'` (quire 0.36.2, engine 0.50.2) reports 134 failed documents on main and on the PR head, the same set; FR-142 and TC-187 validate clean. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (114 and 14 tests).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-142 unit-graph refusal statement | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142 check-order sentence | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142 refused-dimension sentence | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142-AC-13 (new, line 248) | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
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
| FND-001 | low | The obligation's subject, 'The admission of a unit graph', is an event, not the responsible system. FR-142's other statements use 'the semantic kernel' and 'the checker'. Fix: 'When the checker admits a dimension's unit declarations, the checker SHALL refuse ...' (this also settles integrity FND-002). | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:61 |
| FND-002 | low | 'A refused dimension yields no unit mapping.' is an obligation with no SHALL, and next to 'SHALL report the first' it does not say whether the other dimensions of the package are admitted. Fix: 'The checker SHALL admit no unit mapping of a refused dimension' and state whether the package is refused as a whole. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:81 |

## Verdict

Two low findings. The SHALL statements are otherwise singular per obligation, and the new AC row is testable.

## Dispositions

Round 1, bdacb1420 (head 8ba797093).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed in bdacb1420: When the checker admits a package's unit and dimension declarations, the checker SHALL refuse a topology defect before any quantity value exists, with |
| FND-002 | fixed | Fixed in bdacb1420: The checker SHALL admit no unit mapping of a refused dimension. A refusal refuses the package, as every other refusal of this requirement does, and the `quire.checked-package/v2` strict reader refuses the same shapes, as an already-built package, with `invalid_semantic_graph` (FR-322 |
