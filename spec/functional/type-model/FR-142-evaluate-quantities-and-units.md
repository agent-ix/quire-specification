---
id: FR-142
title: "Evaluate dimensions, quantities and exact unit conversions"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-044
    type: references
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
  - target: ix://agent-ix/quire-specification/FR-140
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# FR-142: Evaluate dimensions, quantities and exact unit conversions

## Description

When checking a quantity expression, the semantic kernel SHALL derive its
dimension and unit from declared identities and exact conversion functions.

## Inputs

Dimension vectors, unit identities, exact scale/offset conversions, quantities
and operations.

## Outputs

A completed typed quantity with any explicit conversion loss, or an undefined,
refused or incomplete evaluator outcome. I13 dispositions are not evaluator
outputs.

## Behavior

Addition/comparison require compatible dimensions and an explicit common-unit
conversion. Multiplication/division combine dimension exponents. Affine units
use their selected offset rules and admit conversion/comparison only; complete
V1 has no implicit point-versus-difference type, so arithmetic on a nonzero-
offset quantity refuses. Negation of a non-affine quantity is exact and
unit-preserving. Conversion never occurs implicitly.

## Dimension and conversion algebra

A dimension is a finite sorted map from I04 semantic-node base-dimension
identity to signed integer exponent; zero exponents are removed. Multiplication
adds exponents, division subtracts them and integer power multiplies them.
Quantities compare or add only when these normalized maps are equal.

The unit declarations for one I04 semantic-node dimension identity form a closed directed
graph with exactly one canonical root. The root has no target, scale one and
offset zero. Every other unit names exactly one target unit under the same
semantic-node dimension; each edge means
`target_value = scale × source_value + offset`, where scale is a nonzero exact
rational and offset is an exact rational. The graph is acyclic and every unit
reaches the root. Composing the edges yields
each unit's exact mapping to the canonical unit; conversion applies the source
mapping then the inverse target mapping.

Dimension terms name any dimension of the package, declared before or after
the dimension that names them; unit targets name any unit. When the checker
admits a package's unit and dimension declarations, the checker SHALL refuse a
topology defect before any quantity value exists, with these FR-272 causes;
each refusal names the dimension node and the unit or units shown. The checker
SHALL process the dimension nodes in turn-list order (below) and, for each node in turn:
(-1) check its terms, then (0) for a base dimension with no unit apply the
base-dimension rule below; otherwise (a) check each of its local units in
source order against rows 1 to 3, (b) then check the node against rows 4 to 6,
and (c) report the first defect found. A node is therefore refused by the
earliest row that applies to its earliest local unit, and a later node is not
examined once an earlier one is refused.

Source order is the order of the package's source documents as the checked
request lists them, then the byte offset of the declaration within its
document; a unit's node id is not computable before its target chain is known
to be acyclic, so no key order is used.

The checker SHALL build the list of turns by walking the package's declarations
in source order. A local dimension takes its turn at its own declaration. For
every declaration, the dimension nodes it mentions are visited in mention
order: a local dimension's terms; a local unit's type, then the type of its
target. Each mentioned node is expanded once, depth-first through its own terms,
following an imported dimension's terms transitively, and a node that is not a
local dimension takes its turn the first time it is reached, before the
declaration that reached it and before the nodes reached through its terms. An
imported dimension's own definition and units are resolved by name and are not
re-checked, because their graph defects belong to their package; they count as
already admitted. At a node's turn rows 1 to 6 apply to the local units typed
by it, wherever they are declared, with the imported package's units of that
dimension as admitted members of the graph: a local targetless unit beside an
imported root is a second root (row 4, retaining the imported root and the
local unit), a local cycle is row 6, and a local zero scale is row 1. A local
unit targeting an imported unit of its own node reaches an admitted root, so it
adds no cycle and no targetless unit; a target in another node is row 3. Row 5
applies only to a local root of a node that has no imported root.

A dimension or unit an admitted domain package declares (FR-154) is an
imported node of this list: its package's graph defects were refused at
intake, so its definition, terms and units count as admitted and are not
re-checked, and a local unit typed by a package dimension is checked at that
node's turn with the package's units as admitted members. A package dimension
that a source declaration mentions takes its turn when it is first reached, as
any imported node does; every package dimension not yet reached takes its turn
after the last source declaration's turn, in the lock order of its selection
and then in its authored order within the package's `dimensions`. Within one
package, "earlier" means earlier in authored order. The base-dimension rule
applies to a package base dimension at its turn as to any other node, and a
refusal of a package node is located at its selection row
`/semantic_graph/nodes/<i>/owner`, naming the node's FCD identity.

Step (-1) runs at a local dimension's own turn, before step (0), over that
dimension's own terms only, in this order. First, a term naming no dimension is
refused `missing_declaration`/`missing-name`, retaining the dimension that
owns the term and the term; a term of another dimension reached through a
derived dimension is not examined, and that dimension's own turn reports it.
Second, when the dimension is itself a member of a dimension cycle, or has a
self-term (`dimension A = B; dimension B = A ^ 2;` for `A` and `B`;
`dimension D = D * L;` for `D`), it is refused `invalid_package`/`definition-cycle`,
retaining the term edges `dimension -> term` of that cycle in order from the
cycle's dimension declared first. A dimension that only reaches a cycle or an
unknown term through another dimension is not refused at its own turn for it:
the cycle's member or the term's owner reports it at that dimension's turn. Such
a dimension has no normalized map, so the base-dimension rule ignores it, and
its own unit rows still run at its turn. A term naming an imported dimension
uses that dimension's admitted normalized map.

