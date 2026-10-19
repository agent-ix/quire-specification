---
id: FR-375
title: "Declare a refinement between two state models"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-151
    type: references
  - target: ix://agent-ix/quire-specification/FR-362
    type: references
  - target: ix://agent-ix/quire-specification/FR-376
    type: references
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/FR-378
    type: references
  - target: ix://agent-ix/quire-specification/FR-432
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
  - target: ix://agent-ix/quire-specification/FR-307
    type: references
  - target: ix://agent-ix/quire-specification/FR-323
    type: references
  - target: ix://agent-ix/quire-specification/FR-143
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
  - target: ix://agent-ix/quire-specification/FR-001
    type: references
  - target: ix://agent-ix/quire-specification/FR-147
    type: references
  - target: ix://agent-ix/quire-specification/FR-090
    type: references
  - target: ix://agent-ix/quire-specification/FR-250
    type: references
---

# FR-375: Declare a refinement between two state models

## Description

When a unit declares a refinement between an abstract and a concrete state
model, the checker SHALL admit the declaration only when every abstract
population, every mapped abstract field and every concrete operation has
exactly one well-typed row, and SHALL emit the admitted declaration as one
node of the checked package.

## Inputs

A composed unit with two `model` imports (or one `model` import and one
dependency package that holds the abstract model), an infinite-trace profile
alias, and one `refinement` declaration.

## Outputs

An admitted refinement node in the checked package, or a located refusal
naming the offending row.

## Surface syntax

The declaration extends the shared grammar's `declaration` production. The
productions below use the shared grammar's EBNF conventions and its `ident`,
`member-name`, `type-name`, `operation-name`, `model-name`, `type-ref`,
`signed-int`, `exact-number`, `qualified-name` and `expr` productions.

```ebnf
declaration    = … | refinement ;
refinement     = 'refinement', ident, 'using', ident,
                 'abstract', abstract-subject, 'concrete', ident,
                 '{', { refinement-row }, '}' ;
abstract-subject = ident, [ '::', member-name ] ;
refinement-row = population-row | object-row | step-row
               | history-row | fairness-row ;
population-row = 'population', model-name, 'from', model-name, ';' ;
object-row     = 'object', type-name, 'from', type-name,
                 '{', { field-row }, '}' ;
field-row      = member-name, '=', expr, ';' ;
step-row       = 'step', operation-name, '->', step-target, ';' ;
step-target    = operation-name, '(', expr, { ',', step-argument }, ')'
               | 'stutter'
               | 'any' ;
step-argument  = expr | '_' ;
history-row    = 'history', type-name, '.', member-name, ':',
                 type-ref, '=', history-literal,
                 '{', { 'on', operation-name, ':', expr, ';' }, '}' ;
history-literal = signed-int | 'true' | 'false' | 'none' | string
                | exact-number | qualified-name ;
fairness-row   = ('assume' | 'ensure'), 'fair', ( 'weak' | 'strong' ),
                 [ 'each' | 'whole' ], operation-name, ';' ;
```

The first `ident` after `using` names an infinite-trace profile alias; the
`ident` after `concrete` names a model alias. The `abstract-subject` is
either one `ident`, a model alias (a model subject), or `ident '::' name`, a
model alias and the name of a protocol clause of that model (a protocol
subject, for example `abstract Spec::Twice`). The parser recognizes the two
spellings by the presence of `::` alone and never resolves either name; the
checker resolves them at selection (below). A third segment, a leading or
trailing `::` and a protocol name that is not a `member-name` refuse
`invalid_syntax`/`unexpected-token`, at the token [FR-375-AC-11] names. The
token after `::` is always read as the protocol name, because `member-name`
admits reserved words, so `abstract Spec:: concrete Impl {` reads `concrete`
as the name and refuses at `Impl`, where `concrete` was expected. In a step target
the first expression is the abstract receiver and each further argument is
one per declared abstract parameter, in parameter order. A `fairness-row`
with no granularity is `whole`, and its kind is written: `weak` or `strong`,
with the meaning of [FR-362](../temporal/FR-362-apply-fairness-constraints.md).
The kind has no default here. [FR-362](../temporal/FR-362-apply-fairness-constraints.md)
reads an unwritten kind as `weak` in a temporal clause, where `fair` is one
word of a clause that already states its subject; a refinement row is a
member of a fairness set that a transfer later compares by kind
([FR-380](FR-380-transfer-claims-through-refinement.md)), and a silent
`weak` would let a strong row that lost its keyword pass as a weaker
constraint, so a kindless `fair` refuses. The shared grammar has no general `literal`
production; the initial value of a history row is a `history-literal`, a
closed list of constant spellings: an integer, `true`, `false`, `none`, a
string, an `exact-number`, or a `qualified-name` that names an enum member.
Collection, record and float values are not initial literals; any other
spelling in that position refuses `invalid_syntax`/`unexpected-token` at its
first token. Inside a refinement declaration the
expression grammar also admits the population-valued forms of
[FR-376](FR-376-evaluate-population-valued-mapping-expressions.md) and the
option constructor `Some(e)` (**Option rows**, below); the shared grammar owns
their spellings.

