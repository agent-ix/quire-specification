---
id: FR-383
title: "Explore a model subject under state-space reduction"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-specification/FR-013
    type: references
  - target: ix://agent-ix/quire-specification/FR-382
    type: references
  - target: ix://agent-ix/quire-specification/FR-384
    type: references
  - target: ix://agent-ix/quire-specification/FR-385
    type: references
  - target: ix://agent-ix/quire-specification/FR-336
    type: references
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
---

# FR-383: Explore a model subject under state-space reduction

## Description

When a model-check item carries an admitted reduction selection, the
model-check engine SHALL explore the product breadth-first under that
selection and SHALL return only concrete counterexamples. Under symmetry it
stores canonical representatives, under partial-order reduction it expands
ample sets, and under a state constraint it leaves boundary states
unexpanded.

## Inputs

A model-check item with a selection admitted under
[FR-382](FR-382-select-state-space-reductions.md), the model subject, the
claim's property automaton and the caller's limits.

## Outputs

A retained reduced product for the SCC and fairness phases, its reduction
record, and concrete counterexamples.

## Behavior

**Scope.** Reductions apply to model checking only.
[FR-181](../runtime/FR-181-simulate-finite-executions.md) simulation keeps its
unreduced exploration contract and reports each reachable state. The
automaton component of the product is never reduced.

**Sort canonicaliser.** For each class of the run's group (the group `G` of
FR-382, or a binding's stabiliser), the **sort key** of a key `k` is the JCS
bytes of the key-free encoding of the object `k` names: its FR-181 typed
record with every reference into a symmetric class replaced by that class's
placeholder `{"type":"symmetric-placeholder","class":<index>}`. A key with no
object in the state sorts as the empty byte string. The class's objects are
sorted ascending unsigned-lexicographic by sort key, equal sort keys keeping
their keys' order, and the class's keys are reassigned to them in key order.
The result is a permutation `π` in the group; the canonical form is `π(s)`,
and the canonicaliser returns both. Two states coalesce exactly when their
canonical forms have equal state-key bytes. Every canonical form is a member
of its state's orbit, so no reachable behaviour is lost; the reduced graph
holds at least one state per reachable orbit and never more states than the
unreduced graph. The canonicaliser is a function of the state, so
exploration stays deterministic.

**Binding orbits.** A claim with an `over` parameter has one instance per
object of the universe. Instances whose bound keys lie in one orbit of `G`
have the same verdict. The engine checks the instance with the least key in
each orbit, explores it under the stabiliser of its bound key, and reports
its verdict for every instance of the orbit.

**Locations.** Each transition identity `t` of a system that offers
partial-order reduction has a write footprint `W(t)` and a read footprint
`R(t)`, sets of locations. A model location is one of:

- `Field{o, f}`: field `f` of the resolved object `o`;
- `AnyField{T, f}`: field `f` of any object of type `T` or a type conforming
  to it;
- `Membership{P, k}`: whether the object with key `k` is a member of
  population `P`;
- `AnyMembership{P}`: the membership of any object of population `P`.

Two locations **meet** when they are equal; when one is `AnyField{T, f}` and
the other is `AnyField{T', f}` or `Field{o, f}` over a type related to `T` by
conformance; when one is `Membership{P, k}` and the other is
`AnyMembership{P}`, any `Field{k, f}`, or `AnyField{T, f}` for a field `f`
of `P`'s member type; or when one is `AnyMembership{P}` and the other is any
`Membership{P, _}`, or any `Field` or `AnyField` of a field of `P`'s member
type. A protocol transition system supplies its own protocol locations and
their meets.

**Write footprint.** `W(t)` is the frame of `t`'s operation
([FR-013](../foundation/FR-013-validate-operation-frames.md)), at each
entry's scope:

- a receiver-scoped `modifies self.f` gives `Field{receiver, f}`;
- an every-object `modifies f` gives `AnyField{T, f}` for the declaring
  type `T`;
- a receiver-scoped `deletes self` gives `Membership{P, receiver}`, and an
  argument-scoped `deletes p` gives `Membership{P, argument of p}`, for each
  population `P` whose member type the deleted object's type conforms to;
