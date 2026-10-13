---
id: SR-7887
title: "Integrity review of QSpec PR #241 (ancestor-typed slots, event_type model form)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@cde3a67651d092fdb8c16d3e54b12fc8646b7b89; spec/functional/type-model/FR-143,FR-144,FR-149; spec/objects/foundation/FR-208; spec/objects/interfaces/FR-322; spec/test-cases/TC-188,TC-189,TC-194,TC-233,TC-235; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/node-identity-preimage.schema.json; tests/checked_package_v2.rs"
review_set: subset
date: 2026-10-13
---

## Summary

Ticket: QSL-737. PR: quire-specification#241 (branch spec/child-composite-slots-and-event-form). Requirement source: QSL FR-359 'Cross-repo requirements' 5 and 7 (quire-spec-language PR #800), with the planner rulings: an event uses a new model form (family record, field projection admitted, equality refused), and a model record's runtime key is its model node key.

Consistency of the FR-143/144/149/208/322 edits, the two V2 schema enum additions and the TC-188/189/194/233/235 rows against each other and against QSL FR-359 cross-repo requirements 5 and 7. The FR-208, FR-143, FR-144, FR-149, FR-340 and schema edits implement requirements 5 and 7 and agree with each other; the new FR-322 rules contradict FR-322's unchanged normative operand-fit and `member_of` text, leave `structural.ne` out, and lean on declaration keys FR-143 does not define for model records.

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
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| schema.json OwnerPresenceRule | proposals/checked-package-v2/schema.json | examined |
| node-identity-preimage ModelDeclarationNode | proposals/checked-package-v2/node-identity-preimage.schema.json | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-322's normative operand rules were not amended and now contradict the new 'Record conformance' rules. The `member_of` row (line 1216) and line 863 ('`member_of` holds exactly when operand 0 resolves to the declaring node') still require exact identity, so RC-05 (`record.project` over `GiftTaxedMoney` naming `Money`) is refused by them and admitted by 'Record conformance'. The fit rule (lines 1154-1156: an argument fits 'exactly when' its family is in the group and every constraint holds) admits `structural.eq` over two `event_type` operands (family `record` is in `structural_kind`; `same_type` holds), but the new Equality bullet and ET-07 refuse them. FR-322 calls that section normative and says the catalog copies it. Fix: restate `member_of` as 'operand 0 is, or conforms to, the declaring record type node', and either add a catalog constraint kind for `structural.eq`/`ne` that refuses `event_type` operands or add an explicit exception to the fit rule. A new constraint kind is a change to quire-verification-contracts `contracts/checked-operation-catalog-v1.json` (`constraint_kinds` plus the `structural.eq`/`ne` entries) and to both V2 schemas' copies of the catalog (FR-322-AC-17 closedness). | spec/objects/interfaces/FR-322-checked-package-artifact.md:863-864, 1154-1156, 1216 |
| FND-002 | medium | Only `quire.op.structural.eq` refuses an `event_type` operand. `quire.op.structural.ne` has the same catalog shape (structural_kind x2, same_type) and stays admitted over events. FR-208 gives events no equality rule at all, and QSL FR-359 item 4 keeps `==` over events refused, which includes `!=`. Name `structural.ne` in the Equality bullet, FR-322-AC-68 and ET-07. The admission side should name it too: RC-07 and AC-69 cover `eq` only. | spec/objects/interfaces/FR-322-checked-package-artifact.md:952-958, 1381 |
| FND-003 | medium | FR-144 now orders records by '32-byte declaration node keys', and FR-149/FR-143 compare 'most-specific declaration keys'. FR-143 defines a declaration key only as the producer-assigned, opaque, never-recomputed and package-local key of a `composite_type` record node (FR-143-AC-6 says the same). Subtyping exists only for model records, which are `model`/`record_value_type` nodes keyed by the content-derived `ModelDeclarationNode` preimage. So the key that every new subtype rule depends on has no definition for the values those rules are about. The planner ruling (a model record's runtime key is its model node key) appears nowhere in QSpec. Extend FR-143's identity paragraph and AC-6: the declaration key of a model record or event type is its FR-322 step 2 model node key, recomputable and stable across packages that select the same document. | spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md:89-102, 240 |
| FND-004 | medium | 'Every other argument type node equals its parameter's, as before' refers to a rule FR-322 never states for `quire.op.function.call`. The catalog entry has `rest: any_term` and no constraint, and FR-322 states argument typing only for `dispatch_call` (lines 865-870). The record relaxation therefore has no base rule. RC-02/RC-03 (`Money` at a `TaxedMoney` parameter; `Sequence<TaxedMoney>` at `Sequence<Money>`, refused `operator-ineligible`) depend on an exact-match rule that is not written down. State that each `function.call` argument's type node equals the callee's parameter type node, except for record conformance, and say where the refusal arises. | spec/objects/interfaces/FR-322-checked-package-artifact.md:940-943 |
| FND-005 | low | The new vectors RC-01..RC-07 ('record conformance') reuse the labels of TC-281's RC-01..RC-09 ('reference conformance', FR-322-AC-33, on main). Both sets are FR-322 operand-conformance vectors over the same catalog constraints, so a test comment or report that cites 'RC-03' is ambiguous. Use a distinct prefix such as `RCF-`, or move the vectors to TC-281 (see the evidence review). | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:290-296 |

## Verdict

Findings to fix in this PR before merge.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | The Calls rule checks each `quire.op.function.call` argument against 'the type node of the parameter at its position' and says the parameter types are 'those its declaration gives'. A checked-package `function` node carries no parameter list: its `semantic_type` is the result type, its `declaration` is a qualified name, and the parameter nodes (`value`/`parameter`) are not linked to it by position (positive-all-families.json). A reader therefore cannot apply the count or type check for a `function.call`. The test hands `params` in directly (record_conformance.rs `call`), so it cannot catch this. The gap predates this PR: line 444's 'signature type nodes' already assume a signature. Either state where a callee's ordered parameter types are read in the package, or defer to a ticket that adds that structure. dispatch_call is decidable, because it reads the operation's parameters from the selected document. | spec/objects/interfaces/FR-322-checked-package-artifact.md:942-949, 443-446 |
| FND-007 | low | FR-143 now makes a model record's key stable and equal across checked packages that select one document. That makes 'a runtime composite value from another checked package is invalid_runtime_input' false for model records: such a value carries a key of this package's graph and is admitted under the key-membership rule (lines 114-116). Scope AC-9 to source-declared composites, or say a model record value from another package that selects the same document is admitted. | spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md:252 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | high | The new `parameters` member repeats a parameter list the package already carries. QSL FR-092 'Function nodes' builds every function node's body as an aggregate whose first binding is `parameters = aggregate{[reference(p0), ..., reference(pn-1)]}`: the `value`/`parameter` nodes in parameter order, each with `semantic_type` its type. A QSL-emitted function node will now carry its parameter types twice, as `parameters` and as the semantic_types of its body's parameter references. No rule says the two agree. The Calls rule reads the member while evaluation binds the body. FR-322 also says the member enters 'the structural preimage', which QSL FR-092 owns and whose member table has no `parameters`. That is a cross-repo change to QSL FR-092 that QSL-737's cross-repo requirement list does not contain. Reviewer correction: round-1 FND-006 said a function node 'carries no parameter list'. That is true of QSpec's own text and fixtures (FR-322 never states the function body layout, and positive-all-families' `helper` body is an empty aggregate), but QSL FR-092 does carry one. The planner ruling rested on that incomplete premise. Recommended fix, for the planner to re-rule: either (a) drop the wire member and have FR-322 state the function body layout of QSL FR-092 and read the parameter types from the body's `parameters` binding, or (b) keep the member, require it to equal the body binding's parameter-node types in order, and add the QSL FR-092 preimage and emitter amendment to QSL-737's cross-repo list. Either way, say whether clause functions (`self` and operation parameters) carry it. | spec/objects/interfaces/FR-322-checked-package-artifact.md:120-131, 973-985 |
| FND-009 | low | The `application_node_preimage` definition still lists exactly `{version, node_tag, semantic_form, semantic_type, declaration, recursion, body}`. The optional `parameters` member that line 126 says enters this preimage, and that the preimage schema and the test oracle add, is missing from the list that defines it. Add 'and `parameters` when the node carries it'. | spec/objects/interfaces/FR-322-checked-package-artifact.md:301-305 |
| FND-010 | low | FR-143-AC-9 now admits a model record value from another package when its key is 'a declaration key of the evaluating package's selected document'. The body still admits only keys 'of that package's graph'. A `TaxedMoney` value for a model record the evaluating package never emitted a node for satisfies the AC and fails the body (TC-188 R14). Use one test in both places. | spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md:114-117 |

## Dispositions

Round 1, reviewed at agent-ix/quire-specification@6fa92f760058a4654ed5122d81d9dc8eec980647.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6fa92f7 |
| FND-002 | fixed | 6fa92f7 |
| FND-003 | fixed | 6fa92f7 |
| FND-004 | fixed | 6fa92f7 |
| FND-005 | fixed | 6fa92f7 |

Round 2, reviewed at agent-ix/quire-specification@3713ea827bb7118792208a79f1585176d617baa6 (commits bfe731b and 3713ea8).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-006 | fixed | 3713ea8 |
| FND-007 | fixed | bfe731b |
