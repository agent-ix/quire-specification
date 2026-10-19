---
id: FR-391
title: "Settle state-graph claims from decisive evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-390
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
  - target: ix://agent-ix/quire-specification/FR-383
    type: references
  - target: ix://agent-ix/quire-specification/FR-385
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-392
    type: references
  - target: ix://agent-ix/quire-specification/FR-393
    type: references
  - target: ix://agent-ix/quire-specification/FR-394
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
---

# FR-391: Settle state-graph claims from decisive evidence

## Description

When an exploration of a model subject ends for a state-graph item, the
analyzer SHALL settle the item only from replayed evidence that no unexplored
part of the graph can change, and name its settlement method in the result.

## Inputs

The explored state graph with each node's open or closed status, the item's
form, predicates and instance binding, the run's limits and seed, and the
selected reductions.

## Outputs

One FR-360 label with one FR-243 basis per item, its settlement method, and
its evidence ([FR-393](../../objects/interfaces/FR-393-state-graph-evidence.md)).

## Open and closed nodes

A node whose outgoing edges an exploration has not fully computed is
**open**. These nodes are open: a node at the run's search horizon (its
maximum depth, a method parameter the result states); a boundary
state of a state constraint; a node whose expansion evaluated a contract
conjunction undecided; and every discovered node not expanded when a run limit
stopped the run. Every other explored node is **closed**: its outgoing edges
are exactly its FR-181 successors.

- **Trap.** A closed node from which no node where the target holds (`P`, or
  `Y` for `unique path`) and no open node is reachable is a trap. Every node
  reachable from a trap is closed and none satisfies the target.
- **Known count.** For `unique path`, let `H` be the explored graph restricted
  to the nodes that reach a `Y` node or an open node, with every edge out of a
  `Y` node removed. A node's path count saturates at 2: a `Y` node counts 1; a
  node in a non-trivial strongly connected component of `H`, or with a
  self-loop, counts 2; any other node counts the saturated sum over its edges
  in `H`. A node is **known** when no open node is reachable from it in `H`.
- **Decisive evidence** reads only facts that open nodes cannot change: a
  witness path, a trap with its closed forward closure, or two distinct paths
  from a known node.

## Verdicts

Each evidence row settles only after its evidence replays
([FR-394](FR-394-replay-state-graph-evidence.md)).

| Row | Condition | FR-360 label | FR-243 basis | Evidence |
| --- | --- | --- | --- | --- |
| Witnessed | `possible`: every initial state has a witness path to a node where `P` holds, and exploration completed with no open node and no undefined evaluation of a predicate of the claim or of the subject's guards, `terminal when` predicate or state constraints | `proved` | `decisive-witness` | `Witness` |
| Trap | a trap is reachable where the form requires reaching the target: an initial state (`possible`), a node where `Q` holds (`always possible`), a node where `X` holds (`unique path`) | `refuted` | `closed-scope` | `Trap` |
| Path pair | `unique path`: a known node where `X` holds counts 2 | `refuted` | `decisive-counterexample` | `PathPair` |
| Undefined | a predicate of the claim (`P`, `Q`, `X` or `Y`), or a guard, the `terminal when` predicate or a state constraint of the subject, evaluated undefined at an explored node | `refuted` | `decisive-counterexample` | `Undefined` |
| Exhaustive | `always possible` or `unique path`: exploration completed with no open node and no undefined evaluation of a predicate of the claim or of the subject's guards, `terminal when` predicate or state constraints, and every reachable node where `Q` holds is in `E(P)`, or every reachable node where `X` holds counts exactly 1 | `proved` | `closed-scope` | none |
| Witness found, well-definedness unchecked | `possible`: every initial state has a witness, and exploration ended with open nodes or a limit stopped it before any refuting evidence was found | `inconclusive` | `unsettled` | cause `well-definedness-unchecked`, with the witnesses |
| No decisive evidence | none of the rows above | by the [FR-363](../temporal/FR-363-settle-every-behaviour-temporal-verdicts.md) verdict map | | none |

**Refuting evidence is ordered.** A trap node, a known node that counts 2, and
a node at which a predicate, guard or state constraint evaluates undefined are
each refuting evidence.
When an exploration finds more than one, the analyzer SHALL settle the item
from the one whose node comes first in FR-181 canonical breadth-first
discovery order, as FR-363 orders a violation and an undefined evaluation. A
node at which an evaluation is undefined and which is itself a trap node or a
node that counts 2 settles as `Undefined`, since the evaluation at the node
comes first.

