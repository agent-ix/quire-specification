---
id: TC-318
title: "Protocol refinement over the model-refinement step rules"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-177
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-379
    type: verifies
---

# TC-318: Protocol refinement over the model-refinement step rules

## Description

Decide protocol refinement with FR-377's step function, internal actions as
`stutter` steps, divergence-freedom independent of fairness, and the
refusal-set and terminal-success checks with their typed failures and replay
(FR-379 "Protocol relation failures"). The product tables for cases 4 to 9 are
`proposals/backend-provider-v1/refinement-failure-vectors.json`, run as
`tests/backend_provider_v1.rs` (`tc_318_protocol_relation_failure_vectors`).

## Test Procedure

Specification protocol: one visible step `ack` from `idle` to `done`.
Implementation protocol: internal step `prep` from `idle` to `ready`,
visible `ack` from `ready` to `done`, and internal `spin`, a self-loop at
`ready`, under a profile that makes divergence observable.

1. Check the implementation with `spin` removed.
2. Check it with `spin`, under an empty fairness set and under
   `fair weak whole ack` on the implementation.
3. Encode the same two models as state models with a model refinement:
   `prep -> stutter`, `spin -> stutter`, `ack -> ack(self)`, with `F_A`
   empty, and check.
4. **Refusal not permitted.** Specification: `idle` with visible `ack` to
   `done`. Implementation: internal `prep` from `idle` to `ready`, where `ack`
   is never enabled. Check, then replay.
5. **Refusal permitted.** Specification: `idle` with internal `choose` to `a`
   (visible `ack`) or `b` (no step, intended). Implementation: no step at
   `idle`, intended. Check.
6. **Concrete-only terminal success.** Specification: `idle` with internal
   `choose` to `x` (visible `p`) or `y` (visible `q`). Implementation: no step
   at `idle`, intended. Check, then replay.
7. **Abstract-only terminal success.** Specification as case 4 with `done`
   intended. Implementation: `ack` to `stuck`, no step, not intended. Check.
8. **Matching terminal success.** Case 7's implementation with `stuck`
   intended. Check.
9. **Replay disagreement.** Replay case 4's counterexample with `position` 0,
   with `label` `{step: "other", binder: []}`, and with a one-step loop; replay
   case 6's with `direction` `abstract-only`.

## Expected Results

Case 1 refines: `prep` passes as a `stutter` step and `ack` matches the
specification's `ack` by observation. Case 2 refutes with a divergence
counterexample (the `spin` loop at `ready`, whose specification state can
move) under both fairness sets. Case 3 holds: the model refinement reads the
stutter tail through empty abstract fairness. The same step function and
counterexample type serve all three (FR-177-AC-6).

Case 4 refutes with prefix `prep` and `RefusalNotPermitted{position: 1,
label: {step: "ack", binder: []}}`: at position 0 `prep` is an enabled
`stutter` step, so the state is not stable and is not checked; at position 1
the implementation is stable and refuses `ack`, which the only specification
state `idle` enables; replay reproduces it (FR-379-AC-8, FR-379-AC-13). Case 5
holds: the stable member `b` refuses `ack`, so the implementation's refusal is
permitted, and the implementation's intended terminal state matches terminal
success of `b` (FR-379-AC-9, FR-379-AC-11). Case 6 refutes with the empty
prefix and `TerminalSuccessMismatch{position: 0, direction: concrete-only}`:
`x` and `y` share no enabled label, so no refusal fails first, and no member
is terminal success; replay reproduces it (FR-379-AC-10). Case 7 refutes with
prefix `ack` and `TerminalSuccessMismatch{position: 1, direction:
abstract-only}` (FR-379-AC-10). Case 8 holds (FR-379-AC-11). In case 9 the
`position` 0 and the `other` label each settle `inconclusive`, cause
`replay-parity`, the loop is refused `invalid_runtime_input`/`invalid-value`,
and the wrong `direction` settles `replay-parity` (FR-379-AC-13). A step rule
failing on the incoming step is reported before these kinds (FR-379-AC-12).
