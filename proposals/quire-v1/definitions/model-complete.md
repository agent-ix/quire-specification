# Complete model definition

Definition identity: `quire.model.complete/v1`.
Language: `ix:native` / `1-draft`. This definition is the complete-V1 model,
inheritance, conformance, dispatch, systems-model and closed-environment root.
Implementation and qualification remain separately reported.

## Always-selected definitions

Every package that selects this root also selects the complete-value root and
its always-selected closure ([package selection](value-complete.md#package-selection)), and:

| Identity | Definition artifact |
| --- | --- |
| `quire.state.graph/v1` | [Finite graphs](state-graph.md) |
| `quire.model.complete.rules/v1` | [Normalization rules](model-complete-rules.json) |

The accounting role is `quire.value.accounting/v1`, already selected by the
complete-value root. Its model and graph charge points and its
`ModelNormalizationLimitsV1` object are the only accounting for this root. The
QSL `quire.state.evaluation-work/1` counter set is not selected: a package
selecting this root is evaluated and checked only under
`quire.value.accounting/v1`, and no caller limit is clamped to a default.
Diagnostics are interpreted by `quire.native.diagnostics/v1`; that catalog is selected by the response, not by this
root.

A package that selects this root has dynamic dispatch, inheritance and
redefinition as this definition states. Every other FS03 state-contract rule
applies unchanged.

## Model input

The model is declared in the spec's own artifacts (AD-006). One artifact
declares one domain object, such as `Order`, and its module object type, such
as `entity`, fixes the artifact's shape. quire-rs extracts the declarations.
The FCD extraction-frontend lifts the extracted declarations of one spec
package into one semantic IR document, the **domain package**. It is
serialized with RFC 8785 JCS.

A package imports a domain package with a `model` declaration:

```text
model Orders = "acme/orders";
```

The source names the domain package identity only: no version and no digest
(FR-154 "Model selection in source"). The resolver records the import as a
ModelSelection (FR-321) row of the package lock: `{identity}`. QSL model intake admits the domain package and builds the
Quire model declarations from its IR nodes. Quire owns the meaning of every
declaration. Clauses of a unit name a declaration by qualified name: the model alias,
then the segments of its IR node identity after the package identity, such as
`Orders::Order` for IR node `ix://acme/orders/Order`. A clause that a domain package itself carries replaces the model alias with the self alias of its owner and needs no import (FR-470). One form is an
exception: a case of a variant type is named `Alias::Tier::Gold`, the alias, the
variant type's artifact id and the case name matched byte for byte, and not by
its IR identity `variant/Tier-Gold`. In a type position `Alias::Tier` names the
variant type. A variant type name that resolves to no variant type of the alias
is `missing_declaration`/`missing-name`, and a case the type does not declare is
`ill_typed`/`type-mismatch`. FR-208 "Naming a variant type and its cases" states
the rule.

### Intake

Rule `quire.model.normalize.intake/v1` admits one domain package. The package
input is a map from domain package identity to package bytes, one document per identity. Intake first checks the selection's identity, then looks up the
bytes by that identity and reads the package's own identity. It runs these checks in order. Each failure ends intake with no declaration.

| Check | Refusal |
| --- | --- |
| The selection's domain package identity equals `quire/native` or `quire/unit`, or begins `quire/native/` or `quire/unit/` (a reserved identity) | `invalid_model_binding`/`malformed-declaration` |
| The package input supplies bytes under the selection's identity | `missing_import`/`missing-selection` with the ModelSelection |
| The package's own identity equals the selection's | `invalid_model_binding`/`wrong-model-selection` with both |

Intake then reads each IR node and builds one original declaration per node.
Nodes are read ascending by declaration key. A node that is not a valid IR
node of its kind, or a type definition or population node whose artifact id
is not an object id (defined under the declaration identity rule below), is `invalid_model_binding`/`malformed-declaration`. The
refusal retains the IR node identity and the node's FCD `origin` verbatim. A supertype, field type, subsets, redefines, relationship-end,
frame or population member reference that names no node of the package is
`missing_declaration`/`missing-name` with the referring node's `origin`. Intake
reports every such refusal in node order, and then no later phase runs.
Within one node, every failing check reports in FR-154's table order, the object
id check first. A reference that names a refused node is not reported again as
`missing_declaration`/`missing-name`.

A node's `origin` is the FCD origin object (`common.schema.json` `$defs.origin`): a
source origin (`sourceIdentity`, `path`, `startLine`, `startColumn` and optional
`endLine`, `endColumn`) or a generated origin (`generatorIdentity`,
`generatorVersion`, `inputIdentities`, no span). It appears in diagnostics and
provenance verbatim and is never a key member; the artifact id enters a key only
as the type's identity segment below.

A model-declaring artifact is an artifact whose type is an object type of any
spec-objects module (spec-objects-architecture, spec-objects-business, …), and
its artifact id is an object id: an ASCII letter followed by zero or more
ASCII letters, digits, underscores or hyphens
(`^[A-Za-z][A-Za-z0-9_-]*$`). An object id is carried verbatim in its type
identity; it is distinct from a `member-name`. Source-language qualified-name
spelling is governed by the language grammar and is not part of this intake
identity rule. Artifact-type ids
such as `FR-100` declare no model node, and neither does a composed document,
an artifact-type document that also carries `object:` (such as `FR-204` with
`object: value_object`): extraction classifies a document by its `type` alone
and skips an artifact-type document even if it carries `object:`, so `object:`
never lifts a document or names a model node (FR-154-AC-30). A model-declaring
object artifact may use the same id spelling, including `FR-204`; intake
examines emitted IR nodes, not source documents. FCD extraction selects source
documents by document `type`, so `type: FR` emits no model node regardless of
its id or `object:` value. For an emitted `TypeDefinition` or population node,
intake reads the final identity segment as its artifact id, applies FR-154's
object-id rule, and resolves its `{module, name}` kind through the package
`constructs` table to obtain its meaning under FR-208. It does not classify
the emitted node using the source document's `type`, `object:` or `id`.

A declaration's identity is its artifact id. Every type definition is declared
by its own artifact: an object type, value type, variant type, population,
interface, part, port, connection or allocation declared by the artifact with
id `Order` in domain package `acme/orders` has IR node identity
`ix://acme/orders/Order`, and a port declared by the artifact with id
`pump_out` has `ix://acme/orders/pump_out`. A member's identity appends its
member name to its owner's identity: field `total` of `Order` is
`ix://acme/orders/Order/total`. The members are fields, operations, clauses, states, transitions and steps, each identified by
`<owner identity>/<member name>`, and relationships and the cases of a
variant type, each identified by its own node
(`ix://acme/orders/relationship/<Name>-<verb>-<TargetName>`,
`ix://acme/orders/variant/<Name>-<value>`; FR-154). The owner of a part or port and the ends of a connection
or allocation are references, not members, and never part of an identity. The
artifact title is the declaration's
`displayName`, used for presentation only; it is never part of an identity and
never resolves a qualified name. Retitling an artifact keeps every declaration
key. Changing an artifact's id declares a different type with a different
declaration key, and renaming a member declares a different member.

### Construct meanings

The domain package is a semantic IR 2.0.0 document. Each type definition's
`kind` is `{module, name}` and names one entry of the document's embedded
`constructs` table. Intake binds each type definition by that entry's `meaning`,
a Quire meaning id of [FR-208](../../../spec/objects/foundation/FR-208-quire-meaning-vocabulary.md).
A population node names its construct the same way. The kind name and module
select the construct entry only and never select a meaning. A type definition
or population node whose `kind` names no construct entry, or whose construct
has no `meaning` or a `meaning` outside FR-208, is
`invalid_model_binding`/`malformed-declaration` with its IR node identity
and `origin`.

### Declaration kinds

| Declaration | IR node | Owner |
| --- | --- | --- |
| Object type | `TypeDefinition` whose construct meaning is `quire.meaning.model.object-type/v1` | none |
| Value type | `TypeDefinition` whose construct meaning is `quire.meaning.model.value-type/v1` | none |
| Variant type | `TypeDefinition` whose construct meaning is `quire.meaning.model.variant-type/v1` | none |
| Record value type, event type, state machine, process, persistence interface, namespace | the `TypeDefinition` whose construct meaning is `quire.meaning.model.record-value-type/v1`, `quire.meaning.model.event-type/v1`, `quire.meaning.model.state-machine/v1`, `quire.meaning.model.process/v1`, `quire.meaning.model.persistence-interface/v1` or `quire.meaning.model.namespace/v1`, valid as FR-208 defines that meaning | none |
| Variant case | the `variant` node of a variant type's case, `ix://<package identity>/variant/<Name>-<value>` | its variant type |
| Field | `TypeDefinition.fields[]` entry | its type |
| Relationship | `TypeDefinition.relationships[]` entry | its type |
| Operation | `Operation` | its receiver type |
| Invariant | `Clause` with `language: quire` | its type |
| Population | Population node whose construct meaning is `quire.meaning.model.population/v1` | none |
| Part, Port, Interface, Connection, Allocation | the `TypeDefinition` whose construct meaning is that kind's `quire.meaning.systems.*` id | none; the `owner` and end references place it in the effective view as below |

A clause in any other language is kept with its span. It is never evaluated
and never a proof obligation.

## Model feature rules

Rule `quire.model.features/v1` checks each declaration after intake. Each row
is checked for every declaration of its kind, ascending by declaration key.
Every refusal names the IR node identity and its FCD `origin`.

| Feature | Rule | Refusal |
| --- | --- | --- |
| Identity | A type definition's identity, including a population's and a systems construct's, is `ix://<package identity>/<artifact id>`. A field, an operation, an operation parameter, a clause, a state, a transition and a step nest under their owner (`<owner identity>/<member name>`, `<operation identity>/<param>`); a relationship and every other node kind take their own global slot, as FR-154 states from FCD FR-095, departing from it for the four nested kinds (`relationship/<Name>-<verb>-<TargetName>`). The artifact title is `displayName`, for presentation only, and the declaration's `origin` is not part of any identity. Two nodes with one identity are not allowed. A native value type has no node and no identity of a package: it is named `ix://quire/native/<Name>` (Fields row), and the domain package identities `quire/native` and `quire/unit`, and any identity beginning `quire/native/` or `quire/unit/`, are reserved, so a selection naming one is refused by the first Intake check. | `invalid_model_binding`/`conflicting-binding`; `invalid_model_binding`/`malformed-declaration` for a selection naming a reserved identity |
| Supertypes | Every `supertypes[]` entry of an object type names an object type of the package, and every entry of a record value type, event type, state machine or process names a node of its own meaning (FR-208); an entry naming a node of another meaning is refused. The supertype graph is acyclic; phase 3 checks it. A persistence interface or namespace with a non-empty `supertypes` is refused and its entries are not checked further. | `missing_declaration`/`missing-name`; `invalid_model_binding`/`malformed-declaration`; `unsupported_construct`/`declaration-form`; `invalid_model_binding`/`specialization-cycle` in phase 3 |
| Abstract | An abstract type has no direct instances. A population member whose most-specific type is abstract is refused. Dispatch covers only concrete subtypes. | `invalid_runtime_input`/`abstract-instance` |
| Fields | `typeRef` names a native value type, or a type of the package; a `typeRef` naming an event type, state machine, process, persistence interface or namespace (FR-208) is refused. A native value type is named `ix://quire/native/<Name>`, `<Name>` exactly one of `Boolean`, `Integer`, `Rational`, `Decimal`, `Float32`, `Float64`, `Text`, `String`, `UUID` and `Timestamp`, compared byte for byte; the reference declares no node and carries no parameters, and intake admits `Rational`, `Decimal` and `Text` like any other native name, their use being decided at member resolution (FR-322). A unit of the checked package is named `ix://quire/unit/<name>` and is admitted without being resolved (FR-208). A type of the package is named `ix://<package identity>/<artifact id>`, so it never collides with a native of the same name. Every `typeRef` takes one of these three forms; any other form, and a native `<Name>` outside the set, is refused. A field typed by a native value type declares no type node of its own. Parameters and results follow this row. `multiplicity` has `lower <= upper`. | `missing_declaration`/`missing-name`; `invalid_model_binding`/`malformed-declaration`; `invalid_model_binding`/`unpreserved-model-meaning` |
| Presence | `presence` is exactly `required` or `optional`. A `required` field is present in every object. An `optional` field may be absent. Absent and empty stay distinct: lower bound `0` makes an empty collection legal and never makes a field optional. | `invalid_model_binding`/`malformed-declaration`; `invalid_runtime_input`/`missing-member` |
| Subsets | Every `subsets[]` entry names a field of the owning type or of a supertype. | `missing_declaration`/`missing-name` |
| Redefines | `redefines` names a field or operation of the package. Phase 4 checks that the target is inherited by the owning type and that no two members of one type redefine one target. | `missing_declaration`/`missing-name`; `invalid_model_binding`/`redefinition-target` in phase 4 |
| Relationships | Each end names an object type or a process (FR-208) of the package, and a relationship may be owned by an object type or a process, with object-type relationship meaning; an end naming a node of any other business model meaning (FR-208), and a relationship owned by one, is refused. Each end has a role and a multiplicity with `lower <= upper`. `direction` is exactly `source-to-target`, `target-to-source`, `bidirectional` or `undirected`. | `missing_declaration`/`missing-name`; `invalid_model_binding`/`malformed-declaration`; `invalid_model_binding`/`unpreserved-model-meaning` |
| Operation parameters | Each parameter has a `name`, a string matching `^[A-Za-z][A-Za-z0-9_]*$` and equal to the last segment of its identity `<operation identity>/<param>`; an absent, non-string, empty or otherwise mismatching `name` is refused at the selection row before the operation's frame is read, except on a parameter that shares its identity with another of the operation, which is refused once as a repeated identity before any name is checked; equal names under one operation are equal identities. `deletes_parameters` and every by-name parameter reference resolve against this `name`. | `invalid_model_binding`/`malformed-declaration`; `invalid_model_binding`/`conflicting-binding` |
| Frames | `Operation.frame` is a closed object whose members are the following, and no other. `modifies`, `creates` and `deletes` are required arrays of node identity strings, each present and empty when empty; they carry the type-wide and every-object entries, `modifies f` being the every-object scope of FR-013. Three members are optional, and an absent one is empty or false: `modifies_receiver`, an array of field node identity strings, the receiver-scoped `modifies self.f` entries; `deletes_receiver`, a boolean, `true` for `deletes self`; `deletes_parameters`, an array of parameter-name strings, the `deletes p` entries for reference parameters. An absent required member, a member this row does not name, a member of the wrong JSON type and an array element that is not a string are `invalid_model_binding`/`malformed-declaration` at the member. The member shapes are checked first, then the entries in the order `modifies`, `modifies_receiver`, `creates`, `deletes`, `deletes_parameters`, each in document order, and the first failure is reported. Each `modifies` entry names a field or relationship. Each `modifies_receiver` entry names a field, own or inherited, of the operation's declaring object type: an entry naming no node of the package is `missing_declaration`/`missing-name`; an entry naming a relationship, an object type or a field of a type the declaring object type does not conform to is `invalid_model_binding`/`malformed-declaration`. Each `creates` or `deletes` entry names an object type or a process (FR-208); an entry naming a node of any other meaning is refused. Each `deletes_parameters` name is a parameter of the operation whose type is `Reference<T>`; any other name is `missing_declaration`/`missing-name`. `deletes_receiver: true` is admitted for every operation of an object type. The receiver of an operation of a process or a state machine is a run or the machine context, not an object of an object type, so on such an operation a non-empty `modifies_receiver` and `deletes_receiver: true` are `invalid_model_binding`/`malformed-declaration` at that member, checked once the member shapes hold and before any entry, `modifies_receiver` first; an empty `modifies_receiver`, `deletes_receiver: false` and `deletes_parameters` read as for an object type. Once every entry resolves, a `modifies` entry and a `modifies_receiver` entry for one field are `invalid_model_binding`/`conflicting-binding` naming both entries and the operation, located at the later of the two in document order, and intake never merges them; a field written twice with one scope, and a scoped `deletes` written twice, are one entry. The members lift to the checked-package frame body as FR-340 states. | `missing_declaration`/`missing-name`; `invalid_model_binding`/`malformed-declaration`; `invalid_model_binding`/`conflicting-binding` |
| Populations | Each member type names an object type or a process (FR-208); a member type naming a node of any other business model meaning (FR-208), such as a namespace, is refused. `extent` is exactly `closed` or `open`. | `missing_declaration`/`missing-name`; `invalid_model_binding`/`malformed-declaration` |
| Business model meanings | Each record value type, event type, state machine, process, persistence interface and namespace node meets its FR-208 rules: required members present, identity fields absent or present as its meaning states, `occurrenceField` resolving to `Timestamp`, transition, step, `persists` and `members` references naming nodes of the stated meaning, guard clauses in `quire`, unique state, transition and step identities, and no unsupported form. A process has an object universe of runs; no node of the other five meanings has one. Invariants, `Pre:` and `Post:` contracts and generalization have the meanings FR-208 states. | `missing_declaration`/`missing-name`; `invalid_model_binding`/`malformed-declaration`; `invalid_model_binding`/`conflicting-binding`; `unsupported_construct`/`declaration-form`; `wrong_snapshot`/`forbidden-pre-read`; `invalid_model_binding`/`derivation-conflict` in phase 3 |
| Systems kinds | Each part, port, interface, connection and allocation node meets the systems-model kind rules below. | as below |

Multiplicity is `{lower, upper, ordered, unique}`. `lower` is an integer in
`0..=u64::MAX`. `upper` is such an integer or `unbounded`, which is greater
than every finite value. `ordered` and `unique` are booleans.

## Effective identities

An original declaration key is `{package, node}`: the domain package identity
and the IR node identity. Declaration keys compare by `package`, then `node`,
as UTF-8 bytes, a proper prefix first.

QSL derives effective declarations. Every identity below is structural: a
value compared componentwise, never hashed, never encoded into a digest.

| Name | Shape | Identifies |
| --- | --- | --- |
| Effective declaration identity | `{owner_effective_type, original}` | One effective declaration: an effective type (`owner_effective_type` is `null`) or an effective member of an effective type (`owner_effective_type` is that type's own effective declaration identity). `original` is the original declaration key. |
| Effective view (`quire.model.effective-view/v2`) | `{model_selection, rules, declarations}` | One complete effective view of one ModelSelection under one rule manifest. `declarations` is the array of every effective declaration with its `derivation`, ascending by effective declaration identity. The label names this value shape only. |
| Object universe (`quire.model.object-universe/v2`) | `{model_selection, root_types}` | The object universe shared by a connected generalization component: `root_types` is the ascending array of the effective type identities in that component that have no supertype. The label names this value shape only. |

Effective declaration identities compare by `owner_effective_type` (`null`
first, then recursively by identity), then `original`. An effective
declaration identity binds declaration keys only: no ModelSelection, no
derivation fact and no digest is part of it. Two domain package documents with
equal nodes yield equal effective declaration identities, effective views and
object universes. Every use that must separate selections pairs the effective
declaration identity with the selected domain package identity or with an
object universe.

Every phase-1 original declaration and its FCD origin remains available from
the immutable ModelSelection declaration inventory. That inventory and the
effective declarations are different sets: a scalar, a population, a namespace, a clause
in a language other than `quire` have no
phase-2 qualify rule and remain originals, without an invented qualify fact or
effective declaration. An operation's `Pre:` and `Post:` contract clause is not a
declaration: it is not retained and is never qualified. Effective entries and their fact input
keys reference only that retained inventory. Retention adds no derivation or
effective-declaration charge and changes no effective identity.

After structural normalization, the accounting definition orders systems
checks, source resolution and structural assembly, checked-fact admission,
the complete exhaustive conformance stage, then dispatch. Checked-fact
admission uses one existing checker graph and its actual authored clauses;
it is a process stage, not an extra normalization rule or derivation fact.
The rule manifest and effective identities are unchanged. A normalized
view is internal checking input; a validated package requires completion of
all these stages without refusal or incompleteness.

The effective member key is `(owner effective type identity, original
declaration key)`. Paths that reach the same original declaration collapse to
one effective member that links to each direct parent that reaches it. Two
different original declarations never collapse, whatever their names or
shapes.

### Chain-link derivation

An effective declaration links to the effective declarations of its direct
parents. Its derivation binds exactly its own facts and those links; it never
contains an ancestor path longer than one edge.

`derivation` is the ordered array of the declaration's own facts. Each fact is
`{ordinal, rule, inputs, parent}`: `rule` is `{identity}` from the manifest's
`normalization_rules`; `inputs` is the ordered array of declaration keys the
fact consumed; `ordinal` is the fact's zero-based decimal-string position in
the array; and `parent` is `null` or the effective declaration identity of
the one parent the fact links to. A fact links to its parent only
when its rule is `quire.model.normalize.inherit/v2`; every other fact has
`parent` `null`. Facts are ordered by phase, then by `inputs` compared element
by element in declaration key order, a proper prefix first, then by the rule
identity's exact UTF-8 bytes. Thus equal-input phase-4
`quire.model.normalize.redefine/v2` facts precede
`quire.model.normalize.subset/v2` facts. This is serialization order, not
semantic override or conflict precedence.

For a type `T`, with `parents(T)` its direct supertypes in ascending
declaration key order, the facts are:

1. phase 2 `qualify`, inputs `[T]`, `parent` `null`; then
2. phase 3 `inherit/v2`, inputs `[P]`, `parent` the identity of `P`, for each
   `P` in `parents(T)`.

For the effective member `(T, m)` of the original declaration `m`, owned by
`A`, which exists when `T` is `A` or `A` is a proper ancestor of `T`, the
identity's `owner_effective_type` is the identity of `T`, its `original` is
`m`, and the facts are:

1. phase 2 `qualify`, inputs `[m]`, `parent` `null`, when `T` is `A`;
2. phase 3 `inherit/v2`, inputs `[P, m]`, `parent` the identity of `(P, m)`,
   for each `P` in `parents(T)` for which `(P, m)` exists, ascending; none when
   `T` is `A`;
3. phase 4 `subset/v2`, inputs `[m, t]`, `parent` `null`, for each `t` in
   `m.subsets`, ascending `t`;
4. phase 4 `redefine/v2`, inputs `[m, t]`, `parent` `null`, when `m` redefines
   `t`; and
5. phase 4 `redefine/v2`, inputs `[r, m]`, `parent` `null`, for each member `r`
   that redefines `m` and whose owner is `T` or an ancestor of `T`, ascending
   `r`.

No fact repeats, and two inherit facts of one declaration never share a parent.
A subset or redefine fact states a relation between original declarations; the
path from `T` to an owner is the sequence of parent links, one hop per fact.
A diamond stores one inherit fact per direct edge that reaches the owner, never
one per path, so the number of retained facts depends on the number of types
and direct edges, not on the number of ancestor paths. A hidden declaration (a
redefined or losing member) keeps its identity and stays in the view in
identity order. A redefinition chain gives each member constantly many phase-4
facts (at most four).

Types are placed in topological order, parents before children; members
follow their phase-4 facts, in topological order of their parents. A type not
placed by that pass lies on or below a supertype cycle: it gets no effective
declaration and the cycle refusals below report it.

**Expansion.** The path-form derivation, with every distinct inheritance path
and the same facts, order and ordinals as the earlier path-form fact array, is
the deterministic expansion of the retained facts through the parent
links. A reader derives it on demand under its own limits; no charge is made
for an expanded path. An inherit fact expands
to one fact per distinct path from its declaration to the owner, in ascending
declaration-key depth-first order. A phase-4 fact expands to one fact per
redefiner or subsets target carrying the least path from `T` to the owner of
that declaration, whether one or several paths reach it. Where exactly one path
reaches each owner the expansion equals the path-form array element for
element. Refusals that list derivation paths (`derivation-conflict`,
`redefinition-target`) list expanded paths.

**Link validation.** A strict reader validates every link: the linked
declaration is of the fact's kind (a type for a type fact; a member with the
named supertype as owner and the fact's last input as original for a member
fact), and its original equals the fact's last input. A link of the wrong kind,
owner or original is refused. The recursion ends because the supertype graph is
acyclic.

The effective view's `declarations` is the array of `{identity, derivation}`
for every effective declaration, ascending by `identity`. Replaying
normalization from the same domain package and rule manifest reproduces the
effective view by structural equality. An I04 checked package carries it
as a `correspondence` node with semantic form `model_correspondence`.

### Object universe

The object universe of an object type `T` is the connected component of `T` in
the supertype graph restricted to object types. Its identity is the structural
`{model_selection, root_types}` of the table above. A population is a
membership view inside a universe, never a universe. Its declaration key never
contributes to a universe identity. Every type that conforms to `T` has `T`'s
universe. Components are disjoint, so the least root type names its component:
on the wire a universe is spelled as the checked-package node id (FR-322) of the
declaration node of its least root type, and an effective type as the node id of
its declaration node.

### Reference identity key

A `Reference<T>` value's FR-204 triple is `(universe, type, object)`:

- `universe` is the object universe identity of the referenced object's type;
- `type` is the effective type identity of the object's most-specific type,
  the runtime member's declaration key mapped through the effective view,
  never the static type `T`;
- `object` is the exact UTF-8 bytes of the member's object identity, with no
  parsing, trimming or normalization (FR-035).

The canonical key compares `universe`, then `type`, then `object`, as FR-144
states: `universe` and `type` are structural identities compared componentwise
as above, with each string compared as unsigned UTF-8 bytes and an array
element by element, and `object` is compared by its exact UTF-8 bytes. Every
byte comparison is unsigned bytewise lexicographic, with a proper prefix
first. `Reference<T>` is a static type only: an upcast never changes the
triple.

## Normalization rules

Each manifest `normalization_rules` entry names a rule identity,
`stage` (`normalization`, `conformance`, `dispatch`, `systems` or
`environment`), `phase` (an integer for the `normalization` stage, otherwise
`null`) and the artifact that defines it. Normalization applies the manifest's
`normalization_rules` in phase order. No rule declares an override relation.
Every refusal lists each contributing declaration key and derivation path in
effective-member order.

| Phase | Rule | Derivation |
| --- | --- | --- |
| 1 | `quire.model.normalize.intake/v1` | Admit the domain package and build one original declaration per IR node, then apply `quire.model.features/v1`. No derivation fact. |
| 2 | `quire.model.normalize.qualify/v1` | One fact per original declaration and owner, with inputs `[declaration]`: an object type, a record value type, an event type, a state machine, a process, a persistence interface, a variant type or an interface (owner `null`); a variant type's case, owned by its variant type and keyed by the case node's own declaration key, never the `variant` export key; a field, relationship or operation, owned by its type, interface or persistence interface; an invariant clause in language `quire`, owned by its type; a part, owned by its owning type; a port, owned by the owning type of its part; and a connection or allocation, owned by the end-owner type of each end, source end first. |
| 3 | `quire.model.normalize.inherit/v2` | For each type `T` and each direct supertype `P` of `T`: one fact on `T` with inputs `[P]`, linked to the identity of `P`; then, for each member `m` of `P`'s effective members, one fact on the effective member `(T, m)` with inputs `[P, m]`, linked to the identity of `(P, m)`. |
| 4 | `quire.model.normalize.subset/v2` | For each field `s` whose `subsets[]` names `t`, where the owner of `s` is `T` or a supertype of `T`: one fact on `(T, s)` with inputs `[s, t]`, no link. |
| 4 | `quire.model.normalize.redefine/v2` | For each member `r` whose `redefines` names `t`, where the owner of `r` is `T` or a supertype of `T`: one fact on `(T, r)` with inputs `[r, t]` and one on `(T, t)` with inputs `[r, t]`, no link. The redefined member stays in the view and is hidden from name resolution, lookup and dispatch candidacy. When several non-conflicting members redefine one member reaching `T`, only the redefining member of the most derived owner is exposed; every other redefining member keeps its facts and is hidden in `T` the same way. |
| 5 | `quire.model.normalize.canonicalize/v1` | Place every effective declaration, parents before children, and form the effective view. No derivation fact; ordering never changes a declaration key or resolves ambiguity. |

A record value type, an event type, a state machine and a process are
qualified like an object type: one owner-less fact, then the phase 3 inherit
facts for each direct supertype, with their fields, operations and `quire`
invariant clauses as members owned by the type. A process also has an object
universe of runs; no other kind named here does. A persistence interface
declares no supertype (a non-empty `supertypes` is refused), so it has a
qualify fact and its operations are its members, with no inherit fact. A
`quire` invariant clause owned by a type is a member of that type, qualified
and inherited like a field. A variant type has no inherit fact and no object
universe. A variant type's `supertypes` list is admitted only when it is
acyclic and every entry names another variant type; it is then ignored, so the
type's declarations are those it has without the list. An entry that names no
type, names a type of another kind, or closes a cycle is refused
`unsupported_construct`/`declaration-form` at `supertypes`. Reordering its
`variants` list changes no effective identity, entry or position, and its
effective identity depends on declaration keys alone.

A phase-3 or phase-4 fact states one edge or one relation between original
declarations. The full path of a derivation is not stored; it is the expansion
the chain-link derivation above defines.

The *element owner* of a declaration is: the type itself for an object type or
an interface; the owning type for a field, relationship or operation; the type
a part's `owner` names; and, for a port, the element owner of the part its
`owner` names. The *end-owner type* of
a connection or allocation end is the element owner of the declaration that
the end names. A relationship end is navigable from effective type `T` exactly
when `T` conforms to the type of the end whose role `name` is, ownership of the
relationship being irrelevant; `r.name` over `Reference<T>` navigates from that
end to the destination end when `name` is the role of the end nearest `T` (the source end's role is the forward
member and the target end's role the inverse member). The effective
declaration of a Connection or Allocation, as returned by a binder request, is
the effective member owned by the end-owner type of its source end. Every
systems effective member is keyed by `(owner effective type, construct
declaration key)`; a clause names the construct by its artifact id, never
through its owner.

Phase 3 charges and checks each direct supertype edge once, however many paths
reach its target. A type that the topological pass above does not place lies on
or below a cycle, and only those types are explored by path. A phase 3 extension
of that exploration that would revisit a type already on the current path
closes a cycle. A cycle is identified by the set of its supertype edges, so
two directed cycles over the same nodes with different edges are two cycles.
Every declaration on a cycle is `refused { code: invalid_model_binding, cause:
specialization-cycle }` once per cycle it lies on, at its own declaration: a
declaration on two cycles is refused twice, once listing each cycle. A cycle is
detected at the first extension, in `normalize.cycle-check` charge order, that
closes it, and a later extension closing a cycle with the same edge set is
charged but not reported again. The refusals follow cycle detection order, and
within one cycle declaration-key order. Each refusal lists the cycle's type
keys in cycle order (path order from the revisited type), rotated to start at
the least key. A redefinition whose target is not a member inherited by its owning type,
or two members of one type that redefine one target, is
`refused { code: invalid_model_binding, cause: redefinition-target }` listing
every such redefining member. Two phase 4 redefinitions of one member reaching `T`
conflict unless one redefining owner is a proper descendant of every other
redefining owner; a conflict is `refused { code: invalid_model_binding, cause:
derivation-conflict }` with both derivation paths, and no effective member is
exposed for either.

Phase 3 also derives the business model data of FR-208. A state machine's
effective states and transitions are the union of its own with those of every
ancestor; a state name, or a transition's `from`, `to` and `trigger`, declared
by two distinct owners reaching one type is `refused { code:
invalid_model_binding, cause: derivation-conflict }` with both derivation
paths, and a transition whose `from`, `to`, `trigger` or `guard` names no
effective state, operation or clause of its machine is `refused { code:
invalid_model_binding, cause: malformed-declaration }`. An event type whose
ancestor's occurrence field is neither its own occurrence field nor a field
that its occurrence field redefines is `refused { code: invalid_model_binding,
cause: derivation-conflict }` with both paths. States, transitions, occurrence
fields and steps add no `normalize.inherit` member fact, because they are not
members; a process's steps are not inherited.

## Systems-model kind mapping

Rule `quire.model.systems.kind-mapping/v1` maps each IR node whose construct
meaning is a systems meaning to exactly one systems kind by that meaning id,
never by its kind name or module. A node whose construct meaning is not a
systems meaning has no systems kind, even when its kind name is `port`. Kinds
are disjoint.

| Kind | Construct meaning | Owner and type |
| --- | --- | --- |
| Part | `quire.meaning.systems.part/v1` | owning composite = the effective owning type; declared type; typed multiplicity |
| Port | `quire.meaning.systems.port/v1`, whose owner is a Part | owning Part; direction `in`, `out` or `inout`; interface type, which must be an Interface; typed multiplicity |
| Interface | `quire.meaning.systems.interface/v1` | ordered feature signatures, each a field or an operation |
| Connection | `quire.meaning.systems.connection/v1`, whose two ends both name Ports | effective member owned by the source end-owner type; source and target Port keys; declared `direction` |
| Allocation | `quire.meaning.systems.allocation/v1` | effective member owned by the source end-owner type; source element, which must be a Part, a Port or an operation; target element, which must be a Part |

The rule maps interfaces, then parts, then ports, then connections, then
allocations, each group ascending by declaration key, so a port's kind follows
its part's and its interface's, and a connection's follows its ends'. It reports, for each node, every applicable
refusal: a port whose owner is not a Part is `refused { code:
invalid_model_binding, cause: wrong-export }` with required kind `Part` and
actual kind `none` or the owner's kind, and maps to no kind; a port whose
interface type is not an Interface is `wrong-export` with required kind
`Interface`; and a connection one of whose ends names a node that is not a Port
is `wrong-export` with required kind `Port` and actual kind `none`, once per
such end, source end first, and maps to no kind. A relationship between object
types is a navigation relationship with no systems kind. Rule
`quire.model.systems.allocation/v1` then refuses an Allocation whose target
element is not a Part with `wrong-export`, required kind `Part` and actual
kind `none` or the element's kind.

Direction is read only from a connection's or relationship's declared
`direction` and from a port's direction, never from a role.

## Package selection

This root requires the complete-value root. Its package selection places every
role in `always`: `root`, `state_graph`, `rule_manifest`,
`rule_ad_006`,
`rule_fr_043`, `rule_fr_143`, `rule_fr_144`, `rule_fr_150`, `rule_fr_151`,
`rule_fr_152`, `rule_fr_153`, `rule_fr_154`, `rule_fr_204`, `rule_fr_208`,
`rule_fr_321` and `rule_interface_003`; this root has no conditional or
alternative profile. A package selection is checked with the same closed
refusal codes and order as the complete-value package selection.

## Selected rules

Each rule below is a `rule_*` role of the package selection.

| Rule |
| --- |
| [Model, graph and state architecture](../../../spec/assurance/AD-006-model-graph-state-contracts.md) |
| [Finite graph identity and reachability](../../../spec/functional/foundation/FR-043-evaluate-finite-graph-relations.md) |
| [Records, tuples and object references](../../../spec/functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md) |
| [Collection algebra and reference keys](../../../spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md) |
| [Normalization provenance](../../../spec/functional/type-model/FR-150-preserve-model-normalization-provenance.md) |
| [Conformance, redefinition and dispatch](../../../spec/functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md) |
| [Systems-model binding and navigation](../../../spec/functional/type-model/FR-152-bind-systems-model-structures.md) |
| [Closed model environments](../../../spec/functional/type-model/FR-153-query-closed-model-environments.md) |
| [Domain package model intake](../../../spec/functional/type-model/FR-154-admit-domain-package-model.md) |
| [Declared object identity](../../../spec/objects/foundation/FR-204-declared-object-identity.md) |
| [Quire meaning id vocabulary](../../../spec/objects/foundation/FR-208-quire-meaning-vocabulary.md) |
| [Model-selection artifact](../../../spec/objects/interfaces/FR-321-model-selection-artifact.md) |
| [I03 model selection](../../../spec/objects/interfaces/interface_003-model-selection.md) |
