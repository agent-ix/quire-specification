---
id: SR-16040
title: "Spec review — package units, qualified package unit names and the Clock member type (FR-322 step 4, FR-142)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; spec/functional/type-model/FR-142-evaluate-quantities-and-units.md, spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-187-quantity-and-unit-conversion.md, spec/test-cases/TC-280-model-owned-member-types.md at 10588f0"
review_set: subset
evaluated_revision: "10588f03d48654c6e50fd1cfc252a047d0c144ec"
relationships:
  - target: ix://agent-ix/quire-specification/FR-142
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-271
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-272
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-187
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-280
    type: reviews
---

## Summary

Ticket: STD-308 (with STD-309, QSpec part). PR quire-specification#376, head 10588f0 (main 5ee0071 is an ancestor). Reviewed the FR-322 'Model-owned members' step 4 Clock and unit rows and the package-unit paragraph, FR-322-AC-99, FR-142 'Namespaces' and FR-142-AC-19, the FR-271/FR-272 widening, TC-187 U34 and TC-280 MU-07..10 against filament-core-data FR-146 (origin/main 0439946), FR-322's `declaration` and qualified-reference rules, FR-142's unit-node preimage and topology rules, and FR-415.

## Verdict

CHANGES REQUESTED: three major findings (package-unit naming contradicts FR-322's declaration/qualified-reference rules; the Clock refusal forbids the clock-constraint read FR-415 admits; MU-07's fixture conflicts with the PR's own dimension rules) and one minor. The unit-node owner (ModelRef is an admitted FR-142 owner), the dimension conflict rule, its locus and the FR-271/FR-272 widening are consistent with each other; U34 and FR-142-AC-19 check by hand. Ids FR-322-AC-99, FR-142-AC-19, TC-280 MU-07..10 and TC-187 U34 are unique across open PRs. No test-parsed fixture changed. quire validate: 86 failure lines, same set as main; cargo test checked_package_v2 (119) and backend_provider_v1 (23) pass. Lead defaults: qualifying package units is the right direction but needs FND-001's rule changes; unqualified dimensions are sound for QSL's name-based Time but mean two domain packages that both supply time units can never be admitted together (owner decision); Clock with no node is acceptable only with FND-002 fixed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The package-unit name acme/rpc/ms contradicts FR-322's own `declaration` rule (lines 633-652): qualified_name is the package-local path of declaring scopes from the package root, is carried only by a node with a declaration source occurrence, and 'a producer never invents a name'; a qualified reference a::Name resolves `a` to a locked package and matches Name byte for byte against that package's declaration paths, so `M::ms` (Name = ms) cannot match a unit whose path is acme/rpc/ms, and `M` is a model selection, not 'itself or a dependency_selections entry'. A source unit declared in scopes acme::rpc would also have the path acme/rpc/ms and collide with the package unit, contrary to 'never one ambiguous name'. The PR also gives no checked-package node form for package units and dimensions beyond the unit preimage (whether the node carries `declaration`, its dependencies, and the dimension node's owner). Widen the `declaration`/qualified-reference rules for model-supplied units, or name package units without inventing a source path. Separately, FCD FR-146's unit typeRef segment pattern cannot spell a hyphenated package identity (STD-327, owner decision). | spec/objects/interfaces/FR-322-checked-package-artifact.md:1121-1131 |
| FND-002 | high | The Clock row and AC-99 refuse every `field` application over a clock field with invalid_timed_model/clock-read-outside-constraint ('a field application whose member resolves to it refuses'), but FR-415 admits exactly one read of a clock: the clock operand of an atomic clock constraint (`self.x <= 3`), which is itself a field application. As written, every guard and time invariant over a package clock refuses. FR-322 also gives no checked-package form for that operand (no type node). Restrict the refusal to an application outside an atomic clock constraint and a reset (MU-10 already says 'used as a value'), and state how the clock operand is carried, or say the checked package carries no clock constraint yet. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1101,2022 |
| FND-003 | high | MU-07/MU-08 (and AC-99) put acme/rpc's base dimension Time with root s and unit ms beside the source units ms and s of MU-01 and expect no refusal. Under this PR's FR-142 rule a source dimension Time beside the package Time refuses conflicting_declaration/duplicate-declaration; if instead the source units are typed by the package Time, the source s is a second targetless root of Time, which FR-142's topology rows refuse (compare U11t). MU-01 never says which dimension its units have, so the fixture is unbuildable as stated. Use a package dimension the source does not declare (FR-142-AC-19 and U34 use Rate) or make the source ms target M::s with no source root. | spec/test-cases/TC-280-model-owned-member-types.md:170 |
| FND-004 | medium | FCD FR-146 leaves FR-142's base-dimension rule and first-defect order to the consumer, and FR-154 says 'FR-142 checks that graph', but FR-142 does not place package dimensions and units in its turn list: whether they are local or imported nodes, what 'source order' and 'earlier' mean for them (authored order within a package, then lock order), and where a refusal on a package node is located (the selection row). Two implementers would order and locate those refusals differently. | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md:62-72,257-271 |

## Scope

| ID | Path | Role |
| --- | --- | --- |
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-99 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-142 | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-142-AC-19 | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | examined |
| FR-271 | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md | examined |
| FR-272 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| TC-187 | spec/test-cases/TC-187-quantity-and-unit-conversion.md | examined |
| TC-280 | spec/test-cases/TC-280-model-owned-member-types.md | examined |
| TC-280 | spec/test-cases/TC-280-model-owned-member-types.md | examined |
| TC-280 | spec/test-cases/TC-280-model-owned-member-types.md | examined |
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | context_only |
| FR-142 | spec/functional/type-model/FR-142-evaluate-quantities-and-units.md | context_only |
| FR-415 | spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md | context_only |