- a type-wide `deletes T`, which permits deleting any subset of the
  conforming objects, gives `AnyMembership{P}` for each population `P` of a
  type conforming to `T`;
- a `creates T` gives `AnyMembership{P}` for each population `P` of a type
  conforming to `T`, because the engine chooses the created key.

**Read footprint.** `R(t)` is the union of:

1. every location a precondition reads;
2. every location a postcondition reads in the pre-state, through `pre(...)`;
3. every location a postcondition reads in the post-state that `W(t)` does
   not contain;
4. `Membership{P, receiver}` and `Membership{P, argument}` for each
   reference argument, for each population `P` whose member type the
   object's type conforms to;
5. for each `deletes` entry, `AnyField{U, g}` for every reference-valued
   field `g` of every type `U` whose referenced type the deleted type
   conforms to, including a field holding a collection of references: a
   post-state is admissible only when no reference in it names a deleted
   object, so a delete is enabled only while no such field names it;
6. for each `creates T` entry, `AnyMembership{P}` for each population `P`
   of a type conforming to `T`, and `AnyMembership{Q}` for the target
   population `Q` of each reference-valued field of `T`; for each granted
   write to a reference-valued field, `AnyMembership{Q}` for its target
   population `Q`: a post-state is admissible only when every reference in
   it names a member.

A clause expression reads locations by these rules:

- `self.f` reads `Field{receiver, f}`, and `p.f` for a reference parameter
  `p` reads `Field{argument of p, f}`;
- a navigation `e.g.f` (a field of a field, or `deref(e).f`) reads every
  location `e.g` reads, which includes every intermediate reference field
  along the path, and `AnyField{T, f}` for `g`'s target type `T`; so
  `self.parent.f` reads `Field{receiver, parent}` and `AnyField{T, f}`;
- `reaches(x, y, f)` reads the locations of `x` and `y` and `AnyField{T, f}`;
- a field `f` read on a quantified variable, a binder of `filter` or `map`,
  or a collection element reads `AnyField{T, f}` for the variable's or
  element's type `T`, and the collection being traversed reads its own
  locations;
- a quantification, aggregation (`count`, `sum`) or membership test over a
  population `P`, or `allInstances` of its type, reads `AnyMembership{P}`;
- `result` ranges over the result type's domain in the post-state, so a
  field `f` read through `result` reads `AnyField{T, f}` for the result's
  object type `T`, and a reference-valued `result` reads `AnyMembership{P}`
  for every population `P` its type's objects belong to;
- a call `h(a1, …, an)` of a pure function
  ([FR-146](../expressions/FR-146-check-total-pure-functions.md))
  reads the locations each argument reads and every location the callee's
  body reads, transitively through the calls the body makes; in the body, a
  field read through a reference parameter bound to `self` or to an
  operation parameter reads that object's `Field`, and one bound to any
  other argument reads `AnyField{T, f}` for the parameter's type `T`; so
  `hot(self.cell)` with `function hot(c: Reference<Cell>): Boolean =
  deref(c).v = 1` reads `Field{receiver, cell}` and `AnyField{Cell, v}`;
- a dispatched call reads, beside its arguments' locations, the reads of
  every candidate's body and effective precondition.

The checker derives `R` and `W` per operation from its checked clause bodies
and frame with the receiver and parameters symbolic; the engine instantiates
both per transition identity. A protocol transition system supplies its own
write and read footprints.

**Enabling footprint.** A model transition is enabled when its receiver and
reference arguments are members of their populations, its precondition
holds, and some admissible post-state within its frame satisfies its
postcondition. Its **enabling footprint** is every location that
enabledness reads, which is all of `R(t)`: the precondition's reads, the
postcondition's pre-state and post-state reads, the receiver's and reference
arguments' membership, and the referrer and target-membership reads of items
5 and 6. A protocol step's enabling footprint is the set its protocol
transition system supplies, and it holds at least: the reads of the
operation's precondition and of its postcondition in the pre-state, the
membership of its receiver and reference arguments, every binder its
arguments read, its own thread's control and instance locations, and, for a
`duplicate` or `lose` step on channel `h`, `queue(h)`.

