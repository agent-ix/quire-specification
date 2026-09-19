---
id: TC-272
title: "Replay source structural separation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-197
    type: verifies
---

# TC-272: Replay source structural separation

## Description

Show by type structure that an `Input`-sourced replay settles
`reproduced-without-witness` and never becomes a backend-evidence verdict, that
only a `Witness`-sourced replay constructs one, and that no packet or
replay-result type stores a witness-backed field.

## Test Procedure

Build packets through the real local Rust boundary: one `ReplaySource::Witness`
packet whose transcript is the AD-016 seed counterexample, and one
`ReplaySource::Input` packet carrying the same decoded values (8, 992). Replay
each through the CG replay adapter against the QSL layer-6 `replay` facade.
Run the compile-fail vectors as `compile_fail` doctests or `trybuild` cases,
each asserting the compiler error names the rejected type. Run the runtime
vectors as ordinary tests.

## Vectors

| Vector | Mutation | Expected result |
| --- | --- | --- |
| RS-01 | Replay the `Witness` packet with an executor that reproduces the counterexample | A `Witness`-arm result; a backend-evidence verdict is constructed from it |
| RS-02 | Replay the `Input` packet with an executor that reproduces the counterexample | An `Input`-arm result settling `reproduced-without-witness`; no backend-evidence verdict exists |
| RS-03 | Replay the `Input` packet with a stub executor that returns the expected answer for any input | Still `reproduced-without-witness`; the stub cannot produce a backend-evidence verdict |
| RS-04 | Compile-fail: pass an `Input`-arm result to each function parameter that accepts a backend-evidence verdict | Does not compile |
| RS-05 | Compile-fail: store an `Input`-arm result in each field that holds a backend-evidence verdict, and return one from each function whose return type is one | Does not compile |
| RS-06 | Compile-fail: call every public constructor of the backend-evidence verdict with an `Input`-arm result, and with bare values | Does not compile |
| RS-07 | Enumerate the fields of the packet and of every replay-result type through their serialized form and a destructuring pattern with no `..` | No witness-backed field exists |
| RS-08 | Ask whether each packet is witness-backed | `true` for the `Witness` packet, `false` for the `Input` packet, both from a match on `ReplaySource` |
| RS-09 | Deserialize a packet that carries both a transcript and stored values | Refused; a packet holds exactly one arm |

## Expected Results

RS-01 to RS-03 and RS-08 return exactly the listed result. RS-04 to RS-06 fail
to compile with the rejected type named. RS-07 finds no witness-backed field.
RS-09 refuses with a typed cause before any replay runs.
