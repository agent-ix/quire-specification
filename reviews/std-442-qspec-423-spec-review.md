---
id: SR-16305
title: "spec-review of STD-442 hybrid routing, solver methods and proof coverage (quire-specification PR 423)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@002fbf96; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md; spec/objects/interfaces/FR-331-backend-provider-envelope.md; spec/objects/protocol/FR-290-protocol-claim-kind.md; spec/objects/foundation/FR-272-typed-diagnostic-cause.md; proposals/quire-v1/definitions/native-diagnostics.md; spec/test-cases/TC-373-hybrid-verdict-map.md; spec/test-cases/TC-431-backend-provider-wire-vectors.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-193
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/TC-373
    type: references
  - target: ix://agent-ix/quire-specification/TC-431
    type: references
---

# Spec review of STD-442 hybrid routing, solver methods and proof coverage (quire-specification PR 423)

## Summary

Ticket: STD-442 (gaps G1, G2, G3, G5 of the QSL-900 audit). One spec-review pass over the diff against origin/main 76b7cd48 (an ancestor of head 002fbf96), read with the QSL half, quire-spec-language#1385 at 7bb6fc37.

Examined, clean:

- Routing (G1): the `requirements` item member, the two-way descriptor `dynamics` set, the candidate rule in FR-193 and FR-290 "matches", `inconsistent-candidates` for a listed non-matching backend, `unsupported-requested-capability` for no candidate, request identity in FR-331, and the `invalid-dynamics` registration cause with its defect order (not-a-list, empty, unknown-label, duplicate-label, missing-bounds; judged after pairs, domains, fairness kinds and evidence kinds). FR-272 and native-diagnostics list the cause and its retained members. QSL FR-075 states the same rule and order.
- Hand-derived: FR-193-AC-18 and FR-290-AC-15 candidate sets (`[dreal]`, `[qsl-model-check]`, both sets and `ambiguous-backend` with `{discrete, hybrid}`, empty set when naming `qsl-model-check`, `inconsistent-candidates` naming `qsl-model-check`). FR-193-AC-21: `always [0, 10] holds(h.v < 30)` has reach 10, so horizons 10 and 21/2 cover and 1 does not; `always [0, 10] eventually [0, 5]` has reach 15; `eventually [0, 1]` under model-steps has reach 1 position, so `jumps` 1 covers and 0 does not; `always` with no interval and `on each` have no horizon. These agree with QSL FR-447's reach rule and FR-447-AC-3/AC-10.
- Method table (G2): five kinds, their spellings and bases, and the sound outcome of each, as FR-193-AC-20 and HY-19 state. The first-party refusal at `/basis/method` and the plugin `trusted` rule match QSL FR-250 and FR-290.
- Changed vectors: `certification-outcome`'s `bounded-solver` method `dreal` is now `delta-complete`, because `dreal` is outside the closed table. `hybrid-request` now routes to a `dreal` descriptor with `dynamics` `["hybrid"]` and both budgets in `bounds`. No QSL code reads these vectors.
- G5: the "QSL's built-in hybrid solver" phrase is gone, and each hybrid provider publishes its own defaults. This matches merged ADR-026 RU-7/HY-3 and the ticket's own G5 instruction (state whether a built-in solver exists). It removes no specified capability, so it is not a scope deferral. The PR body flags the owner question without narrowing anything.
- No drop, narrowing or deferral language in the diff. The new AC ids, FR-193-AC-18..21, FR-331-AC-41 and FR-290-AC-15, appear on no other open QSpec PR head. HY-17..HY-20 are scoped to TC-373; TC-966's HY-17..HY-20 are its own rows.

