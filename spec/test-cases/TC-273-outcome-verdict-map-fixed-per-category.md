---
id: TC-273
title: "Outcome-to-verdict map fixed per O-16 category"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: verifies
---

# TC-273: Outcome-to-verdict map fixed per O-16 category

## Description

Show that the replay outcome-to-verdict map assigns one verdict per QSL
ADR-013 O-16 category independently of the request, and that an `Undefined`
native outcome and a refused `FamilyOutcome` never settle agreement.

## Preconditions

- QSL ADR-013 TK-01: the QSL layer-6 `replay` facade exists.
- The QSL #231 replay envelope carries an expected originating verdict.
- `FamilyOutcome` and its `Refused` case are the QSL ADR-011 #209 FND-008
  S6a-stage result type (`Result<FamilyOutcome, InternalFault>`).

## Test Procedure

Build FR-323 `NativeRuntimeEnvelope` requests that differ only in
non-mapping fields (package, selection, state environment) while holding the
native outcome and its O-16 category fixed. Execute each vector through the
real local Rust boundary and read back the settled verdict.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| OV-01 | Two otherwise-distinct requests whose native outcome falls in the same O-16 category | Both settle the same verdict for that category | FR-323-AC-4 |
| OV-02 | The same request replayed twice with the same native outcome | The settled verdict is identical both times; no per-request override exists | FR-323-AC-4 |
| OV-03 | A native outcome of `Undefined`, with the envelope's expected originating verdict set to match it | Settles no agreement; no backend-evidence verdict is constructed | FR-323-AC-4 |
| OV-04 | A native outcome that is a refused `FamilyOutcome` (its `Refused` case), with the envelope's expected originating verdict set to match it | Settles no agreement; no backend-evidence verdict is constructed | FR-323-AC-4 |

## Expected Results

OV-01 and OV-02 find the map fixed per category and independent of request
identity. OV-03 and OV-04 find that `Undefined` and a refused `FamilyOutcome`
never settle agreement and construct no backend-evidence verdict, regardless
of the expected originating verdict.
