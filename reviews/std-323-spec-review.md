---
id: SR-16205
title: "spec-review of execution-budget setting names and max_witness_paths 0 (quire-specification PR 390)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@9c805c9f; spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md; spec/functional/analysis/FR-411-settle-probabilistic-claims-exactly.md; spec/test-cases/TC-426-limit-outcomes-name-their-setting.md; spec/test-cases/TC-365-exact-limits-and-partial-evidence.md; proposals/backend-provider-v1/schema.json; proposals/backend-provider-v1/vectors.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-461
    type: references
  - target: ix://agent-ix/quire-specification/FR-411
    type: references
  - target: ix://agent-ix/quire-specification/TC-426
    type: references
  - target: ix://agent-ix/quire-specification/TC-365
    type: references
  - target: ix://agent-ix/quire-specification/FR-413
    type: references
  - target: ix://agent-ix/quire-specification/FR-323
    type: references
---

# Spec review of execution-budget setting names and max_witness_paths 0 (quire-specification PR 390)

## Summary

Ticket: STD-323 (from QSL PR #1244, STD-271, FR-255). One spec-review pass
over `git diff origin/main...9c805c9f`. Oracles: QSpec FR-461 Behaviors 1 to
7, FR-323 (`limits`, `stage_limits`), FR-413 (path sets),
`proposals/quire-v1/definitions/value-accounting.md` (`ScalarLimitsV1`,
`PopulationAdmissionLimitsV1`, `ModelNormalizationLimitsV1`), the provider
schema's `LimitName`, `SettingName`, `LimitReached`, `Count` and
`PositiveCount`, and QSL main FR-255 ("Accounting setting names", the stage
table and "Engine limit setting names").

Gates, run at the head: `quire validate --scope . 'spec/**/*.md'` (quire
0.36.2, engine 0.50.2) gives 86 failure lines, the same as main.
`cargo test --test checked_package_v2 --test backend_provider_v1` passes
129 + 23 with 0 failed, so the provider tests admit the new adverse vector.
QSL conformance: `cargo test -p qsl-package --lib checked_v2` on QSL main
5ba013c75 with `QSPEC_DIR` at this head passes 46 with 0 failed. No QSL crate
reads `backend-provider-v1`. origin/main is an ancestor of the head.

Examined, clean:

- `SettingName` consistency. `accounting.work_units` matches
  `^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$`, and every
  `quire.value.accounting/v1` counter name matches the `LimitName` pattern,
  so the pattern needs no change. FR-461 B2, B5 and B6 now all give dotted
  names, and the new B6 sentence on `LimitReached` agrees with the schema.
- Schema compatibility with QSL FR-255. Changing
  `ExactProbLimits.max_witness_paths` from `PositiveCount` to `Count` only
  widens the schema, so every document valid before is still valid. QSL
  FR-255 already says `exact_prob.max_witness_paths` gives 0 a meaning and
  that every other engine row is a positive count. The new FR-411 sentence
  agrees with it, and with FR-413, which emits a path set only when at most
  `max_witness_paths` paths reach the bound.
- The FR-461 example `zone_check.max_symbolic_states` is a QSL FR-255 row
  (FR-239).
- Testable ACs. FR-411-AC-16 and TC-365 XL-06 cover 0, the default and -1.
  FR-461-AC-8 has a negative half that the new vector checks.
- Ids. FR-461-AC-8, FR-411-AC-16, LS-08, XL-06 and the vector name are not
  on main and are not added by any other open QSpec PR (#389, #267, #207,
  #59).
- No status or changelog wording. The three lead defaults are labelled in
  the PR body.
- The PR body says no QSL conformance run is needed. The run above passes,
  so that claim holds at this head.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | [major] `accounting.<counter>` gives two independent limits one setting name. FR-461 B6 applies to "its `quire.value.accounting/v1` counter", and `quire.value.accounting/v1` defines `work_units` in `ScalarLimitsV1`, `PopulationAdmissionLimitsV1` and `ModelNormalizationLimitsV1` ("These counters are independent"), and `ancestor_steps` in the last two. FR-323 `limits` carries `ScalarLimitsV1` and `PopulationAdmissionLimitsV1` on one request. So an execution outcome stopped at either `work_units` names `accounting.work_units`, and raising it cannot say which limit it raises. That breaks B2's "one setting name per limit". QSL FR-255 already names the other two objects' counters by stage (`admission.work_units`, `model.work_units`), so the QSL follow-up would also conflict. Fix: limit B6 to the ten `ScalarLimitsV1` counters of the evaluation meter, and say that the counters of the other accounting limit objects take their stage's dotted name under B2 (as QSL FR-255 does). Alternatively, prefix each object's counters with its own name. | spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md:84-90; proposals/quire-v1/definitions/value-accounting.md:9-12, :467-470, :530 | wrong-requirement |
| FND-002 | medium | [major] The new FR-411 sentence "Each is a positive count except `max_witness_paths`" makes TC-365 XL-01 an invalid request. XL-01 sets `max_policy_iterations` 0, which the schema types `PositiveCount` and which QSL FR-255 also requires to be positive. Before this PR, the conflict was between the vector and the schema. After it, the vector also contradicts the FR that this PR edits, and the PR body leaves it for triage. Fix it in this PR: set XL-01's `max_policy_iterations` to a positive value (the vector is about the precision budget, so 1 keeps its intent). Or, if 0 is meant to mean "no policy iteration", widen it as `max_witness_paths` is widened and say so in FR-411. | spec/test-cases/TC-365-exact-limits-and-partial-evidence.md:30; spec/functional/analysis/FR-411-settle-probabilistic-claims-exactly.md:247-250; proposals/backend-provider-v1/schema.json:376 | wrong-requirement |
| FND-003 | low | [minor] The new adverse vector `limit-reached-bare-accounting-counter` does not test what LS-08 and FR-461-AC-8 describe. It patches only `setting` on the Kani `value-validity` base, so `limit` stays `max_objects`. It is therefore a Kani engine stop with a second undotted setting, and it fails for the same reason as the existing `limit-reached-undotted-setting`. AC-8's positive half (`limit` `work_units` with `setting` `accounting.work_units` validates) has no vector. Fix: patch `limit` to `work_units` as well, and add a positive case (a base result or a passing patch) carrying `accounting.work_units`, so the AC can fail on both branches. | proposals/backend-provider-v1/vectors.json:4563-4574; spec/test-cases/TC-426-limit-outcomes-name-their-setting.md:33; spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md:108 | correct-requirement-no-evidence |

## Verdict

Not mergeable yet. Dotting the execution-budget setting names keeps the
`SettingName` pattern as it is. Widening `max_witness_paths` to `Count` is
backward compatible and matches QSL FR-255. The QSL conformance run passes.
FND-001 must be fixed in this PR, because the new name collides across
accounting limit objects. FND-002 is the XL-01 leftover, which this PR now
turns into a direct contradiction, and it is a one-line fix in the same
round. FND-003 makes AC-8 testable on both branches. All three choices are
lead defaults, not owner answers, and the PR body says so.
