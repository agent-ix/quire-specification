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
| `quire.meaning.model.record-value-type/v1` | A record value type: a named record of one or more fields, none an identity field, equal to another value exactly when every field is equal. Structural as stated under Business model meanings. |
| `quire.meaning.model.event-type/v1` | An event type: a record of one occurrence, with no identity field and one occurrence field whose type resolves to `Timestamp`. It has no equality rule. Structural as stated under Business model meanings. |
| `quire.meaning.model.state-machine/v1` | A state machine: operations, a closed set of states and a transition relation of `from` state, `to` state, `trigger` operation, optional guard clause and emitted event types. Structural: no firing is bound. |
| `quire.meaning.model.process/v1` | A process: identity fields that form the correlation key of one run, and steps in declaration order, each with a step kind and the event types it consumes and emits. Structural: no execution is bound. |
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
Intake (FR-154) refuses a node that breaks a rule below with the refusal the
rule names, and a package with a refused node yields no effective view.

Rules common to the six meanings:

- **Declarations.** Each node yields exactly one original declaration keyed
  by the package identity and its IR node identity, `ix://<package>/<artifact
  id>`, and one per field, relationship, operation and clause member. States,
  transitions, steps, `persists`, `members` and `vocabulary` are data of that
  declaration, not members.
- **Not object types.** None of them is an object type. None has an object
  universe, takes part in conformance or dispatch, is a population member, a
  relationship end, a supertype, or a `creates` or `deletes` frame entry.
  Naming one there is `invalid_model_binding`/`malformed-declaration`.
- **Field types.** A field or parameter `typeRef` may name a record value
  type. Naming an event type, state machine, process, persistence interface
  or namespace is `invalid_model_binding`/`malformed-declaration`.
- **Generalization.** None of them is generalized. A non-empty `supertypes` is
  `unsupported_construct`/`declaration-form`, and its entries are not checked
  further.
- **Clauses.** A clause in a language other than `quire` is kept with its span
  and never evaluated. A `quire` clause has no invariant meaning on these
  nodes, because an invariant ranges over the objects of an object universe:
  it is `unsupported_construct`/`declaration-form`, except a state machine's
  guard clause, below.
- **Relationships.** A node of these meanings is never a relationship end, so
  a `relationships[]` entry of such a node is
  `invalid_model_binding`/`malformed-declaration`. Links the lift does not
  carry into the IR, such as `owns` and `transitions_to`, bind nothing.
- **References.** A reference below that names no node of the package is
  `missing_declaration`/`missing-name`. A reference that names a node of the
  wrong meaning is `invalid_model_binding`/`malformed-declaration`.
- **Names.** Two states of one state machine, two transitions of one state
  machine with equal `from`, `to` and `trigger`, or two steps of one process
  with one name have one IR identity, and are
  `invalid_model_binding`/`conflicting-binding`. This is the only
  step-name and state-name uniqueness check.

Per meaning:

- **`record-value-type/v1`.** One or more `fields` and no identity field. Two
  values are equal exactly when every field is equal. A field `typeRef` may
  name it. It declares no operations: an operation is
  `unsupported_construct`/`declaration-form`.
- **`event-type/v1`.** One or more `fields`, no identity field, and an
  `occurrenceField` naming one field of the type whose `typeRef` resolves
  through value types to the native `Timestamp`. The declaration carries no
  occurrence identity, and Quire gives event values no equality rule. It
  declares no operations: an operation is
  `invalid_model_binding`/`malformed-declaration`.
- **`state-machine/v1`.** One or more `operations`, one or more `states` and
  one or more `transitions`, and optional `fields`. Each transition names a
  `from` and a `to` state of the machine, a `trigger` operation of the
  machine, an optional `guard` naming a clause of the machine by clause id,
  and `emits`, a list of event types. A guard clause is `language: quire`,
  otherwise `invalid_model_binding`/`malformed-declaration`; it is bound as
  that transition's guard only, is not an invariant and is not evaluated. A
  `quire` clause that no guard names is
  `unsupported_construct`/`declaration-form`. Each operation binds its name,
  parameters and return; an operation with a `requires`, `ensures` or a
  non-empty frame is `unsupported_construct`/`declaration-form`. Not bound: a
  current state, an initial or final state, when a transition is enabled, firing, the order of
  transitions, and what happens when a trigger is invoked.
