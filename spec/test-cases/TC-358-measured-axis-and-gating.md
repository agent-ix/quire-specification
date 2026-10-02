---
id: TC-358
title: "Measured axis, result combination and pipeline gating"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-409
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-410
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272
    type: verifies
---

# TC-358: Measured axis, result combination and pipeline gating

## Description

Check the closed `measured` vocabulary, its exclusion from truth and proof accounting, and its pipeline gating.

## Test Procedure

Run each vector through the real local Rust boundary of the provider named by the requirement, and compare the complete typed result (value, cause and payload) with the expected result. Use the strict result reader for MA-03.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| MA-01 | Results from SC-01, SC-04 and an undecided run | `measured` values `accepted`, `rejected`, `undecided` with execution `completed`, no truth and no basis; the full `StatisticalVerdict` is present |
| MA-02 | A request with one `accepted` and one `rejected` item | The request does not pass; the rejected item counts as a violation, never as `refuted` |
| MA-03 | A result carrying `measured` together with truth `satisfied`; a verdict missing `basis` | Refused `invalid_result`/`measured-with-truth`; `invalid_result`/`missing-member` |
| MA-04 | Proof accounting over MA-01 | No `measured` value counts as proof evidence |
| MA-05 | A run stopped by a budget, and an unsupported item | Execution `resource-incomplete` and `unsupported`, no `measured` value |
| MA-06 | An SPRT verdict and an Okamoto verdict | `interval` absent for SPRT and present for Okamoto |
| MA-07 | Results carrying `NotMarkov`, `ExactValue` and `Incomplete(ResourceExhausted)` in place of `not-markov`, `exact-value` and `limit-reached`; and a result carrying each FR-331 wire-spelling table value | Each PascalCase or other spelling is refused `invalid_result`/`unknown-cause` with the received spelling; every table value is read |

## Expected Results

Every result combination is exactly as stated, and a measurement never reads as truth or proof.
