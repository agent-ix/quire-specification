---
id: FR-153
title: "Query explicit closed model environments"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-043
    type: references
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
---

# FR-153: Query explicit closed model environments

## Description

When evaluating lookup or all-instances, the evaluator SHALL query only the
explicit typed environment and population closure bound to the request.

## Inputs

A population binding (package typed binding role `population`) with its anchor,
an optional declared maximum `N` (its absence means the population is
unbounded), its population declaration from the admitted domain package and its
runtime member records; the FR-150 effective view and ModelSelection; the
requested type; and, for lookup, a key reference and absence mode.

## Outputs

A typed reference, option or bounded collection, an incomplete result, or a
refusal.

## Behavior

Lookup names its absence behavior per query. `allInstances` returns the complete
selected population only when object and subtype closure are both established;
otherwise it is incomplete. Neither operation consults a process-global registry
or invents objects from samples. Both forms are defined by the complete-V1
grammar and the selected `quire.model.complete/v1` definition (rules
`quire.model.environment.lookup/v1` and `quire.model.environment.population/v1`).

## Environment key and closure

The native type of an explicit population binding SHALL be spelled
`Population<T>[N]` when it declares maximum `N`, and `Population<T>` when it
declares none. `T` SHALL resolve from a qualified name to a model `object`
type; an admitted name of a non-object type refuses
`ill_typed`/`type-mismatch`. `N` SHALL use the grammar's unsigned integer
spelling (`0` or a nonzero digit followed by digits); `0` is an admitted
maximum. An omitted maximum SHALL NOT infer a finite bound. This type
spelling does not declare a population artifact or supply its binding,
closure, anchor or runtime members. The lowercase `population` keyword in
an abstraction binding is a separate source form, not a type constructor.

A population binding is bound to one population declaration. The population
declaration is declared by a `population` artifact and lists its member types
and its extent. Its declaration key must belong to the binding's
ModelSelection, otherwise `foreign_reference`/`foreign-model-selection`. The
runtime member records are observation input; each names its object identity
and the declaration key of its most-specific type. Each member's key is
`(ModelSelection key, effective type identity of the member's most-specific
type, FR-204 reference triple)`. Effective identities are used after
normalization; original identities appear only in provenance. Duplicate member
records collapse only when their keys and member digests are equal; otherwise,
and whenever two members share universe and object identity with different
types, binding admission refuses `invalid_runtime_input`/`conflicting-identity`.
A member type not covered by the population declaration's member types is
`foreign_reference`/`foreign-type`. A member whose most-specific type is
abstract is `invalid_runtime_input`/`abstract-instance`.
After the uncharged model-selection and closure checks, binding admission is
metered by `PopulationAdmissionLimitsV1` of `quire.value.accounting/v1`: each
member in input order is charged `binding.member` and then checked for
foreign type (each supertypes edge its coverage walk follows is charged
`admission.ancestor-edge` against `ancestor_steps` before it is followed),
abstract type and for duplicate collapse or conflicting identity, and each
subsetting value set is charged `binding.subset-value` before its FR-151 check.
Admission stops at the first refusal, and a denied charge is incomplete with no
binding.

Object closure holds exactly when the population declaration's extent is
`closed`; otherwise `incomplete_population`/`incomplete-scope`. Subtype closure
for a queried type `T` holds exactly when every effective type that conforms to
`T` is covered by the population declaration's member types; otherwise
`incomplete_population`/`unclosed-subtypes`. A member type covers itself and
every type that conforms to it. Object closure is decided at binding admission
before any admission charge, subtype closure before the query's first charge,
and no partial collection is returned.

`allInstances<T>(p)` requires `p` to be a population binding, otherwise
`ill_typed`/`operator-ineligible`, and `T` to be a model `object` type,
otherwise `ill_typed`/`type-mismatch`. Its type is `Set<Reference<T>>[0,N]`
when `p` declares a maximum `N`, and the unbounded `Set<Reference<T>>` when `p`
declares none. It contains every member whose most-specific type
conforms to `T`, once by reference key, in canonical reference-key order; the
input member order is never observable. `population.visit` is charged for
every member walked, whether or not it conforms to `T`. When `p` declares a
maximum `N`, a selected count above `N` is
`cardinality_out_of_bound`/`above-maximum` after `collection.bound`; a denied
charge is `incomplete` with no collection. An unbounded `p` admits every
selected count and never returns `cardinality_out_of_bound`.

