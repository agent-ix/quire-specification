---
id: SR-16195
title: 'Spec review of PR #388: induction-not-closed means the induction is not closed at k (STD-336, FR-363, FR-399)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@39c06f23; git diff origin/main(c323199f)...39c06f23 (PR #388, branch spec/std-336-induction-not-closed): FR-363 verdict table row and new reader paragraph, FR-363-AC-10, FR-399 verdict row, FR-399-AC-12, TC-294 step 9, TC-340 VS-07; context QSL #1282 head 6ec3a7455 ADR-018 V-6 and FR-182, QSpec FR-331, FR-468, FR-379; ticket STD-336'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-363
  type: reviews
- target: ix://agent-ix/quire-specification/FR-399
  type: reviews
- target: ix://agent-ix/quire-specification/TC-294
  type: reviews
- target: ix://agent-ix/quire-specification/TC-340
  type: reviews
---

## Summary

Ticket: STD-336. PR: quire-specification#388, head 39c06f23. The PR widens QSpec's `induction-not-closed{depth}` to mean that the induction is not closed at `k`: a step counterexample, or a base, step or search query the solver did not decide, including a solver resource stop that no request limit names. A stop under a named limit stays `incomplete`, `limit-reached`. This matches QSL #1282's ADR-018 V-6 text and FR-182's `Unknown{depth}` and solver-stop bullets word for word in substance. FR-399's row now defers to FR-363's definition. FR-182-AC-12's `Unknown{depth: 1, method: KInduction}` vector agrees with FR-399-AC-12. FR-331's cause vocabulary cites FR-363 for `induction-not-closed{depth}` and needs no edit. FR-379's and FR-468's step-counterexample uses stay true under the wider definition. The PR body flags both lead defaults (widen instead of a new cause; an unnamed resource stop counts as not closed). Ids: FR-363-AC-10, FR-399-AC-12, TC-294 step 9 and TC-340 VS-07 are free on main and in every open PR (#387, #365, #267, #207, #59). Gates on the head: `quire validate --scope . 'spec/**/*.md'` (quire 0.36.2) gives 86 failure lines, the same as the baseline; `cargo test --test checked_package_v2 --test backend_provider_v1` passes 119 and 23. No fixture, vector or schema changed, so no QSL conformance run is needed. origin/main c323199f (#382) is not an ancestor of the head, but it merges cleanly.

The new FR-363 paragraph that tells a reader to retry at a larger depth only after a step counterexample is the main defect. The record has no field that tells a step counterexample from an undecided query, so no reader can follow the rule and no test can check it. QSL #1282 deleted the equivalent sentence in round 4 (its FND-012) and ADR-018 V-6 now says the cause "means only that the induction is not closed at `k`". The PR body says the paragraph "matches #1282's FND-012", but the round-4 disposition removed it.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-363 verdict table "Inconclusive, other" row | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md:103 | examined |
| FR-363 reader retry paragraph | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md:191-195 | examined |
| FR-363-AC-10 | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md:210 | examined |
| FR-399 verdict row (induction not closed) | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:83 | examined |
| FR-399-AC-12 | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:205 | examined |
| TC-294 step 9 and expected result 9 | spec/test-cases/TC-294-verdict-strength-and-stopped-runs.md:41-45,67-69 | examined |
| TC-340 VS-07 | spec/test-cases/TC-340-hyper-vacuity-and-stutter.md:32 | examined |
| FR-363 verdict table "Stopped" and "Inductive at k" rows | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md:99,104 | context_only |
| FR-331 inconclusive and incomplete cause rows, hybrid budgets | spec/objects/interfaces/FR-331-backend-provider-envelope.md:264-265,421-435,481 | context_only |
| FR-468 step counterexample and other backend results | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:98,100 | context_only |
| FR-379 step obligation row | spec/functional/analysis/FR-379-report-and-replay-refinement-counterexamples.md:54 | context_only |
| TC-293 step 2 claim | spec/test-cases/TC-293-every-behaviour-verdicts-config-version.md:34-38 | context_only |
| QSL ADR-018 V-6 InductionNotClosed definition | agent-ix/quire-spec-language@6ec3a7455 spec/decisions/ADR-018-temporal-properties-over-every-behaviour.md | context_only |
| QSL FR-182 Unknown and solver-stop bullets, FR-182-AC-12 | agent-ix/quire-spec-language@6ec3a7455 spec/functional/FR-182-settle-a-hyper-verdict.md:227-235 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new FR-363 paragraph says a reader that retries at a larger depth SHALL do so only after a step counterexample, but an `induction-not-closed{depth}` record carries no field that tells a step counterexample from an undecided query, so no reader can follow it and no AC tests it. It also contradicts QSL #1282's ADR-018 V-6, which says the cause means only that the induction is not closed at `k` and which removed the same retry rule in its round 4 (FND-012). Fix: delete the paragraph. Keeping the rule would need a discriminator on the wire in QSL and QSpec together. | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md:191-195 |
| FND-002 | medium | "A solver resource stop for which no request limit is named" overlaps causes QSpec already assigns. FR-331 records a backend's own wall-clock limit and a provider's overrun of `deadline_ms` as `incomplete`, `timed-out`, and calls `limit-reached` the one limit-stop cause of every engine. FR-363's own Stopped row lists "time" as a stop. A k-induction solver that stops on its own time budget fits both `induction-not-closed` and `timed-out`, so two implementers can write different causes. Fix: say which stops are `timed-out` (backend wall-clock, `deadline_ms`) and which settle `induction-not-closed`, and add the time-stop case to FR-363-AC-10. | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md:103; spec/objects/interfaces/FR-331-backend-provider-envelope.md:265,430 |
| FND-003 | medium | TC-294 step 9 runs k-induction over the step 2 claim, `always eventually holds(c.versionNumber = 2)` with `fair weak whole`. That is a fair liveness claim, which TC-293 step 2 settles `refuted`. FR-363's `inductive` basis and step case are defined for safety ("any `k` consecutive states ... have only satisfying successors"), so a k-induction run of this claim, and its step counterexample, is not a defined outcome. Fix: use a safety claim over the same subject, for example a bounded claim activated `on each`, which FR-363 calls an unbounded safety property. | spec/test-cases/TC-294-verdict-strength-and-stopped-runs.md:41-45 |
| FND-004 | low | FR-468 maps a solver `unknown` on a per-step refinement query to `solver-inconclusive`, while FR-363 now maps a solver `unknown` on a k-induction query to `induction-not-closed`. Both are induction queries, and FR-331's one cause vocabulary now gives the same backend event two causes. The PR body says this is deliberate, but no spec text says why `induction-not-closed`'s wider meaning stops at FR-468. Fix: one sentence in FR-363 or FR-468 stating that per-step refinement queries keep `solver-inconclusive`, or align FR-468 with the wider definition. | spec/functional/analysis/FR-468-certify-per-step-refinement-obligations.md:100 |
| FND-005 | low | FR-363's definition covers an undecided base, step or search query, but FR-363-AC-10 tests only the base and step queries. The search-query (lasso search) case has no vector. Fix: add an `unknown` on the search query at depth 2 to FR-363-AC-10 and TC-294 step 9. | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md:210 |
| FND-006 | low | FR-399-AC-12 has no named-limit case. FR-363-AC-10 checks that a stop under a named request limit settles `incomplete`, `limit-reached`, never `induction-not-closed`, but the HP-2 AC does not, although FR-399's own table settles a limit stop `failed` with execution `resource-incomplete`. Fix: add the named-limit stop to FR-399-AC-12 and TC-340 VS-07. | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:205 |

