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
population, every mapped abstract field, every state-machine field set and
every concrete operation has exactly one well-typed row, and SHALL emit the
admitted declaration as one node of the checked package.

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
nodes. The selected protocol clause's node is in the refinement node's
`dependencies` when the clause is in the declaring package. A clause of a
directly selected dependency package instead joins through that dependency
selection and is not a local `dependencies` entry. Both forms remain
selectable, and both enter the refinement body as its `protocol` key.

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
a replay that dropped it would recompile with an empty list and a
structurally different checked package.

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
is mapped from one concrete object type of that population's source. A
`state_machine` declaration also contributes a field set to the model state:
each abstract `state_machine` declaration is mapped by one `object-row` to
one concrete `state_machine` declaration, without a population row. The row's
`field-row`s map the abstract machine's declared fields from expressions over
the concrete machine's declared fields. A `state_machine` row is not an
object-type row and does not create an object or population. A model may have
no `state_machine` declaration; its mapped object-type fields remain a valid
model subject through the population and object rows. Multiple
`state_machine` declarations are mapped by their explicit rows and are not
ambiguous. An `object-row` SHALL pair two object types or pair two
`state_machine` declarations; it SHALL NOT mix those declaration kinds.
An abstract `state_machine` declaration without a row refuses
`missing_declaration`/`missing-name`, naming that declaration. Two rows for
one abstract machine refuse `invalid_model_binding`/`conflicting-binding`,
naming both rows. A concrete type that is not a `state_machine` declaration
for a state-machine row refuses `invalid_model_binding`/`malformed-declaration`.
Each `field-row` defines one abstract field. Its expression has the concrete
object or concrete machine field set as `self` and reads the concrete state
through the forms a state-clause body admits, history fields
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
outside that set, for example the protocol reference `protocol Fill` naming a protocol declared in the unit, refuses
`unsupported_construct`/`expression-form` at the `protocol` token, naming
`quire.protocol.complete/v1`, the layer that admits the form and that the rows' layer set lacks ([FR-001](../foundation/FR-001-select-native-profile.md)).
These layers count toward the unit's selected layers for the complete-value
lock, so a row using `div` or `rem` brings the lock's integer-division law and
the package identity names the definitions the rows run under (FR-001).
A profile the unit declares for other declarations, such as the one a protocol
uses, does not widen the layers of a refinement's rows. The profile alias
carries no layer list and a refinement has no second `using`.

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
Its `semantic_type` is the concrete selected model package's canonical
`model_root` node, not one of the package's object-type or state-machine
declarations. The root is derived from the existing selected model package
identity (FR-322); it exists independently of declaration count
and source alias. Its
`dependencies` are the distinct local `NodeRef`s named by the body's
`abstract` and optional `protocol` keys, every row's direct declaration and
type key, and the row expressions' `reference` targets and application member
declarations and populations, plus every local `class_catalog` class node and
binder type key, in ascending node-id order. This rule
includes the local header keys even when every row array is empty. As in
FR-322, a literal's `type`, an application's `result_type` and the node's own
`semantic_type` are annotations, not dependencies unless another listed body
key or reference names the same node. External `{package, node}` keys and the
profile `DefinitionRef` never enter this local list; dependency-package nodes
join through the selected dependency package. Its body is
`{"term": "refinement", "name", "abstract", "profile", "populations", "objects",
"steps", "histories", "assume", "ensure"}` with the conditional
`class_catalog` below. Its node id is FR-322's structural
`{package, path, ordinal}` with `path` `["relation", "refinement", name]` (the
derived form over the body's authored `name`) and `ordinal` 0. `semantic_type` is the concrete model root's `NodeRef`,
exactly the node's `semantic_type` member; it is required even if
every row array is empty. The refinement's own authored name is part of the
node id (below), so two declarations never share a node. A `NodeRef` has
FR-322's `node_id` shape and names a node of this graph. A `{package, node}` key
is the closed `{package: identity, node: node_id}` shape, `identity` being the
dependency package's identity string, and names a node of a directly
selected dependency package. Its `node` is that node's `node_id` as the
dependency package writes it: each index segment `{"node": i}` of `node.path`
names `semantic_graph.nodes[i]` of the dependency package the key's `package`
names, never a node of this graph (FR-322 "Operand index segments"), so a key
naming a derived node, such as the no-point `operation_anchor` of an abstract
operation target, keeps its index segments and its fixed depth. When the reader
binds the selected dependency's admitted package, before every refinement join,
an index segment of such a key whose index is not below the length of that
package's `semantic_graph.nodes` refuses `malformed_wire`, with no cause, at
the segment's `node` member, `.../node/path/{s}/node`; the key then joins the
dependency node by its rebuilt structural key. A `RefinementKey` is exactly one of those two
shapes: a key for this graph MUST be a `NodeRef`, and a key for a dependency
MUST be a `{package, node}` key. Source aliases and source-order positions are
never keys. `abstract`
is a `RefinementKey` for the abstract model declaration; a protocol subject
adds `protocol`, a `RefinementKey` for the selected protocol clause (absent
for a model subject). Both keys use the same local-or-dependency rule.
`profile` is the selected infinite-trace profile's `DefinitionRef`, not the
local profile alias.

**Refinement identity.** The body's `name` is the refinement's authored
`ident` exactly as written in the `refinement` header (an `Identifier`; never
qualified, never an alias, never a source position). The authored name is
part of the node's identity: two refinement declarations with different names
are two nodes with two ids, even when their abstract model, protocol, profile,
concrete model and every row are equal, and each node carries its own
declaration occurrence and source-map entry. This follows the owner-nested
declaration identities of FR-322, in which a declaration's name enters its
identity; it also matches the protocol rule above, where a changed part of the
declaration's meaning gives a structurally unequal node whose id is unchanged
except by a rename. Renaming a refinement therefore
changes its node id and the obligation identity of every
item that cites it; renaming a model or profile alias does not. Two refinement
declarations of one unit with the same name refuse
`ambiguous_declaration`/`ambiguous-name` at the second declaration's name
token, before row checks; a producer never emits one node for two declarations
and never fails internally on valid input that declares two refinements of
equal meaning. A refinement node carries no `declaration` or `owner` (FR-322); `name` is a
body member, so no FCD declaration field is added.

The six row arrays use these exact closed entries; all arrays are present,
including empty arrays. `SemanticTerm` means FR-322's closed body term, with
composite expressions referenced through their own nodes rather than nested
applications. A row's local declaration key is a `NodeRef`; a key for an
abstract declaration uses `RefinementKey`.

| Array | Entry | Meaning |
| --- | --- | --- |
| `populations` | `{abstract: RefinementKey, concrete: NodeRef}` | The abstract and source concrete population declarations. |
| `objects` | `{abstract_type: RefinementKey, concrete_type: NodeRef, fields: [{name: Identifier, expression: SemanticTerm}]}` | One object-type mapping or one state-machine field-set mapping and its field rows; `name` is an abstract field name, resolved on `abstract_type`. For a state-machine mapping both type keys name `state_machine` declarations; it does not imply an object or population. |
| `steps` | `{source_class: Nonempty, source_node?: NodeRef, target: StepTarget}` | One concrete operation or protocol step class. `source_class` is its alias-free canonical class spelling; `source_node` is required for a class with a declaration node and absent for a memory step class, which names no declaration. The reader resolves the spelling against the concrete subject and checks the node when present. |
| `histories` | `{concrete_type: NodeRef, name: Identifier, type: NodeRef, initial: HistoryInitial, updates: [{operation: NodeRef, expression: SemanticTerm}]}` | One history field; `HistoryInitial` is a `literal` or local `reference` term naming the admitted `history-literal`; each update names the concrete operation it follows. |
| `assume` | `{operation: NodeRef, kind: "weak"|"strong", granularity: "each"|"whole"}` | A concrete fairness row. |
| `ensure` | `{operation: RefinementKey, kind: "weak"|"strong", granularity: "each"|"whole"}` | An abstract fairness row. |

`StepTarget` is one of four closed shapes. An abstract operation target is
`{kind: "operation", operation: RefinementKey, receiver: SemanticTerm,
arguments: [StepArgument]}`. An abstract protocol target is
`{kind: "protocol_step", class: Nonempty, node?: RefinementKey,
arguments: [StepArgument]}`; its `class` is the alias-free FR-432 spelling,
and `node` is required for a class with a declaration node and absent for a
memory class. The remaining shapes are `{kind: "stutter"}` and
`{kind: "any"}`. `StepArgument` is either a `SemanticTerm` or exactly
`{term: "open"}` for a written `_`. The open term is admitted only in the
arguments of a step target; it is not a general `SemanticTerm`. Every
`source_node`, `operation`, `node`, expression reference and type key is
checked against the selected subject and its declared kind; a schema-valid
but unresolved or wrong-kind key is refused under the existing graph and
model-binding rules, not silently dropped. `histories[].updates` is sorted
by its operation node id, and `objects[].fields` by field name in
UTF-8 byte order.

**Step-row provenance.** An **operation class** is the step class of one
operation `T::op` of the selected concrete model, where `T` is the object
type, process or state machine that **declares** `op`: an operation a type only
inherits is the class of its declaring type, as FR-342 anchors it. Its
alias-free `source_class` spelling is `T::op`, the row's `<C>::T::op` with the
concrete alias `<C>::` removed (FR-432). Its `source_node` is the **no-point
`state`/`operation_anchor` node of `(T, op)`** that
[FR-342](../../objects/interfaces/FR-342-checked-package-operation-anchor-body.md)
keys by (`context` `T`, `operation` `op`, no execution point). The anchor's
`context` is the declaring type `T`; it is never the refinement's
`semantic_type` (the concrete model root), the model-owner node or the
protocol clause. The rule is the same with and without a `class_catalog`:

- with a catalog, the concrete catalog entry for the class is
  `{class: "T::op", node: <that anchor>, binder_types}` where `binder_types`
  is the operation's binder record, and the row's `source_node` MUST equal the
  entry's `node`;
- without a catalog (a model-only refinement), `source_node` MUST be a local
  no-point `operation_anchor` whose `context` is `T`, whose `operation` is
  `op`, and whose `T` is a declaration owned by the selected concrete model
  package; the reader derives `T` and `op` from `source_class`.

The producer SHALL emit the no-point anchor and its frame node
([FR-340](../../objects/interfaces/FR-340-checked-package-frame-location.md))
for every operation class a step row names, even when no clause, attempt or
transition names that operation. Such an anchor carries exactly one `anchor`
occurrence, the source region of the operation's declaration (so FR-322's
every-node-has-an-occurrence rule holds), and FR-342 states the same. The row
is not an occurrence of the anchor; its provenance is the
`refinement_step_rows` sidecar. A memory class has no
`source_node`. The abstract operation target's `operation` key names, by the
same rule, the no-point anchor of (the abstract declaring type, the abstract
operation), as a local `NodeRef` or a `{package, node}` key. A `source_node` that is
an anchor of another operation, a point-qualified anchor, an anchor of a type
the selected concrete model does not own, or not an `operation_anchor`, or
a missing `source_node` on an operation class, refuses
`invalid_model_binding`/`malformed-declaration` at that `source_node` (or at the
step entry when absent); a `source_class` naming no operation that `T` itself declares refuses
`missing_declaration`/`missing-name` at `source_class`. The class spelling
resolves among DECLARED operations only, so the spelling of an inherited
operation under the inheriting type (`Sub::reset`) is a missing class, not
FR-342-AC-4's anchor refusal. The class is resolved before `source_node` is
examined: an unresolvable class refuses at `source_class` whatever
`source_node` holds. A package is one unit of compilation (a supplied library
is compiled as its own package), so the reader's package-level equal-`name`
check and the unit-scoped source check cover the same set of declarations.

