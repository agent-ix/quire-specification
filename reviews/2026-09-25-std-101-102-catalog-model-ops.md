---
id: SR-626
title: "Spec review — STD-101/STD-102 conforming reference equality and model field projection"
type: SpecReview
analysis: failure-domain
scope: "STD-101/STD-102; commit 88e64d4: operation-catalog.json `object` family, `field_owner` group and `conforming_reference` constraint, FR-322 Reference conformance and model field projection, FR-322-AC-33/34, TC-281, model-member-type-vectors.json reference_equality_cases and projection_cases, tests/model_member_types.rs, tests/checked_package_v2.rs oracle"
review_set: subset
---
# Spec review — STD-101/STD-102 conforming reference equality and model field projection

## Summary

Base checklist plus failure-domain review of commit 88e64d4 against
`origin/main`. The commit gives a model declaration node of form
`object_type` or `systems_interface` the family `object`, adds the
`field_owner` group so `quire.op.record.project` reads a field over a
`quire.op.model.deref` result, and replaces `same_type` with
`conforming_reference` on `quire.op.reference.eq`/`ne`. The FR-322 prose and
the catalog agree entry for entry. The QSL lowering on `origin/main`
(`deref` typed at the object's model node, then `record.project` with a
`field` member naming that node) matches the new rule. The TC-281 test reads
the catalog's groups, result form and constraint kinds, so reverting either
catalog change fails it; it does not simply restate itself. The review
found two medium and eight low findings. All are fixed in the follow-up
commit except FND-004's upcast conformance check, which predates this change
and is recorded, not changed.

## Verdict

**PASS after fixes**. FND-001 and FND-002 must be fixed before merge,
because each admits a package the rule does not intend. The low findings
tighten the prose and the vectors.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The same-type-node shortcut runs before step 2. As a result, `Reference<billedTo> = Reference<billedTo>` over the relationship M4, and `Reference<M2> = Reference<M2>` over an unselected owner, are admitted, while the same nodes paired with `Order` refuse. That contradicts FR-322-AC-33's "a reference to a relationship ... refuse" and makes the refusal depend on the pairing. An operand whose `reference` target is not a model declaration node is also not covered. The oracle repeats the shortcut (`if left == right`), so no vector exposes it. Fix: recover both owners by step 2 in every case, require each to be an object type declaration, then apply the chain test. Add vectors for M4 = M4 and M2 = M2. | FR-322 "Reference conformance" first paragraph; FR-322-AC-33; tests/model_member_types.rs `conforming_reference` |
| FND-002 | medium | `object` joins `any_term`. The TC-233 invariant "any_term equals families" requires this. So a `deref` result now fits the rest position of `quire.op.function.call` and of the four clause operations, and FR-322 checks no function-call argument against a parameter type. A package with `f(deref(r))`, or a temporal clause over an object term, is admitted, and QSL never emits either. A bare `deref` as an expression root (`semantic_type` = model node) is also admitted. Fix: state where an `object` term is admitted, for example "an application of family `object` is admitted only as operand 0 of a `field_owner` position". Otherwise, drop `object` from `any_term` and restate the TC-233 invariant. Add one adverse vector. | FR-322 families paragraph and `any_term` row; operation-catalog.json `groups.any_term`; tests/checked_package_v2.rs any_term assertion |
| FND-003 | low | "Object type declarations" does not say whether a `systems_interface` declaration (an object type with `interfaceFeatures`) takes part in conformance. FR-152 gives it structural-conformance meaning, but the rule uses declared supertypes only. No vector compares `Reference<Flow>` (M8). Fix: state the chosen rule for `systems_interface` and add an RC vector over M8. | FR-322 "Reference conformance"; FR-152; vectors M8 |
| FND-004 | low | A model node is now `object`, not `reference`. So `quire.op.reference.upcast`'s `member_family: reference` now admits only a `composite_type`/`reference` type argument, and a model node no longer fits. FR-322 does not say which node the upcast's `type_argument` names. The entry also has no conformance check between operand 0 and the member. This predates the diff and is adjacent to it. Fix: state that the upcast's `type_argument` is `Reference<T>`. Record the missing directional conformance check on the upcast for its owner. | operation-catalog.json `quire.op.reference.upcast`; FR-322 families paragraph |
| FND-005 | low | The prose cites TC-198 L08 and FR-149-AC-12 for this rule. Both refuse a non-conforming pair as `ill_typed`/`type-mismatch`, but the reader refuses `ill_typed`/`operator-ineligible`. Fix: add one sentence saying that the source checker's cause is `type-mismatch` and the package reader's cause is `operator-ineligible`. | FR-322 "Reference conformance"; FR-149-AC-12; TC-198 L08 |
| FND-006 | low | The prose refuses "two object types of different selected documents", but neither an AC nor a vector covers that case, since the vector package has one document. The multiple-supertype path (`Both` over `Tagged` and `Coded`) is also not tested, because Tagged and Coded have no node vectors. Fix: add a node vector for `Tagged` and a case `Both` = `Tagged`. Either add a second selected document or name the cross-document refusal in AC-33 with a vector. | FR-322 "Reference conformance"; model-member-type-vectors.json `reference_equality_cases`; TC-281 |
| FND-007 | low | RP-06 ("a relation node has no family") depends on a rule that is only implied: a type node outside the family list has no family and fails every family check. The family paragraph lists the families but does not say what happens to other nodes. Fix: add "any other type node, including a `relation` node, has no family, and an application whose derived or declared type has none is `operator-ineligible`". | FR-322 families paragraph; TC-281 RP-06; FR-322-AC-34 |
| FND-008 | low | The TC-233 oracle decides `conforming_reference` as `same_type`. A corpus vector that expects a refusal for two distinct reference types would then pass for the wrong reason. The oracle cannot decide conformance without a domain package. Fix: when the two type nodes differ, have the oracle panic or return undecided instead of refusing. | tests/checked_package_v2.rs `conforming_reference` arm |
| FND-009 | low | FR-322 requires the projection's `result_type` to be the step 4 type of the resolved field. The projection cases carry no declared projection `result_type`, and the oracle does not check it, so a wrong declared result is never refused. Fix: add a `result_type` to each RP case, check it in `projection`, and add one adverse case whose result is the wrong type node. | FR-322 model field projection paragraph; FR-322-AC-34; RP-01..RP-07 |
| FND-010 | low | The sentence "the type `quire.op.model.deref` yields for a `Reference<O>` operand is `O`'s model declaration node, of family `object`" appears both in the families paragraph and in the "Model-owned members" projection paragraph. The families-paragraph copy leaves line 413 unwrapped at 120 characters. Fix: keep the statement in one place and cite it from the other, then rewrap. | FR-322 lines 411-413 and 365-367 |

