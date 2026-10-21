---
id: SR-13677
title: 'Code review of QSpec PR #321: population identity in the owner-join tests'
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@8602df3; git diff origin/main...8602df3 (PR #321, branch qspec-population-identity): tests/model_member_types.rs, proposals/checked-package-v2/model-owner-form-vectors.json; context tests/checked_package_v2.rs; ticket QSL-869"
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/TC-280
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
---

## Summary

Ticket: QSL-869. PR: quire-specification#321 (private), head 8602df3f72d07c72d6366aaa4aad675d6d4a38b6. The diff changes tests/model_member_types.rs (`declarations()` and `owner_declares` read the population `identity`; a shared `case_projection` helper applies a case's `projection_override`; the population-key test asserts the projection carries no population `node`) and the owner-form vectors. Mutation results: reading `node` in the owner join fails PR-01 (2 tests); accepting `identity` or `node` fails PR-07 (2 tests); reading `node` in `declarations()` fails 4 tests; dropping PR-07's override, giving its entries an `identity`, ignoring the override in `case_projection`, or expecting admission each fail PR-07 (2 tests); renaming `archive`'s identity fails P8 and PR-02; adding `node` to the real entries fails only the new shape assertion. Every mutant is killed. P7 and P8 recompute independently (Python JCS, sorted keys, compact) to the published keys, and their `owner.node` values are the two entries' `identity`. The author is right that the fixtures' `populations` members are abstraction-relation bodies (positive-clause-operations.json nodes/20, empty). But the full-reader harness in tests/checked_package_v2.rs, which this PR does not touch, still uses the old shape: see FND-001. Gates at HEAD (own clone, own CARGO_TARGET_DIR): `cargo test --test model_member_types --test checked_package_v2 --test backend_provider_v1 --test checked_package_model` passes all: model_member_types 24, checked_package_v2 114, backend_provider_v1 14, checked_package_model 4. `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents, identical to main apart from one FR-322 warning shifting 9 lines; the touched files add none. quoin 0.28.3, quire 0.36.2 (engine 0.50.2).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| tests/model_member_types.rs:143-147 | tests/model_member_types.rs | examined |
| tests/model_member_types.rs:491-496 | tests/model_member_types.rs | examined |
| tests/model_member_types.rs:2237-2245 | tests/model_member_types.rs | examined |
| tests/model_member_types.rs:2340-2345 | tests/model_member_types.rs | examined |
| model-owner-form-vectors.json:/owner_cases/PR-07 | proposals/checked-package-v2/model-owner-form-vectors.json | examined |
| tests/checked_package_v2.rs:2698-2722,13025-13030 | tests/checked_package_v2.rs | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | QSpec's own full-reader harness still uses the shape the amended FR-322 forbids. `document_declares_owner` joins a population owner on `declared["node"]` (:2704 through `declares_in`, :2722). tc_233_a_population_reference_names_the_declared_population_it_reads builds the selected document with `populations: [{"node": "ix://acme/orders/counters"}, {"node": "ix://acme/orders/archive"}]` (:13027-13030). Under the amended owner join those entries declare no population, so `counters-all-instances-admitted`, `counters-lookup-admitted` and `archive-all-instances-admitted` should refuse, not admit. The test passes only because the harness reads `node`. Reproduced: making the harness join on `identity` fails that test at :13098. This is also the shape IR copied into its conformance tests (quire-contract-ir tests/it/checked_package_v2_owners.rs:623-626). Fix: inject real FCD population records (`identity`, `displayName`, `kind`, `members`, `extent`, `origin`) at :13027, join on `identity` at :2722, and recompute the selection digest. | tests/checked_package_v2.rs:2698-2722 |
| FND-002 | low | The new shape assertion checks only that there is no `node` and that `identity` is a string. The vectors claim the real `PopulationRecord` shape, but a dropped `kind`, `members`, `extent` or `origin`, an extra member, or a malformed `origin` still passes, so the vectors can drift from FCD's POPULATION_MEMBERS again unnoticed. Fix: assert the exact member set (the six required plus optional `symmetric`), `extent` in {closed, open}, `kind` as `{module, name}` and `origin` as exactly one of `source` or `generated`. | tests/model_member_types.rs:2340-2345 |

## Verdict

Not mergeable: FND-001 is high. The owner-form tests are sound and every mutant is killed, but the TC-233 full-reader harness in this repo still reads and injects `{node}` populations, so QSpec's own suite contradicts the amended FR-322-AC-82 and still hands the old shape to consumers. Fix it in this PR. FND-002 is a small hardening.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The FR-322 SHALL refuses two kinds of entry: one that lacks an `identity` string, and one that carries a `node` member. The only published case is `population-entry-without-an-identity-refused`, whose entries are `{node}` only, so they do both at once. Each branch of the step-1 pre-check is therefore untested on its own. Making the pre-check ignore `node` (`!population["identity"].is_string()`) survives the suite, and so does making it ignore a missing identity (`population.get("node").is_some()`). The 'carries a `node` member' branch matters for IR: the one-line IR fix (read `identity` at model_members.rs:2235) would admit an entry that has both `identity` and `node`, because IR's population loop does no closed-member check. So IR also needs `if population.get("node").is_some() { malformed }`, and no published case would catch its absence. Fix: add two adverse cases (and TC-233 and AC-82 wording): `population-entry-with-a-node-member-refused`, the published entries plus a `node` member, and `population-entry-without-an-identity-refused`, a published entry with `identity` removed. Both expect `invalid_model_binding`/`malformed-declaration`. | tests/checked_package_v2.rs:2908-2921 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at 85df168: tests/checked_package_v2.rs:2706-2713: let declares_population = // { document["populations"].as_array().into_iter().flatten().any(/population/ population["identity"] == owner_node) }; ... :13051-13058: document["populations"] = if case["document_populations"] == "node-only" { json!([{"node": ...}, {"node": ...}]) } else { adverse["selected_document_populations"].clone() }; ... "digest": unqualified_jcs_sha256(&document), |
| FND-002 | fixed | Round 1 at 85df168: tests/model_member_types.rs:2330-2370: the six required members are all present; nothing beyond them and `symmetric` appears; `identity` starts ix://; `extent` is closed or open; `kind` has `module` and `name`; `members` are strings; `origin` has exactly one of `source` or `generated`. |
