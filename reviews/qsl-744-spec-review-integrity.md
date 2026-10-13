---
id: SR-8719
title: "QSL-744 integrity review of TC-235 M10 population member path"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@3ba328881ee062c4e52d9f64f6ba112f03d4d00d; spec/test-cases/TC-235-business-construct-meaning-binding.md row M10; cross-checked against agent-ix/quire-spec-language@88cdadd279a4247945340f3bd8ae3b982c85e1cf (PR 808 head) and filament-core-data@72527ab5a629b50e1c15f62e8a83826accbcb5f7"
review_set: subset
---

## Summary

Ticket: QSL-744 (AC4). Checks that the new TC-235 M10 run 10 path agrees with the semantic IR shape, with the built QSL behaviour, and with QSL's own spec and tests. All agree. No defect found.

## Verdict

**PASS.**

- IR shape: the semantic-ir schema (`schema/semantic/v1/semantic-ir.schema.json`, `$defs.population`) declares `members` as an array of `semanticIdentity` strings. A member has no `.type`, so the old spelling `Population.members[0].type` named nothing in the IR. The semantic-ir reader (`crates/semantic-ir/src/constructs.rs` `populations`) also reads members as strings.
- Built behaviour: QSL `qsl-semantics/src/model/normalize.rs` (population arm, comment cites TC-235 M10 run 10 / FR-208-AC-9) refuses `invalid_model_binding`/`malformed-declaration` at the population key with `MemberPath::new(MemberRoot::PopulationDefinition).name("members").index(position)`. For `[OrderManagement]` the position is 0, giving `PopulationDefinition.members[0]`. The intake pointer map (`intake.rs` test `a_semantic_ir_pointer_maps_to_its_member_path`) maps `/ir/populations/1/members/0` to `PopulationDefinition.members[0]`. `MemberRoot` renders `PopulationDefinition`; there is no `Population` root.
- QSL spec and tests: FR-056-AC-32 and AC-33, TC-997 and TC-1005 spell `PopulationDefinition.members[i]`; `model_normalization.rs` and `tc235_origin_retained.rs` assert the concrete `PopulationDefinition.members[1]`. The QSpec row's concrete `[0]` matches that convention and the sibling TC-235 rows.
- Cause: unchanged at `malformed-declaration`, which matches the built code and FR-208-AC-9.
- No other QSpec TC or FR spells the old path.

QSL PR 808 adds `a_population_member_of_another_meaning_refuses_malformed_with_origin` (`tc235_origin_retained.rs`, doc comment "TC-235 M10 run 10"). It places the wrong-meaning member (`OrderManagement` among others) second, after `Order`, and asserts `PopulationDefinition.members[1]`. That is the same root and index rule as the QSpec row; the row's single-member `[OrderManagement]` gives index 0.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