## Behavior

**Selection.** The abstract model is a model alias of the unit or of a
dependency package selected in the unit's `dependency_selections`. The
**abstract subject** is the checked package holding the abstract model's
contract clauses; the **concrete subject** is the declaring package. A
profile alias that does not select `quire.temporal.infinite-trace/v1`
refuses `unknown_profile`/`wrong-selection-role` at the alias.

**Protocol subject.** When the `abstract-subject` is `alias::name`, `alias`
resolves as above and `name` resolves among the protocol clauses of the
abstract subject package, the package that holds the abstract model
([FR-425](../protocol/FR-425-define-protocol-subjects-and-state-keys.md) makes
a clause a member of its package). A clause is selectable only if it
acts on the aliased abstract model, in every case: a clause of a dependency
package that imports several models, or of the declaring package when the
unit imports both models itself, whose roles act only on another model
refuses `invalid_model_binding`/`malformed-declaration` at the name. The abstract
subject is then a protocol subject and its step rows follow
[FR-432](../protocol/FR-432-map-every-protocol-step-in-refinements.md). An
alias that is not a model alias of the unit or a selected dependency, or a
name that is not a protocol clause of that package (a type such as
`Spec::Counter`, a population such as `Spec::counters`, or any other
declaration), refuses `missing_declaration`/`missing-name` at the unresolved
segment: a two-segment name is looked up among protocol clauses only, so a
type or population of the same spelling never matches. A single `ident` is
always read as a model alias, so a protocol is never selected without its
model alias. The protocol is part of the declaration's meaning, so the same
model under two protocol names, or under none, gives different refinement
nodes. The selected protocol clause's node is also one of the node's
`dependencies`, beside the nodes its rows name.

**Internal list.** Which abstract protocol steps are internal is not part of
the declaration's source: a profile selection carries an identity and nothing
else, and the refinement grammar has no `internal` row. The list is the
compilation's optional `refinement_internals` input, supplied with the source
beside the supplied libraries of the compile request: one value for the unit
and, for each supplied library, that library's own value
([FR-307](../tooling/FR-307-package-reusable-semantic-libraries.md)). A list
names only refinements declared in the source it is supplied with. Its shape,
class spellings, resolution and refusals are
[FR-432](../protocol/FR-432-map-every-protocol-step-in-refinements.md)'s.
An absent value is the empty list, every abstract step visible.

The locus of each of FR-432's refusals is a pair: the **input** (the unit's
`refinement_internals` value, or the value of a named supplied library) and an
RFC 6901 pointer within that value. This requirement names no member of the
compile request that carries them. In a replay request the value is located
by the request's own members: `/package/refinement_internals` for the proved
package's value and `/package/dependencies/<n>/refinement_internals` for the
`n`th dependency entry's (zero-based). A refusal from a supplied library's
value refuses the whole compilation: the unit is not admitted, whether or not
it imports that library.

The list changes what a declaration means, so it is recorded in the lock
([FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)) and
carried by a replay request ([FR-323](../../objects/interfaces/FR-323-native-runtime-envelope.md));
a replay that dropped it would recompile with an empty list and a different
`package_id`.

