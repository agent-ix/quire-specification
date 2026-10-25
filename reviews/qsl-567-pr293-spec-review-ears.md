---
id: SR-13400
title: "EARS analysis of QSpec PR #293 (QSL-567): memory_models and load-buffering warning text"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@c5f1e93; spec/objects/interfaces/FR-331-backend-provider-envelope.md, proposals/backend-provider-v1/schema.json, tests/backend_provider_v1.rs; ticket QSL-567"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
---

## Summary

Ticket: QSL-567. PR: quire-specification#293 at c5f1e93 (c5f1e93cbe3fed6a7f5c453974e4ba2f171c0cd8), scoped to `git diff refs/remotes/origin/main...c5f1e93cb` (origin/main 0ad3768). The diff touches three files. FR-331's manifest gains an optional `memory_models` member, and its terminal-record `warnings` list gains `memory.load-buffering-shape{load, store}`. The schema's `BackendDescriptor` gains the member and an `else`-forbid clause, and FR-331-AC-30 is added with the tagged TC-431 test `tc_431_memory_models_is_a_nonempty_unique_subset_on_temporal_descriptors`. No vector changed. EARS analysis of the edited requirement text. FR-331's governing statement ("When a contract package is submitted to a backend provider, the producer SHALL use ...") is unchanged and event-driven. The new manifest sentence is an optional-feature clause ("may also carry") under it. The warnings sentence extends a member list. AC-30 states one schema rule, with its admitted and refused cases, and is observable. Gates: `cargo test --test backend_provider_v1 --test checked_package_v2` passes on the head (10 and 108). `quire validate --scope .` (quire 0.36.2, engine 0.50.2; the rollout note names 0.36.1/0.50.1, so the binary is newer, not older) on FR-331 exits 3 on the head with the same two errors as on main (`[value_object] required 'id'` and `semantic.record-invalid` `fields`) and the same legacy-properties warning, so this PR adds none. `quire matrix` lists FR-331-AC-30 `tagged` at tests/backend_provider_v1.rs:628. Mutations, each run and reverted: M1 drop `minItems` fails the test (good); M2 drop `uniqueItems` fails (good); M3 admit `arm` fails (good); M4 delete the non-temporal `else` clause passes (exit 0, survives). Probes: a terminal record that carries `memory.load-buffering-shape{load, store}` is refused by the schema; a mixed temporal and `operation-contract` descriptor with `memory_models` is admitted.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-331-manifest-memory_models | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-accounting-warnings | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-AC-30 | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331 | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Both new sentences turn on terms that QSpec does not define. The manifest member's condition, "decides weak-memory subjects", and the warning's condition, "for a weak-memory item", are neither defined in QSpec nor observable. FR-434 and FR-439 say "an item over a protocol subject with a weak `parallel`" and "resolved model". The optional-feature clause ("may also carry") then has no testable trigger. Use FR-439's term in both places: "a backend that decides items over a subject with a weak `parallel`" and "for an item whose resolved model is `ra`" (or whichever models FND-002 of the base review settles on). | spec/objects/interfaces/FR-331-backend-provider-envelope.md:61 |

## Verdict

Mergeable on this method alone, with one low finding: the triggers "decides weak-memory subjects" and "for a weak-memory item" use a term that QSpec does not define, where FR-439's "subject with a weak `parallel`" exists. The statement form is otherwise sound, and the new prose describes the system rather than the change.

## Dispositions

Round 1, against head 48428b7 (48428b7c037c3d8070b5ff99b320d1bac9015eb1). Against head 48428b7 (48428b7c037c3d8070b5ff99b320d1bac9015eb1). The fix commit changes the schema (a third `Warning` arm and the `memory_models` description), FR-331 (the manifest wording, the warnings bullet, an FR-439 relationship and AC-31), FR-439 (routing by the FR-331 member, "absent means `sc` only", the load-buffering warning rule and a new AC), TC-431 (row BP-19) and the test file. It also commits the three round-0 review files under `reviews/`, each byte-identical to the reviewer's round-0 copy. `cargo test --test backend_provider_v1 --test checked_package_v2`: 11 and 108 pass. Mutations, each run and reverted: M1 drop `minItems`, M2 drop `uniqueItems`, M3 admit `arm`, M4 delete the non-temporal `else` clause, M7 drop the new `Warning` arm, M8 make `store` optional, M9 open the arm (`additionalProperties`), and M10 let `load` be anything. Each fails the suite (exit 101), so M4 is now killed. `quire validate --scope .`: FR-331 has the same two errors as on main, and the legacy-properties warning only moves from line 59 to line 61. FR-439 and TC-431 exit 0, the same as main, and the three review files exit 0. `quire matrix`: FR-331-AC-30 is tagged at tests/backend_provider_v1.rs:628 and FR-331-AC-31 at :681. FR-439 lists two rows with the id FR-439-AC-7.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
