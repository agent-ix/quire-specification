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

Recompute union member keys from the `quire.union-member-node/v1` preimage and
show that members of different unions, and enum members, never share a key,
and that the key binds the member identifier byte for byte. The vectors are
`union_member_vectors` in `proposals/checked-package-v2/node-identity-vectors.json`,
and each preimage validates against `UnionMemberPreimage` in
`node-identity-preimage.schema.json`.

## Preconditions

- The canonical JCS/SHA-256 rule of FR-322 node identity preimages.
- A union node `U1` whose `node_id.digest` is 64 `b` characters, a union node
  `U2` whose `node_id.digest` is 64 `c` characters, both declaring a member
  `Circle`, `U1` also declaring `Empty`, and an enum declaring `Circle`.

## Test Procedure

Build each preimage, serialize it to RFC 8785 bytes, hash it with SHA-256 and
compare the unprefixed lowercase hex digest.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| K01 | `U1`, `Circle` | bytes `{"declaration_node_id":{"digest":"bbbb…bbbb","domain":"quire.checked-semantic-node/v1"},"member":"Circle","version":"quire.union-member-node/v1"}`; digest `40a8492e5616a286cae9977ee9a7d5acab89cc50eb75654df0b8ad1096efd692` |
| K02 | `U1`, `Empty` | digest `83c93448259809db143345be4182a358f92481032ad0ae3b782a482d3897296b` |
| K03 | `U2`, `Circle` | digest `611ebd3f928ce6ca8713c9400e4725af94e51d96f0aaa5ef437ebed8c88ae75c`, unequal to K01 |
| K04 | the enum member `Circle` under `quire.enum-member-node/v1` | a digest unequal to K01 and K03 |
| K05 | K01's preimage with `member` changed to `circle`; K01's preimage with `declaration_node_id` changed to `U2`'s | a digest unequal to K01; exactly K03's digest |

## Expected Results

Every digest equals the stated vector, keys of different unions and of enum
members differ, and changing the member identifier's case or the union changes
the key.
