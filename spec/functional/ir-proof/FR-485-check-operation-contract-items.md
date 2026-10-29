---
id: FR-485
title: "Check an operation-contract item: clauses, frames, step relations and simulation"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-483
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-353
    type: references
  - target: ix://agent-ix/quire-specification/FR-397
    type: references
  - target: ix://agent-ix/quire-specification/FR-468
    type: references
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/FR-342
    type: references
  - target: ix://agent-ix/quire-specification/FR-340
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-013
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
---

# FR-485: Check an operation-contract item: clauses, frames, step relations and simulation

## Description

When an `operation-contract` item reaches a check arm, the arm SHALL decide the form's obligation, from the check input it derives from the claim form that [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) assigns to the claim and from the pre-state premises, frame clauses and invariant clauses of the checked package, for every admitted pre-state and argument vector, settling it by [FR-483](FR-483-settle-backend-items-at-the-check-arm.md).

## Inputs

An `operation-contract` item settled `supported`; the checked package with its `state`/`state_clause`, `state`/`operation_anchor` and `state`/`frame` nodes ([FR-341](../../objects/interfaces/FR-341-checked-package-state-clause-body.md), [FR-342](../../objects/interfaces/FR-342-checked-package-operation-anchor-body.md), [FR-340](../../objects/interfaces/FR-340-checked-package-frame-location.md)); for a relation, the `claim`/`step_relation` node ([FR-397](../analysis/FR-397-check-step-relations-over-model-operations.md)); for a refinement, the refinement node with its step rows ([FR-375](../analysis/FR-375-declare-model-refinement.md)); and each operation's abstraction-relation binding where present ([FR-353](FR-353-declare-abstraction-relation.md)).

## Outputs

A check input, and the settlement FR-483 states, with a `contract-counterexample` for a violation.

## Behavior

### Forms

The claim's node and occurrence select exactly one form:

| Form | Claim node and occurrence | FR-290 row |
| --- | --- | --- |
| `invariant` | a `state`/`state_clause` node whose clause is `invariant`, occurrence role `claim` | Operation invariant |
| `clause` | a `state`/`state_clause` node whose clause is `precondition` or `postcondition`, occurrence role `claim` | Operation precondition or postcondition |
| `frame` | the operation's `state`/`operation_anchor` node, occurrence role `anchor`, whose `frame` node holds a `modifies`, `creates` or `deletes` set | Frame obligation |
| `step-relation` | a `claim`/`step_relation` node, occurrence role `claim`, whose every execution has an implementation binding | Code claim of a `relation` clause |
| `simulation` | a refinement node, occurrence role `generated`, ordinal 0 for the initial-state rule and ordinal `k + 1` for the step row at position `k` of its `steps` (FR-322-AC-88 to FR-322-AC-91, [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)) | Per-step simulation obligation, or implication between two operation contracts |

The `step-relation` item is the item whose kind is `operation-contract` on the claim node whose model claim is `temporal-satisfaction`; their obligation identities differ by kind ([FR-397](../analysis/FR-397-check-step-relations-over-model-operations.md)). Any other node or occurrence settles `declined` `invalid-input` (FR-483).

### Common premises

Every form but `simulation` ranges over *pre-states of the declared types*, not over reachable states. The *pre-state premises* of an operation `o` of context `C` at pre-state `s` and argument vector `a` are, in this order:

1. `s` is a state of the declared types of `C`'s model, including every field's declared bounds;
2. `a` is in the parameter domains of `o`;
3. every invariant clause of `C` holds at `s`, so the invariant a `preservation` obligation checks is among them;
4. every effective precondition of `o` holds at `(s, a)`.

