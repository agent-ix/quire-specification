---
id: TC-294
title: "Verdict strength, bound reached, undecided and stopped runs"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-363
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-361
    type: verifies
---

# TC-294: Verdict strength, bound reached, undecided and stopped runs

## Description

The non-proof rows of the verdict table, each with its exact label, basis,
FR-331 result and cause.

## Test Procedure

Over the TC-293 ConfigVersion subject and the step 2 claim (`fair weak whole`):

1. Run a bounded search with horizon `max_depth` 2, then with horizon 3.
2. Run with `max_states` 5; run with a caller cancellation after the first
   level.
3. Give `attemptUpdate` a precondition that reads an observation the subject's
   binding marks unavailable, so the clause evaluator returns an incomplete
   outcome for it (neither `true` nor `false`), and run unbounded.
4. Run the subject with an empty list of initial states.
5. Run a subject whose population `config_history` has no universe.
6. Check the claim under `quire.temporal.timestamped-event.finite-window/v1`.
7. Check a claim `over (c: Config::ConfigVersion)` against a subject whose
   population `config_history` has an empty universe list, so the population is
   bounded and holds no object; then check the same claim against a subject
   whose `config_history` has no universe.

## Expected Results

1. Horizon 2 completes every depth up to 2 and settles `inconclusive`,
   basis `unsettled`, FR-331 result `inconclusive`, cause `bound-reached`,
   depth 2: the shortest counterexample has length 3. Horizon 3 settles
   `refuted` (FR-363-AC-3).
2. Both settle FR-331 `incomplete`, FR-360 `failed`, execution
   `resource-incomplete`, basis `unavailable`; the first names `max_states`, its
   value 5 and the request member that raises it, the second names the
   cancellation (FR-363-AC-4).
3. `inconclusive`, cause `undecided-successor` (FR-363-AC-5).
4. `inconclusive`, cause `no-initial-state` (FR-361-AC-7).
5. `requires-bound` before expansion (FR-361-AC-7).
6. `unsupported`, `unsupported-requested-capability`, basis `unavailable`.
7. `inconclusive`, basis `unsettled`, cause `no-instance`, before any state is
   explored (FR-361-AC-8); the second subject returns `requires-bound` before
   instantiation, never `no-instance` (FR-361-AC-8).

No row carries a proof basis, and each row's label, basis and result equal the
FR-363 verdict table (FR-363-AC-1).