| Row | Defect | Code and cause | Retains |
| --- | --- | --- | --- |
| 1 | a unit's scale is zero | `invalid_package`/`invalid-value` | the unit and its scale |
| 2 | a unit names a target that is no unit of the package | `missing_declaration`/`missing-name` | the unit and the target name |
| 3 | a unit's target belongs to another dimension node | `invalid_package`/`incompatible-definition` | the unit, its target and both dimension nodes |
| 4 | the dimension has two or more targetless units | `invalid_package`/`conflicting-definition` | the dimension and every targetless unit, in source order |
| 5 | the dimension's one targetless unit (its canonical root) has a scale other than one or an offset other than zero | `invalid_package`/`invalid-value` | the root, its scale and its offset |
| 6 | the target edges form a cycle (a unit targeting itself included) | `invalid_package`/`definition-cycle` | the cycle's dependency edges, `unit -> target`, in order from the cycle's least unit by source position; when the dimension has several cycles, the one that contains the least unit |

The base-dimension rule: a base dimension (declared with no terms, local or
imported) that has no unit in this package or in the package that declares it
has no canonical root for a compound result to rest on when a dimension
(local or imported) that has a local unit carries it in its normalized map. At
the base dimension's own turn the checker SHALL refuse it,
`missing_declaration`/`missing-name`, retaining the base dimension and the
first local unit, in source order, whose dimension has a nonzero exponent of it
in its normalized map; a derived dimension without a unit is not refused,
since only base dimensions anchor a compound result. Relating a base
dimension to another exempts it from nothing: an equivalence names a unit of
each member (see "Dimension equivalence"), so it never anchors a base
dimension that has none. For a nonempty dimension
no separate missing-root cause exists: a finite graph in which every unit
names a known target within the dimension is cyclic, so such a dimension is
refused by row 2, 3 or 6.

A derived dimension whose normalized map is empty, such as
`dimension N = L / L;`, is the dimensionless dimension: the checker SHALL
admit it. Its units form N's own graph under rows 1 to 6, and its empty map
names no base dimension, so it creates no base-dimension obligation.

The checker SHALL admit no unit mapping of a refused dimension. A refusal
refuses the package, as every other refusal of this requirement does, and the
`quire.checked-package/v2` strict reader refuses the same shapes, as an
already-built package, with `invalid_semantic_graph` (FR-322), not with these causes.

A unit is affine exactly when the offset of its composed mapping to the
canonical root is nonzero; a declared edge offset alone does not decide it, so a
zero-offset unit targeting an affine unit is affine and a unit whose nonzero
edge offsets compose to zero is not. An affine unit may be converted or compared
but cannot be added, subtracted, negated, multiplied, divided or raised to a power; such
arithmetic refuses as affine-unit arithmetic. A separately
declared zero-offset difference unit supports arithmetic; complete V1 does not
infer a point/difference kind from an operation. Conversion into an exact target
reports no loss; conversion requiring
decimal/finite-domain rounding follows FR-140 and records that loss.

Direct conversion between two declared I04 units is admitted only when both
units have the same dimension node identity; distinct dimension nodes with
equal base-dimension maps, such as torque and energy, do not admit it. When
either side is an evaluator-owned compound unit, conversion is admitted exactly
when the two normalized base-dimension maps are equal. This nominal rule guards
only direct declared-to-declared conversion: multiplication, division, integer
power and every compound unit operate on base-dimension maps and intentionally
carry no nominal dimension identity, so converting a declared quantity to a
compound unit and then to another declared unit with an equal base-dimension
map is two admitted explicit steps. Every declared canonical root of a
dimension is, by definition, coherent with the product of the canonical roots
of that dimension's base-dimension terms raised to their exponents, with scale
one and offset zero; no unit body carries a coherence scale. A compound
value is therefore the value in the declared side's canonical root.

Type checking decides the static quantity refusals before any evaluation or
charge, in this order, where the first match wins: incompatible dimensions
(unequal dimension nodes for direct declared-to-declared conversion, otherwise
unequal normalized base-dimension maps where an operation requires equal
dimensions); affine-unit arithmetic; and distinct units where an operation
requires the identical unit. Each returns `refused { code: ill_typed }`; this
order is informative only and has no observable outcome difference.
Addition, subtraction, equality and ordering (`==`, `!=`, `<`, `<=`, `>`,
`>=`) require the identical unit. After the operand identity reads, evaluation
decides the runtime conditions in this order, where the first match wins: a
zero divisor, then a zero base under a negative integer exponent; each returns
undefined before any edge charge.

Conversion traverses the full source-to-root path and then the full
root-to-target path, with no common-ancestor shortcut, and has no operation
event of its own; a compound side contributes no edge. Addition and subtraction
of the identical unit traverse no edge, and their operation event is computed
in that unit. Negation traverses no edge, as addition does, and its operation event is
computed in the operand's unit. Multiplication, division and integer power traverse the left and
then the right operand's root path before their operation event. Top-level quantity equality and
ordering expressions share one schedule: both operands are mapped along their root paths,
left then right, and compared by exact root value, so an affine unit or a
negative composed scale compares by root value. A quantity leaf inside an FR-149 composite
equality plan is compared by exact value in its identical unit and charges only
its `equality.pair`. Quantity comparison between
distinct units requires an explicit prior conversion.

Base-dimension and unit identities are I04 checked-semantic-graph structural
node ids `{package, path, ordinal}`. A base dimension's `package` is the
identity of the exact admitted source, DefinitionRef or selected domain package
that owns it (which must join that lock or model_root selection), and its
`path` is its declaration path; the body holds its terms. A unit's body holds
`dimension_node_id`, `target_unit_node_id`, `scale` and `offset`, with
normalized exact rationals. A dimension or unit an admitted
domain package declares (FR-154) has as `owner` its selection's `ModelOwner`
and as `node_id.path` `[node_tag, semantic_form, <FCD identity>]`
(`["scalar_type", "unit", "ix://acme/rpc/unit/ms"]`); it carries no `declaration` (FR-322), and its node id differs
from a source declaration of the same name, whose owner is a `SourceOwner`.
The strict I04 reader refuses a body that disagrees with its dimension or unit
references as `invalid_semantic_graph`. A dimension node is self-typed and depends
exactly on the base-dimension keys in its terms. A unit's `semantic_type` equals
`dimension_node_id`; its dependencies are exactly that dimension plus its
non-null target. Contradictory semantic fields refuse. Equal names, scales or dimension shapes do
not confer identity, while the same exact imported owner/declaration remains
stable across unrelated package edits. Dimension exponents are mathematical signed integers subject to
explicit operation limits; implementations cannot narrow them to a machine
integer. A conversion publishes no partial quantity until canonicalization,
target rounding and target-domain admission all succeed.

