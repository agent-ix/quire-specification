---
id: FR-440
title: "Checked-package union, union value and case nodes"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-143
    type: references
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/FR-441
    type: references
  - target: ix://agent-ix/quire-specification/AD-015
    type: references
---

# FR-440: Checked-package union, union value and case nodes

## Description

When a `quire.checked-semantic-graph/v2` graph carries a `union-decl`, a
union value construction or a `case` expression, the producer SHALL encode
them as the `composite_type`/`union` node, the `value`/`union_value` node
and the `expression`/`case` node below, and the reader SHALL refuse a node
whose joins below do not hold with the refusal this requirement names, in
[FR-322](FR-322-checked-package-artifact.md)'s first-refusal order.

## Properties

- **Union type node** — `node_tag: "composite_type"`,
  `semantic_form: "union"`. It is a source-declared node, so it carries
  `declaration` (`{qualified_name}`) and a `declaration` occurrence, as a
  `record` node does. Its `body` is an `aggregate` with one `binding` per
  member of the union, in member declaration order. A binding's `name` is
  the member identifier, and its `value` is an `aggregate` of `reference`
  terms naming the member's payload type nodes in position order; a nullary
  member's `value` is the empty `aggregate`. Its `dependencies` are the
  payload type nodes, by FR-322's dependency rule. A union whose payload
  names the union itself, directly or through another declaration, names the
  members of that cycle through one `recursion_group`, as FR-322 requires of
  every recursive reference. Its family is `union`.
- **Union value node** — `node_tag: "value"`, `semantic_form: "union_value"`.
  It is the construction `U::m` or `U::m(e, ...)` of
  [FR-143](../../functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md).
  Its `semantic_type` is the union type node of `U`. Its `body` is an
  `aggregate` of exactly one `binding`, whose `name` is the identifier of the
  active member `m` and whose `value` is an `aggregate` of the payload
  argument terms in position order, each spelled as a `record_value`
  binding's value is spelled; a nullary member's `value` is the empty
  `aggregate`. It carries no `declaration`.
- **Case node** — `node_tag: "expression"`, `semantic_form: "case"`. Its
  `body` root is an `application` with `operator: "case"` and
  `operation: {identity: "quire.op.control.case", laws: [], mode: null,
  member: null, leaves: []}`. Its `arguments` are the scrutinee term
  followed by one **arm** per member of the scrutinee's union, in that
  union's member declaration order, whatever order the source arms are
  written in. Its `result_type` and the node's `semantic_type` are the
  `case` result type ([FR-146](../../functional/expressions/FR-146-check-total-pure-functions.md),
  "Case result type"). A `quire.op.control.case` application stands only as
  the body root of an `expression`/`case` node; a `case` used inside another
  term, including another `case`'s arm body, is a `reference` to its own
  `expression`/`case` node.
- **Arm** — a `binding` whose `name` is the member identifier and whose
  `value` is an `aggregate` of exactly two members, in order:
  1. **the binder aggregate** — an `aggregate` of `reference` terms to
     `value`/`parameter` nodes, one per payload position of the member, in
     position order; it is empty for a nullary member;
  2. **the arm body** — the term the arm evaluates.
- **Arm binder** — each binder is a `value`/`parameter` node whose body is
  the [FR-341](FR-341-checked-package-state-clause-body.md) `{name, level}`
  aggregate: `name` is the binder's source identifier, and `level` is its
  binder level under "Binder levels" below. Its `semantic_type` is the
  member's payload type at the binder's position. Every occurrence has role
  `expression`.
- **Catalog entry** — the published operation catalog
  (`quire.checked-operation-catalog/v1`, owned by
  `agent-ix/quire-verification-contracts`, FR-322) SHALL hold
  `quire.op.control.case` with operator class `case`, operands `["union"]`, rest `binder`, result
  form `arm_body`, no laws, no mode, no member kind, no leaf source and the
  one constraint `{kind: "union_arms", operands: [0]}`. The `arm_body` result
  form derives the type every arm body has; the application's `result_type`
  must be that type node. The `union_arms` constraint holds when the rest
  arguments are exactly one arm per member of operand 0's union, in member
  declaration order, each named by its member, each binder aggregate holding
  as many references as that member's payload arity, and each binder's
  `semantic_type` equal to the payload type at its position.
