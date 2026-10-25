---
id: SR-13619
title: 'Spec review of QSpec PR #304: hyperproperty and probabilistic follow-ups, failure domain'
type: SpecReview
analysis: failure-domain
scope: 'agent-ix/quire-specification@90d6c3f; git diff origin/main...90d6c3f (PR #304, branch qspec-hyper-prob-followups): spec/objects/interfaces/FR-400-hyper-counterexample.md, spec/functional/analysis/FR-401-replay-hyper-counterexamples.md, spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md, spec/test-cases/TC-348-undefined-evaluation-refutes-hyperproperties.md; tickets QSL-553, QSL-554, QSL-555, QSL-573, QSL-883'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-400
  type: reviews
- target: ix://agent-ix/quire-specification/FR-401
  type: reviews
- target: ix://agent-ix/quire-specification/TC-348
  type: reviews
---

## Summary

Tickets: QSL-553, QSL-554, QSL-555, QSL-573, QSL-883. PR: quire-specification#304 (private), head 90d6c3fd3c85644fd43799464ed7400b9ea83a8a, branch qspec-hyper-prob-followups, scoped to `git diff origin/main...HEAD` (one commit, 16 files, +113/-31). Verified against merged QSL main (6eceb158b #1125 and 133e7f9f7 #1124): ADR-023 HM-7, PH-4/PH-5, 'What EN-1 admits', HV-8, PA-3, PA-6, CS-1 to CS-4; FR-174, FR-177, FR-178, FR-183, FR-184, FR-185 'Exact values', FR-096 'A stage limit names its kind'; and QSL code qsl-analyze/src/model_check/hyper.rs (NoPartnerPath, Halt::WitnessSet count), qsl-replay/src/model_check/hyper_replay.rs and settle.rs, qsl-semantics/src/check/hyper_clause.rs (copy group on HP-2 and HP-6), qsl-foundation/src/diagnostic/stage.rs and setting.rs (RationalBits, s3.rational_bits), qsl-semantics/src/check/check/typing.rs (quantity magnitude measured). Failure-domain pass over the new counterexample shapes and refusals. Checked and not defective: the HP-6 `undefined` shape. FR-401 refuses it outright, so its unequal lengths never reach the lockstep reader (FR-401 align-skip bullet, QSL hyper_replay.rs:111-115). The copy-swap counterexample is deterministic, since the quotient's canonical lasso is a function of the request (FR-403 Counterexamples, QSL FR-174-AC-3). The V-8 selection is settled before expansion, so no partial state leaks. Its provider-conditional gap is in base FND-004. The empty-X refusal (owner question 1) is deliberately unstated in FR-400 and is not assessed as a finding. The unreplayable-item outcome is in base FND-002. Gates at HEAD (own clone, own CARGO_TARGET_DIR): `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents, byte-identical to origin/main's output after path normalisation; of the 15 touched documents only FR-272 appears, with its one pre-existing failure, so the PR adds none. `cargo test --test checked_package_v2 --test backend_provider_v1`: 112 and 11 passed. quoin 0.28.3; quire 0.36.2 (engine 0.50.2), newer than the 0.36.1 the rollout note names, with no behaviour difference observed.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-400-undefined-paths | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-400-UndefinedEvaluation | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-401-undefined-replay-row | spec/functional/analysis/FR-401-replay-hyper-counterexamples.md | examined |
| TC-348-UE-12 | spec/test-cases/TC-348-undefined-evaluation-refutes-hyperproperties.md | examined |
| FR-400-AC-5 | spec/objects/interfaces/FR-400-hyper-counterexample.md | examined |
| FR-401-align-skip-undefined-refusal | spec/functional/analysis/FR-401-replay-hyper-counterexamples.md | examined |
| FR-403-v8-paragraph | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| FR-403-counterexamples | spec/functional/analysis/FR-403-reduce-hyperproperty-state-spaces.md | examined |
| PR-304-owner-question-1 | PR #304 body, 'Owner question 1' | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | For an undefined `μ_U` on an HP-3 joint step, the existential paths now 'end at the pre-state the step leaves, one step short', with `trace_position` the position the step enters. Two rules this shape must satisfy are left unchanged. FR-400's `where.state_keys` holds 'each component's `quire.simulation.state-key/v1` digest there', at the joint position, and for a match locus it must hold one entry per trace. FR-401's `undefined` replay row checks that 'each component's state key at `trace_position` equals the carried `where.state_keys`'. A short existential path has no state at `trace_position`, so neither rule says which key its entry carries or how replay checks it, yet UE-12 expects 'replay reproduces it'. QSL's replay does not handle the shape either: qsl-replay/src/model_check/hyper_replay.rs `shape()` refuses an `Undefined` counterexample that 'holds traces of unequal step counts', and QSL FR-183 'Undefined evaluation' is silent on it. Fix: state that a short path's `state_keys` entry is the key of its last state (the pre-state), and that FR-401's `undefined` row compares each path's last state with its entry, the universal paths at `trace_position` and the short existential paths at `trace_position - 1`. | spec/objects/interfaces/FR-400-hyper-counterexample.md:88-96 |

## Verdict

One medium finding: the HP-3 short existential path has no defined `state_keys` entry and no replay check, so UE-12's 'replay reproduces it' cannot be tested as written, and QSL's replay refuses the shape today. Fix it with base FND-002 before merge.

## Dispositions

Round 1, at 469fa12.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | After: of its steps. A `state_keys` entry is the key of the last state of its component's path, so for an existential path that ends one step short its entry is the key of its pre-state, the state at `trace_position` minus 1. \| FR-401: - An `undefined` counterexample whose paths differ in step count (that of an `align skip` clause, which holds its skipped steps unmarked and whose position counts visible steps, and that of an HP-3 clause whose undefined `μ_U` leaves the existential paths one step short) has no state at `trace_position` on every path, so the row's state-key check cannot read it against one position. The replay boundary SHALL refuse it as `unsupported_construct`/`not-yet-implemented`, SHALL settle no result from it and SHALL NOT read it in lockstep, which would pair a skipped step of one path with a visible ste |

Round 1 at 469fa12: fixed by the PR's judgment call. FR-400 defines the short path's `state_keys` entry (its pre-state's key), and FR-401 and AC-10 refuse every `undefined` counterexample whose paths differ in step count, HP-3 short existential paths included, as `unsupported_construct`/`not-yet-implemented`, with HR-12 and UE-12 following. This is coherent with FR-400, which now admits that shape on the wire (AC-3 scoped, AC-5), so a refusal on the replay side is correct. QSL code is behind and needs a follow-up: qsl-replay hyper_replay.rs `shape()` refuses unequal step counts as `HyperShape`, which replay.rs maps to `invalid_runtime_input`/`invalid-value`, not `unsupported_construct`/`not-yet-implemented`, and QSL FR-183 says nothing about the HP-3 case. The PR body states this. Gates at 469fa12: `quire validate --scope . 'spec/**/*.md'` gives 134 failed documents, the same set as round 0 and main. The five copied SR files under spec/reviews/ validate and are byte-identical to the round-0 records. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 112 and 11.
