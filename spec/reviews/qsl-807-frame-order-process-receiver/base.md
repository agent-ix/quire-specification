---
id: SR-11691
title: "Spec review of quire-specification PR #263: frame order tiebreak, shared frame node, receiver forms on process operations"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@f6f5af3d1c8631f339b30bd23c2534d5fc92f581; proposals/quire-v1/definitions/model-complete.md, spec/functional/foundation/FR-013-validate-operation-frames.md, spec/objects/foundation/FR-208-quire-meaning-vocabulary.md, spec/objects/interfaces/FR-340-checked-package-frame-location.md, spec/objects/interfaces/FR-342-checked-package-operation-anchor-body.md, spec/objects/interfaces/interface_004-checked-package.md, spec/test-cases/TC-944-scoped-members-of-a-model-document-operation-frame.md"
review_set: subset
---
# Spec review of quire-specification PR #263

## Summary

Ticket: QSL-807 (follow-up; QSL-808 read as context). One pass of
spec-review with the contradiction, ambiguity, completeness, testability and
spec-integrity sub-analyses folded in, over `git diff origin/main...HEAD`.
Spec-only PR, no build. Only major issues block; mediums and lows are
recorded in the Verdict. Paired QSL PR agent-ix/quire-spec-language#889 read
for agreement only.

Oracles. Merged QSL `origin/main`: FR-104 (incl. AC-5), FR-105, FR-114,
FR-154, FR-103, TC-1043. QSpec: FR-322 (occurrences), FR-208, FR-013,
model-complete.md Frames row, TC-944, TC-235 (M07, M10, M15), TC-233 and its
executable vectors in `tests/checked_package_v2.rs`.

Examined:
- FR-342 Description, Reader order uniqueness paragraph, FR-342-AC-6
- FR-340 Canonical ordering, precedence rule 1, FR-340-AC-4, FR-340-AC-6,
  FR-340-AC-16; FR-340 Properties `deletes_scoped` and FR-340-AC-17
- interface_004 frame precedence sentence
- model-complete.md Frames row; FR-013 Behavior and FR-013-AC-14; FR-208
  Operation contracts bullet
- TC-944 fixture, R15 to R20
- context only: `tests/checked_package_v2.rs` `anchor_uniqueness_refusal` and
  `tc_233_duplicate_anchors_and_shared_frames_are_refused`; QSL #889 FR-105,
  FR-114, FR-154, TC-1043 diff

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-342-AC-6 now admits two anchors of one context that reference one frame node, but QSpec's own executable vector for that AC still asserts the withdrawn refusal: `tc_233_duplicate_anchors_and_shared_frames_are_refused` (traced `TC-233`, `FR-342-AC-6`) expects `ambiguous_declaration` for "a frame node bound by two different anchors", and the reference reader `anchor_uniqueness_refusal` still refuses a second anchor claiming a frame (`frames.insert`). The PR leaves the repo's AC and its verifying test asserting opposite results, and the test keeps passing, so nothing flags it. Fix in this PR: drop the frame set from `anchor_uniqueness_refusal`, and turn the second half of the test into an admission vector over two operations the context declares with equal frames (the current mutation's `otherAttempt` would then reach operation resolution). | tests/checked_package_v2.rs:3559-3589; tests/checked_package_v2.rs:9087-9141; spec/objects/interfaces/FR-342-checked-package-operation-anchor-body.md:110 |
| FND-002 | high | FR-340 checks a `deletes_scoped` `parameter` entry against "a reference parameter of the operation" (Properties and FR-340-AC-17). That relied on one frame per operation. With a frame node now shared by several anchors, "the operation" is undefined, and the frame step runs before the anchors are read. Two readers can disagree on a package whose shared frame names `p` while one of its operations has no reference parameter `p` (check the first anchor, any anchor, or every anchor). Fix: state that the entry must name a reference parameter of every operation whose anchor references the frame, and where that check runs. | spec/objects/interfaces/FR-340-checked-package-frame-location.md:69-77; spec/objects/interfaces/FR-340-checked-package-frame-location.md:277 |
| FND-003 | high | The paired PRs disagree on state machines. QSpec #263 refuses non-empty `modifies_receiver` and `deletes_receiver: true` on operations of a process and of a state machine (Frames row, FR-013, FR-013-AC-14, FR-208, TC-944 R19). QSL #889 FR-154 refuses them only on "an operation declared by a process", and TC-1043 has no state-machine row. Once both merge, QSL intake admits `deletes_receiver: true` on a state-machine operation that QSpec TC-944 R19 refuses. Fix: add state machines to QSL #889 FR-154 and a TC-1043 row matching R19, or narrow QSpec to processes. | spec/functional/foundation/FR-013-validate-operation-frames.md:48; spec/test-cases/TC-944-scoped-members-of-a-model-document-operation-frame.md:54,88-90 |
| FND-004 | high | TC-944 R15 to R20 all use a scoped member (`modifies_receiver`, `deletes_receiver`, `deletes_parameters`), including the admitting rows R17 and R18. The filament-core-data semantic-IR frame schema validates before intake and today admits exactly `modifies`, `creates` and `deletes`. Until the in-flight FCD change lands, every one of these rows fails that validator before intake and never reaches the stated result. The rows are not marked as dependent, unlike the paired QSL TC-1043 I13 to I17. Fix: mark the scoped-member rows of TC-944, R15 to R20 at least, as depending on the FCD frame schema admitting the scoped members. | spec/test-cases/TC-944-scoped-members-of-a-model-document-operation-frame.md:49-54 |

## Verdict

FAIL, four blocking findings (FND-001 to FND-004).

Judged sound:
- (a) Shared frame node. This agrees with merged QSL FR-105 (frame row:
  "two operations whose frame node holds equal content still get two
  occurrences") and with QSL FR-104-AC-5 (frame records per operation even
  when frame nodes coincide). FR-322 "one or more occurrences" holds, with
  one `generated` occurrence per operation. FR-342's "cannot share across
  contexts" follows from its join 3 (frame `semantic_type` equals
  `context`). Apart from FND-001 and FND-002, no other QSpec rule, TC or
  fixture depends on the old refusal. QSL #889 FR-105 and FR-114 agree.
