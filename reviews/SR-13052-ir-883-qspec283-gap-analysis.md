---
id: SR-13052
title: "IR-883 QSpec PR 283 gap analysis: closure vectors and AC-35 coverage"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@7baa1104b8a37f8606a6c05a9e0c43fba8ddada8 vs origin/main a2230e1; FR-322-AC-35, FR-323-AC-7 vs proposals/checked-package-v2/dependency-selection-vectors.json, tests/checked_package_v2.rs; quire-contract-ir@d7bf519 tests/conformance_qspec"
review_set: subset
---

## Summary

Ticket: IR-883. Plan completion: not assessed.

**Vector reproduction.** A throwaway reviewer test, not committed, used `quire_verification_contracts::jcs_sha256` over each base fixture's `identity_preimage` with the vector's `dependency_selections` substituted. Every new value reproduces:

- `test/a` (all-families base plus `test/leaf` 09e949d6...) gives c7ff6307a348...
- `test/b` (clause-operations base plus `test/leaf` 8f969c06...) gives fcb75a5001f4...
- The `test/leaf` digests are the real recorded and recomputed `package_id`s of positive-all-families and positive-clause-operations, which have empty selections.
- The transitive-only reference's `node` ac15b477... is the real `Example.run` function node of positive-all-families, the package selected as `test/leaf`.

So the diamond really has two direct dependencies reaching one identity, `test/leaf`, at two different real `package_id`s. The transitive-only reference names a package that is in the derived closure and is not a direct entry.

**Existing vectors.** The JSON diff only appends a `closure_vectors` member. The existing members are byte-unchanged, and base `package_id` 331fed52... is still recorded. quire-contract-ir origin/main d7bf519 `conformance_qspec` with `QUIRE_SPECIFICATION_DIR` set to this PR head gives 14 passed, 1 ignored (IR-630, pre-existing); this includes `tc_048_qspec_version_free_selections_derive_the_recorded_package_id`.

**Gates.** QSpec `cargo test --locked` passes at the PR head, and clippy `-D warnings` passes. `cargo fmt --check` fails in tests/checked_package_model.rs, which the PR does not touch; the failure is the same on origin/main, so it is pre-existing.

**Matrix.** `quire matrix` shows FR-322-AC-35 going from `tagged tests/checked_package_v2.rs:5331` on main to `untagged` on the PR (see SR-13053 FND-001). FR-323-AC-7 is untagged on both.

## Examined criteria

- FR-322-AC-35 (examined), against tests/checked_package_v2.rs `tc_233_dependency_selections_carry_each_dependency_package_identity` and the closure vectors.
- FR-323-AC-7 (examined): untagged before and after; the BP-06 transitive coverage is planned (honest).
- TC-233 "Their reader outcomes remain planned until the IR implementation lands" (examined: honest).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No test in QSpec or in IR conformance reads `closure_vectors`. The parts QSpec can check today are left unasserted: each direct dependency's `package_id` recomputes from its base plus `dependency_selections`, the diamond's two locks share an identity at distinct ids, and the reference's `package` is a closure-only entry. The reviewer reproduced all of them by hand, but nothing in the repo would catch a later edit that breaks them. A TC-233 test should assert these producer-side facts, keyed to the new criterion | proposals/checked-package-v2/dependency-selection-vectors.json:27-53; tests/checked_package_v2.rs:5329 |

## Verdict

**PASS with a medium finding.** The vectors are real and reproduce, the existing vectors are unchanged, and the statuses are honest. The new vectors still need a QSpec-side consumer.