**Quantity negation.** The unary minus `-q` over a non-affine quantity `q` of
type `Quantity<u>` is exact and unit-preserving: the result has type
`Quantity<u>`, in the same unit `u` and not a root-unit conversion of it, and its
value is the exact rational negation of `q`'s value, so `-(2 m)` is `-2 m`,
`-(3 ms)` is `-3 ms` and `-(0 m)` is `0 m`. A quantity of a compound unit that a
multiplication or division produced is negated in that compound unit. Negation
has no zero divisor, no undefined condition and no range obligation, because a
quantity magnitude has no bound. The operand of an affine unit refuses
`ill_typed` at type checking before any charge, as every other arithmetic on an
affine unit does. The operation lowers to `quire.op.quantity.negate` (FR-322).

Multiplication, division and integer power first convert every non-affine
operand to its dimension's canonical root, perform exact value arithmetic and
normalize the resulting base-dimension exponents. The result unit is an
evaluator-owned value identity `terms`, the ascending list of admitted I04
canonical root-unit node ids with their nonzero mathematical integer exponents,
compared structurally. The empty list is the
dimensionless unit. This runtime value identity is not an I04 checked-semantic
node id, graph reference, declaration or source-mapped authority, and
evaluation does not insert it into the checked package. A compound unit that
a checked source type forms is a separate thing: the I04
`scalar_type`/`compound_unit` node of FR-322 "Derived type forms and
forwarding", identified by FR-322's structural node id, whose terms follow
this order. Neither identity is derived from the other. The value
identity's closed schema and vectors are selected
by `quire.value.complete/v1`. An explicit later conversion may select a
compatible declared I04 unit; multiplication never guesses one from a matching
name or scale. For an integer exponent, `x^0` is exactly one for every `x`,
including zero, and a zero base under a negative exponent is undefined.
All identity reads, traversed edges, exact rational operations, target-domain
checks and result retention use the named points in
`quire.value.accounting/v1`, in its unit schedule: operand identity reads, then
the runtime undefined conditions, then every edge charge, then the rational
events, then the target domain and result retention.

## Dimensions of different owners

Two dimension nodes are different dimensions until a dimension equivalence
(next section) relates them, so the unit of compilation's `Time`, the `Time`
of `acme/rpc` and the `Time` of `acme/flow` are three dimensions, each with
its own units, and no equal name, scale or shape makes them one by itself.

- The checker SHALL type a unit by the dimension node it names, and SHALL
  refuse `ill_typed` by the first-match order above when addition,
  comparison, direct conversion or a `Quantity<u>` position meets units of
  two dimension nodes that no dimension equivalence relates, whichever owners
  declare them.
- The checker SHALL form the normalized dimension map of a multiplication,
  division or integer power over base-dimension node ids, so the base
  dimensions of two owners are two entries until an equivalence relates
  them: with no equivalence, `Quantity<R::s>` divided by `Quantity<F::s>` has a
  map of `R::Time` with exponent 1 and `F::Time` with exponent -1 and is not
  dimensionless, while `Quantity<R::s>` divided by `Quantity<R::s>` is.
- The checker SHALL admit a derived dimension whose terms name dimensions of
  several owners (`dimension Speed = M::Length / Time;`), and the node of
  such a derived dimension is its own node, distinct from each term's.

## Dimension equivalence

A dimension equivalence relates two dimension nodes that name one physical
dimension under two owners and states how large the canonical root unit of one
is against the canonical root unit of the other, so that two domain packages
that each declare `Time` exchange time quantities through the unit of
compilation that admits both, whatever unit each chose as its root. The source
form is the shared grammar's `dimension-equivalence`,
`dimension R::Time ~ F::Time by 1 R::s = 1000 F::ms;`, whose two dimension
names resolve by the namespace rules above (a bare name reaches a dimension of
the unit of compilation, `M::Name` a dimension of a selected domain package).
Only a unit of compilation declares an equivalence: a domain package document
names only its own dimensions, so it relates two of them by declaring one, and
the unit of compilation relates any two nodes, including two of one package.

The `by` clause `by x U = y V` names a unit `U` of the first dimension and a
unit `V` of the second, each by its unit name (a bare name reaches a unit of
the unit of compilation, `M::name` a unit of a selected package), and two
positive exact numbers: `x` of `U` is the same quantity as `y` of `V`. The
checker reduces the statement to the **root ratio** of the pair. When `U` is
`a` canonical roots of the first dimension and `V` is `b` canonical roots of
the second (the composed scales of the units to their roots), one canonical
root of the first dimension is `(y * b) / (x * a)` canonical roots of the
second. In the example the root of `R::Time` is `s` (`a` is 1), so the ratio
is `1000 * b`: 1 when `acme/flow` declares `s` as its root and `ms` as
`1/1000 * s`, and 1000 when `acme/flow` declares `ms` as its root. Two roots
of unequal size therefore never become one by writing the pair: the declaration
carries the size of one root in the other.

The checker SHALL decide each equivalence declaration in these steps, the
first match winning, and each refusal is located at the declaration:

1. A dimension name that resolves to no dimension of the unit of compilation
   or of a selected package's document, and then a unit name that resolves to
   no unit visible to the unit of compilation, each refuses
   `missing_declaration`/`missing-name` at the name; a package whose model
   alias the unit of compilation does not declare is not visible, so `Z::Time`
   with no alias `Z` is this refusal.
