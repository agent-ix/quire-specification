---
id: FR-208
title: "Quire meaning id vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-150
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-151
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-152
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-153
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-154
    type: specifies
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
---
# FR-208: Quire meaning id vocabulary

## Description

When model intake binds a semantic IR type definition or population node, the
binder SHALL select the declaration's Quire meaning by the exact `meaning` id of
the node's embedded construct, as listed in this vocabulary, and SHALL NOT select it by the
definition's kind name or module.

QSpec owns every Quire meaning id and its spelling. A module names one of these
ids in the `meaning` of each construct it declares. A semantic IR 2.0.0 domain
package (agent-ix/filament-core-data#172) embeds that construct in its
`constructs` table, and each type definition's and population node's `kind` is
`{module, name}`, naming its construct.

## Values

| Value | Description |
| --- | --- |
| `quire.meaning.model.object-type/v1` | An object type: an identified type with fields, relationships, operations and invariants. It takes part in generalization, conformance, redefinition and dispatch (FR-150, FR-151) and has an object universe (FR-204). |
| `quire.meaning.model.value-type/v1` | A value type: a named type bound to a native value type, with no object identity. Fields may name it as their `typeRef` (FR-154). |
| `quire.meaning.model.variant-type/v1` | A variant type: a closed set of named cases. Normalization admits it as an effective type; a clause naming it is `unsupported_construct`/`declaration-form` (FR-150). |
| `quire.meaning.model.record-value-type/v1` | A record value type: a named record of one or more fields, none an identity field, equal to another value exactly when both have one most-specific type and every field is equal. Its invariants hold for every value of the type. |
| `quire.meaning.model.event-type/v1` | An event type: a record of one occurrence, with no identity field and one occurrence field whose type resolves to `Timestamp`. It has no equality rule, and its invariants hold for every value of the type. |
| `quire.meaning.model.state-machine/v1` | A state machine: context fields, operations with preconditions and postconditions, invariants over its context, a closed set of states and a transition relation of `from` state, `to` state, `trigger` operation, optional guard clause and emitted event types. No firing or current state is bound. |
| `quire.meaning.model.process/v1` | A process: an identified type whose runs are the objects of an object universe, with invariants and operation contracts as for an object type, and steps in declaration order, each with a step kind and the event types it consumes and emits. No execution is bound. |
| `quire.meaning.model.persistence-interface/v1` | A persistence interface: operation signatures, no fields, and the object types it persists. Structural: no storage law is bound. |
| `quire.meaning.model.namespace/v1` | A namespace: the types its members name and a vocabulary of terms. It declares no type. |
| `quire.meaning.model.population/v1` | A population: a membership view over listed member types with a `closed` or `open` extent (FR-153). |
| `quire.meaning.systems.interface/v1` | A systems Interface: ordered feature signatures, each a field or an operation (FR-152). |
| `quire.meaning.systems.part/v1` | A systems Part: owned by a composite type, with a declared type and typed multiplicity (FR-152). |
| `quire.meaning.systems.port/v1` | A systems Port: owned by a Part, with a direction, an Interface type and typed multiplicity (FR-152). |
| `quire.meaning.systems.connection/v1` | A systems Connection: two Port ends and a declared direction (FR-152). |
| `quire.meaning.systems.allocation/v1` | A systems Allocation: a source element (Part, Port or operation) and a target Part (FR-152). |

The model meanings are exactly the ten `quire.meaning.model.*` values, and the
systems meanings are exactly the five `quire.meaning.systems.*` values.
Fields, relationships, operations and clauses are members and carry no meaning
id.

## Business model meanings

The six business model meanings are `record-value-type`, `event-type`,
`state-machine`, `process`, `persistence-interface` and `namespace`, each
`quire.meaning.model.<name>/v1`. Each meaning binds only what the domain
package's IR members state. Where a member states no meaning, the meaning is
structural: intake binds the member as data and derives no behavior from it.
Intake (FR-154) and normalization (FR-150) refuse a node that breaks a rule
below with the refusal the rule names, and a package with a refused node yields
no effective view.

Rules common to the six meanings:

- **Declarations.** Each node yields exactly one original declaration keyed
  by the package identity and its IR node identity, `ix://<package>/<artifact
  id>`, and one per field, relationship, operation and clause member. States,
  transitions, steps, `persists`, `members` and `vocabulary` are data of that
  declaration, not members. An operation's precondition and postcondition are
  part of its operation declaration.
- **Object universe.** Only a process has an object universe (below), so a
  process may be a population member, a `creates` or `deletes` frame entry or
  a relationship end. No node of the other five meanings is any of these.
  Naming one there is `invalid_model_binding`/`malformed-declaration`.
- **Field types.** A field or parameter `typeRef` may name a record value
  type. Naming an event type, state machine, process, persistence interface
  or namespace is `invalid_model_binding`/`malformed-declaration`.
- **Relationships.** A process's `relationships[]` entries have object-type
  relationship meaning over its runs (FR-150, FR-151): each is one relationship
  declaration, its ends name object types or processes, and it is navigated in
  the universe of its ends. A `relationships[]` entry of a node of the other
  five meanings is `invalid_model_binding`/`malformed-declaration`. Links the
  lift does not carry into the IR, such as `owns` and `transitions_to`, bind
  nothing.
- **References.** A reference below that names no node of the package is
  `missing_declaration`/`missing-name`. A reference that names a node of the
  wrong meaning is `invalid_model_binding`/`malformed-declaration`.
- **Names.** Two states of one state machine, two transitions of one state
  machine with equal `from`, `to` and `trigger`, or two steps of one process
  with one name have one IR identity, and are
  `invalid_model_binding`/`conflicting-binding`.
- **Clauses in other languages.** A clause in a language other than `quire` is
  kept with its span and never evaluated.

### Invariants

A `quire` clause of a record value type, event type, state machine or process
is an invariant, except a state machine's guard clause. On a persistence
interface or namespace a `quire` clause is
`unsupported_construct`/`declaration-form`. The effective invariant of a type
is the conjunction of its own invariants with those of every ancestor
(FR-151).

- **Record value type and event type.** `self` is one value of the type, and
  the invariant holds for every value of the type: a value is a value of the
  type only when its effective invariant holds. It reads only that value's
  fields, and the fields of record values they hold. It reads no observation,
  so `pre` in it is `wrong_snapshot`/`forbidden-pre-read` (FR-042), and a read
  through a reference to an object is `wrong_snapshot`/`wrong-observation`.
  Its scope is every value of the type reachable through the fields of every
  object in each invariant observation, and through the parameters and results
  of each evaluated invocation, including collection elements, present optional
  values and record values nested in other record values; the invariant is
  evaluated once for each such value, with `self` bound to it. A false result is
  a logical violation of that clause: a completed Boolean `false`, which carries
  no diagnostic code or cause, because a logical violation is distinct from
  every refusal (FR-047, FR-206-AC-3). The violation is located by the FR-207
  runtime value path where the value was reached: root observation, root
  subject the object identity or the invocation's parameter or result binding,
  then one `field` step per field, one `index` step per collection element and
  one `option_value` step per present optional. The result depends only on the
  value, so equal values give equal results. An object-type invariant instead ranges over
  the objects of an object universe at a named observation and depends on
  object identity (FR-204). Under `quire.model.complete/v1` no evaluation reads
  an event value, because no field, parameter or firing carries one, so an
  event-type invariant is linked and type-checked but never evaluated.
- **State machine.** `self` is the machine's context: the values of its
  effective fields, bound as the State-machine context anchors below state. The
  invariant holds in the pre and the post observation of every invocation of
  every operation of the machine, whatever state the machine is in. No member binds the current state
  (agent-ix/spec-objects-business#14, agent-ix/filament-core-data#178), so an
  invariant is scoped to a state only through the context fields it reads.
- **Process.** A process is an identified type: its runs are the objects of
  its object universe, the universe of its generalization component (FR-204),
  each identified by its correlation key. A process may be a population
  member. `self` is one run, and the invariant has object-type meaning: it
  holds for every run in the population at each invariant observation (state
  contract).
- **Guards.** A clause that a transition's `guard` names is that transition's
  guard only. It is not an invariant, it is not conjoined to any
  precondition, and it is not evaluated. A clause is a guard exactly when some
  effective transition of the machine names it.

### State-machine context anchors

A state-machine context has no object identity and no object universe. This
named extension of the state-contract anchor table (state contract, Operation
anchors) supplies the operation, invocation, context and universe that the
state contract requires every contract and invariant input to name:

- **Context naming.** One evaluation input binds the single context of one
  machine instance, and that context is owned by the input. It is named by the
  machine's declaration key and the invocation identity of the input, never by
  an object identity. The universe the state contract requires is the context
  universe of that invocation: it holds that one context and no objects, and
  identity equality is never applied to it.
- **Contexts per observation.** Each of the invocation's pre and post
  observations holds exactly one context of the machine, a value of the
  machine's effective fields. An observation with no context of the machine is
  `invalid_model_binding`/`wrong-context`; one with two contexts of the
  machine is `invalid_runtime_input`/`duplicate-member`. Neither yields a
  Boolean.
- **Locations.** A value read from the context is located by an FR-207 runtime
  value path whose root is the observation and the invocation binding `self`,
  followed by one `field` step per context field.

The anchors of a state-machine clause:

| Clause | Context self | Parameters | Result | `pre(e)` |
| --- | --- | --- | --- | --- |
| Invariant | The one context in the invocation's pre observation, and separately the one context in its post observation; the invocation is the invariant's handler binding | No operation parameters | Unavailable | Refused |
| Precondition | The one context in the invocation's pre observation | Immutable declared inputs of that invocation | Unavailable | Refused |
| Postcondition | The one context in the invocation's post observation | Same immutable inputs | Exact declared post-only result | Reads the one context in the invocation's pre observation |

### Operation contracts

An operation's precondition is its `Pre:` line and its postcondition is its
`Post:` line, carried as the IR operation's `requires` and `ensures` and the
model's `Operation.pre` and `Operation.post`.

- **State machine and process.** Operations have object-type contract meaning
  (FR-151, state contract). The precondition reads the invocation's pre
  observation and the invocation's immutable inputs, and `pre` in it is
  `wrong_snapshot`/`forbidden-pre-read`. The postcondition reads the post
  observation, the same inputs and the declared result, and may read the pre
  observation through `pre(...)`. For a state machine `self` is the one context
  of the observation, bound by the State-machine context anchors; for a process `self` is the run. Frame entries follow
  the object-type Frames rule. Redefinition follows FR-151: the effective
  precondition is the disjunction and the effective postcondition the
  conjunction over redefined members.
- **Transitions.** An operation's contract is the same in every transition it
  triggers. A transition's guard is not conjoined to the trigger's
  precondition. A transition's `from` and `to` are not related to the
  invocation's pre and post observations, because no member binds the field
  that holds the current state (agent-ix/spec-objects-business#14,
  agent-ix/filament-core-data#178).
- **Other meanings.** An operation of a record value type is
  `unsupported_construct`/`declaration-form`. An event type declares no
  operations: an operation is `invalid_model_binding`/`malformed-declaration`.
  An operation of a persistence interface binds its name, parameters and
  return, and one with a precondition, a postcondition or a non-empty frame is
  `unsupported_construct`/`declaration-form`. A process's steps and its
  operations are unrelated: no `## Workflow` column names an operation.

### Generalization

A record value type, event type, state machine or process may specialize nodes
of its own meaning only. A `supertypes[]` entry naming a node of another
meaning, including an object type, and an object type's entry naming a node of
one of these meanings, is `invalid_model_binding`/`malformed-declaration` at
that entry. A persistence interface or namespace with a non-empty `supertypes`
is `unsupported_construct`/`declaration-form`, and its entries are not checked
further. The supertype graph is acyclic (`invalid_model_binding`/
`specialization-cycle`), and inheritance, subsetting and redefinition of fields,
operations and clauses follow FR-150 and FR-151 for every specializable
meaning. A type conforms to each of its ancestors (FR-151). A required member
that the meaning states, `fields`, `identityFields`, `occurrenceField` or
`operations`, is present when the node or one of its ancestors declares it.

- **Record value type.** A subtype inherits every field and invariant and may
  add fields. A value of the subtype is a value of each ancestor wherever a
  field, parameter or result names the ancestor. Equality compares the
  most-specific type and every effective field, so a subtype value never equals
  a value of a proper ancestor.
- **Event type.** A subtype inherits every field, invariant and the occurrence
  field. Its own `occurrenceField` names a field of the type or of an ancestor.
  Each ancestor's occurrence field is the subtype's occurrence field or a field
  that the subtype's occurrence field redefines; otherwise
  `invalid_model_binding`/`derivation-conflict` in phase 3 with both paths. An
  `emits` or `consumes` entry naming an event type admits its subtypes.
- **State machine.** A subtype inherits every field, operation, invariant and
  guard clause, every state and every transition. Its effective states and
  transitions are the union of its own with those of every ancestor. A state
  name, or a transition's `from`, `to` and `trigger`, declared by two distinct
  owners reaching the subtype is `invalid_model_binding`/`derivation-conflict`
  in phase 3; one declaration reached along several paths is one. A
  transition's `from`, `to`, `trigger` or `guard` that names no state,
  operation or clause of any state machine of the package is
  `missing_declaration`/`missing-name` at intake; one that names no effective
  state, operation or clause of its own machine is
  `invalid_model_binding`/`malformed-declaration` in phase 3. A transition of
  a subtype whose `guard` names a clause that is an invariant of an ancestor,
  because no transition of that ancestor names it, is
  `invalid_model_binding`/`malformed-declaration` in phase 3 at that
  transition's `guard`, since it would remove that invariant from the subtype.
  Because
  the subtype's states and transitions include every ancestor's, it conforms
  to each ancestor.
- **Process.** A subtype inherits every field, identity field, operation and
  invariant with object-type generalization meaning (FR-150, FR-151), so a run
  of the subtype is a run of each ancestor in one object universe (FR-204).
  Steps are not inherited: each process's steps are its own, in its own
  declaration order, because the format has no place for a subtype's steps
  relative to inherited ones (agent-ix/spec-objects-business#14,
  agent-ix/filament-core-data#178).

### Per meaning

- **`record-value-type/v1`.** One or more `fields` and no identity field. Two
  values are equal exactly when they have one most-specific type and every
  field is equal. A field `typeRef` may name it.
- **`event-type/v1`.** One or more `fields`, no identity field, and an
  `occurrenceField` naming one field whose `typeRef` resolves through value
  types to the native `Timestamp`. The declaration carries no occurrence
  identity, and Quire gives event values no equality rule.
- **`state-machine/v1`.** One or more `operations`, one or more `states` and
  one or more `transitions`, and optional `fields`. Each transition names a
  `from` and a `to` state, a `trigger` operation, an optional `guard` naming a
  clause by clause id, and `emits`, a list of event types. A guard clause is
  `language: quire`, otherwise `invalid_model_binding`/`malformed-declaration`,
  because a guard must be type-checkable against the machine's context.
  Not bound: a current state, an initial or final state, when a transition is
  enabled, firing, the order of transitions, and what happens to the state
  when a trigger is invoked.
- **`process/v1`.** One or more `identityFields`, the correlation key of one
  run, other `fields`, and one or more `steps` in declaration order. Each step
  has a `name`, a `stepKind` of exactly `command`, `event`, `decision`,
  `compensation` or `wait`, and `consumes` and `emits` lists of event types.
  Not bound: execution order, branching, a compensation target and a wait
  condition, which no IR member states. A process `States` table and
  process-level `emits` are not carried into the IR and bind nothing.
- **`persistence-interface/v1`.** One or more `operations`, no `fields`, and a
  non-empty `persists` list of object types. A module may narrow `persists`
  further, as spec-objects-business narrows it to its `persistable` role;
  Quire checks only that each entry is an object type. Not bound: any storage
  law, such as read-after-write.
- **`namespace/v1`.** One or more `members`, each a type of the package that
  is not a namespace, and a `vocabulary` of terms, each a non-empty term and
  its description. It declares no `fields` and no `operations`: either is
  `invalid_model_binding`/`malformed-declaration`. A type is a member of at
  most one namespace; the later namespace in node order that repeats a member
  is `invalid_model_binding`/`malformed-declaration`. A namespace yields no
  effective type.

## Identity and comparison

- **Identity kind** — *value*. A meaning id is an exact label with no identity
  of its own.
- **Key** — the label itself. Matching is exact, bytewise over UTF-8. Spellings
  are not normalized, and no prefix, suffix or case variant matches.
- **Key order** — lexicographic over the label, for collection comparison only.
  It carries no precedence between meanings.
- **Absence** — a construct with no `meaning`, or with a `meaning` that is not
  a value of this vocabulary, has no Quire meaning. Model intake refuses its
  type definitions and population nodes (FR-154); it never infers a meaning from the kind name,
  the module, the shape or the members.
- **Kind names** — a kind name is module data. Two kinds with one name in
  different modules, or one kind named like a meaning, carry only the meaning
  their construct names.
- **Versioning** — a meaning with changed semantics is a new value with a new
  `/vN` suffix. A value is never redefined in place.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-208-AC-1 | Each member's value equals the exact label string in the table, and every meaning id that FR-150 through FR-154 bind is a member. | Inspection |
| FR-208-AC-2 | A type definition whose construct names a systems meaning binds to that systems kind whatever its kind name, and a type definition whose kind name is `port` but whose construct names no systems meaning binds to no systems kind. | Test (TC-197) |
| FR-208-AC-3 | A type definition or population node whose construct has no `meaning`, or names a label outside this vocabulary, is refused at intake and receives no default meaning. | Test (TC-234) |
| FR-208-AC-4 | A type definition whose construct names `quire.meaning.model.record-value-type/v1` binds its fields and equality by most-specific type and every field, and one with an identity field is `invalid_model_binding`/`malformed-declaration` and one with an operation is `unsupported_construct`/`declaration-form`. A type definition whose construct names `quire.meaning.model.event-type/v1` binds its occurrence field when that field's type resolves to `Timestamp`; an identity field, an operation or an occurrence field of another type is `invalid_model_binding`/`malformed-declaration`, and an occurrence field naming no field is `missing_declaration`/`missing-name`. | Test (TC-235) |
| FR-208-AC-5 | A type definition whose construct names `quire.meaning.model.state-machine/v1` binds its states and transitions and no firing, current or initial state. No operations, no states or no transitions, an `emits` entry that is not an event type, or a guard clause not in `quire` is `invalid_model_binding`/`malformed-declaration`; a transition `from`, `to`, `trigger` or `guard` naming nothing is `missing_declaration`/`missing-name`; a repeated state is `invalid_model_binding`/`conflicting-binding`. | Test (TC-235) |
| FR-208-AC-6 | A type definition whose construct names `quire.meaning.model.process/v1` binds its correlation key, an object universe of runs, and its steps in declaration order and no execution. No identity field, no steps, or a `consumes` or `emits` entry that is not an event type is `invalid_model_binding`/`malformed-declaration`; an entry naming nothing is `missing_declaration`/`missing-name`; a repeated step name is `invalid_model_binding`/`conflicting-binding`. | Test (TC-235) |
| FR-208-AC-7 | A type definition whose construct names `quire.meaning.model.persistence-interface/v1` binds its operation signatures and `persists` list. A field, no operations, an empty `persists` or a `persists` entry that is not an object type is `invalid_model_binding`/`malformed-declaration`; an entry naming nothing is `missing_declaration`/`missing-name`; an operation contract or a `quire` clause is `unsupported_construct`/`declaration-form`. | Test (TC-235) |
| FR-208-AC-8 | A type definition whose construct names `quire.meaning.model.namespace/v1` yields exactly one original declaration with its members and vocabulary and no effective type. A field, an operation, a namespace member, or a member already in an earlier namespace is `invalid_model_binding`/`malformed-declaration`; a member naming nothing is `missing_declaration`/`missing-name`; a `quire` clause is `unsupported_construct`/`declaration-form`. | Test (TC-235) |
| FR-208-AC-9 | A field `typeRef` may name a record value type, and a population member, a `creates` or `deletes` frame entry or a relationship end may name a process, whose own `relationships[]` entries have object-type relationship meaning. A field `typeRef` naming a node of any other business model meaning, and a population member, frame `creates` or `deletes` entry, relationship end or owned relationship of a node of a business model meaning other than a process, is `invalid_model_binding`/`malformed-declaration`. | Test (TC-235) |
| FR-208-AC-10 | A `quire` invariant of a record value type or event type is evaluated once for every value of the type reachable through the fields of every object in an invariant observation and through the parameters and results of an evaluated invocation, including collection elements, present optionals and nested record values, with `self` bound to that value; a false result is a logical violation with no diagnostic code, located by its FR-207 runtime value path; `pre` is refused with `wrong_snapshot`/`forbidden-pre-read`. A `quire` invariant of a state machine holds for the one context in the pre and post observation of every invocation of its operations, bound by the State-machine context anchors, and an observation with no context is `invalid_model_binding`/`wrong-context`; a `quire` invariant of a process holds for every run in the population at each invariant observation; a guard clause is not an invariant and is not evaluated. | Test (TC-235) |
| FR-208-AC-11 | A `Pre:` precondition and `Post:` postcondition of a state-machine or process operation read the invocation's pre and post observations with `self` the machine context or the run, a `pre` read in a precondition is `wrong_snapshot`/`forbidden-pre-read`, and neither is related to a transition's guard, `from` or `to`. | Test (TC-235) |
| FR-208-AC-12 | A record value type, event type, state machine or process specializing a node of its own meaning inherits its fields, invariants and operations, an event type its occurrence field, and a state machine its states and transitions, and conforms to each ancestor; a process inherits no steps; a supertype of another meaning is `invalid_model_binding`/`malformed-declaration`; a state or transition declared by two owners, or an ancestor occurrence field the subtype's neither is nor redefines, is `invalid_model_binding`/`derivation-conflict`; a subtype transition whose guard names an ancestor's invariant is `invalid_model_binding`/`malformed-declaration` at that guard; a persistence interface or namespace with supertypes is `unsupported_construct`/`declaration-form`. | Test (TC-235) |

## Dependencies

- [FR-152](../../functional/type-model/FR-152-bind-systems-model-structures.md)
  binds the systems meanings.
- [FR-154](../../functional/type-model/FR-154-admit-domain-package-model.md)
  binds every meaning at intake.
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) owns the
  model intake design.

## Status

Accepted complete-V1 meaning vocabulary. Implementation and qualification
remain tracked separately.