**Unbuilt features.** A producer holds a closed support set of the
refinement features it has built, a capability of the producer and not a
request parameter. The feature names are `history-row` (a `history` row),
`protocol-abstract-side` (a protocol abstract subject, `abstract
<A>::<protocol>`), `refinement-internal-list` (a non-empty resolved
internal list for a refinement), and `nested-reference-row` (a field row whose type
nests a reference, see Typing and reference lifting).
This list is the closed set of feature names. Where a producer's support set omits a
feature, the checker SHALL refuse a declaration that uses it
`unsupported_construct`/`not-yet-implemented`, naming the feature, the
refinement and the exact selected profile identity: once for each `history`
row (at its `history` keyword), once per refinement for a protocol abstract
subject (at the protocol name) and once per refinement for a non-empty
resolved list (at the first token of the name written after `abstract`,
which both subject forms have) and once for each field row of nested-reference
type (at the row's expression). The refusals are independent, so a
non-empty list under `abstract Spec::Twice` in a producer omitting both
features gives two. Each refusal produces no refinement node and is decided
before the feature's own checks, so no other diagnostic is reported for that
row; the rest of the declaration is checked as usual.

The internal list is resolved first (FR-432), and its defects (shape,
spelling, `missing-name`, `conflicting-binding`) are reported whatever the
support set says. Only a **non-empty resolved** list then refuses for
`refinement-internal-list`. An object or entry attributable to no refinement
(a value that is not an array of objects, `{}`, an object whose `refinement`
is not a string or names no refinement of the compiled source) is a
resolution defect and gets no support-set refusal, since no refinement
exists to locate it at. An empty list never refuses for the feature. For a
non-empty list on a refinement whose abstract subject is a model, a producer
whose support set lacks `refinement-internal-list` reports only the
not-yet-built refusal at `Spec`; the model-subject refusal
`invalid_model_binding`/`malformed-declaration` (FR-432) applies once the
feature is built.

The refusal is an implementation gap, not a source defect (FR-452 classes it
`unsupported`), which is why it is not `unknown_required_feature`; and it is
`unsupported_construct`/`not-yet-implemented`, not `unimplemented_capability`,
because a feature is a source form or declaration input
([FR-271](../../objects/foundation/FR-271-diagnostic-code-vocabulary.md)'s
rule). A feature in the support set is checked by its own requirement.
`nested-reference-row` has no such requirement yet: this requirement states no
meaning for a nested reference, so a producer's support set holds it only when
a requirement states one.

**Population rows.** Each abstract population has exactly one row naming one
concrete population as its source. An abstract population with no row
refuses `missing_declaration`/`missing-name`; one with two rows refuses
`invalid_model_binding`/`conflicting-binding`, naming both rows. A concrete
population that is no row's source is concrete-only state.

**Object rows.** Each abstract object type that an abstract population admits
is mapped from one concrete object type of that population's source. Each
`field-row` defines one abstract field. Its expression has the concrete
object as `self` and reads the concrete state through the forms a
state-clause body admits, history fields
([FR-378](FR-378-check-refinement-auxiliary-state.md)), pure function
application and the FR-376 forms. A field named in two rows refuses
`invalid_model_binding`/`conflicting-binding`, naming both rows. A field with
no row is a **hidden field** (FR-378).

**Typing and reference lifting.** A field row's expression conforms to the
abstract field's declared type under the conformance rule of
[FR-151](../type-model/FR-151-resolve-conformance-redefinition-dispatch.md),
after reference lifting: a concrete reference to an object of a mapped type
denotes the abstract object with the same key, so `Reference<T_C>` conforms
to `Reference<T_A>` exactly when an object row maps `T_A` from `T_C`. A row
that does not conform refuses `ill_typed`/`type-mismatch` at the row's
expression.

A reference is lifted only as the whole type of a row's expression, or as the
element of a whole-population reference row (`Set<Reference<T_C>>[0, n]`,
[FR-376](FR-376-evaluate-population-valued-mapping-expressions.md)), where each
element is lifted by key. A field row whose type holds a `Reference<T_C>`
nested inside an option, record, union or collection type that is not such a
population reference (for example `Sequence<Reference<T_C>>`, or the
`Option<Reference<T_C>>` of `Some(self.ring)`) uses the feature
`nested-reference-row`: lifting inside a nested value is not defined here. In a
step argument or a history update a nested reference is not lifted either, so
it does not conform and refuses `ill_typed`/`type-mismatch` there.

**Row layers.** The infinite-trace profile a refinement's `using` names
requires `quire.value.complete/v1` and `quire.model.complete/v1` for the
refinement's rows when it governs the refinement
([FR-090](../temporal/FR-090-select-temporal-profile-and-clock.md),
[FR-250](../../objects/temporal/FR-250-temporal-semantic-profile.md)). The rows
are admitted under exactly those two layers and their `requires` closure
(`quire.state.core/v1`, `quire.state.queries/v1`, `quire.state.graph/v1`),
whatever the unit's other declarations select, and under no other layer. So
`mod` ([FR-147](../expressions/FR-147-evaluate-integer-division-domains.md)),
`Reference` comparison such as `s.ring = self`
([FR-149](../type-model/FR-149-apply-complete-equality-matrix.md)) and the
undefined reasons of the native diagnostics check in a row. A row using a form
outside that set, for example the protocol reference `protocol Spec::Twice`, refuses
`unsupported_construct`/`expression-form` at the `protocol` token, naming
`quire.model.complete/v1` ([FR-001](../foundation/FR-001-select-native-profile.md)).
These layers count toward the unit's selected layers for the complete-value
lock, so a row using `div` or `rem` brings the lock's integer-division law and
the package identity names the definitions the rows run under (FR-001).
The profile alias carries no layer list and a refinement has no second `using`.

**Abstract relationships.** No row maps a relationship, so the abstract
model's relationship links could never hold in a mapped state and an abstract
clause reading one would pass or refute by accident. This is a gap in the
language, not a feature a producer has left unbuilt. A refinement whose
abstract model declares a relationship refuses
`unsupported_construct`/`declaration-form` at the abstract alias of the header,
once per refinement, naming the form (a refinement of a model that declares a
relationship), the refinement and the exact selected profile identity, and
produces no node; the refusal is independent of every support set, and no profile admits the form yet, so the profile identity named is the selected one and not a profile that prohibits it ([FR-452](../tooling/FR-452-preserve-admission-across-specification-revisions.md) classes the refusal `prohibited`). The mapped
state ([FR-377](FR-377-decide-model-refinement.md)) holds no relationship extent.

**Option rows.** `Some(e)` has type `Option<E>` for `e` of type `E`. `none`
takes its option type from the abstract field's type at a field row, or from
the other arm of an enclosing `if`; where neither supplies one (an argument, a
comparison operand, an `if` against `Some(e)` with no field type) it refuses
`ill_typed`/`type-mismatch`. An abstract field declared with `?` takes a row of
type `Option<T>`: the row `none` leaves the field absent and `Some(v)` sets it
to `v`. An abstract field declared `Option<T>` takes the row's option value
unchanged, so `none` is an explicit `none` and the field is set. A row reads a
concrete `?` field only through `present` and `value`
([FR-143](../type-model/FR-143-evaluate-record-tuple-recursive-values.md)); a
bare `self.f` of a `?` field is `ill_typed`/`type-mismatch` in a row as
anywhere else.

**Step rows.** Each concrete operation has exactly one step row. An operation
with no row refuses `missing_declaration`/`missing-name`, naming the
operation; no row is supplied by default. An operation with two rows refuses
`invalid_model_binding`/`conflicting-binding`. An abstract-operation target
has one receiver expression and one argument per abstract parameter, each
typed against the abstract parameter type after reference lifting, over the
concrete receiver as `self`, the concrete arguments by parameter name and
the concrete pre-state; `_` leaves that argument open. A wrong argument
count refuses `ill_typed`/`operator-ineligible`. When both operations declare a
result, the concrete result type conforms to the abstract result type after
reference lifting, or the row refuses `ill_typed`/`type-mismatch`.

**History rows.** A history row names a concrete object type, a new field
name that the type does not declare, a bounded declared type and an initial
`history-literal` of that type, and at most one update per concrete
operation of that type. An update expression reads the step's receiver as
`self` (post-state fields), `pre(...)` of pre-state fields and of the history
field, and the step's arguments; it is typed against the history field's type. A name the
concrete type already declares refuses `invalid_model_binding`/
`conflicting-binding`; an unbounded type refuses `ill_typed`/`type-mismatch`;
two updates for one operation refuse `invalid_model_binding`/
`conflicting-binding`.

**History literals.** A `history-literal` conforms to the declared type only
when its value is a member of that type's value set, range included.
A literal's value is statically known, so a non-member is decided at the
literal and refuses `ill_typed`/`type-mismatch` there: `5` for `Int[0, 4]`,
`true` for `Int[0, 4]`, `0` for `Boolean`, a string longer than a `Text`
bound. This differs from the unproved-range case of
[FR-149](../type-model/FR-149-apply-complete-equality-matrix.md), which
concerns computed expressions whose range is not statically decided. `none`
conforms only to an `Option<T>` type, and `true`/`false` only to `Boolean`.
An enum member is written with its full path, the enum type's
`qualified-name` followed by `::` and the member, so `Impl::Mode::idle` is
three segments; a `qualified-name` that does not resolve to an enum member
(a bare `ident`, a path of too few segments, or a name that is not an enum
member) refuses `missing_declaration`/`missing-name` at the name, and a
member of an enum other than the declared type refuses
`ill_typed`/`type-mismatch`.

**Fairness rows.** `assume` rows form the concrete fairness set `F_C` and
name concrete operations; `ensure` rows form the abstract fairness set `F_A`
and name abstract operations. Each row is a weak or a strong constraint of its
operation and granularity, so a strong row constrains the set as FR-362
states; the kind is part of the row, and `fair weak each op` and
`fair strong each op` are different rows. A row is identified by its
triple of kind, operation and granularity, with an unwritten granularity
read as `whole`; two rows of one set with the same triple, written alike or
not (`fair weak op` and `fair weak whole op`), refuse
`invalid_model_binding`/`conflicting-binding`, naming both rows. A row naming an operation of the
other model refuses `missing_declaration`/`missing-name`. Either set may be
empty.

**Checked-package node.** An admitted declaration is one
`quire.checked-semantic-graph/v2` node with `node_tag: "relation"` and
`semantic_form: "refinement"` ([FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)).
Its `semantic_type` is the concrete model declaration node, and its
`dependencies` hold every node its rows name. Its body is
`{"term": "refinement", "abstract", "profile", "populations", "objects",
"steps", "histories", "assume", "ensure"}`: `abstract` is the abstract
model's `PackageNodeKey`, and a protocol subject adds `protocol`, the
selected protocol clause's `PackageNodeKey` (the entry is absent for a model
subject); `populations`, `objects`, `steps` and `histories`
hold one entry per row, each carrying its names as node keys and its
expressions as `SemanticTerm`s; a step entry's `target` is
`{"kind": "operation", "operation", "receiver", "arguments"}` with a
`{"term": "open"}` term for each `_`, or `{"kind": "stutter"}`, or
`{"kind": "any"}`; `assume` and `ensure` hold `{operation, kind, granularity}`
entries with `kind` exactly `weak` or `strong` and `granularity` exactly
`each` or `whole`. A protocol subject whose resolved internal list is
non-empty also carries `abstract_internal`, the resolved classes as
**alias-free spellings** (a class's FR-432 spelling with `<A>::` removed, such
as `Twice::finish`, `lose Twice::ch`, `activate Twice`, `memory flush`) in
ascending UTF-8 byte order; the abstract model and the protocol clause are
identified by `abstract` and `protocol`, so the entry does not depend on the
local abstract alias, and renaming the alias, with the list renamed to
match, leaves the node id unchanged. It is absent for an empty list, as
`protocol` is absent for a model subject, and never null. Every other array
of the body (`populations`, `objects`, `steps`, `histories`, `assume`,
`ensure`) is sorted ascending by the node-key digest of its first named declaration, then by
member name as UTF-8 bytes; within an `objects` entry the field rows are sorted
ascending by field name as UTF-8 bytes. The node enters the identity projection, so it
enters the `package_id` and the obligation identity of every item that
cites it; the internal list is therefore part of both identities through
`abstract_internal`, and needs no member of its own in the identity preimage.

