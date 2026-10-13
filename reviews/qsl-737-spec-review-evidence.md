---
id: SR-7889
title: "Evidence review of QSpec PR #241 (ancestor-typed slots, event_type model form)"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-specification@cde3a67651d092fdb8c16d3e54b12fc8646b7b89; spec/functional/type-model/FR-143,FR-144,FR-149; spec/objects/foundation/FR-208; spec/objects/interfaces/FR-322; spec/test-cases/TC-188,TC-189,TC-194,TC-233,TC-235; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/node-identity-preimage.schema.json; tests/checked_package_v2.rs"
review_set: subset
date: 2026-10-13
---

## Summary

Ticket: QSL-737. PR: quire-specification#241 (branch spec/child-composite-slots-and-event-form). Requirement source: QSL FR-359 'Cross-repo requirements' 5 and 7 (quire-spec-language PR #800), with the planner rulings: an event uses a new model form (family record, field projection admitted, equality refused), and a model record's runtime key is its model node key.

Verification evidence for the new and amended criteria. The new test `tc_233_event_type_is_a_model_declaration_node_with_a_model_owner` discriminates: against the main-branch schemas, ET-01 is refused and ET-02 is admitted by `OwnerPresenceRule`, and the event_type preimage fails the old `ModelDeclarationNode` (python jsonschema 4.26 against origin/main: old false, new true). The enum additions change no existing preimage or golden, because the preimage content is unchanged and only the admitted set widens. FR-322-AC-69 and most of FR-322-AC-68 have no executable test, and the oracle change is exercised by no test.

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
| tests/checked_package_v2.rs::is_model_declaration | tests/checked_package_v2.rs | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-69 (RC-01..RC-07) has no test. FR-322-AC-68's owner-join, projection, invariant-clause-owner and equality clauses (ET-05..ET-07) have no test either; only ET-01..ET-04, the schema half, are tested. The amended clause of FR-322-AC-31 is untested too. TC-233's own description excludes reader behaviour ('does not exercise a production reader ... those criteria remain with TC-217 and TC-187'), so placing RC/ET-05..07 there makes them untestable as scoped. The repo already has an executable home for exactly these checks: TC-281 with `model-member-type-vectors.json` (`reference_equality_cases`, `projection_cases`) and `tests/model_member_types.rs`, which decides `member_of`/`conforming_reference` per catalog constraint and traces FR-322-AC-30/31/33/34. FR-322-AC-67 (three traced tests) is the precedent for operand-check ACs. Add record-conformance and event cases to that harness, keeping the 'Test' verification honest. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1382 |
| FND-002 | low | The oracle change adds `event_type` to `is_model_declaration` but no test reaches it. The new test only calls the jsonschema validators. The only fixture `event_type` and `record_value_type` nodes carry a `declaration`, so the predicate returns false for them either way. The predicate also still omits `record_value_type`, which FR-322 lists as a model declaration node, so the oracle now treats undeclared `event_type` and `record_value_type` nodes differently (stale-key routing in `stale_derived_shape_node`, owner kind in `required_owner_kind`). Add `record_value_type` and cover both with an undeclared-node case, for example the ET-01 package run through `stale_derived_shape_node` and `required_owner_kind`. | tests/checked_package_v2.rs:2381-2391 |

## Verdict

Findings to fix in this PR before merge.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The three new tests rebuild FR-322's rules in Rust (`conforms`, `slot_admits`, the event check) over a hard-coded document and assert inline expectations. They do not apply a published vector document. Reverting the FR-322 prose would leave them green; only the catalog-shape asserts (the constraint kinds of each entry, `leaves: operand:0`) read an external artifact. Unlike TC-281's other cases in `model-member-type-vectors.json`, which run against the real acme-orders document and its recomputed keys, the EV/RCF vectors are not published as data, so the IR reader (quire-contract-ir) cannot consume them. The tests are acceptable as a reference oracle for this PR. Publish EV/RCF as vector data in a follow-up so the reader implementation is checked against them. | tests/record_conformance.rs:1-495 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | PF-05 and PF-06 assert that adding `parameters` changes the digest of the repo's own `application_node_preimage` and `structural_node_preimage`. Those functions were edited in this commit to copy the member in, so the two assert_ne checks only test that edit, and the structural one checks no published schema. The real evidence is PF-01..PF-04 (V2 schema), the preimage-schema admit/refuse checks and PF-07 (dependency shape). Pin PF-05/06 to a recorded preimage digest, or drop the inequality asserts. No published structural-preimage schema exists to check PF-06 against. | tests/checked_package_v2.rs:12695-12779 |

## Dispositions

Round 1, reviewed at agent-ix/quire-specification@6fa92f760058a4654ed5122d81d9dc8eec980647.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6fa92f7 |
| FND-002 | fixed | 6fa92f7 |

Round 2, reviewed at agent-ix/quire-specification@3713ea827bb7118792208a79f1585176d617baa6 (commits bfe731b and 3713ea8).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-003 | deferred | TC-281 now says the EV/RCF vectors are a reference oracle applied by tests/record_conformance.rs over inline data and not yet published as a data file; publishing them as vector data is left to a follow-up (no ticket named yet). Low severity; not blocking. |
