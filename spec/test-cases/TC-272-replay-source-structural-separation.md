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
only an agreeing `Witness`-sourced replay constructs one, that a disagreeing
replay of either arm settles `inconclusive`, and that no packet or
replay-result type stores a witness-backed field.

## Preconditions

- QSL ADR-013 TK-01: the QSL layer-6 `replay` facade exists.
- QSL ADR-013 TK-04: IR admits a `Witness` only through `Witness::parse` and
  the packet carries `ReplaySource`.
- QSL ADR-013 TK-07: the FR-201 obligation-identity digest domain exists.
- CG #50: the CG replay adapter and parity comparator exist.
- WP9 vendors the seed counterexample vector.

## Fixture

The `Witness` packet's transcript is the IR `tests/kani_replay.rs` tc_042
witness transcript at IR `954c2f2`, whose concrete values match the AD-016 seed
counterexample vector. The `Input` packet carries the same decoded values
(8, 992) as canonical assignments keyed by parameter `WireNodeId`.

## Test Procedure

Build packets through the real local Rust boundary and replay each through the
public `Witness` replay entry or the CG replay adapter against the QSL layer-6
`replay` facade. Run the compile-fail vectors as `trybuild` cases with committed
`.stderr` snapshots that name the rejected type, each paired with a
compile-pass control that differs only in passing an accepted type. Take the
closure of positions, constructors and types from a public-API listing of the
replay crates, so a new position without a vector fails the test. Run the
runtime vectors as ordinary tests.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| RS-01 | Replay the `Witness` packet with an executor that reproduces the counterexample | A `Witness`-arm result settling `reproduced-with-evaluated-witness`; a backend-evidence verdict is constructed from it | FR-197-AC-7, FR-197-AC-9 |
| RS-02 | Replay the `Input` packet with an executor that reproduces the counterexample | An `Input`-arm result settling `reproduced-without-witness`; no backend-evidence verdict exists | FR-197-AC-6 |
| RS-03 | Replay the `Input` packet with a stub executor that returns the expected answer for any input | Still `reproduced-without-witness`; the stub cannot produce a backend-evidence verdict | FR-197-AC-6, FR-197-AC-8 |
| RS-04 | Compile-fail: pass an `Input`-arm result to each function parameter that accepts a backend-evidence verdict | Does not compile; snapshot names the rejected type | FR-197-AC-8 |
| RS-05 | Compile-fail: store an `Input`-arm result in each field that holds a backend-evidence verdict, and return one from each function whose return type is one | Does not compile; snapshot names the rejected type | FR-197-AC-8 |
| RS-06 | Compile-fail: build a backend-evidence verdict from an `Input`-arm result, from bare values, by struct literal, through `Default`, `From`, `TryFrom` and deserialization | Does not compile; the listing shows one construction path, taking a `Witness`-arm result | FR-197-AC-9, FR-197-AC-10 |
| RS-07 | Enumerate the fields of the packet and of every replay-result type through their serialized form and a destructuring pattern with no `..` | No witness-backed field exists | FR-197-AC-11 |
| RS-08 | Ask whether each packet is witness-backed | `true` for the `Witness` packet, `false` for the `Input` packet, both from a match on `ReplaySource` | FR-197-AC-12 |
| RS-09 | Deserialize a packet that carries both a transcript and stored values | Refused with a typed cause before any replay runs | FR-197-AC-14 |
| RS-10 | Deserialize a packet that carries neither a transcript nor stored values | Refused with a typed cause before any replay runs | FR-197-AC-14 |
| RS-11 | Replay the `Witness` packet with an executor that does not reproduce the counterexample | `inconclusive` with a typed cause; no backend-evidence verdict | FR-197-AC-13 |
| RS-12 | Replay the `Input` packet with an executor that does not reproduce the counterexample | `inconclusive` with a typed cause; no backend-evidence verdict | FR-197-AC-13 |
| RS-13 | Deserialize a `Witness` whose transcript is a cover playback, has no assertion block, or has two | Refused by `Witness::parse`; no `Witness` value exists | FR-197-AC-15 |
| RS-14 | Replay the `Witness` packet through the public `Witness` replay entry with a recording QSL `replay` facade | The facade records one call whose decoded arguments are (8, 992); the entry's signature takes no executor parameter | FR-197-AC-16 |
| RS-15 | Compile-fail: build a `Witness`-arm result that holds an `Input` source, and destructure each replay-result type with no `..` | Does not compile; no replay-result type has a `ReplaySource` field | FR-197-AC-17 |
| RS-16 | Read back the `native-run-result/2` record of RS-02, and the same record with its arm member altered to `Witness` | The first decodes into the `Input`-arm result type; the altered record refuses with a typed cause; neither yields a backend-evidence verdict | FR-197-AC-18 |

## Expected Results

RS-01 to RS-03, RS-08, RS-11, RS-12 and RS-14 return exactly the listed
result. RS-04 to RS-06 and RS-15 fail to compile with the rejected type named,
and each compile-pass control compiles; they are Compile evidence run under the
test harness. RS-07 finds no witness-backed field. RS-09, RS-10 and RS-13
refuse with a typed cause before any replay runs. RS-16 decodes the unaltered
record into the `Input`-arm result type and refuses the altered record with a
typed cause.
