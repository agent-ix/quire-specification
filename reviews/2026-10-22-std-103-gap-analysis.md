---
id: SR-14652
title: 'Gap analysis of PR #338: FR-322-AC-94 against TC-281 upcast vectors (STD-103)'
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-specification@95cdaac74; git diff origin/main...95cdaac74 (PR #338): FR-322-AC-94 and the "Reference conformance" upcast paragraph against TC-281 RU-01..RU-12 and model-member-type-vectors.json reference_upcast_cases; ticket STD-103'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/TC-281
  type: reviews
---

## Summary

Ticket: STD-103. Plan completion: not assessed. No test reads `reference_upcast_cases` yet; the PR lists the test as remaining work. So I recomputed every expected result independently, by hand and with a script over the `domain_package` projection and the M-node vectors. The script uses the selected identity `acme/orders`, the declared supertypes, `semantic_form`, the operand family and the owner selection. All twelve RU cases match their recorded `expected` value. Sub→Order, SubSub→Order, Order→Order and Both→Tagged are admitted. Order→Sub, Order→Invoice, Sub→Both, Flow→Order, Order→M4 and the `object` operand refuse `ill_typed`/`operator-ineligible`. M2 at either end refuses `missing_declaration`/`missing-selection`. Each TC-281 RU row agrees with its JSON case and traces FR-322-AC-94. Three clauses of the new text have no vector.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-AC-94 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1861 | examined |
| FR-322 Reference conformance (upcast paragraph) | spec/objects/interfaces/FR-322-checked-package-artifact.md:1125-1142 | examined |
| TC-281 RU-01..RU-12 | spec/test-cases/TC-281-model-reference-equality-and-field-projection.md:93-104 | examined |
| reference_upcast_cases | proposals/checked-package-v2/model-member-type-vectors.json:511-524 | examined |
| domain_package, model_declaration_nodes | proposals/checked-package-v2/model-member-type-vectors.json | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Minor. FR-322-AC-94 admits "a type with two supertypes to either", but TC-281 has only RU-04 (`Both` to `Tagged`). No case upcasts `Both` to `Coded`, and `Coded` has no model declaration node vector to name it, so the second supertype is never tested. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1861; spec/test-cases/TC-281-model-reference-equality-and-field-projection.md:96 |
| FND-002 | medium | Minor. FR-322-AC-94 refuses "a reference to a relationship at either end", but only RU-09 puts the relationship M4 at the target end. No case has a relationship operand, for example `Reference<M4>` to `Reference<Order>`, so the source end of the clause is never tested. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1861; spec/test-cases/TC-281-model-reference-equality-and-field-projection.md:101 |
| FND-003 | low | Minor. The upcast paragraph says that "declarations of two documents refuse `ill_typed`/`operator-ineligible`". FR-322-AC-94 does not state that clause and no RU case covers it. Under the TC-281 lock, which selects one document, the only foreign-owner case is the unselected M2, which refuses `missing-selection`. Either AC-94 should state the clause with a two-document vector, or the AC's silence should be deliberate, as for `conforming_reference`. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1137-1141,1861 |

## Verdict

Mergeable once the two minor coverage gaps are closed or accepted. Every recorded vector result is correct, and every RU row traces FR-322-AC-94. FND-001 and FND-002 are AC clauses with no vector, and FND-003 is a prose clause with no AC.

## Resolution

FND-001 is closed by RU-13 (`Both` to `Coded`) and the new node vector M13 for `Coded`. FND-002 is closed by RU-14 (a reference to the relationship M4 as the source). FND-003 is closed by dropping the two-documents clause from the upcast paragraph, so the prose states only what FR-322-AC-94 covers.
