---
id: TC-430
title: "Node-key preimage integers keep one exact spelling at every magnitude"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-430: Node-key preimage integers keep one exact spelling at every magnitude

## Description

Checks the published `integer_encoding_vectors` of
`proposals/checked-package-v2/node-identity-vectors.json` against FR-322's
rule for integers in a node-key preimage: an integer value is a canonical
decimal string at every magnitude, and a counter is a JCS number up to
2^53-1 and a canonical decimal string beyond it. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| NI-01 | `recursion-size-safe-maximum`: the `integer-add` operation vector's preimage with `recursion` `{size: 9007199254740991, ordinal: 9007199254740990}` | The JCS SHA-256 of the preimage is the recorded `bc6fe35a2c34a7162fe0ed0a20473ffb3284bd70236f96feb75b386c55992874`; the preimage validates as `ApplicationNode`, and both counters are JSON numbers in its canonical bytes | FR-322-AC-58 |
| NI-02 | `recursion-size-two-pow-53`: the same preimage with `recursion` `{size: "9007199254740992", ordinal: 9007199254740991}` | The JCS SHA-256 is the recorded `89ec1ac87a8b82a05d5414f0c3b314f363e49328988ff5d8c3fc9a88014839bc`; the preimage validates as `ApplicationNode`; its canonical bytes hold `"size":"9007199254740992"` and `"ordinal":9007199254740991` | FR-322-AC-58 |
| NI-03 | NI-02's preimage with `size` 9007199254740992 written as a JSON number, and NI-01's with `size` written as the string `"9007199254740991"` | The schema refuses both | FR-322-AC-58 |
| NI-04 | `integer-range-u64-maximum`: the `quire.structural-node/v1` `bounded_domain`/`integer_range` preimage of `Int[0, 18446744073709551615]`, typed at the builtin `Integer` node | The JCS SHA-256 is the recorded `f9ea36c52345743ae98ce1c1449610c7e955d986a3ecb08597ca427c6f1b4230`, and its canonical bytes hold `"value":"18446744073709551615"` | FR-322-AC-58 |
| NI-05 | Every `operation_vectors` entry | Each recomputes its recorded digest unchanged | FR-322-AC-58 |

## Expected Results

Every integer in a node-key preimage has exactly one spelling, no preimage
integer is refused for its magnitude, and no published key changes.
