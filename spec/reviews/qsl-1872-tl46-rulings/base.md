---
id: SR-9135
title: "Base review of the TL-46 rulings (role internality, choice-visibility carrier, await false-constraint truth)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification PR #649 at 921f42368a59fb6adb31c6743beec46d78acef95; diff origin/main...HEAD: FR-049-AC-14, FR-050-AC-10, FR-061-AC-18, FR-172-AC-6, FR-211, FR-217-AC-5, FR-479, TC-050, TC-061, TC-174"
review_set: subset
review_date: "2026-11-12"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-217, type: references }
  - { target: ix://agent-ix/quire-specification/FR-050, type: references }
  - { target: ix://agent-ix/quire-specification/FR-061, type: references }
  - { target: ix://agent-ix/quire-specification/TC-050, type: references }
---

## Summary

Tickets: QSL-1872, QSL-1874, QSL-1871. Base checklist over the changed ACs
and TCs: id format, EARS form of the new statements (all use SHALL with a
named actor or an If/When condition), testability of each new AC and TC
coverage. `quire validate` (quire 0.38.3, engine 0.52.1) over the ten changed
files fails only FR-211 and FR-217 with `required 'id'`; the same two errors
reproduce on origin/main, so they are pre-existing, as the PR states.
FR-049-AC-14 (TC-174), FR-172-AC-6 (TC-203, which is generic over FR-172) and
the FR-050-AC-10 internal/external/absence cases are covered. Gaps: the new
FR-217-AC-5 contradiction refusal is not exercised by TC-050, FR-061-AC-18 has
no observable discriminator, and FR-217 keeps two static-role phrasings and an
undefined "one binding".

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-061-AC-18 requires a strict reader to refuse "a record that presents [branch selection] as a result with truth or settlement basis", but FR-061 defines no member or request identity that distinguishes such a record from a result of a requested assessment, and TC-061 Expected Results do not name the refusal; the refusal has no testable trigger. | spec/functional/protocol/FR-061-report-orthogonal-results.md:162-169,242, spec/test-cases/TC-061-report-orthogonal-results.md:111 | wrong-requirement |
| FND-002 | medium | FR-217-AC-5's new refusal (two different internalities for one role under one binding) and FR-050-AC-10's "the checked static role carries none" are both bound to TC-050, whose procedure exercises only exact internal/external, absence, unknown/cased value and inference. | spec/objects/protocol/FR-217-role-internality.md:63, spec/functional/protocol/FR-050-bind-protocol-instances.md:112, spec/test-cases/TC-050-bind-protocol-instances.md:37-39 | correct-requirement-no-evidence |
| FND-003 | low | "one binding" in FR-217 Key and FR-217-AC-5 is undefined: it does not say whether the unit is a workflow instance, a role instance or one binding-authority input, so whether two workflow instances may bind one static role with different internalities is open. | spec/objects/protocol/FR-217-role-internality.md:39-44,63 | wrong-requirement |
| FND-004 | low | FR-217 residue still frames internality per static role: Values rows say "Exact lowercase spelling for a static role" and Absence says "every static role requires exactly one member", where the ruling makes it per bound role from the binding authority. | spec/objects/protocol/FR-217-role-internality.md:26-27,51 | wrong-requirement |

## Verdict

The rulings themselves are coherent and the pre-existing validation claim
holds. FND-001 and FND-002 need a fix round before merge; FND-003 and FND-004
are wording.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-005 | low | FR-061 Result scope and FR-061-AC-18 cite FR-061-AC-4 for a "mismatched claim subject" refusal, but FR-061-AC-4 covers only an impossible dimension combination or conflicting claim strength; the citation does not support the refusal it names (AC-18 itself now states the refusal, so it remains testable). | spec/functional/protocol/FR-061-report-orthogonal-results.md:164-169,232,246 | wrong-requirement |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-061-AC-18 now refuses a `ProtocolResult` whose subject is a selection event rather than a requested requirement, and TC-061 Expected Results name the refusal (aaf84855). |
| FND-002 | fixed | TC-050 procedure and Expected Results now exercise two internalities for one bound role under one binding and a checked static role carrying an internality member (aaf84855). |
| FND-003 | fixed | FR-217 Key and AC-5 define one binding as one binding-authority input for the bound roles of one workflow instance (aaf84855). |
| FND-004 | fixed | FR-217 Values rows and Absence bullet now say bound role (aaf84855). |
| FND-005 | fixed | FR-061 Result scope and AC-18 drop the FR-061-AC-4 citation; AC-18 and TC-061 name the refusal as a non-requested subject (8158d9f9). |
