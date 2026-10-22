---
id: TC-953
title: "Proof gate discharge acceptance"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/NFR-076
    type: verifies
---

# TC-953: Proof gate discharge acceptance

## Description

Run a proof gate over two claimed modules against discharge, mutation and
shared-helper controls, and check that it passes only when every claimed
module discharges a proposition and every control turns it red.

## Test Procedure

Fixture **G**: a proof gate that claims modules `M1` and `M2`. `M1` holds
function `f1`, and `M2` holds function `f2`. Each has one proof harness whose
expectation is taken from the QSpec operation vectors and reaches no helper of
its function under proof, unless a row states otherwise. The prover reports a
status per check location unless a row states otherwise. Each module has one
mutation control: `m1` negates the comparison in `f1`, and `m2` replaces the
result of `f2` with its first argument. Run the gate once per row and read its
report.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| P01 | G with `M1` showing two SUCCESS check locations and `M2` one; `m1` and `m2` each turn the proof red | the gate passes; `M1` is `discharged` with 2 checks and `M2` `discharged` with 1; the report names `m1` and `m2` and states each turned the gate red, and names no shared helper (NFR-076-AC-1) |
| P02 | G with its claimed module set emptied | the gate fails: it declares no claimed module and discharges nothing (NFR-076-AC-1) |
| P03 | P01 with `M2`'s harness removed, so the prover compiles `M2` and reaches no check in it | the gate fails; `M1` is `discharged` with 2 checks and `M2` is `unreached` (NFR-076-AC-2) |
| P04 | P01 with `M2`'s one check reported UNREACHABLE; then reported UNDETERMINED | each run fails with `M2` `unreached`; neither status is counted as discharged (NFR-076-AC-2) |
| P05 | P01 on a backend that reports a status per obligation and none per check: `M1`'s one obligation proved and `M2`'s one obligation inconclusive | the gate fails; `M1` is `discharged` with 1 check and `M2` is `unreached` (NFR-076-AC-2) |
| P06 | P01 with `m2` leaving the proof green; then the same with `m2` listed as exempt in the gate configuration; then P01 with `m2` removed, so `M2` has no mutant | each run fails; the first two name module `M2` and mutant `m2` as surviving, and the third names `M2` as having no mutant (NFR-076-AC-3) |
| P07 | P01 with `M1`'s expectation and `f1` both calling helper `decode`; a mutation of `decode` leaves the proof VERIFICATION SUCCESSFUL | the gate fails and names `decode` as a shared helper whose mutation left the proof green (NFR-076-AC-4) |
| P08 | P07 with `M1`'s expectation taken from the QSpec operation vectors, so it reaches no helper of `f1` | the gate passes with no shared helper reported, as P01 (NFR-076-AC-4) |
| P09 | P01 with the prover replaced by an executor that returns VERIFICATION SUCCESSFUL without running; then with a build-only run that compiles `M1` and `M2` and runs no proof | each run fails, and the run contributes no proof evidence to a qualification claim (NFR-076-AC-5) |
| P10 | P01 with `f1` an exact kernel operation and `M1`'s expectation for it built by calling `f1` itself; then built by calling a different exact kernel operation; then taken from a checked-in specification model that calls no exact kernel operation | the first two runs fail and name `f1`; the third passes (NFR-076-AC-6) |

## Expected Results

Only P01, P08 and the last run of P10 pass. Every other row fails, and the report names the module,
mutant or helper that caused the failure. No status other than SUCCESS or
proved counts as a discharged check.
