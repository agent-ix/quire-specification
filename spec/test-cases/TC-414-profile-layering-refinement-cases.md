---
id: TC-414
title: "Profile-layering refinement cases"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-453
    type: verifies
---

# TC-414: Profile-layering refinement cases

## Description

Run the profile-layering refinement gate over cases on the V1 edge from
`quire.value.complete/v1` to `quire.model.complete/v1`, compiling each
case's unit under both roots, and confirm the case results, the verdict and
the exit code.

## Test Procedure

Build a test-only corpus. Each case is a parent unit whose `profile`
declaration selects the value root and a child unit, byte-equal outside its
`profile` declaration, that selects the model root under the same alias.
Run the gate through the real local Rust boundary and read every case
result, the verdict and the exit code. Each expected result is a literal in
the test.

## Vectors

| Vector | Case | Expected result |
| --- | --- | --- |
| PL-01 | A unit that both roots refuse with a typed refusal (an integer range proof that fails) | Holds, reporting both codes |
| PL-02 | A unit using a model form, prohibited by the value root and admitted by the model root | Not applicable |
| PL-03 | PL-01's case with a test-only fault in the child side's compile that skips the failing range proof, so the child admits the unit | Regression naming the case, the edge and the parent's codes; verdict violation, exit 10 |
| PL-04 | Two units that differ in one byte of a declaration body | Tool failure naming the case; exit 30 |
| PL-05 | A case naming state core as parent and state queries as child | Tool failure naming the case; exit 30 |

## Expected Results

Each vector gives exactly its listed case result, verdict and exit code.
Both sides of every case are compiled; no case carries a recorded parent
outcome.
