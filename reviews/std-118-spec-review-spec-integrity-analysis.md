---
id: SR-13665
title: "Integrity review of QSpec PR #320 (STD-118) replay selected item"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@b1c0365; spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-278-replay-envelope-members.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-278
    type: reviews
---

## Summary

Ticket: STD-118. Completeness, atomicity and verifiability of FR-323-AC-6, FR-323-AC-18 and TC-278 RE-30..RE-32. AC-6's edit is consistent with the statement. Every new row binds FR-323-AC-18 and AC-18's verification column names TC-278. The rows are prose only: no schema or test exists for the replay request.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-323-AC-6 | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| FR-323-AC-18 | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| TC-278-RE-30 | spec/test-cases/TC-278-replay-envelope-members.md | examined |
| TC-278-RE-31 | spec/test-cases/TC-278-replay-envelope-members.md | examined |
| TC-278-RE-32 | spec/test-cases/TC-278-replay-envelope-members.md | examined |
| FR-323-executor | spec/objects/interfaces/FR-323-native-runtime-envelope.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-18 and the executor paragraph name no code or cause for the both, neither and mismatch refusals (only "refused before consumption"), while the same paragraph names one for the undeclared case and FR-323's other replay refusals name theirs (for example `invalid_package`/`invalid-value`). FR-047 and FR-272 require a catalogued typed cause and locus. Name the pair and the locus (for example `/replay/function`, `/replay/frame_operation`). | spec/objects/interfaces/FR-323-native-runtime-envelope.md:481, 137-139 |
| FND-002 | medium | RE-31's expected result, "Each refused before consumption", gives no code, cause or locus, so any pre-consumption refusal, including one caused by the mutation breaking another member, passes all three cases. State each case's expected code, cause and locus once AC-18 names them. | spec/test-cases/TC-278-replay-envelope-members.md:98 |
| FND-003 | medium | AC-18 requires a frame request's occurrence key to be the frame node's `generated` occurrence, but no row fails when it is not: RE-30 is the positive case only. Add a vector with the frame request's occurrence key replaced by an `anchor` occurrence, another node's occurrence, or the frame's occurrence at another ordinal, and its expected refusal (QSL FR-116 refuses `stale_dependency`/`content-mismatch` naming both). | spec/objects/interfaces/FR-323-native-runtime-envelope.md:481, 97 |
| FND-004 | low | AC-18 joins six obligations (two carriage rules, the occurrence-key rule, three member refusals and the undeclared refusal). Split it into the carriage rule, the member refusals and the resolution refusal so each row of TC-278 binds one criterion. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:481 |

## Verdict

0 high and 3 medium finding(s) in this lens; not mergeable until the medium findings are fixed or dispositioned.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The member refusals use the package layer for a runtime-request member. FR-271 gives `invalid_package` to "a malformed, incomplete, conflicting or cyclic package definition" and `invalid_runtime_input` to "malformed, incomplete, conflicting or wrongly-mapped runtime input"; both pairs exist in FR-272 for either code. FR-323 uses `invalid_package` only for the `/package` reference (`/package/dependencies`), and QSL FR-098 refuses a replay request member as `invalid_runtime_input` (`object_environment`, `invalid_runtime_input`/`unknown-member`, FR-098 l.351 and AC-17). Use `invalid_runtime_input`/`missing-member` at `/replay` and `invalid_runtime_input`/`invalid-value` at `/replay/frame_operation` in the statement, AC-19 and RE-31..RE-33, or state why `/replay` is a package-layer member. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:150-152, 504 |

## Dispositions

Round 1, head c0543ce (3470fee, c0543ce).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | RE-36's second locus, "the occurrence-key member's pointer", names no pointer: FR-323 (l.122-123, 155) gives the occurrence key no wire member name, so the row cannot assert an exact locus and any pointer under `/replay` passes. Name the member in the `replay` bullet (and backtick `arguments` there) and give the pointer in step 1, AC-19 and RE-36. | spec/test-cases/TC-278-replay-envelope-members.md:103 |

## Dispositions (round 2)

Round 2, head bfd64fb.

| FND | outcome | reason |
| --- | --- | --- |
| FND-005 | fixed | - |
