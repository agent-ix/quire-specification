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
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
  - target: ix://agent-ix/quire-specification/FR-415
    type: references
---

# FR-154: Admit the domain model from a domain package

## Description

When a package imports a domain package, QSL model intake SHALL admit the
package selected by its identity, SHALL record the identity and the document's
identity in the package lock and SHALL build one Quire
model declaration for each IR node, refusing any invalid node with its IR node identity and
`origin`.

## Inputs

The source `model` declaration with its domain package identity alone; the domain package bytes supplied under that identity; and the selected `quire.model.complete/v1`
definition.

## Outputs

The original declarations of one ModelSelection (FR-321), each keyed by
`(domain package identity, IR node identity)`, or located refusals.

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
   one semantic IR 2.0.0 domain package, serialized with RFC 8785 JCS.
4. **Admission.** Rule `quire.model.normalize.intake/v1` checks the selection
   and the package bytes in the order the selected definition fixes.
5. **Declarations.** Intake builds one original declaration per IR node,
   ascending by declaration key, and rule `quire.model.features/v1` checks
   each declaration.

### Model selection in source

The source `model` declaration is `model <alias> = "<identity>";`. It names the
domain package identity and nothing else, exactly as the complete-unit `import`
names a library identity and nothing else (FR-307 "Import binding and export
identity"): it carries no version and no digest, and a `version` or `digest`
token, or any other token after the identity string, is
`refused { code: invalid_syntax, cause: unexpected-token }` at that token. A
`model` whose identity string is absent or is not a string is
`refused { code: invalid_syntax, cause: unexpected-token }` at the offending
token, or `unexpected-end` at the end of the source; the declaration is never
dropped without a diagnostic. A digest is not part of the grammar.

The package input maps each domain package identity to one domain package
document; a supplier states no digest and no version. At most one document is
supplied per identity; a second document supplied under an identity already
supplied is a duplicate definition and
`refused { code: invalid_package, cause: conflicting-definition }` before intake
runs, naming both. One document per identity is the rule: a package that needs
two releases of one domain package is not supported in v1, and a source that
selects one identity twice is `duplicate_selection`/`duplicate-identity`
(FR-321-AC-5). Which release a supplier offers under an identity is chosen by the
supplier or resolver outside the compiler; the compiler uses what it is given.
Intake looks up the document by the selection's identity and compares the
document's own identity with the selection, then records the selection as the
`model_root` node of identity (FR-321, FR-322); the lock records nothing.

The lock holds no model record. The `model_root` node of the identity is the only
record of the selection. No lock row or version of a model
exists. A replay recompiles against the supplied document and uses the result
(FR-323). The package document's own `version` member is not read by intake
and is not recorded.

Intake keeps the authored order of every type's `fields` and `supertypes`
arrays (FR-208 "Effective field order" reads it). FR-150's sorting by identity
orders the effective view's set of effective declarations, not these arrays.

Admission refusals, in order:

| Check | Refusal |
| --- | --- |
| The selection's domain package identity equals `quire/native` or `quire/unit`, or begins `quire/native/` or `quire/unit/` (a reserved identity) | `invalid_model_binding`/`malformed-declaration` |
| The package input supplies bytes under the selection's identity | `missing_import`/`missing-selection` at the model's identity string |
| The package's own identity equals the selection's | `invalid_model_binding`/`wrong-model-selection` |

Declaration refusals:

| Failure | Refusal |
| --- | --- |
| A type definition's or population node's artifact id does not match `^[A-Za-z][A-Za-z0-9_-]*$` | `invalid_model_binding`/`malformed-declaration` |
| A tuple value type's nested identity is not `<package identity>/<tuple name>`, or a tuple name is not a valid artifact id | `invalid_model_binding`/`malformed-declaration` |
| A type definition's or population node's `kind` names no entry of the `constructs` table, or its construct has no `meaning` or a `meaning` that is not a Quire meaning id (FR-208) | `invalid_model_binding`/`malformed-declaration` |
| An IR node is not valid for its construct's meaning, including a reference that names a node of the wrong meaning and a `supertypes` entry that names a node of another meaning (FR-208) | `invalid_model_binding`/`malformed-declaration` |
| An operation parameter's `name` is absent, is not a string, is empty, does not match `^[A-Za-z][A-Za-z0-9_]*$`, or differs from the last segment of the parameter's identity `<operation identity>/<param>`, on a parameter not refused as a repeated identity of its operation (checked after that row and before the frame entries) | `invalid_model_binding`/`malformed-declaration` |
| A field's or an operation's `name` is absent, is not a string, is empty, does not match `^[A-Za-z][A-Za-z0-9_]*$`, or differs from the last segment of its identity `<owner identity>/<member name>`, on a member not refused as a repeated identity of its owner (checked after the "Two nodes share one identity" row below) | `invalid_model_binding`/`malformed-declaration` |
| A field, parameter, result, alias `target`, sequence `items`, map `keys`, map `values`, option `inner`, tuple position `typeRef` or variant case `payloadType` that is none of `ix://quire/native/<Name>` with `<Name>` a native value type name, `ix://quire/unit/<name>` with a non-empty `<name>`, and a package value type identity (including a tuple's owner-nested identity). This row validates reference form only: tuple position admission is further restricted by FR-322, which refuses Clock as `invalid_model_binding`/`malformed-declaration`; variant payload admission is governed by AC-25. | `invalid_model_binding`/`malformed-declaration` |
| `ix://quire/unit/<name>` as a reference `target` (a quantity has no identity, so no reference names one), a constraint on a clock whose keyword is neither `min` nor `max`, a constraint on a unit whose keyword is neither `min` nor `max`, or a clock member (a field or parameter typed `ix://quire/native/Clock`, or a result of that type) that breaks the clock member shape below | `invalid_model_binding`/`malformed-declaration` |
| A dimension or unit node with a malformed member or a unit-graph defect: a `name` outside `^[A-Za-z][A-Za-z0-9_]*$`, a non-boolean `base`, an `identity` that is not `ix://<package identity>/dimension/<name>` or `ix://<package identity>/unit/<name>`, a base dimension with terms, a term exponent that is not a canonical integer string, `0` or outside `i128`, terms out of order or repeated, a term cycle, a `scale` or `offset` that is not a canonical rational string or is outside `i128`, a zero `scale`, a `target` in another dimension, a second root of one dimension, a root whose `scale` is not `1` or `offset` not `0`, a target cycle, a `dimension` or `target` that is not an identity, or a missing or malformed `origin` | `invalid_model_binding`/`malformed-declaration` |
| A dimension term's `dimension`, a unit's `dimension` or a unit's `target` names no dimension or unit node of the package | `missing_declaration`/`missing-name` |
| A supertype, field type, tuple position `typeRef`, subsets, redefines, relationship end, frame entry, population member, `occurrenceField`, transition `from`, `to`, `trigger`, `guard` or `emits` entry, step `consumes` or `emits` entry, `persists` entry or namespace `members` entry names no node or member of the package | `missing_declaration`/`missing-name` |
| A node of a business model meaning (FR-208) declares a form that meaning does not support: a non-empty `supertypes` or a `quire` clause of a persistence interface or namespace, an operation of a record value type, or an operation contract of a persistence interface | `unsupported_construct`/`declaration-form` |
| A tuple value type (FR-208) has no position, a position index is not a contiguous zero-based ordinal, a position has no `typeRef`, its type is unsupported, or the type declares fields, identity fields, operations, supertypes or clauses | `invalid_model_binding`/`malformed-declaration` |
| An option value type ("Option value types" below) whose `inner` is absent, `null`, empty, not a string, an array, or an object; whose `inner` names (directly or through an alias chain) an option value type, `ix://quire/native/Clock` or a node that is no value type (an event type, state machine, process, persistence interface or namespace); that declares fields, identity fields, operations, supertypes, clauses, positions, variants, `items`, `keys`, `values`, `bound` or `target` member, or a non-empty `constraints`; or a constraint whose `appliesTo` names an option value type | `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.inner` (at the declared member's path for a declared member, `TypeDefinition.constraints` for a constraint of the type and `Constraint.appliesTo` for a constraint naming it); this row owns every `inner` form failure of an option value type and the reference-form row above does not report it a second time |
| An option value type's `inner` is a well-formed package type identity that names no node of the package | `missing_declaration`/`missing-name` at `TypeDefinition.inner` |
| A map type ("Map types" below) with no `keys`, `values` or `bound` member; whose `bound` is not an object of exactly an integer `lower` of at least `0` and, when present, an integer `upper` of at least `0`; that declares fields, identity fields, operations, supertypes, clauses, positions, variants, `items`, `inner` or `target`; or whose `keys` names a node that is no type (an event type, state machine, process, persistence interface or namespace) | `invalid_model_binding`/`malformed-declaration` at the absent, malformed or declared member (`TypeDefinition.keys`, `TypeDefinition.values`, `TypeDefinition.bound` or the extra member) |
| A type definition of any kind other than `map` that declares `keys` or `bound` | `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.keys` or `TypeDefinition.bound` |
| A map type's `keys` or `values` is a well-formed package type identity that names no node of the package | `missing_declaration`/`missing-name` at `TypeDefinition.keys` or `TypeDefinition.values` |
| A map type's `keys` names (directly or through an alias chain) a type that is not a scalar key type ("Map types" below) | `ill_typed`/`operator-ineligible` at `TypeDefinition.keys` |
| A map type's `bound` has `lower > upper`, or has no `upper` and a `lower` above `0` | `invalid_model_binding`/`unpreserved-model-meaning` at `TypeDefinition.bound` |
| A field of option value type (directly or through an alias chain) with presence `optional` and multiplicity `[1, 1]` (lower and upper `1`, any ordered and unique flags), which would be `Option<Option<T>>` | `invalid_model_binding`/`malformed-declaration` at `Field.presence` |
| A `reference` whose `target` names an option value type (a reference names an object) | `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.target` |
| A variant case's `payloadType` is unresolved or names a type not admitted as a value type | `unsupported_construct`/`declaration-form` |
| A variant type (FR-208) declares no case, a case has no source span, a case's identity is not the variant slot identity of its owner and its name, or its construct's shape is not `enumeration` | `invalid_model_binding`/`malformed-declaration` |
| Two nodes share one identity, including two dimensions or two units with one identity (the refusal names the shared identity at the later node's `Dimension.identity` or `Unit.identity`), two fields of one owner (the refusal names the shared identity at `TypeDefinition.fields[j]`), two operations of one owner or a field and an operation of one owner (at `TypeDefinition.operations[k]`; in a field and operation pair the operation, declared after the fields, is the one refused), two parameters of one operation (at `TypeDefinition.operations[k].params[s]`), two tuple positions with one ordinal identity (at the later position's path), two variant cases of one type or of different types (at the later case node's owner's `TypeDefinition.variants[i]`), two states of one owner with one name (the refusal names the shared identity at `TypeDefinition.states[i]`), two steps of one owner with one name (the refusal names the process at `TypeDefinition.steps[i]`), or a clause, state, transition or step whose member name is also the name of a field or operation of its owner (a state or transition names its own identity at `TypeDefinition.states[i]` or `TypeDefinition.transitions[i]`, the later declaration in document order, operations before states) | `invalid_model_binding`/`conflicting-binding` |
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
is reported at the owner of the package's `model_root` node (`/semantic_graph/nodes/<i>/owner`) like the other
declaration refusals of the selected document, names the operation's IR node
identity and the member path `TypeDefinition.operations[k].params[s].name`,
`k` the operation's index in its owner and `s` the parameter's, and retains the
operation's `origin` verbatim. Names that differ only in letter case are
distinct names.

A tuple value type (FR-208) is a closed type definition with owner-nested
identity `<package identity>/<tuple name>`. Its ordered `positions` list has at
least one entry; entry at zero-based index `i` has identity
`<tuple identity>/p<i>` and one `typeRef` naming an admitted value type. The
index is the position identity; the ordinal/type pair binds the position type
to that identity, so the ordered sequence of pairs forms the tuple shape.
Positions have no names. An empty list, a gap, an absent `typeRef` or
unsupported position type, or fields, identity fields,
operations, supertypes or clauses on the tuple refuses as
`invalid_model_binding`/`malformed-declaration` at the tuple or position node.
A position `typeRef` naming no package type refuses
`missing_declaration`/`missing-name`; a `typeRef` naming the tuple type resolves
to that tuple declaration. Two positions whose identities repeat refuse
`invalid_model_binding`/`conflicting-binding` at the later position.

A variant type is a type definition whose construct has meaning
`quire.meaning.model.variant-type/v1` and shape `enumeration`, with a
`variants` list of cases, each a node with an `identity`, a `name` and an
`origin`. A `payloadType` that is unresolved or not an admitted value type is
refused `unsupported_construct`/`declaration-form` at the payload member. The
variants refusals name these members:

- A construct whose shape is not `enumeration` names the type node and
  `TypeDefinition.kind`.
- A variant type with no case names the type node and `TypeDefinition.variants`.
- A case with no source span names the type node and `TypeDefinition.variants[i].origin`.
- A case's identity is the variant slot identity of its owner
  (`ix://<package identity>/variant/<Name>-<value>`) with `<Name>` the owner's
  artifact id verbatim and `<value>` the case name's case-preserving slug under
  filament-core-data FR-095. Thus artifact `Color` with cases `Red` and `Green`
  mints `variant/Color-Red` and `variant/Color-Green`, including in the published
  `domain-package-acme-shapes.json` fixture. Lowercasing either part changes the
  identity. A case whose identity is not its minted identity names the case node,
  as given, and `Variant.identity`.
- A `payloadType` that is unresolved or is not an admitted value type names the
  case node and `Variant.payloadType`, with
  `unsupported_construct`/`declaration-form`.
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
- The repeated-identity check is package-wide, not limited to one owner. Thus
  two cases of different variant types that mint the same slot identity are a
  collision: the later case node in package node order is refused once as
  `invalid_model_binding`/`conflicting-binding` at its owner's
  `TypeDefinition.variants[i]`, naming the shared identity. The collision is
  checked before either case's identity, origin or payload checks. This keeps
  FR-095's slot form unchanged while ensuring that two source declarations
  cannot alias one admitted node id.

A field `typeRef`, an operation parameter's `typeRef` and an operation result's
`typeRef` each take one of three forms, and no other form is read. A native
value type is named `ix://quire/native/<Name>`, where `<Name>` is exactly one
of `Boolean`, `Integer`, `Rational`, `Decimal`, `Float32`, `Float64`, `Text`,
`String`, `UUID`, `Timestamp`, `Bytes`, `Duration`, `JsonObject` and `Clock`,
compared byte for byte. These are the value-type keywords of the shared
grammar's `type-ref` production together with `String`, `UUID`, `Timestamp`,
`Bytes`, `Duration` and `JsonObject`, whose member types FR-322 states, and
`Clock`, the clock type of FR-415, which names no value type. A unit of
the checked package is named `ix://quire/unit/<name>`; intake admits it without
resolving it, and FR-322 "Model-owned members" resolves it (FR-208 "Field
types"). A type of the package is named `ix://<package identity>/<artifact
id>`, so a package type whose artifact id equals a native name
(`ix://test/orders/Integer`) is a different type from the native one and the
two never collide. A `typeRef` that begins `ix://quire/native/` and whose
`<Name>` is outside the set, and a `typeRef` of any other form, refuses
`invalid_model_binding`/`malformed-declaration`.

`ix://quire/native/Clock` is admitted as a field `typeRef`, an operation
parameter `typeRef`, an operation result `typeRef` and an alias `target`
(filament-core-data FR-146 "Clock members"). A field or parameter typed by an
alias whose `target` chain reaches `Clock` is a clock member as well. An alias
of `Clock` builds no type declaration. A **clock member** is a field, parameter
or result of the clock
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
map is a clock collection of its kind (FR-415 "Clock collections"). A map's
`keys` is never clock-bearing ("Map types" below). A `reference`
whose `target` is `Clock` or an alias of it reads as `Reference<Clock>`, a data
value that names one clock place (FR-415 "Clock places"), and builds no
declaration for the clock. A constraint whose `appliesTo` is `Clock` or an alias
of it is a clock constraint (FR-415 "Clock places"): intake admits the keywords
`min` and `max` (the checker refuses a `min` as `time-invariant-shape`) and
refuses any other keyword `invalid_model_binding`/`malformed-declaration` at
`Constraint.keyword`. A constraint whose `appliesTo` is `ix://quire/unit/<name>`
or an alias of it with the keyword `min` or `max` is a unit bound ("Unit
bounds" below), and any other keyword refuses at `Constraint.keyword`; a `reference` whose
`target` is a unit refuses at `TypeDefinition.target`, because a quantity has no
identity for a reference to name. A variant case whose `payloadType` is
`ix://quire/native/Clock` is refused `unsupported_construct`/`declaration-form`
at `Variant.payloadType` (AC-25). This differs from a tuple position typed
`Clock`, which FR-322 refuses as `invalid_model_binding`/`malformed-declaration`.
Intake does not check which construct owns an admitted clock member or whether
the model declares a time source; QSL FR-230 and FR-415 check those where the
clock is used.

**Option value types.** A `typeRef` is one of the three forms above and never
carries a type argument, so `Option<T>` is not spelled in a `typeRef`:
`ix://quire/native/Option` and every `typeRef` that embeds a second reference are
outside the set and refuse `invalid_model_binding`/`malformed-declaration`. An
**option value type** is a type definition node of `kind` `option` (beside
`alias`, `sequence`, `map` and `reference`) with exactly one member `inner`, a
string `typeRef` of the three forms. It denotes `Option<T>`, `T` the type `inner`
names: an absent value, or one present `T` value (FR-322 "Model-owned members"). It
is one type definition node, builds one original declaration keyed by its package
identity and IR node identity, and is named by `ix://<package identity>/<artifact
id>` like any package type, so it is admitted at every position that takes a package
type: a field, parameter or result `typeRef`, an alias `target`, a sequence `items`,
a map `values`, a tuple position `typeRef` and a variant case `payloadType`. `inner`
may name a native value type other than `Clock`, a unit (`Option<Quantity<u>>`), or
any package value type, including a record, tuple, variant, sequence, map or
reference type and an alias of those; a unit bound on `inner` bounds each present
value. `inner` never names an option value type, directly or through an alias
chain: `Option<Option<T>>` has no spelling, and intake does not flatten one. A
clock place is optional by presence alone (FR-415 "Clock places"), so `inner`
never names `Clock` or an alias of it either.

*Presence and option.* Presence `optional` on a field is the member-level
spelling and an option value type is the type-level spelling of one type. A field
of option type `O` over `T` with presence `required` and multiplicity `[1, 1]` and a field of `T` with presence `optional` and the same multiplicity
have the same type `Option<T>`, the same `composite_type`/`option` node, the same
equality and the same values; neither adds a meaning the other lacks, and a
reader derives no difference between them. They differ in reach only: presence
exists only on a field, so an optional element, map value, tuple position, variant
payload, alias target or parameter is expressed by an option value type and by no
other means. Wrapping is single: presence `optional` on a field of option type
with multiplicity `[1, 1]` is the refused `Option<Option<T>>`, while
a collection multiplicity wraps the option type (`Sequence<Option<T>>[0, 3]`) and
presence `optional` on a collection field wraps the collection
(`Option<Sequence<T>[0, 3]>`), both admitted. Failures of one option type report
in the order shape members, `constraints`, `inner` form, `inner` resolution, `inner`
kind (nesting, `Clock`, non-value type), and a field's presence refusal reports
after the field's `typeRef` refusals.

**Map types.** A type definition of `kind` `map` declares exactly three type
members, each required: `keys`, the key type, and `values`, the value type, each
a string `typeRef` of the three forms; and `bound`, the entry-count bound, an
object `{lower, upper}` of non-negative integers whose `upper` is absent when
the map has no maximum, read as a multiplicity's `lower` and `upper` are. A
map with no maximum has `lower` `0`: FR-322 gives no type to an unbounded upper
with a lower above `0`, so that `bound` is refused rather than typed without
its lower bound. It
denotes `Map<K, V>[lower, upper]` (FR-322 "Model-owned members"), `K` the type
`keys` names and `V` the type `values` names: a finite function from keys to
values that holds at least `lower` and at most `upper` entries, one for each key
it holds. Intake reads no default key type and no default bound: a map type
without `keys` or without `bound` is refused at the absent member, and no
`Text` key or unbounded map is assumed for it.

The key type SHALL be a **scalar key type**, a scalar type with a total
canonical key (FR-144 "Element admission and canonical keys"; FR-322 `Map<K,V>`,
`K` a scalar key type): the native value types `Boolean`, `Integer`,
`Float32`, `Float64`, `Text`, `String`, `UUID`, `Timestamp`, `Bytes`,
`Duration` and `JsonObject`, a unit (`Quantity<u>`), a value type of the package
bound as `Int[lo, hi]`, as `Text[min, max; profile]` or as a bounded
`Rational[n1, n2; d1, d2]` with or without a `unit`, a variant type none of
whose cases carries a `payloadType` (an enumeration), or an alias whose
`target` chain reaches one of those. A native `Text` key (no bounds and no
profile) is typed by the `scalar_type`/`text` node and keyed by its exact
Unicode scalar sequence with no normalization, as a `String` key is (FR-144
"Element admission and canonical keys"), and is its key type only: a member typed bare `Text` still has no
type (FR-322 step 4). Intake SHALL refuse every other key type
`ill_typed`/`operator-ineligible` at `TypeDefinition.keys`, naming the map type
node, after the `keys` reference-form and resolution rows: a clock-bearing key
(`Clock`, an alias of it, or a record value type with a clock field such as
`Timer { x: Clock; n: Int }`, FR-415 "Clock places"); a record value type,
tuple value type or variant type with a payload-bearing case (a union) that holds
no clock (`record Cfg { a: Boolean }`); an option value type (`Option<Text>`);
a sequence or map type; a `reference` type or an object type
(`Reference<T>`); the natives `Rational` and `Decimal`, a value type bound as a
`Decimal[...]` domain and a value type no FR-322 row binds (a key type with no
type). A key is an immutable lookup value with a canonical order,
and a clock advances with time. `values` names any type a sequence `items`
names, `Clock` and clock-bearing types included, which makes the map a clock
collection (FR-415 "Clock collections"). A map type's failures report in the
order extra members, `keys` presence, `keys` form, `keys` resolution, `keys`
eligibility, `values` presence, `values` form, `values` resolution, `bound`
presence and shape, then `bound` order.

`ix://quire/unit/<name>` is admitted at every position that takes a type:
a field, parameter or result `typeRef`, an alias `target`, a sequence `items`,
a map `keys` and `values` and a variant case `payloadType` (a payload follows the same
row as any other payload). A member of a unit type has the unit member shape:
`defaultKind` `none`, an empty or absent `constraints`, and no `unit`,
`decimal`, `textProfile` or `defaultValue`; its multiplicity, presence and
nullability are any other member's. A unit as a constraint `appliesTo`
with the keyword `min` or `max` is a unit bound (below); any other keyword refuses
`invalid_model_binding`/`malformed-declaration` at `Constraint.keyword`. A unit
as a `reference` `target` refuses
`invalid_model_binding`/`malformed-declaration` at `TypeDefinition.target`: a
reference names an object and a quantity has no identity.

**Unit bounds.** A unit bound is a value invariant that also forms a bounded
quantity type (below). Its
`keyword` is `min` or `max` and its `operands.value` is a canonical rational
string `c` (the form of a unit's `scale`), a magnitude in the unit `u` that
`appliesTo` names directly or through an alias; an `operands.value` that is not a
canonical rational string refuses `invalid_model_binding`/`malformed-declaration`
at `Constraint.operands`, and a `min` greater than a `max` with the same
`appliesTo` refuses the same at the later constraint's `Constraint.operands`.
The bound constrains every member of the selected package that is typed by `u`
(or, when `appliesTo` is an alias, typed by that alias or an alias of it): a
field, an operation parameter or result, and each element of a sequence, each
value of a map and each present value of an optional member or of an option value type of that type. The
magnitude of such a value is an exact rational in `u`, and the value satisfies
`min c` exactly when its magnitude is at least `c` and `max c` exactly when it is
at most `c`, so a magnitude equal to `c` satisfies both; no unit conversion is
made, and an absent optional member satisfies every bound. A member under
several bounds satisfies their conjunction, and a conjunction no magnitude
satisfies is an unsatisfiable invariant reported as any other is.

A bound is checked as an invariant over the values of the member, with the
constraint's `identity` as the invariant's name. A value supplied to
execution, namely an initial state or population, an operation argument, an
environment choice or a replayed recorded value, that violates a bound refuses
`invalid_runtime_input`/`invalid-value` naming the member path, the constraint
`identity`, its keyword and `c`. A successor whose post-state value violates a
bound is a successor that violates an invariant (FR-181): it is recorded rather
than pruned, and the invariant it violates is the constraint. A value that is
not statically known is checked only as the invariant above.

The type of a bounded member is the bounded quantity type `Quantity<u>[min, max]`,
a refinement of `Quantity<u>` (FR-142) whose values are the quantities of `u`
whose magnitude satisfies every bound that applies to the member (an absent
bound is open). It is a subtype of `Quantity<u>`: a bounded value, and a bounded value assigned or passed to a position of the
same or a wider bounded type (equal or looser bounds), is read as a
`Quantity<u>` by conversion, comparison, equality and arithmetic, and the result
of arithmetic is a plain `Quantity<u>`; a member of type `Quantity<u>` supplied
to a bounded position is admitted by the checker and left to the invariant above.
A quantity literal, or an exact constant expression that folds to one, whose
expected type is a bounded quantity type refuses `ill_typed`/`type-mismatch` at
the literal when its magnitude violates a bound (`11 ms` against `max` `10`),
naming the member path or alias, the constraint `identity`, its keyword, `c`,
the refusal site and the expected type `Quantity<u>[min, max]`, and checks when it satisfies
every bound (`5 ms`, and `10 ms` at `max` `10`, equality satisfying); this holds
at every position, a postcondition assignment included. The constant is the
whole exact constant expression after folding, and the refusal is at its outermost
expression: `-1 ms` is unary minus (FR-142 "Quantity negation") applied to the
literal `1 ms`, so against `min` `0` it refuses at the negation expression `-1 ms`,
not at `1 ms`, and `2 ms - 3 ms` refuses at the subtraction. A sequence, set, map
or optional member with a bounded element or value has the type
`Sequence<Quantity<u>[min, max]>`, `Set<...>`, `Map<K, Quantity<u>[min, max]>` or
`Option<Quantity<u>[min, max]>`, and a literal element, map value or optional
payload written in a collection or `Some` constructor is checked statically as
above, at that element. A literal of another
unit refuses as FR-142 refuses it, with no conversion. A conjunction of bounds no
magnitude satisfies makes every literal of that position refuse. Two bounded
types are the same type exactly when their unit and effective bounds are equal;
a bound declared on an alias gives the type to members of that alias and of its
aliases only, never to another member typed `u`, and a bound on `u` itself types
every member of `u`. The runtime and post-state invariant above stays in force
for every value the checker cannot decide.

The bounded type is not a fact for FR-146: a quotient of quantities
keeps the exact rational magnitude over the whole rational domain and carries no
range obligation, and no guard fact is derived from a bound. A bound holds as an
invariant of the states and supplied values it is checked on, and a function body
is checked for every quantity argument of the unit, so a bound proves nothing
about an argument and a divisor still has no nonzero proof form from it. The
checked package emits a `bounded_quantity` node when a type position must
preserve this refinement for typed operation admission. The node carries the
exact unit and inclusive magnitude bounds as a wire form for the same
`Quantity<u>[min,max]` refinement; it does not replace this requirement's value
invariant or runtime checks. A field's invariant remains checked by (declaring
node, member path) from the admitted document, as a clock constraint is read;
the node is emitted only where a checked-package type position must retain the
bound. The filament-core-data FR-146 mapping of a unit bound to
`x-agent-ix-quantity-min` and `x-agent-ix-quantity-max` has the same meaning,
equality satisfying.

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

`Bytes`, `Duration` and `JsonObject` are the kernel scalars of filament-core-data
FR-032 that a domain package may name. `ix://quire/native/Bytes` names an
octet sequence of any length, `ix://quire/native/Duration` names an amount of
elapsed time in seconds, and `ix://quire/native/JsonObject` names a JSON value of
any kind (filament-core-data FR-032 gives it the unconstrained scalar `any`); FR-322 "Model-owned members" states each member type, its canonical
spelling and its operations, and FR-149 states its equality. Intake admits each in
every position a native name other than `Clock` is admitted in: a field
`typeRef`, an operation parameter or result `typeRef`, an alias or reference
`target`, a sequence `items`, a map `values` and a variant case `payloadType`. It
builds no declaration for one, and a field typed by one carries any
multiplicity, presence, `nullable`, `defaultKind` and `constraints` a field of
another native type carries. Intake does not read a `Duration` as a quantity
or look for the dimension `Time` and the unit `s`: FR-322 resolves a `Duration`
member where the member is used.

Intake admits a `typeRef` naming `Rational`, `Decimal` or `Text` as it admits
any other native name: the name is a type that exists, and a `typeRef` carries
no parameters. The bounds of a `Text`, `Integer` range, `Rational` range and
the like come from a value type of the package that the field names (FR-322
"Model-owned members", and "Rational value types" below), not from the
reference. Whether a member typed by an unparameterized
`Rational`, `Decimal` or `Text` can be used is decided where it is used, at
member resolution: a bare `Text` field refuses there
`ill_typed`/`operator-ineligible` (FR-322-AC-31), and intake raises nothing.
A field typed by a native value type declares no type node of its own. A
refusal of a `typeRef` names the field node and `Field.typeRef`, the operation
node and `Operation.params[i].typeRef`, or the operation node and
`Operation.returns.typeRef`.

**Rational value types.** A value type (FR-208
`quire.meaning.model.value-type/v1`) whose `scalar` is `rational` is a
**Rational value type**. Intake SHALL read its bounds from its own
`constraints` only, by the four keywords `minNumerator`, `maxNumerator`,
`minDenominator` and `maxDenominator`, each given once in any order, each with
`appliesTo` the value type itself. Its `operands.value` SHALL be either a JSON
integer of magnitude at most 2^53 (9007199254740992), or a canonical signed-64
decimal integer string: `0`, a positive digit sequence with no leading zero,
or `-` followed by a nonzero digit and further digits, in each string case
within `i64::MIN..=i64::MAX`. Thus `"-0"`, `"01"`, `"+1"`, and any string
outside that signed range are not canonical bound strings. The integer-number
limit here is the exact-integer limit of FR-272's canonical-number rule, not
FR-322's narrower node-ordinal limit. The package's RFC 8785 number check runs
before declaration intake: any integer number of magnitude above 2^53 is
refused as `noncanonical_wire` with cause `inexact-integer` at its
`document_pointer`, as FR-272 specifies. Thus no integer number above 2^53
reaches rule 5. This requirement owns the Rational bound's accepted value
encoding and range within the producer-defined `Constraint.operands.value`
member; it adds no IR member and does not change the producer-side member
schema. With numerator bounds `n1`, `n2` and denominator bounds `d1`, `d2` it
denotes `Rational[n1, n2; d1, d2]`: the rationals that, in lowest terms with a
positive denominator, have a numerator in `[n1, n2]` and a denominator in
`[d1, d2]` (FR-149's `Rational[..]`). A Rational value type with no
`constraints` is the unbounded `Rational`: intake admits it, and a member of it
has no type, as a member of the native `ix://quire/native/Rational` has none
(FR-322-AC-183). Otherwise intake SHALL refuse the first defect of the value
type, at the value type node, taking the rows below in order. Rows 1 to 3 scan
the value type's `constraints` in array order, and rows 4 and 5 take the
keywords in the order `minNumerator`, `maxNumerator`, `minDenominator`,
`maxDenominator`, so exactly one refusal is reported:

1. a constraint whose keyword is none of the four:
   `unsupported_construct`/`declaration-form` at `Constraint.keyword`, naming
   the keyword;
2. a constraint whose `appliesTo` is not the value type's own identity (another
   node, an alias of the value type, or a native or unit reference):
   `invalid_model_binding`/`malformed-declaration` at `Constraint.appliesTo`;
3. a keyword given a second time, the later constraint in `constraints` order:
   `invalid_model_binding`/`malformed-declaration` at `Constraint.keyword`,
   naming the keyword;
4. a missing keyword:
   `invalid_model_binding`/`malformed-declaration` at
   `TypeDefinition.constraints`, naming the keyword;
5. an `operands` that is not an object whose only member is `value`, or whose
   `value` is absent or is neither a JSON integer in `[-2^53, 2^53]` nor a
   canonical signed-64 decimal integer string in `i64::MIN..=i64::MAX` (a
   fraction such as `1.5` or `0.5`, an exponent spelling such as `1e2`, a
   string such as `"3/4"`, `"01"`, `"+1"` or `"-0"`, or a string such as
   `"9223372036854775808"`):
   `invalid_model_binding`/`malformed-declaration` at that constraint's
   `Constraint.operands`, naming the keyword; no operand is rounded, widened or
   repaired. Integer numbers of magnitude above 2^53 were refused earlier by
   the canonical-number check and do not reach this rule;
6. bounds outside `n1 <= n2` and `1 <= d1 <= d2`:
   `unsupported_construct`/`declaration-form` at `TypeDefinition.constraints`,
   naming the four bounds.

A denominator bound therefore lies in `1 ..= i64::MAX`: a zero or negative
`minDenominator` or `maxDenominator` is a JSON integer in range and is refused
by row 6, not row 5. A defect of the `unit` member (below) reports after these
rows, and only when none of them fails.

A constraint that is not in a Rational value type's own `constraints` and whose
`appliesTo` names a Rational value type, or an alias whose `target` chain
reaches one, SHALL be refused `invalid_model_binding`/`malformed-declaration`
at the node that carries it and its `Constraint.appliesTo`, whatever its
keyword: the bounds are the value type's alone. This refusal is the carrying
node's and does not change the value type's own report.

A Rational value type MAY carry a `unit` member, a unit reference of the form
`ix://quire/unit/<name>` that a unit `typeRef` takes. Intake admits it without
resolving it, as it admits a unit `typeRef`; FR-322 "Model-owned members"
resolves it, and one that names no unit of the checked package refuses
`missing_declaration`/`missing-name` at the value type node and
`TypeDefinition.unit`. An absent `unit` makes the type dimensionless. A `unit`
member that is not a string of that form (a bare name such as `m`, a number, or
a native reference such as `ix://quire/native/Rational`), and a `unit` member
on a value type whose `scalar` is not `rational`, refuse
`invalid_model_binding`/`malformed-declaration` at `TypeDefinition.unit`. A
bounded Rational value type with a `unit` `u` has the carried quantity type
`Quantity<u, Rational[n1, n2; d1, d2]>` (FR-142 "Carried quantity"); two
Rational value types with equal bounds and different units are two types, and
two with equal bounds and equal units, or equal bounds and no unit, are one
type (FR-322 "Model-owned members" step 4). A member of a unit-bearing
Rational value type is a carried quantity, so it is not an operand of `=`,
`!=`, `<` or any other quantity operation (FR-142 "Carried quantity"): two such
members are compared, and combined, through the exact conversion
`convert<Quantity<u>>(e)` of each. Members of its carried type are summed by
`sum<N>` whose `N` is that carried type (FR-142 "Carried quantity sum").

A Rational value type is named by `ix://<package identity>/<artifact id>` and
is admitted at every position that takes a package type: a field, parameter
or result `typeRef`, an alias `target`, a sequence `items`, a map `values`, an
option `inner`, a tuple position `typeRef` and a variant case `payloadType`.
Every member typed by one Rational value type, directly or through an alias
chain, has exactly its bounds and unit; no member restates or narrows them. A
field or parameter typed by a Rational value type, directly or through an
alias, that carries its own non-empty `constraints` refuses
`invalid_model_binding`/`malformed-declaration` at `Field.constraints` (a
parameter's path is `Operation.params[i].constraints`), and an alias whose
`target` chain reaches a Rational value type and that carries a non-empty
`constraints` refuses the same at the alias node and
`TypeDefinition.constraints`. An alias carrying no constraint admits and
denotes the type of its target. Each refusal of this paragraph keeps the IR
node identity and `origin` and reports in node order as every other intake
refusal does; adding a Rational value type to a package changes no other
node's declaration key or order.

If a value supplied to execution at a member of a bounded Rational value type
(an initial state or population, an operation argument, an environment choice
or a replayed recorded value) is not a rational in lowest terms with a positive
denominator whose numerator and denominator lie in the type's bounds, then
execution SHALL refuse it `invalid_runtime_input`/`invalid-value` naming the
member path, and SHALL NOT reduce, round or convert it. Thus `2/4`, `8/1` and
`1/13` against `Rational[-3, 7; 1, 12]`, and the float `0.5`, are refused. A
value at a member of a unit-bearing Rational value type is its magnitude in
`u` and is decided by the same rule.

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
| `INVALID_CLOCK_MEMBER` | `invalid_model_binding`/`malformed-declaration` | A clock member shape member only: `Field.*`, `Operation.params[i].*` or `Operation.returns.*`. This mapping does not cover a variant payload, whose refusal is governed by FR-154-AC-25. |
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
spec-objects module (spec-objects-architecture, spec-objects-business, …). Its
artifact id is an object id: an ASCII letter followed by zero or more ASCII
letters, digits, underscores or hyphens (`^[A-Za-z][A-Za-z0-9_-]*$`). The
identity carries that id verbatim. A node whose id does not match this rule is
refused `invalid_model_binding`/`malformed-declaration`, naming the IR node
identity and its `origin`. Object ids and member names are distinct. This
intake identity rule does not define the source-language spelling by which a
clause names a type; that spelling is governed by the grammar and resolution
requirements for that clause.

Artifact-type ids such as `FR-100` still declare no model node because the
artifact's type is not a model-declaring object type. Extraction selects a
source document by its document `type`; a document whose type is `FR` is
skipped even if it has `object: value_object`. After extraction and lift,
intake examines only each emitted IR node: for a `TypeDefinition` or
population node it reads the final identity segment as the artifact id,
applies the object-id rule above, and resolves the node's `{module, name}`
`kind` through the package `constructs` table to obtain its `meaning` (FR-208).
It does not consult the source document's `type`, `object` or `id` to classify
that emitted node. Therefore a model-declaring source document whose emitted
node has identity `ix://<package identity>/FR-204` is admitted when that node's
id, construct and meaning satisfy the same checks as any other declaration;
this does not cause a source document with `type: FR` to emit a node.

A composed document is a document whose `type` is neither an object type a
loaded module declares nor equal to its own `object` (an artifact type such as
`FR`, `NFR` or `TC`) and that also carries an `object:` member, such as a
requirement `FR-204` with `object: value_object`. The rule sits on extraction
(step 2) and the lift (step 3), which produce the domain package intake reads:
the document selection is the filament-core-data extraction step
(filament-core-data FR-091), which skips a document whose `type` is an
artifact type such as `FR`, that is, neither an object type a loaded module
declares nor equal to the document's own `object`, even if it carries
`object:`. It classifies a document by its `type` alone, never by `object:`.
The lift therefore emits no IR node for a composed document, so the package
has no declaration key and no `<Name>` segment for it. Its hyphenated artifact
id is immaterial: `object:` is descriptive classification for the reader; no
model name is derived from it, from the document's `id` or from any other
member, and there is no explicit-name member. Intake reads package bytes only
and does not classify source documents. A reference to a composed document as
a model declaration is a reference to a node no package carries: a
relationship end, supertype, field type or other IR reference naming
`ix://<package identity>/FR-204` refuses `missing_declaration`/`missing-name`.
A document link to the composed document
(`ix://agent-ix/<repo>/FR-204` in frontmatter relationships) is a document
link and never a model reference. A package that carries a well-formed model
node whose `<Name>` segment resembles an artifact-type id is admitted under
the same object-id and meaning rules as any other node; intake has no
source-document classification with which to infer otherwise.

A declaration's IR node identity is the identity filament-core-data FR-095
mints (the closed slot list of its "Node identities" section), except for the
four kinds below, rooted at
`ix://<package identity>/`, with `<Name>` the declaring artifact's id passed
through verbatim, every owner-nested member name passed through verbatim, and
every other part slugged. A type's identity is
`ix://<package identity>/<artifact id>`, and the alias of a constrained
field is `ix://<package identity>/<Name><Field>` (`<Field>` slugged with its
first character upper-cased), which names no slot segment of its own. This
concatenated alias form is also not injective in general (for example, owner
`a` with field `bC` and owner `aB` with field `C` both produce `aBC`); that
pre-existing alias identity risk is unchanged by this artifact-id amendment.
A field, an operation and an operation
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

A declaration key carries no digest and no digest domain. Two domain package
documents with equal nodes yield equal declaration keys.

Intake charges `normalize.record` under `ModelNormalizationLimitsV1` of
`quire.value.accounting/v1` once per declaration, ascending by declaration key.
Admission checks are decided before the first charge.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-154-AC-1 | An artifact bundle whose extraction and lift yield a valid domain package is admitted, and intake builds exactly one original declaration per IR node, keyed by package identity and IR node identity. | Test (TC-234) |
| FR-154-AC-2 | A missing package and a package whose identity differs each refuse with its named cause before any declaration is built, a package supplied under the selection's identity and carrying it is admitted, and the admitted document's identity is recorded by the package's `model_root` node. | Test (TC-234) |
| FR-154-AC-3 | A malformed IR node or a dangling reference refuses with the IR node identity and its `origin`, and every such refusal is reported in node order. | Test (TC-234) |
| FR-154-AC-4 | Two domain package documents with equal nodes and different `version` members yield equal declaration keys, effective declarations, effective view and object universe; the `version` member is not read. | Test (TC-234) |
| FR-154-AC-5 | Intake at its exact `normalize.record` bound completes, and the one-less run is incomplete at `normalize.record`. | Test (TC-234) |
| FR-154-AC-6 | Retitling an artifact leaves every declaration key unchanged. Changing an artifact's id changes the declaration key of the type it declares and of that type's members, leaves every other declaration key unchanged, and a reference to the old key refuses `missing_declaration`/`missing-name`. | Test (TC-234) |
| FR-154-AC-7 | A type definition or population node whose kind names no construct entry, or whose construct has no Quire meaning id or an id outside FR-208, refuses `invalid_model_binding`/`malformed-declaration` with the IR node identity and its `origin`, and no meaning is inferred from its kind name. | Test (TC-234) |
| FR-154-AC-8 | A type definition or population node whose artifact id begins with an ASCII letter and continues with ASCII letters, digits, underscores or hyphens is admitted with that id in its node identity and declaration key; this includes `sys_pump`, `sys-pump` and `FR-001`. An artifact id that does not match this rule, such as `-sys_pump`, refuses `invalid_model_binding`/`malformed-declaration` with the IR node identity and its `origin`, a reference to that node reports no `missing_declaration`/`missing-name`, and a second failing check on the same node reports after the id refusal. An artifact-type document such as `FR-204` with `object: value_object` still yields no node because extraction classifies by `type`; a model-declaring object artifact with id `FR-204` does yield a node and is admitted. | Test (TC-234) |
| FR-154-AC-9 | The published domain package document of `acme/orders` (`domain-package-acme-orders.json`) carries exactly FR-095's identity forms: types `ix://acme/orders/<artifact id>`, fields, operations and parameters nested under their owner, and relationships `ix://acme/orders/relationship/<Name>-<verb>-<TargetName>` (`Order-order-Order`, `Order-invoiced-Invoice`, `Order-follows-Order`, `Order-twin-Order`), the constrained-field alias `ix://acme/orders/OrderTotal`, none of them owner-nested. | Test (TC-280) |
| FR-154-AC-10 | A relationship declaration of the selected document refuses at the owner of the package's `model_root` node (`/semantic_graph/nodes/<i>/owner`), in node order and, within a node, in the order malformed declaration, unresolved end type, duplicate identity, reversed multiplicity: an identity outside the package's global `relationship` slot (owner-nested, foreign-package or wrong-slot), a missing or invalid `direction`, `category`, `composite` or `origin`, a missing, null, empty or non-string `sourceEnd.role`, a present null, empty or non-string `targetEnd.role` (an absent `targetEnd.role` is valid), a source end naming another owning type, a malformed end or multiplicity, and an end type naming a relationship or another declaration of the wrong meaning each refuse `invalid_model_binding`/`malformed-declaration`; an end type naming no declaration refuses `missing_declaration`/`missing-name`; a repeated relationship identity, including under two owners, refuses `invalid_model_binding`/`conflicting-binding`, and two missing identities are malformed declarations, not conflicting ones; `lower > upper` alone refuses `invalid_model_binding`/`unpreserved-model-meaning`. The published `adverse.json` `relationship_document_cases` hold one entry per refusal and three admitted entries. Each refusal retains the relationship's IR identity and its `origin` verbatim: `declaration_origin_cases` `refusal-retains-source-origin` (a source origin with its `sourceIdentity`, `path` and span) `refusal-retains-generated-origin` (a generated origin with its members and no span) and `refusal-retains-no-malformed-origin` (a malformed origin, refused `malformed-declaration`, retained as none). | Test (TC-280) |
| FR-154-AC-11 | A variant type with no case, a case with no source span, a case whose identity is not `variant/<Name>-<value>` of its owner and name, and a construct of a shape other than `enumeration` each refuse `invalid_model_binding`/`malformed-declaration` (except a case already refused by a same-identity collision, FR-154-AC-13), at the node and member path FR-154 names (`TypeDefinition.kind`, `TypeDefinition.variants`, `TypeDefinition.variants[i].origin`, `Variant.identity`); a field `typeRef`, an operation parameter and an operation result naming a variant type are admitted and raise no refusal; a case with a supported, resolving `payloadType` is admitted as that union member's single payload, and an unresolved or unsupported `payloadType` refuses `unsupported_construct`/`declaration-form` at its case node and `Variant.payloadType`, never `invalid_model_binding`; a type whose cases all omit `payloadType` remains an enumeration of nullary cases, while a case without `payloadType` in a union is a nullary member; two cases with equal names and distinct identities refuse the later case `invalid_model_binding`/`conflicting-binding` at the type node and `TypeDefinition.variants[i]` and raise no other refusal for it; within the type node the failures report in the order shape, empty list, repeated names, source spans, and each case node then reports identity and payload after the type node's refusals; reordering the `variants` list changes no declaration key for an enumeration or a union. | Test (TC-235, TC-939) |
| FR-154-AC-12 | A clause, state, transition and step identity is exactly `<owner identity>/<member name>`: `ix://test/orders/OrderLifecycle/NotYetPlaced`, `.../OrderLifecycle/Draft`, `.../OrderLifecycle/<transition member name>` and `.../Fulfilment/receive_order` are admitted, a clause, state or step whose `clauseId` or `name` differs from the member name of its identity refuses `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.clauses[i].clauseId`, `TypeDefinition.states[i].name` and `TypeDefinition.steps[i].identity`, a state of `OrderLifecycle` with the identity `.../Fulfilment/Draft` (the wrong owner) refuses `malformed-declaration` at `TypeDefinition.states[i].identity`, a state, clause, step or transition whose member name is also a field or operation of its owner (state `cancel` beside operation `cancel`) refuses `invalid_model_binding`/`conflicting-binding`, and the global-slot spellings `clause/<Name>-<clauseId>`, `state/<Name>-<state>`, `transition/<Name>-<from>-<to>-<trigger>` and `step/<Name>-<step>` each refuse `invalid_model_binding`/`malformed-declaration` at the member's `identity`. Two transitions of one machine whose slugged global spellings would be equal (`a_b → c` and `a → b_c` on one trigger) are two distinct nodes with distinct keys. | Test (TC-235) |
| FR-154-AC-13 | Two fields of one owner, two operations of one owner, a field and an operation of one owner, two parameters of one operation, two tuple positions with one ordinal identity, or two variant cases that share one identity each refuse once, `invalid_model_binding`/`conflicting-binding`, naming the shared identity at the later member's path: `TypeDefinition.fields[j]`, `TypeDefinition.operations[k]` (the operation, for a field and an operation), `TypeDefinition.operations[k].params[s]`, the tuple position path or the later case node's owner's `TypeDefinition.variants[i]`, including when the variant cases have different owners. The refusal carries the code, cause, shared identity and path only, and the collision suppresses the identity, origin and payload checks of both colliding cases (FR-154-AC-11 and FR-208-AC-15 apply to a case not already refused by a collision), so exactly one refusal is reported whichever of the two cases carries the correctly minted identity, and no declaration of the package is admitted. The same members with distinct identities are admitted, and one member name used in two different owners is two distinct identities and no collision. | Test (TC-234, TC-235) |
| FR-154-AC-14 | An operation parameter whose `name` is absent, not a string, null, empty, begins with a digit, contains a separator or differs from the last segment of its identity, and which is not one of two parameters sharing an identity, refuses `invalid_model_binding`/`malformed-declaration` at the selection row, naming the operation's identity and `TypeDefinition.operations[k].params[s].name`; two parameters of one operation with one identity refuse once, `invalid_model_binding`/`conflicting-binding`, naming the shared parameter identity at the later parameter's `TypeDefinition.operations[k].params[s]`, and that collision suppresses the name check of both, so a malformed name on either colliding parameter is not reported; distinct valid names, including names differing only in letter case, are admitted and no relationship declaration outcome changes. | Test (TC-280) |
| FR-154-AC-16 | An operation whose `Operation.redefines` names a supertype's operation (`Sub/size` naming `Order/size`) is admitted with that redefinition. The same operation with its `redefines` member removed redefines nothing, and so does one whose redefinition is listed only in an extension payload of its owning type, `required` true or false when the consumer admits the extension; a consumer that does not admit a `required` extension refuses the document before intake, `unknown_required_feature`/`unsupported-feature` at the owning type node and its `extensions` member. An `Operation.redefines` naming no node of the package refuses `missing_declaration`/`missing-name` at the operation node and `Operation.redefines`. | Test (TC-280) |
| FR-154-AC-17 | A field typed `ix://quire/native/Integer`, a field typed `ix://quire/native/Timestamp`, a field typed `ix://quire/native/UUID`, a field typed `ix://quire/native/Text`, a field typed `ix://quire/native/Rational`, a field typed `ix://quire/native/Decimal` and a field typed `ix://quire/unit/ms` are admitted, each native resolving to that native value type and the unit one left unresolved by intake, with no declaration built for a native reference and no declaration key naming `quire/native`; a package type with artifact id `Integer` is admitted beside them and a field typed `ix://test/orders/Integer` resolves to that package type, not to the native `Integer`. A field typed `ix://quire/native/Int`, `ix://quire/native/integer`, `ix://quire/native/Any` or `ix://quire/unit/`, or typed `Integer` with no URI, refuses `invalid_model_binding`/`malformed-declaration` at the field node and `Field.typeRef`; an operation parameter and an operation result typed `ix://quire/native/Int` refuse the same at the operation node and `Operation.params[i].typeRef` and `Operation.returns.typeRef`, while the same parameter and result typed `ix://quire/native/Text` are admitted; a selection naming the identity `quire/native` and a selection naming the identity `quire/unit` each refuse `invalid_model_binding`/`malformed-declaration` at the selection row before its digest domain or bytes are read, and build no declaration. | Test (TC-234) |
| FR-154-AC-18 | A field typed `ix://quire/native/Clock` with multiplicity `{1, 1, false, false}`, presence `required`, `nullable` `false` and `defaultKind` `none` is admitted with no type declaration built for it and no declaration key naming `quire/native`. A clock field with multiplicity `{0, 1, false, false}`, presence `optional`, `nullable` `true`, `defaultKind` `semantic` with a `defaultValue`, `unit` `ms`, a `decimal` policy, a `textProfile` and a `min` constraint refuses `invalid_model_binding`/`malformed-declaration` once at each of `Field.constraints`, `Field.decimal`, `Field.defaultKind`, `Field.defaultValue`, `Field.nullable`, `Field.textProfile` and `Field.unit`, in that member-path order, and raises no other refusal at those members (presence `optional` and the multiplicity are admitted); a reference whose target is `ix://quire/native/Clock` reads as `Reference<Clock>`, a place reference, and builds no declaration; a `max` constraint whose `appliesTo` is Clock or an alias of it is a clock constraint, a `min` one is admitted by intake (the checker refuses it `time-invariant-shape`), and a keyword other than `min` and `max` refuses `invalid_model_binding`/`malformed-declaration` at `Constraint.keyword`; a unit as a reference target refuses `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.target`, and a unit as `appliesTo` with `min` or `max` is admitted; `ix://quire/native/clock` refuses at `Field.typeRef`. | Test (TC-234) |
| FR-154-AC-19 | A package carrying base dimension `Time` (`ix://test/orders/dimension/Time`), derived dimension `Rate` (terms `[{Time, "-1"}]`), root unit `s` and unit `ms` (`target` `s`, `scale` `1/1000`) is admitted with one original declaration per dimension and unit node, keyed `(test/orders, ix://test/orders/dimension/Time)`, `(test/orders, ix://test/orders/dimension/Rate)`, `(test/orders, ix://test/orders/unit/s)` and `(test/orders, ix://test/orders/unit/ms)`, each charged `normalize.record`. Each of these single defects refuses exactly one `invalid_model_binding`/`malformed-declaration`: a unit with `scale` `0` at `Unit.scale`, the derived `Rate` marked `base` `true` (a base dimension with a term) at `Dimension.terms`, a unit whose identity is `ix://test/orders/unit/msec` for name `ms` at `Unit.identity`, a unit named `m-s` at `Unit.name`, a dimension whose `base` is the string `"true"` at `Dimension.base`, a unit with no `origin` at `Unit.origin`, and a two-unit target cycle at the least-index unit's `Unit.target`; a unit whose `dimension` names no dimension node and a unit whose `target` names no unit node refuse `missing_declaration`/`missing-name` at `Unit.dimension` and `Unit.target`; two units named `ms` with one identity refuse once, `invalid_model_binding`/`conflicting-binding`, at the later unit's `Unit.identity`; a dimension `Time` with no unit is admitted. | Test (TC-234) |
| FR-154-AC-20 | A field `created_at` of an owner `O` with identity `ix://test/orders/O/created_at`, an operation `reset_all` with identity `.../O/reset_all` and its parameter `created_at` with identity `.../O/reset_all/created_at` are admitted, each keyed by that verbatim identity; the same field with identity `.../O/created-at` refuses `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.fields[j].name`, the same operation with identity `.../O/reset-all` at `TypeDefinition.operations[k].name`, and the same parameter with identity `.../O/reset_all/created-at` at `TypeDefinition.operations[k].params[s].name`; a field `created_at` beside a state `created_at` of one state machine, both with identity `.../O/created_at`, refuses once, `invalid_model_binding`/`conflicting-binding`, at `TypeDefinition.states[i]`; a field whose `name` is absent, is the number `42`, is empty, or is `_tmp` with identity `.../O/_tmp`, and an operation whose `name` is `_tmp` with identity `.../O/_tmp`, each refuse `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.fields[j].name` or `TypeDefinition.operations[k].name`. | Test (TC-235) |
| FR-154-AC-21 | A field `A/b` typed `ix://quire/native/Bytes`, a field `A/g` typed `ix://quire/native/Duration` and a field `A/j` typed `ix://quire/native/JsonObject`, each presence `required` with multiplicity `{1, 1, false, true}`, an operation `A/op` of `A` whose parameter and result are typed `ix://quire/native/Bytes` and `ix://quire/native/JsonObject`, and an alias whose `target` is `ix://quire/native/Duration`, are admitted by a package that declares no `dimensions` and no `units`, with no declaration built for a native reference, no declaration key naming `quire/native`, and `A/g` neither refused nor resolved by intake. A sequence `items` typed `ix://quire/native/Bytes` is admitted. | Test (TC-234) |
| FR-154-AC-22 | A field typed `ix://quire/native/bytes`, `ix://quire/native/duration`, `ix://quire/native/JSONObject`, `ix://quire/native/Json` or `ix://quire/native/BytesList`, and an operation parameter typed `ix://quire/native/bytes`, each refuse `invalid_model_binding`/`malformed-declaration` at the field node and `Field.typeRef` (the parameter at `Operation.params[0].typeRef`), comparing the name byte for byte; the same fields typed `ix://quire/native/Bytes`, `ix://quire/native/Duration` and `ix://quire/native/JsonObject` are admitted, and a package type with artifact id `Bytes` beside them is a different type from `ix://quire/native/Bytes` (a field typed `ix://test/orders/Bytes` resolves to that package type). | Test (TC-234) |
| FR-154-AC-23 | `ix://quire/native/Clock` is admitted, with no declaration built for it, as an operation parameter `typeRef`, an operation result `typeRef`, an alias `target` (alias `Timer`) and as the `typeRef` of a field of that alias; a field typed `Clock` with presence `optional` is admitted as `Option<Clock>`; an operation result `Clock` with multiplicity `{0, 1}` is admitted as `Option<Clock>`. A clock parameter carrying `unit` `ms`, `defaultKind` `semantic`, a `min` constraint or `nullable` `true`, and a clock result with `nullable` `true`, each refuse `invalid_model_binding`/`malformed-declaration` at the member path of the broken shape member (`Operation.params[i].unit`, `Operation.params[i].defaultKind`, `Operation.params[i].constraints`, `Operation.params[i].nullable`, `Operation.returns.nullable`). | Test (TC-234) |
| FR-154-AC-24 | `ix://quire/unit/ms` is admitted, left unresolved by intake, as a field, an operation parameter, an operation result, an alias `target`, a sequence `items` and a map `values`; a unit-typed member carrying `unit`, a `min` constraint, a `decimal` policy or a semantic default refuses `invalid_model_binding`/`malformed-declaration` once at each broken member path; `ix://quire/unit/ms` as a reference `target` refuses `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.target`, and as a constraint `appliesTo` with `min` or `max` is admitted, while the keyword `minLength` there refuses at `Constraint.keyword`. | Test (TC-234) |
| FR-154-AC-25 | A variant case whose `payloadType` resolves to an admitted value type, such as `ix://quire/native/Boolean` or a declared tuple type, is accepted as the member's payload. An unresolved reference or a reference to a type not admitted as a value type, such as `ix://quire/native/Clock`, `ix://quire/unit/ms` or a namespace, refuses `unsupported_construct`/`declaration-form` at `Variant.payloadType`, never `malformed-declaration`. | Test (TC-234, TC-235) |
| FR-154-AC-26 | A field typed `ix://quire/native/Clock` with multiplicity `{0, 3, true, false}` is a `Sequence<Clock>[0, 3]` clock member, with `{0, 3, false, true}` a `Set<Clock>[0, 3]`, with `{0, 3, false, false}` a `Bag<Clock>[0, 3]`, with `{0, 3, true, true}` an `OrderedSet<Clock>[0, 3]` and with an unbounded upper bound, ordered and not unique, a `Sequence<Clock>`; an operation parameter and an operation result typed `Clock` with multiplicity `{0, 2, true, false}` are admitted the same way; each is admitted with no declaration built and no refusal at `Field.multiplicity`; a clock field with multiplicity `{3, 1, true, false}` refuses `invalid_model_binding`/`unpreserved-model-meaning` at `Field.multiplicity`. | Test (TC-234) |
| FR-154-AC-27 | A sequence `items` and a map `values` typed `ix://quire/native/Clock` (the map with `keys` `ix://quire/native/Text` and `bound` `{lower: 0}`), and the same typed by an alias of `Clock`, are admitted with no declaration built for the clock and read as `Sequence<Clock>` and `Map<Text, Clock>`; a field typed by such a sequence with multiplicity `{0, 2, true, false}` is a `Sequence<Sequence<Clock>>[0, 2]`; a record value type with a clock field, held by a field of multiplicity `{0, 2, true, false}`, is a `Sequence<Timer>[0, 2]` clock collection. | Test (TC-234) |
| FR-154-AC-28 | With the unit `ms` and `A/d` a field typed `ix://quire/unit/ms`, a `min` constraint `1` and a `max` constraint `3` on `ms`, the values `1 ms`, `3 ms` and `3/2 ms` are admitted, `0 ms` and `7/2 ms` supplied as an initial state each refuse `invalid_runtime_input`/`invalid-value` naming `A/d`, the constraint identity, its keyword and its bound, an absent optional `A/d` is admitted, a sequence `items` typed `ms` is checked at each element, and a successor from the pre-state `A/d` = `3/2 ms` whose post-state sets `A/d` to the non-constant `self.d + 2 ms` (`7/2 ms`) is recorded as a successor that violates the `max` invariant and is not pruned; `A/d` has the type `Quantity<ms>[1, 3]`, a subtype of `Quantity<ms>`, `convert<Quantity<s>>(self.d)` is checked for every `ms` quantity, and `self.d / x` still refuses `undefined_expression`/`unproved-nonzero` however the `min` reads. | Test (TC-234) |
| FR-154-AC-29 | A `min` constraint on `ms` with `operands.value` `"1/0"`, `"0.5"` and `""` refuses `invalid_model_binding`/`malformed-declaration` at `Constraint.operands`; a `min` `5` with a `max` `3` on the same `appliesTo` refuses the same at the later constraint; a `min` `5` on an alias `D` of `ms` with a `max` `3` on `ms` is admitted and no value of a `D` member satisfies both; a bound on the alias `D` constrains `D` members and no other `ms` member; a quantity of the unit `s` is not bounded by a bound on `ms`. | Test (TC-234) |
| FR-154-AC-30 | A composed document (`type: FR` with `object: value_object`, or with `object: enumeration`, as `FR-204` and `FR-271`) declares no model node: extraction skips it because its `type` is an artifact type even though it carries `object:`, the lift emits none, and a package built from a bundle that holds it equals the package built without it. A relationship end or supertype naming `ix://<package identity>/FR-204` refuses `missing_declaration`/`missing-name`, and no composed document is ever lifted under a name derived from its id. A model-declaring artifact whose emitted type-definition node identity ends in `FR-204` is governed by FR-154-AC-8, not by the source-document rule in this criterion. | Test (TC-234, TC-945) |
| FR-154-AC-31 | With the unit `ms`, an alias `Level` of `ms` and a `min` `0` and a `max` `10` constraint on `Level`, `Level` is the type `Quantity<ms>[0, 10]`: the literals `0 ms`, `5 ms` and `10 ms` check at a call argument for a `Level` parameter, an annotated `let x: Level` and a postcondition assignment to a `Level` field (a unit member has no default, so no default position exists), and `11 ms` refuses `ill_typed`/`type-mismatch` at the literal at each of those positions, naming the member path or alias, the constraint identity, its keyword, its bound, the refusal site and the expected type `Quantity<ms>[0, 10]`; `-1 ms` refuses at the negation expression `-1 ms` (unary minus on the literal `1 ms`, FR-142-AC-25), not at `1 ms`, and `4 ms + 7 ms` refuses at the sum; `Sequence<Level>` and `Option<Level>` members hold `Level` elements and payloads and `sequence[5 ms, 11 ms]` refuses at `11 ms`; with only `min` `0` `-1 ms` refuses and `1000000 ms` checks, with only `max` `10` `-1000000 ms` checks and `11 ms` refuses; `5 s` for a `Level` position refuses as FR-142 refuses a unit mismatch; a `Level` value is read as `Quantity<ms>` (`self.d + 1 ms` is a plain `Quantity<ms>`, `convert<Quantity<s>>(self.d)` checks) and `self.d < 11 ms` and `self.d = 11 ms` as comparison operands are not refused; a `Level` value flows to a `Quantity<ms>[0, 20]` position with no refusal and a plain `Quantity<ms>` value is admitted by the checker to a `Level` position, left to the run-time invariant of FR-154-AC-28; a plain `ms` member is not typed `Quantity<ms>[0, 10]`; `min` `5` with `max` `3` refuses as FR-154-AC-29, and under `min` `5` with a `max` `3` on an alias and its parent every literal at that position refuses. | Test (TC-234) |
| FR-154-AC-32 | Admitting a type whose `fields` are written `b`, `a` and whose `supertypes` are written `Q`, `P`, then rewriting the document with the identity-sorted order of the same fields and supertypes, gives admitted types whose retained `fields` and `supertypes` arrays equal the authored order in each case, and effective field orders (FR-208) that differ accordingly. | Test (TC-386) |
| FR-154-AC-33 | A tuple type with declaration identity `<package identity>/Pair` and positions `p0: Boolean`, `p1: Integer` is admitted as one tuple declaration plus two ordered position declarations; swapping the position types keeps the declaration identity but changes which type is bound to each position identity, and therefore changes the tuple's type identity and product order; an empty positions list, a gap, absent `typeRef` or unsupported type, or an extra field, identity field, operation, supertype or clause refuses `invalid_model_binding`/`malformed-declaration`; an unresolved package `typeRef` refuses `missing_declaration`/`missing-name`; two positions with one `pN` identity refuse `invalid_model_binding`/`conflicting-binding` at the later position. | Test (TC-234) |
| FR-154-AC-34 | `model M = "test/orders";` is a source selection of identity `test/orders` and no other: it carries no version and no digest. The same declaration followed by a `version` or a `digest` token, or by any token other than `;`, refuses `invalid_syntax`/`unexpected-token` at that token. | Test (TC-234) |
| FR-154-AC-35 | A `model` whose identity string is absent, is not a string, or is followed by the end of the source, refuses `invalid_syntax` (`unexpected-token` at the offending token, `unexpected-end` at the end of the source) and is never dropped without a diagnostic. | Test (TC-234) |
| FR-154-AC-36 | Admitting the document supplied under `test/orders` yields the `model_root` node of identity `test/orders` and no lock row; no source, input, lock or expected value states a digest or a version. | Test (TC-234) |
| FR-154-AC-37 | Two documents supplied under one domain package identity, whether or not they differ, refuse `invalid_package`/`conflicting-definition` before intake, naming both; a selection whose identity no supplied document has refuses `missing_import`/`missing-selection` at the identity string. | Test (TC-234) |
| FR-154-AC-38 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-154-AC-39 | An option value type `OptInt` (`kind` `option`, `inner` `ix://quire/native/Integer`) is admitted as one type definition node and one original declaration keyed `(acme/optionals, ix://acme/optionals/OptInt)`, with no declaration for the native `inner` and no declaration key naming `quire/native`; it is admitted as a field `typeRef`, an operation parameter `typeRef`, an operation result `typeRef`, an alias `target`, a sequence `items`, a map `values`, a tuple position `typeRef` and a variant case `payloadType`, and the same shape is admitted over `ix://quire/unit/ms` (`Option<Quantity<ms>>`, left unresolved by intake) and over a record value type. A field typed `OptInt` with presence `required` and multiplicity `{1, 1, false, true}` and a field typed `ix://quire/native/Integer` with presence `optional` and the same multiplicity are both admitted and are the same type `Option<Integer>`; a field typed `OptInt` with multiplicity `{0, 3, true, false}` is `Sequence<Option<Integer>>[0, 3]` and one typed `ix://quire/native/Integer` with presence `optional` and that multiplicity is `Option<Sequence<Integer>[0, 3]>`, both admitted. | Test (TC-234) |
| FR-154-AC-40 | An option value type whose `inner` is an option value type, whose `inner` is an alias of one, and whose `inner` is itself, each refuse `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.inner` and are not flattened; a field typed by an option value type, or by an alias of one, with presence `optional` and multiplicity `{1, 1, false, true}` or `{1, 1, true, false}` refuses `invalid_model_binding`/`malformed-declaration` at `Field.presence`, and the same field with presence `required`, or with presence `optional` and multiplicity `{0, 3, true, false}`, is admitted; an option value type whose `inner` is `ix://quire/native/Clock` or an alias of it, or names a namespace, refuses `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.inner`, and an option over `ix://quire/native/Integer`, a unit and a package value type is admitted; a `reference` whose `target` is an option value type refuses at `TypeDefinition.target`; a field typed `ix://quire/native/Option`, `ix://quire/native/Option<Integer>` or `ix://quire/native/option` refuses `invalid_model_binding`/`malformed-declaration` at `Field.typeRef`. | Test (TC-234) |
| FR-154-AC-41 | An option value type with no `inner`, a `null`, empty, numeric or object `inner`, an `inner` array of one element and an `inner` array of two elements each refuse `invalid_model_binding`/`malformed-declaration` once at `TypeDefinition.inner`; an `inner` that is `Integer` with no URI or `ix://quire/native/Int` refuses the same at `TypeDefinition.inner`; an `inner` that is a well-formed package identity naming no node refuses `missing_declaration`/`missing-name` at `TypeDefinition.inner`; an option value type that declares a field, an operation, a supertype, a position or an `items` member, or a non-empty `constraints`, and a constraint whose `appliesTo` is an option value type, each refuse `invalid_model_binding`/`malformed-declaration` at that member's path, `TypeDefinition.constraints` or `Constraint.appliesTo`; a refused option value type raises no refusal at the fields that name it, and its refusals report in the order shape members, `constraints`, `inner` form, `inner` resolution, `inner` kind. | Test (TC-234) |
| FR-154-AC-42 | A package with the Rational value type `Ratio` (`scalar` `rational`; constraints `maxDenominator` 12, `minNumerator` -3, `minDenominator` 1 and `maxNumerator` 7, in that order, each with `appliesTo` `Ratio`) and a field, an operation parameter and an operation result typed `Ratio` is admitted with one original declaration for `Ratio`; a plain alias of `Ratio` is admitted; `Ratio` with no `constraints` is admitted as the unbounded `Rational`; JSON integer bounds `-2^53` and `2^53` are admitted; and string bounds `"-9223372036854775808"` and `"9223372036854775807"` are admitted without rounding or widening. | Test (TC-981) |
| FR-154-AC-43 | Over `Ratio`, each single defect refuses once at the `Ratio` node: a `minLength` constraint `unsupported_construct`/`declaration-form` at `Constraint.keyword` naming `minLength`; a `maxNumerator` whose `appliesTo` is another value type, or an alias of `Ratio`, `invalid_model_binding`/`malformed-declaration` at `Constraint.appliesTo`; `maxNumerator` given twice the same at the second `Constraint.keyword`; `minDenominator` removed the same at `TypeDefinition.constraints` naming `minDenominator`; `maxNumerator` operands `1.5`, `1e2`, `"3/4"`, `"01"`, `"+1"`, `"-0"`, `"9223372036854775808"` and `"-9223372036854775809"`, a `maxNumerator` with no `operands.value`, and a `maxNumerator` whose `operands` carries a second member, the same at that `Constraint.operands`; `minNumerator` 8, `minDenominator` 0, `minDenominator` 13, `maxDenominator` 0 and `maxDenominator` -1 each `unsupported_construct`/`declaration-form` at `TypeDefinition.constraints` naming the bounds. A JSON integer of magnitude above 2^53, such as `9007199254740993` or `9007199254740994`, is refused earlier as `noncanonical_wire`/`inexact-integer` at its `document_pointer` (FR-272). | Test (TC-981) |
| FR-154-AC-44 | When two defects of `Ratio` are present only the earlier row is reported: `minLength` with `maxNumerator` given twice reports `minLength`; a foreign `appliesTo` with `maxNumerator` given twice reports the `appliesTo`; `minLength` with a foreign `appliesTo` reports `minLength`; `maxNumerator` given twice with `minDenominator` removed reports the repeated keyword; `minDenominator` removed with `maxNumerator` `1.5` reports the missing keyword; `maxNumerator` `1.5` with `minNumerator` 8 reports the operand; `maxDenominator` `0.5` (listed first) with `maxNumerator` `1.5` reports `maxNumerator`, in keyword order. | Test (TC-981) |
| FR-154-AC-45 | A field typed `Ratio` with its own `maxDenominator` constraint refuses `invalid_model_binding`/`malformed-declaration` at `Field.constraints`, an operation parameter typed `Ratio` with its own constraint refuses the same at `Operation.params[0].constraints`, an alias of `Ratio` carrying a constraint refuses the same at the alias node and `TypeDefinition.constraints`, and a constraint carried by another value type whose `appliesTo` is `Ratio` refuses the same at that value type's node and `Constraint.appliesTo`, while `Ratio` itself is admitted. | Test (TC-981) |
| FR-154-AC-46 | `Ratio` with `unit` `ix://quire/unit/m` is admitted by intake; `Ratio` with `unit` `m`, with `unit` `1` and with `unit` `ix://quire/native/Rational`, and a value type of `scalar` `string` with `unit` `ix://quire/unit/m`, each refuse `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.unit`. | Test (TC-981) |
| FR-154-AC-47 | Supplied values `7/12`, `-3/1` and `0/1` to a `Ratio` field are admitted, and `8/1`, `1/13`, `2/4`, `-4/1` and the float `0.5` each refuse `invalid_runtime_input`/`invalid-value` naming the member path, with no value reduced or rounded; with `Ratio` given `unit` `ix://quire/unit/m`, `1/4` is admitted as `1/4 m` and `1/13` refuses the same. | Test (TC-981) |
| FR-154-AC-48 | A map type `M` with `keys` `ix://quire/native/Text`, `values` the record value type `Timer` (clock field `x`, data field `n`) and `bound` `{lower: 0, upper: 2}` is admitted and reads as `Map<Text, Timer>[0, 2]`, a clock collection; with `bound` `{lower: 0}` it reads as `Map<Text, Timer>` with no maximum; `M` with `keys` `ix://quire/native/Integer`, a value type bound as `Int[0, 9]`, a variant type with no payload-bearing case, a unit `ix://quire/unit/ms` or an alias of `Text` is admitted, each the key type it names. | Test (TC-234) |
| FR-154-AC-49 | Over `M`, each single defect refuses once at the `M` node: `keys` `ix://quire/native/Clock`, an alias of `Clock`, `Timer`, a record value type `Cfg` with one `Boolean` field `a`, an option value type over `Text`, a sequence type, a map type, a tuple value type, a variant type with a payload-bearing case, a `reference` type, an object type, the native `ix://quire/native/Rational` and a value type bound as a `Decimal[...]` domain each refuse `ill_typed`/`operator-ineligible` at `TypeDefinition.keys`; an absent `keys`, and a `keys` naming a namespace, refuse `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.keys` and a `keys` naming no node `missing_declaration`/`missing-name` there; an absent `values` refuses `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.values`; an `items` member on `M` refuses it at that member; an absent `bound`, a `bound` that is a string, has a negative or non-integer `lower` or `upper`, has no `lower` or carries another member refuses `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.bound`; `bound` `{lower: 3, upper: 2}` and `bound` `{lower: 1}` (no maximum) each refuse `invalid_model_binding`/`unpreserved-model-meaning` at `TypeDefinition.bound`; a `bound` on a `sequence` type definition refuses `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.bound`. | Test (TC-234) |

## Dependencies

- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) owns the
  model intake design.
- [FR-208](../../objects/foundation/FR-208-quire-meaning-vocabulary.md) owns
  the Quire meaning ids intake binds.
- The filament-core-data Semantic IR contract owns the `operation` member
  set. Intake reads `Operation.redefines` from the IR `operation` member of
  that name, so a producer's contract has to carry it for a document to
  declare an operation redefinition.
- The filament-core-data Semantic IR contract owns the producer-side
  `TypeDefinition` member set and its schema. FR-154 owns Quire's semantic
  interpretation of the producer-defined Rational `constraints` and the
  accepted encoding/range of `Constraint.operands.value` for those bounds.
  This is a value rule inside the existing member, not a new IR member or a
  revision of the producer-side schema. The producer-side contract must carry
  `scalar` `rational`, the `minNumerator`, `maxNumerator`, `minDenominator` and
  `maxDenominator` constraints, and `TypeDefinition.unit` for FR-154 to admit a
  Rational value type ("Rational value types"). Likewise it must carry the
  map type members `keys` and `bound` beside `values` for FR-154 to admit a
  map type ("Map types").
- [Complete model definition](../../../proposals/quire-v1/definitions/model-complete.md)
  owns the intake and feature rules.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns the
  `model` declaration.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
