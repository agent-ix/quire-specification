---
id: TC-972
title: "Check arm: outcome mapping, evidence, refusals and registration"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-483
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-972: Check arm: outcome mapping, evidence, refusals and registration

## Description

Check the shared check-arm contract over both kinds: negotiation dispositions, the four arm outcomes and their FR-331 results, proof evidence, counterexample kinds, refusals, limits and registration.

## Test Procedure

Read `proposals/backend-check-arm-v1/vectors.json`. For each case, derive each `$node:<label>` NodeId from its label as the file's `rekey` member states, recompute the FR-290 disposition from the case's registry and item, validate the case's outcome stage against `proposals/backend-provider-v1/schema.json`, and compare the arm outcome with the result. A refutation is checked by replaying its counterexample through the native evaluator, never by comparing it with one expected counterexample. The harness is `tests/backend_check_arm_v1.rs`.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| VV-01 | clause over a finite domain holds: kind `value-validity`, form `clause`, extent bounded | `supported`; arm outcome `supported`, result `proved` |
| VV-02 | clause violated at the domain edge: kind `value-validity`, form `clause`, extent bounded | `supported`; arm outcome `violation`, result `refuted` |
| VV-03 | unguarded scalar operation is undefined: kind `value-validity`, form `scalar-operation`, extent bounded | `supported`; arm outcome `violation`, result `refuted` |
| VV-04 | path condition no assignment satisfies: kind `value-validity`, form `scalar-operation`, extent bounded | `supported`; arm outcome `unknown`, result `inconclusive` |
| VV-05 | unbounded item, bounded-only candidate, bound available: kind `value-validity`, form `clause`, extent unbounded | `requires-bound`; no result entry |
| VV-06 | unbounded item, bounded-only candidate, no bound available: kind `value-validity`, form `clause`, extent unbounded | `unsupported`, `unbounded-extent`; no result entry |
| VV-07 | unbounded item, unbounded candidate: kind `value-validity`, form `clause`, extent unbounded | `supported`; arm outcome `supported`, result `proved` |
| VV-08 | no provider advertises value-validity: kind `value-validity`, form `clause`, extent bounded | `unsupported`, `unsupported-requested-capability`; no result entry |
| VV-09 | case-clause over a union, evaluation reaches max_cases: kind `value-validity`, form `case-clause`, extent bounded | `supported`; arm outcome `unknown`, result `incomplete` |
| VV-10 | claim node is a range narrowing: kind `value-validity`, form `clause`, extent bounded | `supported`; arm outcome none (declined), result `declined` |
| VV-11 | domain kind the candidate cannot bound: kind `value-validity`, form `clause`, extent bounded | `unsupported`, `unsupported-requested-capability`; no result entry |
| VV-12 | unknown from the engine: kind `value-validity`, form `clause`, extent unbounded | `supported`; arm outcome `unknown`, result `inconclusive` |
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

Every case's disposition equals the FR-290 recomputation; every outcome stage is admitted by the schema; every arm outcome maps to its table result; `refuted` carries the kind's counterexample type; `proved` by enumeration is `uncertified`; a refused item carries a disposition and no result; a value-validity descriptor lists no `population`; no vector holds a digest.