- (b) Order key and precedence. The scope tiebreak cannot place an admitted
  entry. One scope twice is a schema duplicate, and two scopes are
  `conflicting-binding`, which now outranks the order refusal and ranks after
  every other meaning-join defect. FR-340-AC-4, FR-340-AC-6, FR-340-AC-16,
  precedence rule 1 and interface_004 agree with one another, with merged
  QSL FR-105/FR-154 (no admitted pair is reordered), and with QSL #889
  X8/X9.
- (c) Receiver forms. FR-208's Operation contracts bullet does give state
  machines and processes the same contract (one bullet, "Frame entries follow
  the object-type Frames rule"), so the author's claim holds. The check
  order (member shapes, receiver rule with `modifies_receiver` first, then
  entries) is the same in the Frames row, FR-013-AC-14 and TC-944 R20. No
  existing QSpec or QSL fixture refuses because of it: TC-235 M15 uses
  `modifies` only, and no fixture puts a receiver member on a process or
  state-machine operation. R15 to R20 follow from the rule, given FND-004.
- No hash, pin or version ceremony was added.

Recorded, not blocking:
- medium: a run has object identity in an object universe (FR-208 process
  row), and `deletes Flow` is admitted type-wide, but `deletes self` on
  `Flow`'s own operation is refused. This is a design choice already tracked
  as QSL-808's alternative.
- low: TC-944 does not give the types of `Flow`'s `id` and `x`. QSL TC-1043
  types `id` as `Integer`.
- low: TC-944 does not give the frames of `go` and `flip` in rows R1 to R14.
- low: the FR-340 Properties `semantic_type` text still says "the
  operation" and "the anchor" in the singular.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | high | The fix round's TC-944 dependency note says R12 and R14 "are reachable without" the FCD frame-schema change. R14 is. R12 is not: it omits `deletes`, and today's filament-core-data reader (`frame_schema`, `require_members` over `modifies`/`creates`/`deletes`, plus `forbid_extra`) refuses it before intake. `deletes` stays required after the planned change, so R12 never reaches the intake refusal it expects. QSL FR-056-AC-19 keeps the reader's diagnostic and subsumes an intake diagnostic on that member. R10, R11 and R13 have the same problem: a wrong JSON type, an unnamed member and a non-string element fail the same schema both before and after the change, so the change does not make them reachable. Fix: say that R10 to R13 are member-shape rows the FCD frame schema itself refuses before intake, and that only R14 is reachable today. | spec/test-cases/TC-944-scoped-members-of-a-model-document-operation-frame.md:62-66 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6b0df8ec18cbfe84c0be36fac8c6f516988ded78 |
| FND-002 | fixed | 6b0df8ec18cbfe84c0be36fac8c6f516988ded78 |
| FND-003 | rejected | QSL never intakes a state-machine operation: merged QSL FR-103 declares operations only on object types and processes, and #889 FR-154 now says so. No QSL path admits what QSpec TC-944 R19 refuses. QSpec's Frames row, FR-013, FR-013-AC-14, FR-208 and TC-944 R19 agree with one another. |
| FND-004 | fixed | 6b0df8ec18cbfe84c0be36fac8c6f516988ded78 |
