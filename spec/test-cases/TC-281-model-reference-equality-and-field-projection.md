---
id: TC-281
title: "Model reference equality and field projection"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-281: Model reference equality and field projection

## Description

Show that the published operation catalog and FR-322 admit the two model
operations a native producer emits over the lock-selected domain package:
`quire.op.reference.eq` and `quire.op.reference.ne` over two references whose
object types conform one to the other ("Reference conformance"),
`quire.op.reference.upcast` of a reference to the reference type of a type
its object type conforms to, and
`quire.op.record.project` of a model field over a `quire.op.model.deref`
result, whose family is `object` and which the `field_owner` operand group
admits.

## Preconditions

- `proposals/checked-package-v2/model-member-type-vectors.json` carries
  TC-280's domain package `acme/orders` `1.0.0`, the model declaration node
  vectors M1 to M13, and the `reference_equality_cases`,
  `reference_upcast_cases` and `projection_cases`. In a case, `{reference: M}` is an operand of type
  `Reference<M>`, the `composite_type`/`reference` node over node vector `M`,
  and `{object: M}` is an operand of type `M` itself. An upcast case's
  `type_argument` `{reference: M}` names `Reference<M>` the same way. A projection case
  also carries the projection's declared `result_type`.
- `contracts/checked-operation-catalog-v1.json` in `agent-ix/quire-verification-contracts` supplies the
  `quire.op.reference.eq`, `quire.op.reference.ne`,
  `quire.op.reference.upcast`, `quire.op.model.deref` and
  `quire.op.record.project` entries: their operand positions, groups,
  result form and constraints are read from the catalog.
- The lock selects the identity `acme/orders` only. In that package `Sub` (M5)
  declares supertype `Order` (M1), `SubSub` (M7) declares supertype `Sub`,
  `Both` (M6) declares supertypes `Tagged` (M9) and `Coded`, `Flow` (M8) is
  a systems interface with no declared supertype, and `Invoice` (M3) and
  `Both` are unrelated to `Order` and `Sub`.

## Test Procedure

For each reference equality case, fit each operand's family to the entry's
operand position, then decide each constraint the entry names; for
`conforming_reference`, look up both declarations by FR-322 step 2 from the operands' wire owners and decide
conformance along declared supertypes, for one type node as for two. For each
upcast case, fit the operand to the entry's `reference` position, check the
`type_argument` member's family, and decide `conforming_upcast`: look up the
operand's declaration and the member's declaration by FR-322 step 2 and
decide whether the operand's object type conforms to the member's along
declared supertypes, in that direction only. For each projection case, check a
`deref` operand against the `deref` entry (a `reference` operand, and a
declared result equal to the `inner:0` type with a family), fit the
projection operand to the `record.project` operand group, decide `member_of`,
derive the member by FR-322 steps 2 to 4, and check that the declared
`result_type` is the member type. For each reaches case, fit both operands
to the `quire.op.model.reaches_field` entry's `reference` positions and
decide `reference_edge` ("Reaches over a field"): operand 0 names the
member's declaring node, the field resolves by FR-322 steps 2 and 3, its
step 4 type is a reference, option-of-reference or bounded sequence of
references to its owner, and operand 1's object type conforms to that
owner. Compare each result with the recorded expectation.

The domain package's `Order` also declares `parent` (optional `Order`
`[1, 1]`), `origin` (`Order` `[1, 1]`), `prior` (`Order` `[0, 3]`, ordered,
not unique), `peers` (`Order` `[0, 3]`, unordered, unique) and `trail`
(`Order` `[0, unbounded]`, ordered, not unique).

