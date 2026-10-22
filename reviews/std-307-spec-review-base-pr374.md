---
id: SR-16030
title: "Spec review — Clock fields and package dimensions and units at intake (FR-154, FR-322 step 1)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; spec/functional/type-model/FR-154-admit-domain-package-model.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-234-domain-package-model-intake.md, spec/test-cases/TC-280-model-owned-member-types.md at 4685e6e"
review_set: subset
evaluated_revision: "4685e6e9680d229a9a9b3267b48f6a2c637facc4"
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-234
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-280
    type: reviews
---

## Summary

Ticket: STD-307. PR quire-specification#374, head 4685e6e (main 5ee0071 is an ancestor). Reviewed the FR-154 clock and dimension/unit rows and paragraphs, FR-154-AC-18/19, the FR-322 'Model-owned members' step 1 edit and FR-322-AC-98, TC-234 I13/I14 and TC-280 MS-07/08 against filament-core-data FR-146 as merged (origin/main 0439946). Vectors were walked by hand (K1 has three declarations, so I13 run 1 has four and I14 run 1 seven; AC-18's member-path order matches FR-154's UTF-8 member-name order).

## Verdict

CHANGES REQUESTED: two major findings (a duplicate dimension/unit identity has two contradictory refusal rows; I14's self-term vector misses FCD's cycle refusal) and three minor. The clock-shape row, the Clock parameter/result refusal, the unresolved-reference mapping, the dimension and unit identity slots and the FR-322 step 1 edit agree with FCD FR-146. Ids FR-154-AC-18/19, FR-322-AC-98, TC-234 I13/I14 and TC-280 MS-07/08 are unique across open PRs. No test-parsed fixture or vector file changed (spec text only), so no QSL conformance run is needed. quire validate: 86 failure lines, same set as main; cargo test checked_package_v2 (119) and backend_provider_v1 (23) pass. Lead defaults 1 and 2 are sound except for the duplicate-identity case (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Two dimensions or two units with one identity match two rows with different refusals: the new row maps every FCD FR-146 'Dimensions'/'Units' rule except unresolved references, which includes FCD's DUPLICATE_IDENTITY bullet, to invalid_model_binding/malformed-declaration, while the existing 'Two nodes share one identity' row gives invalid_model_binding/conflicting-binding; the dimension/unit member-path list has no entry for it either. Pick one (conflicting-binding at Dimension.identity/Unit.identity of the later node, like every other node, is the consistent choice), and state the FCD-code-to-pair map for INVALID_UNIT_GRAPH, UNRESOLVED_DIMENSION_REF, UNRESOLVED_UNIT_REF, INVALID_CLOCK_FIELD, CLOCK_OUTSIDE_FIELD, INVALID_OPERAND/INTEGER_OUTSIDE_I128 on units and DUPLICATE_IDENTITY in the existing 'FCD diagnostic / QSL refusal pair' table rather than by reference to FCD section headings. | spec/functional/type-model/FR-154-admit-domain-package-model.md:93-99 |
| FND-002 | high | I14's 'Time with the term {Time, "1"}' run has two defects under FCD FR-146, not one: a base dimension with a non-empty terms (INVALID_UNIT_GRAPH at /dimensions/0/terms) and a self-term cycle (INVALID_UNIT_GRAPH at /dimensions/0/terms/0, 'a self-term included'), and FCD reports every defect. Under FR-154's member-path order the run gives two malformed-declaration refusals on Time, at Dimension.terms then Dimension.terms[0]; the vector lists only Dimension.terms. Either expect both or use a term that names a dimension outside any cycle (for example a third base dimension). | spec/test-cases/TC-234-domain-package-model-intake.md:55 |
| FND-003 | medium | The closed list of member paths a malformed dimension or unit names omits members FCD FR-146 refuses: a name outside ^[A-Za-z][A-Za-z0-9_]*$ (Dimension.name, Unit.name), a non-boolean base (Dimension.base), a non-identity dimension or target (Unit.dimension), a non-canonical exponent/scale/offset outside i128 (covered by path but not by code), and a missing or malformed origin. Add the paths, or say the locus is the FCD pointer's member path in general. | spec/functional/type-model/FR-154-admit-domain-package-model.md:184-199 |
| FND-004 | medium | FR-154 maps CLOCK_OUTSIDE_FIELD only at an operation parameter or result. FCD FR-146 also refuses Clock as an alias or reference target, a sequence items, a map values, a variant payloadType and a constraint appliesTo; FR-154 states nothing for those, and a variant payloadType of ix://quire/native/Clock reads as 'a payloadType that resolves', which FR-154 refuses unsupported_construct/declaration-form instead. State the pair and locus for every CLOCK_OUTSIDE_FIELD position. | spec/functional/type-model/FR-154-admit-domain-package-model.md:97,170-182 |
| FND-005 | low | AC-18 and TC-234 I13 break seven of the nine clock-shape members; Field.decimal and Field.textProfile are never broken, so the two members whose FCD codes are suppressed for clock fields (DECIMAL_POLICY_*, SCHEMA_VIOLATION for textProfile) have no vector. FCD FR-146-AC-2 breaks all nine; add a decimal policy and a textProfile to the run-2 field (nine refusals, order Field.constraints, Field.decimal, Field.defaultKind, Field.defaultValue, Field.multiplicity, Field.nullable, Field.presence, Field.textProfile, Field.unit). | spec/functional/type-model/FR-154-admit-domain-package-model.md:376 |

## Scope

| ID | Path | Role |
| --- | --- | --- |
| FR-154 | spec/functional/type-model/FR-154-admit-domain-package-model.md | examined |
| FR-154 | spec/functional/type-model/FR-154-admit-domain-package-model.md | examined |
| FR-154 | spec/functional/type-model/FR-154-admit-domain-package-model.md | examined |
| FR-154-AC-18 | spec/functional/type-model/FR-154-admit-domain-package-model.md | examined |
| FR-154-AC-19 | spec/functional/type-model/FR-154-admit-domain-package-model.md | examined |
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-98 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-234 | spec/test-cases/TC-234-domain-package-model-intake.md | examined |
| TC-234 | spec/test-cases/TC-234-domain-package-model-intake.md | examined |
| TC-280 | spec/test-cases/TC-280-model-owned-member-types.md | examined |
| TC-280 | spec/test-cases/TC-280-model-owned-member-types.md | examined |
| FR-146 | agent-ix/filament-core-data spec/functional/FR-146-carry-clock-fields-and-a-dimension-and-unit-graph.md (origin/main 0439946) | context_only |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | I14 states each run has exactly one defect, and one of its runs makes a missing `origin` a defect (Unit.origin), but the duplicate-identity run's 'third unit appended, named ms with identity ix://test/orders/unit/ms, target s, scale "1/1000"' gives no `dimension`, `offset` or `origin`. Read literally that unit also refuses at Unit.dimension/Unit.offset/Unit.origin (schema). Say it is a copy of `ms` (dimension Time, offset "0", its own origin). | spec/test-cases/TC-234-domain-package-model-intake.md:55 |

## Dispositions

Round 1 at e9bb63e.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | fixed | - |

Round 1 verdict: MERGEABLE after FND-006 (minor) or with it accepted: all five round-0 findings fixed at e9bb63e. FR-154 now has one `conflicting-binding` row for duplicate dimension/unit identities, a complete FCD-code table, general FCD-pointer loci, Clock refused at every FCD position (payload never `declaration-form`), and AC-18/I13 break all nine shape members. I14's single-defect runs each check by hand. Gates at e9bb63e: quire validate 86 failure lines, same set as main c103300; cargo checked_package_v2 119 and backend_provider_v1 23 pass; main is an ancestor. No test-parsed fixture changed.
