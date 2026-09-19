---
id: FR-355
title: "Check SMT/native runtime differential parity"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-010
    type: references
  - target: ix://agent-ix/quire-specification/FR-349
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/NFR-070
    type: references
---

# FR-355: Check SMT/native runtime differential parity

## Description

When the SMT backend and the native runtime both evaluate a member of the
shared differential corpus, the qualification runner SHALL compare their
verdicts and record agreement or an explicit parity failure for every member.

## Inputs

The shared differential corpus (claims with a defined native `runtime::execute`
evaluation and an SMT-encodable form), the registered SMT backend's verdict
per member, and the native runtime's verdict per member.

## Outputs

Per-member agreement or explicit parity-failure result, and an aggregate
parity report over the corpus.

## Behavior

Every differential corpus member is a bounded instance: a claim whose domain
is finite and exactly the same bounded domain the SMT backend encodes and the
native runtime executes; neither side substitutes a different or narrowed
domain. Agreement requires the same disposition on the SMT side
(`proved`/`refuted`/`inconclusive`) matched to the native runtime's
corresponding executed verdict on that identical bounded domain: SMT `proved`
corresponds to the native runtime holding the property over every instance of
the bounded domain, SMT `refuted` corresponds to the native runtime producing
a counterexample within that same bounded domain, and SMT `inconclusive`
corresponds to the native runtime reaching no decisive holds/counterexample
verdict over the bounded domain either; agreement is recorded only when both
sides land in the same one of these three correspondences, and a decode,
domain or verdict mismatch outside that correspondence is an explicit parity
failure naming the member and both verdicts, never silently resolved toward
either side. The shared differential corpus is non-empty and includes at
least one member for every obligation form FR-349's registered SMT backend
discharges (among unbounded quantification, induction, compositional discharge
and mathematical-integer domains). This corpus-level check is the FR-197
counterexample-replay precedent generalized from one reported counterexample
to every corpus member, run independently of whether any single claim
reported a counterexample. This differential corpus is the executable check
that the SMT encoding and the native runtime agree on meaning over every
covered obligation form; it is not a mechanized equivalence proof.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-355-AC-1 | Every differential corpus member is a bounded instance; the registered SMT backend and the native runtime both evaluate that same bounded domain under the same package, domain and claim identity. | Test (TC-270) |
| FR-355-AC-2 | Matching verdicts are recorded as agreement under the defined correspondence — SMT `proved` against the native runtime holding over the full bounded domain, SMT `refuted` against a native in-domain counterexample, and SMT `inconclusive` against no decisive native verdict over the bounded domain; a decode, domain or verdict mismatch outside that correspondence is recorded as an explicit parity failure naming the member and both verdicts. | Test (TC-270) |
| FR-355-AC-3 | A parity failure records both the SMT and the native verdict unchanged, settles neither toward the other, and is never rewritten as clause success/failure. | Test (TC-270) |
| FR-355-AC-4 | The aggregate parity report accounts for every corpus member exactly once. | Test (TC-270) |
| FR-355-AC-5 | This corpus check is retained as an executable check distinct from, and not claiming to be, a mechanized equivalence proof. | Test (TC-270) |
| FR-355-AC-6 | The shared differential corpus is non-empty and includes at least one member for every obligation form the registered SMT backend discharges under FR-349. | Test (TC-270) |

## Dependencies

- [AD-010](../../assurance/AD-010-target-neutral-ir-backends.md) supplies the
  backend/runtime parity architecture (explicit parity failure on
  disagreement) this requirement's corpus check follows.
- [FR-349](FR-349-register-smt-backend.md) supplies the registered SMT
  backend this requirement's corpus evaluates against.
- [FR-197](FR-197-replay-backend-counterexamples.md) supplies the
  single-counterexample replay-parity precedent this requirement generalizes
  to a full differential corpus.
- [NFR-070](../../non-functional/NFR-070-preserve-complete-semantic-fidelity.md)
  supplies the zero-approximation fidelity constraint this corpus check
  evidences.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification
  acceptance.
