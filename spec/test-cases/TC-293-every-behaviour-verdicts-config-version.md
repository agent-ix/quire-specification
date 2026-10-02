---
id: TC-293
title: "Every-behaviour verdicts on the ConfigVersion subject"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-363
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-362
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-364
    type: verifies
---

# TC-293: Every-behaviour verdicts on the ConfigVersion subject

## Description

The worked example: liveness under weak fairness at both granularities, the
shipped unit's refutation, and a bounded variant, each with its expected
verdict and, for a refutation, a counterexample that must reproduce.

## Test Procedure

The **ConfigVersion** subject: object type `ConfigVersion` with
`versionNumber: Int[0, 1000]` and optional `parent`, population
`config_history`, and operation `attemptUpdate` with frame
`modifies [versionNumber]`, no parameters and a Boolean result, whose
postcondition is `self.versionNumber = (pre(self.versionNumber) + 1) mod 3`.
Universe `config_history = {a, b}`; one initial snapshot with both at version
0 and `parent` absent. Write a state `(va, vb)` and `upd(x)` for
`attemptUpdate` with receiver `x`. The subject has 9 reachable states, each
with the two successors `upd(a)` and `upd(b)`.

1. Check `always eventually holds(c.versionNumber = 2)` under
   `quire.temporal.infinite-trace/v1`, `over (c: Config::ConfigVersion)`, with
   `fair weak each Config::ConfigVersion::attemptUpdate;`.
2. Check the same clause with `fair weak whole …` and with the unmarked
   `fair …`.
3. Replace the postcondition with the shipped `VersionUnchanged`
   (`self.versionNumber = pre(self.versionNumber)`) and repeat step 1.
4. Check `eventually[0,5] holds(c.versionNumber = 2)` under
   `quire.temporal.event-position.false-extension/v1`, `on origin`, with the
   step 1 postcondition.
5. Check `always[0,5] holds(c.versionNumber <= 2)` under the same profile with
   an exhaustive engine and with an engine that unrolls to depth 5.

## Expected Results

1. `proved`, basis `closed-scope`, FR-331 result `proved`, proof basis
   `exhaustive`, for both bindings: every accepting cycle starves the other
   receiver (FR-363-AC-1, FR-362-AC-5).
2. Both spellings have one obligation identity and settle `refuted` with a
   lasso bound to one object, for example `c = b` with an empty stem and the
   loop `(0,0) -upd(a)-> (1,0) -upd(a)-> (2,0) -upd(a)-> (0,0)`, `trace_position`
   0. The returned counterexample reproduces: its steps re-execute, its loop
   closes, it is fair under `whole`, and the formula is false at
   `trace_position` (FR-364-AC-3). The same lasso replayed under the step 1
   premise refuses as an unfair lasso.
3. `refuted` with a lasso whose loop is
   `(0,0) -upd(a)-> (0,0) -upd(b)-> (0,0)` or another valid one: fairness makes
   every identity run and the postcondition prevents progress.
4. `refuted` with a six-position finite prefix on which the bound object's
   version is never 2, such as five `upd(a)` steps for `c = b`, covering the
   horizon 5 (FR-363-AC-6).
5. The exhaustive engine settles `proved`, proof basis `exhaustive`; the
   unrolling engine settles `proved`, proof basis `bounded-complete`, depth 5
   (FR-363-AC-6).

A vector checks that each returned counterexample reproduces with the expected
shape and binding; it does not require the exact lasso shown.
