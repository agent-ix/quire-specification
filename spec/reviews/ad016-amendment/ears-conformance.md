---
id: SR-621
title: "EARS conformance review of the AD-016 amendment"
type: SpecReview
analysis: ears-conformance
scope: "quire-specification task/ad016-amendment diff 55d2fcc..5bd933f; FR-197 Description and Replay source SHALL statements; FR-197-AC-6..AC-10; AD-016 (no SHALL statements)"
review_set: all
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/AD-016, type: references }
  - { target: ix://agent-ix/quire-specification/FR-197, type: references }
---

## Summary

The diff adds or changes five SHALL statements, all in FR-197: the edited
Description and four bullets under "Replay source". The engine check reports
both touched documents grammar-clean, but it reads only the Description, so it
does not see the four bullets. Semantic review finds three of the five
statements clean. The dominant defects are one bullet that packs two SHALLs,
a new subject ("replay boundary") that no spec defines, and statements that
leave the `Input` arm's disagreement and the "not accepted where evidence is
accepted" rule with no SHALL of their own.

## Verdict

**ACCEPT WITH FINDINGS**: FND-001 through FND-004 (medium) should be fixed in
this PR. The low findings can be fixed in this PR or recorded. None of them
changes the ruled design; each is a wording or split fix.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Non-singular. The fourth Replay source bullet holds two SHALLs: "SHALL derive whether a replay is witness-backed by matching on `ReplaySource`, and SHALL store no witness-backed field". They map to two ACs (AC-10 and AC-9). The engine misses it because it does not scan bullets outside the Description. Fix: split into two bullets, "The replay boundary SHALL derive whether a replay is witness-backed by a match on `ReplaySource`." and "The replay boundary SHALL store the witness-backed answer only as the `ReplaySource` arm." | FR-197; FR-197-AC-9; FR-197-AC-10 |
| FND-002 | medium | Undefined subject. The four new bullets name "the replay boundary". No spec defines it; the Description and FR-280/FR-282 use "the verification boundary". One FR now has two subjects with no stated relation, so the reader cannot tell which component owns the obligations (AD-016 splits arrow 7 between IR packet, CG replay adapter and QSL executor). Fix: use "the verification boundary" in all four bullets, or add one sentence to the Replay source intro: "The replay boundary is the part of the verification boundary that runs AD-016 arrow 7 (the CG replay adapter)." | FR-197; AD-016 |
| FND-003 | medium | Missing statement for a ruled obligation. The coordinator's rule "`reproduced-without-witness` cannot be represented where a backend-evidence verdict is accepted" has no SHALL. FR-197-AC-7 tests it, but it follows only by combining bullet 1 (distinct types) and bullet 3 (constructor restriction). An AC should trace to one statement. Fix: add a bullet, "The replay boundary SHALL accept a backend-evidence verdict, in every parameter, field and return position, only as the backend-evidence verdict type, which no `Input`-arm result converts into." | FR-197; FR-197-AC-7 |
| FND-004 | medium | Right pattern missing (unwanted behaviour). Bullet 2 settles only an agreeing `Input` replay. No statement covers an `Input` replay that disagrees or fails to decode. The Behavior section calls a mismatch "an explicit backend parity failure", which does not fit an arm that ran no backend. What gets built for this case is open. Fix: add an If/then bullet, "If a replay of the `Input` arm disagrees with its stored answer, then the replay boundary SHALL settle it as a parity failure in the `Input`-arm result type.", and add an AC and a TC-272 row for it. | FR-197; FR-197-AC-6; TC-272 |
| FND-005 | low | Trigger scope. The Description trigger is "When a proof backend reports a counterexample". An `Input`-arm counterexample ran no backend, so the one statement that fixes the executor (QSL complete-V1, same package and domain) does not cover `Input` replays. Fix: widen the trigger to "When a counterexample is replayed" or add "The replay boundary SHALL replay an `Input`-arm counterexample through the same executor under the same semantic package and domain." | FR-197 |
| FND-006 | low | Asymmetric settlement. Bullet 2 names the `Input`-arm settlement (`reproduced-without-witness`), but no statement names the `Witness`-arm settlement that AD-016 arrow 7 and Replay source call reproduced-with-evaluated-witness. Fix: add "When a replay of the `Witness` arm agrees, the replay boundary SHALL settle it reproduced-with-evaluated-witness in the `Witness`-arm result type." | FR-197; AD-016 |
| FND-007 | low | Subject-vs-intent. Bullet 3 ("The replay boundary SHALL construct a backend-evidence verdict only from a `Witness`-arm result") restricts one component, while FR-197-AC-8 constrains every public constructor of the verdict type, wherever it is called. Fix: make the verdict type the subject, "The backend-evidence verdict type SHALL expose public constructors that accept only a `Witness`-arm result." | FR-197; FR-197-AC-8 |
| FND-008 | low | Vague comparand. Bullet 2 says "When a replay of the `Input` arm agrees" without saying what it agrees with. Fix: "agrees with the answer stored with its input". | FR-197 |
| FND-009 | low | Compound response, kept from before the diff. The edited Description has one SHALL with two responses, "serialize its typed inputs canonically and replay them". The diff changed this sentence. Fix: optional; split into a serialize SHALL and a replay SHALL. | FR-197 |

## Statement checklist

| Statement | Pattern | Engine | Semantic result |
| --- | --- | --- | --- |
| Description: "When a proof backend reports a counterexample, the verification boundary SHALL serialize ... and replay them through the QSL complete-V1 executor ..." | Event-driven | clean | Compound response (FND-009); trigger excludes the `Input` arm (FND-005) |
| Bullet 1: "The replay boundary SHALL yield a replay result of one type for the `Witness` arm and of a distinct type for the `Input` arm." | Ubiquitous | not scanned | Clean apart from the subject (FND-002) |
| Bullet 2: "When a replay of the `Input` arm agrees, the replay boundary SHALL settle it `reproduced-without-witness` ..." | Event-driven | not scanned | "When" fits: agreement is a momentary event. Comparand vague (FND-008); no If/then for disagreement (FND-004) |
| Bullet 3: "The replay boundary SHALL construct a backend-evidence verdict only from a `Witness`-arm result." | Ubiquitous | not scanned | Subject narrower than AC-8 (FND-007) |
| Bullet 4: "The replay boundary SHALL derive ... and SHALL store no witness-backed field ..." | Ubiquitous | not scanned | Non-singular (FND-001) |

## Acceptance criteria check

FR-197-AC-6 through AC-10 are singular direct assertions with no "shall", as
the repo's grammar check requires. Each is verifiable by test (TC-272). AC-7
has no single parent SHALL (FND-003). The `Input`-arm disagreement case has no
AC (FND-004).

## Out of scope

AD-016 is an assurance decision, not an FR, NFR or StR, and the diff adds no
SHALL statement to it. Its new "Replay source" rationale was read only as
context for FR-197. TC-272 and spec/complete-v1/tests.md are tests and are out
of this lens's scope.

## Evidence

- Diff read: `git -C /home/peter/dev/worktrees/qspec-ad016-amend diff 55d2fcc..HEAD`.
- Engine check (quire 0.32.0, engine 0.46.0): `quire validate --scope . FR-197 AD-016 --summary` reports "2/2 docs grammar-clean (100%); 0 grammar finding(s)". `--strict` on FR-197 reports no EARS warning. The four Replay source bullets are not in the scanned Description, which is why FND-001 is not engine-reported.
- "replay boundary" appears nowhere in `spec/` outside FR-197; "verification boundary" appears in FR-197, FR-280 and FR-282.

## Round 2 dispositions

Evaluated against the tree committed with these records (fixes in `201b4a4`, plus the TC-272 RS-13 compile-fail case). Paths are relative to `spec/`.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | FR-197 Replay source: derive-by-match and no-stored-field are separate bullets |
| FND-002 | resolved | FR-197 Replay source defines the replay boundary as the CG replay adapter together with the QSL layer-6 `replay` facade, within the verification boundary |
| FND-003 | resolved | FR-197 Replay source bullet: the replay boundary SHALL accept `reproduced-without-witness` in no position that accepts a backend-evidence verdict; FR-197-AC-8 |
| FND-004 | resolved | FR-197 Replay source disagreement bullet; FR-197-AC-13; TC-272 RS-11 (`Witness`), RS-12 (`Input`); AD-016 Replay source last paragraph |
| FND-005 | resolved | FR-197 Description, Inputs and Replay envelope cover both arms; `Input` defined as "did not come from a backend transcript (a corpus counterexample)" in AD-016 Replay source and FR-197 |
| FND-006 | resolved | FR-197 Replay source `Witness` agreement bullet; FR-197-AC-7; TC-272 RS-01 |
| FND-007 | resolved | FR-197 Replay source: "The backend-evidence verdict type SHALL have exactly one construction path" |
| FND-008 | resolved | FR-197 Replay source: a replay agrees when the native verdict equals the envelope's expected disposition, mapped by the QSL #231 outcome-to-verdict map; Replay envelope: an `Input`-arm envelope's expected disposition is the one recorded with its counterexample |
| FND-009 | resolved | FR-197 Description: one serialize SHALL and one replay SHALL |
