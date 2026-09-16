---
id: FR-145
title: "Convert and query every collection kind explicitly"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-041
    type: references
  - target: ix://agent-ix/quire-specification/FR-144
    type: references
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
---

# FR-145: Convert and query every collection kind explicitly

## Description

When applying a collection conversion or query, the evaluator SHALL derive the
result kind, order, uniqueness, multiplicity and bound from the selected
operation contract.

## Inputs

Bounded collection, typed pure function/predicate, target kind and result bound.

## Outputs

A typed collection/scalar result, explicit loss record, refusal or incomplete
outcome.

## Behavior

`map` and `collect` are one operation. Filter preserves the source kind where
closed under filtering. Flatten defines source/element kind combinations and
combines occurrence order/multiplicity without implicit deduplication. Fold and
reduce over a set or bag admit only the closed step catalog below, fold takes
its declared identity and reduce has no empty value. Conversions that lose order or duplicates report that loss.

## Operation matrix

| Operation | Sequence | Set | Bag | Ordered set |
| --- | --- | --- | --- | --- |
| `map` / `collect` | Preserve source occurrence order and multiplicity. | Produce a set and coalesce equal results. | Preserve result multiplicity. | Preserve first-result order and coalesce later equal results. |
| `filter` | Preserve surviving occurrence order and multiplicity. | Preserve the set of surviving values. | Preserve the multiplicity of surviving values. | Preserve the order of surviving first occurrences. |
| `flatten` | Concatenate nested semantic occurrences in outer then inner order. | Union nested values. | Sum nested multiplicities. | Preserve first occurrence while traversing outer then inner order. |
| `count` | Count occurrences whose predicate is true. | Count members whose predicate is true, so each value contributes zero or one. | Evaluate the predicate once per occurrence and count true occurrences, so each value contributes zero or its multiplicity. | Count members whose predicate is true, so each value contributes zero or one. |
| `fold` | Traverse occurrence order from the declared identity. | Require a commutative and associative function. | Require a commutative and associative function and apply it once per occurrence. | Traverse declared order from the declared identity. |
| `reduce` | Traverse occurrence order; empty input is undefined. | Require a commutative and associative function; empty input is undefined. | Require a commutative and associative function and apply it once per occurrence; empty input is undefined. | Traverse declared order; empty input is undefined. |

`map` and `collect` are one semantic operation. `flatMap` is exactly
`flatten(map(source, function))`; it is not a separately extensible operator.
An explicit target-kind conversion records each discarded property from
`order`, `uniqueness` and `multiplicity`. An empty fold returns only its declared
identity. An empty reduction has no value and returns a located undefined
outcome.

## Forms, result types and bounds

The source forms are the grammar's `map`, `collect`, `filter`, `flatMap`,
`flatten`, `fold<A>(acc, x in c: step, identity: e)`,
`reduce<A>(acc, x in c: step)`, `forall`, `exists`, `count<N>`, `sum<N>`,
`size`, `contains` and `convert<K<T>[min,max]>(c)`. A binder is evaluated once
per visited occurrence. Sequences and ordered sets are visited in occurrence
order, and sets and bags in FR-144 canonical-key order. `forall` stops at the
first false and `exists` at the first true; the empty cases are true and false.
`contains(c, v)` decides FR-149 membership without a visiting order. `size`
returns the FR-144 bound count, and `count<N>` and `sum<N>` keep the
`state-contract` result-domain rules for every kind. A `sum` over a set or bag
must prove that every accumulated prefix lies inside the domain for every visit
order, not only for the canonical order.

The static result type is derived from source `K<T>[a,b]`, as follows. `map`
and `collect` over a sequence or bag return `[a,b]`, and over a set or ordered
set return `[min(a,1),b]`. `filter` returns `[0,b]`. For `flatten` of an outer
`K1<K2<T>[c,d]>[a,b]`, the result kind is `K1`, and the bound is `[a*c,b*d]` for
a sequence or bag, or `[min(a*c,1),b*d]` for a set or ordered set. `flatten`
whose outer kind is a sequence or ordered set and whose inner kind is a set or
bag is `refused { code: ill_typed }`, because the inner kind has no semantic
occurrence order; there is no implicit key-order substitute. Every other
kind combination is admitted by the operation matrix. Because the derived bound
always contains the actual result, only a constructor or `convert` against an
authored bound can return FR-144 `cardinality_out_of_bound`.

## Fold and reduce admission

`fold` requires `identity:` and `reduce` forbids it; each omission or addition
is `refused { code: ill_typed }`. For `reduce`, `A` must be the element type,
and the first element initializes the accumulator. Every empty `reduce` returns
located undefined, whatever the collection kind; a static minimum bound of at
least one proves that the operation is defined. For a sequence or an ordered
set, any pure total `step` of type `A` is admitted.

For a set or bag, a step is commutative and associative only when all of the
following hold: its body is `acc OP e` or `e OP acc`; `acc` does not occur in
`e`; and `OP` with `A` is `+` or `*` on `Integer`, or `and` or `or` on
`Boolean`. Any other set or bag step, including one over a bounded `Int`,
`Rational`, `Decimal` or quantity accumulator, is
`refused { code: ill_typed }` and names the failed commutativity and
associativity obligation. A bounded accumulator is refused because
intermediate membership would depend on visit order.