- **Families and groups** — that catalog SHALL list `union` among its
  families, `arm_body` among its result forms and `union_arms` among its
  constraint kinds. The `union` family is a member of the
  `any_value` and `structural_kind` groups, so `quire.op.structural.eq` and
  `quire.op.structural.ne` compare two union values of one union type
  (FR-143), and a union value fits every `any_value` operand position.
- **Structural leaves** — in [FR-322](FR-322-checked-package-artifact.md)'s
  "Structural leaf walk" each union member is an anchor, as a record field
  or a tuple position is. A text leaf reached through a union has the path
  segments `member:<Name>` then `position:i`, where `i` is the index in that
  member's ordered payload, even for a single payload; members are walked in
  member declaration order, and a member with no payload has no leaf. For
  `union Label { Named(Text[0,8;nfc]), Tagged(Integer, Text[0,8;nfc]), Empty }`
  the text leaves are `["member:Named", "position:0"]` and
  `["member:Tagged", "position:1"]`, and `Empty` has none. A cycle through
  a union is a recursion leaf, never `operator-ineligible`. It lists the
  entry `recursion:<d>` with `laws` `[]` and `mode` `null` when a text type
  is reachable from the union, so
  `union Chain { Link(Text[0,8;nfc], Chain), End }` has the leaves
  `["member:Link", "position:0"]` and
  `["member:Link", "position:1", "recursion:0"]`; and nothing when none is,
  so `quire.op.structural.eq` over two values of
  `union List { Cons(Integer, List), Nil }` is admitted with empty
  `leaves`.
- **Declaration-wide leaf identity** — every declared member participates
  in the walk, including members inactive in both compared values. The
  exact member identifier and zero-based payload position are separate
  segments; a path carries no union node key or member ordinal. The
  enclosing operand/result type binds union identity. FR-322's "Union
  text-leaf vectors" U1–U7 fix profiles, nested wrappers, recursion and
  mutual record/union recursion. Its catalog leaf-source rules also apply
  when a union is a collection element; selecting that element starts the
  path at the empty array.
- **Published schema** — the published V2 package schema enumerates
  `union` among `CompositeTypeNode` forms, `union_value` among `ValueNode`
  forms and `case` among `ExpressionNode` forms, and binds the three bodies
  above (`UnionTypeBody`, `UnionValueBody`, `CaseBody`). Both V2 schemas
  enumerate `case` among the application operator classes and
  `quire.op.control.case` among the operation identities, so they enumerate
  exactly the catalog's identities and operator classes (FR-322-AC-17) once
  the catalog holds the entry above. The node-identity preimage schema
  defines `UnionMemberPreimage` ([FR-441](FR-441-union-member-key.md)).
  The published fixture `positive-union-nodes.json` is a package carrying
  every encoding above.

## Binder levels

A binder level is a non-negative integer that numbers the values bound in
scope at a point of a declaration body, outermost first. The binders that
take a level are exactly: the declaration's own bound values, then each
`let` binder, each collection binder of a query, quantifier or fold
(FR-145), and each `case` arm binder whose scope encloses the point.

