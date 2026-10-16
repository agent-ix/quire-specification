---
id: FR-152
title: "Bind complete systems-model structures"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-035
    type: references
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-208
    type: references
---

# FR-152: Bind complete systems-model structures

## Description

When a clause references a systems-model element, the binder SHALL resolve its
typed part, port, interface, connection or allocation identity through the
declarations of the admitted domain package.

## Inputs

Checked package, the admitted domain package with its ModelSelection (FR-321),
the FR-150 effective view, the selected `quire.model.complete/v1` definition
and a qualified element reference with its required kind.

## Outputs

A typed resolved model element/relationship or located kind, identity,
direction or foreign refusal.

## Behavior

Parts, ports, interfaces, connections and allocations retain distinct kinds and
their declared end, direction and type constraints. Display names and equal
shapes are not identities. Navigation follows only declared relationships and
connections and preserves original/effective declaration provenance.

## Systems-model roles

Rule `quire.model.systems.kind-mapping/v1` of the selected
`quire.model.complete/v1` definition maps each IR node to at most one systems
kind by the Quire meaning id (FR-208) of its construct: the `meaning` of the
`constructs` table entry that the node's `{module, name}` kind names. The kinds
are disjoint. A node's kind name and module never select a systems kind: a node
whose construct names no systems meaning has no systems kind, whatever its kind
name, and a node whose construct names a systems meaning has that kind under
any kind name.

| Kind | Construct meaning | Required identity and constraints |
| --- | --- | --- |
| Part | `quire.meaning.systems.part/v1` | construct declaration key, `owner` reference to the owning composite type, declared type and typed multiplicity |
| Port | `quire.meaning.systems.port/v1`, whose `owner` names a Part | construct declaration key, `owner` reference to the owning part, direction `in`, `out` or `inout`, interface type (an Interface) and typed multiplicity |
| Interface | `quire.meaning.systems.interface/v1` | construct declaration key and ordered feature signatures |
| Connection | `quire.meaning.systems.connection/v1`, whose two ends both name Ports | construct declaration key, effective member owned by the source end-owner type, compatible source and target port keys and a declared `direction` |
| Allocation | `quire.meaning.systems.allocation/v1` | construct declaration key, effective member owned by the source end-owner type, plus source element (Part, Port or operation) and target element (Part) keys |

Each part, port, interface, connection and allocation is declared by its own
artifact and is a construct, not a member. Its IR node identity is
`ix://<package identity>/<artifact id>`, like any type definition (FR-154).
Ownership is a reference of the construct, not a member, and never part of its identity:
a part's `owner` names its owning type; a port's `owner` names its part, and
the port is owned by that part's owning type; and a connection or allocation
end's end-owner type is the element owner of the declaration its source or
target end names. Parts, ports, connections and allocations are qualified into
the FR-150 effective view as effective members `(owner effective type,
construct declaration key)` under those owners; an interface is an effective
type. Changing an `owner` or an end changes the effective member, never the
construct's declaration key. A relationship end is navigable from type `T` exactly
when `T` conforms (FR-082) to the type of the end whose role `r.name` matches,
the source end for the source role and the target end for the target role;
ownership of the relationship plays no part. A relationship whose
two ends both name object types is a navigation relationship with no kind.

Kind mapping runs over interfaces, then parts, then ports, then connections,
then allocations, each ascending by declaration key, charging `systems.kind` before each node, and
is exhaustive under the limits: a port whose owner has no kind or another kind
is `wrong-export` (required Part); a port whose interface type is not an
Interface is `wrong-export` (required Interface); a connection end naming a
node that is not a Port is `wrong-export` (required Port, once per end, source
first). Refusals cascade: a node whose dependency was refused is reported with
actual kind `none`. Every refusal names the IR node identity and its FCD `origin`
(FR-154).

A binder request names the required kind and a qualified name. A qualified
name names a systems construct by the model alias and its artifact id, such as
`M::pump_out`, never through its owner. It resolves to the construct's
effective declaration: an Interface's effective type, a Part's or Port's
effective member under its owner, and a Connection's or Allocation's effective
member under its source end-owner type. A resolved
declaration of another kind is refused `invalid_model_binding`/`wrong-export`
with the required kind, actual kind and both declaration keys; equal names
never pass a kind check.

Rule `quire.model.systems.connection/v1` admits a connection only when all of
these hold, each charged as `systems.connection-condition` in table order and
reported independently before the connection is exposed:

| Condition | Refusal |
| --- | --- |
| For declared direction `source-to-target`, the source port is `out` or `inout` and the target port is `in` or `inout`; for `target-to-source`, the same with source and target exchanged; for `bidirectional`, both ports are `inout`; `undirected` is never a connection direction | `invalid_model_binding`/`port-direction` |
| The flow-source port's interface type conforms under FR-151 to the flow-target port's interface type (for `bidirectional`, the two interface types are the same effective type) | `ill_typed`/`type-mismatch` |
| Each connection end's typed multiplicity conforms under FR-151 to its port's typed multiplicity | `ill_typed`/`multiplicity-narrowing` |

Rule `quire.model.systems.allocation/v1` admits an allocation only when its
target element is a Part, charging `systems.allocation`; any other target is
`invalid_model_binding`/`wrong-export` (required Part, actual kind or `none`).
Allocation compatibility has no other rule.

## Navigation

Static navigation (`qualified-name` resolution through members and ends) is
resolved at checking and charged as `systems.resolve` per qualified-name
segment after the alias and per runtime navigation site, in source order. A
qualified name whose alias names another ModelSelection than the one that owns
the navigated declaration is refused at checking as
`foreign_reference`/`foreign-model-selection`. Runtime navigation `r.name` over
`r: Reference<T>` resolves statically to exactly one field or relationship end
of `T` in the effective view. A relationship end is named by the role of the
end nearest the receiver: the source end's role is the forward member,
navigable from receivers conforming to the source end's type and reaching the
target end, and the target end's role, when declared, is the inverse member,
navigable from receivers conforming to the target end's type and reaching the
source end; the destination end supplies the multiplicity and the result type.
A name matching both a field and an end is
`ambiguous_declaration`/`ambiguous-name`. A name that is no field of `T` and
the role of no end navigable from `T` (a far-end role among them) finds no
candidate: checking source refuses it `ill_typed`/`type-mismatch` at the form,
as for an undeclared field (QSL FR-363), and the IR member check of a
`relationship_end` naming a role the relationship does not name refuses
`missing_declaration`/`missing-name` at `member.name` (FR-322-AC-56). Traversal from source to target
requires declared direction `source-to-target`, `bidirectional` or
`undirected`, and traversal from target to source requires `target-to-source`,
`bidirectional` or `undirected`; any other traversal is
`ill_typed`/`operator-ineligible`. The static result type follows the destination end's
typed multiplicity: `[0,1]` gives `Option<Reference<U>>`; `[1,1]` gives
`Reference<U>`; any other finite `[l,u]` gives `Set<Reference<U>>[l,u]` when
`unique` is true and `Bag<Reference<U>>[l,u]` otherwise, in canonical
reference-key order. An `unbounded` upper bound or `ordered: true` is
`unsupported_construct`/`expression-form`. At runtime, navigation
reads the relationship instances (the links) of the relationship that the
observation holds. A listed relationship extent is complete or incomplete; an
unlisted extent says nothing about the relationship's links. When the extent is
unlisted or incomplete, navigation returns `incomplete_population`/
`incomplete-scope` retaining the relationship and the receiver, after charging
`model.navigate` and with no later charge. Admission never decides this,
because it cannot know which relationships a clause reads. A complete extent
with no link states that the relationship has no links, and navigation returns
the empty result of the destination end's type per its multiplicity: `none`
for `[0,1]`, the empty `Set` or `Bag` for another finite `[l,u]`, and a count
below `l` is then `cardinality_out_of_bound`/`below-minimum`. Otherwise, targets are taken in
canonical reference-key order and a target absent from a complete population is
`dangling_reference`, decided before the count bound; then a target count
outside the bound is `cardinality_out_of_bound`/`below-minimum` or
`above-maximum`. Charges are the `model.navigate` schedule of
`quire.value.accounting/v1`.

## Reachability over a relationship end

`reaches(a, b, name)` where `name` is a relationship end is the Boolean
`quire.op.model.reaches` application (FR-322-AC-57). The end is navigated as
above, and checking admits a destination end of the receiver endpoint's own
type `E` with any finite multiplicity, single-valued (`[0,1]`, `[1,1]`) or
multi-valued (`Set` or `Bag` shaped, unique or not). An `unbounded` upper bound
or `ordered: true` is `unsupported_construct`/`expression-form`, and a
destination end of another type is `ill_typed`/`operator-ineligible`, each at
`member.name`; direction is decided before multiplicity and multiplicity before
type. The result is `Boolean` for every multiplicity.

