---
id: FR-484
title: "Check a value-validity item: clause, case and scalar operation"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-483
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-453
    type: references
---

# FR-484: Check a value-validity item: clause, case and scalar operation

## Description

When a `value-validity` item reaches a check arm, the arm SHALL decide, from the check input it derives from the claim form that [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)'s claim-form table assigns to the claim, whether the goal holds, and is defined, for every assignment of the claim's variables in their domains under the claim's path condition, settling it by [FR-483](FR-483-settle-backend-items-at-the-check-arm.md).

## Inputs

A `value-validity` item settled `supported`, the checked package, and the claim's node and occurrence.

## Outputs

A check input of one of three forms, and the settlement FR-483 states.

## Behavior

### Forms

The claim's node and occurrence select exactly one form:

| Form | Claim node and occurrence | FR-290 row |
| --- | --- | --- |
| `clause` | a Boolean `expression` node, occurrence role `claim` or `declaration`, whose body holds no `case` over a sum type | Boolean clause over pure expressions, including function application |
| `case-clause` | a Boolean `expression` node of the same occurrences whose body holds one or more `case` expressions over a declared `union` | Clause containing a `case` expression over a sum type |
| `scalar-operation` | the application node of a scalar operation, with occurrence role `expression` and the ordinal of its occurrence in the body of a function declaration | Scalar operation application in a function body (FR-146) |

A claim whose node is a range narrowing, a function declaration, an `if`, a `let`, a Boolean connective or any application that FR-290 does not list is not a form, and the arm settles it `declined` `invalid-input` (FR-483).

### Variables and domains

The `variables` of a check input are the free `value`/`parameter` nodes the goal reads, in binder-level order: for a `clause` or `case-clause`, the parameters of the clause's declaration; for a `scalar-operation`, the parameters of the function whose body holds the occurrence. Each domain is the parameter's declared type with its inclusive bounds ([FR-196](FR-196-generate-kani-contracts.md) "Kani mapping contract"), restricted by the item's bounds as FR-483 states. A parameter of union type ranges over every member and, for each member, every value of its payload types.

### Clause and function application

The goal is the clause's Boolean node. A function application inside the goal is evaluated as the callee's declared body, with the arguments bound to its parameters in order; the arm SHALL NOT treat a call as an unconstrained value, and a call is *not evaluable* exactly when the callee's declaration body is in neither the checked package nor the dependency closure the arm was given (a dependency export that carries its signature only, or a reserved intrinsic the arm does not implement), and then the arm settles the item `unsupported` with reason `call-not-evaluable`. A call whose evaluation the arm starts and cannot finish within `max_cases` is `incomplete`, never `unsupported`. A recursive call is evaluated under the callee's declared measure ([FR-146](../expressions/FR-146-check-total-pure-functions.md)); an evaluation that exhausts the arm's `max_cases` is `incomplete` `limit-reached`, not a violation.

The premises are empty. Arithmetic in the clause has the semantics of its operation family: `quire.op.integer` operations are exact mathematical-integer operations ([FR-147](../expressions/FR-147-evaluate-integer-division-domains.md)), so `x + 1` over `x: Int[0, 255]` is the integer `x + 1` with no narrowing to the type of `x`, and a clause is false or undefined where the mathematics says so. Hence `x + 1 > x` holds for every `x` in `[0, 255]` and `x + 1 < 256` is false at `x = 255`. A value leaves its declared type only through a range narrowing, which FR-290 assigns to the application it narrows. The goal is *defined* when every subexpression evaluates to a value; the arm treats an undefined subexpression as a violation, `refuted` with `undefined-evaluation{where, cause}`, `where.locus` the `{node, occurrence}` of the innermost undefined subexpression, which lies in the callee's body when the undefinedness is inside a function application, and `cause` the catalogued reason the native evaluator reports for it. The call site is not a second locus; the replay evaluates the same call and reaches the same innermost subexpression.

### Case over a sum type

For a `case-clause` the arm evaluates each `case` as [FR-146](../expressions/FR-146-check-total-pure-functions.md) "Case exhaustiveness" states: the scrutinee first, an undefined scrutinee being the `case`'s outcome and no arm being evaluated; otherwise the arm of the scrutinee's active member, its payload bound positionally, its body being the `case`'s outcome. The arm SHALL cover every member of the union at every `case`, so a `proved` states the goal under each member and its payload domain. Exhaustiveness is an admission obligation, not an item ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)). A package admitted through [FR-146](../expressions/FR-146-check-total-pure-functions.md) holds no `case` with an uncovered member; a typed in-process package that holds one was not so admitted, and the arm settles the item `declined` `invalid-input` with the FR-146 refusal as its diagnostic, `undefined_expression`/`unproved-exhaustiveness`, naming the union.

A counterexample's value for a union-typed variable is its [FR-181](../runtime/FR-181-simulate-finite-executions.md) `union` value naming the active member and its payload.

### Scalar operation

A `scalar-operation` item is one claim for one occurrence; two occurrences of one application in a body are two items. The check input is:

