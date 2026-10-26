---
id: FR-405
title: "Declare random parameters, workloads and rewards"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-013
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-142
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-205
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-406
    type: references
  - target: ix://agent-ix/quire-specification/FR-415
    type: references
---

# FR-405: Declare random parameters, workloads and rewards

## Description

When a package declares a random parameter, a workload or a reward over a
model's operations, the frontend SHALL admit the declaration only in the exact
form below and SHALL bind it to the model subject before any probabilistic
claim over that subject is evaluated.

## Inputs

A checked package with a state model and its operations (FR-013, FR-181); the
native `random`, `reward` and `workload` declarations of the shared grammar's
probabilistic forms ([shared grammar](../../../proposals/quire-v1/shared-grammar.md));
exact rational literals (FR-205) and quantities (FR-142).

## Outputs

The model subject with each operation parameter classified as random or
non-random, each random parameter's distribution, each operation's rewards and
each declared workload, or a located typed refusal naming the declaration and
the violated rule.

## Behavior

### Random parameters

A random parameter declaration names one parameter of one operation of the
model and gives a distribution: a finite list of (value, weight) pairs. Each
weight is a positive exact rational. Each value is a literal of the
parameter's declared type, and the values are pairwise distinct under the
parameter type's equality. The probability of a value is its weight divided by
the sum of the distribution's weights. The frontend refuses a repeated value
with `invalid_probabilistic_model`/`repeated-support-value`, a weight that is
not a positive exact rational with `invalid_probabilistic_model`/`invalid-weight`,
and a value outside the parameter's type with
`invalid_probabilistic_model`/`value-outside-type`, each at the value's locus.

The set of a random parameter's values is its **support**. The support is the
parameter's finite domain for every purpose of FR-181: successor enumeration,
the requires-bound pre-check and the transition identity. A random parameter
has exactly one distribution: the frontend SHALL refuse a second `random`
declaration of a parameter that already has one with
`invalid_probabilistic_model`/`duplicate-distribution` at the second
declaration's locus, naming the operation and the parameter.

An operation's precondition and guard read no random parameter. Its
postcondition and its rewards may read random parameters. The frontend SHALL
refuse a precondition that reads a random parameter with
`invalid_probabilistic_model`/`random-read-in-precondition` at the read's source locus.

### Rewards

A reward declaration names one operation and gives a reward name and an
expression. The expression's type is an integer type or a quantity type
(FR-142), and it reads only the operation's parameters, random parameters
included. It reads no state (`invalid_probabilistic_model`/`state-read-in-reward`
at the read's locus), and a type other than integer or quantity refuses with
`invalid_probabilistic_model`/`reward-type`. The reward's value is non-negative for every
argument vector in the operation's domains; the frontend checks this over the
finite domains and refuses a negative value with
`invalid_probabilistic_model`/`negative-reward`,
naming the operation, the reward and the argument vector. Reward names
are unique per operation: a second reward of one name on one operation refuses
with `invalid_probabilistic_model`/`duplicate-reward` at the second
declaration, naming the operation and the reward. One reward name declared on
several operations has one type across them. A transition carries the reward values of its operation
under its arguments.

### Workloads

A workload declaration names one model and gives every operation of that model
exactly one positive exact rational weight. The frontend SHALL refuse a
workload that omits an operation (`invalid_probabilistic_model`/`incomplete-workload`),
weights an operation twice (`invalid_probabilistic_model`/`duplicate-weight`), names an
operation of another model (`invalid_probabilistic_model`/`unknown-operation`) or gives a
weight that is not a positive exact rational (`invalid_probabilistic_model`/`invalid-weight`).

### Delay distributions

Over a `time dense` model ([FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md))
an operation may be the subject of one model-level `delay M::o ~ D;`
declaration (`delay-decl` of the shared grammar), `D` one of the families of
[FR-420](FR-420-sample-stochastic-timed-models.md). An operation with no
`delay` declaration has a nondeterministic delay. A `delay` declaration on an
operation of an untimed or digital model refuses with
`invalid_probabilistic_model`/`delay-on-untimed-model`, and a second `delay`
declaration of one operation refuses with
`invalid_model_binding`/`malformed-declaration` at the second declaration.

A **scheduled identity** is a transition identity without its random
arguments: (operation, receiver, non-random argument vector).

### Identity

Random-parameter distributions, delay distributions, rewards and workloads are part of the model
subject's identity. A probabilistic claim's obligation identity includes the
workload it names ([FR-407](FR-407-state-probabilistic-claims.md)). Changing a
weight, a support value or a reward expression changes the identity of every
obligation that reads it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-405-AC-1 | A random parameter with distinct typed values and positive exact rational weights is admitted, each value's probability is its weight over the weight sum, and its support is its FR-181 enumeration domain. | Test (TC-350) |
| FR-405-AC-2 | A precondition or guard that reads a random parameter refuses `invalid_probabilistic_model`/`random-read-in-precondition` at the read's locus. | Test (TC-350) |
| FR-405-AC-3 | A distribution with a repeated value, a non-positive weight, or a value outside the parameter's type refuses `repeated-support-value`, `invalid-weight` or `value-outside-type` under `invalid_probabilistic_model` at the value's locus. | Test (TC-350) |
| FR-405-AC-4 | A reward reading state, typed other than integer or quantity, or negative for some argument vector refuses with its typed cause; an admitted reward is carried by every transition of its operation. | Test (TC-350) |
| FR-405-AC-5 | A workload that omits, duplicates or misnames an operation, or gives an invalid weight, refuses with its typed cause; a complete workload is admitted. | Test (TC-350) |
| FR-405-AC-6 | A `delay` declaration is admitted on an operation of a `time dense` model and refuses `invalid_probabilistic_model`/`delay-on-untimed-model` on an untimed or digital model; a second `delay` declaration of one operation refuses `invalid_model_binding`/`malformed-declaration`; an operation with no `delay` declaration has a nondeterministic delay. | Test (TC-350) |
| FR-405-AC-7 | Changing one weight, support value or reward expression changes the identity of every obligation that reads it. | Test (TC-350) |
| FR-405-AC-8 | A second `random` declaration of one operation parameter refuses `invalid_probabilistic_model`/`duplicate-distribution` at the second declaration's locus, naming the operation and the parameter. | Test (TC-350) |
| FR-405-AC-9 | A second `reward duration on Server::attempt = d;` after the first refuses `invalid_probabilistic_model`/`duplicate-reward` at the second declaration's locus, naming `Server::attempt` and `duration`; `reward duration` on `Server::attempt` and on `Server::request` with one value type is admitted. | Test (TC-350) |

## Dependencies

- [FR-013](../foundation/FR-013-validate-operation-frames.md) and
  [FR-181](../runtime/FR-181-simulate-finite-executions.md) define the
  operations, their finite domains and the successor relation the support
  enters.
- [FR-142](../type-model/FR-142-evaluate-quantities-and-units.md) types
  quantity rewards; [FR-205](../../objects/foundation/FR-205-exact-rational-value.md)
  types weights.
- [FR-406](FR-406-interpret-probabilistic-models.md) gives these declarations
  their probabilistic meaning.

## References

- QSL ADR-024 PM-1, PM-2, PM-5; QSL ADR-026 SD-1.
- Ticket: STD-137 (paired with QSL-371).