2. Two dimension names that resolve to one node refuse
   `invalid_package`/`definition-cycle` retaining the edge `node -> node`.
3. A declaration of a pair of nodes (in either order) that another
   equivalence declaration of the unit of compilation already names refuses
   `invalid_package`/`conflicting-definition`, retaining both declarations,
   at the later one, whether the two declarations state the same root ratio or
   contradict each other.
4. A first unit that is not a unit of the first dimension node, or a second
   unit that is not a unit of the second, refuses
   `invalid_package`/`incompatible-definition`, retaining the unit and the
   dimension node it was named for.
5. A unit that is affine (its composed offset to its canonical root is
   nonzero) refuses `invalid_package`/`invalid-value`, retaining the unit and
   its composed offset; a number `x` or `y` that is not positive refuses
   `invalid_package`/`invalid-value`, retaining the declaration.
6. A declaration whose two nodes are not both base dimensions (declared with
   no terms) refuses `invalid_package`/`incompatible-definition`, retaining
   both nodes, when their normalized base-dimension maps differ, and when its
   root ratio differs from the ratio the base roots fix. The maps are formed
   with the base classes below, so `dimension R::Speed ~ F::Speed by 1 R::mps =
   1 F::mps;` is admitted when `R::Length ~ F::Length` and `R::Time ~
   F::Time` are declared and the two speeds are `Length / Time` of their
   package, and refuses when they differ. The root of a derived dimension is
   the product of its base roots at their exponents (above), so its ratio is
   fixed by the base pairs: each base dimension's root is a ratio of its class
   representative's root (1 for a base dimension in no pair), and the ratio of
   a derived pair is the product of those ratios over the first node's terms
   divided by the product over the second node's.

The relation is symmetric and transitive: declarations `A ~ B`, `B ~ C` and
`C ~ A` are admitted together exactly when the three root ratios, each taken
as written, multiply to one. The equivalence classes are the connected
components of the declared pairs. The base classes are the components of the
pairs whose two nodes are both base dimensions.

7. **Consistency.** The root ratios of base pairs around every cycle of pairs
   SHALL multiply to exactly one, a pair read against its written order
   counting as its reciprocal. The first declaration, in source order, that
   closes a cycle whose product is not one refuses
   `invalid_package`/`conflicting-definition`, retaining the declarations of
   that cycle in source order. Derived pairs need no cycle check: step 6 fixes
   each from the base pairs.

The checker SHALL decide steps 1 to 5 for base pairs and the step 7 check over
the base pairs before the dimension turns, the step 6 check of every other
pair after the last turn, and the pairs in source order within each phase.

**Effect.** Every effect below is derived from the declared pairs, with no
node rewritten:

- The least node id, by ascending node-id, of the base class's members is the
  base class's representative, and a normalized dimension map holds the
  representative in place of each member of its class. Every member of a pair
  has a unit (step 4 names one), so the base-dimension rule is decided by the
  dimensions' own units: an equivalence never anchors a base dimension that
  has none.
