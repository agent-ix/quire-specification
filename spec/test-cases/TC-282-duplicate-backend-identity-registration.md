---
id: TC-282
title: "Duplicate backend identity registration"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272
    type: verifies
---

# TC-282: Duplicate backend identity registration

## Description

Register one backend identity more than once, with the same manifest and with
conflicting manifests, and check that the registry snapshot, its registration
refusals and every candidate set depend only on the set of registrations,
never on their order.

## Test Procedure

Build fixture backend descriptors, each a backend identity, its options and a
set of (kind, mode) pairs. Let `A1` and `A2` be two manifests for identity `a`
advertising (`value-validity`, `bounded`) that differ only in one option, with
`A1`'s RFC 8785 bytes ordered before `A2`'s. Let `A1'` be `A1` advertising
(`value-validity`, `unbounded`) instead, ordered after `A1`. Let `B` be a
manifest for identity `b` advertising (`value-validity`, `bounded`). Let `M`
be a registration for identity `a` advertising the mode `finite`.

Build a registry for every ordering of each vector's registrations through the
real local Rust boundary. For each snapshot, record its held registrations, its
registration refusals in reported order with their catalog code and cause, and
the candidate set of a `value-validity` item that names no backend, one that
names `a` and one that names `b`. Compare every ordering's record with the
first ordering's record and with the expected result.

## Vectors

| Vector | Registrations | Expected result |
| --- | --- | --- |
| DB-01 | `A1`, `A1` | `a` held once with `A1`; no refusal; the unnamed item's candidates are exactly `a`; the snapshot equals one registration of `A1` |
| DB-02 | `A1`, `A1`, `A1`, `B` | As DB-01, with `b` also held; the unnamed item's candidates are `a` then `b`, in candidate order |
| DB-03 | `A1`, `A2` | Two refusals, `invalid_capability`/`duplicate-backend` for (`a`, `A1`) then (`a`, `A2`); `a` is not held; the unnamed item's candidates are empty; the item naming `a` receives the unknown-backend mark carrying `a` |
| DB-04 | `A1`, `A2`, `B` | As DB-03 for `a`; `b` is held; the unnamed item's candidates are exactly `b`; the item naming `b` holds `b` |
| DB-05 | `A1`, `A2`, `A1` | As DB-03; the second `A1` is refused with the first, and no third refusal key is reported |
| DB-06 | `A1`, `A1'` | Two refusals, `invalid_capability`/`duplicate-backend` for (`a`, `A1`) then (`a`, `A1'`); `a` is not held; the item naming `a` receives the unknown-backend mark |
| DB-07 | `A1`, `M` | `M` refuses as `invalid_capability`/`unknown-mode` for `a`; it is not admitted, so `a` is held once with `A1` and no `duplicate-backend` is reported |
| DB-08 | Register `A1` and read the snapshot; register `A2` and read the snapshot again; repeat with `A2` first | The first snapshot holds `a` with the first registration's manifest; the second equals DB-03's snapshot, with the first registration withdrawn and its refusal reported |

## Expected Results

Every ordering of a vector's registrations gives the same snapshot, the same
refusals in the same reported order and the same candidate set for every
item. A repeat of an identical manifest is never refused. A conflicting
identity is never held, contributes no candidate and never refuses a
registration of another identity. Each `duplicate-backend` refusal retains the
repeated backend identity and its own manifest.
