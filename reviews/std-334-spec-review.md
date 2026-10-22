---
id: SR-16181
title: "Spec review of quire-specification PR #380: STD-334, FR-361 order of no-initial-state and no-instance"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@08c559ca3; PR #380 (git diff origin/main...08c559ca3): spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md, spec/test-cases/TC-294-verdict-strength-and-stopped-runs.md"
review_set: subset
evaluated_revision: "08c559ca3"
relationships:
  - target: ix://agent-ix/quire-specification/FR-361
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-294
    type: reviews
---
# SR-16181: Spec review of quire-specification PR #380: STD-334, FR-361 order of no-initial-state and no-instance

## Summary

Ticket: STD-334. Reviewed `git diff origin/main...HEAD` at 08c559ca3 (FR-361 Outputs and Model subject paragraph, FR-361-AC-16, TC-294 step 8) against FR-361's existing requires-bound, no-instance and conflicting-identity text (AC-7, AC-8, AC-14), TC-340 VS-01 (f) and QSL FR-182 (V-6 row, line 135: `NoInitialState` first). The order requires-bound, no-initial-state, conflicting-identity, no-instance agrees with FR-361's "runs first" and with QSL FR-182; an empty list holds no member record, so the conflicting-identity check and no-initial-state cannot both apply, as the lead default says. AC-16's four subjects can each fail (a `no-instance` for any of the first three, a `no-initial-state` for the fourth). FR-361-AC-16 is absent from main and from every open QSpec PR head. Gates at the head: `quire validate --scope . 'spec/**/*.md'` 86 failure lines (main 86); `cargo test --test checked_package_v2 --test backend_provider_v1` 23 + 119 passed. No test-parsed file changes, so no QSL conformance run applies. origin/main is an ancestor. Merges with the other six PRs under review with no textual conflict. Lead defaults are flagged in the PR body.

Method: quoin spec-review (base), with integrity, EARS, criterion-strength (by hand: no Jev client) and failure-domain checks folded in; code-review of the `tests/` change where present is folded in here. Gap-analysis does not apply (spec, vectors and trace tags only).

Units examined:

- `FR-361-AC-16` (examined): spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md
- `FR-361 Model subject (order paragraph)` (examined): spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md
- `FR-361 Outputs` (examined): spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md
- `TC-294` (examined): spec/test-cases/TC-294-verdict-strength-and-stopped-runs.md
- `FR-361-AC-7` (context_only): spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md
- `FR-361-AC-8` (context_only): spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md

## Verdict

**MERGEABLE**, with one low finding to fix or accept.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Two existing sentences still define zero instances to include 'no member record of an initial state admits a key' and settle it `no-instance` with a SHALL; an empty list of initial states meets that wording, so it now conflicts with the new paragraph's SHALL `no-initial-state` until the reader applies the order paragraph. The rationale paragraph at line 202 also still says FR-361-AC-8 settles that case `no-instance`. Qualify both with 'of a non-empty list of initial states', as the Outputs sentence now does. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:67-78, spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:202 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | see after_excerpt |

Disposition pass 1 verdict (head ae946d0a): **MERGEABLE.** FND-001 fixed in ae946d0a; both sentences now say 'of a non-empty list of initial states'. Merges cleanly onto current main 6f24ddd4.
