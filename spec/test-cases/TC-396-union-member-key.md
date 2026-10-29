---
id: TC-396
title: "Union member key"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-441
    type: verifies
---

# TC-396: Union member key

## Description

Compare union member keys, the pair (`declaration_node_id`, `member`), and
show that members of different unions, and enum members, never share a key,
and that the key binds the member identifier byte for byte.

## Preconditions

- Two union nodes `U1` and `U2`, both declaring a member `Circle`, `U1` also
  declaring `Empty`, and an enum declaring `Circle`.

## Test Procedure

Form each member key as the pair of its union's `node_id` and the member
identifier, and compare the pairs componentwise.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| K01 | `U1`, `Circle` and `U1`, `Empty` | unequal keys |
| K02 | `U2`, `Circle` | unequal to K01's `U1`, `Circle` |
| K03 | the enum member `Circle` | unequal to both union `Circle` keys |
| K04 | `U1`, `Circle` with `member` changed to `circle`; `U1`, `Circle` with the union changed to `U2` | unequal to `U1`, `Circle`; the second equals K02's key |
| K05 | the member `Both` and the nullary member `Empty` of the model union `Shape` (a `model`/`variant_type` node) | unequal to each other and to K01 to K03 |

## Expected Results

Keys of different unions and of enum members differ, and changing the member
identifier's case or the union changes the key.
