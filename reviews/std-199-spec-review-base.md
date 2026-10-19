---
id: SR-13065
title: "Base checklist review of QSpec PR #285 (STD-199)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@4d0b395; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md, spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-323-native-runtime-envelope.md, proposals/quire-v1/definitions/native-diagnostics.md, proposals/checked-package-v2/schema.json, TC-047, TC-233, TC-277, TC-310, TC-386"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-432
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
---

## Summary

Ticket: STD-199. Base checklist (ID formats, duplicates, AC-to-TC coverage) over the diff `origin/main...4d0b395`. `quire validate` on FR-375, FR-432 and the five TCs exits 0; the computed `quire matrix` was read to confirm which new criteria are parsed. ID collisions were checked against every open QSpec PR branch.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-AC-12 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-13 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-375-AC-14 | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-432-AC-7 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432-AC-8 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432-AC-9 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432-AC-10 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-432-AC-5 | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | examined |
| FR-322-AC-75 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-323-AC-17 | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| FR-272-AC-20 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| TC-386-IL-01 | spec/test-cases/TC-386-explicit-protocol-step-rows.md | examined |
| TC-386-SR-08 | spec/test-cases/TC-386-explicit-protocol-step-rows.md | examined |
| TC-310-m16 | spec/test-cases/TC-310-refinement-declaration-admission.md | examined |
| TC-277-BP-14 | spec/test-cases/TC-277-replay-byte-provision.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-375-AC-12, AC-13 and AC-14 are appended after the blank line that ends the Acceptance Criteria table, so they form a headerless second table. `quire matrix --scope .` at 4d0b395 lists FR-375-AC-10 and AC-11 but none of AC-12..14 (FR-432-AC-7..10, FR-322-AC-75, FR-323-AC-17 and FR-272-AC-20 all appear), so the three criteria are invisible to the matrix and to trace tags. Remove the blank line before the AC-12 row. | spec/functional/analysis/FR-375-declare-model-refinement.md:299-303 |
| FND-002 | medium | ID collision with open QSpec PR #283 (IR-883, head 3bd59b5): #283 also adds FR-322-AC-75 (diamond closure refusal) and FR-322-AC-76, and its own TC-277 BP-14 (transitive-only entry edit); both PRs also rewrite the same FR-323 replay `dependencies` paragraph (lines 31-35). Whichever merges second carries duplicate criterion and vector ids. Coordinate: renumber this PR's criterion (e.g. FR-322-AC-77) and vector (BP-15) or rebase on #283. PR #200's BP-14..17 are in TC-431 and do not collide. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1583; spec/test-cases/TC-277-replay-byte-provision.md:50 |
| FND-003 | medium | FR-432-AC-8 requires a node path for an event, timeout and fence node to spell and resolve, but the IL-01 fixture declares no event, timeout or fence node and IL-02 names none, so three of the AC's classes have no vector. Add those nodes to `Full` and name each in IL-02. | spec/test-cases/TC-386-explicit-protocol-step-rows.md:41-42; spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:190 |
| FND-004 | low | The FR-432 defect table lists a non-string `refinement` as `malformed-declaration`, and AC-9 covers "mistyped key", but IL-03 mistypes only `internal` (`"internal": "x"`); no vector supplies e.g. `"refinement": 7`. | spec/test-cases/TC-386-explicit-protocol-step-rows.md:43; spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:166,191 |
| FND-005 | low | SR-08 still reads "whose profile's internal list is empty" after the PR moved the list off the profile (AC-5 now says "the refinement's internal list"); a reader looks for a profile member that does not exist. | spec/test-cases/TC-386-explicit-protocol-step-rows.md:39 |

## Verdict

Not mergeable until FND-001 (three criteria the matrix cannot see) and FND-002 (id collision with #283) are fixed. The other new criteria (FR-432-AC-7..10, FR-322-AC-75, FR-323-AC-17, FR-272-AC-20) parse, each names a TC that carries a matching vector, and TC-047, TC-233, TC-277 and TC-310 rows cite their criteria. FND-003..005 are coverage and wording gaps.

## Dispositions

Round 1, against head 986087c.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 986087c: blank line removed; `quire matrix --scope .` now lists FR-375-AC-12, AC-13 and AC-14 |
| FND-002 | fixed | 986087c: renumbered to FR-322-AC-77 and TC-277 BP-15; #283 (head 3bd59b5) still adds only FR-322-AC-75/76 and BP-14, and no open PR adds AC-77, BP-15 or the other new ids. The FR-323 replay paragraph both PRs edit will still need a textual rebase |
| FND-003 | fixed | 986087c: IL-01 adds event `ev`, timeout `to`, fence `fe`; IL-02 names each (17 classes, count checked) |
| FND-004 | fixed | 986087c: IL-03 adds the non-string `refinement` vector and the count is updated to five; FR-432-AC-9 names it |
| FND-005 | fixed | 986087c: SR-08 reworded |