## Vectors

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| RC-01 | `reference.eq` over `Reference<Sub>` and `Reference<Order>` | Admitted: `Sub` conforms to `Order` | FR-322-AC-33 |
| RC-02 | `reference.eq` over `Reference<Order>` and `Reference<Sub>` | Admitted in the other operand order | FR-322-AC-33 |
| RC-03 | `reference.eq` over `Reference<SubSub>` and `Reference<Order>` | Admitted through the supertype chain `SubSub`, `Sub`, `Order` | FR-322-AC-33 |
| RC-04 | `reference.eq` over two `Reference<Order>` operands | Admitted: one type node | FR-322-AC-33 |
| RC-05 | `reference.ne` over `Reference<Sub>` and `Reference<Order>` | Admitted | FR-322-AC-33 |
| RC-06 | `reference.eq` over `Reference<Order>` and `Reference<Invoice>` | `ill_typed`/`operator-ineligible` | FR-322-AC-33 |
| RC-07 | `reference.ne` over `Reference<Sub>` and `Reference<Both>` | `ill_typed`/`operator-ineligible` | FR-322-AC-33 |
| RC-08 | `reference.eq` over `Reference<Order>` and a reference to the relationship `Order-order-Order` (M4) | `ill_typed`/`operator-ineligible` | FR-322-AC-33 |
| RC-09 | `reference.eq` over a reference to M2 (`Order` owned by the unselected identity `acme/billing`) and `Reference<Order>` | `missing_declaration`/`missing-selection` | FR-322-AC-33 |
| RC-10 | `reference.eq` over an `Order` object operand and `Reference<Order>` | `ill_typed`/`operator-ineligible`: family `object` is not `reference` | FR-322-AC-33 |
| RC-11 | `reference.eq` over two references to the relationship `Order-order-Order` (M4) | `ill_typed`/`operator-ineligible` | FR-322-AC-33 |
| RC-12 | `reference.eq` over two references to M2 | `missing_declaration`/`missing-selection` | FR-322-AC-33 |
| RC-13 | `reference.eq` over `Reference<Both>` and `Reference<Tagged>` | Admitted: `Both` conforms to its supertype `Tagged` | FR-322-AC-33 |
| RC-14 | `reference.ne` over `Reference<Tagged>` and `Reference<Both>` | Admitted | FR-322-AC-33 |
| RC-15 | `reference.eq` over `Reference<Flow>` and `Reference<Order>` | `ill_typed`/`operator-ineligible`: no declared supertype chain | FR-322-AC-33 |
| RC-16 | `reference.eq` over two `Reference<Flow>` operands | Admitted | FR-322-AC-33 |
| RU-01 | `reference.upcast` of `Reference<Sub>` to `Reference<Order>` | Admitted: `Sub` conforms to `Order` | FR-322-AC-94 |
| RU-02 | `reference.upcast` of `Reference<SubSub>` to `Reference<Order>` | Admitted through the supertype chain `SubSub`, `Sub`, `Order` | FR-322-AC-94 |
| RU-03 | `reference.upcast` of `Reference<Order>` to `Reference<Order>` | Admitted: one type node | FR-322-AC-94 |
| RU-04 | `reference.upcast` of `Reference<Both>` to `Reference<Tagged>` | Admitted: `Both` conforms to its supertype `Tagged` | FR-322-AC-94 |
| RU-05 | `reference.upcast` of `Reference<Order>` to `Reference<Sub>` | `ill_typed`/`operator-ineligible`: a downcast | FR-322-AC-94 |
| RU-06 | `reference.upcast` of `Reference<Order>` to `Reference<Invoice>` | `ill_typed`/`operator-ineligible`: unrelated types | FR-322-AC-94 |
| RU-07 | `reference.upcast` of `Reference<Sub>` to `Reference<Both>` | `ill_typed`/`operator-ineligible`: unrelated types | FR-322-AC-94 |
| RU-08 | `reference.upcast` of `Reference<Flow>` to `Reference<Order>` | `ill_typed`/`operator-ineligible`: no declared supertype chain | FR-322-AC-94 |
| RU-09 | `reference.upcast` of `Reference<Order>` to a reference to the relationship `Order-order-Order` (M4) | `ill_typed`/`operator-ineligible` | FR-322-AC-94 |
| RU-10 | `reference.upcast` of a reference to M2 to `Reference<Order>` | `missing_declaration`/`missing-selection` | FR-322-AC-94 |
| RU-11 | `reference.upcast` of `Reference<Order>` to a reference to M2 | `missing_declaration`/`missing-selection` | FR-322-AC-94 |
| RU-12 | `reference.upcast` of an `Order` object operand to `Reference<Order>` | `ill_typed`/`operator-ineligible`: family `object` is not `reference` | FR-322-AC-94 |
| RU-13 | `reference.upcast` of `Reference<Both>` to `Reference<Coded>` | Admitted: `Both` conforms to its second supertype `Coded` | FR-322-AC-94 |
| RU-14 | `reference.upcast` of a reference to the relationship `Order-order-Order` (M4) to `Reference<Order>` | `ill_typed`/`operator-ineligible`: a relationship at the source end | FR-322-AC-94 |
| RP-01 | `record.project` of field `total` of M5 over an explicit `deref` of `Reference<Sub>` typed M5 | Resolves the inherited `Order/total`; type `Int[0, 9]`; field member `read_form` is `explicit_deref` | FR-322-AC-34 |
| RP-02 | `record.project` of field `invoice` of M1 over an explicit `deref` of `Reference<Order>` typed M1 | Resolves `Order/invoice`; type `Reference<Invoice>` over M3; field member `read_form` is `explicit_deref` | FR-322-AC-34 |
| RP-03 | `record.project` of field `total` of M5 over a `Reference<Sub>` operand with no `deref` | `ill_typed`/`operator-ineligible` | FR-322-AC-34 |
| RP-04 | `record.project` of field `total` of M1 over `deref` of `Reference<Sub>` typed M5 | `ill_typed`/`operator-ineligible`: `member_of` fails | FR-322-AC-34 |
| RP-05 | `deref` of `Reference<Sub>` declared as M1 | `ill_typed`/`operator-ineligible` | FR-322-AC-34 |
| RP-06 | `deref` of a reference to M4 typed M4 | `ill_typed`/`operator-ineligible`: a relation node has no family | FR-322-AC-34 |
| RP-07 | `record.project` of field `total` of M2 over `deref` of a reference to M2 typed M2 | `missing_declaration`/`missing-selection` | FR-322-AC-34 |
| RP-08 | RP-01 declared with `result_type` `Integer` | `ill_typed`/`operator-ineligible`: not the member type `Int[0, 9]` | FR-322-AC-34 |
| RP-09 | Bare source field read `view.total` of M5 | Resolves the inherited `Order/total`; type `Int[0, 9]`; field member `read_form` is `bare` | FR-322-AC-34 |
| RP-10 | Explicit source read `deref(view).total` of M5 | Resolves the inherited `Order/total`; type `Int[0, 9]`; field member `read_form` is `explicit_deref`; all other projection data equals RP-09 | FR-322-AC-34 |
| RP-11 | RP-09 with `read_form` `implicit` | Fails the closed V2 schema and is refused `malformed_wire` | FR-322-AC-34 |
| RP-12 | A model-object projection with no `read_form` | The schema shape is valid, but the full reader refuses `malformed_wire` | FR-322-AC-34 |
| RP-13 | RP-03 with `read_form: bare` on its undereferenced reference operand | The schema shape is valid, but the full reader refuses the inapplicable `read_form` as `malformed_wire` | FR-322-AC-34 |
| RE-01 | `reaches_field` over `Reference<Order>` twice, field `parent` of M1 | Admitted: `Option<Reference<Order>>` | FR-322-AC-38 |
| RE-02 | Field `prior` of M1 | Admitted: `Sequence<Reference<Order>>[0, 3]` | FR-322-AC-38 |
| RE-03 | Field `origin` of M1 | Admitted: `Reference<Order>` | FR-322-AC-38 |
| RE-04 | `Reference<Sub>` and `Reference<Order>`, field `parent` of M5 | Admitted: inherited edge | FR-322-AC-38 |
| RE-05 | `Reference<Order>` and `Reference<SubSub>`, field `parent` of M1 | Admitted: target conforms to `Order` | FR-322-AC-38 |
| RE-06 | Field `peers` of M1, a set | `ill_typed`/`operator-ineligible` | FR-322-AC-38 |
| RE-07 | Field `trail` of M1, an unbounded sequence | `ill_typed`/`operator-ineligible` | FR-322-AC-38 |
| RE-08 | Field `invoice` of M1, `Reference<Invoice>` | `ill_typed`/`operator-ineligible` | FR-322-AC-38 |
| RE-09 | Field `total` of M1, `Int[0, 9]` | `ill_typed`/`operator-ineligible` | FR-322-AC-38 |
| RE-10 | `Reference<Order>` and `Reference<Invoice>`, field `parent` of M1 | `ill_typed`/`operator-ineligible` | FR-322-AC-38 |
| RE-11 | `Reference<Sub>` and `Reference<Order>`, field `parent` of M1 | `ill_typed`/`operator-ineligible`: operand 0 is not `Reference<M1>` | FR-322-AC-38 |
| RE-12 | Two references to M2, field `parent` of M2 | `missing_declaration`/`missing-selection` | FR-322-AC-38 |
| RE-13 | Field `missing` of M1 | `ill_typed`/`operator-ineligible` | FR-322-AC-38 |
| RE-14 | Two `Reference<Both>`, field `code` of M6 | `ambiguous_declaration`/`ambiguous-name` | FR-322-AC-38 |
| RE-15 | `Reference<Order>` and an `Order` object operand | `ill_typed`/`operator-ineligible`: family `object` | FR-322-AC-38 |

