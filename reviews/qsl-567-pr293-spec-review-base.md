---
id: SR-13398
title: "Base review of QSpec PR #293 (QSL-567): memory_models manifest member and load-buffering warning"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@c5f1e93; spec/objects/interfaces/FR-331-backend-provider-envelope.md, proposals/backend-provider-v1/schema.json, tests/backend_provider_v1.rs; ticket QSL-567"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---

## Summary

Ticket: QSL-567. PR: quire-specification#293 at c5f1e93 (c5f1e93cbe3fed6a7f5c453974e4ba2f171c0cd8), scoped to `git diff refs/remotes/origin/main...c5f1e93cb` (origin/main 0ad3768). The diff touches three files. FR-331's manifest gains an optional `memory_models` member, and its terminal-record `warnings` list gains `memory.load-buffering-shape{load, store}`. The schema's `BackendDescriptor` gains the member and an `else`-forbid clause, and FR-331-AC-30 is added with the tagged TC-431 test `tc_431_memory_models_is_a_nonempty_unique_subset_on_temporal_descriptors`. No vector changed. Base review: I checked the new member and warning against FR-368's `fairness_kinds`, FR-439's routing rule, the schema's `Warning` and `AccountingRecord`, TC-431, and QSL main 47047b1 (FR-226, TC-671, ADR-025 MM-1/MX-4/MX-7, and the QSL-567 #1080 gap note that asked for the manifest member). Mirror of `fairness_kinds`: the naming (snake_case plural set), the placement (next to `fairness_kinds` in `BackendDescriptor.properties`), `uniqueItems`, the closed item enum, the `else: not required` restriction to temporal descriptors and the coverage by `additionalProperties: false` all mirror it. The member differs by design in two ways: it is optional where `fairness_kinds` is required on a temporal descriptor, so existing descriptors and every existing vector stay valid, and it has `minItems: 1` where `fairness_kinds` has none, as the PR states. The names `sc`, `tso` and `ra` match QSL ADR-025 MM-1, QSL FR-226 and QSpec FR-434/FR-439 exactly. Gates: `cargo test --test backend_provider_v1 --test checked_package_v2` passes on the head (10 and 108). `quire validate --scope .` (quire 0.36.2, engine 0.50.2; the rollout note names 0.36.1/0.50.1, so the binary is newer, not older) on FR-331 exits 3 on the head with the same two errors as on main (`[value_object] required 'id'` and `semantic.record-invalid` `fields`) and the same legacy-properties warning, so this PR adds none. `quire matrix` lists FR-331-AC-30 `tagged` at tests/backend_provider_v1.rs:628. Mutations, each run and reverted: M1 drop `minItems` fails the test (good); M2 drop `uniqueItems` fails (good); M3 admit `arm` fails (good); M4 delete the non-temporal `else` clause passes (exit 0, survives). Probes: a terminal record that carries `memory.load-buffering-shape{load, store}` is refused by the schema; a mixed temporal and `operation-contract` descriptor with `memory_models` is admitted. Method `quoin:gap-analysis` was not run, because no production code changed: the only non-spec files are the wire schema and its test.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-331-manifest-memory_models | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-accounting-warnings | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-AC-30 | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| schema-BackendDescriptor-memory_models | proposals/backend-provider-v1/schema.json | examined |
| schema-Warning | proposals/backend-provider-v1/schema.json | examined |
| FR-331-manifest-fairness_kinds | spec/objects/interfaces/FR-331-backend-provider-envelope.md | context_only |
| TC-431-procedure | spec/test-cases/TC-431-backend-provider-wire-vectors.md | context_only |
| FR-439-advertisement-routing | spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md | context_only |
| QSL-FR-226-load-buffering-behaviour | quire-spec-language:spec/functional/FR-226-state-the-code-link-preconditions-of-a-weak-memory-verdict.md | context_only |
| QSL-FR-226-AC-3 | quire-spec-language:spec/functional/FR-226-state-the-code-link-preconditions-of-a-weak-memory-verdict.md | context_only |
| QSL-ADR-025-MX-4 | quire-spec-language:spec/decisions/ADR-025-weak-memory-models-for-parallel.md | context_only |
| QSL-ADR-025-MM-1 | quire-spec-language:spec/decisions/ADR-025-weak-memory-models-for-parallel.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The schema refuses the new warning that the prose adds. FR-331 now lists `memory.load-buffering-shape{load, store}` among a terminal record's `warnings`. But the schema's `Warning` is a closed `oneOf` of two arms, `fairness.strong-would-exclude` and `cross-check-absent`, each with `additionalProperties: false`, and this PR does not change it. I checked this with a probe on the head: an `AccountingRecord` whose warnings hold `{code: memory.load-buffering-shape, load, store}` is refused (valid=false), while the same record with `cross-check-absent{tool}` is admitted. So a provider that follows the prose emits a record that FR-331's own "out-of-domain values ... refuse before consumption" rule refuses. The members `load` and `store` also have no type anywhere. QSL FR-226 says the warning names the two attempts, but it is not stated whether each is a node id, a transition identity or a site. Add a third `Warning` arm with typed `load` and `store` members, update the `Warning` description, add an AC with a positive and an adverse case (a missing `store`, an extra member), and cover it in TC-431 or a vector. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:339-341 |
| FND-002 | medium | QSpec defines no rule for when a provider emits the warning, and the one source the PR cites says something different. The other two warnings each have a QSpec owner: FR-369 for `fairness.strong-would-exclude` and FR-349 for `cross-check-absent`. This one cites only "QSL FR-226". QSL FR-226 and ADR-025 MX-4 have S3, the compiler, report it at check time, "whatever the model a request later resolves", and they leave the checked package unchanged. FR-331 instead puts it in the provider's per-item terminal record, "for a weak-memory item". Moving it to EN-1 is one of the two options in the QSL-567 #1080 gap note, so the move may be intended. But no QSpec FR states what the backend must do: whether it emits the warning for every resolved model, only for `tso`/`ra`, or only for `ra`, where MX-4 says load buffering matters (FR-221-AC-4 makes the load-buffering outcome unreachable under `tso`); whether it emits it once per shape or once per item; and whether the warning changes the result. QSL FR-226 also still says S3. State the rule in a QSpec owner, FR-439 or FR-226's QSpec half: the condition, the resolved models it applies to, and that the result is unchanged. Then cite that owner here, and pair the change with a QSL FR-226 update. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:339-341 |
| FND-003 | medium | The spec does not say what an absent `memory_models` means for routing. FR-439 and QSL FR-226 route an item over a subject with a weak `parallel` only to a candidate that advertises the item's resolved model. QSL FR-226-AC-3 routes even the `sc` request over `SB` by an advertised `sc`. A temporal backend that omits the member, which is every backend registered today, therefore advertises nothing, and no item over a weak-`parallel` subject reaches it, the `sc`-resolved ones included. Yet the text tells only a backend that "decides weak-memory subjects" to carry it, so an `sc`-only backend may read the member as optional for it. Two implementers can disagree on whether absence means {} or {`sc`}. State the meaning of an absent `memory_models`: it advertises no memory model, so the backend receives no item whose subject has a weak `parallel`. Alternatively, state that absence is {`sc`}. Either way, say which items the routing rule reads it for. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:61 |
| FND-004 | low | "A descriptor that advertises another kind carries none" is wider than the schema and AC-30 allow. A descriptor that advertises both `operation-contract` and `temporal-satisfaction` does advertise another kind, but the schema admits `memory_models` on it (probe: valid=true), and AC-30 refuses the member only on a descriptor that "does not advertise `temporal-satisfaction`". Write "a descriptor that does not advertise `temporal-satisfaction` carries none", as AC-30 does. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:61 |
| FND-005 | low | The cross-references do not agree on where the member lives. FR-439 says a candidate advertises its memory models "in its capability advertisement (FR-290)", and its Dependencies say that "FR-290 owns the capability advertisement that carries the memory models". But FR-290 never mentions memory models, and this PR puts the member in FR-331. The schema description names FR-331 as the owner, while the sibling members name their owning FR (`fairness_kinds` names FR-368). FR-331 cites FR-439 without a link and has no relationship edge to it, although it has one to FR-368. Name FR-439 in the schema description and link it in FR-331. Point FR-439's routing sentence and Dependencies at FR-331's `memory_models` member. | proposals/backend-provider-v1/schema.json:194 |
| FND-006 | low | AC-30 says "Test (TC-431)", but TC-431 has no row for it. TC-431's procedure lists BP-01..BP-10 and BP-18, each with its AC, and FR-331-AC-30 and `memory_models` appear in none of them. No positive vector carries `memory_models`, so BP-09's "every positive manifest descriptor" never exercises it. Add a BP row for AC-30 (positive: [`sc`, `tso`, `ra`] and [`ra`] on a temporal descriptor; adverse: [], a repeated model, an unknown model, and a non-temporal descriptor with a valid kind). Also consider giving a positive temporal vector a `memory_models` member. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:391 |

