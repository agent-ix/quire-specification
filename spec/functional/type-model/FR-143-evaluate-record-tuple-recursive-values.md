---
id: FR-143
title: "Evaluate records, tuples and finite recursive values"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-009
    type: references
  - target: ix://agent-ix/quire-specification/AD-005
    type: references
  - target: ix://agent-ix/quire-specification/FR-204
    type: references
  - target: ix://agent-ix/quire-specification/FR-307
    type: references
  - target: ix://agent-ix/quire-specification/AD-015
    type: references
  - target: ix://agent-ix/quire-specification/FR-440
    type: references
  - target: ix://agent-ix/quire-specification/FR-441
    type: references
---

# FR-143: Evaluate records, tuples and finite recursive values

## Description

When checking a record, tuple or recursive value, the semantic kernel SHALL
apply its declared field order, types, structural equality and finite-value
construction rules.

## Inputs

Named record or positional tuple definitions, recursive constructors, values
and traversal limits.

## Outputs

A typed finite value/equality result, refusal or incomplete traversal outcome.

## Behavior

Record fields use declaration-owned identities; tuple positions are ordered.
Structural equality recursively compares the same declared value type and does
not compare object-reference targets structurally. When two record operands
have one static type but different most-specific declarations, because one holds
a record subtype value at an ancestor-typed slot (FR-208), they are unequal and
the comparison is the Boolean `false`; it is not a refusal, and no field is
compared. Recursive definitions are
allowed, but each runtime value must be finite and cycles require explicit
object identity rather than value containment.

## Composite-value contract

A record value contains exactly one value or explicit absence/null state for
each declaration-owned field, ordered by field declaration for canonical
serialization only. A tuple contains exactly its declared arity in positional
order. Missing required fields, extra fields, wrong arity and conflated
absence/null refuse construction before evaluation.

A recursive value definition is legal when every recursion edge crosses a
named constructor field or union member payload position. A runtime value is a
finite constructor tree/DAG; a back-edge in value containment refuses. Sharing
an immutable contained value does not create object identity. Cycles are
representable only by explicit typed `Reference<T>` values resolved in a
closed object environment.

A union value construction `U::m(e, ...)` names exactly one declared member
`m` of `U` and supplies exactly its declared payload positions in order; a
nullary member's construction is the bare qualified reference `U::m`, with no
call syntax. Naming an undeclared member, supplying a payload for a nullary
member, omitting the call for a member that declares a payload, and supplying
an argument count other than a payload member's declared arity are each
`refused { code: ill_typed, cause: type-mismatch }`, located at the
originating expression, before evaluation.

## Declarations and identity

Complete V1 declares composite value types with the grammar's `record-decl`,
`tuple-decl` and `union-decl`. A `union-decl` names a closed set of members,
each with an optional ordered payload shape. A record value expression
`R { f: e, ... }` is the single constructor of record `R`, a tuple value is
the qualified call
`T(e, ...)` of tuple `T`, and a union value is the qualified reference or call
`U::m` / `U::m(e, ...)` of one declared member `m` of union `U`, matching that
member's declared payload arity. An `alias-decl` names its target type and
creates no declaration identity.

