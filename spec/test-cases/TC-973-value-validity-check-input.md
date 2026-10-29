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

Read `proposals/backend-check-arm-v1/vectors.json`. For each case, derive each `$node:<label>` NodeId from its label as the file's `rekey` member states, recompute the FR-290 disposition from the case's registry and item, validate the case's outcome stage against `proposals/backend-provider-v1/schema.json`, apply the arm's own refusal gate, and compare the arm outcome with the result. The harness is `tests/backend_check_arm_v1.rs`; its pinned case list fails when a vector is removed.

Each case with a `model` is evaluated by the harness's own native evaluator under the mathematical-integer semantics of FR-484: the harness enumerates the declared domain, recomputes `proved`, `refuted`, vacuous, not-evaluable or `incomplete`, and replays the recorded counterexample, which must reproduce the violation. A refutation is checked by replaying its counterexample, never by comparing it with one expected counterexample.

The provider's own tests SHALL cover the criteria the harness cannot evaluate: FR-484-AC-3 (a `case` over a union of three members, proved only after every member and payload domain is covered, and a counterexample whose union value names the member and payload) through the real local Rust boundary. This repository qualifies the contract through the vectors; the provider qualifies its evaluator.

## Vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| VV-01 | x + 1 > x over Int[0,255] holds: kind `value-validity`, form `clause` | `supported`; arm outcome `supported`, result `proved`, basis `exhaustive`, `uncertified` | FR-484-AC-1, FR-483-AC-1, FR-483-AC-2 |
| VV-02 | x + 1 < 256 over Int[0,255] fails at 255: kind `value-validity`, form `clause` | `supported`; arm outcome `violation`, result `refuted` | FR-484-AC-1, FR-483-AC-3 |
| VV-03 | function application evaluates the callee body: kind `value-validity`, form `clause` | `supported`; arm outcome `violation`, result `refuted` | FR-484-AC-2 |
| VV-04 | guarded division occurrence holds: kind `value-validity`, form `scalar-operation` | `supported`; arm outcome `supported`, result `proved`, basis `exhaustive`, `uncertified` | FR-484-AC-4 |
| VV-05 | unguarded division occurrence is undefined at b = 0: kind `value-validity`, form `scalar-operation` | `supported`; arm outcome `violation`, result `refuted`, cause `undefined-evaluation` | FR-484-AC-4 |
| VV-06 | right operand of and carries the left operand: kind `value-validity`, form `scalar-operation` | `supported`; arm outcome `supported`, result `proved`, basis `exhaustive`, `uncertified` | FR-484-AC-5 |
| VV-07 | claim node is a range narrowing, not a form: kind `value-validity`, form `range-narrowing` | `supported`; arm outcome none (declined), result `declined`, cause `invalid-input` | FR-484-AC-5, FR-483-AC-5 |
| VV-08 | operand of a range narrowing is checked against the target range: kind `value-validity`, form `scalar-operation` | `supported`; arm outcome `violation`, result `refuted`, cause `undefined-evaluation` | FR-484-AC-6 |
| VV-09 | undefined subexpression of a clause: kind `value-validity`, form `clause` | `supported`; arm outcome `violation`, result `refuted`, cause `undefined-evaluation` | FR-484-AC-7 |
| VV-10 | path condition no assignment satisfies is vacuous: kind `value-validity`, form `scalar-operation` | `supported`; arm outcome `unknown`, result `inconclusive`, cause `kani_vacuous_proof` | FR-484-AC-7, FR-483-AC-2 |
| VV-11 | callee body absent: call-not-evaluable: kind `value-validity`, form `clause` | `supported`; arm outcome `unsupported`, result `unsupported` | FR-484-AC-9 |
| VV-12 | recursive evaluation stops at max_cases: incomplete, not unsupported: kind `value-validity`, form `clause` | `supported`; arm outcome `unknown`, result `incomplete`, cause `limit-reached` | FR-484-AC-9, FR-483-AC-4 |
| VV-13 | unbounded Integer parameters: requires-bound with a bound available: kind `value-validity`, form `clause` | `requires-bound`; no result entry | FR-484-AC-8, FR-483-AC-6 |
| VV-14 | the same claim with an integer-range bound is bounded: kind `value-validity`, form `clause` | `supported`; arm outcome `supported`, result `proved`, basis `exhaustive`, `uncertified` | FR-484-AC-8 |
| VV-15 | clause quantifying over an unbounded population adds a population domain: kind `value-validity`, form `clause` | `unsupported`, `unsupported-requested-capability`; no result entry | FR-484-AC-8, FR-483-AC-8 |
| VV-16 | case over a union reports a union value in its counterexample: kind `value-validity`, form `case-clause` | `supported`; arm outcome `violation`, result `refuted` | FR-484-AC-3 |

## Expected Results

Each value-validity vector settles as its row states. `x + 1 > x` over `x: Int[0, 255]` is proved and `x + 1 < 256` is refuted at 255. `double(x) >= x` over `x: Int[-3, 3]` is refuted because the call is evaluated as its body. Of two `a / b` occurrences the one under `if b != 0` is proved and the unguarded one is refuted at `b = 0`, with `undefined-evaluation` at its own occurrence. The right operand of `b != 0 and a / b > 0` carries `b != 0`. A scalar operation under a range narrowing to `Int[0, 5]` is refuted at `a = 7`. A path condition no assignment satisfies is `inconclusive` `kani_vacuous_proof`. A call with no body is `unsupported` `call-not-evaluable`, while a call stopped by `max_cases` is `incomplete`. A range narrowing, `if`, `let` or Boolean connective claim is declined `invalid-input`.
