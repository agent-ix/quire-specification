---
id: FR-390
title: "Analyze state-graph claims over a model subject"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-043
    type: references
  - target: ix://agent-ix/quire-specification/FR-391
    type: references
  - target: ix://agent-ix/quire-specification/FR-393
    type: references
  - target: ix://agent-ix/quire-specification/FR-361
    type: references
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
---

# FR-390: Analyze state-graph claims over a model subject

## Description

When a `possible`, `always possible` or `unique path` clause is checked over
a model subject, the analyzer SHALL evaluate it over the subject's state
graph by the rules below.

## Inputs

The checked clause, its form, its predicates, its `over` binding, and the
model subject: initial states, universes and proof bounds.

## Outputs

One result per claim instance, settled by
[FR-391](FR-391-settle-state-graph-claims.md), with its evidence
([FR-393](../../objects/interfaces/FR-393-state-graph-evidence.md)).

## The state-graph claim family

The three clauses of the shared grammar's `state-graph-clause` production are
**state-graph claims**, a claim family of their own beside temporal claims.

| Form | Source | Holds when |
| --- | --- | --- |
| `possible` | `possible N using v [over (x: T)] { P }` | from every initial state of the subject some finite path reaches a node where `P` holds (CTL `EF P` at every initial state) |
| `always possible` | `always possible N using v [over (x: T)] [from (Q)] { P }` | from every reachable node where `Q` holds some finite path reaches a node where `P` holds; without `from`, `Q` is `true` (CTL `AG (Q implies EF P)`) |
| `unique path` | `unique path N using v [over (x: T)] from (X) to (Y);` | every reachable node where `X` holds has exactly one path to `Y` |

A state-graph claim has no temporal profile, no clock, no activation and no
fairness set. Its `over` parameter binds one object of a population of the
subject's model, and the clause has one instance per key of that population's
universe; a clause with no `over` parameter has one instance. Where the bound
object does not exist at a node, its predicates read the bound object as a
temporal atom over a model subject reads an absent `over` object.

## The state graph

- **Nodes and edges.** The state graph has a node for each state reachable
  from an initial state of the subject, and an edge `s -t-> s'` for each
  [FR-181](../runtime/FR-181-simulate-finite-executions.md) successor of `s`,
  with transition identity `t` and post-state `s'`. An edge is the triple
  (source, transition identity, post-state). The graph has no stutter step: a
  terminal state is a node with no outgoing edge.
- **Reachability.** A node is reachable when a path of edges leads to it from
  an initial state. Universes and proof bounds define the subject and so the
  graph, and a verdict holds for exactly its subject. A state constraint and
  the run limits decide which nodes an exploration expands; they never decide
  which nodes are reachable.
- **State predicates.** `P`, `Q`, `X` and `Y` are Boolean clauses under the
  unit's value profile, as an invariant body is. A predicate is evaluated at a
  node over that node's state alone, with the `over` parameter bound, so its
  truth at a node does not depend on the edge that reached it.

## Truth

- **`E(P)`.** `E(P)` is the set of nodes from which some path of zero or more
  edges reaches a node where `P` holds: the least fixpoint of
  `Z = P ∪ pre(Z)`, where `pre(Z)` is the set of nodes with an edge into `Z`.
  A node where `P` holds is in `E(P)` by the empty path.
- **`possible`** holds when every initial state is in `E(P)`.
- **`always possible`** holds when every reachable node where `Q` holds is in
  `E(P)`. A terminal node therefore satisfies it exactly when `P` or `not Q`
  holds there.
- **Paths to `Y`.** A path from node `x` to `Y` is a finite edge sequence
  from `x` whose last node satisfies `Y` and no earlier node does. When `x`
  satisfies `Y`, the empty path is its only path. Two paths are distinct when
  their edge sequences differ. A cycle on a path from `x` to `Y` gives `x`
  unboundedly many paths.
- **`unique path`** holds when every reachable node where `X` holds has
  exactly one path to `Y`.

## Fairness, terminal states and deadlocks

- Fairness plays no part in a state-graph claim. Weak and strong fairness of
  operations are machine-closed: every finite path from an initial state
  extends to a fair behaviour, so every reachable node lies on a fair
  behaviour and fairness changes no state-graph verdict.
- A terminal state, intended or not, is a node with no outgoing edge and reads
  by the rules above like any other node. A model's `terminal` member changes
  no state-graph verdict.
- A request that holds a state-graph item over a model subject also holds that
  subject's one derived deadlock-freedom item, as a request with a temporal
  item over that subject does. A subject that both kinds of item name has one
  deadlock-freedom item.

