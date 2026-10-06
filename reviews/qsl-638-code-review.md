---
id: SR-1300
title: "Code review of PR #191 (QSL-638): owner on the checked-package v2 wire, reader and fixtures"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@b6292c68792ca76b68f122c9e11a8b49f356509e; PR #191 diff against origin/main 60630b0; proposals/checked-package-v2/schema.json, proposals/checked-package-v2/fixtures/*.json, proposals/checked-package-v2/*-vectors.json, tests/checked_package_v2.rs, tests/model_member_types.rs; cross-checked against quire-contract-ir 8eb9837b (checked_package/v2/mod.rs, owner.rs)"
review_set: subset
---
# Code review of PR #191 (QSL-638)

## Summary

Ticket: QSL-638. The PR adds `owner` to the V2 schema (`NodeOwner`, `OwnerPresenceRule` on `NodeBase` and `NodeProjection`), adds the owner join to the reference reader ahead of both key passes, re-keys every structural node from its wire `owner`, adds FR-451's identity step, regenerates the fixtures with owners, adds the two-owner fixtures, and adds the FR-440 `union_arms` constraint and `arm_body` result form to the operation oracle. This review includes the rust-review lane (the reader is Rust test code).

Examined:
- schema.json `NodeOwner`, `OwnerPresenceRule`, `NodeBase`, `NodeProjection` against FR-322 "Owner member schema" (examined)
- tests/checked_package_v2.rs `owner_join_refusal`, `selected_model_document`, `document_declares_owner`, `structural_node_preimage`, `group_local_structural_preimage`, `is_rekeyed_structural_node`, `is_model_declaration`, `model_declaration_key_is_current`, `stale_derived_shape_node`, `abstraction_identity_refusal`, `read_package_semantics` order (examined)
- tests/checked_package_v2.rs `union_arms_hold`, `constraint_holds` `union_arms`, `result_descriptor` `arm_body`, `relationship_end` shortcut (examined)
- tests/model_member_types.rs `owner_declares`, `wire_key`, `read_wire` (examined)
- the nine positive fixtures and the two-owner pair: owner placement, node ids, group labels, package ids (examined)
- quire-contract-ir 8eb9837b `CheckedNodeOwner`, `CheckedSemanticNodeV2.owner`, `CheckedNodeProjectionV2.owner`, `owner.rs` requirement/join (context_only)

Measured: `cargo test --test checked_package_v2 --test model_member_types` at head: 83 and 12 passed. One mutation build: `union_arms` forced true and the `owner_join_refusal` call removed; exactly `tc_395_a_case_must_hold_one_arm_per_union_member_in_order`, `tc_233_an_unjoined_owner_refuses_missing_selection_ahead_of_a_stale_key` and `tc_233_a_moved_declaration_region_refuses_invalid_value_at_its_source_map_entry` failed. An independent Python JCS recompute confirms every positive fixture's `package_id` equals the SHA-256 of its `identity_preimage`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The owner join accepts a clause function's `ModelOwner` when its `node` is either an object type or an operation member, whatever the clause. FR-322 allows an object type only for an invariant clause. A precondition clause function owned by `ix://acme/orders/Order` is admitted by both QSpec readers, while IR 8eb9837b (`owner.rs` `model_kind_matches`) refuses it `missing_declaration`/`missing-selection`. Discriminate by clause kind in both readers once the spec defines where that kind is read (see SR-1302 FND-001). | tests/checked_package_v2.rs:2350; tests/model_member_types.rs:354 |
| FND-002 | medium | `arm_body` derives the result from the first arm's body only (`terms.get(1)?["value"]["members"][1]`). FR-440 defines it as "the type every arm body has", and FR-440-AC-4 refuses "an arm body of a type other than the `result_type`". A `case` whose second or later arm body has another type is admitted. Derive from every arm and refuse when they differ, or check each arm body against `result_type`. | tests/checked_package_v2.rs:1189 |
| FND-003 | low | A `relationship_end` member on a model-owned relationship returns `Descriptor::family("reference")` without resolving the end against the selected document. The new FR-322 sentence says it "resolves the same way" (by identity in the selected document), so any end name passes the oracle. Resolve the end in the document, or state in the TC that this oracle does not cover it. | tests/checked_package_v2.rs:1126 |
| FND-004 | low | The fixture loop that recomputes `package_id` from the derived identity preimage omits `positive-two-owners-a`, `positive-two-owners-b` and `positive-union-nodes`. The AC-53 test checks only `derive_identity_preimage` equality, so no test pins those three `package_id`s (they are correct today: measured with an independent JCS recompute). Iterate `positive_fixtures()` in that loop. | tests/checked_package_v2.rs:4441-4450 |