**Claim kinds.** A request to check the whole declaration is one item of
kind `temporal-satisfaction`; each per-step simulation obligation is one item
of kind `operation-contract` ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-375-AC-1 | A declaration with one population row per abstract population, one well-typed object row per admitted abstract type, one step row per concrete operation and well-typed history and fairness rows is admitted and emitted as one `relation`/`refinement` node whose body carries every row. | Test (TC-310) |
| FR-375-AC-2 | A concrete operation with no step row refuses `missing_declaration`/`missing-name` naming the operation, and the declaration is not admitted with any default target; the same declaration with `step … -> any;` written for it is admitted. | Test (TC-310) |
| FR-375-AC-3 | An abstract population with no row refuses `missing_declaration`/`missing-name`; a second row for one population, a field named twice, a second step row for one operation, a second history update for one operation and two fairness rows of one set with the same kind, operation and granularity each refuse `invalid_model_binding`/`conflicting-binding` naming both rows. | Test (TC-310) |
| FR-375-AC-4 | A field-row or step-argument expression whose type does not conform to the abstract type after reference lifting refuses `ill_typed`/`type-mismatch` at that expression; a `Reference<T_C>` expression for a `Reference<T_A>` field is admitted exactly when an object row maps `T_A` from `T_C`. | Test (TC-310) |
| FR-375-AC-5 | A history row whose name the concrete type already declares, whose type is unbounded, or whose update is ill-typed refuses with its named cause; an admitted history field appears in no concrete clause, frame or successor relation. | Test (TC-310) |
| FR-375-AC-6 | A fairness row with no granularity is read as `whole`; an `assume` row naming an abstract operation, or an `ensure` row naming a concrete operation, refuses `missing_declaration`/`missing-name`. | Test (TC-310) |
| FR-375-AC-7 | Changing any row of an admitted declaration changes the refinement node's id, the `package_id` and the obligation identity of an item that cites it; reordering rows in source does not. | Test (TC-310) |
| FR-375-AC-8 | A request to check the whole declaration requires `temporal-satisfaction`, each per-step simulation obligation requires `operation-contract`, and neither requires `refinement`. | Test (TC-310) |
| FR-375-AC-9 | A history row with an initial `history-literal` written as a negative or non-negative integer, `true`/`false`, `none`, a string, an exact number or a full-path enum member is admitted when its value is in the declared bounded type's value set; a literal of the right kind outside the value set or of the wrong kind (`5` or `true` for `Int[0, 4]`, `none` for a non-`Option` type) refuses `ill_typed`/`type-mismatch` at the literal; an unresolved enum path refuses `missing_declaration`/`missing-name`; a collection, record or float spelling refuses `invalid_syntax`/`unexpected-token`. | Test (TC-310) |
| FR-375-AC-10 | An `assume` or `ensure` fairness row written `fair strong` is admitted with or without `each` or `whole`, and its emitted entry carries kind `strong`; the same row written `fair weak` carries kind `weak`, and the two rows give different refinement node ids; `fair` with no kind, or any kind other than `weak` or `strong`, refuses `invalid_syntax`/`unexpected-token` at the token, because a refinement row has no default kind. | Test (TC-310) |
| FR-375-AC-11 | `abstract Spec` selects a model subject; `abstract Spec::Twice` and `abstract Spec::Twice2`, two protocol clauses of the abstract package with identical operation-target rows, are each admitted, and the three declarations (`Spec`, `Spec::Twice`, `Spec::Twice2`) have three distinct node ids, the protocol subjects carrying `protocol` and listing the clause among `dependencies`. `abstract Spec::Thrice` (no such clause), `abstract Spec::Counter` (a type), `abstract Spec::counters` (a population) and `abstract Nope::Twice` (no such alias) refuse `missing_declaration`/`missing-name` at the unresolved segment (`Thrice`, `Counter`, `counters`, `Nope`); a clause of the abstract package that acts only on another model refuses `invalid_model_binding`/`malformed-declaration` at the name. Syntax refusals are `invalid_syntax`/`unexpected-token` at these tokens: `abstract Spec::Twice::finish concrete Impl` at the second `::`; `abstract ::Twice` at the `::`; `abstract Spec:: concrete Impl` at `Impl` (`concrete` is read as the name); `abstract Spec:: {` at the `{`; `abstract Spec::1` at the `1`. | Test (TC-310) |
| FR-375-AC-12 | The `refinement_internals` input is the only host of the internal list: `abstract Spec::Twice` with no row naming internals and the unit value `[{"refinement": "R", "internal": ["Spec::Twice::finish"]}]` is admitted with `abstract_internal` `["Twice::finish"]`; the same source with the value absent, `[]` or naming another refinement is admitted with no `abstract_internal` entry. A supplied library declaring its own refinement and given its own value is admitted with that value affecting only the library's node and `package_id` and the importer's selection of it; a value naming a refinement the library does not declare refuses `missing_declaration`/`missing-name` at (that library's value, `/0/refinement`) and the compilation, the unit included, is not admitted. | Test (TC-310) |
| FR-375-AC-13 | The two lists `["Spec::Twice::finish"]` and none give refinement nodes with different ids, `package_id`s and obligation identities for an item citing the node; reordering the value's objects and entries changes none of them; renaming the abstract alias from `Spec` to `S`, with the value's spellings renamed to match, changes none of them. | Test (TC-310) |
| FR-375-AC-14 | For each feature a producer's support set omits, a declaration using it refuses `unsupported_construct`/`not-yet-implemented` as the Unbuilt features paragraph states: two `history` rows give two refusals at their keywords naming `history-row`; `abstract Spec::Twice` one at `Twice` naming `protocol-abstract-side`; a non-empty resolved list one at `Spec` naming `refinement-internal-list`, both together giving two refusals for the one declaration; each names the refinement and the profile identity and produces no node. A list with a resolution defect (`missing-name`, `conflicting-binding`, malformed) reports that defect and no support-set refusal for the list, and a value attributable to no refinement reports its defect alone. A producer that has built a feature is checked by that feature's own criteria, and this criterion places no test on it. | Test (TC-310) |
| FR-375-AC-15 | A field row of type `Sequence<Reference<T_C>>[0, 2]` for an abstract `Sequence<Reference<T_A>>[0, 2]` field, and the row `Some(self.ring)` for an abstract `?` field of type `Reference<T_A>`, each use `nested-reference-row`: where the support set omits it, each refuses `unsupported_construct`/`not-yet-implemented` naming the feature at the row's expression, once per such row, and no node is produced; a whole-population reference row and a plain `Reference<T_C>` row for a `Reference<T_A>` field are admitted; a `Sequence<Reference<T_C>>` step argument for a `Sequence<Reference<T_A>>` parameter refuses `ill_typed`/`type-mismatch`. | Test (TC-310) |
| FR-375-AC-16 | A refinement whose abstract model declares a relationship refuses `unsupported_construct`/`declaration-form` at the abstract alias of the header, once per refinement, naming the form, the refinement and the selected profile identity, whatever the producer's support set, and produces no node. | Test (TC-310) |
| FR-375-AC-17 | `Some(e)` for `e` of type `E` is typed `Option<E>`; over an abstract `?` field of type `T`, the row `none` leaves the field absent and `Some(v)` sets it, and a row that is not `Option<T>`, including a bare `self.f` of a concrete `?` field, refuses `ill_typed`/`type-mismatch`; over an abstract `Option<T>` field the row's option value is taken unchanged; `none` with no field type or other `if` arm refuses `ill_typed`/`type-mismatch`; outside a refinement declaration `Some` is an ordinary identifier, so `Some(1)` in a state clause refuses `missing_declaration`/`missing-name` at `Some` when no function `Some` is declared and is a call to it when one is. | Test (TC-310) |
| FR-375-AC-18 | In a refinement governed by `quire.temporal.infinite-trace/v1`, the ring-buffer rows `s.ring = self`, `(self.head + i) mod 2` and an `a div b` row are admitted even when the unit's own declarations select no layer, the lock selection counts the rows' layers (the `always` rows and exactly one integer-division law, `selection_trigger_unsatisfied` when none is selected), and a row using `protocol Spec::Twice` refuses `unsupported_construct`/`expression-form` at the `protocol` token naming `quire.model.complete/v1`. | Test (TC-310) |

## Dependencies

- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) owns
  the checked-package graph that carries the refinement node.
- [FR-151](../type-model/FR-151-resolve-conformance-redefinition-dispatch.md)
  owns the conformance rule used for row typing.
- [FR-376](FR-376-evaluate-population-valued-mapping-expressions.md) owns
  the population-valued expression forms admitted inside the declaration.
- [FR-377](FR-377-decide-model-refinement.md) owns what an admitted
  declaration means; [FR-378](FR-378-check-refinement-auxiliary-state.md)
  owns history and hidden fields.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns the
  claim-kind vocabulary.
- [FR-432](../protocol/FR-432-map-every-protocol-step-in-refinements.md)
  owns the step rows of a refinement whose abstract or concrete subject is a
  protocol, and the shape, spellings and resolution of its internal list.
- [FR-272](../../objects/foundation/FR-272-typed-diagnostic-cause.md) owns the
  `not-yet-implemented` cause.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns
  the productions this requirement extends.

## References

- Design: quire-spec-language ADR-020 (RM-1 to RM-7, AX-1, QS-1, QS-4,
  QS-5, RU-1).
- Tickets: Linear STD-133; paired QSL-367; Linear STD-182 (protocol abstract
  subject); Linear STD-199 (internal step list, unbuilt-feature refusal).