- The units of the dimension nodes in one equivalence class form one unit
  graph: the union of the members' graphs with the canonical root of the member
  holding the least dimension node id as the class root, and every other
  member's canonical root as a unit that targets the class root with offset
  zero and with the scale equal to the product of the root ratios along the
  declared pairs from that member to the class root (a pair taken from the
  member's side toward the class root counts as written or as its reciprocal),
  which step 7 makes the same along every path. The identification edge from
  one root to another therefore carries the root ratio of the two. Rows 1 to 6
  apply to each node's own graph, unchanged.
- Direct conversion between two declared units is admitted when their
  dimension nodes are equal or in one equivalence class, and traverses the
  full source-to-root path and root-to-target path of the class graph, so
  with `by 1 R::s = 1000 F::ms` over equal-sized roots `convert<Quantity<F::s>>(3 R::s)`
  is `3 F::s` and charges the one identification edge of the class graph, and
  a conversion across a class of three roots charges two. Over unequal roots
  (`acme/clock` with root `ms` and `by 1 R::s = 1000 G::ms`) the same
  conversion `convert<Quantity<G::ms>>(3 R::s)` is `3000 G::ms`, not
  `3 G::ms`.
- Addition, subtraction, equality and ordering still require the identical
  unit, as for any two units of one dimension (`1 m` and `100 cm`): `3 R::s +
  3 F::s` and `3 R::s = 3 F::s` refuse `ill_typed` as distinct units, and
  `convert<Quantity<R::s>>(3 F::s) + 3 R::s` is `6 R::s`. A `Quantity<R::s>`
  position meets a `Quantity<F::s>` operand with the same explicit conversion.
- Multiplication, division and integer power convert each operand to the
  canonical root of its class, so `3 R::s / 3 F::s` is dimensionless and
  `(3 R::s) * (2 F::s)` has the compound unit of `(3 R::s) * (2 R::s)`. The
  compound value is the value in the class root, so over unequal roots
  `3 R::s / 3 G::ms` is the dimensionless `1000`.
- A derived dimension converts to another dimension whose normalized map
  equals its own by the compound pivot of the conversion rule above whether
  or not an equivalence relates them; an equivalence of two nodes with equal
  maps (and the ratio step 6 fixes) additionally admits their units' direct
  conversion.

**The pivot expression.** The two-step conversion of `3 R::s` into `dsec`,
with `dimension Dur = R::Time; unit dsec : Dur = 1;` and no equivalence, is
the expression `let c = ((3 R::s) * (1 R::s)) / (1 R::s) in convert<Quantity<dsec>>(c)`, whose binding is step 1 (the quotient is `3` of the compound unit with the one term `R::s` at
exponent 1, the declared quantity converted to the compound value of its
canonical root) and whose body is step 2 (the compound
side's map `R::Time` 1 equals the map of `Dur`, so the conversion is admitted
and yields `3 dsec`). The direct `convert<Quantity<dsec>>(3 R::s)` refuses
`ill_typed` for two unequal dimension nodes.

## Quantity type reference

Complete-V1 source names the type of a quantity in a declared unit as
`Quantity<u>`, where `u` is a qualified unit name. `Quantity` is a reserved
word of the complete-V1 facets, so a declaration named `Quantity` is
`invalid_syntax`.

**Namespaces.** Dimensions, units and everything else a unit of compilation
declares are three namespaces. The third, the type namespace, holds records,
tuples, unions, enums, aliases, functions, predicates, protocols, claims and
declared values, which FR-322 carries as every declared node but a dimension
or a unit; a name is unique within its namespace in the
linked package, and two units of one name in the linked package refuse
`ambiguous_declaration`/`ambiguous-name` at the use. The unit namespace holds
the units the unit of compilation declares and the units of every admitted
domain package (FR-154). A package unit carries no declaration name: a
source names it through the selection's model alias, `M::ms`, matched against
the `name`s of the units of that selection's document, and a domain package
`typeRef` names it by its package identity and `name` (FR-322 "Model-owned
members" step 4), so a unit `ms` the source declares (`Quantity<ms>`) and a
unit `ms` the domain package `acme/rpc` declares (`Quantity<M::ms>`) are two
units and never one ambiguous name. A dimension's identity is its owner and its name. The dimension
namespace therefore holds each owner's dimensions apart: the dimensions the
unit of compilation declares, reached by their bare name, and, for each
admitted domain package (FR-154), the dimensions of its selected document,
reached as `M::Name`, where `M` is the model alias of a `model_root` node
and `Name` matches, byte for byte, the `name` of a dimension of that row's
document (FCD identity `ix://<package identity>/dimension/<Name>`). No bare name
reaches a package dimension, and no case folding, suffix match or search
across packages applies. A dimension name declared by the unit of compilation
and by an admitted domain package, or by two admitted domain packages, is not
a conflict: the declarations are distinct dimensions and refuse nothing. A
name that two dimensions of the unit of compilation declare refuses
`ambiguous_declaration`/`ambiguous-name` (FR-091), and two dimensions of one
domain package that carry one identity refuse at intake
(`invalid_model_binding`/`conflicting-binding`, FR-154). A source dimension
term or unit dimension may name a package dimension as `M::Name`. A bare name
that no dimension of the unit of compilation declares, a qualified name whose
first segment is no model alias, and a qualified name that no dimension of the
row's document carries each refuse `missing_declaration`/`missing-name` at the
name. A bare `type-ref` looks up
the type namespace only, and `Quantity<...>` is the one `type-ref` that looks up
the unit namespace. A source that writes a unit name where a type is required
(`d: ms`) therefore refuses `missing_declaration`/`missing-name` at the name,
unless a type of that name is declared, and then it means that type.

**Where the form is written.** `Quantity<u>` is a `type-ref`, so it appears in
every position that takes one: a function, predicate or operation parameter and
result, a record field, a tuple member, a union payload, a type alias target,
the element of `Option` and of the collection types, and the target of
`convert<...>(e)`. A conversion into a unit is written `convert<Quantity<u>>(e)`.

**Behavior.**

- The checker SHALL read `Quantity<u>` as the type of the quantities whose unit
  is `u`: the kernel quantities with an exact rational magnitude and the unit id
  of `u`.
- The checker SHALL resolve `Quantity<u>` to the unit's own nominal
  `scalar_type`/`unit` node, which is the type of a quantity in that unit
  (family `quantity`, FR-322). It SHALL introduce no node of its own, so the
  type's key is the unit's declaration key.
- The checker SHALL type a quantity literal `n u` as `Quantity<u>`.
- When an expression of type `Quantity<u>` stands where `Quantity<u>` is
  required, the checker SHALL accept it.
- When an expression of any other type stands where `Quantity<u>` is required,
  including a quantity in another unit of the same dimension, the checker SHALL
  refuse `ill_typed`/`type-mismatch` at the expression. Conversion between
  units is explicit only (see Behavior), never implied by a type annotation. A
  declared random-parameter support is the exception: FR-405-AC-3's
  `value-outside-type` at the value refuses a support that is not a
  `Quantity<u>` value.
- When the qualified name of `Quantity<u>` names no unit in the unit namespace,
  a dimension name included, the checker SHALL refuse
  `missing_declaration`/`missing-name` at the name.
- The checker SHALL admit under `Quantity<u>` every quantity of the unit and no
  bound on the magnitude; `Quantity<u>[min, max]` is not a source type form. A
  domain package's `min` or `max` constraint on a unit gives its members the
  bounded quantity type `Quantity<u>[min, max]`, a subtype of `Quantity<u>`
  (FR-154 "Unit bounds").
- The checker SHALL NOT admit `Quantity<D>` over a dimension `D` as a type
  form; a source that wants it declares a unit of `D` and writes `Quantity<u>`.
- The checker SHALL lower a quantity literal to a checked-package `literal` term
  of `value_kind` `quantity` typed by the unit's node.
- The checker SHALL read a unit `typeRef` of a domain-package member as FR-322
  "Model-owned members" step 4 says, so an operation parameter of a domain
  package has type `Quantity<u>`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-142-AC-1 | Exact compatible zero-offset conversion followed by arithmetic produces the declared result unit and value. | Test (TC-187) |
| FR-142-AC-2 | Incompatible dimensions, including distinct dimension nodes with equal base-dimension maps under direct conversion, then affine-unit arithmetic, then distinct units where the identical unit is required, refuse `ill_typed` at type checking before any charge. | Test (TC-187) |
| FR-142-AC-3 | A lossy conversion returns its exact loss classification or refuses when the target permits no loss. | Test (TC-187) |
| FR-142-AC-4 | Normalized dimension maps remove zero exponents and make multiplication/division inverse for admitted quantities. | Test (TC-187) |
| FR-142-AC-5 | Exact affine conversion composes through the unique canonical root; arithmetic on a unit with a nonzero composed offset refuses; quantity ordering compares root values. | Test (TC-187) |
| FR-142-AC-6 | Multiplication, division and integer power produce the exact value in the canonical compound-unit identity, including dimensionless cancellation, `0^0 = 1` and an undefined zero base under a negative exponent. | Test (TC-187) |
| FR-142-AC-7 | Exact-bound accounting succeeds and denial of a named next unit charge returns incomplete without a partial quantity or compound identity; every schedule boundary, including decimal-target rounding and membership, occurs at its defined charge position. | Test (TC-187) |
| FR-142-AC-8 | The published `quire.value.compound-unit/v1` schema and vectors fix the compound-unit value identity: each vector is schema-valid, and its terms name admitted root-unit node ids with nonzero exponents in strictly ascending node-id order; each authored noncanonical mutation, including a `-0` exponent, is refused by exactly its named schema or semantic check. | Test (TC-233) |
| FR-142-AC-9 | Equality or ordering of quantities in distinct units refuses `ill_typed` before any charge, while identical-unit equality and ordering compare exact root values after the left and then the right root path. | Test (TC-187) |
| FR-142-AC-10 | A conversion with a compound side compares normalized base-dimension maps, traverses only the declared side's root path, and admits a declared-to-compound-to-declared pivot in two explicit steps. | Test (TC-187) |
| FR-142-AC-11 | With the dimension `T`, the unit `s` (root of `T`) and the unit `ms` (target `s`, scale `1/1000`), a function `f(d: Quantity<ms>)` called as `f(3 ms)` admits the argument `3 ms` and refuses `f(3 s)` with `ill_typed`/`type-mismatch` at the argument; `d: Quantity<T>` (a dimension) and `d: Quantity<nothing>` each refuse `missing_declaration`/`missing-name` at the name, and a bare `d: ms` refuses `missing_declaration`/`missing-name` at `ms`. | Test (TC-187) |
| FR-142-AC-12 | A function `f(d: Quantity<ms>): Quantity<ms>` returning `d` checks; returning `convert<Quantity<s>>(d)` refuses `ill_typed`/`type-mismatch` at the returned expression; and `Quantity<ms>` annotations introduce no checked-package node, naming the node of `ms`. | Test (TC-187) |
| FR-142-AC-13 | A unit and a record of one name coexist, and a use of the name in a type position means the record while `Quantity<name>` means the unit; two units of one name in the linked package refuse `ambiguous_declaration`/`ambiguous-name` at `Quantity<name>`; and a declaration named `Quantity` refuses `invalid_syntax`. | Test (TC-187) |
| FR-142-AC-14 | A quantity literal in a lowered body is a checked-package `literal` of `value_kind` `quantity` typed by its unit's node, and a domain-package operation parameter whose `typeRef` names a unit has the unit's type; FR-322-AC-79 and FR-322-AC-80 own the wire checks. | Test (TC-187, TC-233, TC-280) |
| FR-142-AC-15 | Over the `L` unit graph of TC-187, each single defect of TC-187 U06 and U11a to U11f refuses with its row's code and cause naming the dimension and units shown, and exposes no conversion value; the cycle of U11b and U11l is retained from the least unit by source position, which differs from name order; the orderings of U11h to U11k fix the first-defect rule: scale zero before an unknown target in one unit, the earlier unit before a later one, duplicate root before a self-cycle, and the earlier dimension before a later one. | Test (TC-187) |
| FR-142-AC-16 | The base-dimension rule refuses `L` in U11g1, U11g2, U11g4 and U11z1 (an imported `L` reached only through the imported `Area` a local unit is typed by) and admits U11g3 (`N = L / L` with `unit one : N = 1;`); U11m fixes that the check runs at the base dimension's own turn: a unit cycle in the earlier-declared `Area` is reported first, and with the order reversed `L` is refused; U11y refuses an imported unitless base dimension named by a local dimension's term. | Test (TC-187) |
| FR-142-AC-17 | A dimension's own terms are checked at its turn before its units: U11n (cycle of two), U11o (self-cycle) and U11p (unknown term) refuse as FR-142 states; U11w refuses the unknown term before the cycle; U11x refuses a transitive unknown term at the dimension that owns it, not at the dimension that reaches it, and U11z3 reports a dimension that only reaches a cycle by its own earlier unit defect before the cycle member's refusal. The order of two documents in the request fixes the first declaration (U11q). | Test (TC-187) |
| FR-142-AC-18 | Local units typed by an imported dimension are checked at that node's turn with the imported units as admitted members: U11r admits a local unit targeting an imported root; U11s refuses a local cycle, U11t a second local root beside the imported root, and U11u a local zero scale; the imported dimension's own definition is not re-checked; U11z2 fixes the turn list: a dimension's mentioned nodes take their turn in mention order before the declaration, so `P.L` precedes `Q.T`. | Test (TC-187) |
| FR-142-AC-19 | A unit of compilation that declares `dimension Time; unit s : Time = 1; unit ms : Time = 1/1000 * s;` beside the admitted domain package `acme/rpc` declaring base dimension `Rate`, root `us` of `Rate` and its own `ms` of `Rate` checks with no `ambiguous_declaration`: `Quantity<ms>` names the source unit and `Quantity<M::ms>` under `model M = "acme/rpc"` names the package unit (FCD identity `ix://acme/rpc/unit/ms`), which carries no declaration name; a source `unit tick : M::Rate = 1/2 * M::us;` is admitted, typed by the package dimension, and the same unit written `unit tick : Rate = 1/2 * M::us;` refuses `missing_declaration`/`missing-name` at `Rate`; the package also declaring a base dimension `Length` with no unit, beside a source `dimension Area = M::Length ^ 2; unit m2 : Area = 1;`, refuses `missing_declaration`/`missing-name` by the base-dimension rule at `Length`'s turn, located at `/semantic_graph/nodes/0/owner` and naming `ix://acme/rpc/dimension/Length` and `m2`. | Test (TC-187) |
| FR-142-AC-20 | Dimensions of different owners compose: a unit of compilation declaring `dimension Time; unit s : Time = 1;` with `model R = "acme/rpc"` and `model F = "acme/flow"`, whose packages each declare base dimension `Time` and root unit `s` of it, checks with no refusal; bare `Time` names the source's dimension, `R::Time` the `acme/rpc` dimension (`ix://acme/rpc/dimension/Time`) and `F::Time` the `acme/flow` dimension, three distinct dimensions with `Quantity<s>`, `Quantity<R::s>` and `Quantity<F::s>` three distinct types; a source `unit tenth : R::Time = 1/10 * R::s;` is admitted and a source `dimension Speed = R::Time ^ -1 * F::Time;` is admitted; a source that declares no `Time` and writes `unit x : Time = 1;`, a source writing `Z::Time` with no model alias `Z`, and a source writing `R::Speed` where `acme/rpc` declares no `Speed` each refuse `missing_declaration`/`missing-name` at the name; a source declaring `dimension Time;` twice refuses `ambiguous_declaration`/`ambiguous-name`; a package whose document carries two dimensions of identity `ix://acme/rpc/dimension/Time` refuses `invalid_model_binding`/`conflicting-binding` at intake. | Test (TC-187) |
| FR-142-AC-21 | Over the dimensions of FR-142-AC-20 with no dimension equivalence declared, `f(d: Quantity<R::s>)` called as `f(3 R::s)` is admitted and `f(3 F::s)` and `f(3 s)` refuse `ill_typed`/`type-mismatch` at the argument; `3 R::s + 3 F::s`, the comparison `3 R::s < 3 F::s` and `convert<Quantity<F::s>>(3 R::s)` refuse `ill_typed` by incompatible dimensions before any charge; `3 R::s / 3 F::s` has the normalized map `R::Time` 1 and `F::Time` -1 and is not the dimensionless unit, `3 R::s / 3 R::s` is, and `(3 R::s) * (2 F::s)` has a compound unit of the two root units, unequal to that of `(3 R::s) * (2 R::s)`; with the source derived dimension `dimension Dur = R::Time;` and `unit dsec : Dur = 1;`, `convert<Quantity<dsec>>(3 R::s)` refuses `ill_typed` for two unequal dimension nodes, while `let c = ((3 R::s) * (1 R::s)) / (1 R::s) in convert<Quantity<dsec>>(c)` is admitted (step 1 yields `3` of the compound unit with the one term `R::s` at exponent 1, step 2 meets the map `R::Time` 1 of `Dur` and yields `3 dsec`). | Test (TC-187) |
| FR-142-AC-22 | A dimension equivalence is decided by FR-142 "Dimension equivalence" steps 1 to 7. Over the sources of FR-142-AC-20, `dimension R::Time ~ F::Time by 1 R::s = 1 F::s;`, `dimension R::Time ~ Time by 1 R::s = 1 s;` and, with the source `dimension Dur = R::Time; unit dsec : Dur = 1;` after `R::Time ~ F::Time by 1 R::s = 1 F::s;`, `dimension Dur ~ F::Time by 1 dsec = 1 F::s;` are each admitted; `dimension R::Time ~ Z::Time by 1 R::s = 1 Z::s;` with no model alias `Z`, `dimension R::Time ~ R::Speed by 1 R::s = 1 R::mps;` where `acme/rpc` declares no `Speed`, `dimension R::Time ~ Nothing by 1 R::s = 1 s;`, and `dimension R::Time ~ F::Time by 1 R::s = 1 F::second;` where `acme/flow` declares no `second` each refuse `missing_declaration`/`missing-name` at the unresolved name; `dimension R::Time ~ R::Time by 1 R::s = 1 R::s;` refuses `invalid_package`/`definition-cycle` retaining the edge `R::Time -> R::Time`; a second declaration `dimension F::Time ~ R::Time by 1 F::s = 1 R::s;` after `dimension R::Time ~ F::Time by 1 R::s = 1 F::s;`, and `dimension F::Time ~ R::Time by 1 F::s = 2 R::s;` after it, each refuse `invalid_package`/`conflicting-definition` at the later declaration, retaining both; `dimension R::Time ~ F::Time by 1 R::m = 1 F::s;` where `R::m` is a unit of `acme/rpc`'s base dimension `Length`, and `dimension R::Time ~ F::Time by 1 F::s = 1 R::s;` (each unit named for the other dimension), refuse `invalid_package`/`incompatible-definition` retaining the unit and the dimension node; `dimension R::Time ~ F::Time by 0 R::s = 1 F::s;`, and the same with `rational(-1, 1)` in place of `0`, refuse `invalid_package`/`invalid-value`, and so does `dimension R::Time ~ F::Time by 1 R::s = 1 F::local;` where `acme/flow` declares `unit local : Time = 1 * s + 5;`, retaining `F::local` and its composed offset `5`; `dimension R::Time ~ R::Area by 1 R::s = 1 R::m2;` over the derived `Area = Length ^ 2` of `acme/rpc` refuses `invalid_package`/`incompatible-definition` retaining both nodes, and `dimension Dur ~ F::Time by 1 dsec = 1 F::s;` with no `R::Time ~ F::Time` refuses `invalid_package`/`incompatible-definition` (map `R::Time` 1 against `F::Time` 1); `dimension R::Speed ~ F::Speed by 1 R::mps = 1 F::mps;` with both speeds `Length / Time` of their package and each `mps` the root of its speed is admitted after `R::Length ~ F::Length by 1 R::m = 1 F::m;` and `R::Time ~ F::Time by 1 R::s = 1 F::s;`, refused `incompatible-definition` without them, and refused `incompatible-definition` as `by 1 R::mps = 2 F::mps` even after them, because the base roots fix the ratio 1; with three base dimensions `A`, `B` and `C` whose root units are `ua`, `ub` and `uc`, `A ~ B by 1 ua = 1 ub;`, `B ~ C by 1 ub = 1000 uc;` and `C ~ A by 1000 uc = 1 ua;` are admitted together, and `C ~ A by 1 uc = 1 ua;` in place of the third refuses `invalid_package`/`conflicting-definition` at the third declaration retaining all three in source order; the decisions are the same in any source order of the declarations within a phase. | Test (TC-187) |
| FR-142-AC-23 | With `dimension R::Time ~ F::Time by 1 R::s = 1000 F::ms;` over the dimensions of FR-142-AC-20 and `acme/flow` declaring `unit ms : Time = 1/1000 * s;` (root ratio 1), `convert<Quantity<F::s>>(3 R::s)` is `3 F::s` with one identification edge charged, `convert<Quantity<F::ms>>(3 R::s)` is `3000 F::ms` with two edges charged, `convert<Quantity<R::s>>(1500 F::ms)` is `3/2 R::s`, and `convert<Quantity<R::s>>(3 F::s) = 3 R::s` is `true`; `3 R::s + 3 F::s` and `3 R::s = 3 F::s` refuse `ill_typed` as distinct units of one equivalence class, not as incompatible dimensions, `convert<Quantity<R::s>>(3 F::s) + 3 R::s` is `6 R::s`, `f(d: Quantity<R::s>)` called as `f(convert<Quantity<R::s>>(3 F::s))` is admitted and as `f(3 F::s)` refuses `ill_typed`/`type-mismatch`; `3 R::s / 3 F::s` is the dimensionless `1` and `(3 R::s) * (2 F::s)` is `6` of the compound unit of `(3 R::s) * (2 R::s)`; `convert<Quantity<R::m>>(3 F::s)` where `acme/rpc` declares an unrelated root unit `m` of base dimension `Length` refuses `ill_typed`. With the third declaration `dimension Time ~ F::Time by 1 s = 1 F::s;` a conversion between the two members that are not the class root charges two edges and one involving the class root charges one. | Test (TC-187) |
| FR-142-AC-24 | An equivalence never anchors a base dimension: with `acme/rpc` declaring base dimension `Mass` with root unit `kg` and `acme/flow` declaring base dimension `Mass` with no unit, `dimension R::Mass ~ F::Mass by 1 R::kg = 1 F::kg;` refuses `missing_declaration`/`missing-name` at `F::kg`, and `dimension Force = F::Mass; unit n : Force = 1;` refuses `missing_declaration`/`missing-name` by the base-dimension rule naming `ix://acme/flow/dimension/Mass` and `n`. The normalized map of a product, quotient or power holds the base class representative: `(3 R::s) / (3 F::s)` and `(3 R::s) / (3 R::s)` are the same dimensionless unit and `Quantity<R::s>` squared and `Quantity<F::s>` squared have one compound unit. A derived dimension converts through the compound pivot with or without an equivalence, and `dimension Torque ~ Energy by 1 N_m = 1 J;` over two derived dimensions of equal map, `N_m` and `J` each the root of its dimension, admits the direct conversion of `N_m` to `J`, which without the declaration refuses `ill_typed`; `by 1 N_m = 2 J` refuses `invalid_package`/`incompatible-definition`, the base roots fixing the ratio 1. | Test (TC-187) |
| FR-142-AC-25 | Unary minus over a non-affine quantity returns the exact negated value in the operand's own unit and type, including `-(2 m)` as `-2 m`, `-(3 ms)` as `-3 ms` in `ms`, `-(0 m)` as `0 m` and a compound-unit operand in its compound unit, with no edge charge; unary minus over an affine-unit quantity refuses `ill_typed` before any charge; `-(2 m) + 1 m` is `-1 m`. | Test (TC-187) |
| FR-142-AC-26 | The root ratio carries the size of one package's canonical root in the other's. With `acme/rpc` (alias `R`, root `s`) and `acme/clock` (alias `G`, base dimension `Time`, root unit `ms`, and `unit s : Time = 1000 * ms;`), `dimension R::Time ~ G::Time by 1 R::s = 1000 G::ms;` and `dimension R::Time ~ G::Time by 1 R::s = 1 G::s;` state one root ratio, 1000 (`b` is 1000 for `G::s`), and `dimension G::Time ~ R::Time by 1000 G::ms = 1 R::s;` is the same equivalence written the other way; the class root is the root `R::s` of the member with the least key. `convert<Quantity<G::ms>>(3 R::s)` is `3000 G::ms` with one identification edge charged, `convert<Quantity<R::s>>(1500 G::ms)` is `3/2 R::s`, `convert<Quantity<G::s>>(3 R::s)` is `3 G::s` with two edges charged, `3 R::s / 3 G::ms` is the dimensionless `1000`, `(3 R::s) * (2 G::ms)` is `3/500` of the compound unit with the one term `R::s` at exponent 2, and `3 R::s + 3 G::ms` refuses `ill_typed` as distinct units. With `dimension R::Time ~ F::Time by 1 R::s = 1000 F::ms;` of FR-142-AC-23 (`acme/flow` root `s`) and the first of the declarations above declared, `dimension F::Time ~ G::Time by 1 F::s = 1000 G::ms;` is admitted and `dimension F::Time ~ G::Time by 1 F::s = 1001 G::ms;` refuses `invalid_package`/`conflicting-definition` at itself, retaining the three declarations in source order, because the ratios around the cycle multiply to 1.001. | Test (TC-187) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