The *frame premise* on a post-state `s'` is that `(s, s')` lies within `o`'s frame: every location outside the `modifies`, `creates` and `deletes` sets of the frame node is equal in `s` and `s'`, and no instance outside `creates` appears nor any inside `deletes` remains ([FR-013](../foundation/FR-013-validate-operation-frames.md)). The *post premise* is that the effective postcondition holds over `(s, s', a, r)` for the result `r`.

An empty premise set is not a failure of a clause: a check whose pre-state premises no `(s, a)` satisfies makes zero checks and FR-483 settles it `inconclusive` `kani_vacuous_proof`.

### The oracle

Each form states what relates `s` to `s'` and `r`. When the operation has an implementation binding ([FR-353](FR-353-declare-abstraction-relation.md)), the *oracle* is the bound implementation: the arm runs it once from the pre-state, every outcome of a nondeterministic implementation is covered, and the calls of one item share no state. The FR-353 abstraction relation is a premise of the item and is read as authored, never inferred or repaired: implementation pre-states range over the states whose abstraction image is a state of the declared types, the pre-state premises are evaluated on the image, and a counterexample carries the images, not the implementation states. An operation, object or population the item reads with no binding makes the arm settle `declined` `incomplete-input` with the FR-353 diagnostic `missing_declaration`/`missing-name`, naming the unbound element and its owning model, and the refusal is scoped to the item.

When the operation has no implementation binding, the oracle is its *contract relation*: `(s, s', a, r)` is a behaviour when the frame premise and the post premise hold.

### Invariant

The item's goal is that the invariant clause holds in every initial state and is preserved by every operation of its context. The arm checks two obligations and the item holds only when both do:

- *initial-state*: the clause holds in every initial state of the context's model;
- *preservation*: for every operation `o` of `C` in declaration order, every `(s, a, s', r)` that satisfies the pre-state premises, the frame premise and the post premise satisfies the clause at `s'`.

One item carries both obligations; a violation names the failing one in its counterexample's `obligation`, `initial-state` or `preservation`, and for `preservation` names the operation. A request item is identified by its claim and kind ([FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) request identity) and an invariant clause's occurrence names no operation, so a second item on the clause would repeat the first's identity: an invariant clause is one item and a model refinement's *hypothesis item* ([FR-468](../analysis/FR-468-certify-per-step-refinement-obligations.md)) is that item. An invariant clause that is violated by an unreachable pre-state is still violated: the obligation ranges over declared types.

### Clause

For a `postcondition` or `precondition` clause the arm reads the oracle as follows.

- With an implementation binding, the goal of a `postcondition` is that the post premise holds over every outcome of the implementation run from every `(s, a)` that satisfies the pre-state premises, with the frame premise ignored (it is the `frame` form's goal). The goal of a `precondition` is that the precondition is defined at every `(s, a)` satisfying premises 1 to 3.
- Without an implementation binding, the goal of either is that the clause is *defined*: every subexpression of the condition evaluates to a value at every state and argument vector that satisfies the premises, and for a `postcondition` at every `(s, s', a, r)` that also satisfies the frame premise. An undefined subexpression is a violation, `refuted` with `undefined-evaluation{where, cause}`, `where.locus` that subexpression.

A `postcondition` with an implementation binding whose value the arm cannot run settles `unsupported` with reason `no-implementation-engine`.

### Frame

The goal of a `frame` item is that, from every `(s, a)` that satisfies the pre-state premises, every outcome of the implementation leaves every location outside the frame equal. A frame item with no implementation binding for its operation has no contract-level reading, because the contract relation assumes the frame; the arm settles it `unsupported` with reason `no-contract-level-reading`. A provider with no implementation engine settles a bound frame item `unsupported` with reason `no-implementation-engine`.

### Step relation

The check input of a `step-relation` item is the node's body ([FR-397](../analysis/FR-397-check-step-relations-over-model-operations.md) "Code claim"): the executions in order, the lowered relation body, each model's declared invariants, and each operation's effective precondition, frame, postcondition and binding. The variables are, per execution in binding order, a pre-state, an argument vector, a result and a post-state. The premises are, for each execution, the pre-state premises of its operation on its own `s_i` with its own invariants, and each execution runs its bound implementation once from its own `s_i`; the goal is that the relation body holds on the executions' results and post-states. The item states nothing about whether the invariants hold: a premise no state satisfies is vacuity. The relation is a hyperproperty over several executions and is not an induction over reachable states.

### Simulation

The `simulation` item's check input is the refinement's record ([FR-468](../analysis/FR-468-certify-per-step-refinement-obligations.md) "The queries"). For the initial-state rule (ordinal 0), the goal is that `map(c0)` is an abstract initial state for every concrete initial state `c0` satisfying the concrete invariants. For the step row at position `k` (ordinal `k + 1`), the variables are `(s, s')` of the declared types and the row's receiver, arguments and `_` arguments; the premises are the concrete invariants at `s`, the concrete operation's effective precondition at `s`, the frame premise and the post premise over `(s, s')`; and the goal is the row's simulation rule over `(map(s), map(s'))`. A row, receiver or argument that is undefined or undetermined at `s` or `s'` is a violation. The item reads no implementation binding.