## Verdict

Not mergeable. One high finding: the prose adds a `memory.load-buffering-shape{load, store}` warning that the closed `Warning` schema refuses, and its members have no type. Two medium: no QSpec rule says when a provider emits that warning, and the rule the PR cites (QSL FR-226) puts it in S3; and the routing meaning of an absent `memory_models` is left open. Three low: "advertises another kind" is wider than AC-30, the FR-439, FR-290 and FR-331 cross-references disagree on the owner, and TC-431 has no row for AC-30. Sound as written: the `memory_models` member mirrors `fairness_kinds` where it should and differs only in the two stated ways; it is optional, so every existing backend and vector stays valid; the model names match QSL; AC-30 is one schema rule with its refusals, like AC-22.

## New findings (disposition pass 1)

Against head 48428b7 (48428b7c037c3d8070b5ff99b320d1bac9015eb1). The fix commit changes the schema (a third `Warning` arm and the `memory_models` description), FR-331 (the manifest wording, the warnings bullet, an FR-439 relationship and AC-31), FR-439 (routing by the FR-331 member, "absent means `sc` only", the load-buffering warning rule and a new AC), TC-431 (row BP-19) and the test file. It also commits the three round-0 review files under `reviews/`, each byte-identical to the reviewer's round-0 copy. `cargo test --test backend_provider_v1 --test checked_package_v2`: 11 and 108 pass. Mutations, each run and reverted: M1 drop `minItems`, M2 drop `uniqueItems`, M3 admit `arm`, M4 delete the non-temporal `else` clause, M7 drop the new `Warning` arm, M8 make `store` optional, M9 open the arm (`additionalProperties`), and M10 let `load` be anything. Each fails the suite (exit 101), so M4 is now killed. `quire validate --scope .`: FR-331 has the same two errors as on main, and the legacy-properties warning only moves from line 59 to line 61. FR-439 and TC-431 exit 0, the same as main, and the three review files exit 0. `quire matrix`: FR-331-AC-30 is tagged at tests/backend_provider_v1.rs:628 and FR-331-AC-31 at :681. FR-439 lists two rows with the id FR-439-AC-7.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | medium | The new FR-439 criterion reuses the id FR-439-AC-7, which the `flush`-position criterion already has (line 152, "A `flush` position carries the `memory` anchor ..."). `quire matrix` lists two FR-439-AC-7 rows, and `quire validate` does not catch it. A `#[trace(..., "FR-439-AC-7")]` tag, a TC row or a ticket citing FR-439-AC-7 can no longer say which criterion it means. Number the new row FR-439-AC-9. | spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md:152-154 |
| FND-008 | medium | The new criterion's fixture has no load-buffering shape, and the test case it names has no row for it. The AC asks for "an item over `SB` with a load-buffering shape". But TC-390's `SB` is store buffering: each branch stores, then loads (`Sx` then `Ly`, `Sy` then `Lx`). A load-buffering shape is a load followed by a store to another location, so `SB` yields no warning under any model, and the `ra` case cannot pass. The Verification column says TC-393, but TC-393 has no row for the warning or for routing to a descriptor without `memory_models`. Use the LB litmus (TC-391 LT-06, whose branches have QSL FR-226-AC-1's shape `r1 := x (relaxed); y := 1 (relaxed)`) for the warning cases and `SB` for the routing case, and add the rows to TC-393 or TC-394. | spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md:154 |
| FND-009 | low | "So it receives only items resolved to `sc`" is wider than FR-439's routing rule. FR-439 reads `memory_models` only when it routes an item over a subject with a weak `parallel`. A temporal descriptor without the member still receives temporal items whose subject has no `parallel`, and those have no resolved model. Write "so, of the items over a subject with a weak `parallel`, it receives only those resolved to `sc`". | spec/objects/interfaces/FR-331-backend-provider-envelope.md:63 |

## New findings (disposition pass 2)

Against head 11741c6 (11741c6f308edd83cb0099987ebd75cfc53779f9). The fix commit renumbers the warning criterion to FR-439-AC-9 on the LB litmus, keeps `SB` for routing, adds TC-393 rows MB-10 and MB-11, and rewords FR-331's routing sentence; the schema and tests are unchanged and the committed review files equal the reviewer's round-1 copies. `cargo test --test backend_provider_v1 --test checked_package_v2`: 11 and 108 pass. Mutations M1-M4 and M7-M10, each run and reverted, each fail the suite (exit 101). `quire validate --scope .` on FR-331, FR-439, TC-431 and TC-393 is identical to main after line-number normalisation (FR-331's two pre-existing errors; the others exit 0). `quire matrix`: FR-331-AC-30/31 tagged (:628, :681); FR-439-AC-1..9 each listed once, untagged as AC-1..8 are on main, since FR-439's implementation tests live in QSL; no reference to the old duplicate FR-439-AC-7 remains.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | medium | MB-10 expects one warning where the LB litmus has two shapes. TC-391 LT-06 is load buffering, "each load reads the other thread's later store": each of its two branches loads one location and then stores the other, so each branch is a distinct (load, store) shape, and FR-439's rule (one warning per distinct shape) gives two warnings under `ra`. MB-10's "one ... for the one shape" therefore fails a correct provider. The `acquire` variant is also ambiguous: "the load made `acquire`" names one load of two, and with only one load `acquire` the other branch's shape still warns. Expect two warnings, one per branch, each naming that branch's load and store; make both loads `acquire` in the no-warning variant (or add a one-load-acquire variant expecting one warning). | spec/test-cases/TC-393-memory-budgets-and-verdicts.md:41 |

## Dispositions

Round 1, against head 48428b7 (48428b7c037c3d8070b5ff99b320d1bac9015eb1).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | fixed | - |
| FND-006 | fixed | - |

Round 2, against head 11741c6 (11741c6f308edd83cb0099987ebd75cfc53779f9).

| FND | outcome | reason |
| --- | --- | --- |
| FND-007 | fixed | - |
| FND-008 | fixed | - |
| FND-009 | fixed | - |

Round 3, against head b2b158c (b2b158c5ba41bd8712f98c99be6d0c65bef833da). The fix commit changes only FR-439-AC-9, TC-393 MB-10 and the committed SR-13398 file, which equals the reviewer's round-2 copy. I re-derived MB-10 by hand from LT-06 (branch 1 `r1 := x; y := 1`, branch 2 `r2 := y; x := 1`, all `relaxed`). `x` and `y` are each written by one branch and read by the other, so both are shared. Each branch has a load followed by a store to the other location with no `acquire` between, so there are two distinct shapes and two warnings under `ra`. FR-439 adds none under `sc` and `tso`. With both loads `acquire` neither branch qualifies, so there is no warning. With one load `acquire` only the other branch qualifies, so there is one warning. The result and identities stay unchanged, as the warning rule states. FR-439-AC-9 now agrees. Gates: `cargo test --test backend_provider_v1 --test checked_package_v2` 11 and 108 pass; `quire validate --scope .` on FR-439 and TC-393 exits 0, identical to main; `quire matrix` lists FR-439-AC-1..9 once each (untagged, as on main) and FR-331-AC-30/31 tagged at :628/:681. No finding is open.

| FND | outcome | reason |
| --- | --- | --- |
| FND-010 | fixed | - |
