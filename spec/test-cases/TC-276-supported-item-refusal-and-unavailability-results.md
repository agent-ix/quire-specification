---
id: TC-276
title: "Supported-item refusal and unavailability results"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-276: Supported-item refusal and unavailability results

## Description

Show that a supported item whose Kani run ends `Refused`, `InvalidInput` or
`IncompleteInput` records exactly one terminal FR-331 result `declined` with a
typed refusal cause, and that a supported item whose Kani run ends
`Unavailable` records exactly one terminal result `unsupported` with a typed
unavailability cause, per the QSL ADR-013 O-16 proof column.

## Preconditions

- AD-016 arrow 6 `KaniOutcomeKind` closed-set map (kinds 3-6: `Refused`,
  `InvalidInput`, `IncompleteInput`, `Unavailable`).
- CG #50: the CG replay adapter and parity comparator exist.

## Test Procedure

Build a Kani run outcome for each of the four kinds and map it through the
closed-set `KaniOutcomeKind` table into the FR-331 `results` vocabulary and
`accounting` record. Execute each vector through the real local Rust boundary
and read back the settled result, cause and accounting record count.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| RU-01 | A `Refused` Kani run on a supported item | Result `declined` with a typed refusal cause; one terminal accounting record | FR-331-AC-10 |
| RU-02 | An `InvalidInput` Kani run on a supported item | Result `declined` with a typed refusal cause; one terminal accounting record | FR-331-AC-10 |
| RU-03 | An `IncompleteInput` Kani run on a supported item | Result `declined` with a typed refusal cause; one terminal accounting record | FR-331-AC-10 |
| RU-04 | An `Unavailable` Kani run on a supported item (solver or backend absent after negotiation) | Result `unsupported` with a typed unavailability cause; one terminal accounting record | FR-331-AC-10 |
| RU-05 | Enumerate the closed `results` vocabulary after RU-01..04 | Exactly the eight existing values; `declined` is not a variant of `failed` or `incomplete` | FR-331-AC-10 |

## Expected Results

RU-01 through RU-03 find the refusal category settling result `declined`.
RU-04 finds the unsupported category settling result `unsupported`, the same
value a tool-absence item records. RU-05 finds no new result value beyond
`declined` and no collapse into another category.