**Undefined evaluation.** An undefined evaluation refutes by the FR-363 rule
for every model proof engine. The expressions evaluated at an explored node
are the predicates of the claim and the subject's own expressions: a guard of
a transition enabled-tested at the node, the `terminal when` predicate at a
node with no enabled transition, and a state constraint that restricts the
explored states. When one of them evaluates undefined, the analyzer SHALL
settle the item `refuted` with cause `undefined-evaluation`, whose `where`
names the node's `quire.simulation.state-key/v1` digest and the locus of the
expression that has no value, plus the predicate for a predicate of the claim
and the FR-181 transition identity for a guard, and whose `cause` names the
undefined value's catalogued reason. An undefined guard is never read as a
disabled transition, and an undefined `terminal when` predicate never as a
non-terminal node. A subject's expression belongs to the subject, so its
undefined evaluation refutes every state-graph item over the subject whose
exploration reaches that node. Its evidence is the `undefined` arm of
[FR-393](../../objects/interfaces/FR-393-state-graph-evidence.md), which
carries that `undefined-evaluation` value, with the canonical breadth-first
path to the node as its stem; [FR-394](FR-394-replay-state-graph-evidence.md)
replays it to the carried value.

**Well-definedness is global.** A `proved` result asserts that the claim is
well-defined at every reachable node. The analyzer SHALL settle a state-graph
item `proved` only after an exploration with no open node has evaluated every
predicate of the claim, and every guard, `terminal when` predicate and state
constraint of the subject, at every reachable node without an undefined value. A
witness, whether sampled ([FR-392](FR-392-sample-possible-witnesses.md)) or
explored, finds the path that makes `possible` true; it does not by itself
settle the item. When every initial state has a witness and the exploration
ends with open nodes, or a limit stops it, before it finds refuting evidence,
the item settles `inconclusive` with cause `well-definedness-unchecked`, and the
result carries the witnesses and states that well-definedness was not
checked. The verdict therefore does not depend on `witness_samples`.

A trap found before a run stops settles the item, as does a known node that
counts 2, when no refuting evidence precedes it in discovery order.

**No decisive evidence.** The FR-363 verdict map applies unchanged: a run a
limit stopped settles FR-331 result `incomplete` with cause `limit-reached{limit, value, setting}` (FR-360 `failed`, execution
`resource-incomplete`), and a cancelled run `incomplete` with cause
`cancelled{source}`, unless every initial state already has a witness, when it settles
`inconclusive` with `well-definedness-unchecked`; a completed run with open nodes settles `inconclusive` with
`undecided-successor` when an undecided expansion is among them, else with
`constraint-reached` when a boundary state is among them
([FR-383](FR-383-explore-under-state-space-reduction.md)), else with
`bound-reached` naming the search horizon, except that under partial-order
reduction such a run settles as [FR-385](FR-385-report-reduced-verdicts.md)
states for a run with open nodes; a subject with no initial state settles
`inconclusive` with `no-initial-state`; an item with zero instances settles
`inconclusive` with `no-instance` before expansion; a selected reduction the form does not
admit settles `inconclusive` with `reduction-not-preserving` before expansion
(FR-385). A search bounded by its horizon that found no refuting evidence and
not every initial state's witness is `inconclusive`, never `proved` or
`refuted`.

**Replay disagreement.** Evidence that fails its replay check settles the item
`inconclusive` with `replay-parity`. No `proved` from a witness and no
`refuted` reaches an item before replay reproduces its evidence.

FR-360's five labels and their FR-241, FR-242 and FR-243 combinations apply to
state-graph results unchanged.

## Settlement method

Every state-graph result states how it was settled, in its proof basis and in
its FR-331 terminal record:

- a witnessed `proved`, and a `well-definedness-unchecked` result, names, per
  initial state, its witness source:
  `sampled` with the seed and trace index of the walk
  ([FR-392](FR-392-sample-possible-witnesses.md)), `explored`, or
  `unrolled` with the unrolling depth;
- an exhaustive `proved` names exhaustive exploration, or reduced exploration
  with its reductions;
- a `refuted` names exploration and carries its evidence;
- an `inconclusive` or `failed` result names the limits and the search
  horizon the run used; a `well-definedness-unchecked` result also states that
  the claim's well-definedness was not checked.

The verdict does not depend on the method: a sampled witness and an explored
witness give the same `proved`.

## Determinism and canonical evidence

A result and its evidence are a function of the subject, the claim, the
limits and the seed. Edges out of a node are ordered by their transition
identity's canonical bytes (FR-181), then by the post-state's state-key bytes.

- An explored witness from initial state `i` is the shortest path from `i` to
  a `P` node, taking at each node the first edge in canonical order whose
  target is one step nearer.
- A trap is the first trap node in FR-181 canonical breadth-first discovery
  order where the form requires reaching the target, and its stem is the
  canonical breadth-first path to it.
- A path pair from node `x` is the shortest path to `Y`, ties broken by
  canonical edge order, and the least deviation from it: over every position
  of the first path and every other edge out of that position's node into
  `H`, the prefix, that edge, then the shortest path on, ordered by length
  and then canonical edge order.

## Reductions

These rows extend the FR-385 preservation table. A state-graph item admits a
reduction exactly where its row reads "yes"; a
"no" settles the item before expansion as `inconclusive` with
`reduction-not-preserving`.

| Form | Admitted symmetry | Partial-order reduction | State constraint |
| --- | --- | --- | --- |
| `possible` | yes | yes, with every transition that writes a location `P` reads made visible | trap decisive; `proved` only when no boundary state is reached; witnesses with a boundary state reached settle `well-definedness-unchecked` |
| `always possible` | yes | no: branching property | trap decisive; `proved` only when no boundary state is reached |
| `unique path` | no: the quotient merges paths | no: it prunes distinct paths | path pair decisive; trap decisive; `proved` only when no boundary state is reached |

