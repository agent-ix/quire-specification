---
id: FR-376
title: "Evaluate population-valued expressions in refinement mappings"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/FR-145
    type: references
  - target: ix://agent-ix/quire-specification/FR-149
    type: references
---

# FR-376: Evaluate population-valued expressions in refinement mappings

## Description

When a refinement mapping row builds an abstract value from a whole concrete
population, the checker SHALL type the population-valued form against the
abstract type, and the evaluator SHALL compute it over the population's
objects in the current concrete state.

## Inputs

An admitted refinement declaration ([FR-375](FR-375-declare-model-refinement.md))
whose field rows, step-row arguments or history updates use a population
name or one of the forms below, and a concrete state.

## Outputs

A typed value, or `map` undetermined at that state
([FR-377](FR-377-decide-model-refinement.md)).

## Surface syntax

Inside a refinement declaration, and only there, the `primary` production of
the [complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md)
admits a population name and the productions `count`, `sum`, `exists`, `all`,
`only` and `seq`, with the `binder` production and the `..` token that `seq`
uses. The grammar owns the spellings; this requirement owns what they mean.
A `model-name` in `primary` position names a concrete population. A
`binder` with no `where` clause selects every object of the population.
A state-clause body outside a refinement declaration that uses one of these
forms refuses `unsupported_construct`/`expression-form`.

## Behavior

**Population value.** A concrete population name `C::p` denotes the set of
that population's objects in the current state, of type
`Set<Reference<T>>[0, n]`, where `T` is the population's member type and `n`
its universe size. The binder variable has type `Reference<T>`; the `where`
condition and the body read it, the concrete state and the enclosing row's
`self`.

**Forms.** Let `S` be the objects of `C::p` satisfying the `where` condition.

| Form | Type | Value |
| --- | --- | --- |
| `count(x in C::p where c)` | `Int[0, n]` | the number of objects in `S` |
| `sum(x in C::p where c: e)` | `e`'s numeric type, with bounds `[min(0, n·lo), max(0, n·hi)]` for `e` bounded by `[lo, hi]` | the exact sum of `e` over `S`, `0` when `S` is empty; the bound includes `0` even when `lo` and `hi` are both positive or both negative, because `S` can be empty |
| `exists(x in C::p where c)` | `Boolean` | `S` is non-empty |
| `all(x in C::p where c)` | `Boolean` | every object of `C::p` satisfies `c` |
| `only(x in C::p where c: e)` | `e`'s type | `e` for the one object of `S`; undefined when `S` is empty or has two or more objects |
| `seq(i in lo .. hi: e)` | `Sequence<E>[0, m]` | the sequence of `e` for each integer `i` from `lo` to `hi` in ascending order, empty when `hi < lo` |

In `seq`, `lo` and `hi` are integer expressions with declared bounds
`[lo_min, lo_max]` and `[hi_min, hi_max]`, `E` is `e`'s type with `i` of type
`Int[lo_min, hi_max]`, and the declared maximum length `m` is
`max(0, hi_max - lo_min + 1)`.

**Order independence.** `count`, `sum`, `exists` and `all` are independent of
the order of `S`. `only` reads one object. No form traverses `S` in an order,
so no value depends on the identity order of the population's objects.

**Typing.** The checker types each form by the table and then checks the
row's expression against the abstract type as FR-375's row typing states. A
`where` condition that is not `Boolean`, or a `seq` bound that is not an
integer, refuses `ill_typed`/`type-mismatch` at that expression. A population
name that is not a concrete population of the refinement's concrete model
refuses `missing_declaration`/`missing-name`.

