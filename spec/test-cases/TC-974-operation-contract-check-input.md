---
id: TC-974
title: "Operation-contract check input: invariant, clause, frame, relation and simulation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-485
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-353
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-397
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-468
    type: verifies
---

# TC-974: Operation-contract check input: invariant, clause, frame, relation and simulation

## Description

Check the operation-contract forms: an invariant with initial-state and preservation obligations, a pre or postcondition clause with and without an implementation binding, a frame item, a bound step relation and a per-step simulation item.

## Test Procedure

Read `proposals/backend-check-arm-v1/vectors.json`. For each case, derive each `$node:<label>` NodeId from its label as the file's `rekey` member states, recompute the FR-290 disposition from the case's registry and item, validate the case's outcome stage against `proposals/backend-provider-v1/schema.json`, and compare the arm outcome with the result. A refutation is checked by replaying its counterexample through the native evaluator, never by comparing it with one expected counterexample. The harness is `tests/backend_check_arm_v1.rs`.

The obligation-level checks of FR-485-AC-2 to FR-485-AC-9 run through the real local Rust boundary in the consumer's own test; this repository qualifies the contract through the vectors.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| OC-01 | invariant violated by a preserving operation: kind `operation-contract`, form `invariant`, extent bounded | `supported`; arm outcome `violation`, result `refuted` |
| OC-02 | invariant holds initially and is preserved: kind `operation-contract`, form `invariant`, extent bounded | `supported`; arm outcome `supported`, result `proved` |
| OC-03 | frame item with no implementation binding: kind `operation-contract`, form `frame`, extent bounded | `supported`; arm outcome `unsupported`, result `unsupported` |
| OC-04 | bound item reads an element with no abstraction binding: kind `operation-contract`, form `clause`, extent bounded | `supported`; arm outcome none (declined), result `declined` |
| OC-05 | unbounded simulation item meets a bounded-only candidate: kind `operation-contract`, form `simulation`, extent unbounded | `requires-bound`; no result entry |
| OC-06 | simulation step with a cardinality bound: kind `operation-contract`, form `simulation`, extent bounded | `supported`; arm outcome `violation`, result `refuted` |
| OC-07 | step relation over invariants no state satisfies: kind `operation-contract`, form `step-relation`, extent bounded | `supported`; arm outcome `unknown`, result `inconclusive` |
| OC-08 | determinism relation refuted, no reachability prefix: kind `operation-contract`, form `step-relation`, extent bounded | `supported`; arm outcome `violation`, result `refuted` |
| OC-09 | candidate counterexample does not replay: kind `operation-contract`, form `clause`, extent bounded | `supported`; arm outcome `unknown`, result `inconclusive` |
| OC-10 | postcondition undefined without a binding: kind `operation-contract`, form `clause`, extent bounded | `supported`; arm outcome `violation`, result `refuted` |
| OC-11 | no provider advertises operation-contract: kind `operation-contract`, form `invariant`, extent bounded | `unsupported`, `unsupported-requested-capability`; no result entry |

## Expected Results

Each form settles per FR-485: an invariant is one item with both obligations and a preservation violation names the operation; an unbound frame is `unsupported` `no-contract-level-reading`; an unbound element of a bound item declines with `missing_declaration`/`missing-name`; a step relation's counterexample carries two executions and no reachability prefix; a simulation violation is `refuted`, never `inconclusive`; an unreplayable candidate is `inconclusive` `replay-parity`.