## Verdict

Changes requested. The verdict-table rows in FR-363 and FR-399 match QSL #1282's ADR-018 V-6 and FR-182, the ids are unique, the lead defaults are flagged and both gates pass. FND-001 must be fixed before merge: the retry paragraph adds a rule no reader can follow, and it reintroduces the sentence QSL deleted, so the two specs would disagree again. FND-002 and FND-003 should be fixed in the same round. FND-004 to FND-006 are small additions. The head also needs origin/main c323199f merged in, which merges cleanly.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | FR-399's verdict table has no row for a stop under the backend's own wall-clock limit or an overrun of `deadline_ms`. FR-399-AC-12 and TC-340 VS-07 (4374ec62) now test that this stop settles `incomplete`, `timed-out`, and FR-363 "Undecided k-induction runs" item 2 and QSL #1288's V-7 `Incomplete(TimedOut)` agree. But FR-399's two stop rows give only `limit-reached` ("a run limit, meter or time budget") and `cancelled{source}`, so the AC tests an outcome the table does not state. Fix: add `timed-out` to FR-399's stop rows, with label `failed` and execution `resource-incomplete`. | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:84-85 |
| FND-008 | low | TC-294 step 9's safety claim `on each always[0,1] holds(c.versionNumber >= 0)` holds for every state of the declared type `Int[0, 1000]`. Its step case therefore always closes, and the first vector (a step counterexample to induction) cannot come from a real backend, only from an injected outcome. Fix: use a claim that is true on reachable states but not inductive over the declared type, for example `on each always[0,1] holds(c.versionNumber <= 2)` together with a step case whose counterexample starts at an unreachable `versionNumber` of 3 or more, or say that the vectors inject the backend outcome. | spec/test-cases/TC-294-verdict-strength-and-stopped-runs.md:41-48 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (fix diff 39c06f23..4374ec62): the retry paragraph is gone. FR-363 now says `induction-not-closed` "means only that the induction is not closed at `k`; it does not say whether the step case had a counterexample or the solver did not decide", which matches ADR-018 V-6 in QSL #1282 and #1288. |
| FND-002 | fixed | Round 1: the new "Undecided k-induction runs" section orders the stops: a tripped cancellation handle, the caller's own deadline included, gives `cancelled{source}` with `source` `requested` or `deadline`; then the backend's own wall-clock limit or an overrun of the request's `deadline_ms` gives `timed-out`; then a named request limit (time budget included) gives `limit-reached`; and only the solver's own `unknown` (an unnamed internal stop included) gives `induction-not-closed`. This is the same order and the same causes as QSL #1288 (09a19b63) ADR-018 V-6 and FR-182 stop list 1-4. FR-363-AC-10 and TC-294 step 9 test the time-budget, `deadline_ms`, requested-cancellation and caller-deadline cases, and `unknown` together with the caller's deadline, where rule 1 wins. |
| FND-003 | fixed | Round 1: TC-294 step 9 now uses the safety claim `on each always[0,1] holds(c.versionNumber >= 0)`, a bounded `on each` claim. A remaining low issue with this vehicle is recorded as FND-008. |
| FND-004 | fixed | Round 1: FR-363 states that a per-step refinement query (FR-468) is not a k-induction run, so a solver `unknown` on it keeps `solver-inconclusive`, and only its step counterexample settles `induction-not-closed{depth: 1}`. This agrees with FR-468:98,100. |
| FND-005 | fixed | Round 1: FR-363-AC-10 and TC-294 step 9 add the `unknown` on the search query at depth 2. |
| FND-006 | fixed | Round 1: FR-399-AC-12 and TC-340 VS-07 add a stop under a named request limit (`failed`, `resource-incomplete`, `limit-reached`), the caller's own deadline (`cancelled{source: deadline}`) and a `deadline_ms` overrun (`timed-out`), each never `induction-not-closed`, in FR-363's order. |
