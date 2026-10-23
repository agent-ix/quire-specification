---
id: FR-154
title: "Admit the domain model from a domain package"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-150
    type: references
  - target: ix://agent-ix/quire-specification/FR-321
    type: references
  - target: ix://agent-ix/quire-specification/FR-208
    type: references
  - target: ix://agent-ix/filament-core-data/FR-095
    type: references
  - target: ix://agent-ix/filament-core-data/FR-091
    type: references
  - target: ix://agent-ix/filament-core-data/FR-146
    type: references
  - target: ix://agent-ix/quire-specification/FR-142
    type: references
  - target: ix://agent-ix/quire-specification/FR-415
    type: references
---

# FR-154: Admit the domain model from a domain package

## Description

When a package imports a domain package, QSL model intake SHALL admit the
package by identity and `sha256-jcs` digest and SHALL build one Quire
model declaration for each IR node, refusing any invalid node with its IR node identity and
`origin`.

## Inputs

The source `model` declaration with its domain package identity and digest; the domain package bytes; and the selected `quire.model.complete/v1`
definition.

## Outputs

The original declarations of one ModelSelection (FR-321), each keyed by
`(domain package identity, IR node identity, sha256-jcs)`, or located refusals.

## Behavior

The domain model is declared in the spec's artifacts (AD-006). The model intake
chain is:

1. **Artifact bundle.** The spec package's artifacts declare the domain
   objects. Each domain object is one artifact whose object type, such as
   `entity`, comes from a module.
2. **Extraction.** quire-rs reads each artifact under its module manifest and
   yields field, relationship, operation, frame, clause, population and systems
   declarations with source spans.
3. **Lift.** The FCD extraction-frontend lowers the extracted declarations into
   one semantic IR 2.0.0 domain package, serialized with RFC 8785 JCS and
   fingerprinted with `sha256-jcs`.
4. **Admission.** Rule `quire.model.normalize.intake/v1` checks the selection
   and the package bytes in the order the selected definition fixes.
5. **Declarations.** Intake builds one original declaration per IR node,
   ascending by declaration key, and rule `quire.model.features/v1` checks
   each declaration.

The package input maps each `sha256-jcs` digest to package bytes. Intake looks
up the bytes by the selected digest and compares the package's own identity
with the selection. Intake does not recompute the supplied document's digest or
compare it with the selected digest: the selected digest is the key of that
lookup and the content identity member of the ModelSelection (FR-321), and the
document supplied under it is taken as the document that digest names.

Admission refusals, in order:

| Check | Refusal |
| --- | --- |
| The selection's domain package identity equals `quire/native` or `quire/unit`, or begins `quire/native/` or `quire/unit/` (a reserved identity) | `invalid_model_binding`/`malformed-declaration` |
| The digest domain is exactly `sha256-jcs` | `stale_dependency`/`digest-domain-mismatch` |
| The package input supplies bytes under the selected digest | `missing_import`/`missing-selection` |
| The package's own identity equals the selection's | `invalid_model_binding`/`wrong-model-selection` |

Declaration refusals:

| Failure | Refusal |
| --- | --- |
| A type definition's or population node's artifact id does not match `^[A-Za-z][A-Za-z0-9_]*$` | `invalid_model_binding`/`malformed-declaration` |
| A type definition's or population node's `kind` names no entry of the `constructs` table, or its construct has no `meaning` or a `meaning` that is not a Quire meaning id (FR-208) | `invalid_model_binding`/`malformed-declaration` |
| An IR node is not valid for its construct's meaning, including a reference that names a node of the wrong meaning and a `supertypes` entry that names a node of another meaning (FR-208) | `invalid_model_binding`/`malformed-declaration` |
| An operation parameter's `name` is absent, is not a string, is empty, does not match `^[A-Za-z][A-Za-z0-9_]*$`, or differs from the last segment of the parameter's identity `<operation identity>/<param>`, on a parameter not refused as a repeated identity of its operation (checked after that row and before the frame entries) | `invalid_model_binding`/`malformed-declaration` |
| A field's or an operation's `name` is absent, is not a string, is empty, does not match `^[A-Za-z][A-Za-z0-9_]*$`, or differs from the last segment of its identity `<owner identity>/<member name>`, on a member not refused as a repeated identity of its owner (checked after the "Two nodes share one identity" row below) | `invalid_model_binding`/`malformed-declaration` |
| A field, parameter, result, alias `target`, sequence `items`, map `values` or variant case `payloadType` that is none of `ix://quire/native/<Name>` with `<Name>` a native value type name, `ix://quire/unit/<name>` with a non-empty `<name>`, and `ix://<package identity>/<artifact id>` | `invalid_model_binding`/`malformed-declaration` |
| `ix://quire/native/Clock` as a reference `target`, a sequence `items`, a map `values` or a constraint `appliesTo`, `ix://quire/unit/<name>` as a reference `target` or a constraint `appliesTo`, or a clock member (a field or parameter typed `ix://quire/native/Clock`, or a result of that type) that breaks the clock member shape below | `invalid_model_binding`/`malformed-declaration` |
| A dimension or unit node with a malformed member or a unit-graph defect: a `name` outside `^[A-Za-z][A-Za-z0-9_]*$`, a non-boolean `base`, an `identity` that is not `ix://<package identity>/dimension/<name>` or `ix://<package identity>/unit/<name>`, a base dimension with terms, a term exponent that is not a canonical integer string, `0` or outside `i128`, terms out of order or repeated, a term cycle, a `scale` or `offset` that is not a canonical rational string or is outside `i128`, a zero `scale`, a `target` in another dimension, a second root of one dimension, a root whose `scale` is not `1` or `offset` not `0`, a target cycle, a `dimension` or `target` that is not an identity, or a missing or malformed `origin` | `invalid_model_binding`/`malformed-declaration` |
| A dimension term's `dimension`, a unit's `dimension` or a unit's `target` names no dimension or unit node of the package | `missing_declaration`/`missing-name` |
| A supertype, field type, subsets, redefines, relationship end, frame entry, population member, `occurrenceField`, transition `from`, `to`, `trigger`, `guard` or `emits` entry, step `consumes` or `emits` entry, `persists` entry or namespace `members` entry names no node or member of the package | `missing_declaration`/`missing-name` |
| A node of a business model meaning (FR-208) declares a form that meaning does not support: a non-empty `supertypes` or a `quire` clause of a persistence interface or namespace, an operation of a record value type, or an operation contract of a persistence interface | `unsupported_construct`/`declaration-form` |
| A case of a variant type (FR-208) has a `payloadType` that resolves | `unsupported_construct`/`declaration-form` |
| A variant type (FR-208) declares no case, a case has no source span, a case's identity is not the variant slot identity of its owner and its name, or its construct's shape is not `enumeration` | `invalid_model_binding`/`malformed-declaration` |
| Two nodes share one identity, including two dimensions or two units with one identity (the refusal names the shared identity at the later node's `Dimension.identity` or `Unit.identity`), two fields of one owner (the refusal names the shared identity at `TypeDefinition.fields[j]`), two operations of one owner or a field and an operation of one owner (at `TypeDefinition.operations[k]`; in a field and operation pair the operation, declared after the fields, is the one refused), two parameters of one operation (at `TypeDefinition.operations[k].params[s]`), two variant cases of one type (at `TypeDefinition.variants[i]`), two states of one owner with one name (the refusal names the shared identity at `TypeDefinition.states[i]`), two steps of one owner with one name (the refusal names the process at `TypeDefinition.steps[i]`), or a clause, state, transition or step whose member name is also the name of a field or operation of its owner (a state or transition names its own identity at `TypeDefinition.states[i]` or `TypeDefinition.transitions[i]`, the later declaration in document order, operations before states) | `invalid_model_binding`/`conflicting-binding` |
| Two transitions of one state machine have equal `from`, `to` and `trigger`, whatever their identities: the refusal names the later transition's identity at `TypeDefinition.transitions[i]` | `invalid_model_binding`/`conflicting-binding` |
| Two cases of one variant type have equal names and distinct identities | `invalid_model_binding`/`conflicting-binding` |
| A multiplicity has `lower > upper` | `invalid_model_binding`/`unpreserved-model-meaning` |

An operation parameter's `name` is the authoritative name of the parameter: the
frame entry `deletes p` and every reference to a parameter by name resolve
against it. Within one operation the checks run in this order, whatever the
position of their rows in the table: first the repeated-identity check over the
operation's `params` (two parameters with one identity refuse once, naming the
shared identity at the later parameter's path, and the collision suppresses
the name check of both colliding parameters); then the name check of every
parameter not suppressed, in `params` order; then the frame entries, so that
`deletes p` resolves against validated names. A parameter whose `name` equals
the last segment of its identity and whose name is repeated has a repeated
identity, so a repeated name is a repeated-identity refusal. The name refusal
is reported at the selection row (`/lock/model_selections/<i>`) like the other
declaration refusals of the selected document, names the operation's IR node
identity and the member path `TypeDefinition.operations[k].params[s].name`,
`k` the operation's index in its owner and `s` the parameter's, and retains the
operation's `origin` verbatim. Names that differ only in letter case are
distinct names.

A variant type is a type definition whose construct has meaning
`quire.meaning.model.variant-type/v1` and shape `enumeration`, with a
`variants` list of cases, each a node with an `identity`, a `name` and an
`origin`. A `payloadType` the reader cannot resolve is refused by the reader
before intake, so the payload row applies to a payload that resolves. The
variants refusals name these members:

- A construct whose shape is not `enumeration` names the type node and
  `TypeDefinition.kind`.
- A variant type with no case names the type node and `TypeDefinition.variants`.
- A case with no source span names the type node and `TypeDefinition.variants[i].origin`.
- A case's identity is the variant slot identity of its owner
  (`ix://<package identity>/variant/<Name>-<value>`) with `<Name>` the owner's
  artifact id verbatim and `<value>` the case name slugged. A case whose identity is
  not that identity names the case node, as given, and `Variant.identity`.
- A case with a `payloadType` names the case node and `Variant.payloadType`.
- Two cases of one type compare by exact UTF-8 byte sequence of their name. The
  later case in the `variants` list is the one refused, naming the type node and
  `TypeDefinition.variants[i]`, `i` the later case's index, and its identity,
  origin and payload checks are skipped. The list order has no other meaning.
