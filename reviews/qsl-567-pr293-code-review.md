---
id: SR-13399
title: "Code review of QSpec PR #293 (QSL-567): TC-431 memory_models schema test"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@c5f1e93; spec/objects/interfaces/FR-331-backend-provider-envelope.md, proposals/backend-provider-v1/schema.json, tests/backend_provider_v1.rs; ticket QSL-567"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---

## Summary

Ticket: QSL-567. PR: quire-specification#293 at c5f1e93 (c5f1e93cbe3fed6a7f5c453974e4ba2f171c0cd8), scoped to `git diff refs/remotes/origin/main...c5f1e93cb` (origin/main 0ad3768). The diff touches three files. FR-331's manifest gains an optional `memory_models` member, and its terminal-record `warnings` list gains `memory.load-buffering-shape{load, store}`. The schema's `BackendDescriptor` gains the member and an `else`-forbid clause, and FR-331-AC-30 is added with the tagged TC-431 test `tc_431_memory_models_is_a_nonempty_unique_subset_on_temporal_descriptors`. No vector changed. Code review with the rust-review lane, over the one new test and the schema change it exercises. Mirror of `fairness_kinds`: the naming (snake_case plural set), the placement (next to `fairness_kinds` in `BackendDescriptor.properties`), `uniqueItems`, the closed item enum, the `else: not required` restriction to temporal descriptors and the coverage by `additionalProperties: false` all mirror it. The member differs by design in two ways: it is optional where `fairness_kinds` is required on a temporal descriptor, so existing descriptors and every existing vector stay valid, and it has `minItems: 1` where `fairness_kinds` has none, as the PR states. The names `sc`, `tso` and `ra` match QSL ADR-025 MM-1, QSL FR-226 and QSpec FR-434/FR-439 exactly. Gates: `cargo test --test backend_provider_v1 --test checked_package_v2` passes on the head (10 and 108). `quire validate --scope .` (quire 0.36.2, engine 0.50.2; the rollout note names 0.36.1/0.50.1, so the binary is newer, not older) on FR-331 exits 3 on the head with the same two errors as on main (`[value_object] required 'id'` and `semantic.record-invalid` `fields`) and the same legacy-properties warning, so this PR adds none. `quire matrix` lists FR-331-AC-30 `tagged` at tests/backend_provider_v1.rs:628. Mutations, each run and reverted: M1 drop `minItems` fails the test (good); M2 drop `uniqueItems` fails (good); M3 admit `arm` fails (good); M4 delete the non-temporal `else` clause passes (exit 0, survives). Probes: a terminal record that carries `memory.load-buffering-shape{load, store}` is refused by the schema; a mixed temporal and `operation-contract` descriptor with `memory_models` is admitted. `rustfmt --edition 2024 --check` on the test file is clean, and `cargo clippy --tests --test backend_provider_v1` emits no warnings. The `#[trace("TC-431", "FR-331-AC-30")]` tag names the right AC.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| tests/backend_provider_v1.rs::tc_431_memory_models_is_a_nonempty_unique_subset_on_temporal_descriptors | tests/backend_provider_v1.rs | examined |
| FR-331-AC-30 | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| schema-BackendDescriptor-memory_models | proposals/backend-provider-v1/schema.json | examined |
| schema-Kind | proposals/backend-provider-v1/schema.json | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The test's "not a temporal-satisfaction descriptor" case cannot fail. Its `other` descriptor advertises `state-satisfaction`, which is not in the closed `Kind` vocabulary. The schema refuses it at `/advertises/0/kind` whether or not `memory_models` is present (probe: `other` without `memory_models` is valid=false). Mutation M4, which deletes the new `allOf` clause `else: not required memory_models`, passes the whole suite (exit 0). With the kind changed to `operation-contract`, the test passes on the head and fails under M4. So AC-30's last clause is untested. Use a real non-temporal kind (for example `operation-contract`). Assert that the same descriptor without `memory_models` is admitted, so the refusal is attributable to the member. Optionally add a mixed temporal and other-kind descriptor that admits the member. Mutations M1 (drop minItems), M2 (drop uniqueItems) and M3 (admit `arm`) each fail the test, as they should. | tests/backend_provider_v1.rs:652-660 |

## Verdict

Not mergeable. One high finding: the non-temporal refusal case uses the kind `state-satisfaction`, which is not in the vocabulary, so it passes vacuously, and deleting the new `else` clause (M4) passes the suite. The empty, repeated and unknown cases each catch their mutation (M1-M3), the positive cases pin the admitted sets, and the test is idiomatic: a closure builds each variant, and each assertion has a message.