**Undefinedness.** An `only` whose `S` is empty or has two or more objects
makes the enclosing row undefined at that state with the undefined reason `only-not-one`, the same reason for no object and for several
([native diagnostics](../../../proposals/quire-v1/definitions/native-diagnostics.md)),
and the refinement settles
`refuted` with cause `undefined-evaluation` at that state (FR-377, FR-379).
The same holds for any undefined sub-expression. A refused or incomplete
sub-expression makes `map` undetermined at that state (FR-377).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-376-AC-1 | A concrete population name denotes the set of its objects in the current state, typed `Set<Reference<T>>[0, n]` with `n` the universe size, and the six forms have exactly the types and values of the table. | Test (TC-311) |
| FR-376-AC-2 | `seq(i in lo .. hi: e)` yields `e` for each integer from `lo` to `hi` in ascending order, the empty sequence when `hi < lo`, and declared maximum length `max(0, hi_max - lo_min + 1)`. | Test (TC-311) |
| FR-376-AC-3 | `only` over exactly one matching object yields its body; over no matching object, or over two or more, the enclosing mapping row is undefined at that state. | Test (TC-311, TC-316) |
| FR-376-AC-4 | A non-Boolean `where` condition or a non-integer `seq` bound refuses `ill_typed`/`type-mismatch`, a form outside a refinement declaration refuses `unsupported_construct`/`expression-form`, and a population name that is not a concrete population refuses `missing_declaration`/`missing-name`. | Test (TC-311) |
| FR-376-AC-5 | Under the ring-buffer mapping `items = seq(i in 0 .. self.size - 1: only(s in R::slots where s.ring = self and convert<Integer>(s.index) = (self.head + i) mod 2: s.value))`, the state with `head = 1`, `size = 2`, `s1.value = 1`, `s0.value = 0` maps to `items = [1, 0]`, and renaming the slot keys leaves every mapped value unchanged; the comparison is written with `convert<Integer>(s.index)` because `mod` yields an `Integer` and an `Integer` against an `Int[..]` under `=` is `ill_typed`/`type-mismatch` ([FR-149](../type-model/FR-149-apply-complete-equality-matrix.md)). | Test (TC-311, TC-316) |
| FR-376-AC-6 | `sum(x in P where c: e)` over a universe of `n` objects with `e` bounded by `[lo, hi]` has the bound `[min(0, n·lo), max(0, n·hi)]`: a summand in `[1, 2]` over two objects has the bound `[0, 4]`, not `[2, 4]`, a row of it into `Int[0, 4]` is admitted and into `Int[2, 4]` refuses `ill_typed`/`type-mismatch`, and an empty `S` sums to `0`. | Test (TC-311) |
| FR-376-AC-7 | An `only` whose matching set is empty and one whose matching set holds two objects each give an undefined evaluation with the reason `only-not-one` and a payload naming the population binding, the `only` locus and the enclosing row; a counterexample's cause carries that reason. | Test (TC-311) |
| FR-376-AC-8 | The shared grammar parses `count(x in P)`, `count(x in P where c)`, `sum(x in P where c: e)`, `exists(x in P)`, `all(x in P where c)`, `only(x in P where c: e)` and `seq(i in lo .. hi: e)` inside a refinement row, and scans `0..1` and `0 .. 1` as the same three tokens. Outside a refinement declaration a bare population name and the `count`, `sum`, `exists` and `all` population forms refuse `unsupported_construct`/`expression-form`, `exists(x in P: e)` is the state-clause quantifier, and `only(x in P where c: e)` and `seq(i in lo .. hi: e)` are syntax errors, `invalid_syntax`/`unexpected-token` at the `in` token, because `only` and `seq` are ordinary identifiers there. Inside a refinement row `only(`, `seq(` and `Some(` always read as the forms, so a pure function of one of those names cannot be called from a row. | Test (TC-311) |

## Dependencies

- [FR-375](FR-375-declare-model-refinement.md) admits the declaration these
  forms appear in and owns row typing.
- [FR-377](FR-377-decide-model-refinement.md) owns the reading of an
  undefined row as an undefined evaluation of the refinement claim and of a
  refused or incomplete row as `map` undetermined.
- [FR-145](../expressions/FR-145-convert-and-query-collections.md) owns the
  collection algebra whose set type the population value has.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns
  the expression productions this requirement extends.

## References

- Design: quire-spec-language ADR-020 (RM-8, QS-10, RU-5).
- Tickets: Linear STD-133; paired QSL-367.