## Dispositions

| ID | Disposition |
| --- | --- |
| FND-001 | Fixed: both owners are always recovered by step 2, one type node included; RC-11 (M4 with itself) and RC-12 (M2 with itself) added. |
| FND-002 | Fixed: `object` is dropped from `any_term`; FR-322 states that an `object` term is admitted only at a `field_owner` position, TC-233 asserts `any_term` is every family except `object`, and TC-281 asserts `field_owner` is the only group holding `object`. RC-10 is the adverse vector. |
| FND-003 | Fixed: conformance follows declared supertypes only, a `systems_interface` included, never FR-152 structural conformance; RC-15 and RC-16 over M8 added. |
| FND-004 | Fixed in part: FR-322 states that the upcast's `type_argument` names a `Reference<T>` node. The upcast's missing operand-to-member conformance check predates this change and is recorded here, not changed. |
| FND-005 | Fixed: FR-322 states that the reader's cause is `operator-ineligible` and that FR-149-AC-12 and TC-198 L08 report the pair from source checking as `type-mismatch`. |
| FND-006 | Fixed: node vector M9 (`Tagged`) and RC-13/RC-14 (`Both` with `Tagged`) added. Cross-document non-conformance is stated as a consequence of every supertype edge belonging to one document, not as a separate refusal. |
| FND-007 | Fixed: FR-322 states that a `relation` node and a `model` node of any other form have no family and fit no operand position. |
| FND-008 | Fixed: the TC-233 oracle panics on `conforming_reference` instead of deciding it. |
| FND-009 | Fixed: each projection case carries its declared `result_type`, the test checks it, and RP-08 is the adverse case. |
| FND-010 | Fixed: the `deref` family statement is kept in the families paragraph and cited from the projection paragraph. |
