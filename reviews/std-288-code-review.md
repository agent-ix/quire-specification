---
id: SR-15840
title: "Code review of QSpec PR #371 (STD-288) derived identity projection"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@59998eb9349b6680d3d8d499276d7042527b53a5; tests/checked_package_v2.rs, tests/backend_provider_v1.rs, proposals/checked-package-v2/schema.json, proposals/checked-package-v2/fixtures/adverse.json, proposals/checked-package-v2/fixtures/positive-*.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---

## Summary

Ticket: STD-288. The scope is `git diff origin/main...59998eb`. `origin/main` (`5ee0071`) is an ancestor of the head. The review uses the `rust-review` lane for the two test files.

Independent checks, by a Python script that does not use the harness:

- For each of the 12 positive fixtures, the main-side `identity_preimage.identity_projection` equals `semantic_graph.nodes` with `occurrences` removed, in graph order. Every other `identity_preimage` member and every other top-level member is unchanged. The recorded `package_id` is unchanged and equals the SHA-256 over sorted-key compact JSON of the head-side `identity_preimage` plus the derived projection. It also equals the same hash of the main-side preimage.
- In `schema.json`, `NodeBase` is closed (`additionalProperties: false`). Its properties are exactly the removed `NodeProjection` properties plus `occurrences`. So "node minus `occurrences`" is the old projection shape, member for member.
- `IdentityPreimage` stays closed and no longer lists `identity_projection`, so a shipped copy fails the schema.

`cargo test --test checked_package_v2 --test backend_provider_v1` passes, with 119 and 23 tests. QSL conformance (`cargo test -p qsl-package --lib checked_v2`, QSL main `b2c306e`) passes 42 of 42 with `QSPEC_DIR` at QSpec main and 38 of 42 at this head. The four failures are `conformance_i2_read_over_qspec_checked_package_v2_fixtures`, `conformance_c14_source_map_lookup_over_qspec_positive_fixtures`, `qspec_union_value_application_keys_are_admitted_by_i2` and `conformance_dependency_selection_vectors`. In each one, the QSL reader refuses `malformed_wire` at `/identity_preimage` because the member it still requires is missing, or it hashes `identity_preimage` without the projection. This is the expected break for the QSL lock bump.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| occurrence_free_projection / package_id_preimage / derive_package_id_preimage | tests/checked_package_v2.rs | examined |
| package_nominal_nodes_are_valid | tests/checked_package_v2.rs | examined |
| owner_join_refusal (projection-owner branch removed) | tests/checked_package_v2.rs | examined |
| tc_233_an_identity_preimage_that_ships_a_projection_is_refused | tests/checked_package_v2.rs | examined |
| tc_233_package_identity_ignores_excluded_members_and_binds_every_named_member | tests/checked_package_v2.rs | examined |
| tc_431_package_member_carries_the_v2_bytes_of_its_package_id | tests/backend_provider_v1.rs | examined |
| IdentityPreimage / NodeBase | proposals/checked-package-v2/schema.json | examined |
| shipped_projection_cases | proposals/checked-package-v2/fixtures/adverse.json | examined |
| identity_preimage of 12 positive fixtures | proposals/checked-package-v2/fixtures/positive-*.json | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | In `tc_233_an_identity_preimage_that_ships_a_projection_is_refused`, the assertion "the package-id preimage ignores a shipped copy" cannot fail. `package_id_preimage` always inserts the derived projection over any existing `identity_projection` key. The `expected_locus` assertion compares fixture data with a literal, and no reader result is involved. The schema refusal is the real check, and because the base is asserted valid before the single insertion, it is sound. Drop the tautological assertion, or assert something a reader produces. | tests/checked_package_v2.rs:13336-13345 |

## Verdict

One low finding, and the change is otherwise correct. The derivation is implemented in one helper and used at every place that formerly hashed `identity_preimage`. No main-side reader of `identity_projection` remains in either test file. The two fixture helpers that patched both the graph and the projection now patch only the graph. The removed projection-owner refusal has no remaining caller.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at f8be235. The test now checks three things that can fail. First, the shipped preimage differs from the lock-only derivation. Second, the package id rebuilt from the lock and graph (`derive_package_id_preimage`) equals the published one. Third, `expected_locus` is checked against the document: it names a member of the candidate and none of the base, and removing it gives back the base, which the schema admits. The trace moves to FR-322-AC-100. Gates: 119 + 23 passed. |
