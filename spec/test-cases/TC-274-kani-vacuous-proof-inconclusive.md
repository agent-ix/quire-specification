---
id: TC-274
title: "Vacuous Kani proof settles inconclusive"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-274: Vacuous Kani proof settles inconclusive

## Description

Show that a Kani `Proved` run whose obligation has zero SUCCESS checks
records the FR-331 item result as `inconclusive` with a typed vacuity cause,
never as `proved`, and that no eighth `results` value exists for it.

## Preconditions

- CG #50: the CG replay adapter and parity comparator exist.
- AD-016 arrow 6 `KaniOutcomeKind` closed-set map (kind 1 `Proved`, kind 10
  `Inconclusive`).

## Test Procedure

Build a Kani run outcome carrying a `Proved` disposition and a SUCCESS-check
count, and map it through the closed-set `KaniOutcomeKind` table into the
FR-331 `results` vocabulary. Execute each vector through the real local Rust
boundary and read back the settled result and its cause.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| VC-01 | A `Proved` run with zero SUCCESS checks in the obligation | Item result `inconclusive` with the typed cause `kani_vacuous_proof` | FR-331-AC-8 |
| VC-02 | A `Proved` run with one or more SUCCESS checks in the obligation | Item result `proved`; no vacuity cause | FR-331-AC-8 |
| VC-03 | Enumerate the closed `results` vocabulary that the vacuity cause maps into | Exactly the seven existing values (`proved`, `refuted`, `tested`, `inconclusive`, `unsupported`, `incomplete`, `failed`); the vacuity cause is carried inside `inconclusive`, not an eighth value | FR-331-AC-8 |

## Expected Results

VC-01 finds a vacuous `Proved` run recorded as `inconclusive` with the typed
`kani_vacuous_proof` cause. VC-02 finds a non-vacuous `Proved` run recorded
as `proved`. VC-03 finds the `results` vocabulary unchanged at seven values.