**Row-expression binders.** In a row expression, `self` and the concrete
operation's arguments are encoded in the style of
[FR-341](../../objects/interfaces/FR-341-checked-package-state-clause-body.md)'s
state clauses: one `value`/`parameter` node per binder, read through a
`reference` term. `self` is the node named `"self"` at level `0`, its
`semantic_type` `Reference<C>` for `C` the declaring concrete type of the row
(an object type, a process or a state machine); each operation parameter is the
node named by its declared name at level `1..n` in declared order, its
`semantic_type` the parameter's type. A step row has no `result` binder, and an
object row has the `self` binder alone. These are FR-341's own parameter nodes,
so the binders of a step row of `op` are the very nodes of `op`'s
precondition clause. A pre-state field is read as FR-341 reads it: a
`quire.op.record.project` application with member `{kind: "field", name,
declaration}` over a `quire.op.model.deref` application of the `self`
node. A binder is never encoded by an inline name, an alias or a position. A
`reference` to a `value`/`parameter` node whose name, level or type is not the
binder's refuses `invalid_model_binding`/`malformed-declaration` at the
reference; an argument whose type does not conform refuses
`ill_typed`/`type-mismatch` as above. Every binder node a row reads is in the
refinement's exact local `dependencies` as an expression `reference` target.