- `variables` the function's parameters;
- `premises` the occurrence's *path condition*, the conjunction, in nesting order, of the guards the occurrence stands under: for an enclosing `if`, its condition in the `then` branch and its negation in the `else` branch; and for the right operand of an enclosing `and`, `or` or `implies`, its left operand, as FR-146's propagation rule carries guard facts;
- `goal` the *definedness-and-bound* obligation: the application is defined at the arguments its operands evaluate to, and its result lies in its result bound. The result bound is the target range of the range narrowing whose operand the application is, and otherwise the application's own result type ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)).

The occurrence ordinal is the ordinal of the occurrence's [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) `source_map` key (`node_id`, `expression`, `ordinal`): two source occurrences of one application, in one function or in two, share the application node and have distinct ordinals, and so are two items. The arm locates the occurrence in a function body by the occurrence's first source region, which lies inside the body region of exactly one function declaration, and the path condition is the guards on the path from the body root to that region. An occurrence in no function body, including one in a termination measure, is not a form and settles `declined` `invalid-input`.

Every violation of a scalar-operation goal is an undefined evaluation: the operation is undefined at its arguments, or its result lies outside the bound. The result is `refuted` with `undefined-evaluation{where, cause}`, `where.locus` the occurrence, `cause` the catalogued reason of the operation's family (FR-044, FR-140, FR-141, FR-142, FR-147, FR-148, FR-149). A path condition that no assignment satisfies makes the premises unsatisfied, and FR-483 settles it `inconclusive` `kani_vacuous_proof`.

The claim is requested beside the function's [FR-146](../expressions/FR-146-check-total-pure-functions.md) admission. The arm's `proved` or `refuted` discharges nothing FR-146 requires and replaces no FR-146 refusal.

### Extent

The roots of a `value-validity` item are its variables' declared types and each population its clause ranges over by a quantifier or `allInstances`, which is one `population` domain boundable by `cardinality` when the population has no maximum. A type with no inclusive maximum, such as the mathematical `Integer` (FR-147), a collection without a maximum cardinality or a recursive type without a depth maximum, is an unbounded `integer`, `collection` or `recursive` domain; a type with finite inclusive bounds adds no domain. An item with no unbounded root is `bounded`.

### Kani mapping

A Kani arm maps a `value-validity` item to one harness ([FR-196](FR-196-generate-kani-contracts.md) "Kani mapping contract"): each variable is an unconstrained value of its declared type with the inclusive bounds as assumptions, the premises are `kani::assume`d, and the goal, with its definedness checks, is asserted. The `checks` of `bounded-proof{checks}` is that harness's SUCCESS check count.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-484-AC-1 | A Boolean clause `x + 1 > x` over `x: Int[0, 255]`, requested `bounded`, settles `proved`, `exhaustive`, `uncertified`; the same clause over `x: Int[0, 255]` with the goal `x + 1 < 256` settles `refuted` with the assignment `x = 255`, replayed. | Test (TC-973) |
| FR-484-AC-2 | A clause that applies a function evaluates the callee's declared body: with `double(n) = n + n`, the clause `double(x) >= x` over `x: Int[-3, 3]` settles `refuted` with a violating assignment such as `x = -1`, never `proved` as an unconstrained call would. | Test (TC-973) |
| FR-484-AC-3 | A clause with a `case` over a union of three members settles `proved` only after every member and payload domain is covered; a violation under one member gives a counterexample whose union value names that member and payload. | Test (TC-973) |
| FR-484-AC-4 | A function body with two occurrences of `a / b` is two items; the occurrence under `if b != 0` has the premise `b != 0` and settles `proved`, and the unguarded occurrence settles `refuted` with `undefined-evaluation` at its own occurrence with `b = 0`. | Test (TC-973) |
| FR-484-AC-5 | The right operand of `b != 0 and a / b > 0` carries the premise `b != 0`; a range narrowing, an `if`, a `let`, a Boolean connective and an occurrence in a termination measure are not forms and settle `declined` `invalid-input`. | Test (TC-973) |
| FR-484-AC-6 | A scalar operation that is the operand of a range narrowing to `Int[0, 5]`, over `a: Int[0, 9]`, is checked against `Int[0, 5]`, not its own result type; `a = 7` is `refuted` with `undefined-evaluation`. | Test (TC-973) |
| FR-484-AC-7 | An undefined subexpression of a clause settles `refuted` with `undefined-evaluation{where, cause}` whose `where.locus` is that subexpression; a path condition no assignment satisfies settles `inconclusive` with `kani_vacuous_proof`. | Test (TC-973) |
| FR-484-AC-8 | A claim over `Integer` parameters with no bounds is `unbounded` with `integer` domains and reaches the arm only on a candidate advertising `unbounded`; the same claim with an `integer-range` bound for each parameter is `bounded`; a clause quantifying over an unbounded population adds one `population` domain. | Test (TC-973) |
| FR-484-AC-9 | A call whose callee body is in neither the package nor the given dependency closure settles `unsupported` with reason `call-not-evaluable`, never `proved`; a call the arm starts and cannot finish within `max_cases` settles `incomplete`. | Test (TC-973) |

## Dependencies

- [FR-483](FR-483-settle-backend-items-at-the-check-arm.md) owns the outcome mapping, evidence and refusals.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns the claim-form table this mapping follows.
- [FR-146](../expressions/FR-146-check-total-pure-functions.md) owns `case` semantics, guard propagation and the admission obligations that stay separate.
- [FR-196](FR-196-generate-kani-contracts.md) owns the inclusive domain bounds a Kani arm uses.
