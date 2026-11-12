---
id: SR-9136
title: "Failure-domain review of derived-artifact identity for projection and realizability witnesses"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification; PR #650 diff origin/main...0403e284: FR-174 and FR-175 Derived-artifact identity, against FR-288, FR-249, FR-054 and quire-protocol FR-152, FR-030 (ticket QSL-1873)"
review_set: subset
review_date: "2026-11-12"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-174, type: references }
  - { target: ix://agent-ix/quire-specification/FR-175, type: references }
  - { target: ix://agent-ix/quire-specification/FR-288, type: references }
---

## Summary

REVISE. Ticket: QSL-1873. The ruling's form is right: a derived, structural
key with no digest, the same discipline as FR-288 and FR-249, and the analyzer
identity and version in the witness key mirrors FR-249's adapter identity and
version. FR-174 correctly states that FR-062/FR-249 identify only the E02
result around the artifact. The defect is identity confusion: the projection
key omits inputs the projection depends on, and the witness key can collide
for a nondeterministic analyzer. The consumer (quire-protocol FR-152-AC-6 via
FR-030-AC-5) also needs the artifact to carry what it compares.

## Review set

FR-174 and FR-175 as amended; FR-288 key; FR-249 identity; FR-054-AC-4;
quire-protocol `origin/main` FR-152 and FR-030.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The FR-174 projection key is definition key + role + projection profile + bounds, but FR-174's Inputs include "channel/choice premises" and its Description makes soundness depend on "visibility premises". FR-054-AC-4 makes every channel premise identity-bearing, quire-protocol FR-030 retains them per disposition, and FR-175's own key lists them. Two projections of one definition and role under different FIFO or delivery premises get one identity while their graphs (or one sound result and one refusal) differ. Fix: add the selected channel/choice and visibility premises to the FR-174 key and to FR-174-AC-6. | spec/functional/protocol/FR-174-project-global-protocols.md:26; spec/functional/protocol/FR-174-project-global-protocols.md:63; spec/functional/protocol/FR-054-bind-channel-premises.md:77 |
| FND-002 | medium | FR-175's witness key excludes artifact bytes and producing run but does not require the analyzer to return one witness per key. A solver-backed analyzer with a random seed or thread-order dependence can return two distinct valid witnesses for one key, which then share an identity. Fix: either require a fixed analyzer identity and version to be deterministic for a fixed key, or make the analyzer's determinism-relevant settings (seed) a key member; state which in FR-175-AC-6. | spec/functional/protocol/FR-175-analyze-protocol-realizability.md:62; spec/functional/protocol/FR-175-analyze-protocol-realizability.md:76 |
| FND-003 | medium | Neither FR-174 nor FR-175 requires the produced artifact to carry its key members (or the derived identity) in the result. quire-protocol FR-152-AC-6 refuses "a projected local artifact whose identity disagrees with the one derived from its selected tuple", which needs a carried identity to compare. Fix: state that the artifact carries its key members as a structured value on the result. | spec/functional/protocol/FR-174-project-global-protocols.md:52 |

## Verdict

REVISE. FND-001 is the blocking finding: as written, the identity ruling
would let quire-protocol FR-030-AC-5 accept a projection computed under
different channel premises as the same artifact.