Under an admitted symmetry the predicates `P`, `Q`, `X` and `Y` are among the
clauses that must be identity-transparent. Evidence found by a reduced search
is concrete and carries no canonical state or permutation, so a witnessed
`proved` names its witness sources, never a reduction.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-391-AC-1 | A `possible` item whose every initial state has a replayed witness settles `proved`, `decisive-witness`, only after an exploration with no open node found no undefined evaluation of its predicates or of the subject's guards, `terminal when` predicate or state constraints. | Test (TC-331, TC-336) |
| FR-391-AC-2 | A reachable trap where the form requires reaching the target settles `refuted`, `closed-scope`, also when the run reached its maximum depth elsewhere. | Test (TC-331, TC-332, TC-334) |
| FR-391-AC-3 | A known node where `X` holds that counts 2 settles `unique path` `refuted`, `decisive-counterexample`, with a path pair. | Test (TC-333) |
| FR-391-AC-4 | `always possible` and `unique path` settle `proved`, `closed-scope`, only from a completed exploration with no open node. | Test (TC-332, TC-333, TC-334) |
| FR-391-AC-5 | A run with no decisive evidence settles by the model-subject verdict map: `bound-reached`, `constraint-reached`, `undecided-successor`, `no-initial-state` or `no-instance` as `inconclusive`, a stopped run as `incomplete` with cause `limit-reached{limit, value, setting}`, a cancelled run as `incomplete` with cause `cancelled{source}`; it never settles `proved` or `refuted`. | Test (TC-334) |
| FR-391-AC-6 | Every result states its settlement method: per-initial-state witness sources with seed and trace index for a sampled witness, exhaustive or reduced exploration, or the limits used. | Test (TC-331, TC-336) |
| FR-391-AC-7 | Each state-graph result label matches exactly one FR-360 member with its exact FR-241, FR-242 and FR-243 combination. | Test (TC-334) |
| FR-391-AC-8 | Each form admits symmetry, partial-order reduction and a state constraint exactly as its reductions row reads; a reduction it does not admit settles `inconclusive` with `reduction-not-preserving` before expansion. | Test (TC-334) |
| FR-391-AC-9 | Two runs with the same subject, claim, limits and seed return the same result and the same evidence. | Test (TC-336) |
| FR-391-AC-10 | A predicate of a `possible`, `always possible` or `unique path` claim that evaluates undefined at an explored node settles `refuted`, `decisive-counterexample`, cause `undefined-evaluation` naming the predicate, the node's state-key digest, the expression's locus and the catalogued reason, with an `undefined` arm that carries that value and replays to it, also when a witness exists. | Test (TC-349) |
| FR-391-AC-11 | When an exploration finds a trap node, a node that counts 2 and a node with an undefined predicate, the item settles from the one first in FR-181 discovery order: a trap discovered before the undefined node settles `refuted`, `closed-scope`, and an undefined node discovered first settles `refuted` with cause `undefined-evaluation`. | Test (TC-349) |
| FR-391-AC-12 | A `possible` item with a sampled witness for every initial state and an undefined predicate at a reachable node off every walk settles `refuted` with cause `undefined-evaluation`, the same verdict as with `witness_samples` 0. | Test (TC-336, TC-349) |
| FR-391-AC-13 | A `possible` item with a witness for every initial state whose exploration a limit stops, or ends with open nodes, before refuting evidence is found settles `inconclusive` with cause `well-definedness-unchecked`, carrying the witnesses and stating that well-definedness was not checked; never `proved`. | Test (TC-334, TC-336) |
| FR-391-AC-14 | A guard, the `terminal when` predicate or a state constraint of the subject that evaluates undefined at an explored node settles every `possible`, `always possible` and `unique path` item over the subject `refuted`, `decisive-counterexample`, cause `undefined-evaluation`, whose `where` carries the node's state-key digest and the expression's locus, plus the FR-181 transition identity for a guard, also when a witness exists for every initial state; an undefined guard is not read as a disabled transition. | Test (TC-349) |

## Dependencies

- [FR-390](FR-390-analyze-state-graph-claims.md) states the forms and their
  truth.
- [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
  and [FR-243](../../objects/protocol/FR-243-settlement-basis.md) supply the
  labels and bases.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  carries the terminal record.
- [FR-394](FR-394-replay-state-graph-evidence.md) replays the evidence.
- [FR-363](../temporal/FR-363-settle-every-behaviour-temporal-verdicts.md)
  owns the model-subject verdict map, its inconclusive causes and the
  undefined-evaluation rule.
- [FR-383](FR-383-explore-under-state-space-reduction.md) and
  [FR-385](FR-385-report-reduced-verdicts.md) own state constraints and the
  preservation table these rows extend.

## References

- STD-135 (this requirement); QSL-369 and its ADR-022 (design).
- QSL-366: the owner ruling that an undefined evaluation refutes.
