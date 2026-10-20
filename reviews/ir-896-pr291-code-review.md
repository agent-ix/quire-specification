---
id: SR-13367
title: "Code review of QSpec PR #291 (IR-896): tc_233 single-pairing test"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@866c90d; tests/checked_package_v2.rs; ticket IR-896"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322-AC-78
    type: reviews
---

## Summary

Ticket: IR-896. PR: quire-specification#291 at 866c90d. Code review with the rust-review lane folded in. One Rust test is added to tests/checked_package_v2.rs (:10392-10432); no production code changed, so gap-analysis was not run. The `#[trace("TC-233", "FR-322-AC-78")]` tag binds the test to the right AC. The test uses the file's existing helpers (`schema_for_def`, `string_list`, `diagnostic_row_causes`, which reads FR-272 through `include_str!`). It has no panics outside assertions, no I/O and no unsafe code. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (9, 108).

Mutation runs of the new test (each reverted after): widening the schema's `unsupported-wire` pairing to `unknown_wire|unknown_required_feature` fails it; deleting the `unsupported-feature` pairing fails it; listing `unsupported-wire` in FR-272's `unknown_contract_version` row fails it; removing `unsupported-wire` from FR-272's `unknown_wire` row fails it. Rewriting FR-322's new prose to pair the causes with `unknown_contract_version`/`unknown_required_capability` passes it (the test does not read FR-322; AC-78 does not claim it does). No mutation of any reader can reach it: it calls no reader and reads no vector.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| tc_233_unsupported_wire_and_feature_causes_have_one_pairing | tests/checked_package_v2.rs | examined |
| tc_233_diagnostic_schema_pairs_each_selection_and_type_cause_with_its_codes | tests/checked_package_v2.rs | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The `for reader in [...]` block is nested inside the `for (cause, owner)` loop, but nothing in it depends on `cause` or `owner`. Its four assertions run twice with the same inputs. Move it after the loop. | tests/checked_package_v2.rs:10421-10430 |
| FND-002 | low | The schema half repeats the code-enum sweep in tc_233_diagnostic_schema_pairs_each_selection_and_type_cause_with_its_codes (:10357), which already iterates `(cause, admitted)` over every Diagnostic code. Adding `("unsupported-wire", &["unknown_wire"])` and `("unsupported-feature", &["unknown_required_feature"])` to that table, or sharing a helper, would avoid a second copy of the same sweep. | tests/checked_package_v2.rs:10398-10416 |

## Verdict

Two low findings. The test is sound for what it asserts: four of the five mutations fail it, and the one that passes is outside what AC-78 claims. Its coverage gap against AC-78's refusal clause is recorded as SR-13368 FND-001.