## Record conformance and event vectors

These vectors apply FR-322 "Record conformance" and event equality
(FR-322-AC-68, FR-322-AC-69, FR-322-AC-31) with the published catalog's
`quire.op.record.project`, `quire.op.structural.eq`, `quire.op.structural.ne`,
`quire.op.collection.contains`, `quire.op.collection.set`,
`quire.op.collection.sequence` and `quire.op.function.call` entries. The
selected document for them declares the record value types `Money`,
`TaxedMoney` (supertype `Money`), `GiftTaxedMoney` (supertype `TaxedMoney`) and
`Other`, the event types `OrderPlaced` and `ExpressOrderPlaced` (supertype
`OrderPlaced`) and the object type `Order`. `TaxedMoney` adds `tax_minor` and a
text field `note`; `Money` declares `amount_minor`. These vectors are a reference oracle: `tests/record_conformance.rs` applies the prose rules over inline data, and the vectors are not yet published as a data file.

| Vector | Input | Expected result | Criteria |
| --- | --- | --- | --- |
| EV-01 | A `model`/`event_type` node owned by `OrderPlaced` | Passes the owner join | FR-322-AC-68 |
| EV-02 | A `model`/`event_type` node owned by `Money` (a record value type), by `Order` (an object type) or by an undeclared node | `missing_declaration`/`missing-selection` | FR-322-AC-68 |
| EV-03 | `record.project` of `amount_minor` with `declaration` `OrderPlaced` over an `OrderPlaced` operand, where `OrderPlaced` declares it | Admitted: family `record` | FR-322-AC-68 |
| EV-04 | `structural.eq` and `structural.ne` over two `OrderPlaced` operands | `ill_typed`/`operator-ineligible` | FR-322-AC-68 |
| EV-05 | `structural.eq` over two `Option<OrderPlaced>`, two tuples holding `OrderPlaced` and two `Set<OrderPlaced>` operands | `ill_typed`/`operator-ineligible` | FR-322-AC-68 |
| EV-06 | `structural.eq` and `structural.ne` over two `Money` operands, and over two `Option<Money>` operands | Admitted | FR-322-AC-68 |
| RCF-01 | `record.project` of `amount_minor` with `declaration` `Money` over `Money`, `TaxedMoney` and `GiftTaxedMoney` operands | Admitted; type is the one `Money` gives the field | FR-322-AC-69 |
| RCF-02 | `record.project` with `declaration` `GiftTaxedMoney` over a `TaxedMoney` operand; with `declaration` `TaxedMoney` over a `Money` operand; with `declaration` `Other` over a `Money` operand | `ill_typed`/`operator-ineligible`: `member_of` fails | FR-322-AC-69 |
| RCF-03 | `function.call` over a function whose body `parameters` binding references one `Money` parameter node, and `dispatch_call` of an operation with a `Money` parameter, each with a `TaxedMoney` argument, then a `GiftTaxedMoney` argument | Admitted | FR-322-AC-31, FR-322-AC-69 |
| RCF-04 | The same calls with a `TaxedMoney` parameter and a `Money` argument; with a `Money` parameter and an `Other` argument; with two arguments for one parameter; `function.call` over a function with no `parameters` binding and one argument | `ill_typed`/`operator-ineligible` | FR-322-AC-31, FR-322-AC-69 |
| RCF-05 | The same calls with a `Sequence<Money>` parameter and a `Sequence<TaxedMoney>` argument; with `Option<Money>` and `Option<TaxedMoney>` | `ill_typed`/`operator-ineligible`: a `K<child>` type is not a `K<parent>` type | FR-322-AC-69 |
| RCF-07 | `collection.contains` over a `Set<Money>` and a `TaxedMoney` operand; over a `Set<TaxedMoney>` and a `Money` operand | Admitted; `ill_typed`/`operator-ineligible` | FR-322-AC-69 |
| RCF-08 | `collection.set` and `collection.sequence` with `result_type` `Set<Money>` and `Sequence<Money>` and rest arguments `Money` and `TaxedMoney`; with a `Set<TaxedMoney>` result and a `Money` rest argument | Admitted; `ill_typed`/`operator-ineligible` | FR-322-AC-69 |
| RCF-09 | A union value of a member with a `Money` payload and a `TaxedMoney` term; with a `Money` term at a `TaxedMoney` payload | Admitted; `ill_typed`/`type-mismatch` | FR-322-AC-69 |
| RCF-10 | `structural.eq` and `structural.ne` over two `Money` operands with the leaves of `Money`, where the values are `TaxedMoney` values whose `note` has no listed leaf | Admitted: the leaf check reads `Money`'s walk only | FR-322-AC-69 |
| RCF-11 | A `function` typed `Money` whose body root is an application typed `TaxedMoney`; a `function` typed `TaxedMoney` whose body is typed `Money` | Admitted; `ill_typed`/`type-mismatch` at the function node | FR-322-AC-69 |
| RCF-13 | A `function` typed `Money` whose `body` binding references a root expression typed `TaxedMoney`; typed `Money` with a `Voucher`-typed root (an unrelated record); typed `TaxedMoney` with a `Money` root | Admitted; `ill_typed`/`type-mismatch`; `ill_typed`/`type-mismatch` (locus: the function node) | FR-322-AC-69 |
| RCF-12 | A `case` with result type `Money` whose arm bodies are typed `Money` and `TaxedMoney`; with arm bodies `TaxedMoney` and `Other` | Admitted; `ill_typed`/`operator-ineligible` | FR-322-AC-69 |