**Checked class catalog.** When either subject contributes a protocol or
memory step class, the producer SHALL emit `class_catalog` in the refinement
body; otherwise the member SHALL be absent. This closed object has exactly
`concrete` and `abstract` arrays. Each lists the complete source-level step
classes of its selected subject, once each by **catalog key**, in ascending
order of that key. The catalog key of a declared class entry is its
alias-free `class` spelling alone. The catalog key of a memory entry is the
pair (`class`, `memory_model`): the `class` spelling is shared by every
model that defines the class (`memory drain` is a class of each of `sc`,
`tso` and `ra`), so the model, not the spelling, tells two such entries
apart. Entries order by the UTF-8 bytes of `class`, then, when the spellings
are equal, by the UTF-8 bytes of `memory_model` (`ra`, then `sc`, then `tso`).
A declared class and a memory class never share a spelling, since the
declared spelling excludes the `memory ` prefix. Two entries of one array
with equal catalog keys are a duplicate and refuse as the canonical-order
rule below states. A declared class entry is exactly
`{class, node, binder_types}`: `node` is a local `NodeRef` for a concrete
class and a `RefinementKey` for an abstract class; `binder_types` is the
ordered array of local `NodeRef`s or abstract-side `RefinementKey`s for that
class's checked binder record and may be empty. A memory entry is one of
`{class: "memory drain", memory_model, drain}` and
`{class: "memory flush", memory_model: "tso"}`, with no `node` or
`binder_types`. A drain entry's `memory_model` is `sc`, `tso` or `ra`, and
its `drain` is `store-buffer-flush`. A
`memory drain` is a class of each source-default model (`sc`, `tso` and
`ra`), and the catalog contains it for each of those models selected by a
source `parallel` in that subject. Executing the class drains the thread's pending writes until they
are visible according to that model's visibility rule; under `sc` it is a
no-op. Under `tso` it flushes the thread's store buffer in FIFO order; under
`ra` it has no store-buffer entries to remove and grants no additional
visibility or ordering beyond FR-437. `memory flush` remains a `tso` class
and is catalogued when a source `parallel` selects `tso`; it removes the
oldest buffered write for one thread. Other memory entries
are admitted only when the source-default model defines their named step
(FR-434, FR-436, FR-437).
FR-432's closed class table determines the spellings and node associations.
Every concrete step class of that table is a catalog class, and so is every
abstract class: this includes the `effect` and `commit` classes (each the
`event` step kind of its node, spelled by the bare node path, `node` the effect
or commit node, `binder_types` its binder record) and the `cend` class of each
template (`cend <protocol>::<template>`, `node` the template node, no binder
types). A catalog without them omits a class and refuses as an omitted class
below; an abstract `protocol_step` target naming an `effect` or `commit` node
resolves against the abstract entry of that node. Further:
`fork` and `join` are two classes of one `parallel` node, and a memory class
has no node. The `activate` class of a protocol clause with activation
`on each` is a declared class entry whose `class` is the alias-free
spelling `activate <protocol>`, whose `node` is the protocol clause node
itself (a local `NodeRef` on the concrete side, the `RefinementKey` of the
selected `protocol` clause on the abstract side), and whose `binder_types`
is the trigger binder record of FR-432's "The binder record of an abstract
node": the trigger's declared type `T` when `T` is not a record type, else
one key per field of `T` in the effective field order of FR-432. The trigger
is neither a separate catalog node nor a clause of its own: the class
associates with the clause that declares the trigger, and `binder_types`
alone carries the trigger's type. A clause with activation `on origin` has
no `activate` class and no entry. The concrete catalog covers the selected concrete model's
operation classes and all protocol clauses in the declaring package whose
identity-bound `owner_model` edge names the object-type node of a model owned
by that selected concrete declaration, with their source memory
defaults. The edge is on each protocol-clause node, names the object-type
`NodeRef` of a model that a selected declaration owns (never an unselected
declaration's node, and never the selected concrete model or state-machine
node itself), is part of its exact
`dependencies`, and is not inferred from the `over` parameter's type. Every
such clause contributes its classes even when the refinement has no step
rows; a clause whose edge names a model owned by another declaration is
foreign and contributes none.
The concrete catalog is therefore an authority independent of `semantic_type`
for a protocol subject: a refinement whose `semantic_type` names another
selected model carries a catalog the reader derives differently
(FR-375-AC-33). The checks run in this order: the presence check below,
then the row joins of step 5, then the FR-464 step 7 comparison, within which
a class the concrete identity does not derive is an extra class
(`invalid_semantic_graph`) and is decided before a class whose node alone is
foreign or mismatched (`invalid_model_binding`/`malformed-declaration`).
Extra and omitted classes are located by walking the declared classes of the
written catalog and of the derived class set together in catalog-key order (class UTF-8 bytes): the first
discrepancy decides, an extra class refusing at its entry and an omitted class
at the `concrete` array, where declared classes are the entries other than `memory` entries (`memory
drain`, `memory flush`), which are reconstructed from admitted `parallel`
bodies and compared separately, so a catalog `[Zed::Run]` against a derived
`[Other::Go]` refuses at the array and `[Aaa::Run]` at entry 0.
The abstract catalog covers the selected `protocol` clause when present,
or the abstract model's operation classes otherwise. It excludes classes
of another model or an unselected dependency.