1. **Declaration.** A function's parameters take levels `0` to `n - 1` in
   declared order. A state clause's bound values take the levels
   [FR-341](FR-341-checked-package-state-clause-body.md) gives them (`self`
   at `0`, then `result` when present, then the operation's parameters).
   A declaration that binds nothing starts with no level taken.
2. **Nested binder.** A `let` binder or a collection binder takes the level
   one above the highest level in scope at its binding point, or `0` when
   none is in scope. It is in scope only in the expression it scopes over.
3. **Arm binders.** The first binder of a `case` arm takes the level one
   above the highest level in scope at the `case`, or `0` when none is in
   scope; each later binder of that arm, in payload position order, takes
   the next level. The arm's binders are in scope only in that arm's body.
   Sibling arms start from the same level, because no arm's binders are in
   scope in another arm.

Worked vector: in
`function f using V (s: Shape, u: Other, k: Integer): Integer pure { let t = k in case s { Rect(w, h): w * h + t; Circle(r): case u { Only(x): x + r }; Empty: t } }`,
`s`, `u` and `k` take levels `0`, `1` and `2`; `t` takes `3`; the `Rect`
arm's `w` and `h` take `4` and `5`; the `Circle` arm's `r` takes `4`; and the
nested `Only` arm's `x` takes `5`.

## Reader joins

The reader checks these joins at FR-322's operation step, in ascending
node-id digest order, each node at its place in that order:

1. **Case placement.** A `quire.op.control.case` application at the body
   root of any node other than an `expression`/`case` node refuses
   `ill_typed`/`operator-ineligible` at the node that holds it. An
   `expression` node whose `semantic_form` contradicts its root
   application's operator class is FR-322's `invalid_semantic_graph`. A
   `quire.op.control.case` application nested inside another term, such as
   another application's arguments, an aggregate's members or a binding's
   value, refuses `ill_typed`/`operator-ineligible` at that nested
   application's `operator` member, not as an FR-322 body-grammar failure
   (FR-322 "Body grammar"). A diagnostic `details` reference to an
   `expression`/`case` node, and a `case` application inside a `details`
   term, are refused by
   [FR-370](FR-370-checked-package-temporal-clause-body.md)'s placement
   check; a `details` reference to a `composite_type`/`union` or
   `value`/`union_value` node is admitted.
2. **Union type.** Two bindings of one union type node with one `name`
   refuse `invalid_package`/`duplicate-member` at the node. A
   payload reference that is not a type node refuses
   `ill_typed`/`operator-ineligible` at the node.
3. **Union value.** The `semantic_type` of a `union_value` node is a union
   type node, its binding names a member of that union, its payload count
   equals that member's payload arity, and each payload term's type is the
   member's payload type at its position, or, for a record-typed payload, a
   record type that conforms to it
   ([FR-322](FR-322-checked-package-artifact.md) "Record conformance");
   otherwise the node refuses
   `ill_typed`/`type-mismatch`, as FR-143 refuses the same construction at
   type checking.
4. **Case.** The `case` application's operand family, its `union_arms`
   constraint and its `arm_body` result are catalog checks; a failure of
   any of them refuses `ill_typed`/`operator-ineligible`, located at the
   `case` node.

A `case` node and a `union_value` node are keyed by the
`quire.application-node/v1` preimage and the node identity preimage FR-322
already defines; a `case` node's arms enter that preimage in member
declaration order, so reordering the arms of a source `case` changes no node
id and no `package_id`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-440-AC-1 | `union Shape { Circle(Integer), Rect(Integer, Integer), Empty }` is a `composite_type`/`union` node carrying `declaration`, whose body is an `aggregate` of the bindings `Circle`, `Rect` and `Empty` in that order, holding one, two and zero `reference` terms to the `Integer` node; a duplicated member name refuses `invalid_package`/`duplicate-member` and a payload reference to a non-type node `ill_typed`/`operator-ineligible`, each at the union node. | Test (TC-395) |
| FR-440-AC-2 | `Shape::Rect(2, 3)` is a `value`/`union_value` node whose `semantic_type` is the `Shape` node and whose body is one binding `Rect` over the two payload terms in order, and `Shape::Empty` is one binding `Empty` over the empty `aggregate`; a binding naming `Triangle`, a `Rect` binding with one payload term and a `Circle` binding whose payload term is `Text` each refuse `ill_typed`/`type-mismatch` at the node. | Test (TC-395) |
| FR-440-AC-3 | `case s { Empty: 0; Rect(w, h): w * h; Circle(r): r }` is an `expression`/`case` node whose body is a `quire.op.control.case` application over the scrutinee and the arms `Circle`, `Rect` and `Empty` in member declaration order, each arm an `aggregate` of its binder aggregate and its body, with the binders `r`, `w` and `h` as `value`/`parameter` nodes typed `Integer`; the same `case` with its source arms in another order has the same node id. | Test (TC-395) |
| FR-440-AC-4 | A `case` application whose arms are out of member order, omit or repeat a member, carry a binder aggregate whose count differs from the member's payload arity, type a binder other than its payload position, or carry an arm body of a type other than the `result_type` refuses `ill_typed`/`operator-ineligible` at the `case` node. | Test (TC-395) |
| FR-440-AC-5 | In the "Binder levels" worked vector, `t`, `w`, `h`, `r` and `x` carry the levels `3`, `4`, `5`, `4` and `5`; with `let t = k in` removed (and `t` replaced by `k`), `w`, `h`, `r` and `x` carry `3`, `4`, `3` and `4`. | Test (TC-395) |
| FR-440-AC-6 | `quire.op.structural.eq` over two `Shape` values is admitted, and over a `Shape` value and a value of another union refuses `ill_typed`/`operator-ineligible` by its `same_type` constraint. | Test (TC-395) |
| FR-440-AC-7 | A `quire.op.control.case` application as the body root of a `function` node refuses `ill_typed`/`operator-ineligible` at that node, and the same `case` as an `expression`/`case` node referenced from the function body is admitted; the same `case` application nested in another application's arguments refuses `ill_typed`/`operator-ineligible` at that nested application's `operator` member, not `malformed_wire`. | Test (TC-395) |
| FR-440-AC-8 | `quire.op.structural.eq` over two `Label` values carries the leaves `["member:Named", "position:0"]` and `["member:Tagged", "position:1"]` and none for `Empty`; over two `List` values of `union List { Cons(Integer, List), Nil }` it is admitted with empty `leaves`; and over two `Chain` values of `union Chain { Link(Text[0,8;nfc], Chain), End }` it is admitted with the leaves `["member:Link", "position:0"]` and `["member:Link", "position:1", "recursion:0"]`. | Test (TC-395) |
| FR-440-AC-9 | Structural equality and inequality over each FR-322 Union text-leaf vector U1–U7 require exactly that vector's complete ordered list, independent of active values. U2's member rename, member reorder, payload swap and source arm reorder have exactly the effects stated with those vectors. | Test |
| FR-440-AC-10 | Fixed `record_value` and `tuple_value` inputs whose bodies contain no application retain their existing `quire.structural-node/v1` preimage bytes and keys. Reordering the source arms of AC-3's `case` retains its declared-order body, node id and package id. | Test (TC-395) |
| FR-440-AC-11 | For fixed union and payload node ids, nullary `Shape::Empty` and `Shape::Rect` with two reference payloads each select `quire.application-node/v1` with the exact aggregate body, `declaration: null`, no `owner` and `recursion: null` outside a recursion group. The producer and strict reader independently match fixed RFC 8785 preimage bytes and canonical node keys; retaining the structural-preimage key refuses `invalid_package`/`stale-node-key` at the application-node key stage. | Test |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the `semantic_graph`,
  the operation catalog and its families, groups and constraints, the
  application-node preimage and the reader's refusal order.
- [FR-341](FR-341-checked-package-state-clause-body.md) owns the
  `value`/`parameter` node body an arm binder uses.
- [FR-143](../../functional/type-model/FR-143-evaluate-record-tuple-recursive-values.md)
  owns union declarations, construction and equality, and
  [FR-146](../../functional/expressions/FR-146-check-total-pure-functions.md)
  owns `case` exhaustiveness, evaluation and its result type.
- [FR-441](FR-441-union-member-key.md) owns the key of each union member.

## Status

Accepted complete-V1 boundary value contract for the union, union value and
`case` node encodings above. Implementation and qualification remain tracked
separately.
