---
id: SR-14743
title: "spec-integrity analysis of STD-188 (QSpec PR #350)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@a8be66647bfb9a32fd2b546d467a1cbe54bc121c; spec/functional/analysis/FR-405-declare-probabilistic-models.md, spec/functional/analysis/FR-407-state-probabilistic-claims.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, proposals/quire-v1/definitions/native-diagnostics.md, proposals/quire-v1/shared-grammar.md, spec/test-cases/TC-350-probabilistic-declarations.md, spec/test-cases/TC-351-step-probability-and-markov-condition.md, spec/test-cases/TC-353-probabilistic-claim-forms-and-routing.md, spec/test-cases/TC-356-regenerative-long-run.md, spec/test-cases/TC-362-exact-long-run.md, spec/test-cases/TC-364-probabilistic-counterexample-replay.md, spec/test-cases/TC-365-exact-limits-and-partial-evidence.md; context agent-ix/quire-spec-language@0f34179e2 spec/functional/FR-185, FR-186, FR-189"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-407
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-405
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-272
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-353
    type: reviews
---
# SR-14743: spec-integrity analysis of STD-188 (QSpec PR #350)

## Summary

Ticket: STD-188. Scope is `git diff origin/main...a8be666`.

Checked against the shared grammar's probabilistic productions:

- `Monthly`'s event threshold is now `rational(999, 1000)`, which is
  correct. A measure comparison's `bound-op` takes an `expr`, and
  `decimal-literal` is an `exact-value` only.
- Every other decimal left in TC-351..365 is in an `exact-value` position
  (probability, quantile level, `mean`, `long_run_fraction`, confidence
  items) or is a quantity magnitude. A search found no bare decimal after a
  measure's, quantile's or `expected` form's `bound-op`.
- `under ident` takes no `fairness-item`, so PF-16 and FR-407-AC-15 match the
  grammar.
- PD-06's weight `0` matches QSL FR-185, which says a bare `-1` is no
  `exact-value`.

The nine new causes are the same set, spelled the same way, in FR-272 and
native-diagnostics, and they match QSL FR-185/FR-186's QSL-proposed names.
The ticket's comment about a `rational bits` stage limit is already met on
main (`stage_limit_exceeded`/`rational-bits-exceeded`, `s3.rational_bits`).
The merged QSL changes STD-252 (FR-189 `quantile` field, FR-185-AC-9
reference support values) and STD-250 (FR-193 replay subject) touch nothing
this PR states. No other open QSpec PR edits FR-405, FR-407, TC-350, TC-353
or the `invalid_probabilistic_model` row, and the new ids (FR-405-AC-9,
FR-407-AC-10..15, PD-11, PF-11..16) do not collide with them.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `fairness-under-workload` is kept, but nothing can produce it, so FR-407-AC-6's refusal clause and TC-353 PF-02's second vector cannot be built. FR-407 now refuses `under W fair ...` at S1. It keeps the cause for "a claim that reaches the frontend by any other route", but no spec defines such a route. QSL FR-186 also says `SchedulerForm::Workload` cannot hold a fairness set and that S3 has no producer of one. STD-188 asked QSpec to settle this. Either remove the cause from FR-272, native-diagnostics, FR-407 and AC-6 and drop PF-02's second vector, or name a concrete input that carries a workload and a fairness set. | spec/functional/analysis/FR-407-state-probabilistic-claims.md:72-74; spec/functional/analysis/FR-407-state-probabilistic-claims.md:200; spec/test-cases/TC-353-probabilistic-claim-forms-and-routing.md:39 |
| FND-002 | high | The new `unbounded-event` rule refuses the timed-profile events that FR-407, FR-416 and FR-421 admit. It refuses any formula event "whose profile is not a bounded profile", and the only exception is the infinite-trace reachability forms. FR-407's own event list treats "under the timed profile, a bounded-window formula" as a different kind from "a formula under a bounded profile", and FR-090-AC-7 gives timed the unbounded grammar edition. So TC-374's `Deadline`, `RareLate` and `Late` (`eventually[0 ms, 4 ms] holds(...)` and similar) would refuse `unbounded-event`. A strict timed interval would also refuse at the frontend ("closed interval"), but FR-421's closed condition settles it `unsupported`/`strict-clock-constraint`. Exempt the timed bounded-window formula, and keep the strict-interval outcome with FR-421. QSL FR-186 has the same wording. | spec/functional/analysis/FR-407-state-probabilistic-claims.md:86-89; spec/functional/analysis/FR-407-state-probabilistic-claims.md:93-96; spec/functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md:68-72; spec/test-cases/TC-374-stochastic-and-probabilistic-timed.md:33-34 |
| FND-003 | high | FR-407-AC-11 and TC-353 PF-12 expect `unbounded-event` for `always holds(n.healthy)` under the event-position false-extension profile. FR-048-AC-9 (TC-238) gives that same spelling, under any profile other than infinite-trace, `unsupported_construct` at its construct span. FR-090-AC-7 also says the false-extension profile admits no unbounded operator. The two requirements give different outcomes for one input. Either state which refusal wins, or move the vector to a form that FR-048 admits under that profile. | spec/functional/analysis/FR-407-state-probabilistic-claims.md:205; spec/test-cases/TC-353-probabilistic-claim-forms-and-routing.md:49; spec/functional/foundation/FR-048-bind-native-temporal-syntax.md:89 |

## Verdict

Not mergeable. Three major findings remain. FND-001 is an AC that cannot be
built and is left by lead default 1. FND-002 and FND-003 are new
`unbounded-event` text that contradicts FR-421/TC-374 and FR-048-AC-9. The
rest checks out: the fixture rewrites, the cause set, the duplicate-reward,
trace-subject, profile, `over`-type, fraction-bound, quantile-over-fraction
and confidence rules, and the grammar note.
`quire validate --scope . spec/**/*.md` reports 134, the baseline, and the
touched files have the same diagnostics as on main.
