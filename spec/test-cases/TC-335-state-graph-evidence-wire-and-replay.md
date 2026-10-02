---
id: TC-335
title: "State-graph evidence wire and replay disagreements"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-393
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-394
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-181
    type: verifies
---

# TC-335: State-graph evidence wire and replay disagreements

## Description

Serialize each evidence arm, refuse malformed evidence before replay, and replay agreeing and disagreeing evidence, including exploration from a recorded state.

## Test Procedure

Encode each vector with RFC 8785 JCS through the real local Rust boundary, read it back, and submit it to the replay boundary with the subject and claim named. Record the refusal, or the replay result and the item's settlement.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| GE-01 | a `witness`, a `trap` and a `path-pair` value from TC-331 to TC-333 | byte-identical round trip; each replays and reproduces |
| GE-02 | a witness whose last node fails `P` | `inconclusive`, `replay-parity` |
| GE-03 | a trap whose closure, explored afresh, reaches a target node | `inconclusive`, `replay-parity` |
| GE-04 | a path pair whose two sequences are equal | `inconclusive`, `replay-parity` |
| GE-05 | a path whose step's transition is not enabled, and one whose post-state digest matches no successor | `inconclusive`, `replay-parity` |
| GE-06 | the same witness steps with sources `sampled`, `explored` and `unrolled` | identical replay with no sampler or engine present |
| GE-07 | a trap whose closure exploration exceeds `max_states` | `incomplete`, cause `limit-reached` naming `max_states`, its value and its setting |
| GE-12 | GE-07's trap with the caller cancelling the closure exploration before it completes | `incomplete`, cause `cancelled` |
| GE-08 | an unknown arm; a duplicate key; a witness with one entry missing; entries out of order; a mismatched instance; a `loop_entry` on a witness of a `possible` claim; a `loop_entry` greater than its path's step count on a single-existential witness; an `undefined` arm with no `undefined` member or with an uncatalogued `cause`; an `undefined` member on a `trap` arm | each refused `invalid_runtime_input`/`invalid-value` before replay |
| GE-09 | equal evidence in envelopes for two different claims | different obligation identities |
| GE-10 | FR-181 exhaustive exploration from the recorded state `Lost` of TC-332 | the only initial state is `Lost`; closure `{Lost}`; every explored edge retained with each state's open or closed status |
| GE-11 | an `undefined` arm from TC-349 serialized and read back, then with its carried locus changed | byte-identical round trip carrying the `undefined-evaluation` value; with the changed locus, replay settles `inconclusive`, `replay-parity` |

## Expected Results

Replay settles each item only from its own re-execution and re-exploration, and every disagreement or malformed value is reported with its cause.
