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

Run the profile-layering refinement gate over cases on all five AD-003
`requires` edges E1 to E5, compiling each case's unit under both layers, and
over each layer's witness unit and each edge's distinguishing unit. Confirm
the case, layer and edge results, the verdict and the exit code.

## Test Procedure

Build a test-only corpus. Each case is a parent unit whose `profile`
declaration names the parent layer and a child unit, byte-equal outside that
declaration's identity string, that names the child layer under the same
alias. Each layer has a witness unit selecting only that layer, and each edge
a distinguishing unit using one form the child adds. Run the gate through the
real local Rust boundary and read every result, the verdict and the exit
code. Each expected result is a literal in the test.

## Vectors

| Vector | Case | Expected result |
| --- | --- | --- |
| PL-01 | On each of E1 to E5, a unit both layers refuse with a typed refusal (a bounded-integer range proof that fails) | Holds on each edge, reporting both codes |
| PL-02 | On each of E1 to E5, the edge's distinguishing unit run as a case | Not applicable on each edge |
| PL-03 | PL-01's E1 to E5 cases, each with a test-only fault in the child side's compile that skips the failing range proof, so the child admits the unit | Regression on each edge naming the case, the edge and the parent's codes; verdict violation, exit 10 |
| PL-04 | Two units that differ in one byte of a declaration body | Tool failure naming the case; exit 30 |
| PL-05 | A case naming state core as parent and complete model as child | Tool failure naming the case; exit 30 |
| PL-06 | Each layer's witness unit selected alone: a bounded-scalar monitor invariant (state core), a named-predicate query over a bounded log (state queries), a reachability invariant over a finite universe (state graph), a record validation function (complete value), a dispatched operation contract (complete model), each witness item requested with its layer's `witness_request` (`operation-contract`/`bounded`, or `value-validity`/`bounded` for complete value) against a test registry holding one backend that advertises both kinds in `bounded` mode | Holds for each of the five layers, every witness item settling `supported`; verdict success, exit 0 |
| PL-07 | PL-06's state-core witness with a test-only fault that makes state core refuse it | Regression naming state core and the refusal; verdict violation, exit 10 |
| PL-07b | PL-06's state-core witness requested against a test registry whose one backend advertises only `value-validity` | Regression naming state core, disposition `unsupported` and the empty candidate set; verdict violation, exit 10 |
| PL-08 | Each edge's distinguishing unit compiled under its parent and its child | Holds on each edge: the parent gives `unsupported_construct` naming the parent layer and the child admits |
| PL-09 | E1's distinguishing unit with a test-only fault that makes state core admit named predicates | Regression naming E1; verdict violation, exit 10 |

## Expected Results

Each vector gives exactly its listed results, verdict and exit code. Every
layer is selected by a `profile` declaration in at least one compiled unit,
and every edge is compiled on both sides; no case carries a recorded outcome.
