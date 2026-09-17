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
| `quire.meaning.model.record-value-type/v1` | A record value type: a named record of one or more `fields`, none of them an identity field, with no object identity. Two values are equal exactly when every field is equal. Fields may name it as their `typeRef`. It may declare operations and invariant clauses over `self`. |
| `quire.meaning.model.event-type/v1` | An event type: a record of one occurrence. It has one or more `fields`, none of them an identity field, and its `occurrenceField` names one of those fields, typed `Timestamp`, as the instant of the occurrence. A value is not changed after construction. The declaration carries no occurrence identity and declares no operations. |
| `quire.meaning.model.state-machine/v1` | A state machine: one or more `operations`, optional context `fields`, a closed set of `states` and a transition relation over them. Each `transitions[]` entry is `from` state, `to` state, `trigger` operation of the machine, an optional `guard` naming one clause of the machine by clause id, and the event types it `emits`. A transition is enabled when its `trigger` is invoked in its `from` state and its `guard`, evaluated in the pre-state of that invocation, holds; an absent guard holds. Firing an enabled transition moves the machine from its `from` state to its `to` state and emits one event of each type in `emits`. Every enabled transition is a possible step, and a trigger invoked in a state with no enabled transition fires no transition. The declaration names no initial or final state, and Quire derives none. |
| `quire.meaning.model.process/v1` | A process: an identified type whose `identityFields` are the correlation key of one run, with `fields`, and an ordered list of `steps`. Each step has a name unique in the process, a `stepKind` of exactly `command`, `event`, `decision`, `compensation` or `wait`, and the event types it `consumes` and `emits`. The meaning is structural: step order is declaration order, and Quire derives no execution order, branch, compensation target or wait condition from the steps. |
| `quire.meaning.model.persistence-interface/v1` | A persistence interface: one or more `operations`, no fields and no state, and a `persists` list naming the object types its operations persist. Each operation means its declared contract only. Quire derives no storage law, such as read-after-write, from `persists`. |
| `quire.meaning.model.namespace/v1` | A namespace: a boundary over the types its `members` name and a `vocabulary` of terms, each a term and its description. It declares no fields, operations or instances and is not a type: no field `typeRef`, relationship end, supertype or population member names it. No member is a namespace, and a type is a member of at most one namespace. |
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
| FR-208-AC-4 | A type definition whose construct names `quire.meaning.model.record-value-type/v1` or `quire.meaning.model.event-type/v1` binds as that value or event type whatever its kind name, and one that declares an identity field, or an event type whose `occurrenceField` names no `Timestamp` field of the type, is refused. | Test (TC-235) |
| FR-208-AC-5 | A type definition whose construct names `quire.meaning.model.state-machine/v1` binds its closed state set and transition relation, and a transition whose `from` or `to` names no state, whose `trigger` names no operation of the machine, whose `guard` names no clause of the machine or whose `emits` names no event type is refused; no initial state is bound. | Test (TC-235) |
| FR-208-AC-6 | A type definition whose construct names `quire.meaning.model.process/v1` binds its correlation key and its steps in declaration order, and a process with no identity field, a repeated step name or a step whose `consumes` or `emits` names no event type is refused; no execution order is bound. | Test (TC-235) |
| FR-208-AC-7 | A type definition whose construct names `quire.meaning.model.persistence-interface/v1` binds its operations and `persists` list, and one that declares a field, declares no operation or persists a type that is not an object type is refused. | Test (TC-235) |
| FR-208-AC-8 | A type definition whose construct names `quire.meaning.model.namespace/v1` binds no type, and one that declares a field or operation, names a namespace as a member, or shares a member with another namespace is refused, and so is a field whose `typeRef` names a namespace. | Test (TC-235) |

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
