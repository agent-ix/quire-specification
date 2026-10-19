---
id: SR-13327
title: "Scope-boundary review of QSpec PR #288 (STD-208)"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-specification@07eb2ff; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md, spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md, spec/functional/tooling/FR-306-compile-and-cache-exactly.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/test-cases/TC-373-hybrid-verdict-map.md; ticket STD-208"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-193
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-306
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-373
    type: reviews
---

## Summary

Ticket: STD-208. PR: quire-specification#288 at 07eb2ff. Allocation of the two budgets between the request envelope (FR-331), the limit-naming rule (FR-461) and the QSL consumer (QSL FR-250, merged #1037; QSL-586 code, merged #1031). Wire spelling: proposals/backend-provider-v1/schema.json requires `LimitName` `^[a-z][a-z0-9_]*$` and a dotted `SettingName` `^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$`. The PR's `limit: max_solver_steps, setting: hybrid.max_solver_steps` satisfies both; QSL FR-250's `limit: solver-steps, setting: max_solver_steps` fails both (hyphen in the limit, no dot in the setting), and so does the merged QSL code (qsl-replay/src/hybrid.rs:332-334, model_check/limits.rs `SolverSteps`). The PR's spelling is the right one; `hybrid.` follows FR-461 rule 5 (`<engine>.<member>`), though QSpec names no `hybrid` engine yet.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-193-row-step | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-row-time | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-steps | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-time | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-budgets-identity | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-AC-8 | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-193-AC-9 | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md | examined |
| FR-331-incomplete-row | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-hybrid-solver-budgets | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-request-identity | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-331-AC-29 | spec/objects/interfaces/FR-331-backend-provider-envelope.md | examined |
| FR-306-outside-key | spec/functional/tooling/FR-306-compile-and-cache-exactly.md | examined |
| TC-373-HY-04 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |
| TC-373-HY-07 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |
| TC-373-HY-08 | spec/test-cases/TC-373-hybrid-verdict-map.md | examined |
| FR-461-rule-5 | spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md | context_only |
| FR-331-options | spec/objects/interfaces/FR-331-backend-provider-envelope.md | context_only |
| FR-331-descriptor-defaults | spec/objects/interfaces/FR-331-backend-provider-envelope.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Putting both budgets in the item's `options` is inconsistent with reporting the step stop as a limit. `limit-reached` names `setting: hybrid.max_solver_steps`, and FR-461 rules 3-5 require that setting to raise the limit through the library setter, the replay request and `--limit`, with its default taken from the descriptor's `bounds` keyed by limit name. An `options` entry has none of that: its default is the descriptor's `options` and nothing maps `hybrid.max_solver_steps` onto `options.max_solver_steps`. FR-193 even cites both FR-331 and FR-461 for the default. The premise "FR-331 has no hybrid limits object" is true, but the consistent fix is to add one: a per-item `hybrid` member with `limits.max_solver_steps`, like `temporal.limits` and `probabilistic.limits`, with the default in `bounds`. `max_solver_time` can stay an option or sit beside it outside identity. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:58,192,206-208,212-222; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:75,81-86; spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md:69-80 |
| FND-002 | low | Who owns the defaults is split across repos. QSpec gives both budgets "the provider's published default" with no numbers, while QSL FR-250 fixes 60 s and 1,000,000 steps and its AC-3/TC-705 say "a request that sets none uses 60". If the QSL request writer always fills the value, the provider's published default never applies; if a provider publishes another default, QSL's AC-3 disagrees with it. State in FR-193 which side supplies the value when the caller sets none. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:81-83,91-92 |

## Verdict

Wire spelling: the PR's `limit: max_solver_steps, setting: hybrid.max_solver_steps` is right against the schema, and QSL FR-250 (behavior bullet, verdict-map row, AC-2, AC-3), TC-705 step 3 and the merged QSL code must change to it. FND-001 (budgets in `options`) is the main boundary issue: the schema-conformant setting name points at a limit that the `options` placement does not create.
