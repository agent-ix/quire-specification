---
id: TC-345
title: "Hyper counterexample wire and replay refusals"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-400
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-401
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-399
    type: verifies
---

# TC-345: Hyper counterexample wire and replay refusals

## Description

Serialize each counterexample kind, refuse malformed counterexamples before replay, and refuse or disagree at replay as the rules require.

## Test Procedure

Encode each vector with RFC 8785 JCS through the real local Rust boundary, read it back and submit it to the replay boundary with its subjects and clause; record the refusal or the replay result and settlement.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| HR-01 | one counterexample of each kind from TC-338, TC-339, TC-343 and TC-344 | byte-identical round trip; each reproduces |
| HR-02 | a `lockstep` counterexample whose traces have unequal step counts | refused `invalid_runtime_input`/`invalid-value` before replay |
| HR-03 | a `lockstep` counterexample with `μ_U` false at one joint step | refused `invalid_runtime_input`/`invalid-value`; item `inconclusive`, `replay-refused` |
| HR-04 | a `lockstep` counterexample with an unfair component on its loop | refused `invalid_runtime_input`/`invalid-value`; item `inconclusive`, `replay-refused` |
| HR-05 | a `witness-exhausted` counterexample whose recomputed `X_position` is non-empty | `inconclusive`, `replay-parity` |
| HR-06 | a `witness-exhausted` counterexample whose recomputation reaches `max_witness_set` | `incomplete`, cause `limit-reached` naming `max_witness_set`, its value and the setting `model_check.max_witness_set` |
| HR-07 | a `projected` step with no mark; a `mark` on a `lockstep` step; a terminal-stutter step followed by a non-stutter step; a terminal-stutter step in a `step-tuple` trace; an `initial` out of range; traces in the wrong variable order | each refused `invalid_runtime_input`/`invalid-value` before replay |
| HR-08 | a `lockstep` counterexample whose body evaluates true over the tuple | `inconclusive`, `replay-parity` |
| HR-09 | a `lockstep` counterexample with `loop_entry` beyond its step count; a `witness-exhausted` counterexample with `position` beyond its step count; a `loop_entry` on a `step-tuple` counterexample | each refused `invalid_runtime_input`/`invalid-value` before replay |
| HR-10 | a well-formed `lockstep` counterexample whose terminal-stutter step follows a state at which `step` is enabled | `inconclusive`, `replay-parity` |
| HR-11 | an `undefined` counterexample with no `undefined` member, with `state_keys` of the wrong length, or with an uncatalogued `cause` | each refused `invalid_runtime_input`/`invalid-value` before replay |
| HR-12 | an `undefined` counterexample whose paths differ in step count: that of an `align skip` clause (TC-348 UE-13), whose unmarked paths each hold a skipped step and a visible step, and that of an HP-3 undefined `μ_U` (TC-348 UE-12) | refused `unsupported_construct`/`not-yet-implemented`; no result is settled and it is not replayed as a lockstep counterexample |

## Expected Results

Malformed counterexamples never reach replay, and replay settles an item `refuted` only when it reproduces the violation itself.