## Admission

- When a predicate of a state-graph claim reads an operation anchor or a
  pre-state, the checker SHALL refuse it as
  `unsupported_construct`/`expression-form`, located at the read.
- When a state-graph claim contains a temporal operator, at any operand
  position of its body, `from` or `to` predicate and with or without an
  interval, the checker SHALL refuse it as
  `unsupported_construct`/`expression-form`, located at the operator; where a predicate contains several temporal operators,
  the refusal is located at the first one in source order.
- When a state-graph claim carries a fairness constraint, the checker SHALL
  refuse it as `unsupported_construct`/`expression-form`, located at the
  constraint.

The shared grammar's `graph-predicate` and `graph-body` productions recognize
a temporal operator at any operand position of a body, `from` or `to`
predicate, and a fairness constraint at the start of a body. Exactly those
cases reach this admission step as parsed source and are refused here. A
fairness constraint elsewhere (after the predicate, in a `from` or `to`
operand, or on `unique path`, which has no body), and a temporal operator in
an `if` condition, a `let` binding or a primary's operand, and a chain of
binary temporal operators without parentheses (`c.a until c.b until c.c`; the
parenthesised `(c.a until c.b) until c.c` parses and is refused), are syntax
errors and never reach admission. The fairness refusal therefore applies to
`possible` and `always possible` only. An interval, including `[a,*]`, does not
change which refusal applies: the operator is refused as written, and no
temporal profile is consulted.

## Routing

Each claim instance is one requested item. It requires the FR-290 kind
`temporal-satisfaction` with an unbounded extent naming the claim, and its
form (`possible`, `always possible` or `unique path`) is recorded beside the
item as an obligation form. A candidate's negotiation arm settles the item
`supported` only when it discharges that form, and settles it `unsupported`,
warned, `unsupported_projection`/`unsupported-requested-capability` otherwise.
A trace monitor evaluates no state-graph claim.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-390-AC-1 | Each of the three state-graph clauses parses to its form, with its predicates (each a `graph-predicate`), its optional `over` parameter and, for `always possible`, its optional `from` predicate; an `always possible` clause without `from` reads `Q` as `true`. | Test (TC-330) |
| FR-390-AC-2 | A state-graph predicate that reads an operation anchor or a pre-state, a temporal operator in a state-graph claim (unary or binary, with or without an interval, in a body, `from` or `to` operand, including `c.n = 1 and eventually c.n = 2`), and a fairness constraint at the start of a claim body each parse and then refuse as `unsupported_construct`/`expression-form` located at the offending construct. | Test (TC-330) |
| FR-390-AC-3 | `possible` holds exactly when every initial state is in `E(P)`, and `always possible` exactly when every reachable node where `Q` holds is in `E(P)`, on the ConfigVersion and game vectors. | Test (TC-331, TC-332) |
| FR-390-AC-4 | `unique path` counts paths per reachable node where `X` holds: two interleavings of independent steps give one such node two paths, a sequenced model gives one, and a cycle on a path to `Y` gives more than one. | Test (TC-333) |
| FR-390-AC-5 | A reachable terminal node where `Q` holds and `P` does not refutes `always possible`; declaring a `terminal when` predicate that holds there changes no state-graph verdict and settles the subject's deadlock-freedom item `proved`. | Test (TC-332) |
| FR-390-AC-6 | A request whose only items over a subject are state-graph items holds that subject's one deadlock-freedom item. | Test (TC-330) |
| FR-390-AC-7 | Each claim instance requires `temporal-satisfaction` with its form recorded as an obligation form; a single candidate whose arm does not discharge the form settles the item `unsupported` with `unsupported_projection`/`unsupported-requested-capability`; an `over` clause yields one item per universe key. | Test (TC-330) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns
  the `state-graph-clause` production.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the successor
  relation, canonical exploration order and state keys the state graph uses.
- [FR-043](../foundation/FR-043-evaluate-finite-graph-relations.md) owns
  `reaches`, which reads one state's object graph and is distinct from the
  state graph read here.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) assigns the
  capability kind.
- [FR-391](FR-391-settle-state-graph-claims.md) settles each instance.
- [FR-361](../temporal/FR-361-read-model-behaviours-as-temporal-traces.md) and [FR-366](../temporal/FR-366-check-deadlock-freedom.md)
  own the model subject, the `over` binding over a model population, the derived deadlock-freedom item and the `terminal` member.

## References

- STD-135 (this requirement); QSL-369 and its ADR-022 (design).
