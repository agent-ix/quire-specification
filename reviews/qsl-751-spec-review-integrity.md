---
id: SR-9026
title: "Integrity review of QSpec PR #249 (QSL-751 follow-ups)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@a8689c3d01eb084ae596c883451da255a2dbca65; spec/functional/type-model/FR-150-*.md, FR-153-*.md, spec/objects/foundation/FR-271-*.md, FR-272-*.md, spec/test-cases/TC-195-*.md, TC-196-*.md, TC-198-*.md, TC-233-*.md, proposals/quire-v1/definitions/value-accounting.md, native-diagnostics.md, shared-grammar.md"
review_set: subset
---

## Summary

Ticket: QSL-751. ID uniqueness holds. FR-150-AC-18, FR-153-AC-10/11,
FR-272-AC-18/19, TC-196 C10 and TC-198 L12/L13 appear on no other origin
branch. FR-150-AC-12..17 exist only on `spec/670-identity-cost` (#209), and
AC-18 skips them correctly. The cross-reference sweep found no remaining
"semantic maximum" phrase in QSpec. FR-272's verbatim rule quote matches
native-diagnostics. `required` was added to the canonical payload-name list,
which is still parseable by `canonical_payload_names`. New TC rows keep each
table's three-column shape. QSpec validation of the changed files fails only on
FR-271 and FR-272 ("required 'id' missing", FR-271 Identifier cells, FR-272
"fields" record). origin/main 74738f6 has identical failures, so they are
pre-existing. The PR adds no file-tracking or pin content.

Sibling sweep for other enumerations of the two limits types' field lists:
QSpec holds none in JSON or schemas. quire-contract-ir,
quire-verification-contracts, quire-integration, quire-contract-codegen,
quire-contract-runtime, quire-rs, quire-exact and quire-semantic-value have
none. quire-protocol has an unrelated `max_population_members`. The one stale
enumeration is TC-195 N01 (FND-002).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The TC-198 L12 oracle depends on an undefined `n`. In PA (member types `[A, B]`), `b1`'s type `B` is itself a declared member type. Whether its coverage walk follows 0 or 1 edges depends on the order of the declared member types and on whether identity is tested before walking, and QSpec states neither. QSL walks `[A, B]` in order and follows 1 edge. The `work_units: 0` step denies `a1`'s `binding.member` before any walk, so it never creates the "simultaneous" edge denial it claims to test. | TC-198 L12, FR-153-AC-10, proposals/quire-v1/definitions/value-accounting.md:489-497 | wrong-requirement |
| FND-002 | low | TC-195 N01 writes `ModelNormalizationLimitsV1 { ... }` with six fields and omits `ancestor_steps` and, after this PR, `family_steps`. `value-accounting.md` says "Omission is a refused request", so read literally N01 is refused. This was pre-existing for `ancestor_steps`, and the PR widens it. Owner: QSpec spec/test-cases/TC-195-model-normalization-provenance.md:54 (also touched by #209). | TC-195 N01, proposals/quire-v1/definitions/value-accounting.md:512-516 | wrong-requirement |
| FND-003 | low | FR-153-AC-11 lists `abstraction-decl` in its "names exactly" set, but its verification TC-198 L13 does not exercise it. The obligation is already FR-450-AC-3 (TC-410), and FR-153 does not own that form, so AC-11 restates FR-450-AC-3 without its own test. | FR-153-AC-11, FR-450-AC-3, TC-198 L13 | wrong-requirement |

## Verdict

Findings: one medium and two low. ID allocation, the wording sweep and the
canonical-name list are clean. The pre-existing validation failures are
confirmed against origin/main.