This syntactic catalog is the only way a step's commutativity and associativity
is established. A called function, a library export, a comment or any
declaration of algebraic properties is never trusted and never proves them, so
a step whose body calls a function is refused for a set or bag even when that
function is commutative and associative.

## Explicit kind conversion and loss

`convert<K2<T>[min,max]>(c)` of a `K1<T>` value is the only collection-kind
conversion, and writing `convert` is the explicit acceptance of its loss. The
element type must be identical, and element conversion is
`refused { code: ill_typed }`. Kind properties are: sequence
{`order`, `multiplicity`}; set {`uniqueness`}; bag {`multiplicity`}; ordered set
{`order`, `uniqueness`}. The completed result carries
`CollectionLoss { discarded }`, where `discarded` is the static set difference
of the `K1` properties minus the `K2` properties, listed in the order `order`,
`uniqueness`, `multiplicity`. It is recorded even when no duplicate actually
occurs. Converting a set or bag to a sequence or an ordered set uses ascending
canonical-key order and is `refused { code: ill_typed }` for an unkeyed element
type. The authored target bound is checked after coalescing and before
materialization.

## Accounting

Every form charges the `quire.value.accounting/v1` collection family. A
*visit* is one `collection.visit` (one work unit) charged before the binder is
bound to that occurrence; the body, predicate or step is then evaluated with its
own charges. *Formation* is FR-144 construction steps 2 to 4 (membership
decisions, `collection.bound` and `collection.result-retain`) over the produced
occurrences in production order. A *scalar retain* is one
`collection.result-retain` with `result_units += 1`. Each form charges exactly:

| Form | Charges in order |
| --- | --- |
| `map`, `collect` | a visit and body per source occurrence; then formation of the results, with membership decisions for a set, bag or ordered-set result |
| `filter` | a visit and predicate per source occurrence; then formation of the survivors, with no membership decision because survivors are already distinct or grouped |
| `flatten` | per outer occurrence, a visit of it and then a visit of each of its inner occurrences; then formation of the inner occurrences in visit order, with membership decisions for a set, bag or ordered-set result |
| `flatMap` | the `map` charges, then the `flatten` charges |
| `fold` | the identity expression; then a visit and step per occurrence; no retain charge, because the last evaluated step or identity has already charged its value |
| `reduce` | a visit of the first occurrence, which initializes the accumulator; then a visit and step per later occurrence; an empty source charges nothing and returns located undefined |
| `forall`, `exists`, `count`, `sum` | a visit and predicate or summand per occurrence until a short-circuit stops the query; each `sum` addition charges what that addition charges; then a scalar retain |
| `size` | a scalar retain only |
| `contains(c, v)` | one `collection.member-test` whose amount is the sum of the FR-149 plan sizes of `v` against every member of `c` (every occurrence of a sequence, and every distinct member of a set, bag or ordered set), with no early exit and no charge when `c` is empty; every plan-formation check precedes it; then a scalar retain |
| `convert` | a visit per source occurrence (for a bag to a set or ordered set, one visit per distinct member); then formation, with membership decisions only when the source is a sequence and the target is a set, bag or ordered set |

Visits and the charges of evaluated bodies interleave in visiting order. The
first unavailable charge returns incomplete, and no partial collection or
accumulator is exposed. Because every per-element evaluation is preceded by a
named charge, an unbounded source can never evaluate unmetered.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-145-AC-1 | Map/filter/flatten over each collection kind returns the specified kind and deterministic semantic occurrences. | Test (TC-190) |
| FR-145-AC-2 | Fold on an empty collection uses only its required declared identity; an empty reduction returns located undefined, and a fold without or a reduction with `identity:` refuses. | Test (TC-190) |
| FR-145-AC-3 | A conversion that loses order or multiplicity requires explicit acceptance and records the exact loss. | Test (TC-190) |
| FR-145-AC-4 | The operation matrix determines result kind, occurrence order, uniqueness and multiplicity for every source kind. | Test (TC-190) |
| FR-145-AC-5 | Set and bag folds or reductions refuse a function that is not both commutative and associative. | Test (TC-190) |
| FR-145-AC-6 | Result bounds, flatten kind admission and set/bag canonical-key visit order are derived exactly as specified, and a stopped binder exposes no partial result. | Test (TC-190) |
| FR-145-AC-7 | A kind conversion records exactly the static `CollectionLoss` property difference and refuses element conversion or unkeyed ordering. | Test (TC-190) |
| FR-145-AC-8 | Every query and conversion form charges exactly its tabled visits, membership decisions, bound and retain charges in order, and each denied charge returns incomplete with no partial result. | Test (TC-190) |
| FR-145-AC-9 | A set or bag step outside the closed syntactic catalog is `ill_typed` even when it calls a function that declares or has commutative and associative behavior. | Test (TC-190) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
