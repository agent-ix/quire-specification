---
id: FR-361
title: "Read a model subject's behaviours as temporal traces"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/FR-090
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-161
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-362
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
---

# FR-361: Read a model subject's behaviours as temporal traces

## Description

When a temporal clause is checked over a model subject, the analyzer SHALL
read every behaviour of that subject as a temporal trace whose positions,
anchors, clock and terminal-state reading are the ones stated below.

## Inputs

A checked package with a state model; a non-empty ordered list of initial
states, each an admitted snapshot of the model's populations; the universe of
each population the subject reads; any proof bounds the request substitutes;
and one checked temporal clause with its selected profile.

## Outputs

The subject's behaviours as temporal traces, each with its positions and
their anchors; or `unsupported` for a profile the model subject does not
admit; or `requires-bound` for an unbounded root; or the FR-363 verdict
`inconclusive` with cause `no-initial-state` when the list of initial states
is empty; or the FR-363 verdict `inconclusive` with cause `no-instance` for an
item with zero instances over a non-empty list of initial states.

## Behavior

### Model subject

A **model subject** is the tuple (checked package, ordered list of initial
states, universe of each population, substituted proof bounds). Its initial
states are listed in FR-181's canonical initial-state order, ascending by
state-key bytes with equal keys coalesced, whatever order the request supplies
them in. Its successor
relation is the successor rule of
[FR-181](../runtime/FR-181-simulate-finite-executions.md): an operation applied
to a receiver and an argument vector from the operation's finite declared
parameter domains, enabled when the operation's effective precondition holds,
times every post-state within the operation's declared frame that satisfies
its postcondition. Each transition carries its FR-181 transition identity,
which names the operation, the receiver and the argument vector. The subject is
part of every verdict's scope and of every obligation identity that reads it
([FR-368](FR-368-negotiate-temporal-model-check-items.md)).

### Model selection of a subject

