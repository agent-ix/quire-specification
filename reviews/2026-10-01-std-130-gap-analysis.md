---
id: SR-729
title: "Gap analysis — PR #167 STD-130 definition lock accessor"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@ce802b78d2569db659890e1b8a4c587e850192ee; PR #167 diff against origin/main 4634f5f; acceptance taken from Linear STD-130; tests TC-233 (tests/checked_package_v2.rs), tests/checked_package_model.rs"
review_set: subset
---
# Gap analysis — PR #167 STD-130 definition lock accessor

## Summary

Ticket: STD-130. No plan bundle exists for this ticket, so the acceptance
criteria are the ticket's ask, checked against the code at ce802b7.

| Ask (STD-130) | Result |
| --- | --- |
| A small Rust `[lib]` exposing `complete-value-lock.json` via `include_str!` | Met: `src/lib.rs` `COMPLETE_VALUE_LOCK` |
| Also expose the native diagnostics catalog document | Met: `NATIVE_DIAGNOSTICS` |
| No parsing logic and no copied data | Met: two `include_str!` of the authored files; no other items |
| Bytes come from the file TC-233 tests | Met: both TC-233 tests and `checked_package_model.rs` now read through the lib constant; no test file still `include_str!`s either file |
| Consumers depend on it by git at branch=main | Met: a scratch git consumer of ce802b7 resolved one package and built only the lib |

TC-233 still proves every lock entry's digest equals the SHA-256 of the
artifact bytes it names (`definition_lock_entry_matches_bytes`, with stale,
relabelled, wrong-revision and wrong-role negatives) and now parses the lock
from `quire_specification::COMPLETE_VALUE_LOCK`. Bindings checked:
`tc_233_complete_value_lock_catalogs_every_artifact_and_separates_package_selection`
and `tc_233_package_selection_vectors_admit_exact_conditional_and_alternative_profiles`
both trace TC-233 / FR-201-AC-6, which is what they test.

The diagnostics catalog is not in the value lock, so a consumer that needs its
digest hashes `NATIVE_DIAGNOSTICS` itself, as the lib docs say.

## Verdict

**Approve.** One low finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new public library API has no owning requirement or interface artifact in spec/; AGENTS.md asks for /specify before implementation. The owner may rule a two-constant accessor exempt. | src/lib.rs:1-20 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | FR-201-AC-7 ends "and the library exposes no digest", but its only test (`tc_233_definition_library_exposes_the_exact_authored_bytes`) checks the bytes alone, so that clause has no test that could fail. It is also a negative statement. Drop the clause from AC-7 and "It records no digest." from the section. | spec/objects/foundation/FR-201-canonical-identity-domain.md:96,110 |
| FND-003 | low | FR-201-AC-7 has no row in the complete-v1 Test Matrix; FR-201-AC-6 has one (`FR-201 | FR-201-AC-6 | TC-233 | planned`). | spec/complete-v1/tests.md:542 |

## Dispositions

Round 1, reviewed at 21e5a7bda4b2acfffe077af1b2649f24f1d9caf9. FR-201's
`quire validate` output is identical at 4634f5f and 21e5a7b (23 lines: the
existing missing `id` frontmatter and 21 `invalid-model-cell` rows), so the
edit adds no validation error, as the coder said.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 21e5a7b |
