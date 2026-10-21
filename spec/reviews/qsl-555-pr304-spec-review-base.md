---
id: SR-13616
title: 'Spec review of QSpec PR #304: hyperproperty and probabilistic follow-ups, base'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@90d6c3f; git diff origin/main...90d6c3f (PR #304, branch qspec-hyper-prob-followups): proposals/quire-v1/definitions/native-diagnostics.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md, spec/functional/analysis/FR-398-align-hyperproperty-traces-on-a-projection.md, spec/functional/analysis/FR-399-settle-hyperproperty-results.md, spec/objects/interfaces/FR-400-hyper-counterexample.md, spec/functional/analysis/FR-401-replay-hyper-counterexamples.md, spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md, spec/test-cases/TC-339-forall-exists-safety-hyperproperty.md, spec/test-cases/TC-340-hyper-vacuity-and-stutter.md, spec/test-cases/TC-343-step-relations-model-and-code.md, spec/test-cases/TC-344-projection-aligned-hyperproperty.md, spec/test-cases/TC-345-hyper-counterexample-wire-and-replay-refusals.md, spec/test-cases/TC-347-hyperproperty-reductions-and-copy-swap.md, spec/test-cases/TC-348-undefined-evaluation-refutes-hyperproperties.md, tests/checked_package_v2.rs; tickets QSL-553, QSL-554, QSL-555, QSL-573, QSL-883'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-395
  type: reviews
- target: ix://agent-ix/quire-specification/FR-398
  type: reviews
- target: ix://agent-ix/quire-specification/FR-399
  type: reviews
- target: ix://agent-ix/quire-specification/FR-400
  type: reviews
- target: ix://agent-ix/quire-specification/FR-401
  type: reviews
- target: ix://agent-ix/quire-specification/FR-403
  type: reviews
- target: ix://agent-ix/quire-specification/FR-272
  type: reviews
- target: ix://agent-ix/quire-specification/TC-347
  type: reviews
- target: ix://agent-ix/quire-specification/TC-348
  type: reviews
---

## Summary

Tickets: QSL-553, QSL-554, QSL-555, QSL-573, QSL-883. PR: quire-specification#304 (private), head 90d6c3fd3c85644fd43799464ed7400b9ea83a8a, branch qspec-hyper-prob-followups, scoped to `git diff origin/main...HEAD` (one commit, 16 files, +113/-31). Verified against merged QSL main (6eceb158b #1125 and 133e7f9f7 #1124): ADR-023 HM-7, PH-4/PH-5, 'What EN-1 admits', HV-8, PA-3, PA-6, CS-1 to CS-4; FR-174, FR-177, FR-178, FR-183, FR-184, FR-185 'Exact values', FR-096 'A stage limit names its kind'; and QSL code qsl-analyze/src/model_check/hyper.rs (NoPartnerPath, Halt::WitnessSet count), qsl-replay/src/model_check/hyper_replay.rs and settle.rs, qsl-semantics/src/check/hyper_clause.rs (copy group on HP-2 and HP-6), qsl-foundation/src/diagnostic/stage.rs and setting.rs (RationalBits, s3.rational_bits), qsl-semantics/src/check/check/typing.rs (quantity magnitude measured). Confirmed against QSL: FR-395 HP-6 never vacuous = HM-7/PA-3 and FR-178 Vacuity, with VS-06 = FR-178-AC-7. FR-398/FR-400 `projected` lengths free = PA-6 and FR-178 Outputs; TC-344 PA-05 = FR-178-AC-2 (lengths 1 and 2 mid-operation, 1 and 1 idle), checked by hand on the leaky post `l = (i + h) mod 2`. FR-399 count = value + 1 for the two hyper limits only = FR-184 and its AC-2/AC-3 (FE-03 3, SR-04 2). FR-400 `undefined` paths = FR-177 (universal paths include the step, existential paths one short, `trace_position` the entered position; one variables/state_keys entry per trace however many universals, one for a contract conjunction) and FR-178 (unmarked, projected position); UE-12 = FR-177-AC-10, UE-13 = FR-178-AC-9. FR-401 align-skip refusal = FR-183 and FR-183-AC-7, HR-12 = TC-608 step 5. FR-403: the fairness rule, the all-or-nothing acceptance rule, HP-2-only copy-swap, the quotient's canonical lasso, the run's group and the V-8 order (key symmetry, partial order, state constraint; first 'no' decides, otherwise the first selected is named) = FR-174 and ADR-023 CS-1 to CS-4 and 'What EN-1 admits'. AC-2 7/6 against 10/8 = FR-174-AC-2, rechecked by hand: 10 = 4 + 2 x 3 unequal-h states, 7 = 4 fixed + 3 pairs. HP-6 copy-group detection matches qsl-semantics hyper_clause.rs:995-996. `rational-bits-exceeded` with `s3.rational_bits` = QSL FR-096 (eight LimitKind variants), FR-185 'Exact values' and setting.rs:69. The scope wording 'rational, decimal or quantity magnitude, and ... a distribution total' matches FR-185 and typing.rs:924-929 (quantity magnitude measured). FR-439 is not touched, as intended. No changelog, status or owner wording in the added spec text. Owner question 1 is stated exactly as QSL FR-177 and HV-8 state it (`HyperRefusal::NoPartnerPath`, no terminal record, no V-x settlement, no catalog code; hyper.rs:688, 758, 1651). Its omission from FR-400 is not a finding. Owner question 2 is assessed in FND-003 and FND-004, and the QSL-565 note in FND-008. Gates at HEAD (own clone, own CARGO_TARGET_DIR): `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents, byte-identical to origin/main's output after path normalisation; of the 15 touched documents only FR-272 appears, with its one pre-existing failure, so the PR adds none. `cargo test --test checked_package_v2 --test backend_provider_v1`: 112 and 11 passed. quoin 0.28.3; quire 0.36.2 (engine 0.50.2), newer than the 0.36.1 the rollout note names, with no behaviour difference observed.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-395-vacuity | spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md | examined |
| FR-395-AC-6 | spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md | examined |
| TC-340-VS-06 | spec/test-cases/TC-340-hyper-vacuity-and-stutter.md | examined |
| TC-340-expected | spec/test-cases/TC-340-hyper-vacuity-and-stutter.md | examined |
| FR-398-projected-counterexample | spec/functional/analysis/FR-398-align-hyperproperty-traces-on-a-projection.md | examined |
| FR-398-AC-2 | spec/functional/analysis/FR-398-align-hyperproperty-traces-on-a-projection.md | examined |
| TC-344-description | spec/test-cases/TC-344-projection-aligned-hyperproperty.md | examined |
| TC-344-PA-03 | spec/test-cases/TC-344-projection-aligned-hyperproperty.md | examined |
| TC-344-PA-05 | spec/test-cases/TC-344-projection-aligned-hyperproperty.md | examined |
| FR-399-count-paragraph | spec/functional/analysis/FR-399-settle-hyperproperty-results.md | examined |
| FR-399-AC-3 | spec/functional/analysis/FR-399-settle-hyperproperty-results.md | examined |
| FR-399-AC-4 | spec/functional/analysis/FR-399-settle-hyperproperty-results.md | examined |
| TC-339-FE-03 | spec/test-cases/TC-339-forall-exists-safety-hyperproperty.md | examined |
| TC-343-SR-04 | spec/test-cases/TC-343-step-relations-model-and-code.md | examined |
| TC-345-HR-06 | spec/test-cases/TC-345-hyper-counterexample-wire-and-replay-refusals.md | examined |
| FR-331-incomplete-cause | spec/objects/interfaces/FR-331-backend-provider-envelope.md | context_only |
| backend-provider-v1-LimitReached | proposals/backend-provider-v1/schema.json | context_only |
| FR-400-UndefinedEvaluation | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-400-undefined-paths | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-400-AC-4 | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-400-AC-5 | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| TC-348-UE-12 | spec/test-cases/TC-348-undefined-evaluation-refutes-hyperproperties.md | examined |
| TC-348-UE-13 | spec/test-cases/TC-348-undefined-evaluation-refutes-hyperproperties.md | examined |
| FR-401-align-skip-undefined-refusal | spec/functional/analysis/FR-401-replay-hyper-counterexamples.md | examined |
| FR-401-AC-10 | spec/functional/analysis/FR-401-replay-hyper-counterexamples.md | examined |
| TC-345-HR-12 | spec/test-cases/TC-345-hyper-counterexample-wire-and-replay-refusals.md | examined |
| FR-399-description | spec/functional/analysis/FR-399-settle-hyperproperty-results.md | context_only |
| FR-399-AC-7 | spec/functional/analysis/FR-399-settle-hyperproperty-results.md | context_only |
| FR-403-preservation-lead | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-v8-paragraph | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-preservation-table | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-copy-swap-row | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-detection | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-fairness | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-acceptance-condition | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-form | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-canonical-state | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-counterexamples | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-AC-1 | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-AC-2 | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-AC-4 | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-AC-5 | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-AC-6 | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-AC-7 | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-AC-8 | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| TC-347-description | spec/test-cases/TC-347-hyperproperty-reductions-and-copy-swap.md | examined |
| TC-347-RD-01 | spec/test-cases/TC-347-hyperproperty-reductions-and-copy-swap.md | examined |
| TC-347-RD-08 | spec/test-cases/TC-347-hyperproperty-reductions-and-copy-swap.md | examined |
| TC-347-RD-09 | spec/test-cases/TC-347-hyperproperty-reductions-and-copy-swap.md | examined |
| TC-347-RD-10 | spec/test-cases/TC-347-hyperproperty-reductions-and-copy-swap.md | examined |
| TC-347-RD-11 | spec/test-cases/TC-347-hyperproperty-reductions-and-copy-swap.md | examined |
| TC-347-expected | spec/test-cases/TC-347-hyperproperty-reductions-and-copy-swap.md | examined |
| native-diagnostics-stage_limit_exceeded-rational-bits | proposals/quire-v1/definitions/native-diagnostics.md | examined |
| FR-272-stage_limit_exceeded-row | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-272-AC-10 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-439-boundary-state | spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md | context_only |
| FR-439-AC-3 | spec/functional/memory/FR-439-bound-replay-and-report-weak-memory-runs.md | context_only |
| PR-304-owner-question-1 | PR #304 body, 'Owner question 1' | examined |
| PR-304-owner-question-2 | PR #304 body, 'Owner question 2' | examined |
| PR-304-QSL-565-note | PR #304 body, 'Terra's QSL-565 question' | examined |
| PR-304-judgment-calls | PR #304 body, 'Judgment calls' | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-399-AC-3 and AC-4 now require the settled `limit-reached` to name 'the count reached, one past the value', and FR-399 line 161 speaks of 'the `count` of a `limit-reached` record'. The QSpec wire has no such member. FR-399 settles 'FR-331 result `incomplete` with cause `limit-reached{limit, value, setting}`' (line 84 and the Run limits paragraph), FR-331 line 107 defines that cause with those three members, and backend-provider-v1/schema.json:669 `LimitReached` requires exactly type, limit, value and setting with `additionalProperties: false`. A result that carries a count fails the schema, so AC-3, AC-4, TC-339 FE-03 ('the count reached, 3'), TC-343 SR-04 (2) and TC-345 HR-06 cannot pass. FR-401 and FR-401-AC-4 also still write the replay stop as `{limit, value, setting}`, so HR-06's count is not in its own requirement. QSL agrees with the wire, not with the ACs: FR-184 puts the count on the engine's `Stopped` outcome, and qsl-replay/src/model_check/settle.rs:326 drops it (`ModelCheckOutcome::Stopped { cause, limit, .. }`) when it settles the record. Fix: either add an optional `count` member to FR-331's `limit-reached` and to the `LimitReached` schema (and to FR-401's replay stop), or state the count as the engine outcome's and keep it out of the settled cause, the ACs and the four TC rows. | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:179 |
| FND-002 | high | UE-13 expects the align-skip clause to settle '`refuted`, cause `undefined-evaluation` `division-by-zero`' while 'replay is refused (TC-345 HR-12)'. FR-399's Description says the analyzer 'SHALL settle a refutation only after its counterexample replays', FR-399-AC-7 tests that, and the undefined row of the Verdicts table requires 'the `undefined` counterexample ending there replays'. FR-401's new bullet and AC-10 say the refused counterexample 'settles no result'. So UE-13's expected `refuted` is forbidden by three statements, and FR-399's Outputs ('One result per item') is broken by 'settles no result'. The PR body leaves this open as a judgment call, but the TC row picks an answer the spec rules out. QSL does not settle it either: FR-178-AC-9 is the engine's `Violated` before replay, and FR-183-AC-7 refuses the replay. Fix: until FR-399 states the outcome of an item whose only counterexample cannot be replayed, UE-13 should expect what the spec gives: the engine finds the `undefined` counterexample and the replay boundary refuses it `unsupported_construct`/`not-yet-implemented` with no settled result. Or add that row to FR-399 (for example `inconclusive`, `replay-refused`) and use it in UE-13, FR-401 and AC-10. | spec/test-cases/TC-348-undefined-evaluation-refutes-hyperproperties.md:48 |
| FND-003 | medium | Owner question 2 asks whether the provider-conditional V-8 wording is coherent. As written it is not. FR-403-AC-1 still says 'Each form admits symmetry, partial-order reduction and a state constraint exactly as its row reads', and the section's first sentence (line 43) still says 'A reduction applies exactly where its row reads "yes"'. The new paragraph (lines 48-56) and AC-8 say a 'yes' selection settles `unsupported` on a provider that does not build it. The reference provider, QSL, is exactly that provider, so it fails AC-1 and line 43 on every 'yes' selection while passing AC-8. The Outputs section (lines 38-39: 'The reductions applied ... or the `reduction-not-preserving` settlement') and TC-347's Expected Results ('A reduction applies to a hyper item only by its row') leave out the `unsupported` outcome too. Fix: restate line 43 and AC-1 as 'a "no" row settles `reduction-not-preserving`; a "yes" row preserves the verdict, and the reduction is applied or settles `unsupported` (below)'. Add the `unsupported` settlement to Outputs and to TC-347's Expected Results. | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md:142 |
| FND-004 | medium | The condition 'Where a provider builds neither the diagonal action of a key symmetry nor a state constraint on a hyper product' covers only a provider that builds both or neither. It does not say what a provider that builds one of them does when asked for the other. Because 'and never runs with the selection ignored' sits inside the same conditional sentence, such a provider is free to ignore the selection it cannot build. 'Diagonal action' also does not describe the HP-1 row's symmetry ('the first execution ranges over orbit representatives, the others over full orbits'), yet TC-347 RD-08 applies the rule to HB-11 (HP-1). QSL FR-174 states the rule unconditionally for its own EN-1. Fix: state it per selection and unconditionally: 'A selected reduction whose row reads "yes" and which the provider does not build for that form settles `unsupported`, `unsupported_projection`/`unsupported-requested-capability` before expansion, naming the reduction; no selection is ignored.' Then the HP-1 symmetry and each state constraint are covered on their own. | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md:48-56 |
| FND-005 | medium | On TC-347's stated vault (the secure post is `self.l = i`, initial `l = 0`, no fairness), a behaviour that always takes `step(0)` keeps `l = 0`, so the second body `eventually holds(v.l @ a = 1) and eventually holds(v.l @ b = 1)` is refuted. FR-403 Verdicts says 'A refutation names no reduction', so 'the second names copy-swap' cannot be observed. The first body is refuted too (`step(1)` sets `l = 1`), so 'names no reduction' cannot tell a whole product from a reduced one. QSL FR-174-AC-6 runs the second body over a vault whose `step` toggles `l`, from `(0, 0)` and `(0, 1)`, and checks stored counts (5 against 8). Fix: give RD-10 a vault and initial states on which the second body holds, and compare stored-state counts (whole against reduced) for both bodies instead of the named reductions. | spec/test-cases/TC-347-hyperproperty-reductions-and-copy-swap.md:35 |
| FND-006 | medium | RD-09's fairness constraint is `fair weak V::Vault::reset`, but TC-347's vault declares 'one operation `step(i: Int[0, 1])`'. A fairness constraint that names an undeclared operation is refused before any run (QSL hyper.rs: 'the fairness constraint on `{name}` names an operation the subject does not declare'), so the vector cannot produce 'stored whole ... with the unreduced stored-state count'. The second run 'names copy-swap with class `{c, d}` alone' only if it is proved, but no body or verdict is given. QSL FR-174-AC-5 gives the whole fixture: a `reset` enabled at `l = 1` that keeps it, `step` flipping `l`, `a = (0, 0)` and `b = (0, 1)`, body, match and the 4-state count. Fix: declare the `reset` variant in TC-347's description and give RD-09's body, match, initial states and expected counts and verdicts. | spec/test-cases/TC-347-hyperproperty-reductions-and-copy-swap.md:34 |
| FND-007 | low | TC-344's fixture never states the busy/mix vault's initial snapshots, so PA-01 to PA-04 run over unspecified initial states. PA-03 then has to say its lengths are 'fixture-dependent: see PA-05'. ADR-023 §13 now fixes them: idle `(0, 0, false)` and `(1, 0, false)`, with a variant where the secret-1 run starts at `(1, 0, true)`. Fix: state the idle snapshots in the description, then give PA-03's lengths as 1 and 1 (or point to PA-05) without the hedge. | spec/test-cases/TC-344-projection-aligned-hyperproperty.md:17 |
| FND-008 | low | The note's main point holds. Under `tso` the cut store is the one past `max_store_buffer`, so that thread's buffer is full and non-empty and its `flush` is enabled, and a state whose only enabled steps are cut stores cannot arise. The FR-439 boundary-state rule is therefore vacuous under `tso` alone, and QSL FR-225-AC-4's scenario (AC-2 with bound 2, only the third store enabled) is QSL's to fix. Two details are wrong. (1) FR-439-AC-3 is not vacuous under `tso`: bound-limited states do arise (FR-439-AC-2), and 'never terminal or a deadlock' holds for them trivially because `flush` is enabled. (2) The argument also needs `max_store_buffer` >= 1, which holds only because backend-provider-v1/schema.json types it `PositiveCount`; FR-439 says only 'no ceiling'. Fix (PR body only): say the boundary-state rule, not AC-3, is vacuous for `tso`, and cite the positive bound. | PR #304 body:Terra's QSL-565 question |

## Verdict

Not mergeable. Two high findings. FND-001: the `count` the ACs and four TC rows require has no member on the FR-331 `limit-reached` wire, whose schema forbids extra members, and QSL drops it at settlement too. FND-002: TC-348 UE-13 expects `refuted` for a counterexample whose replay the spec refuses, and FR-399 forbids a refutation without replay. Owner question 2: the provider-conditional wording is not coherent as written. Line 43 and AC-1 contradict it (FND-003), and the 'builds neither' condition leaves a provider that builds only one reduction free to ignore the other (FND-004). Stated per selection and unconditionally, it would be coherent and would keep the rows' meaning as verdict-preserving. FND-005 and FND-006 are TC-347 fixtures that cannot show what they claim. The rest of the alignment with QSL #1125 and #1124 checks out statement by statement.
