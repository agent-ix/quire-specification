---
id: SR-503001
title: 'Spec review of quire-specification PR #460 (STD-503): Boolean model-field state/transition body'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@ac717128 (PR #460, spec/std-503-state-transition-body, base e3d48d76); FR-341 State transition body, FR-341-AC-15..17; FR-464 operator catalog and FR-464-AC-19; FR-322 member sentence; TC-233 ST-01..ST-15; proposals/checked-package-v2 schema.json StateTransitionBody, fixtures/positive-state-transition.json, fixtures/positive-clause-operations.json, dependency-selection-vectors.json, README; tests/checked_package_v2.rs'
review_set: subset
date: '2026-10-27'
relationships:
- target: ix://agent-ix/quire-specification/FR-341
  type: reviews
- target: ix://agent-ix/quire-specification/FR-464
  type: reviews
---

## Summary

Ticket: STD-503. PR: quire-specification#460 (head ac717128). Consumers read: IR-1052 (IR reader/writer), QSL-1163 (QSL emission), IR-1049 (CG FR-003@4 producer).

Verified independently (own script, first reproduced all 15 existing positive fixtures: node keys and package ids all match; the one `correspondence` node in positive-clause-operations is keyed by a different preimage and is skipped identically on main and on the PR): all 14 node keys and the package id d3ff644e of positive-state-transition.json recompute; positive-clause-operations recomputes to package id efe00705 (26 nodes, only the placeholder transition removed); both dependency-selection-vectors ids (4da290aa direct dependency, efe00705 lock entry) recompute from the base package id; nothing is fudged. The fixture validates against schema.json and its application preimages against node-identity-preimage.schema.json. Schema closedness mutants (extra body, operation, binding and member keys; reordered, renamed, missing or sixth binding; two `pre` references; aggregate or literal `subject`; literal input; extra law; missing result_type, leaves or member; second occurrence role) are all refused; an empty `pre` aggregate is admitted by design. `quire validate`: identical diagnostics before and after (changed files and whole spec tree, modulo line numbers). `quire matrix` lists FR-341-AC-15 and AC-17 (tagged), FR-464-AC-19 (tagged) and AC-16 (method-without-symbol, declared Analysis). Criterion ids FR-341-AC-15..17 and FR-464-AC-19 are appended, unused on origin/main, #459 and every other open PR; ST-01..ST-15 are new in TC-233. Merge-tree of #460 with origin/main 2b991805 (which now contains #459) is clean; the two PRs touch different hunks of FR-322, FR-464 and TC-233, so landing order does not matter and #459 has already landed.

Downstream (QSpec head vs QSpec main): QSpec `cargo test --test checked_package_v2` 151 passed / 3 failed on the PR, 150 / 3 on main, the same three failures (K15 protocol node vector, model-root nominal vector, execution-point-initialization), no new red. QSL `conformance` in qsl-semantics: 7 pass. QSL `conformance` in qsl-package: 13 pass / 2 FAIL on the PR, 15 / 0 on main: both failures are the new fixture (FND-001). IR `conformance_qspec`: 54 pass / 17 fail / 1 ignored on the PR, and the identical 17 fail on QSpec e3d48d76 and on QSpec main 2b991805, so none is caused by the rekey or the new fixture (IR conformance lists fixtures by name and does not read the new one). Nothing in QSL or IR hardcodes d8373551 or the old transition node.

The ruling is complete for the body positions IR-1052, QSL-1163 and IR-1049 need (anchor, subject, ordered Boolean inputs, optional pre, post, Boolean result, claim occurrence, whole-node identity) and is consistent with FR-322 reader order, FR-342 point-bound anchors, FR-464 (the "exactly these operators" wording was amended) and the catalog entry quire.op.state.transition (operator class state_transition, rest any_term, result clause, law protocol_profile, member profile_operator): no quire-verification-contracts change is needed. It is not shippable as is because the published fixture is not admitted by the strict reader.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-341 State transition body and reader order | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:166 | examined |
| FR-341-AC-15 | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:329 | examined |
| FR-341-AC-16 | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:330 | examined |
| FR-341-AC-17 | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:331 | examined |
| FR-464 operator catalog second table and FR-464-AC-19 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:132 | examined |
| FR-322 member sentence | spec/objects/interfaces/FR-322-checked-package-artifact.md:338 | examined |
| TC-233 ST-01..ST-15 | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:574 | examined |
| StateTransitionBody | proposals/checked-package-v2/schema.json:67 | examined |
| positive-state-transition.json | proposals/checked-package-v2/fixtures/positive-state-transition.json | examined |
| positive-clause-operations.json, dependency-selection-vectors.json | proposals/checked-package-v2/ | examined |
| tc_233_state_transition_package_binds_anchor_subject_pre_and_post | tests/checked_package_v2.rs:18746 | examined |
| FR-342 operation anchor body, FR-322-AC-31 and MT-03, protocol-finite.md | spec/objects/interfaces/FR-342-checked-package-operation-anchor-body.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | positive-state-transition.json is refused by the QSL strict V2 reader: `ill_typed`/`operator-ineligible` at /semantic_graph/nodes/7/body/result_type, the subject `record.project` of `note`. The fixture types the subject as `scalar_type`/`boolean`, but in the acme/orders domain package Order.note is presence `optional`, so FR-322-AC-31 and vector MT-03 give `Option<Boolean>`; the fixture's lock selects that package and its Order node is M1. QSL `cargo test -p qsl-package --lib conformance` (QSPEC_DIR = PR) goes red (c14_source_map_lookup and i2_read over the positive fixtures, 2 failed vs 15 green on main) because it globs positive-*.json. The new QSpec test hides it: its operation oracle skips the `query` projection ("TC-280 owns it"). FR-341's text says only "the field is Boolean" and is silent on optional Boolean fields, so the spec and fixture also disagree about what is in scope. | proposals/checked-package-v2/fixtures/positive-state-transition.json:1 |
| FND-002 | medium | The fixture's operation `Order.reset()` has no parameters, so `inputs` is empty: no positive coverage of the one non-trivial binding (ordered Boolean parameter references, which IR-1049's `input_current` needs), and ST-13 "`inputs` missing a Boolean parameter of the operation" and ST-12 "a projection of a non-Boolean field" cannot be produced by editing this package, though TC-233 says each reader vector edits it. acme/orders has no required Boolean field and no operation with a Boolean parameter, so FND-001 and FND-002 need one supply: a model (domain package) with a required Boolean field and an operation with a Boolean parameter, selected by the fixture and published for the readers. | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:595 |
| FND-003 | medium | No placement refusal for transition: FR-341 now says a `quire.op.state.transition` application "stands nowhere else" and a `state`/`transition` node's body root is that application, but the reader order lists only state_clause placement; unlike FR-341-AC-6 (state.clause at another root refuses `ill_typed`/`operator-ineligible`, nested refuses `malformed_wire`) no criterion or step gives the cause and locus for a transition application at another node's root, nested, or a `state`/`transition` node with another body root. IR-1052 would have to invent it. | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:276 |
| FND-004 | medium | The profile law is stated but its definition is not: FR-464 says the finite-global profile defines the operator `transition`, yet the profile definition artifact named by the law (proposals/quire-v1/definitions/protocol-finite.md) admits only protocol forms and does not mention a state transition. A state-only package must select the protocol profile (the author's open question 1: yes, forced by the catalog law role protocol_profile and FR-322 profile_selections equality; accepted) but nothing in the profile definition says why. Add the sentence there, and record that the QSL emitter must write the protocol_profile selection whenever it emits a transition (it enters package_id). | proposals/quire-v1/definitions/protocol-finite.md:21 |
| FND-005 | low | Overclaims against the test: ST-01 says "admitted by the operation step" and FR-341-AC-17 says "validates against both V2 schemas", but the test checks only four forms (deref, unary, state_clause, transition) and only schema.json (the preimage schema validates by my check). ST-08 says the clauses and the anchor take new ids and ST-09 says the transition id is unchanged under a region move; the test asserts only the transition id changing and the package id staying equal. | tests/checked_package_v2.rs:18746 |
| FND-006 | low | The new fixture is not in positive_fixtures(), so the fixture-wide checks (identity-preimage derivation, lock and selection checks, derived structural keys, exact dependencies) do not run on it; only the one new test does. | tests/checked_package_v2.rs:13532 |
| FND-007 | low | FR-342 is not extended: anchor occurrences are "one per clause or protocol attempt", the transition now also names a (usually no-point) anchor, and "the anchor of the same point as its clauses" is vacuous when both `pre` and `post` are present (distinct points); say the no-point anchor, as FR-464-AC-17 does for attempts. | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:211 |
| FND-008 | low | Vector ids ST-01..ST-15 repeat TC-374's ST-01..ST-07 (stochastic timed), so a bare "ST-03" citation is ambiguous; the helper comment cites a "State transition operator" table that does not exist (the header is "State operator"). | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:574 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | low | FR-341-AC-18 (Test, TC-233) is `untagged` in `quire matrix`: the test that executes TS-16 and TS-18 carries trace tags for FR-341-AC-15, AC-17 and FR-464-AC-19 only. | tests/checked_package_v2.rs:18795 |
| FND-010 | low | The renamed vector ids TS-01..TS-18 now repeat TC-369's TS-01..TS-14 (timed semantics), the same ambiguity FND-008 named for ST-nn against TC-374; a bare "TS-03" citation is still ambiguous. | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:575 |

## Dispositions

Round 1, reviewed at ae57549d (merge of main 2b991805 included).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The fixture is now `Switch.set(flag: Boolean)` over the required Boolean `on`; QSL qsl-package conformance 15 pass / 0 fail and qsl-semantics 7 pass with QSPEC_DIR = ae57549d; FR-341 states required single-valued Boolean subjects, `Option<Boolean>` refused (FR-341-AC-16, TS-12); the QSpec test now runs the operation step over the whole package, projection included |
| FND-002 | fixed | `inputs` is [flag]; acme/orders gained `Switch` (id, on, level; set(flag)) in domain-package-acme-orders.json and model-member-type-vectors.json; TS-12 and TS-13 are producible from the package; every digest recomputes (independent script) |
| FND-003 | fixed | FR-341 Placement paragraph and FR-341-AC-18: root elsewhere refuses `ill_typed`/`operator-ineligible` before any anchor check, nested fails the schema, wrong body root fails the schema; TS-16, TS-17 |
| FND-004 | fixed | protocol-finite.md gains "Checked-package state transition carrier" (profile selection required, `operation-law-unselected`); TS-18 executed by the test |
| FND-005 | fixed | The test validates every application preimage under the preimage schema, runs the operation step over the whole package, asserts the renamed ids (anchor, both clauses, transition) and unchanged ids and package id under a region move |
| FND-006 | fixed | positive-state-transition is in positive_fixtures() |
| FND-007 | fixed | The transition's anchor is the no-point anchor (FR-464-AC-17 precedent); FR-342 occurrences name `state`/`transition` nodes |
| FND-008 | fixed | Vector ids renamed (but see FND-010); the helper comment now cites the "State operator" table |

## Verdict

Round 1 (ae57549d): FND-001..008 fixed; two new low findings (FND-009 missing trace tag for FR-341-AC-18, FND-010 vector-id prefix collision with TC-369), neither blocking: the lead may take them as a one-line tag addition and a prefix rename in this PR or as a follow-up. Mergeable. Evidence: every node key, package id, lock digest (acme/orders 5368187144c7 to 179fb0d98f7f) and dependency-selection and dependency-call id recomputes independently for all 16 positive fixtures and the closure vectors, after first reproducing main; no old digest remains in QSpec, QSL, IR, codegen, verification-contracts or quire-cli; QSL qsl-package 15/0, qsl-semantics 7/0; QSpec checked_package_v2 151/3 (same three pre-existing failures as main), model_member_types tc_280 fails identically on main; IR conformance_qspec 54/17/1 with the identical 17 failures as IR main against QSpec main. Widening acme/orders is acceptable: QSL, IR and codegen load domain-package-acme-orders.json from the QSpec checkout at run time and none hardcodes its digest.

Original verdict was request changes (one high, three medium).
