---
id: SR-13602
title: 'Spec review of QSpec PR #303: FR-322 population reference operand, base'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@ab6ff70; git diff origin/main...ab6ff70 (PR #303, branch qspec-fr322-popref): spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/functional/type-model/FR-153-query-closed-model-environments.md, spec/objects/interfaces/FR-451-checked-package-abstraction-relation-body.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-280-model-owned-member-types.md, proposals/checked-package-v2 (README.md, schema.json, node-identity-preimage.schema.json, node-identity-vectors.json, model-owner-form-vectors.json, fixtures/adverse.json, fixtures/positive-operation-identities.json), tests/checked_package_v2.rs, tests/model_member_types.rs; ticket QSL-869'
review_set: subset
date: '2026-10-20'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/FR-153
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
- target: ix://agent-ix/quire-specification/TC-280
  type: reviews
---

## Summary

Ticket: QSL-869. PR: quire-specification#303 (private), head ab6ff70128fba0cf30b0d323d1300b98a86a441f, branch qspec-fr322-popref, scoped to `git diff origin/main...HEAD` (one commit, 14 files, +768/-52). It adds the FR-322 population reference: a self-typed `relation`/`population` model declaration node with an empty aggregate body, empty dependencies and a `ModelOwner` {kind, identity, node} that is the population's declaration key. It is operand 0 of all_instances and lookup, refused in every other position, and the ownerless placeholder of the published operation fixture is withdrawn. The design is checked and sound. Identity: P7 and P8 recompute independently (Python, JCS = sorted keys, compact separators, UTF-8; every value is a string or null) to 15ed74b1... and cc575f3d.... The regenerated all-instances (c1af3f20...) and model-lookup-undefined (e270495a...) vectors recompute from their preimages. The fixture's new `census` parameter (e3440431...) and `Population<Order>[2]` (1ffe8def...) nodes recompute under the `quire.structural-node/v1` rule. The fixture package_id 0a40d8e8... equals the JCS SHA-256 of its `identity_preimage`, and graph order equals projection order. No withdrawn key (c7c2b384, aaa99117, d1d98b1c, 1420f577) remains in the tree outside an old review. Owner join: PR-01..PR-06 and the full-reader cases cover object-type owner, undeclared owner, population-owned object_type and relationship, and ownerless -> malformed_wire. Use: all_instances and lookup operand admitted; collection.contains operand, the reaches declaring node and the reaches population link refused ill_typed/operator-ineligible. The judgment calls hold. The old `Population<O>[N]` form keeps its own meaning, and AC-62's RM-POP-MAX vectors are untouched. No U or N on the node, with N on result_type collection_bounds, matches QSL FR-391 'Typing'. The reaches link is not extended. There is no compat layer, which is right pre-release. Consistency: matches QSL FR-391 'Lowering' and FR-391-AC-6 (two populations of one member type check to different terms with their own declaration keys). Matches FR-154 (declaration key carries no digest), FR-153, FR-451 (population entries target this node) and ADR-012 (StateModel owns v2 model/relation nodes). Matches ADR-013 R-02 (the wire is authored in QSpec) and QC-25's model-owned node pattern. ADR-018 is not affected. The family-rule edit makes 'a relation reference is `population`' apply to `relation`/`population` only. Main contradicted 'A relation node ... have no family'; HEAD no longer does. Hash/digest note: the P7/P8 and fixture digests are product identity vectors (content-addressed node and package ids are the feature), not repo bookkeeping, so the pin antipattern rule does not apply. Gates at HEAD: `quire validate --scope . 'spec/**/*.md'` gives 134 failed documents, identical to main once the FR-322 lines are shifted. FR-322 keeps its 2 pre-existing errors and 4 warnings, and FR-153, FR-451, TC-233 and TC-280 add nothing. `cargo test --test checked_package_v2 --test backend_provider_v1 --test model_member_types` (own CARGO_TARGET_DIR): 112, 11 and 23 passed. quoin 0.28.3, quire 0.36.2 (engine 0.50.2).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-population-references-intro | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-node | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-declaration-key | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-identity | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-owner-join | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-use | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-quantifier-domain | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-relation-to-population | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-abstraction-relations | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-family-rule | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-62 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-82 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-83 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-153-population-name-in-package | spec/functional/type-model/FR-153-query-closed-model-environments.md | examined |
| FR-451-population-entry-key | spec/objects/interfaces/FR-451-checked-package-abstraction-relation-body.md | examined |
| TC-233-population-reference-cases | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |
| TC-280-PR-01..PR-06 | spec/test-cases/TC-280-model-owned-member-types.md | examined |
| PR-303-consumer-notes | PR #303 body, 'Consumer notes (for luna:IR:plan)' | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-83 says the `Population<O>[N]` binding operand of the published operation fixture 'is admitted unchanged and the keys of AC-62 do not change'. That operand did not exist before this PR. On main the fixture's operand 0 of all_instances and lookup was the ownerless `relation`/`population` node c7c2b384...; this PR replaced it with the new `value`/`parameter` `census` (e3440431...) typed by a new `bounded_domain`/`model_population` [max 2] node (1ffe8def...). So 'unchanged' describes a past state that is false, and as a criterion it only restates 'is admitted'. The same history wording appears in FR-322 'Relation to Population<O>[N]' ('The anonymous type form of FR-322-AC-62 stays', 'AC-62's keys and shapes are unchanged', F322:1280-1291), in TC-233:290 ('binding operand, unchanged'), in the adverse.json case id `published-parameter-operand-keeps-its-form` (line 78) and in the test doc comment (tests/checked_package_v2.rs:12995-12996). A spec says what the system is, not what changed. Fix: AC-83: 'The published operation fixture's operand 0 is a `value`/`parameter` binding typed by `Population<Order>[2]`, and the full reader admits it; AC-62's vectors recompute to their published keys.' Drop 'stays' and 'unchanged' from the FR-322 bullet and TC-233, and rename the case, e.g. `parameter-operand-admitted`. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1732 |
| FND-002 | medium | The note says CG/RT readers 'adapt additively'. They do not. On main the V2 OwnerPresenceRule put `relation`/`population` in the 'every other node' row, so an `owner` was forbidden, and the published fixture carried exactly that ownerless node as operand 0 of all_instances and lookup. At HEAD an owner is required and the same node is `malformed_wire`. A reader that accepted main's form must now refuse it. Published identities also changed: fixture package_id aaa99117... -> 0a40d8e8..., all-instances d1d98b1c... -> c1af3f20..., model-lookup-undefined 1420f577... -> e270495a..., the `retained_sha256` of lookup-absence-mode-missing and all-instances-extra-operand, and the operand node c7c2b384... -> e3440431.... This is the no-compat withdrawal of judgment call 2, which is fine pre-release. But the note sent to the IR planner should call it a breaking wire change: refuse an ownerless `relation`/`population` node, and re-pin any copied fixture or vector keys. Impact found: no repo under ~/dev outside this one pins the withdrawn keys, and QSL S4 never emitted the form (EmitRefusal::PopulationReferenceNotEmitted). Fix: change 'adapt additively' to 'one breaking change (the ownerless form is withdrawn), otherwise additive', and list the changed keys. | PR #303 body:Consumer notes |
| FND-003 | low | The bullet says 'a `field`, `operation` or `relationship_end` member whose declaring node is a population reference refuses `ill_typed`/`operator-ineligible`'. AC-83 and the adverse cases only cover `relationship_end` (`population-as-a-relationship-declaration` on model.reaches). No criterion or case covers a `quire.op.record.project` `field` member or a `quire.op.model.dispatch_call` `operation` member that names a population reference. Fix: add both to AC-83 and add two `population_reference_cases`, or narrow the sentence to what is covered. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1257-1259 |
| FND-004 | low | 'A clause `forall(o in allInstances<T>(M::P) : c)` is therefore three nodes' is false as written: the clause also holds the binder, `c` and the type nodes. The intended claim is that the quantifier's domain takes three nodes. Fix: 'its domain is three nodes: ...'. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1276-1279 |

## Verdict

Not mergeable yet, because of two medium findings. The model is right and every identity vector checks: P7/P8, the regenerated application keys, the new fixture nodes and the package id all recompute independently, and validation and tests match main. To merge: restate AC-83 and the FR-322 'Relation to' bullet without history wording (FND-001), and correct the consumer note: withdrawing the ownerless form breaks readers, and the listed keys changed (FND-002, PR body only). FND-003 and FND-004 are small wording or coverage fixes.
