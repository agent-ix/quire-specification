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

Read `proposals/backend-check-arm-v1/vectors.json`. For each case, derive each `$node:<label>` NodeId from its label as the file's `rekey` member states, recompute the FR-290 disposition from the case's registry and item, validate the case's outcome stage against `proposals/backend-provider-v1/schema.json`, apply the arm's own refusal gate, and compare the arm outcome with the result. The harness is `tests/backend_check_arm_v1.rs`; its pinned case list fails when a vector is removed.

The harness checks the shape of every `contract-counterexample` against the schema and FR-485: `bound` mirrors the item, `concrete` is present for a bound item and absent for an unbound one, an initial-state violation has no operation, a relation violation has two executions, a simulation violation's obligation and locus ordinal follow ordinal 0 = initial rule and k + 1 = step row k, and a frame violation hidden in a concrete location leaves the images equal and the concrete values different. It also checks the arm's refusal gate (no form, missing node, unbound element, no implementation engine, unbound frame).

The provider's own tests SHALL cover the criteria that need a real contract evaluator, through the real local Rust boundary: FR-485-AC-2 and FR-485-AC-3 (invariant initial-state and preservation over declared types), FR-485-AC-4 (definedness of an unbound clause), FR-485-AC-5 (a bound postcondition over every implementation outcome), FR-485-AC-6 (a bound frame write outside `modifies`), FR-485-AC-8 (the step relation and the determinism counterexample), FR-485-AC-10 (replay with the real contract evaluator and the real bound implementation; the harness here replays only the vector models) and FR-485-AC-11 (a violation visible in `concrete` alone). This repository qualifies the contract through the vectors.

## Vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| OC-01 | invariant violated by an operation from an invariant-satisfying pre-state: kind `operation-contract`, form `invariant` | `supported`; arm outcome `violation`, result `refuted` | FR-485-AC-2, FR-485-AC-3 |
| OC-02 | invariant violated by an initial state: kind `operation-contract`, form `invariant` | `supported`; arm outcome `violation`, result `refuted` | FR-485-AC-2 |
| OC-03 | invariant holds initially and is preserved: kind `operation-contract`, form `invariant` | `supported`; arm outcome `supported`, result `proved`, basis `exhaustive`, `uncertified` | FR-485-AC-2 |
| OC-04 | node outside the forms: kind `operation-contract`, the claim selects no form | `supported`; arm outcome none (declined), result `declined`, cause `invalid-input` | FR-485-AC-1, FR-483-AC-5 |
| OC-05 | implication whose claim is not a refinement node: kind `operation-contract`, form `simulation` | `supported`; arm outcome none (declined), result `declined`, cause `invalid-input` | FR-485-AC-1 |
| OC-06 | postcondition without a binding: definedness: kind `operation-contract`, form `clause` | `supported`; arm outcome `violation`, result `refuted`, cause `undefined-evaluation` | FR-485-AC-4 |
| OC-07 | bound postcondition refuted, images and concrete values recorded: kind `operation-contract`, form `clause` | `supported`; arm outcome `violation`, result `refuted` | FR-485-AC-5, FR-485-AC-10, FR-485-AC-11 |
| OC-08 | bound postcondition and a provider with no implementation engine: kind `operation-contract`, form `clause` | `supported`; arm outcome `unsupported`, result `unsupported` | FR-485-AC-5 |
| OC-09 | frame item with no implementation binding: kind `operation-contract`, form `frame` | `supported`; arm outcome `unsupported`, result `unsupported` | FR-485-AC-6 |
| OC-10 | bound frame item: a write outside modifies is visible in the concrete values only: kind `operation-contract`, form `frame` | `supported`; arm outcome `violation`, result `refuted` | FR-485-AC-6, FR-485-AC-11 |
| OC-11 | bound item reads an element with no abstraction binding: kind `operation-contract`, form `clause` | `supported`; arm outcome none (declined), result `declined`, cause `incomplete-input` | FR-485-AC-7 |
| OC-12 | a sibling item with its bindings is unaffected: kind `operation-contract`, form `clause` | `supported`; item 0 operation-contract `declined`, item 1 operation-contract `proved` | FR-485-AC-7 |
| OC-13 | step relation over invariants no state satisfies: kind `operation-contract`, form `step-relation` | `supported`; arm outcome `unknown`, result `inconclusive`, cause `kani_vacuous_proof` | FR-485-AC-8 |
| OC-14 | determinism relation: two executions, no reachability prefix: kind `operation-contract`, form `step-relation` | `supported`; arm outcome `violation`, result `refuted` | FR-485-AC-8, FR-485-AC-10 |
| OC-15 | simulation initial rule is ordinal 0: kind `operation-contract`, form `simulation` | `supported`; arm outcome `violation`, result `refuted` | FR-485-AC-9 |
| OC-16 | simulation step row 1 is ordinal 2, refuted and never inconclusive: kind `operation-contract`, form `simulation` | `supported`; arm outcome `violation`, result `refuted` | FR-485-AC-9 |
| OC-17 | unbounded population root meets a bounded-only candidate: kind `operation-contract`, form `simulation` | `requires-bound`; no result entry | FR-485-AC-12, FR-483-AC-6 |
| OC-18 | the same item with a cardinality bound on the population: kind `operation-contract`, form `simulation` | `supported`; arm outcome `supported`, result `proved`, basis `exhaustive`, `uncertified` | FR-485-AC-12 |
| OC-19 | candidate counterexample does not replay: kind `operation-contract`, form `clause` | `supported`; arm outcome `unknown`, result `inconclusive`, cause `replay-parity` | FR-485-AC-10, FR-483-AC-3 |
| OC-20 | bound candidate whose concrete outcome the implementation does not produce: kind `operation-contract`, form `clause` | `supported`; arm outcome `unknown`, result `inconclusive`, cause `replay-parity` | FR-485-AC-10, FR-485-AC-5, FR-483-AC-3 |
| OC-21 | reader with no implementation engine receives a bound counterexample: kind `operation-contract`, form `clause` | `supported`; arm outcome `unknown`, result `inconclusive`, cause `replay-refused` | FR-485-AC-13, FR-483-AC-3 |
| OC-22 | the same bound counterexample with an implementation engine is replayed and reproduced: kind `operation-contract`, form `clause` | `supported`; arm outcome `violation`, result `refuted` | FR-485-AC-13, FR-485-AC-10, FR-483-AC-3 |

## Expected Results

Each operation-contract vector settles as its row states. A bound counterexample records images and concrete values, and a replay that re-executes the implementation and does not reproduce the recorded outcome or image settles `inconclusive` `replay-parity`. A claim node outside the five forms, an implication whose claim is not a refinement node, and a bound item whose element has no abstraction binding are declined; the sibling item with its bindings is unaffected. Simulation ordinal 0 refutes with `simulation-initial` and ordinal 2 (step row 1) with `simulation-step`, never `inconclusive`. An operation over an unbounded population is `requires-bound` on a bounded-only candidate and `supported` with a cardinality bound.

The harness computes each contract replay from the case's `replay_model`, so `refuted` is reported only for a counterexample it reproduced: an unbound counterexample is evaluated on the recorded values, a bound one re-executes the model implementation on the recorded concrete inputs, requires the recorded concrete outcome to be one the implementation produces, checks the abstraction images, then evaluates the obligation, and a frame obligation compares the concrete locations outside `modifies`. A candidate that does not reproduce settles `inconclusive` `replay-parity` (OC-19, OC-20). A reader with no implementation engine settles a bound counterexample `inconclusive` `replay-refused` and never `refuted` (OC-21); the same candidate with an implementation engine is replayed and settles by the computed replay, here `refuted` (OC-22). The harness asserts the recorded cause in both directions: every `replay-refused` is a computed refusal, every `replay-parity` a computed non-reproduction, and every operation-contract case that records or receives a counterexample carries a `replay_model`.
