---
id: SR-13659
title: 'Spec review of QSpec PR #317: FR-364 observed and model arm coupling (STD-147)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@ae1ebe4d4; PR #317 diff origin/main...HEAD: FR-364, TC-431, backend-provider-v1/schema.json, backend-provider-v1/vectors.json; ticket STD-147'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-364
  type: reviews
- target: ix://agent-ix/quire-specification/TC-431
  type: reviews
---

## Summary

Ticket: STD-147. PR quire-specification#317, head ae1ebe4d4bebbb8a79e0e511d511391587a91ca1.

Base lens, with EARS, integrity and failure-domain folded in. Gates: `cargo test --test checked_package_v2 --test backend_provider_v1` passes (13 + 114). `quire validate --scope . 'spec/**/*.md'` (quire 0.36.2, engine 0.50.2) reports 134 failures, the same as main, and none in FR-364 or TC-431. FR-364-AC-10 and TC-431 BP-23 collide with nothing on main or in the open PRs (#267, #207, #200, #59; #200 touches TC-431 but adds BP-11 and BP-12 only).

The claim that deadlock and time-lock are model-only holds. FR-364 on main marks deadlock "model arm only" (Shapes and the `kind` row). FR-366 derives the deadlock-freedom item per model subject, and FR-417 derives the time-lock-freedom item per timed subject. QSL FR-331 (qsl-lane2 origin/main 15e011cca) reads the observed arm as `kind` formula or the undefined-evaluation envelope, `initial_state` and `over_binding` null and `{document}` steps, which matches the new coupling. QSL has no wire decoder for the `trace` member: the string `"observed"` does not occur in any QSL `src/`. So the PR makes no QSL behaviour claim that the code could contradict. The observed arm itself (document steps, replay rules, AC-9) is on main from #170, as the author says.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-364 arm coupling paragraph | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md | examined |
| FR-364-AC-10 | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md | examined |
| FR-364 wire-form table | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md | examined |
| FR-364 where-key table | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md | examined |
| FR-364-AC-2 | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md | context_only |
| FR-364-AC-9 | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md | context_only |
| TC-431 BP-23 | spec/test-cases/TC-431-backend-provider-wire-vectors.md | examined |
| TC-431 relationships | spec/test-cases/TC-431-backend-provider-wire-vectors.md | examined |
| TemporalCounterexample arm conditional | proposals/backend-provider-v1/schema.json | examined |
| observed-counterexample-outcome | proposals/backend-provider-v1/vectors.json | examined |
| FR-331 counterexamples bullet | spec/objects/interfaces/FR-331-backend-provider-envelope.md | context_only |
| FR-366 derived deadlock-freedom item | spec/functional/temporal/FR-366-check-deadlock-freedom.md | context_only |
| FR-417 time-lock counterexample | spec/functional/temporal/FR-417-settle-time-lock-freedom.md | context_only |
| FR-369 strong-fairness warning | spec/functional/temporal/FR-369-name-excluding-strong-fairness.md | context_only |
| FR-362 enabledness from a model only | spec/functional/temporal/FR-362-apply-fairness-constraints.md | context_only |
| FR-363 refuting evidence | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md | context_only |
| QSL FR-331 observed-trace replay | quire-spec-language spec/functional/FR-331-replay-a-temporal-counterexample-over-an-observed-trace.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The coupling is one-sided, and "a counterexample that mixes them" has no definition. The wire-form table (line 72) says the model arm carries the zero-based index of an initial state and the observed arm `null`, but the new paragraph constrains only the model arm's steps. The schema's `else` branch constrains only `prefix` and `loop`. Verified: `temporal-outcome` with `initial_state: null` is admitted. A model counterexample with an observed-arm `initial_state` is a mix by the table, yet nothing refuses it. Fix: add "a counterexample of the model arm SHALL carry a non-null `initial_state`" (`over_binding` may stay nullable for `kind: deadlock`), add `initial_state: Count` to the `else` branch with an adverse vector, and replace "mixes them" with the list of conditions that refuse. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:80-85; proposals/backend-provider-v1/schema.json:502-505 |
| FND-002 | medium | The undefined-evaluation envelope's `where` keys are arm-specific (`state_key` model arm, line 104; `document` observed arm, line 105), but the coupling does not reach them. Verified: an observed counterexample whose `kind` is the envelope with `where: {position, state_key, locus}` is admitted. That is a model-arm key on an observed counterexample, which the paragraph says refuses before replay, and replay step 6 cannot read the state it names. Fix: in the observed branch, constrain `kind`'s envelope `where` to carry no `state_key`/`state_keys`, and the model branch to carry no `document`. State the rule in the paragraph, and add an adverse vector for it. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:80-85; spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:104-105 |
| FND-003 | medium | The positive vector cannot be produced. Its `accounting` record was copied from `temporal-outcome`: `method: exploration`, `witness_samples`, `limits.max_states`/`max_walk_steps`, and an FR-369 `fairness.strong-would-exclude` warning naming `Example::Session::close` at position 1. FR-369 emits that warning only for a reproduced lasso, from the enabledness that replay recomputes from the model. FR-362 and FR-364 step 4 say an observed trace has no enabledness. This counterexample is a finite prefix (`loop: []`) with empty `fairness`, so no conforming producer emits this record. Fix: give the observed vector an accounting record with no FR-369 warning and no model-exploration members, for example `{request_index: 0, settled_by: result}`. | proposals/backend-provider-v1/vectors.json:2212-2236 |
| FND-004 | low | The new paragraph excludes `time-lock` from the observed arm. But the wire-form `kind` row (line 71), which the table calls "the one list of its members, arms and kinds", does not list `time-lock`. Read literally, AC-2 then refuses `time-lock` on every arm. The omission is older than this PR (FR-417 never added the kind to the row), but this PR is the first to name `time-lock` in FR-364. Fix: add `"time-lock"` (model arm only, FR-417) to the `kind` row, and `final_delay` and `refinement_failure` to the member list, or cite FR-417 in the paragraph instead of naming a kind the table omits. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:71; spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:83 |
| FND-005 | low | EARS: the new SHALL has a datum ("A counterexample of the observed arm SHALL carry ...") as its subject, not the system, and "refuses before replay" names no refusal, where every other refusal in FR-364 names its code. An implementer cannot tell which located refusal to produce. Fix: "The replayer SHALL refuse before replay, as an FR-462 wire failure (or `invalid_runtime_input`/`invalid-value`), a counterexample whose ...". | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:80-85 |
| FND-006 | low | FR-331 still says "A temporal refutation over a model subject is an FR-364 `TemporalCounterexample`". It leaves out the supplied-trace arm, which FR-364 line 117 says travels in the same `counterexamples` entry, and which this PR now carries in an FR-331 outcome vector owned by FR-364. Fix: "A temporal refutation over a model subject or a supplied trace is an FR-364 `TemporalCounterexample`". | spec/objects/interfaces/FR-331-backend-provider-envelope.md:70 |

## Verdict

Not mergeable as is. FND-001 and FND-002 are gaps in the coupling the PR sets out to state: each admits a counterexample that the wire table assigns to the other arm. FND-003 makes the new positive vector something a conforming backend could produce. All three are small schema or vector edits inside this PR. The rest is sound: the observed-branch rules for `initial_state`, `over_binding`, `kind` and `prefix` and the model-branch rule for `prefix` each match FR-364's table, keeping a non-empty observed `fairness` as a replay outcome (`unsupported`) agrees with AC-9 and FR-362, and TC-431 now correctly verifies FR-364.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The new `refinement_failure` row links `[FR-379](FR-379-report-and-replay-refinement-counterexamples.md)` relative to `spec/functional/temporal/`, but FR-379 lives in `spec/functional/analysis/`, so the link is broken. Fix: `../analysis/FR-379-report-and-replay-refinement-counterexamples.md`. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:79 |
| FND-008 | low | FR-364-AC-2 still says a counterexample carries "exactly the members `trace`, `kind`, `initial_state`, `prefix`, `loop`, `over_binding`, `fairness`, `interval` and `trace_position`" and that an extra member refuses. The table now also lists `final_delay` and `refinement_failure`, so by AC-2 a time-lock counterexample carrying its required `final_delay` refuses. Fix: name the two conditional members in AC-2 (present only for `kind: time-lock` and for a refinement item respectively). | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:203 |
| FND-009 | low | The new SHALL (and the schema's model branch) refuses a model-arm envelope whose `where` carries `documents`, but `documents` is not a `where` key. FR-364 names only `positions` and `state_keys` as plural forms, and the schema's `Where` is closed (`additionalProperties: false`) with no `documents` member. So the prose names a key that cannot occur, and the schema clause can never fire. Fix: drop `documents` from the SHALL and the schema, or define a `documents` plural key in the `where` table and in `Where` if an observed engine needs one. | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:82-91 |
| FND-010 | low | The `final_delay` row calls it "the delay of the time-locked sample". FR-417 and FR-331 define it as "the exact rational delay that reaches the time-locked state". "Sample" is the word of FR-331's unrelated stochastic cause `time-locked-sample{state}`. Fix: "the exact rational delay that reaches the time-locked state (FR-417)". | spec/functional/temporal/FR-364-replay-temporal-counterexamples.md:78 |

## Dispositions

Round 1, head 0f1cba012.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The model branch now requires `initial_state` to be a Count. The SHALL lists every refused condition for both arms and defines a mix as failing any of them. `model-counterexample-without-an-initial-state` is refused, and admitted once `else.initial_state` is removed (mutation verified). |
| FND-002 | fixed | Both branches now constrain the envelope's `where` keys, and the SHALL states the rule. `observed-undefined-evaluation-with-a-state-key` and `model-undefined-evaluation-with-a-document` each fail only when their own clause is removed (verified). Residuals are FND-009 here and FND-005 in SR-13660. |
| FND-003 | fixed | The observed accounting record is now `{request_index: 0, settled_by: result}`, with no FR-369 warning and no exploration members. That is consistent with FR-369 (warnings only for a reproduced lasso, from model enabledness) and FR-362 (no enabledness on an observed trace). The schema admits it. |
| FND-004 | fixed | The `kind` row now lists `time-lock` (FR-417, model arm only), and the table adds `final_delay` and `refinement_failure`. New defects in those rows are FND-007, FND-008 and FND-010. |
| FND-005 | fixed | The replayer is now the subject, and the refusal is `malformed_wire`. FR-271 defines that as a wire whose closed structure the reader's schema refuses. |
| FND-006 | fixed | FR-331:70 now reads "A temporal refutation over a model subject or a supplied trace is an FR-364 `TemporalCounterexample`". |
