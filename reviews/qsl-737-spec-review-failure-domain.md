---
id: SR-7888
title: "Failure-domain review of QSpec PR #241 (ancestor-typed slots, event_type model form)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@cde3a67651d092fdb8c16d3e54b12fc8646b7b89; spec/functional/type-model/FR-143,FR-144,FR-149; spec/objects/foundation/FR-208; spec/objects/interfaces/FR-322; spec/test-cases/TC-188,TC-189,TC-194,TC-233,TC-235; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/node-identity-preimage.schema.json; tests/checked_package_v2.rs"
review_set: subset
date: 2026-10-13
---

## Summary

Ticket: QSL-737. PR: quire-specification#241 (branch spec/child-composite-slots-and-event-form). Requirement source: QSL FR-359 'Cross-repo requirements' 5 and 7 (quire-spec-language PR #800), with the planner rulings: an event uses a new model form (family record, field projection admitted, equality refused), and a model record's runtime key is its model node key.

Edge cases of carrying a record subtype value at an ancestor-typed slot through the checked-package rules, and of the event-type equality refusal. The FR-144 key order is total and agrees with FR-149 equality: distinct declarations compare by key bytes first, one declaration has one key, and records only meet records in a record-typed collection, so no kind clash arises. Three gaps remain: subtype-only text fields have no pinned leaf, FR-208's new payload and element slots have no admitting checked-package rule, and the event refusal covers only a top-level operand.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-208 | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md | examined |
| FR-208-AC-14 | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md | examined |
| FR-144 | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md | examined |
| FR-144-AC-15 | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md | examined |
| FR-143 | spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md | examined |
| FR-143-AC-5 | spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md | examined |
| FR-149 | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | examined |
| FR-149-AC-17 | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md | examined |
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-31 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-68 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-69 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-233 | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |
| FR-340 | spec/objects/interfaces/FR-340-checked-package-frame-location.md | context_only |
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 inner_type | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 'The leaves are those of `P`' leaves text fields that only a subtype declares with no pinned leaf, law or mode. Two operands of static type `Money` that both hold `TaxedMoney` have one most-specific declaration, so FR-149's plan descends 'one child pair per field' over every effective field of `TaxedMoney`. If `TaxedMoney` adds `note: Text[0,8;nfc]`, that comparison has no `{path, laws, mode}` leaf in the application. FR-322 otherwise refuses a missing text leaf (`operation-law-missing`, FR-322-AC-59). The same applies to `Option<Money>`, collections and tuples over `Money`. State where the law for a subtype-only text field comes from: the subtype's declared field type, or a rule that the leaf list covers every subtype in the selected document. Add a vector for it. | spec/objects/interfaces/FR-322-checked-package-artifact.md:952-956 |
| FND-002 | medium | FR-208 now admits a subtype value at `Option` payload, collection element, tuple position and union payload slots. FR-322 'Record conformance' relaxes only call arguments, `record.project` and `structural.eq`, which is exactly the list in cross-repo requirement 5. The lowered forms of the other slots stay exact-type: `quire.op.collection.contains` carries `inner_type` ('the same element type node'), so `contains(s: Set<Money>, t: TaxedMoney)` is refused. The collection literal (`uniform_rest`), option, tuple, union and record value construction state no conformance either. A QSL program that FR-208 and FR-359 AC-6/AC-7 call valid therefore lowers to a package the reader refuses, unless QSL always types the element at `P` first. Either state that the lowered element or payload term is typed at `P` (the producer's obligation), or extend 'Record conformance' and the `inner_type` row. This goes beyond requirement 5's enumerated list, so it needs a planner ruling. | spec/objects/foundation/FR-208-quire-meaning-vocabulary.md:236-241 |
| FND-003 | low | The event-equality refusal tests only 'an operand whose type node is a `model`/`event_type` node'. An `Option<OrderPlaced>`, a tuple holding one, or a `Set<OrderPlaced>` (whose element type must admit FR-149 `=`, FR-144) reaches event equality through `structural_kind` without being refused. FR-208 lets no field or parameter carry an event value, so this arises only inside event-invariant clause bodies that use `self`. State the refusal for an event type reached at any depth of the compared type, and for an event as the element type of a set, bag or ordered set. | spec/objects/interfaces/FR-322-checked-package-artifact.md:956-958, 1381 |

## Verdict

Findings to fix in this PR before merge.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The Slots list leaves out result positions. A function or expression whose body root is an application must have `semantic_type` equal to that `result_type`, and `return:0` and `arm_body` fix the result type exactly. So `function f(): Money { make_taxed() }` (body root a `function.call` with `result_type` `TaxedMoney`) and a `case` with a `Money` arm and a `TaxedMoney` arm are refused. FR-208 says a subtype value is a value of the ancestor wherever a result names it, and QSL FR-359 item 6 requires a P-typed result to admit a child. Add the result positions (a function or expression `semantic_type` against its body root's type, a `case`'s arm bodies against its result) to the Slots list, with a vector. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1186-1189, 957-968 |
| FND-005 | low | Event equality is refused only for `structural.eq`/`ne`. The author says keyed constructors and `contains` over event elements are refused upstream by FR-144, but that refusal holds only if FR-149 says an event type admits no `=`. Neither FR-149 nor FR-144 mentions event types, and FR-149's record row gives every family-`record` type equality. A package with `quire.op.collection.set` over `OrderPlaced` rest arguments, or `contains` over a `Set<OrderPlaced>`, is admitted and decides membership by an equality FR-208 withholds. Fix: one FR-149 sentence (an event type, or a type that contains one, admits no `=`), and extend the exception to `contains` and the keyed constructors. It only arises in event-invariant bodies, so it is low. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1196-1206 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | FR-322's Results bullet admits a `case` whose arm bodies are typed `Money` and `TaxedMoney` with result type `Money` (RCF-12). FR-440 still says the `arm_body` form 'derives the type every arm body has' and the `result_type` 'must be that type node'. FR-440-AC-4 still refuses 'an arm body of a type other than the `result_type`'. The PR amended FR-440 item 3 ('Union value') but not the case rule, so the two FRs contradict for RCF-12. Amend FR-440's `arm_body` sentence and AC-4 with the record-conformance exception, as item 3 was. | spec/objects/interfaces/FR-440-checked-package-union-and-case-nodes.md:84-88, 217 |

## Dispositions

Round 1, reviewed at agent-ix/quire-specification@6fa92f760058a4654ed5122d81d9dc8eec980647.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6fa92f7 |
| FND-002 | fixed | 6fa92f7 |
| FND-003 | fixed | 6fa92f7 |

Round 2, reviewed at agent-ix/quire-specification@3713ea827bb7118792208a79f1585176d617baa6 (commits bfe731b and 3713ea8).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | bfe731b |
| FND-005 | fixed | bfe731b |

Round 3, reviewed at agent-ix/quire-specification@d2ec448dcdf6829020e30c0594d5f7dec728f803.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-006 | fixed | d2ec448 |