- **`process/v1`.** One or more `identityFields`, the correlation key of one
  run, other `fields`, and one or more `steps` in declaration order. Each step
  has a `name`, a `stepKind` of exactly `command`, `event`, `decision`,
  `compensation` or `wait`, and `consumes` and `emits` lists of event types.
  Not bound: execution order, branching, a compensation target and a wait
  condition, which no IR member states. A process `States` table and
  process-level `emits` are not carried into the IR and bind nothing. It
  declares no operations: an operation is
  `unsupported_construct`/`declaration-form`.
- **`persistence-interface/v1`.** One or more `operations`, no `fields`, and a
  non-empty `persists` list of object types. A module may narrow `persists`
  further, as spec-objects-business narrows it to its `persistable` role;
  Quire checks only that each entry is an object type. Each operation binds its
  name, parameters and return. An operation with a `requires`, `ensures` or a
  non-empty frame is `unsupported_construct`/`declaration-form`. Not bound: any
  storage law, such as read-after-write.
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
| FR-208-AC-4 | A type definition whose construct names `quire.meaning.model.record-value-type/v1` binds its fields and field-wise equality, and one with an identity field is `invalid_model_binding`/`malformed-declaration` and one with an operation or a `quire` clause is `unsupported_construct`/`declaration-form`. A type definition whose construct names `quire.meaning.model.event-type/v1` binds its occurrence field when that field's type resolves to `Timestamp`; an identity field, an operation or an occurrence field of another type is `invalid_model_binding`/`malformed-declaration`, and an occurrence field naming no field is `missing_declaration`/`missing-name`. | Test (TC-235) |
| FR-208-AC-5 | A type definition whose construct names `quire.meaning.model.state-machine/v1` binds its states and transitions and no firing, current or initial state. No operations, no states or no transitions, an `emits` entry that is not an event type, or a guard clause not in `quire` is `invalid_model_binding`/`malformed-declaration`; a transition `from`, `to`, `trigger` or `guard` naming nothing is `missing_declaration`/`missing-name`; a repeated state is `invalid_model_binding`/`conflicting-binding`; a `quire` clause no guard names, or an operation with a contract, is `unsupported_construct`/`declaration-form`. | Test (TC-235) |
| FR-208-AC-6 | A type definition whose construct names `quire.meaning.model.process/v1` binds its correlation key and its steps in declaration order and no execution. No identity field, no steps, or a `consumes` or `emits` entry that is not an event type is `invalid_model_binding`/`malformed-declaration`; an entry naming nothing is `missing_declaration`/`missing-name`; a repeated step name is `invalid_model_binding`/`conflicting-binding`; an operation or a `quire` clause is `unsupported_construct`/`declaration-form`. | Test (TC-235) |
| FR-208-AC-7 | A type definition whose construct names `quire.meaning.model.persistence-interface/v1` binds its operation signatures and `persists` list. A field, no operations, an empty `persists` or a `persists` entry that is not an object type is `invalid_model_binding`/`malformed-declaration`; an entry naming nothing is `missing_declaration`/`missing-name`; an operation contract or a `quire` clause is `unsupported_construct`/`declaration-form`. | Test (TC-235) |
| FR-208-AC-8 | A type definition whose construct names `quire.meaning.model.namespace/v1` yields exactly one original declaration with its members and vocabulary and no effective type. A field, an operation, a namespace member, or a member already in an earlier namespace is `invalid_model_binding`/`malformed-declaration`; a member naming nothing is `missing_declaration`/`missing-name`; a `quire` clause is `unsupported_construct`/`declaration-form`. | Test (TC-235) |
| FR-208-AC-9 | A field `typeRef` may name a record value type. A field `typeRef` naming a node of any other business model meaning, and a supertype, relationship end or population member naming a node of any business model meaning, is `invalid_model_binding`/`malformed-declaration`; a business model node with a non-empty `supertypes` is `unsupported_construct`/`declaration-form`. | Test (TC-235) |

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
