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
object types conform one to the other ("Reference conformance"), and
`quire.op.record.project` of a model field over a `quire.op.model.deref`
result, whose family is `object` and which the `field_owner` operand group
admits.

## Preconditions

- `proposals/checked-package-v2/model-member-type-vectors.json` carries
  TC-280's domain package `acme/orders` `1.0.0`, the model declaration node
  vectors M1 to M9, and the `reference_equality_cases` and
  `projection_cases`. In a case, `{reference: M}` is an operand of type
  `Reference<M>`, the `composite_type`/`reference` node over node vector `M`,
  and `{object: M}` is an operand of type `M` itself. A projection case
  also carries the projection's declared `result_type`.
- `contracts/checked-operation-catalog-v1.json` in `agent-ix/quire-verification-contracts` supplies the
  `quire.op.reference.eq`, `quire.op.reference.ne`, `quire.op.model.deref`
  and `quire.op.record.project` entries: their operand positions, groups,
  result form and constraints are read from the catalog.
- The lock selects `acme/orders` `1.0.0` only. In that package `Sub` (M5)
  declares supertype `Order` (M1), `SubSub` (M7) declares supertype `Sub`,
  `Both` (M6) declares supertypes `Tagged` (M9) and `Coded`, `Flow` (M8) is
  a systems interface with no declared supertype, and `Invoice` (M3) and
  `Both` are unrelated to `Order` and `Sub`.

## Test Procedure

For each reference equality case, fit each operand's family to the entry's
operand position, then decide each constraint the entry names; for
`conforming_reference`, look up both declarations by FR-322 step 2 from the operands' wire owners and decide
conformance along declared supertypes, for one type node as for two. For each projection case, check a
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
| RC-08 | `reference.eq` over `Reference<Order>` and a reference to the relationship `billedTo` (M4) | `ill_typed`/`operator-ineligible` | FR-322-AC-33 |
| RC-09 | `reference.eq` over a reference to M2 (`Order` keyed under the unselected version `2.0.0`) and `Reference<Order>` | `missing_declaration`/`missing-selection` | FR-322-AC-33 |
| RC-10 | `reference.eq` over an `Order` object operand and `Reference<Order>` | `ill_typed`/`operator-ineligible`: family `object` is not `reference` | FR-322-AC-33 |
| RC-11 | `reference.eq` over two references to the relationship `billedTo` (M4) | `ill_typed`/`operator-ineligible` | FR-322-AC-33 |
| RC-12 | `reference.eq` over two references to M2 | `missing_declaration`/`missing-selection` | FR-322-AC-33 |
| RC-13 | `reference.eq` over `Reference<Both>` and `Reference<Tagged>` | Admitted: `Both` conforms to its supertype `Tagged` | FR-322-AC-33 |
| RC-14 | `reference.ne` over `Reference<Tagged>` and `Reference<Both>` | Admitted | FR-322-AC-33 |
| RC-15 | `reference.eq` over `Reference<Flow>` and `Reference<Order>` | `ill_typed`/`operator-ineligible`: no declared supertype chain | FR-322-AC-33 |
| RC-16 | `reference.eq` over two `Reference<Flow>` operands | Admitted | FR-322-AC-33 |
| RP-01 | `record.project` of field `total` of M5 over `deref` of `Reference<Sub>` typed M5 | Resolves the inherited `Order/total`; type `Int[0, 9]` | FR-322-AC-34 |
| RP-02 | `record.project` of field `invoice` of M1 over `deref` of `Reference<Order>` typed M1 | Resolves `Order/invoice`; type `Reference<Invoice>` over M3 | FR-322-AC-34 |
| RP-03 | `record.project` of field `total` of M5 over a `Reference<Sub>` operand with no `deref` | `ill_typed`/`operator-ineligible` | FR-322-AC-34 |
| RP-04 | `record.project` of field `total` of M1 over `deref` of `Reference<Sub>` typed M5 | `ill_typed`/`operator-ineligible`: `member_of` fails | FR-322-AC-34 |
| RP-05 | `deref` of `Reference<Sub>` declared as M1 | `ill_typed`/`operator-ineligible` | FR-322-AC-34 |
| RP-06 | `deref` of a reference to M4 typed M4 | `ill_typed`/`operator-ineligible`: a relation node has no family | FR-322-AC-34 |
| RP-07 | `record.project` of field `total` of M2 over `deref` of a reference to M2 typed M2 | `missing_declaration`/`missing-selection` | FR-322-AC-34 |
| RP-08 | RP-01 declared with `result_type` `Integer` | `ill_typed`/`operator-ineligible`: not the member type `Int[0, 9]` | FR-322-AC-34 |
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

## Expected Results

Every reference equality, projection and reaches case gives exactly its
recorded result.