**Fairness class.** A fairness class is the set of transition identities one
constraint of the claim's fairness set covers, authored or derived: the
transitions of one operation or attempt node for a `whole` constraint, one
transition identity or the steps of one thread for an `each` constraint, and
the steps of the thread target of each default scheduler constraint of a
protocol. A transition that would add a member to a class's population, or
remove one, writes that class's members' enabling footprints.

**Enforced footprints.** Writes are enforced by frame admission at each
entry's scope (FR-013). Reads are enforced by evaluating each clause body for
the engine over an observation restricted to `R(t)`; a read outside it is an
internal fault that settles the item `failed`, never a value.

**Independence and visibility.** `t` and `u` are independent when
no location of `W(t)` meets a location of `R(u) ∪ W(u)` and no location of
`W(u)` meets a location of `R(t)`. So deleting an object is dependent on
every transition whose receiver or reference argument is that object, and on
every transition that writes a reference field that could name it. A transition is visible when its write footprint meets a
location some atom of the claim reads or a location the state constraint
reads, or when an atom reads the operation anchor of its position and names
its operation. Under a non-empty fairness set a transition is also visible
when its write footprint meets the enabling footprint of a member of a
fairness class it does not belong to; a `creates` that adds a member to a
`whole` class is therefore visible. The deadlock-freedom item's predicate
makes no transition visible.

**Ample sets.** At a product state with model state `s` and full enabled set
`E(s)`, the engine expands `A(s) ⊆ E(s)` that satisfies:

- **C0** `A(s)` is empty only when `E(s)` is;
- **C1** no transition dependent on a member of `A(s)` can run from `s`
  before a member of `A(s)` runs;
- **C2** when `A(s) ≠ E(s)`, every member is invisible;
- **C3** when `A(s) ≠ E(s)`, every successor through `A(s)` is a product
  state not yet discovered.

For each enabled `t` in canonical transition order, the candidate seeded by
`t` is the least set containing `t` that holds, for each enabled member,
every transition identity dependent on it, and for each disabled member,
every transition identity whose write footprint meets that member's enabling
footprint. `A(s)` is the enabled part of the candidate with the fewest
enabled members that satisfies C2 and C3, ties broken by the seed's canonical
order, or `E(s)` when none does. A visible transition is never a member of
an ample set smaller than `E(s)`.

**Breadth-first cycle proviso.** Exploration is breadth-first, so C3 takes its
breadth-first form: every cycle of the reduced product contains an edge into
a product state discovered earlier, and the source of that edge is fully
expanded. Every cycle therefore has a fully expanded state, checked on
product states.

**Boundary states.** The engine expands a state only when the state
constraint holds there. A reached state where it is false is a boundary
state: it is discovered, keyed, read by the property automaton and by the
safety monitor, and never expanded. A boundary state is never terminal, never
stutter-extended and never a deadlock. Enabledness at expanded states is
computed from the model and is unaffected by the cut. A constraint that
evaluates undefined at a reached state settles the item `refuted` with cause
`undefined-evaluation` at that state, with a counterexample ending there that
replays; that state is not a boundary state. Replay re-evaluates the
constraint at the reported state, so the prefix reproduces the undefined
value.

**Combination.** Symmetry and partial-order reduction combine: the engine
computes `A(s)` on the canonical representative, canonicalises each
successor and applies C3 to canonical product states. A constraint combined
with symmetry is subject to FR-381's check; combined with partial-order
reduction, the locations it reads are visible.

**Expansion and limits.** For each product state in breadth-first order the
engine computes the enabled set, picks `A(s)` or takes `E(s)`, applies each
member, canonicalises each successor and keeps its permutation, marks
boundary states, applies C3 against the discovered set, and retains each edge
as (source, transition identity, canonical target, permutation). The SCC and
fairness phases run on the retained graph. The caller's state limit counts
stored product states, boundary states included.

