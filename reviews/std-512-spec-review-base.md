---
id: SR-512001
title: 'Spec review of quire-specification PR #463 (STD-512): schema/reader stage boundary for XT-06/XT-07 and several postconditions on a state transition'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@7c595c48 (PR #463, spec/std-512-post-clauses-xt07) off main 02dfc7dd; FR-341 Stage boundary and Several postconditions sections, FR-341-AC-15, AC-19, AC-20, FR-464-AC-19, TC-233 XT-06, XT-07, XT-19, XT-20, tests/checked_package_v2.rs tc_233_state_transition_package_binds_anchor_subject_pre_and_post'
review_set: subset
date: '2026-10-27'
---

## Summary

Ticket: STD-512. PR: quire-specification#463, head 7c595c48, base main 02dfc7dd. Consumers read: IR-1052 (IR PR #607, FR-067 AC-2 and AC-7), QSL-1163 and its comments, QSL FR-105, FR-104, FR-107, ADR-012 section 15.4.

Merge status against current main 000cd943: does NOT merge cleanly. One content conflict, in spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md, in the FR-464-AC-19 / FR-464-AC-20 rows of the acceptance table (this PR rewrites the AC-19 row; #462 added the AC-20 row directly after it). TC-233, FR-341 and tests/checked_package_v2.rs auto-merge. Resolution is mechanical: keep this PR's AC-19 row and main's AC-20 row. Not resolved here.

Verdict: approve with two medium findings to settle (FND-001, FND-002) and two low ones. Ruling (1) is sound and verified. Ruling (2) is sound as a statement of the wire: no wire conflict with QSL FR-105-AC-4. It does not by itself satisfy IR-1052's QSL-authored AC-7 or unblock QSL-1163, because QSL has no rule that produces a transition at all.

Checked and clean:
- Schema first. Run with an independent validator (python jsonschema 4.19.2, Draft 2020-12) over proposals/checked-package-v2/schema.json: the unedited positive-state-transition.json and positive-clause-operations.json are valid; node 14 of the former with member operator `sequence` is invalid (leaf errors at `.../body/operation/member/operator` const, plus the form and role consts of the oneOf); node 4, 22 and 23 (every quire.op.protocol.control application) of the latter with operator `transition` are each invalid. The schema is not changed (no proposals/ file touched).
- Reconciliation. XT-05 and XT-06 now agree (schema failure); FR-341-AC-15 now says the schema refuses and the operation step alone refuses `operation-member-mismatch`; FR-464-AC-19 agrees with FR-341-AC-19. No other spec or proposals file states the old reachable-by-a-complete-reader outcome (grep over spec and proposals for operation-member-mismatch with transition or protocol.control). The ruling is the same doctrine as merged FR-322-AC-139 (wire precedes graph).
- XT-19 and XT-20 pass the operation step alone: tc_233_state_transition_package_binds_anchor_subject_pre_and_post is ok; it asserts the schema rejects the edited document and operation_refusal / package_operations_refusal return MemberMismatch. The control that the unedited clause package is schema-valid is asserted.
- Ids. FR-341-AC-19, FR-341-AC-20, XT-19 and XT-20 are absent on main and on the other four open PR branches (#432, #433, #440, #444). SR-512001 is absent from main and all open PR branches (SR-512 is a different, existing id).
- AC rows. FR-341-AC-1 to AC-20 contiguous. quire matrix: FR-341-AC-15, 17, 18, 19 and FR-464-AC-19 tagged to tests/checked_package_v2.rs:18826; FR-341-AC-20 and AC-16 are Analysis rows, method-without-symbol (same state as AC-16).
- quire validate (the three changed spec files) is identical before (02dfc7dd) and after: only the pre-existing FR-464-AC-18 non-canonical-shape warning.
- cargo test --test checked_package_v2 at 7c595c48: 156 passed, 2 failed, the two pre-existing failures (protocol_node_case_vectors_publish_the_qspec_examples K15, and the execution-point-initialization failure in tc_233_operation_vectors_key_every_application_node_of_the_published_packages).
- No fixture or key change: git diff 02dfc7dd..7c595c48 touches four files, none under proposals/.

Ruling (2) analysis, verified in quire-spec-language (origin/main 2417c95e5):
- QSL FR-105-AC-4 and its test s4_a_second_post_on_attempt_update_adds_one_clause_node_and_no_anchor_or_frame (qsl-replay/src/spine/clause/tests.rs:4968) say exactly what the author says: a second post adds one state_clause node and an anchor occurrence, no second anchor, no second frame. FR-105-AC-1 and the FR-105 requirements say S4 emits no `state`/`transition` node for a state clause; ADR-012 section 15.4 says the same ("QSL emits no state/transition node from a state clause"). The only QSL transition code is identity vocabulary (CheckedClauseKind::StateTransition in qsl-semantics identity.rs and node_key); there is no emitter. So today QSL produces no transition and plural posts cannot yield two transitions.
- QSL reads several posts as a conjunction (FR-107 evaluate_clause_set: True only when every entry is Completed(true); preconditions short-circuit; FR-120). So QSpec's sentence that an operation's postconditions are the conjunction of its postcondition clauses is the QSL meaning.
- The FR-003@4 claim shape (codegen scripts/kani_claims.json, harness kani_fr_003_4_proof_boolean_transition_module, subject `crate::subject(input_current, state_pre)`) takes one precondition, one postcondition and one subject. A transition that binds one of several posts, read as "the" postcondition, would give a claim weaker than the source contract. The new FR-341 text forbids that reading, in prose.
- Gap that remains, owned by QSL: QSL-1163 must introduce the first emitter, which contradicts FR-105 and ADR-012 15.4 as written, and must choose which source construct is a transition and which post it binds. QSL-1163's own comment (operator escalation e38, four options, no selection) records exactly this. The PR's ruling is one of the producer-side options (the author states one `post A and B`), but QSL cannot do that for a unit with two separate `post` clauses without either synthesizing a conjunction clause or refusing or skipping the transition. QSL emitting a conjunction state_clause is not forbidden by this ruling (it says no conjunction node is synthesized by the wire; the producer may still state one clause), but it is a QSL source-meaning decision (FR-104/FR-105/ADR-012), not settled here. Two transitions on one subject from QSL-1163 would be refused by FR-341-AC-16 (`ambiguous_declaration`/`ambiguous-name`), correctly.
- IR-1052 AC-7: QSpec's ruling satisfies the reader-side expectation (IR admits one transition bound to one of several posts; the QSL-authored positive case needs one pre, one post, one subject). It does not satisfy the QSL-authored part: AC-7 remains gated on QSL-1163 and on e38. IR FR-067 (PR #607 head f716f12d) is not a defect here, but its AC-2 still cites `operation-member-mismatch` for XT-06/XT-07; it must move to `malformed_wire` for those two and cite XT-19/XT-20 for the standalone operation step.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-341 Stage boundary for a mismatched profile operator | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:291 | examined |
| FR-341 Several postconditions on one operation | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:314 | examined |
| FR-341 State transition body (pre binding) | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:189 | examined |
| FR-341-AC-15 | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:393 | examined |
| FR-341-AC-16 | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:394 | context_only |
| FR-341-AC-19 | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:397 | examined |
| FR-341-AC-20 | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:398 | examined |
| FR-464-AC-19 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:546 | examined |
| TC-233 XT-05 to XT-07, XT-19, XT-20 and the closing note | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:596 | examined |
| tc_233_state_transition_package_binds_anchor_subject_pre_and_post | tests/checked_package_v2.rs:18826 | examined |
| schema.json ProtocolControlApplication and state transition body | proposals/checked-package-v2/schema.json | examined |
| positive-state-transition.json, positive-clause-operations.json | proposals/checked-package-v2/fixtures | examined |
| QSL FR-105, FR-105-AC-1, AC-4, ADR-012 15.4, FR-107 clause sets | quire-spec-language spec | context_only |
| IR FR-067-AC-2, AC-7 (PR #607) | quire-contract-ir spec/checked_package/functional/FR-067-admit-boolean-state-transition.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new "Several postconditions" section and FR-341-AC-20 rule only postconditions, but a transition's optional `pre` has the same shape (at most one reference) and QSL allows several preconditions per operation, evaluated as a conjunction (FR-104, FR-107, FR-120; QSL-1163 comment names both). The same consumer misreading (a transition's `pre` as the operation's complete precondition) is not forbidden. | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:314 |
| FND-002 | medium | The ruling leaves the QSL producer rule open, and the text reads as if it were settled: "the producer that wants a transition for it states one postcondition clause for that subject (the source's own `post A and B`)". No QSL requirement defines which source construct yields a transition, and QSL FR-105 and ADR-012 15.4 currently forbid emitting any. QSL-1163 and IR-1052 AC-7 stay gated on QSL decision e38. The PR text should say this is a producer obligation owned by QSL, not a QSpec-settled behaviour, so IR-1052 does not close AC-7 on it. | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:326 |
| FND-003 | low | Test-case closing note lists only XT-10 to XT-15 and XT-17 as reader outcomes the QSpec crate does not implement. XT-06 and XT-07 name `malformed_wire` at `operation/member/operator`, a code and locus the QSpec test does not exercise (it asserts only schema invalidity, and a oneOf validator reports the node plus several branch leaves). FR-341-AC-19 is labelled Test (TC-233) for both halves. | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:613 |
| FND-004 | low | XT-05 (member operator `sequence`) and XT-06 (member operator `sequence`) now state the same input and outcome; XT-06 differs only by pointing to XT-19. | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:596 |

## Dispositions

Round 1, reviewed at 6e60514f (main 000cd943 merged in, unmoved).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The FR-341 section is now "Several postconditions and preconditions on one operation": it covers `state_clause` nodes of clause `precondition` as well, says a consumer SHALL NOT read `pre` as the operation's complete precondition, and FR-341-AC-20 now reads "and likewise for two `precondition` nodes and `pre`". |
| FND-002 | fixed | The `post A and B` text is now "One producer option, not a requirement of this specification ... Which option QSL takes is QSL-owned and OPEN (QSL-1163, escalation e38). QSL FR-105 and ADR-012 section 15.4 currently say QSL emits no `state`/`transition` node"; FR-341-AC-20 and the TC-233 note both state that the QSL-authored part of IR-1052 AC-7 is not closed. |
| FND-003 | fixed | TC-233 now has a paragraph listing the vectors the crate executes (XT-01 to XT-09, XT-16, XT-18, XT-19, XT-20; verified against tc_233_state_transition_package_binds_anchor_subject_pre_and_post: XT-01 unedited package, XT-02 to XT-07 `refused(...)` schema checks, XT-08 anchor rewrite and rekey, XT-09 region move, XT-16 nesting, XT-18 unselected law, XT-19 and XT-20 operation step alone) and says the `malformed_wire` code and locus for XT-06 and XT-07 are reader outcomes IR owns. |
| FND-004 | fixed | XT-05 now lists only the law role, mode and occurrence role; XT-06 owns member operator `sequence` and cross-references XT-05. |

Merge result: clean; the only difference from main in FR-464 is the FR-464-AC-19 row, FR-464-AC-20 is byte-identical to main; no proposals/ file differs from main; the four changed spec and test files plus this SR are the whole diff. cargo test --test checked_package_v2: 156 passed, 2 failed (the same K15 and execution-point-initialization failures). quire validate: only the pre-existing FR-464-AC-18 warning. quire matrix: FR-341-AC-15, AC-19 and FR-464-AC-19 tagged; FR-341-AC-20 and FR-464-AC-20 Analysis rows. FR-341-AC-1 to AC-20 contiguous; FR-341-AC-19/20, XT-19/20 and SR-512001 unused on main and the other open PR branches. No new finding.

## Verdict

Approve with FND-001 and FND-002 settled in this PR (one sentence each, and one added clause in FR-341-AC-20) and a rebase onto main 000cd943 resolving the single FR-464-AC-19 / AC-20 row conflict. FND-003 and FND-004 are optional. Rulings (1) and (2) are sound as wire rulings. The deeper QSL question (what source produces a transition when a unit has several posts or pres) is not answered by this PR and needs QSL decision e38; that is expected, not a defect of this PR, but its text must not imply otherwise.

Round 1 verdict: all four findings fixed at 6e60514f; no new finding. Approve.
