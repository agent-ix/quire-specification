---
id: TC-401
title: "Pre-call precondition observation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-046
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-208
    type: verifies
---

# TC-401: Pre-call precondition observation

## Description

Show that a precondition is evaluated over a pre-call observation (the pre
snapshot, the receiver and the parameters) with no post snapshot, and that
its verdict does not depend on any post-side content of an invocation.

## Test Procedure

Reuse the pre-version=1 operation of TC-046, with the precondition
`self.version = 1 and delta > 0` and a parameter `delta`, and a state-machine
operation `advance(step: Integer)` with the precondition `self.count < 10`.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| Q01 | pre-call observation: pre snapshot with version 1, `self`, `delta = 2`, no post snapshot | completed `true`; no post read, delta or frame check |
| Q02 | Q01's pre snapshot, `self` and `delta` as an invocation whose post snapshot changes a field outside the selected frame | completed `true`, equal to Q01 |
| Q03 | Q01 with `delta = 0` | completed `false` |
| Q04 | Q01 with the snapshot marked `post` | refused before evaluation (`wrong_snapshot`/`wrong-observation`) |
| Q05 | Q01 with `delta` omitted | refused before evaluation (`invalid_runtime_input`/`missing-member`) |
| Q06 | Q01 with the pre snapshot absent | unavailable/incomplete, no Boolean |
| Q07 | `advance` precondition over a pre-call observation whose pre snapshot holds one machine context with `count = 3`, `step = 1` | completed `true` |
| Q08 | Q07 with no context of the machine in the pre snapshot | `refused { code: invalid_model_binding, cause: wrong-context }` |

## Expected Results

A precondition reads only its pre-call observation, its verdict is the same
whether that observation is supplied alone or inside an invocation, and each
adverse input returns its named disposition with no Boolean.