## Verdict

The core of the change is correct.

1. **Schema.** `NodeOwner` is `oneOf [SourceOwner, ModelOwner]`. `OwnerPresenceRule` encodes FR-322's table row for row: structural means no `nominal_identity_preimage` and `body.term` not `application`; declared means required `SourceOwner`; `model/object_type`, `model/systems_interface`, `relation/relationship` and any `function` without `declaration` mean required `ModelOwner`; everything else means absent. It sits in both `NodeBase` and `NodeProjection`. The ten `owner_mutations` each fail the schema.
2. **Owner join.** It runs after the application-key check and before `stale_derived_shape_node`. It covers every owner-bearing node in ascending digest order, checks `SourceOwner` against `lock.sources` and every `declaration` region's source, and resolves every `ModelOwner` node up front against the admitted document. The mutation run shows both join tests depend on it.
3. **Keys.** `structural_node_preimage` and `group_local_structural_preimage` copy the wire `owner`. The source-map derivation is gone. Model declaration nodes and clause functions are re-keyed. Only `correspondence/abstraction_relation` is excluded, and `abstraction_identity_refusal` (FR-451 step 1) runs after the state step and before operations.
4. **Fixtures.** Owners sit exactly where the rule requires (asserted per node, projection included). The two-owner pair differs on `Point`, on both `List` group members and on the group label, and shares `Integer` and the application-keyed `three`.
6. **Oracle strength.** The mutations above confirm that the union-arms test and both owner-join tests fail when their check is removed.

Cross-repo (IR 8eb9837b): the serde shape matches: internally tagged `kind`, `source`/`model`, `authority`/`identity` and `identity`/`node`, `deny_unknown_fields`, optional on both node and projection. The presence rule and the join refusal codes and loci also match. Divergences: the clause-kind discrimination (FND-001, rooted in SR-1302 FND-001), and a projection owner that differs from its node's owner, which IR refuses `stale_dependency` while QSpec specifies no code (SR-1302 FND-003).

Fix FND-001 and FND-002 in this PR. The low findings ride along.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The invariant branch of `document_declares_owner` accepts any object type, including one with `interfaceFeatures` (a systems interface). IR 8eb9837b (`owner.rs` `model_kind_matches`, `.is_some_and(\|object\| !object.interface)`) refuses an invariant clause function owned by a systems interface. FR-322 says "an `invariant` clause function is owned by an object type", while step 2's table uses "object type" for `object_type` (no `interfaceFeatures`) apart from `systems_interface`. Pin it in the spec (an object type with no `interfaceFeatures`) and match it in both QSpec readers, or rule the other way and file an IR ticket. | tests/checked_package_v2.rs:2405-2407; tests/model_member_types.rs:358-360 (at b6910d8) |

## Dispositions

Round 1, reviewed at b6910d8 (fix range b6292c6..b6910d8). The checks follow the plan-lead rulings as stated. The coder reported 86/0 and 12/0, and an independent run at b6910d8 measured the same. Mutation builds were queued behind the build lock and cancelled at the coordinator's request, so oracle strength for the new test edits was judged by reading the code.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 0f4002a |
| FND-002 | fixed | f4f815a |
| FND-003 | fixed | f4f815a |
| FND-004 | fixed | f4f815a |