**Concrete counterexamples.** The engine chooses the canonical counterexample
over the reduced graph and concretises it before returning it. Under
symmetry it keeps `σ` with concrete state `σ(canonical state)`: it starts at
the first initial state in subject order whose canonical form is the stem's
first state, with `σ = π⁻¹` for that state's canonicalising permutation; each
retained edge (source, `t`, target, `π`) becomes the concrete step `σ(t)` and
`σ` becomes `σ ∘ π⁻¹`. A lasso's loop is repeated until the concrete state
equals the concrete entry state, at most the order of the loop's closing
permutation times; with an `each` constraint the loop is built as
[FR-384](FR-384-decide-each-fairness-on-annotated-quotient.md) states. A
partial-order or constrained path is already a path of the subject. A
counterexample carries no canonical state, permutation or reduction; it is
an ordinary model trace that replays through the model's successor relation
with fairness re-established.

**Reduced-versus-unreduced agreement.** A reduction changes how much of the
product is explored and never a verdict. For every model-check case of the
[FR-336](../../objects/interfaces/FR-336-conformance-corpus.md) conformance
corpus, and for every selection of symmetry, partial-order reduction, or both
whose [FR-385](FR-385-report-reduced-verdicts.md) preservation rows all read
"yes" for the case's claim form and fairness set, the engine SHALL settle the
reduced run and the unreduced run of the same claim with the same FR-341
label. Symmetry applies to a case that declares a `symmetric` population and
partial-order reduction to a subject whose transitions have footprints. When
both runs settle `refuted`, each counterexample SHALL replay through the
model's successor relation; the two counterexamples may differ. When both
settle `proved`, the reduced run names proof basis `reduced` and the
unreduced run `exhaustive`. A case with a state constraint, and a pair in
which either run ends at its search horizon or at a resource limit, is not
compared, because neither settles what the subject satisfies. A pair that
settles different labels is a conformance failure of the engine.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-383-AC-1 | The sort canonicaliser maps each state to a member of its orbit with the stated sort key and tie rule, returns its permutation, and coalesces exactly the states whose canonical forms have equal state-key bytes; the reduced graph never holds fewer states than reachable orbits nor more than the unreduced graph. | Test (TC-321) |
| FR-383-AC-2 | Under full symmetry of three configs, the reduced model graph holds 10 states where the unreduced one holds 27, one `ReachesTwo` instance is checked for the orbit of `a`, and its product under the stabiliser of `a` holds 30 states where the unreduced instances hold 135. | Test (TC-321) |
| FR-383-AC-3 | Write and read footprints are derived from frames at entry scope and from clause bodies as stated, and a clause read outside the read footprint settles the item `failed`, never a value. | Test (TC-326) |
| FR-383-AC-4 | Every expanded set satisfies C0 to C3 with the breadth-first cycle proviso, and the choice among candidates is the one stated; under partial-order reduction the three independent counters reduce from 27 model states and 54 transitions to 7 states and 6 transitions for the instance `x = a`. | Test (TC-323) |
| FR-383-AC-5 | A visible transition, including one made visible under a non-empty fairness set because it writes the enabling footprint of a member of another fairness class, is never a member of an ample set smaller than the full enabled set. In the `Gate` unit `arm` and `arm2` are visible under `fair weak go`, both when `go`'s guard is its precondition and when the guard is in its postcondition, and the item settles `refuted` with the loop `toggle, toggle`. | Test (TC-325) |
| FR-383-AC-6 | A state where the constraint is false is stored, keyed and read by the automaton and the safety monitor, and is never expanded, stutter-extended or reported as a deadlock; the caller's state limit counts it; a constraint that evaluates undefined at a reached state settles `refuted`, cause `undefined-evaluation`, and that state is not a boundary state. | Test (TC-324) |
| FR-383-AC-7 | Every counterexample from a reduced search is a concrete model trace with no canonical state, permutation or reduction, and replays through the model's successor relation; a symmetry lasso whose loop closes only after more than one repetition is repeated until the concrete state equals the entry state. | Test (TC-321, TC-322) |
| FR-383-AC-8 | A simulation request over the same subject reports every reachable state, unaffected by any reduction selected on a model-check item. | Test (TC-327) |
| FR-383-AC-9 | A model transition's enabling footprint is its whole read footprint, including the postcondition's pre-state reads, its post-state reads outside the write footprint, and its receiver's and reference arguments' membership; a writer that enables a transition only through its postcondition is in the candidate of that disabled transition, so the reduced search reaches the violation the unreduced search reaches. | Test (TC-323) |
| FR-383-AC-10 | `deletes self` writes the receiver's membership and `deletes p` the argument's; a type-wide `deletes T` and every `creates T` write `AnyMembership{P}`, which meets every membership and field location of `P`'s member type; each is dependent on every transition whose receiver or reference argument is an object it may delete, and no ample set drops that object's visible transition. | Test (TC-323) |
| FR-383-AC-11 | A navigation read `self.cell.v` reads `Field{receiver, cell}` as well as `AnyField{Cell, v}`, so a transition that rewrites `cell` is in the candidate of a disabled transition whose precondition navigates through it, and the reduced search reaches the violation the unreduced search reaches. | Test (TC-323) |
| FR-383-AC-12 | A `deletes` entry reads `AnyField{U, g}` for every reference-valued field `g` whose referenced type the deleted type conforms to, so a transition that clears a reference to an object is in the candidate of the disabled delete of that object, and the reduced search reaches the violation the unreduced search reaches. | Test (TC-323) |
| FR-383-AC-13 | A field read on a quantified variable, a `filter` or `map` binder or a collection element reads `AnyField{T, f}`, a quantification or aggregation over a population reads `AnyMembership{P}`, a `creates` and a granted write to a reference field read the target populations' `AnyMembership`, and `Membership{P, k}` meets `AnyField{T, f}` for a field of `P`'s member type: `forall(o in allInstances<Counter>(M::counters) : o.v < 2)` in a state clause reads `AnyField{Counter, v}` and `AnyMembership{counters}`, and is dependent with every `bump` and every delete of a counter. | Test (TC-326) |
| FR-383-AC-14 | A postcondition that reads a field through a reference-valued `result` reads `AnyField{T, f}` and the result population's `AnyMembership`, so a transition that writes that field on any object is in the candidate of the disabled operation, and the reduced search reaches the violation the unreduced search reaches. | Test (TC-323) |
| FR-383-AC-15 | For every model-check case of the FR-336 conformance corpus and every selection of symmetry, partial-order reduction or both that FR-385's preservation table admits for the case, the reduced run and the unreduced run settle the same FR-341 label, and every counterexample from either run replays; the vectors of TC-321 to TC-325 are such cases, and each settles the same label reduced and unreduced. | Test (TC-329) |
| FR-383-AC-16 | A precondition `self.armed and hot(self.cell)`, with `hot` a pure function whose body reads `deref(c).v`, reads `Field{receiver, cell}` and `AnyField{Cell, v}`; a call's reads include its callee's body reads transitively and a dispatched call's reads every candidate's, so a reduced run refutes wherever the unreduced run refutes. | Test (TC-323) |

## Dependencies

- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the
  unreduced exploration contract, state keys and canonical transition order.
- [FR-013](../foundation/FR-013-validate-operation-frames.md) owns frame
  entries and their scope.
- [FR-382](FR-382-select-state-space-reductions.md) admits the selection.
- [FR-384](FR-384-decide-each-fairness-on-annotated-quotient.md) owns `each`
  fairness on the retained quotient.
- [FR-385](FR-385-report-reduced-verdicts.md) owns the verdicts.
- [FR-336](../../objects/interfaces/FR-336-conformance-corpus.md) owns the
  conformance corpus the agreement requirement runs over.

## References

- Design: quire-spec-language ADR-021 (RD-2, SYM-6, SYM-7, POR-1 to POR-11,
  SC-2, EI-2 to EI-7, RV-10, QS-2, QS-7, RU-2); ADR-018 FA-2 (enabledness); ADR-027
  FT-1 and FT-3 (protocol footprints and enabling footprints).
- Review: SR-810 FND-001, FND-003; SR-811 FND-001, FND-002, FND-005;
  quire-spec-language SR-1140 FND-001, FND-002, FND-004, FND-006 to FND-010.
- Tickets: Linear STD-134; paired QSL-368.
