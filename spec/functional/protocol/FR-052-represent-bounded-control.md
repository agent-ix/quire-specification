---
id: FR-052
title: "Represent bounded causal protocol control"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-050
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-220
    type: references
  - target: ix://agent-ix/quire-specification/FR-221
    type: references
  - target: ix://agent-ix/quire-specification/FR-227
    type: references
  - target: ix://agent-ix/quire-specification/FR-228
    type: references
  - target: ix://agent-ix/quire-specification/FR-479
    type: references
  - target: ix://agent-ix/quire-specification/FR-426
    type: references
---
## Description

When linking a protocol body, the protocol linker SHALL construct a finite causal graph for authored sequence, labeled choice, parallel branches, join, iteration and termination without deriving causal edges from source order or timestamps.

## Inputs

Typed protocol nodes, authored control edges, optional iteration maxima and
loop invariants and variants, branch join rules and termination declarations.

## Outputs

An acyclic finite control representation with explicit branch and progress edges,
or a located typed refusal.

## Behavior

Sequence SHALL add only its authored order. Parallel branches SHALL admit every
interleaving that preserves each branch's causal edges. A join SHALL name the
branches and completion rule it awaits. Iteration SHALL carry progress: every cycle in
the repeat's body graph, the back edge included, passes through at least one
step node: a node whose traversal is a step under
[FR-426](FR-426-apply-protocol-steps.md), which is an event, send, receive,
attempt, effect, commit or fence node, an `await` (its match or its
`timeout` step), or a `parallel` (its `fork` and `join` steps). An authored iteration maximum and an authored
invariant with a variant are each optional model meaning. A stated maximum
bounds the loop to that many iterations; an invariant and variant are read
only by a termination claim ([FR-171](FR-171-evaluate-dynamic-control.md)). A
loop with neither runs for as long as its guard admits, and an explorer closes
its cycles through the state key. The linker SHALL refuse missing joins,
cyclic causal dependencies, zero-progress loops and termination conditions
that cannot be reached within the declared finite graph. Reachability of
termination is structural: it reads the graph's edges, never a guard's value,
so a `while { true }` loop with a progress edge, whose exit edge exists but is
never taken, is admitted. A zero-progress loop, a
`repeat` with a cycle in its body graph that passes through no step node,
such as a `choice` case with an empty or check-only body, refuses
`unsupported_construct`/`declaration-form` at the `repeat`, naming the repeat
node and the selected protocol profile.

The linked graph SHALL retain, for each protocol activation that establishes
an obligation of a temporal clause, that authored relation exactly as the
protocol's activation relation
([FR-464](../../objects/interfaces/FR-464-checked-package-protocol-node-bodies.md)
"Activation relation") states it: the protocol clause, its activation and each
temporal clause the protocol requires. A source-table coordinate, a matching
profile, a clock, a display name or a runtime observation SHALL NOT add,
remove or substitute a relation, and an obligation is bound to its temporal
clause only through that relation
([FR-479](../../objects/protocol/FR-479-obligation-activation-binding.md)).

## Surface syntax

The shared grammar's `repetition` production
([shared grammar](../../../proposals/quire-v1/shared-grammar.md)) makes the
maximum and the invariant and variant each optional; only a repetition with a
maximum has an `exhausted` control.

## Causal interleavings

The interleavings a `parallel` admits are the behaviours of the protocol
system ([FR-426](FR-426-apply-protocol-steps.md)):

- **Every causal edge is enforced.** A step of node `m` is enabled only when
  every causal predecessor of `m` has occurred: a sequence edge through the
  owning thread's rest point, a branch edge through the fork that created the
  thread, a join edge through the join step, send to receive through the
  queue, attempt to effect through the effect's binder condition, and an
  await's anchor edge through the anchor's step.
- **No other order is added.** A step waits on another thread's step only
  through a join, a queue, a binder, a memory gate or the model state its own
  precondition or constraint reads. Source order, array order, timestamps and
  FIFO order on another channel add nothing. Canonical transition order
  orders the search and the choice of counterexample, never the set of
  behaviours.
- **The causal interleavings are the behaviours.** The projections of the
  behaviours onto their `attempt`, `event`, `send`, `receive`, `finish` and
  `cattempt` steps are exactly the linear extensions of the causal edge
  relation, unfolded along the choices and iterations each behaviour takes,
  in which each step is enabled when it is reached. For a protocol whose
  preconditions and constraints are all true and that uses no channel, the
  projections are every linear extension.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-052-AC-1 | Split shipments S1 and S2 may interleave while each branch preserves its own order and both satisfy the authored join before completion. | Test (TC-052) |
| FR-052-AC-2 | Reordering equal-time events does not add a causal edge absent from the protocol. | Test (TC-052) |
| FR-052-AC-3 | A missing join input, causal cycle or zero-progress loop refuses admission, the zero-progress loop with `unsupported_construct`/`declaration-form` naming the repeat, including a repeat whose body holds a step in one `choice` case and an empty sequence in another; a repeat whose body holds a step is admitted with a maximum only, with an invariant and variant only, with both, and with neither. | Test (TC-052) |
| FR-052-AC-4 | Changing any admitted semantic graph content—including a node/control kind, edge, guard or constraint value graph and dependency, event kind or binding, channel/FIFO rule, await rule, compensation reference, finish rule, branch, join, iteration maximum, loop invariant and variant, or a `parallel`'s source `memory` clause—requires a different qualified protocol-definition identity, which is package identity, while a check's obligation identity reads the resolved memory model ([FR-434](../memory/FR-434-select-memory-models-for-parallel.md)); source loci remain provenance rather than graph-key members. | Test (TC-052) |
| FR-052-AC-5 | Reusing one qualified protocol-definition identity with different complete typed graph content is an identity contradiction and refuses admission rather than selecting either graph. | Test (TC-052) |
| FR-052-AC-6 | For the split-shipment protocol with two branches of two events each and all constraints true, the event projections of the explored behaviours are exactly the six linear extensions of the causal edges; adding a precondition that disables one order removes exactly the extensions that need it, and reordering source text or timestamps changes none. | Test (TC-379) |
| FR-052-AC-7 | Each protocol activation of the linked graph relates exactly the temporal clauses the protocol requires, one relation per required clause; a relation inferred from a shared clock, profile, name or position, or one naming a temporal clause the protocol does not require, is absent from the graph, and a binding that uses it refuses (FR-479-AC-4). | Test (TC-052) |

## Dependencies

- [FR-050](FR-050-bind-protocol-instances.md).
- Agent A's shared grammar and predicate contracts.

## References

- Causal interleavings: quire-spec-language ADR-027 CI-1 to CI-3 and QS-5;
  invariant/variant loops: ADR-027 PS-6. Linear STD-140, paired QSL-396.