## Expected Results

Every reference equality, upcast, projection, reaches, record conformance and event case gives exactly its
recorded result.

## Relationship ends

`model-member-type-vectors.json` `relationship_navigation_cases` apply
`quire.op.model.navigate` and `quire.op.model.reaches` over the relationship
ends of the selected document (FR-322-AC-57). A case that patches an end
multiplicity edits the selected document before it is read.

| ID | Case | Expected | Criterion |
| --- | --- | --- | --- |
| RN-01 | `navigate` `order` on M4, receiver M1, result `Reference<Order>` | Admitted | FR-322-AC-57 |
| RN-02 | `navigate` `next` on M4, receiver M1, result `Set<Order>` | `unsupported_construct`/`expression-form` at `member.name` | FR-322-AC-57 |
| RN-03 | `navigate` `next` on M4, receiver M1, result `Set<Order>`, with the end multiplicity patched | Admitted | FR-322-AC-57 |
| RN-04 | `navigate` `next` on M4, receiver M1, result `Bag<Order>`, with the end multiplicity patched | Admitted | FR-322-AC-57 |
| RN-05 | `navigate` `order` on M4, receiver M1, result `Option<Order>`, with the end multiplicity patched | Admitted | FR-322-AC-57 |
| RN-06 | `navigate` `order` on M4, receiver M5, result `Reference<Order>` | Admitted | FR-322-AC-57 |
| RN-07 | `navigate` `order` on M4, receiver M3, result `Reference<Order>` | `ill_typed`/`operator-ineligible` at `arguments/0` | FR-322-AC-57 |
| RN-08 | `navigate` `order` on M4, receiver M1, result `Option<Order>` | `ill_typed`/`operator-ineligible` at `result_type` | FR-322-AC-57 |
| RN-09 | `navigate` `follows` on M11, receiver M1, result `Reference<Order>` | Admitted | FR-322-AC-57 |
| RN-10 | `navigate` `followedBy` on M11, receiver M1, result `Set<Order>`, with the end multiplicity patched | `ill_typed`/`operator-ineligible` at `member.name` | FR-322-AC-57 |
| RN-11 | `navigate` `next` on M10, receiver M3, result `Option<Order>` | `missing_declaration`/`missing-name` at `member.name` | FR-322-AC-57 |
| RN-12 | `navigate` `twin` on M12, receiver M1, result `Reference<Order>` | `ambiguous_declaration`/`ambiguous-name` at `member.name` | FR-322-AC-57 |
| RN-13 | `navigate` `order` on M1, receiver M1, result `Reference<Order>` | `ill_typed`/`operator-ineligible` at `member.declaration` | FR-322-AC-57 |
| RN-14 | `navigate` `invoiced` on M10, receiver M1, result `Reference<Invoice>` | Admitted | FR-322-AC-57 |
| RN-15 | `reaches` `order` on M4, operands M1, M1 | Admitted | FR-322-AC-57 |
| RN-16 | `reaches` `order` on M4, operands M1, M3 | `ill_typed`/`operator-ineligible` at `arguments/1` | FR-322-AC-57 |
| RN-17 | `reaches` `order` on M4, operands M1, M1, with the end multiplicity patched | Admitted | FR-322-AC-57 |
| RN-18 | `reaches` `order` on M4, operands M1, M1, with the end multiplicity patched to `{0,3}` unique | Admitted | FR-322-AC-57 |
| RN-19 | `reaches` `invoiced` on M10, operands M1, M1 | `ill_typed`/`operator-ineligible` at `member.name` | FR-322-AC-57 |
| RN-20 | `reaches` `order` on M4, operands M1, M1, with the end multiplicity patched to `{0,3}` and `unique` false | Admitted: a `Bag`-shaped destination | FR-322-AC-57 |
| RN-21 | `reaches` `order` on M4, operands M1, M5, with the end multiplicity patched to `{0,3}` | Admitted: a subtype operand | FR-322-AC-57 |
| RN-22 | `reaches` `order` on M4, operands M1, M3, with the end multiplicity patched to `{0,3}` | `ill_typed`/`operator-ineligible` at `arguments/1` | FR-322-AC-57 |
| RN-23 | `reaches` `order` on M4, operands M1, M1, with the end multiplicity patched to `{0,unbounded}` | `unsupported_construct`/`expression-form` at `member.name` | FR-322-AC-57 |
| RN-24 | `reaches` `order` on M4, operands M1, M1, with the end multiplicity patched to `{0,3}` and `ordered` true | `unsupported_construct`/`expression-form` at `member.name` | FR-322-AC-57 |
| RN-25 | `reaches` `invoiced` on M10, operands M1, M1, with the end multiplicity upper bound patched to 3 | `ill_typed`/`operator-ineligible` at `member.name`: a multi-valued destination of another type | FR-322-AC-57 |
| RN-26 | `reaches` `invoiced` on M10, operands M1, M1, with the end multiplicity patched to `{0,unbounded}` | `unsupported_construct`/`expression-form` at `member.name`: multiplicity is decided before type | FR-322-AC-57 |
| RN-27 | `reaches` `followedBy` on M11, operands M1, M1, with the end multiplicity patched to `{0,unbounded}` | `ill_typed`/`operator-ineligible` at `member.name`: direction is decided before multiplicity | FR-322-AC-57 |
