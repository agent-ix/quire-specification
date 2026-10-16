---
id: SR-11688
title: "Spec review of quire-specification PR #262: receiver-scoped members of the model-document operation frame"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@e35854b37fb4ccc3c4045549cecab13b9f7e6f50; proposals/quire-v1/definitions/model-complete.md, spec/functional/foundation/FR-013-validate-operation-frames.md, spec/objects/interfaces/FR-340-checked-package-frame-location.md, spec/test-cases/TC-944-scoped-members-of-a-model-document-operation-frame.md"
review_set: subset
---
# Spec review of quire-specification PR #262

## Summary

Ticket: QSL-807. One pass of spec-review with the contradiction, ambiguity,
completeness, testability and spec-integrity sub-analyses over the PR diff
(`origin/main...HEAD`). Spec-only PR, so no code review. Only major issues
block. Mediums and lows are recorded in the Verdict.

Oracles, merged QSL `origin/main`: FR-154 (Inputs, Behavior: Intake and
Emission), FR-103 (Behavior), FR-105 (frame row, one anchor and frame per
named operation), FR-115-AC-11, TC-1043 (fixture, I1 to I12, W1, W2).
QSpec rules: model-complete.md Frames row, FR-013, FR-340, FR-154, FR-342,
FR-208, FR-322 (occurrences).

Examined:
- model-complete.md Frames row (examined)
- FR-013 Behavior, FR-013-AC-11, FR-013-AC-12, FR-013-AC-13 (examined)
- FR-340 Description (examined); FR-340 Properties, Wire encoding, Canonical
  ordering and Dependencies (context_only)
- TC-944 fixture, R1 to R14, step 2 bodies of `bump` and `merge` (examined)
- QSpec FR-154 declaration refusals (examined, for the "needs no change" claim)
- QSpec FR-342 Description and FR-342-AC-6, FR-208 process row (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | TC-944 step 2 cannot produce the two frame bodies it expects. It emits a checked package for `bump` and `merge`, but no clause names either operation. Merged QSL FR-105 emits an operation_anchor and a frame "none for an operation no clause and no protocol attempt names", QSpec FR-342 gives an anchor one occurrence per clause that names the operation, and QSpec FR-322 requires one or more occurrences on every node. QSL TC-1043 names each operation by its own postcondition clause "so that the package holds its frame node"; TC-944 omits this, and `merge` is not said to be an operation of `Counter`. So FR-013-AC-13 has no reachable vector. Fix: declare `bump` and `merge` on `Counter`, each named by a `{ true }` postcondition clause, as TC-1043 does. | spec/test-cases/TC-944-scoped-members-of-a-model-document-operation-frame.md:50-54 |
| FND-002 | medium | TC-944 R13 expects the refusal "at the `modifies_receiver` element". The Frames row and FR-013-AC-11 locate a non-string array element at the member, which matches QSL FR-154 Inputs ("at the member"). A test written from R13 asserts a different locus from one written from the rule. Fix: "R13 at `modifies_receiver`". | spec/test-cases/TC-944-scoped-members-of-a-model-document-operation-frame.md:73-77 |

## Verdict

FAIL, two blocking findings (FND-001, FND-002).

Traced and correct:
- Frames row against QSL FR-154 Inputs and Intake: the member names, JSON
  types, required and optional members, absent = empty or false, the shape
  refusal and its locus, the check order (shapes, then `modifies`,
  `modifies_receiver`, `creates`, `deletes`, `deletes_parameters`, each in
  document order, first failure), the `modifies_receiver` and
  `deletes_parameters` refusals, `deletes_receiver` admitted for an object
  type's operation, and the conflict last at the later entry with no merge
  are identical. The order agrees with QSL FR-103.
- TC-944 R1 to R12 are TC-1043 I1 to I12 byte for byte, and each expected
  result follows from the Frames row. R8 reports only `missing-name` because
  the conflict runs last. R9 is located at the `modifies_receiver` entry;
  the domain package is JCS-serialized, so `modifies` always precedes
  `modifies_receiver`. R13 (apart from FND-002) and R14 are reachable.
- Step 2 bodies, once reachable, are TC-1043 W1 and W2 byte for byte: JCS
  member order (`creates`, `deletes`, `deletes_scoped`, `modifies`, `term`;
  `declaration`, `kind`, `name`, `scope`), NodeRef objects, one declaration
  so `step` before `value`, and `deletes_scoped` receiver first then `a`
  before `b`.
- FR-013 Behavior and FR-340 Description state the lift as QSL FR-154
  Emission does. FR-013-AC-11 and FR-013-AC-12 are testable through R1 to
  R14.
- QSpec FR-154 needs no change: its frame-entry `missing-name` row and the
  per-meaning `malformed-declaration` row cover the new entries, and the
  conflict is a `quire.model.features/v1` check that the Frames row owns.
- FCD schema delta (PR body): keeping `required` and
  `additionalProperties: false` and adding the three optional members with
  the stated types matches the member shapes. No `uniqueItems` is needed,
  because a repeated entry is one entry.
- Author's open Choices are real: (a) QSL FR-105 gives two operations with
  equal frame content one frame node, and QSpec FR-342-AC-6 refuses a frame
  targeted by two anchors; (b) FR-340's canonical-ordering and precedence
  sections do not place conflicting-binding against equal order keys;
  (c) FR-208 gives processes operation contracts, and the receiver forms are
  stated for object types only.
- No tracking, hash or version ceremony was added.

Recorded, not blocking:
- medium: QSL-807 item (3) asks QSpec to resolve Choices (a) and (b); the PR leaves them open for the planner.
- low: FR-340 Dependencies still says the Frames row's shape carries "unchanged" onto the wire, but the Description now lifts it.
- low: FR-340 Description says "exactly four members", and the Wire encoding body also has `term` (this was already so).
- low: FR-154's conflicting-binding rows do not list the frame scope conflict (covered by the Frames row).
- low: in R7, it is not said whether `Counter.t` still exists next to `Sub.t` (same as TC-1043 I7).
