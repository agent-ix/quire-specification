---
id: SR-9105
title: "base review of QSL-681 supplied-value ceilings in FR-323 and TC-278 (PR 205)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@3369fa050eefb3ce298764d2c319a172dea52116; spec/objects/interfaces/FR-323-native-runtime-envelope.md; spec/test-cases/TC-278-replay-envelope-members.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: references
  - target: ix://agent-ix/quire-specification/TC-278
    type: references
---

# Base review of QSL-681 supplied-value ceilings (quire-specification PR 205)

## Summary

Ticket: QSL-681. One base pass over the PR diff (`origin/main...HEAD`, three
commits) that adds the typed `limits.supplied` request member, the closed
pre-call `supplied` result context, FR-323-AC-10..14 and TC-278 RE-12..22.
The base checklist covered AC-to-TC binding, id uniqueness, agreement with
the paired QSL PR (quire-spec-language 694 at 86628e9f1, plus the fix round
that was uncommitted in its worktree when read), and coherence with the
existing `limits` and identity rules. Integrity, contradiction and identity
checks were folded into this base pass rather than written as separate
sub-analysis files. The PR caller's rules applied: only contradictions,
unbound ACs, QSL disagreements and broken gates count as blocking.

Examined, clean: FR-323-AC-11, AC-12, AC-13 and AC-14. Each is bound to a
TC-278 vector (RE-15/16/20, RE-17/18/19/20, RE-21 as Analysis, and RE-22).
AC ids 10..14 and vector ids RE-12..22 are unique, with no collision on
main. Member and field names, the 16777216 defaults, the stage_limits
exclusion, the five cause forms, the phase-marked inconclusive/NoValue
projection, the InternalFault route and the per-argument C/occ units all
agree with QSL FR-277, FR-098 and FR-255, at the tip and in the uncommitted
fix round. The fix round drops the FR-255 registry rows but keeps the names,
and FR-277 still owns the defaults. No private QSpec prose appears in the
public QSL PR beyond shared contract names.

Validation: `quire validate` (0.36.2, engine 0.50.2) on FR-323 reports a
missing value_object `id`, `semantic.record-invalid` (fields required) and
the `semantic.legacy-properties-form` warning at line 21. The same three
diagnostics reproduce byte-for-byte on `origin/main`'s FR-323, copied to a
scratch path and validated with the same scope, so they are pre-existing and
not introduced by this PR. TC-278 validates clean.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | The amendment preamble says it is not a claim of a "selected default", but the Request ceilings section and AC-10 now select 16777216 for each ceiling. This leftover from the old draft contradicts the settled default. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:115-117, :137-142 | wrong-requirement |
| FND-002 | high | AC-10 says an omitted ceiling resolves to its published default, with no qualifier. RE-13, bound to the same AC, expects a request that omits an effective supplied ceiling to refuse before consumption. The FR body separates caller-API omission (defaulted) from an incomplete wire object (refused), but the AC does not, so AC-10 and RE-13 contradict each other. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:323, :130-140; spec/test-cases/TC-278-replay-envelope-members.md:79 | wrong-requirement |
| FND-003 | high | The FR body resolves caller-API omission to the defaults before request identity is computed, so an API-default request and an explicit 16777216/16777216 request must have the same request identity. RE-14 only says the default request normalizes the fields "into effective ceilings/identity". It never compares the two identities, so an implementation that hashes a defaulted marker still passes. | spec/test-cases/TC-278-replay-envelope-members.md:80; spec/objects/interfaces/FR-323-native-runtime-envelope.md:139-141 | correct-requirement-no-evidence |
| FND-004 | low | Recorded, not blocking: the AC-10..14 Verification cells say bare `Test`/`Analysis` with no `(TC-278)`. The binding exists from the TC-278 vector Criteria column. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:323-327 | correct-requirement-no-evidence |
| FND-005 | low | Recorded, not blocking: AC-10's zero-ceiling clause is exercised only by RE-15, which is bound to AC-11. Also, `limits.supplied` has no named versioned type alongside ScalarLimitsV1 and PopulationAdmissionLimitsV1. | spec/test-cases/TC-278-replay-envelope-members.md:81 | correct-requirement-no-evidence |

## Verdict

CHANGES REQUESTED: three blocking findings (FND-001..003), all about the
default and omission identity rule for `limits.supplied`. The fix stays in
these two files: remove "selected default" from the preamble; qualify AC-10's
omission clause as caller-API omission, with RE-13 covering only an
incomplete wire object; and make RE-14 assert that an API-default request and
an explicit 16777216/16777216 request have equal request identity. Nothing
else blocks: QSL agreement, AC binding, id uniqueness and validation (the
failures are pre-existing on main) pass.

## Dispositions

Round 1, reviewed at agent-ix/quire-specification@461620c41064ad02961b4f5b5ecdf0da46beb518. QSL pair 694 tip is unchanged at 86628e9f1. Its uncommitted fix round still uses the same names, defaults, `limits.supplied` carriage and phase-marked projection.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 461620c |
| FND-002 | fixed | 461620c |
| FND-003 | fixed | 461620c |
| FND-004 | fixed | 461620c |
| FND-005 | fixed | 461620c |