Reachability is one or more links: an object does not reach itself without a
cycle or a self-loop. The neighbours of an object are the targets of its links
in the relationship's extent, taken in canonical reference-key order, the
repeats of one reference adjacent. A single-valued end gives at most one
neighbour and a multi-valued end gives each of them: the search is breadth-first
in discovery order, and the frontier expands every neighbour of a dequeued
object in that order. The charges are the `graph` rows of
`quire.value.accounting/v1`: `graph.expand` for `a` and then once for each
object when it is first discovered (`value_occurrences` the number of distinct
objects discovered, `a` included), one `graph.edge` for each link occurrence
examined, including a repeat of a neighbour already discovered, and one
`graph.result-retain`. The search stops at the first neighbour equal to `b`.
`model.navigate` and the `collection` rows are not charged.

The destination end's bound is not a reachability refusal: a neighbour count
above `u` or below `l` refuses nothing, because an observation is evidence of a
state and the form does not count neighbours. An extent that is unlisted or
listed incomplete gives `incomplete_population`/`incomplete-scope` retaining the
relationship and `a`, after the `graph.expand` for `a` and with no later
charge; a Boolean comes only from a complete extent. The start `a` is decided first, before any charge: `a` absent from a complete
population is `dangling_reference`/`absent-target-in-complete-population`, and
`a` undecidable (absent from a population that is not known complete) is
`incomplete_population`/`incomplete-scope`, each retaining `a` and the
relationship, with no `graph.expand` and no later charge. A neighbour absent from a
complete population is `dangling_reference`, decided before that neighbour's
`graph.edge`, and the first neighbour in order that is not present decides.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-152-AC-1 | Valid references to every supported systems-model kind resolve with exact declaration key and effective-declaration identity. | Test (TC-197) |
| FR-152-AC-2 | Substituting a part for a port or a connection for an allocation refuses `invalid_model_binding`/`wrong-export` even when names match. | Test (TC-197) |
| FR-152-AC-3 | A dangling end reference or an ambiguous effective target refuses without ambient lookup. | Test (TC-197) |
| FR-152-AC-4 | Port direction, port interface type and multiplicity are all enforced before a connection is exposed. | Test (TC-197) |
| FR-152-AC-5 | Relationship navigation never crosses ModelSelections or substitutes display-name equality for identity. | Test (TC-197) |
| FR-152-AC-6 | Each declared relationship direction admits exactly the port-direction pairs and traversals stated above, and every other pair or traversal refuses with its named cause. | Test (TC-197) |
| FR-152-AC-7 | Runtime navigation returns the multiplicity-determined type in canonical reference-key order and charges exactly the `model.navigate` schedule. | Test (TC-197) |
| FR-152-AC-8 | A node whose construct meaning is not a systems meaning has no systems kind even when its kind name is `port`: a binder request for a Port and a connection end naming it each refuse `invalid_model_binding`/`wrong-export` with required kind `Port` and actual kind `none`. | Test (TC-197) |
| FR-152-AC-9 | A relationship end is navigable by the role of the end nearest the receiver: `r.name` names the source end's role from a source-typed receiver and the target end's role from a target-typed receiver, a far-end role finds no candidate and refuses `ill_typed`/`type-mismatch` at checking, and the destination end supplies the multiplicity and result type. | Test (TC-197) |
| FR-152-AC-10 | Navigating a relationship whose extent is unlisted, and one whose extent is listed incomplete, each return `incomplete_population`/`incomplete-scope` after `model.navigate` and no later charge, while a complete extent with no links returns `none` or the empty collection of the destination end's type after the charges of the multiplicity, or `cardinality_out_of_bound`/`below-minimum` when the destination end's lower bound is above 0. | Test (TC-197) |
| FR-152-AC-11 | `reaches(a, b, name)` over a relationship end admits a single-valued and a finite multi-valued destination end of the receiver endpoint's own type, and over a multi-valued end the frontier expands every neighbour in reference-key order, charging `graph.expand` per newly discovered object, `graph.edge` per link occurrence including repeats, and one `graph.result-retain`, with exact required counters completing and one fewer incomplete at the stated charge point. | Test (TC-197) |
| FR-152-AC-12 | An `unbounded` or `ordered` destination end, a destination end of another type and a traversal against the declared direction each refuse with the cause and order stated above; a neighbour count outside the destination bound refuses nothing; an unlisted or incomplete extent gives `incomplete_population`/`incomplete-scope` after the start's `graph.expand`; a dangling neighbour refuses `dangling_reference` before its `graph.edge`; a dangling or undecidable start decides before any charge. | Test (TC-197) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [FR-208](../../objects/foundation/FR-208-quire-meaning-vocabulary.md) owns
  the systems meaning ids.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