`lookup<T>(p, r) absent m` requires the same population binding and a key
`r: Reference<S>` with `S` conforming to `T`, otherwise `ill_typed`/`type-mismatch`.
After `lookup.key`, a key whose universe differs
from the universe of `T` is `foreign_reference`/`foreign-universe`. It is present
when `p` has a member with the key of `r`. The absence mode is part of the
query and is never inferred from the result type:

| Mode | Static type | Present | Absent |
| --- | --- | --- | --- |
| `undefined` | `Reference<T>` | the member reference | `undefined` with the catalogued reason `absent-key` |
| `empty` | `Option<Reference<T>>` | the present option | `none` |
| `refused` | `Reference<T>` | the member reference | `refused { code: invalid_runtime_input, cause: absent-key }` |

A population binding carries its anchor: in a postcondition,
`allInstances<T>(p)` and `lookup` read the post population, and
`pre(allInstances<T>(p))` reads the invocation's pre population. An object
deleted in the post state keeps its pre most-specific type. Charges are the
`lookup.*`, `population.visit` and `collection.*` schedules of
`quire.value.accounting/v1`. A checked downcast is outside V1.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-153-AC-1 | Closed all-instances returns every and only member of the selected typed population in canonical reference-key order. | Test (TC-198) |
| FR-153-AC-2 | Unknown object or subtype closure returns incomplete, and a missing key follows the query's absence mode. | Test (TC-198) |
| FR-153-AC-3 | Foreign universe or type keys and conflicting duplicate entries refuse with their named causes without ambient fallback; a known count above the declared maximum refuses `cardinality_out_of_bound`, while a denied charge is incomplete. | Test (TC-198) |
| FR-153-AC-4 | Each absence mode produces its distinct undefined, empty or refused result. | Test (TC-198) |
| FR-153-AC-5 | All-instances includes closed subtype populations exactly once and emits no partial population when closure is unknown. | Test (TC-198) |
| FR-153-AC-6 | A subtype object seen through supertype and subtype queries has one reference key whose type component is its most-specific type. | Test (TC-198) |
| FR-153-AC-7 | `pre(allInstances<T>(p))` in a postcondition reads the pre population, and a deleted object keeps its pre type. | Test (TC-198) |
| FR-153-AC-8 | A binding typed `Population<T>[N]` requires a model object target `T` and an unsigned maximum `N`, including zero; an admitted non-object target refuses `ill_typed`/`type-mismatch`. `allInstances<T>(p)`'s result is exactly `Set<Reference<T>>[0,N]`, typed to the queried `T` and bounded by `p`'s declared maximum `N`, when `p` declares one. Lookup's present result is `Reference<T>` in the `undefined` and `refused` modes and `Option<Reference<T>>` in the `empty` mode, in every case typed to the queried `T`; every lookup spells its mandatory `absent` mode. | Test |
| FR-153-AC-9 | A binding typed `Population<T>` declares no maximum and infers no finite default; `allInstances<T>(p)`'s result is exactly the unbounded `Set<Reference<T>>`, admits every selected count, and never returns `cardinality_out_of_bound`. | Test |
| FR-153-AC-10 | Admitting a member whose most-specific type is `n` supertypes edges from a covering population member type completes at `ancestor_steps = n` and, at `n - 1`, returns exactly `incomplete(ancestor_steps, n-1, n-1, 1, admission.ancestor-edge)` with no population admitted; a zero limit denies the first followed edge; an unavailable `binding.member` charge wins before a simultaneous edge denial. | Test (TC-198) |
| FR-153-AC-11 | The `unsupported_construct` refusal that a package not selecting `quire.model.complete/v1` receives for model forms names exactly `lookup`, a dispatched call, an `operation-body` and an `abstraction-decl`. A `Population<T>[N]` type, an `allInstances<T>(p)` call and field or relationship-end navigation are not refused by that rule: a record-field navigation is a value-layer form, and without the model profile `T` resolves to no model object type, so a population type or `allInstances` over it refuses under its own typing rule, not `unsupported_construct`. | Test (TC-198) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
