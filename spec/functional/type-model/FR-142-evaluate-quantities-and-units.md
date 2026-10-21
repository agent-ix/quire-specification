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
offset quantity refuses. Conversion never occurs implicitly.

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

The admission of a unit graph SHALL refuse a topology defect before any
quantity value exists, with these causes, none of which needs a catalog
revision (FR-272); each refusal names the dimension node and the unit or units
shown:

| Defect | Code and cause | Retains |
| --- | --- | --- |
| a unit's scale is zero | `invalid_package`/`invalid-value` | the unit and its scale |
| the canonical root's scale is not one or its offset is not zero | `invalid_package`/`invalid-value` | the root, its scale and its offset |
| a unit names a target that is no unit of the checked package | `missing_declaration`/`missing-name` | the unit and the target name |
| a unit's target belongs to another dimension node | `invalid_package`/`incompatible-definition` | the unit, its target and both dimension nodes |
| the dimension has two or more targetless units | `invalid_package`/`conflicting-definition` | the dimension and every targetless unit, in ascending identity order |
| the target edges form a cycle (a unit targeting itself included) | `invalid_package`/`definition-cycle` | the units of the cycle in path order |

A dimension whose units include no targetless unit has an unknown target, a
cross-dimension target or a cycle, since a finite graph in which every unit
names a target within the dimension is cyclic; it is refused by the cause of
the first such defect, and no separate missing-root cause exists. The admission
SHALL check the defects in the table's order, for each dimension in ascending
dimension identity and each unit in ascending unit identity, and SHALL report
the first. A refused dimension yields no unit mapping.

A unit is affine exactly when the offset of its composed mapping to the
canonical root is nonzero; a declared edge offset alone does not decide it, so a
zero-offset unit targeting an affine unit is affine and a unit whose nonzero
edge offsets compose to zero is not. An affine unit may be converted or compared
but cannot be added, subtracted, multiplied, divided or raised to a power; such
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
one and offset zero; no unit preimage carries a coherence scale. A compound
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
in that unit. Multiplication, division and integer power traverse the left and
then the right operand's root path before their operation event. Top-level quantity equality and
ordering expressions share one schedule: both operands are mapped along their root paths,
left then right, and compared by exact root value, so an affine unit or a
negative composed scale compares by root value. A quantity leaf inside an FR-149 composite
equality plan is compared by exact value in its identical unit and charges only
its `equality.pair`. Quantity comparison between
distinct units requires an explicit prior conversion.

Base-dimension and unit identities are opaque I04 checked-semantic-graph node
keys. A base dimension's `quire.checked-semantic-node/v1` digest is SHA-256 over
RFC 8785 JCS of `{ version: quire.dimension-node/v1, owner,
qualified_declaration, terms }`, where owner is the stable subject projection of
an exact admitted source, DefinitionRef or ModelRef and must join that lock
selection. A unit digest uses `{ version: quire.unit-node/v1, owner,
qualified_declaration, dimension_node_id, target_unit_node_id, scale,
offset }`, with normalized exact rationals. Acyclic unit targets make these keys
recomputable in root-to-leaf order. The strict I04 reader refuses a body/key
mismatch as `invalid_semantic_graph`. A dimension node is self-typed and depends
exactly on the base-dimension keys in its terms. A unit's `semantic_type` equals
`dimension_node_id`; its dependencies are exactly that dimension plus its
non-null target. Contradictory semantic fields refuse even when the retained
preimage and key agree. Equal names, scales or dimension shapes do
not confer identity, while the same exact imported owner/declaration remains
stable across unrelated package edits. Dimension exponents are mathematical signed integers subject to
explicit operation limits; implementations cannot narrow them to a machine
integer. A conversion publishes no partial quantity until canonicalization,
target rounding and target-domain admission all succeed.

