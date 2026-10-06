---
id: SR-1301
title: "Gap analysis of PR #191 (QSL-638): owner wire ACs against TC-233, TC-280 and TC-395"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@b6292c68792ca76b68f122c9e11a8b49f356509e; PR #191 diff against origin/main 60630b0; FR-322-AC-29, FR-322-AC-50 to FR-322-AC-55, FR-440-AC-4; TC-233, TC-280 MW-01 to MW-14, TC-395; tests/checked_package_v2.rs, tests/model_member_types.rs"
review_set: subset
---
# Gap analysis of PR #191 (QSL-638)

## Summary

Ticket: QSL-638. The analysis is planless. Plan completion: not assessed. Each AC was mapped to the tests that carry its trace tag, and each test was checked to see whether it asserts the AC's clauses. Two adverse checks were confirmed by mutation: `union_arms` forced true, and the owner join removed. Each made its test fail.

Examined:
- FR-322-AC-29 against `tc_280_model_declaration_node_vectors_recompute_and_recover_only_selected_own...` and MW-01 to MW-10 (examined)
- FR-322-AC-50 against `tc_233_positive_fixtures_carry_derived_structural_keys` and `tc_233_every_positive_fixture_carries_owners_exactly_where_the_rule_requires` (examined)
- FR-322-AC-51 against `tc_233_every_positive_fixture_carries_owners...`, `tc_233_owner_mutations_fail_the_schema_as_malformed_wire` and `tc_233_a_changed_owner_refuses_stale_node_key_at_that_node` (examined)
- FR-322-AC-52 against `tc_233_an_unjoined_owner_refuses_missing_selection_ahead_of_a_stale_key` and `tc_233_a_moved_declaration_region_refuses_invalid_value_at_its_source_map_entry` (examined)
- FR-322-AC-53 against `tc_233_two_owners_share_content_keyed_nodes_and_differ_on_owner_keyed_ones` (examined)
- FR-322-AC-54 against `tc_233_the_key_stage_leaves_an_abstraction_relation_node_to_fr_451` (examined)
- FR-322-AC-55 against MW-07 and MW-10 to MW-14 (examined)
- FR-440-AC-4 against `tc_395_a_case_must_hold_one_arm_per_union_member_in_order` (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-29 still requires that "a node whose key was computed under another tag (the relationship's id on a `model` node whose owner names an object type)" refuses `invalid_package`/`stale-node-key`. MW-06 used to carry that case and now carries the owner-join case instead. No MW vector puts M4's id on a `model`/`object_type` node owned by `ix://acme/orders/Order`, so that clause has no test. Add it as a new MW row. | spec/test-cases/TC-280-model-owned-member-types.md:110; spec/objects/interfaces/FR-322-checked-package-artifact.md:1001 |
| FND-002 | medium | The `tc_395_a_case_must_hold_one_arm_per_union_member_in_order` test is tagged FR-440-AC-4 but asserts only two of its six clauses: arms out of order (swapped) and a member omitted (arm dropped). A repeated member, a binder count that differs from the payload arity, a binder typed other than its payload position, and an arm body whose type is not the `result_type` are untested. Mutating the arity or binder-type sub-checks of `union_arms_hold` would survive, and the arm-body defect (SR-1300 FND-002) is invisible. | tests/checked_package_v2.rs:11002 |
| FND-003 | medium | FR-322's owner join says a clause function's `ModelOwner` names "an operation member, or an object type for an invariant clause". FR-322-AC-55 and MW-11 to MW-14 test only the passing pair and the undeclared or field owners. No vector puts a non-invariant clause under an object-type owner, so the clause-kind half of the rule has no adverse test, and the readers that ignore it (SR-1300 FND-001) pass. Add an AC-55 clause and an MW row: a precondition clause function owned by `ix://acme/orders/Order` refuses `missing_declaration`/`missing-selection`. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1027 |

## Verdict

FR-322-AC-50 to AC-54 are covered, with oracles that bite. AC-50 holds through the full reader over all nine positive fixtures, with per-node owner and projection assertions. AC-51 covers all ten schema mutations and the changed-owner stale key. AC-52 covers both unjoined owner kinds, the ordering ahead of a lower-digest stale key, and the source-map `invalid-value`. AC-53 covers the shared and differing ids, the group recompute and the crossed-owner stale key. AC-54 covers the FR-451 identity refusal. AC-55's listed cases are covered. FR-322-AC-29 has one clause without a vector (FND-001). FR-440-AC-4 is a third covered (FND-002). The clause-kind rule has no adverse case (FND-003). Close all three in this PR.

## Dispositions

Round 1, reviewed at b6910d8 (fix range b6292c6..b6910d8). Focused tests at b6910d8: checked_package_v2 86/0 and model_member_types 12/0, measured.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f4f815a |
| FND-002 | fixed | f4f815a |
| FND-003 | fixed | 0f4002a |

## New findings (disposition pass 2)

Over the new commits b6910d8..d8f3dbc. The dropped AGE-2226 cases were checked for dangling AC/TC references across spec/, tests/ and proposals/. There is no remaining `Digit`, `billedTo`, MT-08..10, MT-15 or MA-14..16 reference outside the TC-280 status note. TC-412's `AN-02` is that TC's own id.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | FR-342-AC-4 still requires that `(Sub, "size")` resolves to `Sub`'s redefinition. AN-02 was dropped, and the published document's `Sub` declares no operation, but FR-342-AC-4 carries no AGE-2226 status note like the ones added to FR-322-AC-30 and AC-44. That clause now has no vector and no test while its row still claims Test (TC-280). Add the same status note to FR-342-AC-4. | spec/objects/interfaces/FR-342-checked-package-operation-anchor-body.md:105 |
| FND-005 | low | FR-322-AC-44's verification cell still reads Test (TC-280), but TC-280 has no AC-44 row left and `tc_280_text_value_type_members_carry_their_bounds_and_profile` was deleted. The status note explains why, but the cell points at a TC that does not test it. Say "Test (TC-280, pending AGE-2226)" or equivalent so the matrix reads as deliberate, not dangling. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1060 |

## Dispositions (round 3)

Round 3, reviewed at e938a77 (fix range d8f3dbc..e938a77).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | e938a77 |
| FND-005 | fixed | e938a77 |

## New findings (disposition pass 4)

Scoped to bc04ca78..5555986.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | `tc_280_a_declaration_refusal_retains_the_nodes_origin_verbatim` covers a source origin, a generated origin and a missing origin, but not the malformed-origin half of "a node whose `origin` is missing or malformed retains no origin". This is why SR-1300 FND-008 passes. Add a `declaration_origin_cases` entry with a both-branches origin expecting no retained origin. | tests/model_member_types.rs:1869-1894 |

## Dispositions (round 5)

Round 5, reviewed at 6fd6acf (fix range 5555986..6fd6acf).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-006 | fixed | 6fd6acf |
