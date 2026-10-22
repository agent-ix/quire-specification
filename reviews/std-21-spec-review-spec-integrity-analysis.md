---
id: SR-15250
title: 'Spec integrity analysis of PR #361: FR-470 domain clause name resolution (STD-21)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@8a0ac91; git diff origin/main(0a11d84)...8a0ac91 (PR #361, branch std-21-domain-clause-names): FR-470 (new, AC-1..8), TC-953 (new, R01..R09), spec/spec.md contains edge; context FR-154, FR-208, model-complete.md, shared-grammar.md, TC-939, open PR #360; QSL FR-366/TC-1050 (quire-spec-language read only); ticket STD-21 (QSpec#84)'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-470
  type: reviews
- target: ix://agent-ix/quire-specification/TC-953
  type: reviews
---

# Spec integrity analysis of PR #361: FR-470 domain clause name resolution (STD-21)

## Summary

Ticket: STD-21 (QSpec#84). PR: quire-specification#361, head 8a0ac91, base 0a11d84. FR-470 answers the ticket's question. A headerless domain-package clause resolves qualified names through the self alias, which is the id of the `domain` (namespace) artifact whose `members` list the clause's owner. The importing unit's aliases are not visible, and a leading header, import, profile or model form refuses `invalid_syntax`/`unexpected-token`. Re-measured: QSpec main a3a79a3 still has no such rule (shared-grammar.md:266-280 covers unit aliases only), and QSL b453e10 FR-366/TC-1050 parse kept invariants with the unit clause production and state no alias rule, so the author's gap claim holds. The lead default (candidate 1 keyed on the artifact id) is consistent with FR-154 and model-complete.md:116-118 (a title never resolves a name) and with FR-208's at-most-one-namespace rule (FR-208:317-322), so an owner has at most one self alias and no AC is unbuildable. The identities in the vectors were recomputed from model-complete.md:105-116 and match: case `ix://test/orders/variant/OrderStatus-Draft`, clause `ix://test/orders/Order/ByUnitAlias`. The vectors contain no digest or other hashed value. FR-470 is free on main and in every open PR. TC-953 collides with #360. Gates: `quire validate --scope . spec/**/*.md` (quire 0.36.2, engine 0.50.2) on the head reports 134 failed documents; origin/main a3a79a3 reports 116. The whole difference is the 18 spec/objects/interfaces documents that #356 fixed on main after this branch's base 0a11d84, and no diagnostic names a touched file. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 23 and 126.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-470 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-1 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-2 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-3 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-4 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-5 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-6 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-7 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| FR-470-AC-8 | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md | examined |
| TC-953-R01 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R02 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R03 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R04 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R05 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R06 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R07 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R08 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| TC-953-R09 | spec/test-cases/TC-953-domain-clause-name-resolution.md | examined |
| spec/spec.md FR-470 contains edge | spec/spec.md | examined |
| FR-208 Naming a variant type and its cases | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md | context_only |
| model-complete naming rule | proposals/quire-v1/definitions/model-complete.md | context_only |
| shared-grammar import rule | proposals/quire-v1/shared-grammar.md | context_only |
| TC-939 step 5 | spec/test-cases/TC-939-admit-variant-typed-field-parameter-and-result.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | (major) FR-470-AC-7, the Refusals paragraph and TC-953 R08 refuse a case the variant type does not declare (`OrderManagement::OrderStatus::Shipped`) as `missing_declaration`/`missing-name` with absent node `ix://test/orders/variant/OrderStatus-Shipped`. FR-208 "Naming a variant type and its cases" (line 372), FR-208-AC-17 and model-complete.md:61 make that case `ill_typed`/`type-mismatch` at the name, naming the type and the case. FR-470 also says the segments after the self alias resolve "exactly as a unit resolves them after its model alias" (line 65), so it contradicts itself. Make AC-7, the Refusals example and R08 `ill_typed`/`type-mismatch` naming `OrderStatus` and `Shipped` (and `Lost`). Keep `missing-name` for a variant type name that resolves to nothing, for example `OrderManagement::Nope::Draft`. | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md:99 |
| FND-002 | high | (major) TC-953 is also the id of spec/test-cases/TC-953-proof-gate-discharge-acceptance.md in open PR #360 (STD-5, head 21343242). #360 was opened first (2026-10-22T05:29:48Z, against 05:30:06Z for #361) and its body claims TC-953 as the next free id. Whichever merges second gives the repo two TC artifacts with one id. #361 (STD-21) renumbers: move its TC to TC-954, which is free on main a3a79a3 and in every open PR, and update the file name, the `id`, the heading and FR-470's Downstream link (FR-470:123). | spec/test-cases/TC-953-domain-clause-name-resolution.md:2 |
| FND-003 | medium | (minor) TC-939 step 5 adds a domain clause to `Money`, which K2's `OrderManagement` namespace lists, written as `self.tier = M::Tier::Gold`. Under FR-470-AC-3 the unit alias `M` is not visible in a domain clause, so this clause refuses `missing_declaration`/`missing-name` when it is checked. The step's expected result covers admission only, so it is not a direct contradiction. It is the only domain-clause example left in the spec, and it uses the spelling FR-470 now refuses. Respell it `self.tier = OrderManagement::Tier::Gold` (Tier being a type of the same package). | spec/test-cases/TC-939-admit-variant-typed-field-parameter-and-result.md:35 |
| FND-004 | medium | (minor) The ticket asks for the answer in the shared grammar or the state-core profile, and it quotes this sentence as the source of the question. The PR leaves it unchanged, and leaves model-complete.md:53 ("Clauses name a declaration by qualified name: the model alias, then the segments ...") unchanged too. Read literally, both still contradict FR-470's "needs no import" and self alias. Add one sentence to each that excepts a domain clause and points to FR-470. | proposals/quire-v1/shared-grammar.md:1011 |
| FND-005 | low | (minor) "Each refused name is reported once" can be read as one refusal per distinct name. R06's `NoAlias` names `OrderManagement::OrderStatus::Draft` twice and expects two refusals. Say "each refused occurrence" (or "each qualified name occurrence") so the rule matches R06. | spec/functional/type-model/FR-470-resolve-names-in-domain-package-clauses.md:99 |

## Verdict

Not mergeable. Two findings are major: FND-001, where FR-470-AC-7 and R08 refuse an undeclared variant case with `missing_declaration`/`missing-name`, against FR-208 and model-complete's `ill_typed`/`type-mismatch`; and FND-002, the TC-953 id collision with #360, which #361 must fix by renumbering to TC-954. The minor findings are FND-003 (TC-939 step 5's stale `M::` spelling), FND-004 (shared-grammar and model-complete text not amended) and FND-005 (wording of "reported once"). The self-alias rule, title refusal, foreign-namespace refusal, unit-alias isolation and header refusal are otherwise sound and consistent with FR-154 and FR-208.
