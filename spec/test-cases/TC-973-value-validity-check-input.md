---
id: TC-973
title: "Value-validity check input: clause, case and scalar operation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-484
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
---

# TC-973: Value-validity check input: clause, case and scalar operation

## Description

Check the value-validity forms: a Boolean clause with a function application, a clause with a `case` over a union, and a scalar operation occurrence with its path condition.

## Test Procedure

Read `proposals/backend-check-arm-v1/vectors.json`. For each case, derive each `$node:<label>` NodeId from its label as the file's `rekey` member states, recompute the FR-290 disposition from the case's registry and item, validate the case's outcome stage against `proposals/backend-provider-v1/schema.json`, and compare the arm outcome with the result. A refutation is checked by replaying its counterexample through the native evaluator, never by comparing it with one expected counterexample. The harness is `tests/backend_check_arm_v1.rs`.

The scalar-operation, function-application and case checks of FR-484-AC-1 to FR-484-AC-9 are run through the real local Rust boundary by the consumer's own test; this repository qualifies the contract through the vectors.

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

## Expected Results

Each form settles per FR-484. The following are checked in the evaluator beside the vectors: `x + 1 < 256` over `x: Int[0, 255]` is refuted at 255; `double(x) >= x` over `x: Int[-3, 3]` is refuted at -1 because the call is evaluated as its body; of two `a / b` occurrences the one under `if b != 0` is proved and the unguarded one is refuted at `b = 0`; the right operand of `b != 0 and a / b > 0` carries `b != 0`; a range narrowing, `if`, `let` and connective claim is declined; a `case` is proved only after every member is covered.