A model subject reads exactly the `model_root` nodes
([FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) "Model
root") of the domain packages that declare the populations of its universes; a
subject over a refinement item reads the row of the refinement's concrete model.
A subject whose universes name populations of one domain package reads one row,
and its `model_root` node is the subject's anchor
([FR-368](FR-368-negotiate-temporal-model-check-items.md) "Claim and anchor of
a derived or refinement item"). A subject whose universes name populations of
several domain packages reads several rows, and a subject that names no
population of any selected domain package (an empty set of universes, or
populations that no `model_root` node selects) reads none. A subject that
reads several rows or none has no `model_root` anchor, so the producer settles
each derived item over it `unsupported` as FR-368 states and writes no request
item for it. This settlement is the producer's and comes first for a derived
item: the item is never written, so `requires-bound`, `no-initial-state`,
`conflicting-identity` and `no-instance` (below) do not arise for it, whatever
its universes hold, and a subject of zero universes reads no row. An authored
`formula` item has its own claim anchor, so a subject that reads no row does not
stop it, and it settles by the order below.

The `requires-bound` classification below runs first. Only over bounded
universes is an item with zero instances found, an item having zero
instances when it has no admitted key (below): no population overlaps its
`over` type, every overlapping population has an empty universe, or no member
record of a non-empty list of initial states admits a key of their
universes. A population overlapping the type that has no universe makes the
item `requires-bound`, never `no-instance`. A key whose object does not
conform to the type is still an instance (below), so it never makes the item
`no-instance`. Over a non-empty list of initial states, the analyzer SHALL
settle it `inconclusive` with cause `no-instance` before it
explores any state, so an item that was never evaluated is never read as
holding.

An empty list of initial states holds no member record, so it admits no key
and every item over that subject also has zero instances. The analyzer SHALL
settle such an item `inconclusive` with cause `no-initial-state`, never
`no-instance`. This order applies to authored `formula` items; a derived item over a subject that reads no model selection or several settles per FR-361-AC-17 ("Model selection of a subject") and never reaches it. The order of these outcomes is: the `requires-bound`
classification first, then `no-initial-state` for an empty list of initial
states, then the cross-population `conflicting-identity` check below, then
`no-instance`. So `no-instance` names an item whose subject has an initial
state and no admitted key, and `no-initial-state` names a subject with no
behaviour at all, whatever its universes hold.

Before any expansion, the analyzer SHALL classify every root of the subject
under the request's universes. An unbounded population root, parameter domain,
result domain or field domain returns the FR-181 `requires-bound` negotiation
disposition, which the caller answers with a universe or a bounded domain.

### Behaviours and positions

A **behaviour** is a maximal path of the successor relation from an initial
state. The analyzer SHALL number its positions from 0:

| Position | State | Anchor |
| --- | --- | --- |
| 0 | the initial state | `initialization` |
| `i > 0` | the post-state of the behaviour's `i`-th transition | that transition's operation anchor |

Each position is observed as the synthesized state observation of its state,
anchored as the table states. A `holds` atom SHALL evaluate through the same
clause evaluator that evaluates state clauses, over the observation at its
position. The step sequence is the event-position sequence authority of every
profile the subject admits.

### Profiles and clock

A model subject admits exactly two profiles:
`quire.temporal.event-position.false-extension/v1` and
`quire.temporal.infinite-trace/v1`. Under both, one tick is one step of the
behaviour, and the clause's clock role binds to the subject's step sequence
([FR-090](FR-090-select-temporal-profile-and-clock.md)). A clause under any
other profile over a model subject settles `unsupported`,
`unsupported_projection`/`unsupported-requested-capability`, at negotiation,
because a model behaviour carries no sample period or timestamp.

### Terminal states

A state at which no transition identity is enabled is **terminal**; over a
timed subject a timed state is terminal as
[FR-181](../runtime/FR-181-simulate-finite-executions.md) "Timed subjects"
states (no discrete step enabled at it or after any admissible delay). Over a
timed subject only a terminal state in which time can pass without bound (the
idle tail of [FR-161](FR-161-analyze-infinite-trace-properties.md)) is
stutter-extended; a bounded-delay terminal state has no time-divergent
continuation, so no admitted infinite behaviour passes through it and claims at
it hold vacuously (FR-161). Every other terminal state reads by this rule, whether the model intends it or it is a
deadlock ([FR-366](FR-366-check-deadlock-freedom.md)).

- Under the event-position false-extension profile, a behaviour that reaches a
  terminal state is a closed finite execution. The analyzer SHALL read it with
  that profile's closed-boundary rule: atomic predicates are false after the
  last position and constants keep their value.
- Under the infinite-trace profile, a behaviour that reaches a terminal state
  is extended by a **terminal stutter step** that repeats the terminal state
  forever. The stutter step has the transition identity
  `{"type":"terminal-stutter"}`. It enables no operation, takes no fairness
  constraint, and lies outside every fairness constraint
  ([FR-362](FR-362-apply-fairness-constraints.md)). Each stutter position is
  observed as the terminal state's observation, anchored by the stutter step.

### Binding of the clause's `over` parameter

A *universe* of a population is an ordered list of object keys, each a
(universe component, object identity) pair of the FR-204 triple with no type:
a key has no type until a member record admits it. The `universes` of a subject
(FR-368) are these lists, one per population.

A universe provision is the RFC 8785 canonical form of a JSON array of the
population's keys in list order. Each key is a closed object with exactly two
members, `universe`, 64 lowercase hex digits, and `identity`, the lowercase hex
of the identity bytes (the spelling of FR-181's reference form, without its
`object_type`). The provision is named by its population name. The form is this requirement's own: FR-181 supplies only the
spelling of the two members. For the population `counters` with the universe
list `[c, d]` in one universe `11…11` (64 hex digits `11` repeated), the
document is `[{"identity":"63","universe":"11…11"},{"identity":"64","universe":"11…11"}]`,
spelled in full in the published vector `counters-c-d`; the list `[d, c]` is a
different document.

A clause `over (x: T)` over a model subject has one *instance* for each
distinct *admitted key*: a key that occurs in the universe of at least one
population that overlaps `T` and that a member record of some initial state of
the subject admits, which fixes the key's type up front. A key only an operation
could create is not an instance; that is the stated limitation of this rule.
The item's **excluded keys** are the universe keys of the populations that
overlap `T` that no member record of any initial state admits, in instance
order, a key in several overlapping universes being listed once. A verdict over the item covers its instances only: a `proved` item over
a model whose operation creates an object under an excluded key has not
checked that object, and a `refuted` item never names an excluded key. The
excluded keys are a function of the subject the verdict carries (its
universes and initial states, FR-363 "Scope"), so the verdict determines them
without a further wire member.
A population *overlaps* `T` when some member type of the population conforms to `T`, or `T`
conforms to some member type of it. A population whose member types are
subtypes of `T` only, and one whose member type is a supertype of `T`, each
overlap `T`; one whose member types are unrelated to `T` does not. A key listed
by several overlapping populations is one instance, the object `x` denotes
being the object with that universe and identity. The analyzer SHALL order the
instances by first occurrence in the sequence that lists the overlapping
populations by the UTF-8 bytes of their declaration identity text and each
universe in its list order. The reported instance of a refuted item is the
first refuted instance in that order, and the counterexample, the basis and
the cause are those of that instance
([FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md)). The item is
`refuted` exactly when some instance is refuted, whatever the order, and the
clause holds exactly when every instance holds; but when instances refute by
different causes, the cause follows the order, so the order is normative and
enters the request identity through the universe lists the subject carries.
An instance is identified by its key alone, which is why the wire forms that
name an instance by parameter and key (the replay `over_binding`, the evidence
`Instance` and the obligation identity) need no population member.

The analyzer SHALL bind `x` at each position to the reference (`universe`,
`object_type`, `identity`) of the object the state holds under the instance's
key, its `object_type` being that object's most-specific type (FR-204), never a
population's member type. That type is unique per key, because FR-153 refuses
two members with one universe and identity and different types
(`conflicting-identity`), so the reference does not depend on which overlapping
population lists the key or on the population order, and reference equality
(FR-204) holds between `x` and the same object reached from any population. A
key has no type until a member record admits it, so the instance itself names
none. A refutation names the instance, its key, that it refutes.

Every temporal clause names an `over` parameter, so the clause is checked once for each of its instances (one per admitted key, above); a claim that reads no one object still names one, and each check evaluates the same formula. The cost of that rule is that such a claim's decidability depends on the admitted keys of the named type: when the item has no admitted key (no population overlaps the named type, every overlapping population has an empty universe, or no member record of a non-empty list of initial states admits a key of their universes), FR-361-AC-8 settles it `inconclusive`/`no-instance` before any exploration although the formula never reads the parameter, so an author names a type for which the subject has at least one admitted key.

FR-153 checks one type per key within one population binding, so the analyzer
SHALL check it across populations: after the `requires-bound`
classification, which comes first because a population with no universe has no
member records to compare, it compares the member records of every overlapping
population in every initial state of the subject, and two member records with
the same universe and identity and different types, in one population or in
two, in one initial state or in two, refuse the item
`invalid_runtime_input`/`conflicting-identity`, as every other per-item input
refusal does, before any instance is formed. A key that no member record of any initial state of the subject admits
is not an instance. The wire reference of a
binding carries the type of the member record admitted for its key, unique by
the rule above; a position where the state holds no object under the key still
carries that type, the object being absent.

The declared type of `x` is `Reference<T>`, and an instance whose admitted
record has a type that does not conform to `T` (an `Entity` that is not a
`Counter`) is allowed: `x` is bound to (`universe`, `Entity`, `identity`), and
every use of `x` other than the `lookup` guard is undefined with reason
`absent-key`; behind the guard it is never read. The same holds at a position
where no object is held under the key of an instance whose record conforms to
`T`: every use of `x` other than the guard is undefined with reason
`absent-key`.

At a position where no overlapping population holds an object under the
instance's key, or the object it holds does not conform to `T`, a read through
`x` SHALL return the kernel's undefined value with reason `absent-key`, the
same value `lookup<T>(p, x) absent undefined` returns for that binding. For a
binding that `over` introduces, `lookup<T>(p, x)` is present exactly when `p`
holds an object under that key whose most-specific type conforms to `T`. The
claim evaluates undefined at that position, and the item settles `refuted`
with cause `undefined-evaluation`
([FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md)), naming the
instance, the position and the locus of the read.

A clause cannot name the population an object is found in, so the guard is the
disjunction over every population that overlaps `T`: `lookup<T>(p1, x)` being
`some` or `lookup<T>(p2, x)` being `some` or ... for each overlapping
population `p1`, `p2`, ... The guard is true for an instance exactly when some
overlapping population holds a conforming object under its key, which is
exactly when a read through `x` evaluates. A guard naming only one overlapping
population is false for the keys the others alone list and keeps their reads
from ever evaluating, which is not a guard against `absent-key` but a vacuous
hold; the rule adds no type-test operator and no population-free membership
form, so a key whose object does not conform to `T` refutes the item under an
unguarded read, whatever the property.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-361-AC-1 | Position 0 of every behaviour reads the initial state with the `initialization` anchor, and position `i > 0` reads the post-state of transition `i` with that transition's operation anchor; a `holds` atom evaluated at a position returns the clause evaluator's value over that position's observation. | Test (TC-290) |
| FR-361-AC-2 | A clause under `quire.temporal.fixed-sample.false-extension/v1` or `quire.temporal.timestamped-event.finite-window/v1` over a model subject settles `unsupported` with `unsupported-requested-capability` at negotiation; the event-position false-extension and infinite-trace profiles are admitted, with one tick per step. | Test (TC-290) |
| FR-361-AC-3 | One model with a reachable terminal state is read as a closed finite execution under event-position false-extension and as a terminal stutter loop under infinite-trace, and the same `always[0,4]` claim receives different verdicts under the two profiles. | Test (TC-290) |
| FR-361-AC-4 | The terminal stutter step carries the transition identity `{"type":"terminal-stutter"}`, enables no operation and takes no fairness constraint. | Test (TC-290, TC-291) |
| FR-361-AC-5 | A clause `over (x: T)` over a universe of two objects of `T` is checked once per object; it holds for the subject only when both checks hold, and its refutation names the refuted object. | Test (TC-290) |
| FR-361-AC-6 | An unguarded read through the bound object at a reachable position where that object is absent settles the item `refuted` with cause `undefined-evaluation` naming the binding, the position and the read's locus, the same outcome as the `lookup ... absent undefined` spelling of that read; the same read guarded by population membership evaluates. | Test (TC-290, TC-305) |
| FR-361-AC-7 | A subject with an unbounded population root and no universe returns `requires-bound` before expansion; a subject with an empty list of initial states settles `inconclusive` with cause `no-initial-state`. | Test (TC-294) |
| FR-361-AC-8 | An item with zero instances, because no population overlaps its `over` type, every overlapping population has an empty universe, or no member record of an initial state admits a key of their universes (a subject whose only population is `counters = [z]` with `z` held by no initial record), settles `inconclusive` with cause `no-instance` before any state is explored, and never `proved` or `refuted`; the same item over a population with no universe returns `requires-bound`, not `no-instance`; an item over `Counter` whose only population is `all: Entity = [e]` with `e` admitted as a non-`Counter` has the instance `e` and is not `no-instance`. | Test (TC-294, TC-290) |
| FR-361-AC-9 | A clause `over (x: Counter)` over a subject with populations `counters` (member type `Counter`) and `all` (member type `Entity`, a supertype of `Counter`), universes `counters = [c, d]` and `all = [c, e]`, and `specials` (only member type `SpecialCounter`, a subtype of `Counter`) with `[s]`, has the instances `c`, `e`, `d`, `s` in that order, the key `c` listed by `all` and `counters` being one instance; a population of an unrelated type adds none. | Test (TC-290) |
| FR-361-AC-10 | The item is `refuted` exactly when some instance is refuted, for every permutation of the universe lists; the reported instance, counterexample, basis and cause are those of the first refuted instance in the instance order: with `all = [c, e]` and the unguarded claim `always holds(x.value <= 0)`, where `c` refutes by a false evaluation after `inc` and `e` by an undefined read, the cause is none and the counterexample names `c`, and with `all = [e, c]` the cause is `undefined-evaluation` and it names `e`; permuting a universe list changes the request identity. | Test (TC-290) |
| FR-361-AC-11 | Over `Counter` instances, the instance `e`, whose key holds an `Entity` that is not a `Counter`, reads `absent-key` through `x`: an unguarded `holds(x.value >= 0)` refutes with `undefined-evaluation` naming `e`, while the same read guarded by the disjunction of `lookup<Counter>(p, x)` being `some` over `all`, `counters` and `specials` evaluates for every instance; with `inc` disabled, `c.value = 1`, `s.value = 1` and `d.value = 0`, the claim `always holds(x.value >= 1)` guarded by the disjunction is refuted at `d`, while a guard naming only `all` holds it vacuously at `d` and the item is `proved`. | Test (TC-290) |
| FR-361-AC-12 | The binding of `x` for the key `c`, listed by `all` (member type `Entity`) before `counters` (member type `Counter`), has `object_type` `Counter`, the most-specific type of `c`, and is equal under FR-204 to the reference to `c` in `allInstances<Counter>(counters)`; renaming `all` so that `counters` sorts first leaves the binding's bytes unchanged; an object of the subtype `SpecialCounter` held under `s` binds with `object_type` `SpecialCounter`; two members with one universe and identity and different types refuse `conflicting-identity` before any binding. | Test (TC-290) |
| FR-361-AC-13 | The universe provision of population `counters` with list `[c, d]` is the RFC 8785 canonical array of its two keys in list order, each exactly `universe` and `identity`, published as the vector `counters-c-d` and named by the population name `counters`; the list `[d, c]` gives a different document; a key with a missing, extra (`object_type` included) or non-hex member is not a universe key. | Test (TC-290) |
| FR-361-AC-14 | After the `requires-bound` classification, two member records with the universe and identity of `c` and different types, one in `all` as `Entity` and one in `counters` as `Counter`, in one initial state or in two initial states of the subject, refuse the item `invalid_runtime_input`/`conflicting-identity` before any instance is formed, as do two such records in one population; a population with no universe gives `requires-bound` first; one record per key with agreeing types in every initial state is admitted. | Test (TC-290) |
| FR-361-AC-15 | The instance `e` (an `Entity` that is not a `Counter`) is bound to (`universe`, `Entity`, `e`) under `x: Reference<Counter>`, and an unguarded read through it is `absent-key`; after `drop` removes `d`, the instance `d` still carries `Counter` from its admitted record, every use of `x` other than the guard is `absent-key`, and a universe key that no member record of any initial state admits is not an instance; with `inc` disabled, `c` and `d` at value 0 and an operation `make` that creates a `Counter` with value 1 under such a key `z` of `counters`, `z` is still not an instance, the item's excluded keys are exactly [`z`], and `always holds(x.value = 0)` is checked for `c` and `d` only and settles `proved`, although `z`'s object has value 1 at a reachable position. | Test (TC-290) |
| FR-361-AC-16 | A subject with an empty list of initial states and a bounded universe `counters = [c, d]`, whose item over `Counter` therefore has no admitted key, settles `inconclusive` with cause `no-initial-state`, never `no-instance`, `proved` or `refuted`; the same subject with an empty universe list for `counters` and with no population overlapping `Counter` each settles `no-initial-state` too; the same item over that subject with `counters` given no universe returns `requires-bound`, not `no-initial-state`. | Test (TC-294) |
| FR-361-AC-17 | A model subject reads the `model_root` nodes of the domain packages that declare the populations of its universes (a refinement item: the row of its concrete model); one row is the subject's `model_root` anchor, and a subject that reads several rows or none has no anchor, so each derived item over it settles `unsupported`, `unsupported_projection`/`unsupported-requested-capability`, with no request item written, before any of `requires-bound`, `no-initial-state` or `no-instance`; an authored `formula` item over such a subject is not affected. As a consequence, a derived item over a zero-universe subject (which reads no row) is `unsupported` with no anchor and no request item, not `no-initial-state` or `requires-bound`. | Test (TC-290) |

## Dependencies

- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the successor
  rule, the transition identity and the canonical breadth-first order.
- [FR-090](FR-090-select-temporal-profile-and-clock.md) and
  [FR-161](FR-161-analyze-infinite-trace-properties.md) own the profiles and
  the infinite-trace semantics read over these traces.
- [FR-362](FR-362-apply-fairness-constraints.md) owns fairness over the
  behaviours; [FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md)
  owns the verdict over them.

## References

- quire-spec-language ADR-018 SM-2 to SM-4 and QS-1, QS-2, QS-3 and QS-5;
  Linear STD-131 (paired with QSL-366).
