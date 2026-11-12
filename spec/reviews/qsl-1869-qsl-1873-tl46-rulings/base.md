---
id: SR-9134
title: "Base review of the TL-46 rulings on FR-177 outputs, FR-360 labels and derived-artifact identity"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR #650 diff origin/main...0403e284: FR-174, FR-175, FR-177, FR-377, FR-378, FR-379, FR-489 (tickets QSL-1869, QSL-1873)"
review_set: subset
review_date: "2026-11-12"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-177, type: references }
  - { target: ix://agent-ix/quire-specification/FR-174, type: references }
  - { target: ix://agent-ix/quire-specification/FR-175, type: references }
  - { target: ix://agent-ix/quire-specification/FR-379, type: references }
  - { target: ix://agent-ix/quire-specification/FR-489, type: references }
---

## Summary

REVISE. Tickets: QSL-1869, QSL-1873. The base checklist passes on IDs,
relationships, link targets (the stale FR-341 links in FR-379 now point at
FR-360, which exists; FR-341 is the checked-package state-clause body) and
`quire validate` on the seven changed files (only pre-existing EARS warnings
in FR-377). Rulings QSL-1869-1 and QSL-1869-2 are sound on the merits:
FR-360 is the closed five-label vocabulary of FR-161, FR-241 already has
`resource-incomplete`, and FR-363, FR-399 and FR-367 already write a stopped
run as FR-360 `failed`, execution `resource-incomplete`, FR-331 `incomplete`.
Not adding a sixth label is the right call: it keeps FR-161, FR-363, FR-391
and FR-399 untouched and the execution axis already separates exhaustion from
backend failure. The defects are in test coverage of the three new criteria
and in one under-specified refusal.

## Review set

`git diff origin/main...origin/pr-650` (7 files). Base checklist only here;
integrity and failure-domain lenses have their own SpecReview files.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-177-AC-8 cites TC-318, but TC-318 has no case for it: no step stops a refinement run on a resource limit and no expected result names `failed`/`resource-incomplete`, nor the `refines`->`proved`/`violates`->`refuted` reading. Fix: add a TC-318 case with a budget stop and expected `failed`, FR-241 `resource-incomplete`, FR-331 `incomplete`, and state the label of cases 1 and 2 in FR-360 terms. | spec/functional/protocol/FR-177-check-protocol-refinement.md:107; spec/test-cases/TC-318-protocol-refinement-step-rules.md |
| FND-002 | medium | FR-174-AC-6 cites TC-205, whose procedure is a generic "positive case, each boundary, each refusal" with no identity comparison: no step varies a key member or a non-key member, or omits a member. Fix: add TC-205 steps that change each of the key members and each excluded member and one that omits a member, with expected identity equal/different/refused. | spec/functional/protocol/FR-174-project-global-protocols.md:81; spec/test-cases/TC-205-sound-global-projection.md |
| FND-003 | medium | FR-175-AC-6 cites TC-206, which likewise has no identity step (no variation of strategy profile, premises, bounds or analyzer identity/version). Fix: add the matching TC-206 steps. | spec/functional/protocol/FR-175-analyze-protocol-realizability.md:76; spec/test-cases/TC-206-protocol-realizability-witnesses.md |
| FND-004 | medium | FR-174 "Derived-artifact identity" and FR-174-AC-6 say "an absent key member is refused" without naming the refusal code or who refuses (the producer building the key, or a reader of a carried artifact). The repo names every other refusal (`invalid_runtime_input`/`invalid-value`, FR-283 codes). Fix: name the code and the refusing actor; FR-175-AC-6 inherits the same fix. | spec/functional/protocol/FR-174-project-global-protocols.md:67; spec/functional/protocol/FR-174-project-global-protocols.md:81 |
| FND-005 | low | FR-177-AC-8 says each label settles "with the FR-241, FR-242 and FR-243 combination FR-360 names for it", but FR-360 leaves `proved` open between `closed-scope` and `decisive-witness`; the FR-379 verdict table fixes the basis per outcome, as the new FR-177 Outputs prose says. Fix: cite the FR-379 verdict table for the basis in AC-8. | spec/functional/protocol/FR-177-check-protocol-refinement.md:107 |

## Verdict

REVISE. Rulings 1 and 2 are sound and consistently applied inside the
diff (FR-177 Outputs, FR-377 budget-stop prose and AC-13, FR-378, FR-379
table and AC-1, FR-489 table and AC-9). The FR-377 rewording "never execution
`failed`" correctly disambiguates the execution value from the FR-360 label.
Ruling 3's structural, digest-free form matches FR-288 and FR-249.