QSL SHALL derive the catalog from its checked subjects and binder records,
independently of the emitted step rows. The reader SHALL compare every
`steps[i].source_class` and `source_node` to one concrete entry, and every
`protocol_step` target's `class`, optional `node`, argument count and
argument types to one abstract entry. An empty `arguments` array leaves
every binder open as FR-432's omitted argument list does; a nonempty array
has one entry per binder. A memory class has no node or binder record.
A missing class refuses `missing_declaration`/`missing-name` at
the class member; a row node id unequal to its catalog entry refuses
`invalid_model_binding`/`malformed-declaration` at that row key.
A nonempty target argument count unequal to its catalog binder count
refuses `ill_typed`/`operator-ineligible`, and an argument of another type
refuses `ill_typed`/`type-mismatch`, at that target argument as FR-432 states.
The refinement step uses only already checked node ids, declarations,
types, dependency selections, canonical catalog order and the catalog's
own row joins. It does not treat protocol control bodies as admitted.
At FR-464 protocol body step 7, the reader SHALL compare the complete
catalog with classes reconstructed from the selected model declarations
and admitted protocol graph under FR-432 and FR-464 to FR-466; source-default
memory classes are reconstructed from admitted `parallel` bodies and
FR-434/FR-436/FR-437. A wrong-kind or wrong-subject catalog node or a binder
signature that disagrees with the admitted subject refuses
`invalid_model_binding`/`malformed-declaration` at the catalog entry.
An omitted or extra catalog class refuses `invalid_semantic_graph` at the
first offending entry, or at that catalog array for an omission, located by
the tie-break of the paragraph above that begins "Extra and omitted classes"
(FR-375-AC-33); duplicate
or misordered entries refuse during refinement canonical order. This is a
producer attestation and wire/graph consistency check, not a second parse
of QSL source. A later analysis request may override a parallel's memory
model; its resolved memory classes belong to that request's obligation
identity and are outside checked-package admission.

The catalog is part of the refinement node's body. A changed class, node,
binder type or source-default memory model changes the node's value and the
obligation identity. Every local catalog
`node` and `binder_types` key enters the refinement's exact local
`dependencies`; external `{package, node}` keys do not. A present catalog on
a model-only refinement, or an absent catalog when a protocol or memory
class participates, refuses `invalid_semantic_graph` at
`/semantic_graph/nodes/{n}/body/class_catalog` after the dedicated key check
and before row joins.

A protocol subject whose resolved internal list is
non-empty also carries `abstract_internal`, the resolved classes as
**alias-free spellings** (a class's FR-432 spelling with `<A>::` removed, such
as `Twice::finish`, `lose Twice::ch`, `activate Twice`, `memory flush`) in
ascending UTF-8 byte order; the abstract model and the protocol clause are
identified by `abstract` and `protocol`, so the entry does not depend on the
local abstract alias, and renaming the alias, with the list renamed to
match, leaves the node id unchanged. It is absent for an empty list, as
`protocol` is absent for a model subject, and never null. The `steps` array
is in **declaration order**: the order in which the declaration lists its step
rows, for a model subject and a protocol subject alike. It is not sorted by
node id, class spelling or any other key, so reordering step rows in source
reorders `steps`. Every other array
of the body (`populations`, `objects`, `histories`, `assume`,
`ensure`) is sorted ascending by the node id of its first named declaration, then by
member name as UTF-8 bytes; within an `objects` entry the field rows are sorted
ascending by field name as UTF-8 bytes. The node is part of the checked package, so the internal list is part of
the obligation identity of every item that cites it through
`abstract_internal`, and needs no member of its own.

For every emitted `steps[i]`, the producer SHALL retain the exact
`SourceRegion` of its original checked source row and write one
`refinement_step_rows` sidecar entry keyed by the refinement node id and
zero-based step position `i` in declaration order (FR-322). It writes these entries for
proof-eligible and ineligible refinements alike. The sidecar is outside the
refinement body; source
reordering therefore changes row regions without changing the body.
The reader checks the sidecar against the retained body step count and, for
proof-eligible owners, against generated `source_map` occurrences. This is a
closed wire consistency check, not a second parse of the QSL source.

**Claim kinds.** A request to check the whole declaration is one item of
kind `temporal-satisfaction`; each per-step simulation obligation is one item
of kind `operation-contract` ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)).

## Reader order

