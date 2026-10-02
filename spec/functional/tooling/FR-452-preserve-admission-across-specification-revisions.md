---
id: FR-452
title: "Preserve admission across specification revisions"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-202
    type: references
  - target: ix://agent-ix/quire-specification/FR-301
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-340
    type: references
---

# FR-452: Preserve admission across specification revisions

## Description

When a refinement test gate compares a prior and a superseding revision of
one specification over a corpus of cases, the gate SHALL report a
regression for every case the prior revision admits and the superseding
revision refuses, and SHALL never report an unresolved case as holding.

## Inputs

A corpus of revision pairs. Each pair is two complete-V1 source revisions of
one specification, with equal FR-202 document identity, one named `prior`
and the other `superseding` by the corpus, and a set of cases. The two
revision labels may be equal or differ; the gate does not read them. A case
is one selected clause, Boolean function or operation frame with its input,
run against each revision in one build, against the same domain packages and
libraries.

## Outputs

One case result per case (holds, regression, unresolved, not applicable or
tool failure), one verdict for the run and its FR-301 exit code.

## Behavior

### Pairs

- The direction of a pair is the corpus's `prior` and `superseding` naming.
  The gate reads no order from, and compares nothing about, the revision
  labels.
- A pair whose two revisions differ in FR-202 document identity is a tool
  failure naming the pair; none of its cases runs.

### Run outcomes

Each run of a case against one revision has exactly one outcome, by the
stage that settles it:

- **admitted**: the revision compiles, the selection resolves, the input is
  admitted and the evaluation settles true (for a frame, no violation);
- **absent**: the revision no longer declares the selected clause, function
  or frame;
- **refused**: a typed compile, selection or admission refusal; an
  evaluation that settles false; or an evaluation that settles refuted,
  including an undefined evaluation, which settles refuted with cause
  `UndefinedEvaluation{where, cause}`;
- **prohibited**: a compile refusal whose every cause is
  `unsupported_construct` with a catalog cause;
- **unsupported**: an implementation gap (`unsupported_construct` with cause
  `not-yet-implemented`, or `unsupported_projection`);
- **incomplete**: a run that reaches a caller-configurable limit;
- **tool failure**: the case's setup is broken (a missing input, an internal
  fault).

### Case results

The first matching rule gives the case result:

1. either run is a tool failure: **tool failure**;
2. the prior run is incomplete, at any stage: **unresolved**;
3. the prior run is unsupported, at any stage: **unresolved**;
4. the prior run is refused, prohibited or absent at the compile or
   selection stage: **tool failure**, because the case is malformed;
5. the prior run is admitted, and the superseding run is admitted, or
   absent for a clause or frame: **holds** (a revision that drops a
   constraint admits more);
6. the prior run is admitted, and the superseding run is refused,
   prohibited, or absent for a function: **regression**;
7. the prior run is admitted, and the superseding run is unsupported or
   incomplete: **unresolved**;
8. otherwise (the prior run is refused at admission or evaluation): **not
   applicable**.

An unresolved case is never reported as holding.

### Verdict

- The verdict is the highest-severity case result under FR-301's order, and
  the exit code is FR-301's code for it: tool failure 30, unsupported 21,
  incomplete 22, regression (violation) 10, otherwise 0. Every regression is
  listed whatever the verdict.
- A case result is test evidence over its corpus. The gate records no
  requirement record, requests no capability kind and settles no claim
  (FR-290). It compares outcomes, never package identities or revision
  labels.

quire-spec-language FR-340 to FR-344 implement this gate, including the
classification of each compile and run result into the outcomes above.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-452-AC-1 | A pair whose superseding revision tightens one clause, with one case the prior admits and the superseding refuses, reports exactly that case as a regression, verdict violation, exit 10; the pair's other cases hold. | Test (TC-413) |
| FR-452-AC-2 | A pair whose superseding revision drops the selected clause reports the case as holding; a pair whose superseding revision drops a selected function reports a regression. | Test (TC-413) |
| FR-452-AC-3 | A case whose superseding result is unsupported, and one whose superseding run reaches its work limit, are reported unresolved, with verdicts unsupported (exit 21) and incomplete (exit 22), and neither is reported as holding. | Test (TC-413) |
| FR-452-AC-4 | Swapping a pair's revision labels while keeping its `prior` and `superseding` naming gives the same case results. | Test (TC-413) |
| FR-452-AC-5 | A case whose prior revision fails to compile with a typed refusal, and a pair whose two revisions differ in document identity, are each a tool failure naming the case or pair; verdict tool failure, exit 30. A case whose prior compile is unsupported is unresolved, not a tool failure. | Test (TC-413) |
| FR-452-AC-6 | A case whose prior run is refused at evaluation is not applicable, whatever the superseding run gives. | Test (TC-413) |
| FR-452-AC-7 | A case the prior revision admits and whose superseding evaluation is undefined (settled refuted with cause `UndefinedEvaluation`) is a regression. | Test (TC-413) |

## Dependencies

- [FR-202](../../objects/foundation/FR-202-authored-source-document.md) owns
  the document identity two revisions share.
- [FR-301](FR-301-expose-complete-cli.md) owns the severity order and exit
  codes.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns claim
  kinds; this gate requests none.

## References

- Capability row V1-TOOL-011.
- quire-spec-language ADR-017 §2 RF-1, RF-2, RF-4; FR-340 to FR-344.
- Specification ticket STD-117.

## Status

Proposed complete-V1 requirement for the spec-versioning refinement gate.
Implementation and qualification remain tracked separately.