Gates at 002fbf96: `quire validate --scope . 'spec/**/*.md'` gives 86 failure lines, all pre-existing. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 24 + 139. QSL conformance `cargo test -p qsl-package --lib checked_v2` with QSPEC_DIR at this head passes 49.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | "Proof coverage" defines covering as reading every state the claim reads, then lets each clock ignore one of the proof's two bounds. Under model-time, `horizon >= r` covers whatever `jumps` is, so a `jumps` 0 proof settles `proved` for a claim that reads states after a jump that can occur before `r`. Under model-steps, `jumps >= n` covers whatever `horizon` is, so FR-193-AC-21's `jumps` 1 with `horizon` 10 settles `proved` for `eventually [0, 1]` over `Braking`, where position 1 can be reached at t = 20 (v = 20, a = 1), outside the proof. Either way the verdict is `proved` while the proof did not read states the claim reads, which contradicts the definition and the "proved only where ... its proof covers the claim" rule (QSL ADR-026 RU-7/HY-2). Fix: make coverage require both bounds. One way is for the solver to report that no behaviour exceeds its jump bound by `horizon` (and, under model-steps, that every behaviour makes its `n`-th jump by `horizon`), and otherwise settle `insufficient-horizon`. Or keep the residual bound, but state it as a qualification of the `proved` verdict, drop the "reads every state the claim reads" definition, and have an AC pin the jumps-0 case. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:400-413 | wrong-requirement |
| FND-002 | medium | Row id BP-36 is now used twice in TC-431. Main already has BP-36 (line 59, `hybrid-refuted-point-trace-outcome`), and the PR adds a second BP-36 (line 65) for the routing and coverage vectors. The PR body says the ids were checked, but the check covered open PR heads, not main. Fix: renumber the new row to the next free id (BP-37). | spec/test-cases/TC-431-backend-provider-wire-vectors.md:59,65 | wrong-requirement |
| FND-003 | medium | HY-18's ordering case "one with both `["continuous"]` and missing bounds" cannot fail on order. `missing-bounds` applies only when `dynamics` holds `hybrid`, so `["continuous"]` has only the `unknown-label` defect, and an implementation that checks bounds first still reports `unknown-label`. Fix: use `["continuous", "hybrid"]` (or `["hybrid", "continuous"]`) with `bounds` lacking `max_solver_time`, expecting `unknown-label`. | spec/test-cases/TC-373-hybrid-verdict-map.md:45 | correct-requirement-no-evidence |
| FND-004 | low | FR-290's candidate table row "empty" says the warning names the item's kind and any named backend. FR-193 "Routing a hybrid item" and FR-193-AC-18 say the warning also names `hybrid-dynamics`, and FR-272 retains `requirements`. Fix: add "and its `requirements` when it carries them (FR-193)" to the FR-290 row. | spec/objects/protocol/FR-290-protocol-claim-kind.md:337 | wrong-requirement |
| FND-005 | low | QSpec does not say how a reader handles `insufficient-horizon` on an item whose claim is not a hybrid claim, though QSL FR-290 refuses it `Malformed` at `/cause/type`. The `trace` of `solver-inconclusive` and a `hybrid-counterexample` each have such a rule (FR-331 "Plugin entries", FR-331-AC-34). Fix: state the rule for `insufficient-horizon` in FR-193 "Solver result on the wire" or in FR-331, with its `invalid_result` cause and pointer. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:285 | missing-requirement |

## Verdict

Not mergeable. FND-001 (high) blocks the PR, and FND-002 (a duplicate row id) must be fixed with it. FND-003 to FND-005 should be fixed in the same round. Routing, the method table, the schema and the hand-derived vectors are otherwise correct and consistent with QSL #1385.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-006 | medium | The new worked example and FR-193-AC-21 rest on a false premise: "Over `Braking`, `v` falls from 20 ... so the one jump ... occurs at a time between 10 and 20". `Braking` (QSL FR-447) has no `initial` clause, so by "Initial states" and FR-193-AC-11 its initial states are all of `Moving`'s invariant: `v` in [0, 20] and `a` in [1, 2]. The jump time `v0 / a` therefore ranges over [0, 20], and `v0 = 0` jumps at time 0. The verdicts stated still follow from the rule, but the rationale "a behaviour may jump at time 10" and the AC's premise are wrong, and an implementer building the fixture from them would derive wrong `exhaustive` values. Fix: say the one jump occurs at a time between 0 and 20 (`v0 / a`, with `v0 <= 20` and `a >= 1`), and that the `jumps` 0 proof misses every behaviour that jumps by time 10. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:443-457,730 | wrong-requirement |
| FND-007 | low | Two HY-20 / FR-193-AC-21 fixture results claim `exhaustive` dimensions that are false for `Braking`, which FR-193 forbids ("a provider SHALL list a dimension only when its solver established it"). `horizon` 10, `jumps` 3 with `["jumps"]` is false, since the first jump can occur at time 20, after 10. `horizon` 100, `jumps` 0 with `["time"]` is false, since every behaviour jumps by time 20. The verdicts still follow from the coverage rule. Fix: use values that hold, for example `horizon` 20, `jumps` 3, `["jumps"]` against the model-time claim (still `insufficient-horizon`, since model-time needs `time`), and `horizon` 100, `jumps` 0, `["jumps"]` (vacuously true). | spec/test-cases/TC-373-hybrid-verdict-map.md:47 | correct-requirement-no-evidence |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | fixed | - |