A record, tuple or union declaration identity is the opaque I04 node key of
its `composite_type` node whose `semantic_form` is `record`, `tuple` or
`union`. Complete V1 defines no nominal identity preimage for these nodes,
unlike the enum, dimension and declared-unit keys of FR-141, FR-142 and
FR-322. The checked package producer assigns the key; it is unique within one
checked semantic graph, and no reader or evaluator recomputes it. Two
declarations are the same exactly when their keys are equal within one checked
package. Equal qualified names, owners, field/member names, field/member
types or shapes never make two keys equal, and one declaration has exactly one
key. An imported declaration has the key that the importing package's graph
assigns to the resolved FR-307 export, so every dependency path that resolves
one export yields one key. No source-declared composite key is stable across checked packages. A model
record value type or event type (FR-208) is the exception: its declaration key
is its `model`/`record_value_type` or `model`/`event_type` node key
([FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
"Model-owned members" step 2), a function of the selected document's identity
and the node alone. It is not derived from the selecting unit's source owner or
from the alias a unit gives the model, so two units that select one model under
different aliases agree on every such key, and the key is recomputable and
equal across checked packages that select that document. FR-144's key order
and FR-149's equality of a subtype value use this key.
Mixing is impossible at check time, because every type named in one checked
package, imported ones included, resolves to that package's own keys, so no
source expression can denote a composite type of another package. Values meet
evaluation only through runtime inputs and snapshots validated against the
evaluating package. A supplied composite value whose declaration key is neither a
key of that package's graph nor, for a model record value type or event type, a
declaration key of the evaluating package's selected document is
`refused { code: invalid_runtime_input, cause: wrong-value-kind }` during input
validation, before any evaluation or charge. A supplied union value names its
union by declaration key, its active member by that member's
[FR-441](../../objects/interfaces/FR-441-union-member-key.md) member key, and
its payload values in position order. Input validation refuses it with
`refused { code: invalid_runtime_input, cause: wrong-value-kind }`, before any
evaluation or charge, when its member key is not the key of a member of that
union, when its payload count differs from that member's declared payload
arity, or when a payload value is not a value of the member's declared type at
its position. Input validation walks each payload value exactly as it walks a
record field value, so a reference held in a payload is validated as one held
in a field is. A record field identity is
(declaration key, field identifier), a tuple position identity is
(declaration key, zero-based position), and a union member identity is
(declaration key, member identifier).

A union value carries exactly one active member and, when that member
declares a payload, exactly one value for each of its declared positions in
order; a nullary member carries no payload. Two union values of the same
declared union compare structurally equal exactly when their active member is
the same declared member and, if that member carries a payload, every
positional value compares equal in declaration order; values of the same
member shape from different declared unions never become interchangeable,
exactly as for record and tuple declarations above.

## Value profile

`quire.value.complete/v1` admits union declarations, union values, union
construction and `case` (FR-146) in every package that selects it, exactly
where it admits record and tuple declarations and values. A package that
declares or evaluates a union selects the same always-selected definitions,
triggers and alternatives as one that declares only records and tuples.

## Field presence

A field declared without `?` is required, and its slot holds exactly one
present value. A field declared with `?` holds exactly one of a present value,
`absent` or explicit `null`. In a record value expression, omitting a `?` field
constructs `absent` and `f: null` constructs explicit `null`. Type checking
refuses each of the following with `refused { code: ill_typed }`, located at the
originating field or call: omitting a required field; naming an undeclared
field; naming a field twice; supplying `null` for a required field; and calling
a tuple with an argument count other than its declared arity. `null` is never a
value of `Option<T>`, and `none` is never a field state.

A projection `r.f` of a `?` field is admitted only as the operand of
`present(r.f)` or `value(r.f)`. `present(r.f)` is true exactly when the slot
holds a present value, and false for both `absent` and explicit `null`.
`value(r.f)` returns that value and carries the FR-146 presence obligation, so
without a proved `present(r.f)` a linked body is
`refused { code: undefined_expression, cause: unproved-presence }`. Any other
use of `r.f`, such as an arithmetic operand, call argument, comparison operand
or field value, is `refused { code: ill_typed, cause: type-mismatch }`, because a
field slot is not a value of `T`. Whole-record equality still compares slot
states, so `absent` and `null` stay distinct. A projection of a field declared
without `?` is always admitted.

## The `absent` Option literal

`absent` is a literal expression of type `Option<T>`: the value that holds no
`T`. It is a `primary` (shared-grammar `primary` gains the alternative `'absent'`,
an already reserved word, and no operator), and a second spelling of the
`none` primary: it denotes exactly the `none` value of `Option<T>` that FR-149 compares, that
`lookup<T>(p, r) absent empty` returns (FR-153) and that the refinement rows of
FR-375 write as `none`, so `(lookup<T>(p, r) absent empty) = absent` is true
when `r` is not a member of `p`. It is how a state clause, a function
body or a transition writes that value: `self.v = absent` drops an
`Option<Clock>` place (FR-415) and sets any other `Option<T>` field to no value.

`absent` and `none` have no standalone type and obey one rule below. Like a collection literal (FR-144) it is
checked against the unique expected type of its position: a declared field,
parameter or result type, a `let` annotation, a `convert` target, the element
of an enclosing literal, a tuple position or union member payload, the
assigned place of a write such as `self.v = absent`, the static type of the
other operand of `=` or `!=`, or an arm body or `if` branch whose own expected
type is unique. The expected type must be `Option<T>`; the literal then has
exactly that type, whatever `T` is, and a `Clock`-bearing `T` is no exception
(an `absent` of type `Option<Clock>` is a write that holds no clock, so it is
not a clock read). A position with no unique expected type, such as both
operands of `=` or `!=`, the right side of an unannotated `let`, or the operand
of `present` or `value`, is `refused { code: ill_typed, cause: ambiguous-literal }`
before any charge. A position whose expected type is not an option, such as an
`Integer` field, a `Reference<T>` or a collection, is
`refused { code: ill_typed, cause: type-mismatch }`. A `?` field is not an
`Option<T>` position (Field presence): `P { b: absent }` for `b: Integer?` is
`type-mismatch`, because that slot is absent by omitting the field and is never
written with an option value, and `null` is still never an `Option<T>` value.

The word `absent` has one other use: in `lookup<T>(p, r) absent m` it follows
the closing parenthesis of the `lookup` and introduces the absence mode `m`
(`undefined`, `empty` or `refused`). That is a part of the `lookup` primary and
never an expression; the literal stands only where a `primary` starts, so the
two cannot meet, and a `lookup` carries its mode in either position of a larger
expression (`lookup<T>(p, r) absent empty = absent` parses as the lookup with
mode `empty`, then `=`, then the literal).

## Recursion rule

The declaration containment graph has one edge from declaration `D` to each
record, tuple or union declaration named anywhere in the type of a field,
position or member payload position of `D`, including through `Option`,
`Sequence`, `Set`, `Bag` and `OrderedSet`. `Reference<T>` contributes no
containment edge. An edge is *named* when it starts at a record field or a
union member payload position; the field's or position's whole type,
including every option or collection nested inside it, belongs to that field
or position. A tuple position is not a named constructor field, so an edge
that starts at a tuple position is unnamed even when an option or collection
lies on it. An edge *escapes* when its path passes a `?` field, an
`Option<...>` or a collection type whose minimum bound is `0`.

Type checking admits the containment graph exactly when both the subgraph of
unnamed edges and the subgraph of non-escaping edges are acyclic, self-edges
included. Otherwise every declaration on an offending cycle is
`refused { code: ill_typed }`, and the refusal names the cycle. Thus every
recursion crosses a named field or union member payload position, and every
recursion can end in a finite value. For example,
`record List { head: Integer; tail: List?; }` is admitted,
`record Loop { next: Loop; }` is refused because it does not escape,
`tuple Pair(Integer, Option<Pair>);` is refused because it is unnamed, and
`union Tree { Leaf, Node(Integer, Option<Tree>, Option<Tree>) }` is admitted on
the same basis as `List`, because each recursive member payload position
escapes through `Option`.

## Construction evaluation order

Every construction refusal above is decided by type checking before any field
expression is evaluated. After the last field or argument completes,
construction charges one `composite.result-retain` with
`value_occurrences = occ(result)` and `result_units += occ(result)` under
`quire.value.accounting/v1`, then exposes the value. Record field expressions are evaluated in field
declaration order, regardless of the order of the source fields. Tuple
arguments and union payload arguments are evaluated in position order. A
union value construction charges the same `composite.result-retain` after its
last payload argument completes; a nullary member's construction evaluates no
argument and charges it at once. `occ` of a union value is one plus `occ` of
each of its payload values, so a nullary union value has `occ` one. The first field or argument whose
outcome is undefined, refused or incomplete stops construction. That outcome
becomes the construction's outcome, no later field or argument is evaluated or
charged, and no record, tuple or union value exists. Operands of a binary expression are
evaluated left and then right, so a stopped left operand prevents evaluation of
the right operand.

## Object references

In `Reference<T>`, `T` must name a model object type exported by a bound model;
a record, tuple, enumeration, alias of a value type or other value type is
`refused { code: ill_typed, cause: type-mismatch }`.

No Complete-V1 source literal, constructor or conversion creates an object
identity or a `Reference<T>` value. A reference is obtained only from a bound
model snapshot under the selected state contract, as a model field of
reference type, a receiver, a population member, an `allInstances` member, a
`lookup` result or a relationship-end navigation result (FR-152, FR-153). It can
also arrive through a parameter, `let` binding or composite field that carries
such a value. Its identity is the snapshot-supplied FR-009/FR-204 triple
(universe, object-type declaration identity, declared object identity). Under
the selected `quire.model.complete/v1` definition the universe is the
`quire.model.object-universe/v2` identity, the type is the effective
declaration identity of the object's most-specific type, never the static `T`,
and `Reference<S>` converts to `Reference<T>` only by the FR-149 upcast row. Its retained observation is
not part of identity. `Reference<T>` is terminal in containment, construction
and equality.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-143-AC-1 | Equal finite values of the same declared record/tuple type compare structurally equal. | Test (TC-188) |
| FR-143-AC-2 | Equal field shapes from different declared types do not become interchangeable. | Test (TC-188) |
| FR-143-AC-3 | Cyclic value containment refuses, while bounded recursive construction and object-reference cycles remain distinct. | Test (TC-188) |
| FR-143-AC-4 | Missing/extra record fields, wrong tuple arity and absence/null substitution each refuse at the originating component. | Test (TC-188) |
| FR-143-AC-5 | Canonical field ordering does not make equal-shaped values of different declared record types equal. Two operands of one static record type whose most-specific declarations differ, such as a `TaxedMoney` and a `Money` with equal shared fields at `Money`, are unequal: `=` is `false` and `!=` is `true`, in either operand order, and neither is a refusal. | Test (TC-188) |
| FR-143-AC-6 | Declaration identity is the producer-assigned record/tuple/union node key compared only within one checked package; a model record value type or event type is identified by its model node key instead. | Test (TC-188) |
| FR-143-AC-7 | The recursion rule admits exactly the declaration graphs whose unnamed-edge and non-escaping-edge subgraphs are both acyclic, and names each refused cycle. | Test (TC-188) |
| FR-143-AC-8 | Record fields are evaluated in declaration order, the first stopped field propagates its outcome with no later field evaluated or charged, and a completed construction charges `composite.result-retain`. | Test (TC-188) |
| FR-143-AC-9 | `Reference<T>` admits only a model object type, and a runtime composite value from another checked package is `invalid_runtime_input` before evaluation, except a model record value type or event type value, whose key is its model node key and is admitted when it is a declaration key of the evaluating package's selected document. | Test (TC-188) |
| FR-143-AC-10 | A `?` field projection is admitted only under `present` or `value`, an unguarded `value` is `undefined_expression` with cause `unproved-presence`, and any other use is `ill_typed` with cause `type-mismatch`. | Test (TC-188) |
| FR-143-AC-11 | Under `quire.model.complete/v1`, a reference obtained by navigation, lookup or `allInstances` carries the object's most-specific effective type, and no other source creates a reference. | Test (TC-198) |
| FR-143-AC-12 | A `union-decl` construction admits exactly one declared member with its declared payload arity, two union values compare equal exactly when their active member and payload positions match, and equal member shapes from different declared unions never become interchangeable. | Test (TC-262) |
| FR-143-AC-13 | The recursion rule treats a union member payload position as a named edge on the same basis as a record field, so a union recursion is admitted exactly when it escapes. | Test (TC-263) |
| FR-143-AC-14 | `Shape::Rect(2, 3)` evaluates `2` then `3`, then charges one `composite.result-retain` with `value_occurrences = 3` and `result_units += 3`; `Shape::Empty` charges one `composite.result-retain` with `value_occurrences = 1` and evaluates nothing; a stopped first payload argument propagates its outcome with the second argument neither evaluated nor charged and no `composite.result-retain`. | Test (TC-399) |
| FR-143-AC-15 | A supplied `Shape` argument is refused `invalid_runtime_input`/`wrong-value-kind` before any evaluation or charge when its member key is a member key of another union, when its member key is no member's key, when it carries one payload value for `Rect`, and when its `Circle` payload is a `Text` value; a well-formed supplied `Shape::Rect(2, 3)` is admitted. | Test (TC-400) |
| FR-143-AC-16 | A package selecting `quire.value.complete/v1` with exactly the always-selected definitions and no trigger admits a union declaration, its construction and a `case` over it, and its selection equals that of the same package with the union replaced by a record. | Test (TC-400) |
| FR-143-AC-17 | Two units that select one model under different aliases (`model M = ...` and `model N = ...`) give `Money` and `TaxedMoney` the same declaration keys, each equal to the record's model node key; a `TaxedMoney` built in one unit equals the same value built in the other, and a `Set<Money>` holding a `Money` and a `TaxedMoney` orders its two members identically in both. | Test (TC-188) |
| FR-143-AC-18 | `absent` takes its `Option<T>` from the unique expected type of its position and denotes the `none` of that type: the write `self.v = absent` for `v: Option<Clock>` and for `v: Option<Integer>`, the argument for an `Option<Integer>` parameter, a function result typed `Option<Integer>`, an `if` branch whose other branch is an `Option<Integer>` operand, the record field `v: absent` for `v: Option<Integer>`, and either operand of `=` against an `Option<Integer>` operand are admitted; `(lookup<M::A>(p, r) absent empty) = absent` is true for a key `p` does not hold and false for one it holds; `absent = absent` of two `Option<Integer>` operands is true. | Test (TC-188) |
| FR-143-AC-19 | An `absent` with no unique expected type (both operands of `=`, `let x = absent in x`, `present(absent)`, `value(absent)`) is `refused { code: ill_typed, cause: ambiguous-literal }` before any charge, and one whose expected type is not an option (an `Integer` field or parameter, a `Reference<T>`, a `Set<Integer>`) is `refused { code: ill_typed, cause: type-mismatch }`; neither is read as the `none` of a guessed type. | Test (TC-188) |
| FR-143-AC-20 | A `?` field takes no `absent` literal: `P { a: 1, b: absent }` for `b: Integer?` is `refused { code: ill_typed, cause: type-mismatch }` while omitting `b` still constructs the absent slot, and `null` for an `Option<Integer>` parameter stays `ill_typed`. `lookup<M::A>(p, r) absent undefined`, `absent empty` and `absent refused` parse as before, the `absent` after the `)` introducing the mode, and `lookup<M::A>(p, r) absent absent` is not a mode and is `refused { code: invalid_syntax, cause: unexpected-token }`. | Test (TC-188) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