The arm reports a violation of a `simulation` item as `refuted` with a counterexample, as for every item. That result is the item's result only; the verdict of the refinement's safety half, which never settles `refuted` for a step violation, is FR-468's and FR-379's to compute from the item results. The arm SHALL NOT report a simulation violation as `inconclusive`.

The other implication between two operation contracts ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md), relation-family refinement gate) uses the form `simulation` only when its claim node is a refinement node. An implication whose claim node is another node is not a form and settles `declined` `invalid-input`.

### Counterexample

A `violation` records one `contract-counterexample`, `{type, obligation, violated, bound, executions}`:

- `obligation` is `initial-state`, `preservation`, `definedness`, `postcondition`, `frame`, `relation`, `simulation-initial` or `simulation-step`;
- `violated` is the `{node, occurrence}` locus of the clause, relation body or rule that is false or undefined;
- `bound` is `true` when the item's oracle is a bound implementation and `false` when it is the contract relation;
- `executions` holds one entry per execution, in binding order (one entry for every form but `step-relation`), each `{operation, pre_state, arguments, result, post_state}` and, when `bound` is `true`, also `concrete`: `operation` the anchor node or `null` for an initial-state violation; `pre_state` and `post_state` the [FR-181](../runtime/FR-181-simulate-finite-executions.md) typed value of the model state the item reads, a `record` named by the context's declaration identity whose fields are the context object's fields, with a `set` field for each population the item reads, and for a bound item the *abstraction images* of the concrete states; `arguments` one `{parameter, value}` per parameter node; `result` the typed value or `null`; `post_state` `null` when the violated obligation reads none; `concrete` `{pre_state, arguments, result, post_state}` with the typed values of the *implementation state representations* ([FR-353](FR-353-declare-abstraction-relation.md)) the bound implementation was run on and produced.

A bound item records both the images and the concrete values: the images are what the premises and the obligation read, and the concrete values are what a replay re-executes. A violation that lives only in a concrete location the abstraction does not expose, such as a `frame` write outside `modifies`, is visible in `concrete` alone.

### Replay

This requirement owns the replay of a `contract-counterexample`; [FR-197](FR-197-replay-backend-counterexamples.md) owns only the replay of `assignments`. The arm SHALL replay a candidate counterexample before it reports `refuted` (FR-483), and a reader replays a received one the same way, with no reachability prefix:

- *Unbound* (`bound` `false`): evaluate the pre-state premises, the frame premise where the form uses it, the post premise and the violated obligation on the recorded values with the native evaluator. The replay reproduces the violation when the premises hold and the obligation is false or undefined with the recorded cause.
- *Bound* (`bound` `true`): for each execution, re-execute the bound implementation from the recorded `concrete.pre_state` and `concrete.arguments`; the replay requires the recorded `concrete.result` and `concrete.post_state` to be one of the implementation's outcomes, so every outcome of a nondeterministic implementation is admitted and none is invented; map every concrete state through the FR-353 abstraction relation and require the result to equal the recorded images; then evaluate the premises and obligation on the images, except that the `frame` obligation compares the concrete locations outside the frame. A recorded outcome the implementation does not produce, or an image that differs, does not reproduce the violation.

A counterexample that does not reproduce settles the item `inconclusive` with cause `replay-parity` (FR-483) and is recorded nowhere.

### Extent

