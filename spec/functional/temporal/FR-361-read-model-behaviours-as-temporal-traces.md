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
item with zero instances.

## Behavior

### Model subject

A **model subject** is the tuple (checked package, ordered list of initial
states, universe of each population, substituted proof bounds). Its successor
relation is the successor rule of
[FR-181](../runtime/FR-181-simulate-finite-executions.md): an operation applied
to a receiver and an argument vector from the operation's finite declared
parameter domains, enabled when the operation's effective precondition holds,
times every post-state within the operation's declared frame that satisfies
its postcondition. Each transition carries its FR-181 transition identity,
which names the operation, the receiver and the argument vector. The subject is
part of every verdict's scope and of every obligation identity that reads it
([FR-368](FR-368-negotiate-temporal-model-check-items.md)).

The `requires-bound` classification below runs first. Only over bounded
universes is an item with zero instances found, an item having zero
instances when no population overlaps its `over` type or every overlapping
population has an empty universe (no overlapping population has a key); a
overlapping population with no universe is `requires-bound`, never
`no-instance`. A key whose object does not conform to the type is still an
instance (below), so it never makes the item `no-instance`. The
analyzer SHALL settle it `inconclusive` with cause `no-instance` before it
explores any state, so an item that was never evaluated is never read as
holding.

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

A state at which no transition identity is enabled is **terminal**. Every
terminal state reads by this rule, whether the model intends it or it is a
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
a key has no type until a state holds its object. The `universes` of a subject
(FR-368) are these lists, one per population.

A universe provision is the RFC 8785 canonical form of a JSON array of the
population's keys in list order. Each key is a closed object with exactly two
members, `universe`, 64 lowercase hex digits, and `identity`, the lowercase hex
of the identity bytes (the spelling of FR-181's reference form, without its
`object_type`). The provision is named by the `sha256-jcs` digest of that
document. The form is this requirement's own: FR-181 supplies only the
spelling of the two members. For the population `counters` with the universe
list `[c, d]` in one universe `11…11` (64 hex digits `11` repeated), the
document is `[{"identity":"63","universe":"11…11"},{"identity":"64","universe":"11…11"}]`,
spelled in full in the published vector `counters-c-d`; the list `[d, c]` is a
different document with a different digest.

A clause `over (x: T)` over a model subject has one *instance* for each
distinct key that occurs in the universe of at least one population that
overlaps `T`: some member type of the population conforms to `T`, or `T`
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

The analyzer SHALL bind `x` for each instance to the reference (`universe`,
`object_type`, `identity`) whose `object_type` is the canonical member type of
the first overlapping population, in the order above, that lists the key (that
population's first member type in declaration order), and SHALL NOT compare the
type component. The analyzer SHALL match a state's object to the binding by
universe and identity alone. A refutation names the instance, its key, that it
refutes.

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
| FR-361-AC-8 | An item with zero instances, because no population overlaps its `over` type or every overlapping population has an empty universe, settles `inconclusive` with cause `no-instance` before any state is explored, and never `proved` or `refuted`; the same item over a population with no universe returns `requires-bound`, not `no-instance`; an item over `Counter` whose only population is `all: Entity = [e]` with `e` not a `Counter` has the instance (`all`, `e`) and is not `no-instance`. | Test (TC-294, TC-290) |
| FR-361-AC-9 | A clause `over (x: Counter)` over a subject with populations `counters` (member type `Counter`) and `all` (member type `Entity`, a supertype of `Counter`), universes `counters = [c, d]` and `all = [c, e]`, and `specials` (only member type `SpecialCounter`, a subtype of `Counter`) with `[s]`, has the instances `c`, `e`, `d`, `s` in that order, the key `c` listed by `all` and `counters` being one instance; a population of an unrelated type adds none. | Test (TC-290) |
| FR-361-AC-10 | The item is `refuted` exactly when some instance is refuted, for every permutation of the universe lists; the reported instance, counterexample, basis and cause are those of the first refuted instance in the instance order: with `all = [c, e]` and the unguarded claim `always holds(x.value <= 0)`, where `c` refutes by a false evaluation after `inc` and `e` by an undefined read, the cause is none and the counterexample names `c`, and with `all = [e, c]` the cause is `undefined-evaluation` and it names `e`; permuting a universe list changes the request identity. | Test (TC-290) |
| FR-361-AC-11 | Over `Counter` instances, the instance `e`, whose key holds an `Entity` that is not a `Counter`, reads `absent-key` through `x`: an unguarded `holds(x.value >= 0)` refutes with `undefined-evaluation` naming `e`, while the same read guarded by the disjunction of `lookup<Counter>(p, x)` being `some` over `all`, `counters` and `specials` evaluates for every instance; with `inc` disabled, `c.value = 1`, `s.value = 1` and `d.value = 0`, the claim `always holds(x.value >= 1)` guarded by the disjunction is refuted at `d`, while a guard naming only `all` holds it vacuously at `d` and the item is `proved`. | Test (TC-290) |
| FR-361-AC-12 | A binding's `object_type` is the population's canonical member type and is not compared: an object held under the key whose most-specific type is a subtype of that member type is matched, and reference equality of `x` with `allInstances` members of that object holds. | Test (TC-290) |
| FR-361-AC-13 | The universe provision of population `counters` with list `[c, d]` is the RFC 8785 canonical array of its two keys in list order, each exactly `universe` and `identity`, whose `sha256-jcs` digest is the published `counters-c-d` digest; the list `[d, c]` gives a different document and digest; a key with a missing, extra (`object_type` included) or non-hex member is not a universe key. | Test (TC-290) |

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
