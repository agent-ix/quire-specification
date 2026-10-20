---
id: TC-387
title: "Protocol counterexample wire and replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-433
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-426
    type: verifies
---

# TC-387: Protocol counterexample wire and replay

## Description

Check the protocol counterexample's members and its replay outcomes,
including each refusal and each `replay-parity` case.

## Test Procedure

Take the counterexamples the vectors name, serialize each, apply the stated
mutation, and replay it through the real local Rust boundary, recording the
refusal or settlement.

## Vectors

| Vector | Counterexample and mutation | Expected result |
| --- | --- | --- |
| RP-01 | `Fill` deadlock (TC-376 FL-03), unmodified | members `kind: deadlock`, `subject`, `initial_state`, `prefix` of two steps (`fork(Both)`, `attempt(A)`) each with a post-state digest, empty `loop`, `blocked`; replays to `reproduced-with-evaluated-witness` |
| RP-02 | RP-01 truncated after step 1 | the prefix ends at s1, which enables both attempts: settles `inconclusive`, cause `replay-parity` |
| RP-03 | RP-01 with step 1 removed | `attempt(A)` is not enabled at s0: refuses `invalid_runtime_input`/`invalid-value` |
| RP-04 | RP-01 with step 2's post-state digest altered | refuses `stale_dependency`/`revision-mismatch` |
| RP-05 | RP-01 with `blocked` naming `left` | settles `inconclusive`, cause `replay-parity` |
| RP-06 | RP-01 with `subject` removed, with an extra member, and with a transition identity missing `node` | each refuses before replay |
| RP-07 | the `Fill` lasso of TC-376 FL-04 | replays: the loop is s2's terminal stutter step, and the formula is false at `trace_position` |
| RP-08 | TC-384 SF-02 lasso, unmodified, then replayed against the SF-01 subject | replays; then refuses as unfair |
| RP-09 | every refutation of TC-376 to TC-386 | each replays to `reproduced-with-evaluated-witness` |
| RP-10 | A counterexample through a `send` at a full `loss head` channel, unmodified; then with its `send_record` `displaced` changed, with the `send_record` removed, and with a `send_record` added to an `attempt` step | The unmodified one replays; each of the three modified ones refuses `stale_dependency`/`revision-mismatch` |
| RP-10 | TC-380 CP-09 counterexample, unmodified, then with its last step replaced so that the recovery predicate's read is defined | replays to `reproduced-with-evaluated-witness` and the item `refuted`, cause `undefined-evaluation`; then settles `inconclusive`, cause `replay-parity` |

## Expected Results

Every replay outcome matches the table; replay reads no engine state.
