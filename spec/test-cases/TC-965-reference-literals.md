---
id: TC-965
title: "Reference literals parse, type, lower and resolve against a subject"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-480
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-965: Reference literals parse, type, lower and resolve against a subject

## Description

Check that a reference literal `M::T#"k"` parses as its own production, types
`Reference<M::T>`, lowers to a `reference` literal term, resolves once at
subject binding to an admitted key, and is refused at its name, its string or
the literal for each stated cause. Scope: FR-480-AC-1 to FR-480-AC-5 and
FR-322-AC-101.

Fixture **S**: a domain package `M` with object type `Server` (field
`load: Int[0, 3]`), record type `Point`, process `Flow` and population
`servers` of `Server`, and an operation `spawn()` that creates a `Server`; a unit selecting it through aliases `M` and `N`; a
subject whose `servers` universe in `Server`'s object universe is `[s1, s2]`,
both admitted by an initial-state member record as `Server`, and whose
universe also lists `s3`, which no initial member record admits and an operation `spawn()` creates as a `Server`.

## Test Procedure

Check each vector through the real local Rust boundary of the parser,
checker, producer, reader or analyzer the requirement names, and compare the
typed result (node, type, term, value or refusal code, cause, locus and
retained members) with the expected result.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| RL-01 | `M::Server#"s1"`; `M::Server#s1`; `M::Server#`; `#"s1"` | one `reference-literal` node holding the qualified name and the string, rendered back byte for byte; each of the other three a located parser syntax diagnostic |
| RL-02 | `self = M::Server#"s1"` in an invariant of `Server`; `M::Absent#"s1"`; `M::Point#"s1"`; `M::Server#""` | the literal types `Reference<M::Server>` with the same type node as a `Reference<M::Server>` parameter and the invariant checks; `missing_declaration`/`missing-name` at `M::Absent`; `ill_typed`/`type-mismatch` at `M::Point` with expected `object type` and actual `record`; `invalid_package`/`invalid-value` at the string |
| RL-03 | `M::Server#"s1"` and `N::Server#"s1"` in one unit | one `literal` term `{type: <reference node over Server>, value_kind: "reference", value: "7331"}` shared by both occurrences |
| RL-04 | subject **S** with RL-02's invariant; `M::Server#"s9"`; `M::Server#"s3"` | `s1`'s literal is the FR-181 reference with `Server`'s object universe, `Server`'s type identity and identity `7331`, and the invariant holds for `s1` only; `missing_declaration`/`missing-name` at the `s9` literal naming `Server` and `7339` before any state is explored; the `s3` literal undefined with reason `absent-key` at the initial state and `s3`'s reference after an operation creates `s3` as a `Server` |
| RL-05 | a choice guard comparing a `receive` record's `Reference<M::Server>` field to `M::Server#"s1"`; the same with `M::Server#"s3"`; a guard reading `deref(M::Server#"s1").load` | the first checks; the second refuses `ill_typed`/`operator-ineligible` at the literal when **S** is bound; the third refuses `ill_typed`/`operator-ineligible` at the dereference |
| RL-06 | a package `literal` of `value_kind` `reference` typed by the reference node over `Server`, `value` `7331`; the same with `value` `7G31`, `733`, `73A1` and empty text; typed by the reference node over `Flow`; typed by an integer node | admitted with family `reference` by the reader and by both V2 schemas; `invalid_package`/`invalid-value` at `value` for the four values; `ill_typed`/`operator-ineligible` for the two types |

## Expected Results

Every node, type, term, value and refusal is exactly as stated, each refusal
read by its typed fields and never from a message.
