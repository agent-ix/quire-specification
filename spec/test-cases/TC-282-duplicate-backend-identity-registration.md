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

Register one backend identity more than once and check that the registry snapshot, its registration
refusals and every candidate set depend only on the set of registrations,
never on their order.

## Test Procedure

Build fixture backend descriptors, each a backend identity, its options and a
set of (kind, mode) pairs. Let `A1` and `A2` be two manifests for identity `a`
advertising (`value-validity`, `bounded`) that differ only in one option. Let `A1'` be `A1` advertising
(`value-validity`, `unbounded`) instead. Let `B` be a
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
| DB-01 | `A1`, `A1` | One refusal, `invalid_capability`/`duplicate-backend` for `a`; `a` is not held; the unnamed item's candidates are empty |
| DB-02 | `A1`, `A1`, `A1`, `B` | As DB-01 for `a`; `b` is held; the unnamed item's candidates are exactly `b` |
| DB-03 | `A1`, `A2` | One refusal, `invalid_capability`/`duplicate-backend` for `a`; `a` is not held; the unnamed item's candidates are empty; the item naming `a` receives the unknown-backend mark carrying `a` |
| DB-04 | `A1`, `A2`, `B` | As DB-03 for `a`; `b` is held; the unnamed item's candidates are exactly `b`; the item naming `b` holds `b` |
| DB-05 | `A1`, `A2`, `A1` | As DB-03; no second refusal is reported for `a` |
| DB-06 | `A1`, `A1'` | As DB-03 |
| DB-07 | `A1`, `M` | `M` refuses as `invalid_capability`/`unknown-mode` for `a`; it is not admitted, so `a` is held once with `A1` and no `duplicate-backend` is reported |
| DB-08 | Register `A1` and read the snapshot; register `A2` and read the snapshot again; repeat with `A2` first | The first snapshot holds `a`; the second equals DB-03's snapshot, with the first registration withdrawn and the identity's refusal reported |

## Expected Results

Every ordering of a vector's registrations gives the same snapshot, the same
refusals in the same reported order and the same candidate set for every
item. A repeated identity is never held, contributes no candidate and never refuses a
registration of another identity. Each `duplicate-backend` refusal retains the
repeated backend identity.
