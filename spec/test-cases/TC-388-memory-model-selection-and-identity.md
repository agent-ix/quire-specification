---
id: TC-388
title: "Memory model selection, nesting and identity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-434
    type: verifies
---

# TC-388: Memory model selection, nesting and identity

## Description

Check the `memory` clause's source default, the request's `memory` member,
nesting rules, the weak join-policy rule, the resolved model in the
obligation identity, and that `sc` is the interleaving meaning.

## Test Procedure

Use the store-buffering protocol `SB` of TC-390 and the variants below.
Submit each request through the real local Rust boundary and record its
refusal, its resolved models, its obligation identity and its verdict.

## Vectors

| Vector | Source and request | Expected result |
| --- | --- | --- |
| MS-01 | `parallel Both memory sc`, no `memory` member | resolved `sc` |
| MS-02 | `parallel Both memory sc`, `memory: [{parallel: "SB::Both", model: "tso"}]` | resolved `tso`; identity differs from MS-01 |
| MS-03 | `parallel Both memory tso`, no `memory` member | resolved `tso`; obligation identity and verdict equal MS-02's; package identity differs from MS-02's |
| MS-04 | `parallel Both` with no `memory` clause | resolved `sc`; states, steps and verdict equal MS-01's; every state key's `memory` member is empty |
| MS-05 | a nested `parallel Inner memory ra` inside `parallel Outer memory tso` | refuses `invalid_model_binding`/`conflicting-binding` naming `Outer` and `Inner` |
| MS-06 | `parallel Outer memory tso` with nested `parallel Inner` and no clause, request selecting `ra` for `Outer` | both resolve to `ra` |
| MS-07 | a request entry naming `SB::Both::left` (a branch, not a `parallel`), one naming a nested `parallel`, and one naming `SB::Both` twice | each refuses the request `invalid_runtime_input`/`invalid-value` naming the entry |
| MS-08 | `parallel Both memory tso { … } join any [left, right];` | refuses `unsupported_construct`/`declaration-form` at the join |
| MS-09 | `parallel Both { … } join any [left, right];` with source default `sc`, request selecting `tso` | settles `unsupported`, cause `weak-access-shape`, shape `join-policy` |
| MS-10 | two sequential `parallel` controls, the first `memory tso` and the second `memory sc` | each governs its own branches; after the first `join all` memory holds one value per location and no buffer exists |

## Expected Results

Every refusal, resolved model, identity comparison and verdict matches the
table.
