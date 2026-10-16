---
id: SR-11742
title: "gap analysis of quire-specification PR 265 (AC to test, FR-322/340/341/342)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@71f8ac5ec0f95f3e16c3dbf3394b517ec3806286; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-340-checked-package-frame-location.md, spec/objects/interfaces/FR-341-checked-package-state-clause-body.md, spec/objects/interfaces/FR-342-checked-package-operation-anchor-body.md, tests/checked_package_v2.rs, tests/model_member_types.rs"
review_set: subset
---

## Summary

Ticket: QSL-815. This maps every new or revised AC of the PR to its tagged
tests (`#[trace]`). Plan completion: not assessed.

| AC | Tests |
| --- | --- |
| FR-322-AC-72 | `tc_280_process_state_machine_and_persistence_owners_join_the_selected_document` (OF-01..08, P1..P6) |
| FR-340-AC-3, AC-12 | `tc_233_frame_entries_join_a_declared_node_of_the_required_meaning`, `tc_233_a_declaring_node_that_is_none_of_the_three_is_no_context` |
| FR-340-AC-7 | `tc_233_frame_body_outputs_are_exactly_the_schemas_required_members` |
| FR-340-AC-19, AC-20 | `tc_233_a_process_or_state_machine_frame_holds_no_receiver_scoped_member` (RF-01..10), `tc_280_frame_field_entries_resolve_among_the_fields_of_a_process_or_state_machine` (FP-01..07) |
| FR-341-AC-11 | `tc_233_a_process_or_state_machine_declares_the_anchor_frame_and_clauses_of_its_operations`, `tc_280_a_clause_on_a_process_or_state_machine_binds_self_result_and_parameters` (CP-01..10) |
| FR-342-AC-7, AC-8, AC-9 | the two TC-233 tests above, plus `tc_280_an_anchor_context_is_an_object_type_a_process_or_a_state_machine` (AP-01..09) |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-340-AC-16 and FR-340-AC-17 have no test, and the oracle cannot decide them. The `conflicting-binding` refusal for two scopes of one field and its precedence, the scope order key, the `deletes_scoped` canonical order, and a `parameter` entry naming no reference parameter are all missing. `frame_entries` omits `scope`, and `FrameGroup::MEMBERS` leaves out `DeletesScoped`. This predates the PR, but the PR revises FR-340 and AC-20 now places the receiver refusal in that precedence. IR-816 needs these published vectors. Add the oracle rules and vectors in this PR. | spec/objects/interfaces/FR-340-checked-package-frame-location.md:320-321; tests/checked_package_v2.rs:2995, 3051-3076 |
| FND-002 | medium | Recorded. FR-341-AC-12 is only partly tested. No test covers an invariant anchored at a record value type, an event type, a persistence interface or a systems interface. None covers an invariant at an operation anchor, or a precondition at the process node, with a process context. The tests cover the CP signature rows. `ANCHOR_CONTEXTS` is shared with the anchor `context` join, so the FR-342-AC-9 test catches a list regression only indirectly. | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:188 |
| FND-003 | low | Recorded. The second sentence of FR-342-AC-8 has no test for a process or state-machine context: two anchors of one context share one frame, and a frame never joins two declaring types. The check (frame `semantic_type` equals the anchor `context`) does not depend on the form, and FR-342-AC-6 covers it for object types. | spec/objects/interfaces/FR-342-checked-package-operation-anchor-body.md:128 |

## Verdict

One blocking finding: FND-001, a pre-existing AC with no test in the FR this PR
revises. Every new AC (FR-322-AC-72, FR-340-AC-19/20, FR-341-AC-11, FR-342-AC-7/8/9)
and every revised AC (FR-340-AC-3/7/12) has a tagged test that can fail. The
exception is the FR-341-AC-12 and FR-342-AC-8 parts recorded above.
