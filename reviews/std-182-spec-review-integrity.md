---
id: SR-13055
title: "Integrity review of PR #284: abstract protocol subject header, ACs and vectors"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@04cc1b9; FR-375 Surface syntax, Checked-package node, AC-11; FR-432 abstract-side selection paragraph, AC-6; TC-310 mutation 13; TC-386 Test Procedure, SR-02, SR-09"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-432
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-386
    type: reviews
---

## Summary

Ticket: STD-182. Checks internal consistency and testability of the new
ACs and vectors: refusal locations, spelling of abstract protocol names
across FR-432 and TC-386, and the node body's dependency set.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-386 does not say which header SR-01 to SR-08 use, and after this PR the header is the only way to select the abstract protocol `Req`; under `abstract Spec` SR-02 would refuse (SR-09's second half), not be "admitted; the refinement holds". SR-09 then says SR-02's "`node Spec::Req::…` rows are admitted", but SR-02 writes `node Req::Ask` and `node Req::Done` (no alias), while FR-432's table writes `node <A>::<protocol>::<node>`. A test author cannot tell which spelling is meant, or whether `Req::Ask` is admitted. Fix: state `abstract Spec::Req` in the Test Procedure for every vector and spell SR-02 and SR-06 as `node Spec::Req::…`. | spec/test-cases/TC-386-explicit-protocol-step-rows.md:20-27,32,36,39; spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:66-67 |
| FND-002 | medium | AC-11's syntax refusals are "at the offending token", which the grammar does not make obvious for two of the four. Because `member-name` admits any word, `abstract Spec:: concrete Impl {` consumes `concrete` as the protocol name and refuses at `Impl`, not at `concrete` or `::`; `abstract Spec::Twice::finish` refuses at the second `::`, not at `finish`. Two parsers (one with lookahead for `concrete`) give different spans and both satisfy the text. Name the token for each vector in AC-11 and TC-310. | spec/functional/analysis/FR-375-declare-model-refinement.md:85-88,250; spec/test-cases/TC-310-refinement-declaration-admission.md:73-75,111-114 |
| FND-003 | low | The node's `dependencies` "hold every node its rows name"; the protocol clause is named by the header, not a row, so whether its node enters `dependencies` (it already enters the body as `protocol`) is unstated. Two conformant producers can emit different `dependencies` and so different node ids. | spec/functional/analysis/FR-375-declare-model-refinement.md:213-219 |

## Verdict

Not mergeable on its own findings: FND-001 and FND-002 leave the new
vectors ambiguous to a test author. FND-003 is a one-clause fix.
AC ids are fresh (FR-375-AC-11, FR-432-AC-6), each AC is traced to a TC
that verifies its FR, and TC-310 mutation 13 and TC-386 SR-09 follow their
files' numbering.
