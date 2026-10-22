---
id: SR-13618
title: 'Spec review of QSpec PR #304: hyperproperty and probabilistic follow-ups, integrity'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@90d6c3f; git diff origin/main...90d6c3f (PR #304, branch qspec-hyper-prob-followups): spec/functional/analysis/FR-399-settle-hyperproperty-results.md, spec/objects/interfaces/FR-400-hyper-counterexample.md, spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, proposals/quire-v1/definitions/native-diagnostics.md, spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md, spec/functional/foundation/FR-460-bound-every-stage-by-resource-limits.md; tickets QSL-553, QSL-554, QSL-555, QSL-573, QSL-883'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-399
  type: reviews
- target: ix://agent-ix/quire-specification/FR-400
  type: reviews
- target: ix://agent-ix/quire-specification/FR-403
  type: reviews
- target: ix://agent-ix/quire-specification/FR-272
  type: reviews
- target: ix://agent-ix/quire-specification/FR-461
  type: reviews
- target: ix://agent-ix/quire-specification/FR-460
  type: reviews
---

## Summary

Tickets: QSL-553, QSL-554, QSL-555, QSL-573, QSL-883. PR: quire-specification#304 (private), head 90d6c3fd3c85644fd43799464ed7400b9ea83a8a, branch qspec-hyper-prob-followups, scoped to `git diff origin/main...HEAD` (one commit, 16 files, +113/-31). Verified against merged QSL main (6eceb158b #1125 and 133e7f9f7 #1124): ADR-023 HM-7, PH-4/PH-5, 'What EN-1 admits', HV-8, PA-3, PA-6, CS-1 to CS-4; FR-174, FR-177, FR-178, FR-183, FR-184, FR-185 'Exact values', FR-096 'A stage limit names its kind'; and QSL code qsl-analyze/src/model_check/hyper.rs (NoPartnerPath, Halt::WitnessSet count), qsl-replay/src/model_check/hyper_replay.rs and settle.rs, qsl-semantics/src/check/hyper_clause.rs (copy group on HP-2 and HP-6), qsl-foundation/src/diagnostic/stage.rs and setting.rs (RationalBits, s3.rational_bits), qsl-semantics/src/check/check/typing.rs (quantity magnitude measured). Integrity pass: internal consistency of FR-394 to FR-403, FR-461, FR-460 and FR-272 after the edits, plus AC traceability. Traceability is complete. Each new or changed AC (FR-395-AC-6, FR-398-AC-2, FR-399-AC-3/4, FR-400-AC-4/5, FR-401-AC-10, FR-403-AC-2 and AC-4 to AC-8, FR-272-AC-10) names a TC whose frontmatter `verifies` the FR, and the TC has a row for it: VS-06; PA-03/PA-05; FE-03, SR-04; UE-12, UE-13; HR-12; RD-01, RD-02, RD-08 to RD-11; the tc_233 test. TC-348's Expected Results lists FR-401-AC-10, whose Verification column names TC-345 only. HR-12 points to UE-13, so both TCs cover it, and this is not a defect. FR-439 is unchanged and not affected. The native-diagnostics and FR-272 rows agree on the eight causes, as FR-272-AC-10 and its test require. Gates at HEAD (own clone, own CARGO_TARGET_DIR): `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents, byte-identical to origin/main's output after path normalisation; of the 15 touched documents only FR-272 appears, with its one pre-existing failure, so the PR adds none. `cargo test --test checked_package_v2 --test backend_provider_v1`: 112 and 11 passed. quoin 0.28.3; quire 0.36.2 (engine 0.50.2), newer than the 0.36.1 the rollout note names, with no behaviour difference observed.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-400-AC-3 | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-400-AC-5 | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-400-validation-lockstep-counts | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-400-kinds-table | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-400-AC-2 | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-400-AC-4 | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-403-preservation-table | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-v8-paragraph | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-AC-8 | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| TC-347-RD-08 | spec/test-cases/TC-347-hyperproperty-reductions-and-copy-swap.md | examined |
| FR-403-soundness | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-fairness | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-AC-5 | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-AC-6 | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-AC-7 | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-399-count-paragraph | spec/functional/analysis/FR-399-settle-hyperproperty-results.md | examined |
| FR-461-behaviour-1 | spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md | examined |
| FR-461-behaviour-2 | spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md | examined |
| FR-460-inputs | spec/functional/foundation/FR-460-bound-every-stage-by-resource-limits.md | examined |
| FR-272-stage_limit_exceeded-row | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| native-diagnostics-stage_limit_exceeded-rational-bits | proposals/quire-v1/definitions/native-diagnostics.md | examined |
| FR-272-AC-10 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-399-run-limits | spec/functional/analysis/FR-399-settle-hyperproperty-results.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Soundness bullet still argues that '`σ` ... maps acceptance sets, the rejecting monitor state and every fairness constraint onto themselves, so the quotient has a fair accepting cycle ... exactly when the product does'. The new Fairness bullet says the opposite for any variable with a fairness set: 'the quotient would decide fair cycles wrongly'. The new Acceptance condition bullet shows a swap that does not map the acceptance sets onto themselves. QSL rewrote the same argument (ADR-023 CS-3, 'Soundness, and when the swap is withheld'): `σ` is an automorphism 'when the variables it swaps carry no fairness set' and when it fixes the acceptance condition. Fix: make the bullet conditional. 'For a class with no fairness set, whose exchange fixes the automaton's initial state and each acceptance set, `σ` maps a matched tuple ... and the rejecting monitor state onto themselves, so ...'. Drop 'every fairness constraint'. | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md:120-124 |
| FND-002 | medium | The V-8 rule and AC-8 apply to 'a state constraint ... for a hyper item whose row reads "yes"', but no cell in the State constraint column reads 'yes'. The cells read 'refutation decisive; `proved` only when no boundary state is reached', 'lasso decisive; ...', 'as the row above', 'as the form's own row' or 'no: ...'. Read literally, the state-constraint half of the rule never applies, and RD-08's 'HB-01 with ... a state constraint selected ... since the preservation row reads "yes"' has no row to point to. QSL's PH table has the same cells; QSL's text names the state constraint outright instead of relying on the cell. Fix: say 'whose row admits it (every cell other than "no")', or name the state constraint of HP-1, HP-2 and HP-6 explicitly in the rule and in AC-8. | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md:58-65 |
| FND-003 | medium | AC-3 still lists 'Traces of unequal step count' as a refusal with no kind attached, while the new AC-5 says 'The paths of an `undefined` counterexample may differ in step count', and a `projected` counterexample's total lengths are free. The Validation rule it tests is scoped ('`lockstep` or `witness-exhausted` traces of unequal step counts'), but a test written from AC-3 can reasonably send an `undefined` or `projected` counterexample of unequal length and expect a refusal that AC-5 forbids. Fix: '`lockstep` or `witness-exhausted` traces of unequal step count, ...'. | spec/objects/interfaces/FR-400-hyper-counterexample.md:143 |
| FND-004 | medium | The paragraph cites FR-461 for the count ('as FR-461 states it') and then says 'A limit other than these two (`max_states`, `max_transitions`, `max_automaton_states` and the evaluation meter) carries no count'. FR-461 Behavior 1 says every limit outcome names 'Its value: the configured bound in force, and the count the denied charge would have reached', and Behavior 5 brings engine limits such as `model_check.max_states` under the same rule. The two statements contradict each other for every model-check limit other than the two hyper ones. QSL FR-184 has the exclusion, but QSpec FR-461 has no exception for it. Fix: either add the exception to FR-461 Behavior 1 (the model checker's state, transition, automaton and meter stops carry the bound only), or drop the exclusion from FR-399 and let those limits report a count too. | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:161-166 |
| FND-005 | medium | The row adds `rational-bits-exceeded` with `setting` `s3.rational_bits` and calls it 'the limit's FR-461 setting name'. FR-461 Behavior 2 limits the counter of a stage setting to 'a resource (bytes, tokens, nodes, edges, occurrences, diagnostics or work units)', and FR-460's Inputs and Behavior 1 list a stage's resource limits as 'input bytes, node and edge counts, occurrence and diagnostic counts, work units and time'. Neither list has rational bits, so `s3.rational_bits` is a setting FR-461 does not allow and the S3 ceiling is a limit FR-460 does not list. QSL FR-096 lists eight LimitKind variants, rational bits among them. Fix: add 'rational bits' to FR-461 Behavior 2's counter list and to FR-460's Inputs and Behavior 1 resource lists. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:91 |
| FND-006 | low | AC-8 writes the settlement as '`unsupported`, `unsupported-requested-capability`'. The statement it tests (line 51-52) and FR-399's unsupported row write '`unsupported_projection`/`unsupported-requested-capability`', with the code. The AC leaves the code out, so a result under another code would pass it. Fix: '`unsupported`, `unsupported_projection`/`unsupported-requested-capability`'. | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md:149 |

## Verdict

Not mergeable on integrity grounds until the medium findings are settled. FND-001: the FR-403 Soundness bullet contradicts the new Fairness and Acceptance rules. FND-002: no state-constraint cell reads 'yes', so half of the V-8 rule never applies. FND-003: FR-400-AC-3 is unscoped against AC-5. FND-004: the FR-399 'no count' exclusion contradicts FR-461. FND-005: `s3.rational_bits` is outside FR-461's and FR-460's closed resource lists. FND-006 is a missing code in an AC.

## Dispositions

Round 1, at 469fa12.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | After: - **Soundness.** For a class whose variables carry no fairness set and whose exchange fixes the automaton's initial state and each acceptance set, `σ` is an automorphism: it maps a matched tuple in the domain to a matched tuple in the domain with the same body truth, and maps the acceptance sets and the rejecting monitor state onto themselves, so the quotient has a fair accepting cycle, or a reachable rejecting state, exactly when the product does. A class with a fairness set or a failing exchange is not reduced, since `σ` is not then an automorphism of the fairness or acceptance condition. |
| FND-002 | fixed | After: A reduction is admitted where its cell reads anything other than "no". Where the cell reads "no", the item settles `inconclusive` with `reduction-not-preserving` before expansion. A cell other than "no" says the reduction preserves the verdict; in the state-constraint column it also says how the verdict is read when the constraint cuts states. Copy-swap symmetry applies only when the analyzer has verified it on the checked clause. |
| FND-003 | fixed | After: \| FR-400-AC-3 \| `lockstep` or `witness-exhausted` traces of unequal step count, mismatched variables or aliases, a misplaced or out-of-range `loop_entry` or `position`, a missing or misplaced `mark`, a terminal-stutter step followed by a non-stutter step or placed in a `step-tuple` trace, an `undefined` kind whose carried value is missing or mismatched, and an out-of-range `initial` each refuse as `invalid_runtime_input`/`invalid-value` before replay. \| Test (TC-345) \| |
| FND-004 | fixed | After: The locus is the one FR-272 retains for the cause. This requirement defines the count; an engine's stop record exposes it where the owning requirement says so (FR-399 for the model checker's hyper limits), and the settled `limit-reached` cause names the limit, its value and its setting. |
| FND-005 | fixed | After: stable dotted name: the stage, a dot and the counter, for example `s1.tokens`, `s3.nodes`, `s3.work_units` or `intake.input_bytes`. The counter names a resource (bytes, tokens, nodes, edges, occurrences, diagnostics, work units or rational bits), never a depth (FR-460). \| FR-460: resource limits: input bytes, node and edge counts, occurrence and diagnostic counts, rational bits, work units and time. No stage has a depth limit kind, |
| FND-006 | fixed | After: \| FR-403-AC-8 \| A key symmetry of an HP-1, HP-2 or HP-6 item and a state constraint of an HP-1, HP-2 or HP-6 item that the provider does not build, each selected on its own, settle `unsupported`, `unsupported_projection`/`unsupported-requested-capability` before expansion naming the reduction, and neither run ignores the selection; with a selection whose cell reads "no" and another that is admitted, the first "no" in the order key symmetry, partial-order reduction, state constraint decides. \| Test (TC-347) \| |

Round 1 at 469fa12: all six fixed. FR-461 Behavior 1 now says an engine's stop record exposes the count where its owning requirement says so, and that the settled cause names limit, value and setting. That makes FR-399 consistent with it. Gates at 469fa12: `quire validate --scope . 'spec/**/*.md'` gives 134 failed documents, the same set as round 0 and main. The five copied SR files under spec/reviews/ validate and are byte-identical to the round-0 records. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 112 and 11.
