---
id: SR-13617
title: 'Spec review of QSpec PR #304: hyperproperty and probabilistic follow-ups, EARS conformance'
type: SpecReview
analysis: ears-conformance
scope: 'agent-ix/quire-specification@90d6c3f; git diff origin/main...90d6c3f (PR #304, branch qspec-hyper-prob-followups): spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md, spec/functional/analysis/FR-398-align-hyperproperty-traces-on-a-projection.md, spec/functional/analysis/FR-399-settle-hyperproperty-results.md, spec/objects/interfaces/FR-400-hyper-counterexample.md, spec/functional/analysis/FR-401-replay-hyper-counterexamples.md, spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md; tickets QSL-553, QSL-554, QSL-555, QSL-573, QSL-883'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-395
  type: reviews
- target: ix://agent-ix/quire-specification/FR-399
  type: reviews
- target: ix://agent-ix/quire-specification/FR-400
  type: reviews
- target: ix://agent-ix/quire-specification/FR-401
  type: reviews
- target: ix://agent-ix/quire-specification/FR-403
  type: reviews
---

## Summary

Tickets: QSL-553, QSL-554, QSL-555, QSL-573, QSL-883. PR: quire-specification#304 (private), head 90d6c3fd3c85644fd43799464ed7400b9ea83a8a, branch qspec-hyper-prob-followups, scoped to `git diff origin/main...HEAD` (one commit, 16 files, +113/-31). Verified against merged QSL main (6eceb158b #1125 and 133e7f9f7 #1124): ADR-023 HM-7, PH-4/PH-5, 'What EN-1 admits', HV-8, PA-3, PA-6, CS-1 to CS-4; FR-174, FR-177, FR-178, FR-183, FR-184, FR-185 'Exact values', FR-096 'A stage limit names its kind'; and QSL code qsl-analyze/src/model_check/hyper.rs (NoPartnerPath, Halt::WitnessSet count), qsl-replay/src/model_check/hyper_replay.rs and settle.rs, qsl-semantics/src/check/hyper_clause.rs (copy group on HP-2 and HP-6), qsl-foundation/src/diagnostic/stage.rs and setting.rs (RationalBits, s3.rational_bits), qsl-semantics/src/check/check/typing.rs (quantity magnitude measured). EARS pass over the requirement statements this PR adds or rewrites. FR-401's new bullet is an unwanted-behaviour statement with three SHALL clauses about one counterexample (refuse, settle nothing, never read in lockstep). It is compound in form, but the three clauses are one response, so it is not flagged. FR-395 Vacuity, FR-398, FR-400 'Paths of an `undefined` counterexample' and FR-403's Fairness, Acceptance condition and Form bullets are definitional or descriptive, like the house style of the surrounding text, and each is atomic. Gates at HEAD (own clone, own CARGO_TARGET_DIR): `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents, byte-identical to origin/main's output after path normalisation; of the 15 touched documents only FR-272 appears, with its one pre-existing failure, so the PR adds none. `cargo test --test checked_package_v2 --test backend_provider_v1`: 112 and 11 passed. quoin 0.28.3; quire 0.36.2 (engine 0.50.2), newer than the 0.36.1 the rollout note names, with no behaviour difference observed.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-401-align-skip-undefined-refusal | spec/functional/analysis/FR-401-replay-hyper-counterexamples.md | examined |
| FR-403-v8-paragraph | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-399-count-paragraph | spec/functional/analysis/FR-399-settle-hyperproperty-results.md | examined |
| FR-395-vacuity | spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md | examined |
| FR-400-undefined-paths | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-403-fairness | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-acceptance-condition | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-form | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-398-projected-counterexample | spec/functional/analysis/FR-398-align-hyperproperty-traces-on-a-projection.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The paragraph holds three requirements in two sentences, with no SHALL and three triggers: a 'where' state (the provider builds neither), an event (a request selects one), and a 'when' (several are selected) with an 'otherwise'. The neighbouring requirement in FR-174 is written as separate SHALL bullets. Fix: three statements. 'Where a provider does not build a selected reduction whose row reads "yes", the analyzer SHALL settle the item `unsupported` ... before expansion, naming the reduction.' 'The analyzer SHALL NOT ignore a selected reduction.' 'When several reductions are selected, the analyzer SHALL decide by the first whose row reads "no", in the order ..., and otherwise SHALL name the first selected reduction.' | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md:48-56 |
| FND-002 | low | The count rule is written in the indicative ('is the count ...', 'carries no count') while the rest of FR-399's obligations use SHALL, and it puts the positive rule and the exclusion together. Fix: 'When a run stops at `max_witness_set` or `max_relation_tuples`, the analyzer SHALL report the count the denied charge would have reached, the value plus 1.' Then state the exclusion separately, after FND-001 of the base review settles where the count is carried. | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:161-166 |

## Verdict

Two low EARS findings: the FR-403 V-8 paragraph and the FR-399 count rule should be split into SHALL statements. Neither blocks the merge by itself. The blocking issues are in the base review.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The new trigger clause says the counterexample 'has no state at `trace_position` on every path'. That can be read as 'no path has a state at `trace_position`', which is false, since the universal paths do have one. The intended reading is 'not every path has a state at `trace_position`'. Fix: 'some path has no state at `trace_position`, so the row's state-key check cannot read every path at one position.' | spec/functional/analysis/FR-401-replay-hyper-counterexamples.md:78-86 |

## Dispositions

Round 1, at 469fa12.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | After: - Where a selected reduction is admitted by its cell and the provider does not build it for the item's form, the analyzer SHALL settle the item `unsupported`, `unsupported_projection`/`unsupported-requested-capability` before expansion, naming the reduction. The rule is evaluated for each selection on its own, for the key symmetry of every form (the orbit representatives of HP-1 and the diagonal action of HP-2, HP-3 and HP-6) and for the state constraint of HP-1, HP-2 and HP-6. - The analyzer SHALL NOT ignore a selected reduction. - When several reductions are selected, the analyzer SHALL deci |
| FND-002 | fixed | After: `max_witness_set` or `max_relation_tuples`, the engine's stop record SHALL expose that count, the value plus 1, since the run stops at the first member or tuple that would pass the limit and counts that one. The stop records of the other model-check limits (`max_states`, `max_transitions`, |

Round 1 at 469fa12: both fixed. New FND-003 (low): the FR-401 trigger clause is ambiguous. Gates at 469fa12: `quire validate --scope . 'spec/**/*.md'` gives 134 failed documents, the same set as round 0 and main. The five copied SR files under spec/reviews/ validate and are byte-identical to the round-0 records. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 112 and 11.