- Two cases of one type that share one identity, whatever their names, are one
  collision of one owner: the later case in the `variants` list is refused once,
  `invalid_model_binding`/`conflicting-binding`, naming the shared identity
  (`ix://<package identity>/variant/<Name>-<value>`) at
  `TypeDefinition.variants[i]`, `i` the later case's index, whichever of the two
  carries the correctly minted identity. The collision refuses before the
  per-case checks run, so it suppresses the identity, origin and payload checks
  of both colliding cases, including an earlier case whose identity is not its
  slot identity (`[Silver, Gold]` with `Silver` carrying `Gold`'s identity):
  exactly one refusal is reported.

A field `typeRef`, an operation parameter's `typeRef` and an operation result's
`typeRef` each take one of three forms, and no other form is read. A native
value type is named `ix://quire/native/<Name>`, where `<Name>` is exactly one
of `Boolean`, `Integer`, `Rational`, `Decimal`, `Float32`, `Float64`, `Text`,
`String`, `UUID`, `Timestamp` and `Clock`, compared byte for byte. These are the
value-type keywords of the shared grammar's `type-ref` production together with
`String`, `UUID` and `Timestamp`, whose member types FR-322 states, and `Clock`,
the clock type of FR-415, which names no value type. A unit of
the checked package is named `ix://quire/unit/<name>`; intake admits it without
resolving it, and FR-322 "Model-owned members" resolves it (FR-208 "Field
types"). A type of the package is named `ix://<package identity>/<artifact
id>`, so a package type whose artifact id equals a native name
(`ix://test/orders/Integer`) is a different type from the native one and the
two never collide. A `typeRef` that begins `ix://quire/native/` and whose
`<Name>` is outside the set, and a `typeRef` of any other form, refuses
`invalid_model_binding`/`malformed-declaration`.

`ix://quire/native/Clock` is admitted as a field `typeRef`, an operation
parameter `typeRef`, an operation result `typeRef`, a variant case
`payloadType` and an alias `target` (filament-core-data FR-146 "Clock
members"). A field or parameter typed by an alias whose `target` chain reaches
`Clock` is a clock member as well, and an alias of `Clock` builds no type
declaration. A **clock member** is a field, parameter or result of the clock
type; it has the clock member shape: any multiplicity `{lower, upper, ordered,
unique}` that the member rows of the package admit for any member (a multiplicity
other than `{1, 1}` makes the member a clock collection, FR-415 "Clock
collections", typed by FR-322 "Model-owned members"; a result `{0, 1,
false, false}` is `Option<Clock>`), presence `required` or `optional`
(`optional` is `Option<Clock>`, FR-415 "Clock places"), `nullable` `false`,
`defaultKind` `none`, an empty or absent `constraints`, and no `unit`,
`decimal`, `textProfile` or `defaultValue`. Each shape member a clock member
breaks refuses once, naming the node and its member path (`Field.presence`,
`Field.nullable`, `Field.defaultKind`, `Field.defaultValue`, `Field.unit`,
`Field.decimal`, `Field.textProfile` or `Field.constraints`; a parameter's path
is `Operation.params[i].<member>` and a result's `Operation.returns.nullable`),
and no other row reports at those members; a multiplicity is read by the
multiplicity rows as for any member, so `lower > upper` refuses
`invalid_model_binding`/`unpreserved-model-meaning` at `Field.multiplicity`.
`Clock` as a sequence `items` and a map `values` is admitted: the sequence or
map is a clock collection of its kind (FR-415 "Clock collections"), a package map
is keyed by `Text`, so no key is a clock. `Clock` as a
`reference` `target` or a constraint `appliesTo` refuses at that member
(`TypeDefinition.target` or `Constraint.appliesTo`): a reference names an
object, and no constraint keyword applies to a clock. A variant case whose
`payloadType` is `Clock` is read by the variant payload row of the construct
validity table as any other payload is. Intake does not check which construct
owns a clock member or whether the model declares a time source; QSL FR-230
and FR-415 check those where the clock is used.

`ix://quire/unit/<name>` is admitted at every position that takes a type:
a field, parameter or result `typeRef`, an alias `target`, a sequence `items`,
a map `values` and a variant case `payloadType` (a payload follows the same
row as any other payload). A member of a unit type has the unit member shape:
`defaultKind` `none`, an empty or absent `constraints`, and no `unit`,
`decimal`, `textProfile` or `defaultValue`; its multiplicity, presence and
nullability are any other member's. A unit reference as a `reference` `target`
or a constraint `appliesTo` refuses `invalid_model_binding`/
`malformed-declaration` at that member: a reference names an object, and a
bounded quantity is not a type form (FR-142 "Quantity type reference").

A domain package may carry top-level `dimensions` and `units` arrays
(filament-core-data FR-146). A dimension node carries `identity`, `name`,
`base`, `terms` and `origin`; a unit node carries `identity`, `name`,
`dimension`, `scale`, `offset`, `origin` and an optional `target`. Intake
builds one original declaration for each dimension and each unit node, keyed
by (domain package identity, IR node identity) like every other node, and
charges `normalize.record` for each. A refusal of a dimension or unit names
the node and the member path of the FCD pointer of the defect: `/dimensions/<i>/<member>…`
is `Dimension.<member>…` and `/units/<j>/<member>…` is `Unit.<member>…`, so the
paths are `Dimension.identity`, `Dimension.name`, `Dimension.base`,
`Dimension.terms`, `Dimension.terms[t]`, `Dimension.terms[t].dimension`,
`Dimension.terms[t].exponent`, `Dimension.origin`, `Unit.identity`,
`Unit.name`, `Unit.dimension`, `Unit.target`, `Unit.scale`, `Unit.offset`,
`Unit.origin`, and the unit node itself (`Unit`) for a second root of one
dimension. A term cycle names the term of the cycle's least-index dimension
that FCD FR-146 names, and a target cycle the `Unit.target` of its least-index unit. Intake keeps the authored order of
`dimensions`, `units` and each `terms` list. It does not apply FR-142's
base-dimension rule or first-defect order: FR-322 "Model-owned members" reads
the admitted dimensions and units into the unit graph of the checked package,
and FR-142 checks that graph.

A native reference declares no node: intake builds no declaration for it, no
declaration key names the identity `quire/native`, and it resolves the same
way in every package. The domain package identities `quire/native` and `quire/unit`, and any
identity beginning `quire/native/` or `quire/unit/`, are reserved: a selection
that names one refuses before any other admission check, so no package can
declare a node whose identity reads as a native or a unit reference.

Intake admits a `typeRef` naming `Rational`, `Decimal` or `Text` as it admits
any other native name: the name is a type that exists, and a `typeRef` carries
no parameters. The bounds of a `Text`, `Integer` range and the like come from a
value type of the package that the field names (FR-322 "Model-owned members"),
not from the reference. Whether a member typed by an unparameterized
`Rational`, `Decimal` or `Text` can be used is decided where it is used, at
member resolution: a bare `Text` field refuses there
`ill_typed`/`operator-ineligible` (FR-322-AC-31), and intake raises nothing.
A field typed by a native value type declares no type node of its own. A
refusal of a `typeRef` names the field node and `Field.typeRef`, the operation
node and `Operation.params[i].typeRef`, or the operation node and
`Operation.returns.typeRef`.

A field `typeRef`, an operation parameter or an operation result that names a
variant type is admitted and raises no refusal of its own (FR-208), including
when the variant type's own declaration is refused. A field, parameter or result
that names an event type, state machine, process, persistence interface or
namespace keeps its refusal: for a field it names the field node and
`Field.typeRef`; for a parameter it names the operation node and
`Operation.params[i].typeRef`, `i` the parameter's index; for a result it names
the operation node and `Operation.returns.typeRef`.

A field and an operation each carry an optional `redefines` member holding
one IR node identity: the inherited member that it redefines
(`quire.model.complete/v1` Redefines row). Intake SHALL read a field's
redefinition only from `Field.redefines` and an operation's only from
`Operation.redefines`. An extension payload of the owning type declares no
redefinition to intake, whatever its identity or its `required` flag, so an
operation whose redefinition appears only in an extension redefines nothing,
and a consumer that does not admit a `required` extension refuses the
document before intake reads it, `unknown_required_feature`/`unsupported-feature`
naming the extension identity and the consuming scope, at the owning type node
and its `extensions` member
([FR-133-AC-3](../source-package/FR-133-extend-language-uniformly.md)). An
operation that shares a supertype operation's name stays a second member of
that name (FR-322 "Model-owned members" step 3). An `Operation.redefines` that
names no node of the package refuses `missing_declaration`/`missing-name` at
the operation node and `Operation.redefines`. Whether the named member is
inherited by the owner is phase 4's check (`redefinition-target`).

Within one node, the failures of a variant type report in this order: the
construct shape, then the empty `variants` list, then each case's repeated
name, in `variants` order, and then the source span of each case, in
`variants` order (these name the type node). A case node reports its identity
and then its payload. Nodes report in node order as above, so the type node's
refusals come first and each case node's follow in `variants` order. This order
replaces table order for the variant rows.

A package whose cases carry equal names under one identity is refused once as
that collision (`conflicting-binding` at the shared identity), and no
declaration is built.

When FCD extraction, or the FCD semantic IR reader (filament-core-data FR-050,
FR-146) over the selected document, reports one of the following semantic diagnostics, the
FR-154 intake projection SHALL preserve the affected IR node and `origin` and
map it to the QSL refusal pair used by TC-235:

| FCD diagnostic | QSL refusal pair | Scope |
| --- | --- | --- |
| `UNRESOLVED_CONSTRUCT_REF` | `missing_declaration`/`missing-name` | A construct member reference names no declared node or member, including an occurrence field, transition state or trigger, or emitted/consumed type. |
| `DANGLING_CLAUSE_REF` | `missing_declaration`/`missing-name` | A transition guard names no clause of its owning type. |
| `CLOCK_POSITION_INVALID` | `invalid_model_binding`/`malformed-declaration` | `ix://quire/native/Clock` as a reference `target`, a sequence `items`, a map `values` or a constraint `appliesTo`, at the member the Clock row above names. |
| `INVALID_CLOCK_MEMBER` | `invalid_model_binding`/`malformed-declaration` | A clock member shape member, at its `Field.*`, `Operation.params[i].*` or `Operation.returns.*` path. |
| `INVALID_UNIT_GRAPH` | `invalid_model_binding`/`malformed-declaration` | A dimension or unit graph defect, at the member path of its FCD pointer. |
| `INVALID_OPERAND`, `INTEGER_OUTSIDE_I128` | `invalid_model_binding`/`malformed-declaration` | A term `exponent`, or a unit `scale` or `offset`, at `Dimension.terms[t].exponent`, `Unit.scale` or `Unit.offset`. |
| `UNRESOLVED_DIMENSION_REF` | `missing_declaration`/`missing-name` | A term's or unit's `dimension` names no dimension node, at `Dimension.terms[t].dimension` or `Unit.dimension`. |
| `UNRESOLVED_UNIT_REF` | `missing_declaration`/`missing-name` | A unit's `target` names no unit node, at `Unit.target`. |
| `DUPLICATE_IDENTITY` on two dimension or two unit nodes | `invalid_model_binding`/`conflicting-binding` | The later node's `Dimension.identity` or `Unit.identity`, the "Two nodes share one identity" row. |
| `INVALID_OCCURRENCE_FIELD` | `invalid_model_binding`/`malformed-declaration` | An occurrence field resolves to a field whose type, including aliases, is not scalar `datetime`; a missing target remains `UNRESOLVED_CONSTRUCT_REF`. |

Every declaration refusal retains the IR node identity and the node's FCD
`origin` verbatim (filament-core-data `schema/semantic/v1/common.schema.json`,
`$defs.origin`): a source origin gives `sourceIdentity`, `path`, `startLine`,
`startColumn` and, when present, `endLine` and `endColumn`, as given; a
generated origin gives `generatorIdentity`, `generatorVersion` and
`inputIdentities`, as given, with no span. The artifact id the object id rule checks is the `<Name>` segment of the IR node
identity; a FCD document carries no separate artifact id member. Nothing is synthesized: a node whose `origin` is missing or malformed
retains no origin. Intake
reports every declaration refusal in node order, and then no later phase runs.
Within one node, every check of the table that fails reports, in table order,
so the object id check reports first. Several failures of one row within one node report in member-path order: a member path is a sequence of segments, each a member name or a numeric index, where `[]` denotes the index-free segment of a list member and sorts as an index before every numeric index; paths compare segment by segment, a member name before a numeric index, member names by their UTF-8 bytes and indices numerically, and a proper prefix orders first. A reference that names a node refused by
an earlier row of the table is not reported again as
`missing_declaration`/`missing-name`; that node's own refusal stands for it.
An admission refusal ends intake with no declaration.

A model-declaring artifact is an artifact whose type is an object type of any
spec-objects module (spec-objects-architecture, spec-objects-business, …). Its artifact id is an object id: ASCII letters, digits and underscores,
starting with a letter (`^[A-Za-z][A-Za-z0-9_]*$`). An id such as `entity-001`
is not an object id, and intake refuses the node it declares
`invalid_model_binding`/`malformed-declaration`, naming the IR node identity
and its `origin`. Because every object id is a `member-name`, a
clause names each declaration by qualified name, such as `M::sys_pump`.
Artifact-type ids such as `FR-100` declare no model node and are outside this
rule.

A declaration's IR node identity is the identity filament-core-data FR-095
mints (the closed slot list of its "Node identities" section), except for the
four kinds below, rooted at
`ix://<package identity>/`, with `<Name>` the declaring artifact's id passed
through verbatim, every owner-nested member name passed through verbatim, and
every other part slugged. A type's identity is
`ix://<package identity>/<artifact id>`, and the alias of a constrained
field is `ix://<package identity>/<Name><Field>` (`<Field>` slugged with its
first character upper-cased), which names no slot segment of its own. A field, an operation and an operation
parameter nest under their owner: `<owner identity>/<field>`,
`<owner identity>/<operation>` and `<operation identity>/<param>`. Their last
segment is the member's declared `name` verbatim, never slugged, and matches
the member-name rule `^[A-Za-z][A-Za-z0-9_]*$`: a field `created_at` of
`Order` is `ix://<package identity>/Order/created_at`, an operation
`reset_all` is `.../Order/reset_all` and its parameter `created_at` is
`.../Order/reset_all/created_at`. A field or an operation whose `name` fails
the member-name rule or differs from the last segment of its identity refuses
`invalid_model_binding`/`malformed-declaration`, naming the member node and
`TypeDefinition.fields[j].name` or `TypeDefinition.operations[k].name`, after
the repeated-identity check of its owner, which suppresses it for both
colliding members as it does for parameters. Because a member name is not
slugged, a field `created_at` and a state `created_at` of one owner share one
identity and refuse as that collision. A clause, a
state, a transition and a step nest under their owner too: each is exactly
`<owner identity>/<member name>`, the owner being the identity of the type that
declares it and the member name matching `^[A-Za-z][A-Za-z0-9_]*$`. A clause's
`clauseId`, a state's `name` and a step's `name` equal the member name of its
identity; a transition has no name of its own and its member name is any such
identifier not used by another member of its owner (a field, operation, clause,
state, transition or step), chosen by the producer of the document and never
derived from `from`, `to` and `trigger`. This departs from filament-core-data
FR-095, which mints these four kinds as global slots
(`clause/<Name>-<clauseId>`, `state/<Name>-<state>`,
`transition/<Name>-<from>-<to>-<trigger>`, `step/<Name>-<step>`): FR-095 and
its minter must mint the nested form, with a member name for each transition,
before a lifted package that declares a clause, state machine or process is
admitted. A mismatch between `clauseId`, a state's `name` or a step's `name` and
the member name of its identity is `invalid_model_binding`/`malformed-declaration`
at `TypeDefinition.clauses[i].clauseId`, `TypeDefinition.states[i].name` and
`TypeDefinition.steps[i].identity` respectively. An identity that is not
exactly the owner's identity, `/` and one such member name, or whose owner is
not the declaring type, is `invalid_model_binding`/`malformed-declaration`, so
the same member name under two owners names two nodes. Every other node kind
mints under its own slot and is not owner-nested: a relationship is
`ix://<package identity>/relationship/<Name>-<verb>-<TargetName>` with
`<TargetName>` the target artifact's id, a constraint is
`ix://<package identity>/constraint/<Name>-<field>-<keyword>`, an enumeration
variant is `ix://<package identity>/variant/<Name>-<value>`, a dimension is
`ix://<package identity>/dimension/<name>` and a unit is
`ix://<package identity>/unit/<name>`, `<name>` the node's `name` verbatim
(filament-core-data FR-146).
A relationship is declared in its source type's `relationships`
list (with `sourceEnd` and `targetEnd`), not under the source type's identity. The artifact title is the declaration's
`displayName`, used for presentation only and never part of a key or a name
lookup. Retitling an artifact keeps the declaration keys of the type it declares
and of that type's members. Changing an artifact's id declares a new type: its
declaration key and its members' keys change, and a reference to the old key
refuses `missing_declaration`/`missing-name`. Renaming a member changes that
member's key.

Intake binds each IR type definition and population node by the `meaning` of
its construct. The node's `kind` is `{module, name}` and names one entry of the package's
embedded `constructs` table; that entry's `meaning` is a Quire meaning id
(FR-208). The kind name and module select the construct entry only and never
select a meaning.

A declaration key carries no digest. Two domain package documents with equal nodes yield
equal declaration keys. The ModelSelection, which binds the digest, separates
them.

Intake charges `normalize.record` under `ModelNormalizationLimitsV1` of
`quire.value.accounting/v1` once per declaration, ascending by declaration key.
Admission checks are decided before the first charge.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-154-AC-1 | An artifact bundle whose extraction and lift yield a valid domain package is admitted, and intake builds exactly one original declaration per IR node, keyed by package identity and IR node identity. | Test (TC-234) |
| FR-154-AC-2 | A wrong digest domain, a missing package and a package whose identity differs each refuse with its named cause before any declaration is built, a package supplied under the selected digest with the selection's identity is admitted, and a supplied document is admitted whatever its own canonical digest is. | Test (TC-234) |
| FR-154-AC-3 | A malformed IR node or a dangling reference refuses with the IR node identity and its `origin`, and every such refusal is reported in node order. | Test (TC-234) |
| FR-154-AC-4 | Publishing a domain package document with equal nodes and a different digest leaves every declaration key and effective declaration unchanged, while the ModelSelection, the effective view and the object universe change. | Test (TC-234) |
| FR-154-AC-5 | Intake at its exact `normalize.record` bound completes, and the one-less run is incomplete at `normalize.record`. | Test (TC-234) |
| FR-154-AC-6 | Retitling an artifact leaves every declaration key unchanged. Changing an artifact's id changes the declaration key of the type it declares and of that type's members, leaves every other declaration key unchanged, and a reference to the old key refuses `missing_declaration`/`missing-name`. | Test (TC-234) |
| FR-154-AC-7 | A type definition or population node whose kind names no construct entry, or whose construct has no Quire meaning id or an id outside FR-208, refuses `invalid_model_binding`/`malformed-declaration` with the IR node identity and its `origin`, and no meaning is inferred from its kind name. | Test (TC-234) |
| FR-154-AC-8 | A type definition or population node declared by an artifact whose id does not match `^[A-Za-z][A-Za-z0-9_]*$`, such as `sys-pump`, refuses `invalid_model_binding`/`malformed-declaration` with the IR node identity and its `origin`, a reference to that node reports no `missing_declaration`/`missing-name`, and a second failing check on the same node reports after the id refusal; the artifact id `sys_pump` is admitted and a clause names its declaration `M::sys_pump`. | Test (TC-234) |
| FR-154-AC-9 | The published domain package document of `acme/orders` (`domain-package-acme-orders.json`) carries exactly FR-095's identity forms: types `ix://acme/orders/<artifact id>`, fields, operations and parameters nested under their owner, and relationships `ix://acme/orders/relationship/<Name>-<verb>-<TargetName>` (`Order-order-Order`, `Order-invoiced-Invoice`, `Order-follows-Order`, `Order-twin-Order`), the constrained-field alias `ix://acme/orders/OrderTotal`, none of them owner-nested. | Test (TC-280) |
| FR-154-AC-10 | A relationship declaration of the selected document refuses at the selection row (`/lock/model_selections/<i>`), in node order and, within a node, in the order malformed declaration, unresolved end type, duplicate identity, reversed multiplicity: an identity outside the package's global `relationship` slot (owner-nested, foreign-package or wrong-slot), a missing or invalid `direction`, `category`, `composite` or `origin`, a missing, null, empty or non-string `sourceEnd.role`, a present null, empty or non-string `targetEnd.role` (an absent `targetEnd.role` is valid), a source end naming another owning type, a malformed end or multiplicity, and an end type naming a relationship or another declaration of the wrong meaning each refuse `invalid_model_binding`/`malformed-declaration`; an end type naming no declaration refuses `missing_declaration`/`missing-name`; a repeated relationship identity, including under two owners, refuses `invalid_model_binding`/`conflicting-binding`, and two missing identities are malformed declarations, not conflicting ones; `lower > upper` alone refuses `invalid_model_binding`/`unpreserved-model-meaning`. The published `adverse.json` `relationship_document_cases` hold one entry per refusal and three admitted entries. Each refusal retains the relationship's IR identity and its `origin` verbatim: `declaration_origin_cases` `refusal-retains-source-origin` (a source origin with its `sourceIdentity`, `path` and span) `refusal-retains-generated-origin` (a generated origin with its members and no span) and `refusal-retains-no-malformed-origin` (a malformed origin, refused `malformed-declaration`, retained as none). | Test (TC-280) |
| FR-154-AC-11 | A variant type with no case, a case with no source span, a case whose identity is not `variant/<Name>-<value>` of its owner and name, and a construct of a shape other than `enumeration` each refuse `invalid_model_binding`/`malformed-declaration` (except a case already refused by a same-identity collision, FR-154-AC-13), at the node and member path FR-154 names (`TypeDefinition.kind`, `TypeDefinition.variants`, `TypeDefinition.variants[i].origin`, `Variant.identity`); a field `typeRef`, an operation parameter and an operation result naming a variant type are admitted and raise no refusal; a case with a resolving `payloadType` refuses `unsupported_construct`/`declaration-form` at its case node and `Variant.payloadType`, never `invalid_model_binding`; two cases with equal names and distinct identities refuse the later case `invalid_model_binding`/`conflicting-binding` at the type node and `TypeDefinition.variants[i]` and raise no other refusal for it; within the type node the failures report in the order shape, empty list, repeated names, source spans, and each case node then reports identity and payload after the type node's refusals; reordering the `variants` list of an admitted type changes no declaration key. | Test (TC-235, TC-939) |
| FR-154-AC-12 | A clause, state, transition and step identity is exactly `<owner identity>/<member name>`: `ix://test/orders/OrderLifecycle/NotYetPlaced`, `.../OrderLifecycle/Draft`, `.../OrderLifecycle/<transition member name>` and `.../Fulfilment/receive_order` are admitted, a clause, state or step whose `clauseId` or `name` differs from the member name of its identity refuses `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.clauses[i].clauseId`, `TypeDefinition.states[i].name` and `TypeDefinition.steps[i].identity`, a state of `OrderLifecycle` with the identity `.../Fulfilment/Draft` (the wrong owner) refuses `malformed-declaration` at `TypeDefinition.states[i].identity`, a state, clause, step or transition whose member name is also a field or operation of its owner (state `cancel` beside operation `cancel`) refuses `invalid_model_binding`/`conflicting-binding`, and the global-slot spellings `clause/<Name>-<clauseId>`, `state/<Name>-<state>`, `transition/<Name>-<from>-<to>-<trigger>` and `step/<Name>-<step>` each refuse `invalid_model_binding`/`malformed-declaration` at the member's `identity`. Two transitions of one machine whose slugged global spellings would be equal (`a_b → c` and `a → b_c` on one trigger) are two distinct nodes with distinct keys. | Test (TC-235) |
| FR-154-AC-13 | Two fields of one owner, two operations of one owner, a field and an operation of one owner, two parameters of one operation and two variant cases of one type that share one identity each refuse once, `invalid_model_binding`/`conflicting-binding`, naming the shared identity at the later member's path: `TypeDefinition.fields[j]`, `TypeDefinition.operations[k]` (the operation, for a field and an operation), `TypeDefinition.operations[k].params[s]` and `TypeDefinition.variants[i]`; the refusal carries the code, cause, shared identity and path only, and the collision suppresses the identity, origin and payload checks of both colliding cases (FR-154-AC-11 and FR-208-AC-15 apply to a case not already refused by a collision), so exactly one refusal is reported whichever of the two cases carries the correctly minted identity, and no declaration of the package is admitted. The same members with distinct identities are admitted, and one member name used in two different owners is two distinct identities and no collision. | Test (TC-235) |
| FR-154-AC-14 | An operation parameter whose `name` is absent, not a string, null, empty, begins with a digit, contains a separator or differs from the last segment of its identity, and which is not one of two parameters sharing an identity, refuses `invalid_model_binding`/`malformed-declaration` at the selection row, naming the operation's identity and `TypeDefinition.operations[k].params[s].name`; two parameters of one operation with one identity refuse once, `invalid_model_binding`/`conflicting-binding`, naming the shared parameter identity at the later parameter's `TypeDefinition.operations[k].params[s]`, and that collision suppresses the name check of both, so a malformed name on either colliding parameter is not reported; distinct valid names, including names differing only in letter case, are admitted and no relationship declaration outcome changes. | Test (TC-280) |
| FR-154-AC-16 | An operation whose `Operation.redefines` names a supertype's operation (`Sub/size` naming `Order/size`) is admitted with that redefinition. The same operation with its `redefines` member removed redefines nothing, and so does one whose redefinition is listed only in an extension payload of its owning type, `required` true or false when the consumer admits the extension; a consumer that does not admit a `required` extension refuses the document before intake, `unknown_required_feature`/`unsupported-feature` at the owning type node and its `extensions` member. An `Operation.redefines` naming no node of the package refuses `missing_declaration`/`missing-name` at the operation node and `Operation.redefines`. | Test (TC-280) |
| FR-154-AC-17 | A field typed `ix://quire/native/Integer`, a field typed `ix://quire/native/Timestamp`, a field typed `ix://quire/native/UUID`, a field typed `ix://quire/native/Text`, a field typed `ix://quire/native/Rational`, a field typed `ix://quire/native/Decimal` and a field typed `ix://quire/unit/ms` are admitted, each native resolving to that native value type and the unit one left unresolved by intake, with no declaration built for a native reference and no declaration key naming `quire/native`; a package type with artifact id `Integer` is admitted beside them and a field typed `ix://test/orders/Integer` resolves to that package type, not to the native `Integer`. A field typed `ix://quire/native/Int`, `ix://quire/native/integer`, `ix://quire/native/JsonObject` or `ix://quire/unit/`, or typed `Integer` with no URI, refuses `invalid_model_binding`/`malformed-declaration` at the field node and `Field.typeRef`; an operation parameter and an operation result typed `ix://quire/native/Int` refuse the same at the operation node and `Operation.params[i].typeRef` and `Operation.returns.typeRef`, while the same parameter and result typed `ix://quire/native/Text` are admitted; a selection naming the identity `quire/native` and a selection naming the identity `quire/unit` each refuse `invalid_model_binding`/`malformed-declaration` at the selection row before its digest domain or bytes are read, and build no declaration. | Test (TC-234) |
| FR-154-AC-18 | A field typed `ix://quire/native/Clock` with multiplicity `{1, 1, false, false}`, presence `required`, `nullable` `false` and `defaultKind` `none` is admitted with no type declaration built for it and no declaration key naming `quire/native`. A clock field with multiplicity `{0, 1, false, false}`, presence `optional`, `nullable` `true`, `defaultKind` `semantic` with a `defaultValue`, `unit` `ms`, a `decimal` policy, a `textProfile` and a `min` constraint refuses `invalid_model_binding`/`malformed-declaration` once at each of `Field.constraints`, `Field.decimal`, `Field.defaultKind`, `Field.defaultValue`, `Field.nullable`, `Field.textProfile` and `Field.unit`, in that member-path order, and raises no other refusal at those members (presence `optional` and the multiplicity are admitted); `ix://quire/native/Clock` as a reference target and a constraint `appliesTo` each refuse `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.target` and `Constraint.appliesTo`; `ix://quire/native/clock` refuses at `Field.typeRef`. | Test (TC-234) |
| FR-154-AC-19 | A package carrying base dimension `Time` (`ix://test/orders/dimension/Time`), derived dimension `Rate` (terms `[{Time, "-1"}]`), root unit `s` and unit `ms` (`target` `s`, `scale` `1/1000`) is admitted with one original declaration per dimension and unit node, keyed `(test/orders, ix://test/orders/dimension/Time)`, `(test/orders, ix://test/orders/dimension/Rate)`, `(test/orders, ix://test/orders/unit/s)` and `(test/orders, ix://test/orders/unit/ms)`, each charged `normalize.record`. Each of these single defects refuses exactly one `invalid_model_binding`/`malformed-declaration`: a unit with `scale` `0` at `Unit.scale`, the derived `Rate` marked `base` `true` (a base dimension with a term) at `Dimension.terms`, a unit whose identity is `ix://test/orders/unit/msec` for name `ms` at `Unit.identity`, a unit named `m-s` at `Unit.name`, a dimension whose `base` is the string `"true"` at `Dimension.base`, a unit with no `origin` at `Unit.origin`, and a two-unit target cycle at the least-index unit's `Unit.target`; a unit whose `dimension` names no dimension node and a unit whose `target` names no unit node refuse `missing_declaration`/`missing-name` at `Unit.dimension` and `Unit.target`; two units named `ms` with one identity refuse once, `invalid_model_binding`/`conflicting-binding`, at the later unit's `Unit.identity`; a dimension `Time` with no unit is admitted. | Test (TC-234) |
| FR-154-AC-20 | A field `created_at` of an owner `O` with identity `ix://test/orders/O/created_at`, an operation `reset_all` with identity `.../O/reset_all` and its parameter `created_at` with identity `.../O/reset_all/created_at` are admitted, each keyed by that verbatim identity; the same field with identity `.../O/created-at` refuses `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.fields[j].name`, the same operation with identity `.../O/reset-all` at `TypeDefinition.operations[k].name`, and the same parameter with identity `.../O/reset_all/created-at` at `TypeDefinition.operations[k].params[s].name`; a field `created_at` beside a state `created_at` of one state machine, both with identity `.../O/created_at`, refuses once, `invalid_model_binding`/`conflicting-binding`, at `TypeDefinition.states[i]`; a field whose `name` is absent, is the number `42`, is empty, or is `_tmp` with identity `.../O/_tmp`, and an operation whose `name` is `_tmp` with identity `.../O/_tmp`, each refuse `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.fields[j].name` or `TypeDefinition.operations[k].name`. | Test (TC-235) |
| FR-154-AC-23 | `ix://quire/native/Clock` is admitted, with no declaration built for it, as an operation parameter `typeRef`, an operation result `typeRef`, an alias `target` (alias `Timer`) and as the `typeRef` of a field of that alias; a field typed `Clock` with presence `optional` is admitted as `Option<Clock>`; an operation result `Clock` with multiplicity `{0, 1}` is admitted as `Option<Clock>`. A clock parameter carrying `unit` `ms`, `defaultKind` `semantic`, a `min` constraint or `nullable` `true`, and a clock result with `nullable` `true`, each refuse `invalid_model_binding`/`malformed-declaration` at the member path of the broken shape member (`Operation.params[i].unit`, `Operation.params[i].defaultKind`, `Operation.params[i].constraints`, `Operation.params[i].nullable`, `Operation.returns.nullable`). | Test (TC-234) |
| FR-154-AC-24 | `ix://quire/unit/ms` is admitted, left unresolved by intake, as a field, an operation parameter, an operation result, an alias `target`, a sequence `items` and a map `values`; a unit-typed member carrying `unit`, a `min` constraint, a `decimal` policy or a semantic default refuses `invalid_model_binding`/`malformed-declaration` once at each broken member path; `ix://quire/unit/ms` as a reference `target` and as a constraint `appliesTo` each refuse `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.target` and `Constraint.appliesTo`. | Test (TC-234) |
| FR-154-AC-25 | A variant case whose `payloadType` is `ix://quire/native/Clock` or `ix://quire/unit/ms` is read by the variant payload row as any payload type is, `unsupported_construct`/`declaration-form` at `Variant.payloadType`, and never `malformed-declaration`. | Test (TC-234) |
| FR-154-AC-26 | A field typed `ix://quire/native/Clock` with multiplicity `{0, 3, true, false}` is a `Sequence<Clock>[0, 3]` clock member, with `{0, 3, false, true}` a `Set<Clock>[0, 3]`, with `{0, 3, false, false}` a `Bag<Clock>[0, 3]`, with `{0, 3, true, true}` an `OrderedSet<Clock>[0, 3]` and with an unbounded upper bound, ordered and not unique, a `Sequence<Clock>`; an operation parameter and an operation result typed `Clock` with multiplicity `{0, 2, true, false}` are admitted the same way; each is admitted with no declaration built and no refusal at `Field.multiplicity`; a clock field with multiplicity `{3, 1, true, false}` refuses `invalid_model_binding`/`unpreserved-model-meaning` at `Field.multiplicity`. | Test (TC-234) |
| FR-154-AC-27 | A sequence `items` and a map `values` typed `ix://quire/native/Clock`, and the same typed by an alias of `Clock`, are admitted with no declaration built for the clock and read as `Sequence<Clock>` and `Map<Text, Clock>`; a field typed by such a sequence with multiplicity `{0, 2, true, false}` is a `Sequence<Sequence<Clock>>[0, 2]`; a record value type with a clock field, held by a field of multiplicity `{0, 2, true, false}`, is a `Sequence<Timer>[0, 2]` clock collection. | Test (TC-234) |

## Dependencies

- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) owns the
  model intake design.
- [FR-208](../../objects/foundation/FR-208-quire-meaning-vocabulary.md) owns
  the Quire meaning ids intake binds.
- The filament-core-data Semantic IR contract owns the `operation` member
  set. Intake reads `Operation.redefines` from the IR `operation` member of
  that name, so a producer's contract has to carry it for a document to
  declare an operation redefinition.
- [Complete model definition](../../../proposals/quire-v1/definitions/model-complete.md)
  owns the intake and feature rules.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns the
  `model` declaration.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