The refinement step is a body step of
[FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
"Body steps": it runs after the abstraction step and before the terminal
step. It visits every `relation`/`refinement` node in ascending
node-id order. Schema, graph-shape checks (including the exact local
`dependencies` list), declaration, frame, state, and the earlier body steps
retain FR-322's precedence. For one
refinement node, the reader checks in this order and reports its first defect:

1. **Identity.** `owner`, `declaration` and `recursion_group` are absent on
   the node; a document carrying one fails the closed V2 node shape as
   `malformed_wire` before this step. Two refinement
   nodes of one package whose bodies carry an equal `name` refuse
   `ambiguous_declaration`/`ambiguous-name` at the later node's `body/name`
   in ascending node-id order.
2. **Canonical order.** Check each sorted array and nested array in the order
   `populations`, `objects` (each entry's `fields` first),
   `histories` (each entry's `updates` first), `assume`, `ensure`, and then
   `abstract_internal` and `class_catalog` (`concrete`, then `abstract`) when
   present. `steps` is in declaration order. Apply the ordering rules above, with catalog entries sorted
   by their catalog key (`class` UTF-8 bytes, then `memory_model` UTF-8 bytes
   for memory entries);
   the first out-of-order entry refuses `invalid_semantic_graph` at its
   array entry. A duplicate first key or tied row that violates the same
   total order refuses at the later entry.
3. **Header joins.** Resolve `semantic_type` to the unique local model root
   whose owner joins the selected concrete model package. The `identity` of
   that root's `model_root` owner is the refinement's **concrete identity**;
   `semantic_type` is the only header member that carries it, and the wire has
   no other concrete-selection field (FR-375-AC-25). FR-375 requires no
   distinctness from the `abstract` root's identity (FR-375-AC-34). Then resolve `abstract` to
   the selected abstract model root,
   then optional `protocol` to a protocol clause of that abstract model,
   and `profile` to the selected infinite-trace profile. Resolve a local
   `NodeRef` only in this graph, and a `{package, node}` key only in its named
   directly selected dependency's admitted package. A missing local or
   external key refuses `missing_declaration`/`missing-name` at that key;
   a package absent from direct `dependency_selections` refuses
   `missing_declaration`/`missing-selection` at its `package` member. A
   present key of the wrong declaration kind, a protocol of a different
   abstract model, or a profile of the wrong kind refuses
   `invalid_model_binding`/`malformed-declaration` at that member.
4. **Class catalog.** After the header joins, check presence and the closed
   catalog entries against already admitted node ids, declaration kinds,
   type keys and direct dependency selections. Check `concrete` before
   `abstract`, in class order, and each entry's node before its binder type
   keys. This does not inspect protocol body layout, membership, control
   associations, binder records or source memory defaults: FR-464's later
   protocol step checks those after admitting the protocol graph.
5. **Rows.** In the same array and nested-array order as step 2, with `steps`
   visited between `objects` and `histories` in declaration order, resolve
   each local and external declaration key under the rule above, and check
   its role and type against the concrete and abstract subjects and the
   entry shape specified in the row table. A missing target refuses
   `missing_declaration`/`missing-name` at that key; a present target of the
   wrong kind or subject refuses `invalid_model_binding`/
   `malformed-declaration` at that key. A **concrete key** is a local `NodeRef`
   of `populations[].concrete`, `objects[].concrete_type` (an object type or a
   `state_machine`), `histories[].concrete_type`,
   `histories[].updates[].operation`, `assume[].operation` or
   `steps[].source_node`. For a declaration or an anchor (an anchor takes the
   owner of its `context` declaration) the subject is correct only when its
   `ModelOwner` `identity` is the concrete identity, so a key of the right
   kind owned by another selected model is a wrong subject. For
   `steps[].source_node` this is the owner rule of "Step-row provenance"
   above, not a second rule. The rows are checked in the array order of step
   2 (`steps` in declaration order), then by index, then by member. Protocol-body nodes (channel, role,
   `parallel`, template, effect, commit) carry no `ModelOwner`, and a clause's
   `owner_model` edge is typed only at FR-464 step 6, so this step does not
   own them: they and every `class_catalog` node are compared at FR-464
   step 7, below;
   an expression whose checked result
   type (the referenced node's `semantic_type`, or the inline term's
   declared `type` or `result_type`) does not conform to its row's abstract target type
   refuses `ill_typed`/`type-mismatch` at the expression. An `open` term
   outside a step target's `arguments` fails the V2 schema before this step.
   A row expression that reads a history field, a whole population or
   `seq` is a node form FR-322 "Refinement row expressions" defines; a
   history read is joined to the refinement's `histories` here, before the
   operation stage.

The step does not recompile source or infer an alternate key from a row's
source position. An admitted wire target is checked against the selected
declarations and the graph; its body remains the exact
content the reader checks. The retained `dependencies` list was checked
before this step and cannot substitute for a missing body key.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-375-AC-1 | A declaration with one population row per abstract population, one well-typed object row per abstract object type admitted by those populations and per abstract `state_machine` field set, one step row per concrete operation and well-typed history and fairness rows is admitted and emitted as one `relation`/`refinement` node whose body carries every row. A model with no `state_machine` declaration remains a valid subject when its object-type rows are valid. | Test (TC-310) |
| FR-375-AC-2 | A concrete operation with no step row refuses `missing_declaration`/`missing-name` naming the operation, and the declaration is not admitted with any default target; the same declaration with `step … -> any;` written for it is admitted. | Test (TC-310) |
| FR-375-AC-3 | An abstract population with no row refuses `missing_declaration`/`missing-name`; a second row for one population, a field named twice, a second step row for one operation, a second history update for one operation and two fairness rows of one set with the same kind, operation and granularity each refuse `invalid_model_binding`/`conflicting-binding` naming both rows. | Test (TC-310) |
| FR-375-AC-4 | A field-row or step-argument expression whose type does not conform to the abstract type after reference lifting refuses `ill_typed`/`type-mismatch` at that expression; a `Reference<T_C>` expression for a `Reference<T_A>` field is admitted exactly when an object row maps `T_A` from `T_C`. | Test (TC-310) |
| FR-375-AC-5 | A history row whose name the concrete type already declares, whose type is unbounded, or whose update is ill-typed refuses with its named cause; an admitted history field appears in no concrete clause, frame or successor relation. | Test (TC-310) |
| FR-375-AC-6 | A fairness row with no granularity is read as `whole`; an `assume` row naming an abstract operation, or an `ensure` row naming a concrete operation, refuses `missing_declaration`/`missing-name`. | Test (TC-310) |
| FR-375-AC-7 | Changing any row of an admitted declaration changes the refinement node's value and the obligation identity of an item that cites it; reordering rows other than step rows in source does not (reordering step rows reorders `steps` and changes the value); changing only the refinement's authored name changes the node id and the obligation identity (FR-375-AC-29). Each row-empty refinement node retains the local `abstract` root key in `dependencies` despite its empty rows. | Test (TC-310) |
| FR-375-AC-8 | A request to check the whole declaration requires `temporal-satisfaction`, each per-step simulation obligation requires `operation-contract`, and neither requires `refinement`. | Test (TC-310) |
| FR-375-AC-9 | A history row with an initial `history-literal` written as a negative or non-negative integer, `true`/`false`, `none`, a string, an exact number or a full-path enum member is admitted when its value is in the declared bounded type's value set; a literal of the right kind outside the value set or of the wrong kind (`5` or `true` for `Int[0, 4]`, `none` for a non-`Option` type) refuses `ill_typed`/`type-mismatch` at the literal; an unresolved enum path refuses `missing_declaration`/`missing-name`; a collection, record or float spelling refuses `invalid_syntax`/`unexpected-token`. | Test (TC-310) |
| FR-375-AC-10 | An `assume` or `ensure` fairness row written `fair strong` is admitted with or without `each` or `whole`, and its emitted entry carries kind `strong`; the same row written `fair weak` carries kind `weak`, and the two rows give different refinement node ids; `fair` with no kind, or any kind other than `weak` or `strong`, refuses `invalid_syntax`/`unexpected-token` at the token, because a refinement row has no default kind. | Test (TC-310) |
| FR-375-AC-11 | `abstract Spec` selects the whole abstract model through its canonical local or dependency-package model-root key; `abstract Spec::Twice` and `abstract Spec::Twice2`, two protocol clauses of the abstract package with identical operation-target rows, are each admitted, with the model subject and two protocol subjects distinguished by their root/clause keys and protocol subjects carrying `protocol`; a clause of the declaring package is listed among local `dependencies`, while a clause of a directly selected dependency package joins through `dependency_selections` and is absent from local `dependencies`. `abstract Spec::Thrice` (no such clause), `abstract Spec::Counter` (a type), `abstract Spec::counters` (a population) and `abstract Nope::Twice` (no such alias) refuse `missing_declaration`/`missing-name` at the unresolved segment (`Thrice`, `Counter`, `counters`, `Nope`); a clause of the abstract package that acts only on another model refuses `invalid_model_binding`/`malformed-declaration` at the name. Syntax refusals are `invalid_syntax`/`unexpected-token` at these tokens: `abstract Spec::Twice::finish concrete Impl` at the second `::`; `abstract ::Twice` at the `::`; `abstract Spec:: concrete Impl` at `Impl` (`concrete` is read as the name); `abstract Spec:: {` at the `{`; `abstract Spec::1` at the `1`. | Test (TC-310) |
| FR-375-AC-12 | The `refinement_internals` input is the only host of the internal list: `abstract Spec::Twice` with no row naming internals and the unit value `[{"refinement": "R", "internal": ["Spec::Twice::finish"]}]` is admitted with `abstract_internal` `["Twice::finish"]`; the same source with the value absent, `[]` or naming another refinement is admitted with no `abstract_internal` entry. A supplied library declaring its own refinement and given its own value is admitted with that value affecting only the library's node and the importer's selection of it; a value naming a refinement the library does not declare refuses `missing_declaration`/`missing-name` at (that library's value, `/0/refinement`) and the compilation, the unit included, is not admitted. | Test (TC-310) |
| FR-375-AC-13 | The two lists `["Spec::Twice::finish"]` and none give refinement nodes with different bodies and obligation identities for an item citing the node; reordering the value's objects and entries changes none of them; renaming the abstract alias from `Spec` to `S`, with the value's spellings renamed to match, changes none of them. | Test (TC-310) |
| FR-375-AC-14 | For each feature a producer's support set omits, a declaration using it refuses `unsupported_construct`/`not-yet-implemented` as the Unbuilt features paragraph states: two `history` rows give two refusals at their keywords naming `history-row`; `abstract Spec::Twice` one at `Twice` naming `protocol-abstract-side`; a non-empty resolved list one at `Spec` naming `refinement-internal-list`, both together giving two refusals for the one declaration; each names the refinement and the profile identity and produces no node. A list with a resolution defect (`missing-name`, `conflicting-binding`, malformed) reports that defect and no support-set refusal for the list, and a value attributable to no refinement reports its defect alone. A producer that has built a feature is checked by that feature's own criteria, and this criterion places no test on it. | Test (TC-310) |
| FR-375-AC-15 | A field row of type `Sequence<Reference<T_C>>[0, 2]` for an abstract `Sequence<Reference<T_A>>[0, 2]` field, and the row `Some(self.ring)` for an abstract `?` field of type `Reference<T_A>`, each use `nested-reference-row`: where the support set omits it, each refuses `unsupported_construct`/`not-yet-implemented` naming the feature at the row's expression, once per such row, and no node is produced; a whole-population reference row and a plain `Reference<T_C>` row for a `Reference<T_A>` field are admitted; a `Sequence<Reference<T_C>>` step argument for a `Sequence<Reference<T_A>>` parameter refuses `ill_typed`/`type-mismatch`. | Test (TC-310) |
| FR-375-AC-16 | A refinement whose abstract model declares a relationship refuses `unsupported_construct`/`declaration-form` at the abstract alias of the header, once per refinement, naming the form, the refinement and the selected profile identity, whatever the producer's support set, and produces no node. | Test (TC-310) |
| FR-375-AC-17 | `Some(e)` for `e` of type `E` is typed `Option<E>`; over an abstract `?` field of type `T`, the row `none` leaves the field absent and `Some(v)` sets it, and a row that is not `Option<T>`, including a bare `self.f` of a concrete `?` field, refuses `ill_typed`/`type-mismatch`; over an abstract `Option<T>` field the row's option value is taken unchanged; `none` with no field type or other `if` arm refuses `ill_typed`/`type-mismatch`; outside a refinement declaration `Some` is an ordinary identifier, so `Some(1)` in a state clause refuses `missing_declaration`/`missing-name` at `Some` when no function `Some` is declared and is a call to it when one is. | Test (TC-310) |
| FR-375-AC-18 | In a refinement governed by `quire.temporal.infinite-trace/v1`, the ring-buffer rows `s.ring = self`, `(self.head + i) mod 2` and an `a div b` row are admitted even when the unit's own declarations select no layer, the lock selection counts the rows' layers (the `always` rows and exactly one integer-division law, `selection_trigger_unsatisfied` when none is selected), and a Boolean field row `ready = protocol Fill` naming a local protocol refuses `unsupported_construct`/`expression-form` at the `protocol` token naming `quire.protocol.complete/v1`, the layer that admits the form and that the rows' layer set lacks. | Test (TC-310) |
| FR-375-AC-19 | Withdrawn: the sort of `steps` by node id, member name and alias-free class spelling this criterion tested is removed; `steps` is in declaration order. | Withdrawn |
| FR-375-AC-20 | Withdrawn: the content-hash identity (package id, node key, digest or stale comparison) this criterion tested is removed. | Withdrawn |
| FR-375-AC-21 | The reader visits refinement nodes in ascending node-id order and applies canonical order (of the sorted arrays, never `steps`), header joins and row joins after the identity check; an out-of-order entry of a sorted array refuses `invalid_semantic_graph` at the first offending entry, an absent local `abstract` key refuses `missing_declaration`/`missing-name`, an absent direct dependency for an external `abstract` key refuses `missing_declaration`/`missing-selection`, and a wrong-kind local `semantic_type` or object-row type key refuses `invalid_model_binding`/`malformed-declaration`, each at the member this section names. Object rows join either two object types admitted by mapped populations or two `state_machine` declarations and reject mixed or unrelated declaration kinds. | Test (TC-310) |
| FR-375-AC-22 | For each step row the producer emits one `refinement_step_rows` entry with the source region of that original checked row, associated with the refinement owner id and the zero-based position of that row in declaration order; reversing the source order of the step rows reverses `steps` and the region assignments but keeps the owner id. | Test (TC-310) |
| FR-375-AC-23 | A protocol refinement carries a complete, canonical `class_catalog` of its source-level concrete and abstract classes and checked binder types in both V2 schemas. A tied `fork`/`join` pair names one parallel node under two distinct class spellings; a class of another selected model or protocol, a changed class/node association, a missing or duplicated class, a wrong binder record, or a memory kind absent from its source-default model refuses under the catalog and row-join rules. The concrete catalog includes classes from every clause whose identity-bound `owner_model` is the object-type node of a model owned by the selected concrete declaration, including when no step rows name those classes, and excludes clauses whose edge names a model owned by another declaration. For each of `sc`, `tso` and `ra` used as a source default in that subject, `memory drain` is recorded; `memory flush` is recorded when `tso` is a source default. Each `memory drain` entry carries `drain: store-buffer-flush`, whose SC behavior is a no-op. A later request's memory override changes no checked-package class fact. | Test (TC-310, TC-233) |
| FR-375-AC-24 | A `state_machine` field set is represented by an existing `objects[]` row whose `abstract_type` and `concrete_type` keys resolve to `state_machine` declarations and whose field names resolve on the abstract machine; the row keeps the existing closed shape and requires no population row. A model with only object types and populations is admitted using its ordinary object rows without a `state_machine` anchor. Missing or duplicate abstract machine rows refuse as the corresponding missing or conflicting object mapping; an object row that mixes a `state_machine` declaration and an object type refuses `invalid_model_binding`/`malformed-declaration`. The reader admits the same rows by joining their keys to the selected packages; no additional wire member is present in either V2 schema. | Test (TC-310) |
| FR-375-AC-25 | For a selected model with any number of object types and state machines, including none, the producer derives exactly one identity-bound model root from the selected package identity; it does not choose a declaration as a representative. Model-subject `abstract` keys and refinement `semantic_type` keys name that root, and protocol-subject `abstract` keys name that root while `protocol` names the exact clause. Changing a source alias leaves the root identity unchanged; a model root names the selected package by its identity. The model root is derived from existing selection state and requires no FCD declaration or new selection field. | Test (TC-310) |
| FR-375-AC-26 | A catalog is unique and ordered by catalog key. Over a subject whose source `parallel` controls default to `sc`, `tso` and `ra`, `concrete` holds `memory drain` three times, once per model, in the order `ra`, `sc`, `tso`, each with `drain: store-buffer-flush`, and `memory flush` once for `tso`, after the three drains; every such catalog is admitted. The same models selected by two `parallel` controls produce one entry per model, not two. Reordering the three drains, repeating a `(memory drain, memory_model)` pair, or repeating a declared class spelling refuses `invalid_semantic_graph` at the first out-of-order or later duplicate array entry. | Test (TC-310, TC-233) |
| FR-375-AC-27 | A concrete protocol clause with activation `on each (x: T)` carries one catalog entry `{class: "activate <protocol>", node: <clause node>, binder_types: [...]}` where `binder_types` is `[T]` for a non-record `T` and one key per field of `T`, in effective field order, for a record `T`; the abstract catalog carries the same entry with `RefinementKey`s for a selected abstract clause with that activation. A clause with activation `on origin` carries none. A catalog that associates `activate` with a node other than its clause, or lists a wrong or reordered binder type, refuses `invalid_model_binding`/`malformed-declaration` at the catalog entry in FR-464 step 7; one that drops the entry refuses `invalid_semantic_graph` at the catalog array, and one that adds an `origin` clause's entry refuses `invalid_semantic_graph` at that extra entry (the omitted or extra class rule above). | Test (TC-310) |
| FR-375-AC-28 | A protocol refinement's concrete and abstract catalogs carry one declared entry for each `effect` node and each `commit` node of the selected subject, with `class` the bare node path (the `event` step kind), `node` that node and `binder_types` its binder record, and one `cend <protocol>::<template>` entry for each template, with `node` the template node and empty `binder_types`, whether or not a step row names them; a protocol with none of these nodes carries none of these entries. Each of these independent mutations of a valid catalog refuses as stated: dropping the `effect`, the `commit` or the `cend` entry refuses `invalid_semantic_graph` at the `concrete` (or `abstract`) catalog array; repeating one of those entries refuses `invalid_semantic_graph` at the later duplicate entry; an entry of one of those classes for a node that is not such a node (a `cend` of an attempt, or a path naming no node) refuses `invalid_semantic_graph` at that entry; and associating the `commit` entry with the `effect` node, or changing a binder type, refuses `invalid_model_binding`/`malformed-declaration` at the entry. | Test (TC-310, TC-233) |
| FR-375-AC-29 | The refinement's authored `ident` is the body's `name`. Two declarations in one unit whose abstract model, protocol, profile, concrete model and rows are equal and whose names differ (`Alpha` and `Beta`) are admitted as two `relation`/`refinement` nodes with different node ids, each with its own declaration occurrence and source-map entry; the producer emits no internal failure for them. Renaming one refinement changes its node id and the obligation identity of an item citing it; renaming a model or profile alias changes none. A body without `name`, or with a non-`Identifier` `name`, fails both V2 schemas. | Test (TC-310) |
| FR-375-AC-30 | Two refinement declarations of one unit with the same name refuse `ambiguous_declaration`/`ambiguous-name` at the second declaration's name token, whatever their rows; a package whose two refinement nodes carry equal `body.name` refuses `ambiguous_declaration`/`ambiguous-name` at the later node's `body/name` in ascending node-id order. | Test (TC-310) |
| FR-375-AC-31 | For an operation class `T::op`, `source_node` is the no-point `state`/`operation_anchor` of (`T`, `op`) whose `context` is the declaring type `T`, with and without a `class_catalog`; a catalog entry for the class carries that anchor as `node`. A `source_node` naming the concrete model root, the model-owner node, an anchor of another operation or type, or a point-qualified anchor, or a missing `source_node` on an operation class, refuses `invalid_model_binding`/`malformed-declaration`; a `source_class` naming no operation that `T` itself declares (including an inherited operation spelled under the inheriting type), checked before `source_node`, refuses `missing_declaration`/`missing-name` at `source_class`. A no-point anchor no clause names carries one `anchor` occurrence, the operation's declaration region. An operation is the class of its declaring type only: an inherited operation is spelled and anchored at the type that declares it, and a class naming a type that does not declare the operation refuses `missing_declaration`/`missing-name`. The abstract operation target's `operation` key names the abstract declaring type's no-point anchor by the same rule. | Test (TC-310) |
| FR-375-AC-32 | A step row's `self` and concrete operation arguments are `reference` terms to FR-341 `value`/`parameter` nodes: `self` named `"self"` at level 0 with `Reference<C>`, then each parameter by declared name at levels 1..n in declared order; a step row has no `result` binder and an object row has `self` alone; the nodes are those of the operation's precondition clause. A reference to a node with a wrong name, level or type refuses `invalid_model_binding`/`malformed-declaration` at the reference. | Test (TC-310) |
| FR-375-AC-33 | The refinement's concrete identity is the `identity` of the `model_root` owner joined by its `semantic_type`. With two or more selected models, swapping only a valid refinement's `semantic_type` to the root of another selected model refuses, in this order: an absent concrete catalog on a protocol subject `invalid_semantic_graph` at `/body/class_catalog`; the first concrete key (array order of reader step 2) whose declaration or anchor owner is not the concrete identity `invalid_model_binding`/`malformed-declaration` at that key; then, at FR-464 step 7, `invalid_semantic_graph` at the first discrepancy between the catalog and the classes the new concrete identity derives, walked together in catalog-key order: at the entry for a class the identity does not derive, at the `class_catalog/concrete` array for a derived class the catalog lacks. The swap is undetectable only when the refinement has no concrete key and the swapped-to model derives an identical class set (every protocol subject with an empty concrete catalog over a model deriving no class, and every model subject, which carries no catalog): it is then admitted as a different node whose id differs by `semantic_type` (FR-375-AC-7). Protocol-body nodes are compared only at step 7. No `model_root` nodes or wire field is added. | Test (TC-310) |
| FR-375-AC-34 | A refinement whose `semantic_type` root is the same node as its `abstract` model root is admitted when every concrete key and the concrete catalog agree with that one identity as in FR-375-AC-33; FR-375 imposes no distinctness between the two. | Test (TC-310) |
| FR-375-AC-35 | The index segments of an external `{package, node}` key resolve against the graph of the dependency package its `package` names, never the importing graph: mutation 13's package with an index segment `{"node": N}` appended to `body.abstract.node.path`, `N` the length of the selected dependency's `semantic_graph.nodes`, passes the V2 schema and refuses `malformed_wire`, with no cause, at `/semantic_graph/nodes/{n}/body/abstract/node/path/{s}/node`, `s` the appended segment's position, even when `N` is below the length of the importing graph (`RW-09`). | Test (TC-310) |

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