Multiplication, division and integer power first convert every non-affine
operand to its dimension's canonical root, perform exact value arithmetic and
normalize the resulting base-dimension exponents. The result unit is an
evaluator-owned value identity in the `quire.value.compound-unit/v1` domain.
Its JCS preimage is `{ version: quire.value.compound-unit/v1, terms }`, where
`terms` is the ascending list of admitted I04 canonical root-unit node IDs with
their nonzero mathematical integer exponents. The empty list is the
dimensionless unit. This digest is not an I04 checked-semantic node, graph
reference, declaration or source-mapped authority, and evaluation does not
insert it into the checked package. Its closed schema and vectors are selected
by `quire.value.complete/v1`. An explicit later conversion may select a
compatible declared I04 unit; multiplication never guesses one from a matching
name or scale. For an integer exponent, `x^0` is exactly one for every `x`,
including zero, and a zero base under a negative exponent is undefined.
All identity reads, traversed edges, exact rational operations, target-domain
checks and result retention use the named points in
`quire.value.accounting/v1`, in its unit schedule: operand identity reads, then
the runtime undefined conditions, then every edge charge, then the rational
events, then the target domain and result retention.

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
`ambiguous_declaration`/`ambiguous-name` at the use. A bare `type-ref` looks up
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
  bound on the magnitude; a bounded magnitude is not a complete-V1 type form.
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
| FR-142-AC-8 | The published `quire.value.compound-unit/v1` schema and vectors fix the compound-unit identity preimage: each vector is schema-valid, its RFC 8785 JCS SHA-256 equals its recorded identity, and its terms name admitted root-unit node keys with nonzero exponents in strictly ascending key order; each authored stale-key mutation changes the identity and each authored noncanonical mutation, including a `-0` exponent, is refused by exactly its named schema or semantic check. | Test (TC-233) |
| FR-142-AC-9 | Equality or ordering of quantities in distinct units refuses `ill_typed` before any charge, while identical-unit equality and ordering compare exact root values after the left and then the right root path. | Test (TC-187) |
| FR-142-AC-10 | A conversion with a compound side compares normalized base-dimension maps, traverses only the declared side's root path, and admits a declared-to-compound-to-declared pivot in two explicit steps. | Test (TC-187) |
| FR-142-AC-11 | With the dimension `T`, the unit `s` (root of `T`) and the unit `ms` (target `s`, scale `1/1000`), a function `f(d: Quantity<ms>)` called as `f(3 ms)` admits the argument `3 ms` and refuses `f(3 s)` with `ill_typed`/`type-mismatch` at the argument; `d: Quantity<T>` (a dimension) and `d: Quantity<nothing>` each refuse `missing_declaration`/`missing-name` at the name, and a bare `d: ms` refuses `missing_declaration`/`missing-name` at `ms`. | Test (TC-187) |
| FR-142-AC-12 | A function `f(d: Quantity<ms>): Quantity<ms>` returning `d` checks; returning `convert<Quantity<s>>(d)` refuses `ill_typed`/`type-mismatch` at the returned expression; and `Quantity<ms>` annotations introduce no checked-package node, naming the node of `ms`. | Test (TC-187) |
| FR-142-AC-13 | Over the `L` unit graph of TC-187, a unit `zero` with scale zero, the root `m` with scale two, and the root `m` with offset one each refuse `invalid_package`/`invalid-value`; a unit `x` with target `nothing` refuses `missing_declaration`/`missing-name`; `cm` with target `s` of dimension `T` refuses `invalid_package`/`incompatible-definition`; a second targetless unit `m_alt` refuses `invalid_package`/`conflicting-definition` naming `m` and `m_alt`; `m` targeting `cm` (with `cm` targeting `m`) refuses `invalid_package`/`definition-cycle` naming both in path order; and a unit targeting itself refuses `definition-cycle`. Each names the dimension node and the units shown, and no conversion value is exposed. | Test (TC-187) |
| FR-142-AC-13 | A unit and a record of one name coexist, and a use of the name in a type position means the record while `Quantity<name>` means the unit; two units of one name in the linked package refuse `ambiguous_declaration`/`ambiguous-name` at `Quantity<name>`; and a declaration named `Quantity` refuses `invalid_syntax`. | Test (TC-187) |
| FR-142-AC-14 | A quantity literal in a lowered body is a checked-package `literal` of `value_kind` `quantity` typed by its unit's node, and a domain-package operation parameter whose `typeRef` names a unit has the unit's type; FR-322-AC-79 and FR-322-AC-80 own the wire checks. | Test (TC-187, TC-233, TC-280) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