## Dispositions (round 2)

Round 2, reviewed at d8f3dbc (fix range b6910d8..d8f3dbc). Focused tests at d8f3dbc, measured: checked_package_v2 88/0, model_member_types 15/0.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | fb31c2c |

## New findings (disposition pass 2)

Over the new commits b6910d8..d8f3dbc. The plan-lead rulings are taken as decided. The relationship refusal codes, loci and order were compared line by line with IR quire-contract-ir ce634ee6 FR-038:983-1135 and match. The hand-authored document validates against FCD origin/main 68c0acba `schema/semantic/v1/semantic-ir.schema.json` with 0 errors (measured with jsonschema 4.26 plus the referenced `common.schema.json`).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | `read_relationships` approximates FCD's relationship schema by hand. It checks the origin's closed shape, but not the relationship's, the ends' or the multiplicity's `additionalProperties: false`, the `semanticIdentity` syntax of identities and end types, the source-locus `path` pattern, or `generatorVersion` as semver. IR-661 validates "against those authoritative schema definitions". A relationship carrying an unknown member is admitted by this reader and refused `invalid_model_binding`/`malformed-declaration` by IR, and no `relationship_document_cases` entry covers it. Add one extra-member case and the closed-shape checks. | tests/model_member_types.rs:1551-1660 |
| FND-007 | low | `project_document` still reads `extensions[].payload.redefines` and a field-level `redefines`, and emits `redefines` on every projected field and operation. Ruling (6) removed that invented extension. The published document has empty `extensions`, so this path is dead code that keeps the invented member shape alive in the projection. Delete it and let AGE-2226 add redefinition from the real FCD form. | tests/model_member_types.rs:1249-1261 |

## Dispositions (round 3)

Round 3, reviewed at e938a77 (fix range d8f3dbc..e938a77). The closed-shape refusal is `invalid_model_binding`/`malformed-declaration` at the selection row, which matches IR ce634ee6 FR-038 ("an absent, null, wrong-typed, unsupported-enum or otherwise schema-invalid non-end member refuses `invalid_model_binding`/`malformed-declaration` in step 1 at the selection row"; "a malformed end or multiplicity refuses `invalid_model_binding`/`malformed-declaration`"). The field-level `redefines` read that remains is FCD's own `field.redefines` (semantic-ir.schema.json `field.properties.redefines`), not the removed extension.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-006 | fixed | e938a77 |
| FND-007 | fixed | e938a77 |

## New findings (disposition pass 4)

Scoped to the one amendment bc04ca78..5555986 (declaration refusals retain the FCD `origin`), with the plan-lead ruling taken as decided. FR-154's origin text matches FCD origin/main 68c0acba `common.schema.json`: `$defs.origin` is `oneOf` {`source`: `sourceLocus`} | {`generated`: `generatedOrigin`}; `sourceLocus` requires `sourceIdentity`, `path`, `startLine` and `startColumn`, with optional `endLine` and `endColumn`; `generatedOrigin` requires `generatorIdentity`, `generatorVersion` and `inputIdentities`, with no span. The new test is a real oracle for the source and generated cases: a reader that synthesized `{artifact, start, end}`, added a span to a generated origin, or dropped the origin would fail it. Tests were not rerun this round; the coder reports checked_package_v2 88 and model_member_types 16.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | `read_relationships_retaining` retains `relationship.get("origin")` whatever its shape, so a malformed origin (both branches, an extra member, a missing locus member) is retained as is. FR-154 says "a node whose `origin` is missing or malformed retains no origin". `relationship-origin-both-branches` therefore refuses while retaining the malformed object, against the rule. Retain the origin only when `origin_is_valid` holds. | tests/model_member_types.rs:1604-1610 |

## Dispositions (round 5)

Round 5, reviewed at 6fd6acf (fix range 5555986..6fd6acf).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-008 | fixed | 6fd6acf |