An `operation-contract` item's roots are the receiver (`self`), result and parameter types of its operation; the context population that covers the receiver's type; each population the operation's clauses range over by a quantifier, `allInstances` or `reaches`; and, for `simulation`, the roots [FR-468](../analysis/FR-468-certify-per-step-refinement-obligations.md) "Claim extent" lists. A root with no finite maximum is an unbounded domain of its kind, and an item with no unbounded root is `bounded`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-485-AC-1 | The claim node and occurrence select one of five forms; a node outside them, and an implication whose claim is not a refinement node, settle `declined` `invalid-input`. | Test (TC-974) |
| FR-485-AC-2 | An invariant `self.l <= self.h` is one item holding `initial-state` and `preservation` for every operation of its context; an operation `step` that can set `l` above `h` from a pre-state satisfying the invariant refutes it with `obligation` `preservation` naming `step`, and an initial state with `l > h` refutes it with `initial-state`. | Test (TC-974) |
| FR-485-AC-3 | The preservation obligation ranges over declared types: a violating pre-state that no run reaches still refutes the invariant item. | Test (TC-974) |
| FR-485-AC-4 | A postcondition without an implementation binding is checked for definedness at every state satisfying the pre-state and frame premises; `self.h / self.d >= 0` with `d` possibly zero is `refuted` with `undefined-evaluation` and `obligation` `definedness`. | Test (TC-974) |
| FR-485-AC-5 | A bound postcondition is checked over every implementation outcome; an implementation that leaves a field unchanged that the postcondition requires incremented is `refuted` with `obligation` `postcondition` and abstraction images in the counterexample; a provider with no implementation engine settles it `unsupported` `no-implementation-engine`. | Test (TC-974) |
| FR-485-AC-6 | A frame item with no implementation binding settles `unsupported` `no-contract-level-reading`; a bound frame item whose implementation writes outside `modifies` is `refuted` with `obligation` `frame`. | Test (TC-974) |
| FR-485-AC-7 | An operation, object or population an implementation-bound item reads with no abstraction binding settles `declined` `incomplete-input` with `missing_declaration`/`missing-name` naming the element, and a sibling item with its bindings is unaffected. | Test (TC-974) |
| FR-485-AC-8 | A step relation whose every execution is bound is one `operation-contract` item distinct from its `temporal-satisfaction` item; a determinism relation over a nondeterministic implementation is `refuted` with a two-execution `relation` counterexample whose pre-states satisfy the invariants and carry no reachability prefix; a relation whose invariants no state satisfies is `inconclusive` `kani_vacuous_proof`. | Test (TC-974) |
| FR-485-AC-9 | A `simulation` item with ordinal 0 is the initial-state rule and with ordinal `k + 1` is step row `k`; a violation of either is `refuted` with `obligation` `simulation-initial` or `simulation-step`, never `inconclusive`, and the refinement's safety half is computed from the item results by FR-468. | Test (TC-974) |
| FR-485-AC-10 | A `contract-counterexample` carries, per execution, `pre_state`, `arguments`, `result` and `post_state`, and for a bound item `bound` `true` with `concrete` beside the images; it replays with no reachability prefix by the rules of "Replay", re-executing the bound implementation on the recorded concrete inputs; and a counterexample that does not replay settles the item `inconclusive` `replay-parity`. | Test (TC-974) |
| FR-485-AC-11 | The FR-353 abstraction relation is read as authored: a counterexample of a bound item carries abstraction images and the concrete values, a violation in a concrete location the images do not expose is carried by `concrete` and reproduced by re-execution, and no binding is inferred for an element the relation does not bind. | Test (TC-974) |
| FR-485-AC-12 | An `operation-contract` item over an operation whose context population has no maximum is `unbounded` with one `population` domain, and `bounded` with a cardinality bound on that population. | Test (TC-974) |

## Dependencies

- [FR-483](FR-483-settle-backend-items-at-the-check-arm.md) owns the outcome mapping, evidence and refusals.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns the claim-form table.
- [FR-341](../../objects/interfaces/FR-341-checked-package-state-clause-body.md), [FR-342](../../objects/interfaces/FR-342-checked-package-operation-anchor-body.md) and [FR-340](../../objects/interfaces/FR-340-checked-package-frame-location.md) own the clause, anchor and frame nodes read.
- [FR-353](FR-353-declare-abstraction-relation.md) owns the abstraction relation that is a premise of a bound item.
- [FR-397](../analysis/FR-397-check-step-relations-over-model-operations.md) owns the step relation and its code claim.
- [FR-468](../analysis/FR-468-certify-per-step-refinement-obligations.md) and [FR-377](../analysis/FR-377-decide-model-refinement.md) own the simulation obligations and the half's verdict.
- [FR-197](FR-197-replay-backend-counterexamples.md) owns the replay of `assignments` only.
- [FR-196](FR-196-generate-kani-contracts.md) and [FR-350](FR-350-emit-verus-contracts.md) own the Kani and Verus generation of the same clauses.
