---
id: SR-610
title: "Evidence review of the FR-290 capability-vocabulary widening"
type: SpecReview
analysis: evidence
scope: "quire-specification #134 diff 818f555..5d3aa25: FR-290-AC-1..8, FR-331-AC-4..6, FR-272-AC-9, FR-349-AC-2/5, FR-355-AC-6, TC-060, TC-061, TC-266, TC-270, TC-271, spec/protocol/tests.md, spec/complete-v1/tests.md"
review_set: all
evaluated_revision: "5d3aa25+wt"
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-290, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
---

## Summary

ACCEPT WITH FINDINGS. `quoin advise` at `5d3aa25` reports no method mismatch
and no inconclusive obligation for any criterion in scope. Every one is
authored `Test` and the catalog recommends a Test-class method
(property-based, unit or example). TC-271 is a real negative-vector corpus. It
refuses unknown labels, modes, versions and backends, checks both
registration orders, and runs the tool-absence path. Both matrices list every
new criterion against TC-271.

The gaps are vectors that cannot fail. Rows 1 and 5 of the advertised-mode
table have no vector. TA-01 says "no other candidate runs" with one candidate
in the fixture. CK-06 checks only a refused `case`, never an admitted one.
FR-290-AC-2/AC-3 were widened from six to ten kinds, and TC-060 still
enumerates protocol claims only. FR-349-AC-2 and its new vector SMT-01b
describe different obligation forms. FR-355-AC-6 lost its oracle when
obligation forms stopped being advertised. None of these needs a design
change. Each one is fixed by adding a vector or editing AC text in this PR.

## Review set

Scope is this diff only. Out of scope, per the settled rulings: FR-290 as the
single vocabulary authority, `unsupported`-with-warning for absence, CG
`negotiate_*` as the one negotiation point, refusal of QSL's four-kind
vocabulary, and the extent and finite-bound predicate that QSL #222 owns.
FR-331-AC-1..3 (Analysis) and FR-349-AC-1/3/4/6 are unchanged in method and
are not re-assessed.

## Method confirmation

| Criterion | Authored | Catalog recommendation (matched rule) | Evidence vectors | Can fail? |
| --- | --- | --- | --- | --- |
| FR-290-AC-1 | Test | property-based / design-by-contract (invariant) | TC-271 CK-01..03; TC-060, TC-061 | Partly: the identity mutation is not exercised (FND-008) |
| FR-290-AC-2 | Test | property-based (universal) | TC-060, TC-061 | No for non-protocol kinds (FND-004) |
| FR-290-AC-3 | Test | property-based (universal) | TC-060, TC-061 | No for non-protocol kinds (FND-004) |
| FR-290-AC-4 | Test | property-based (universal) | TC-271 NG-01; TC-060 | Yes |
| FR-290-AC-5 | Test | property-based (universal) | TC-271 CK-05, CK-06 | Partly (FND-003, FND-009) |
| FR-290-AC-6 | Test | property-based (invariant) | TC-271 NG-05..07 | 3 of 5 table rows (FND-001) |
| FR-290-AC-7 | Test | property-based (universal) | TC-271 NG-02, NG-04 | Yes |
| FR-290-AC-8 | Test | property-based (universal) | TC-271 TA-01..03 | "No other candidate runs" is vacuous (FND-002) |
| FR-331-AC-4 | Test | unit / example | TC-271 NG-01..04, NG-08 | Partly: no named-but-incapable backend vector (FND-007) |
| FR-331-AC-5 | Test | golden-approval (serialization) / unit | TC-271 NG-09 | Yes |
| FR-331-AC-6 | Test | property-based (universal) | TC-271 TA-01 | Yes |
| FR-272-AC-9 | Test | unit / example | TC-271 CK-02..04, NG-01/02/04/06, TA-01/02 | The obligation-form cause has no vector (FND-005) |
| FR-349-AC-2 | Test | unit / example | TC-266 SMT-01, SMT-01b | The AC and the vector disagree (FND-006) |
| FR-349-AC-5 | Test | property-based (universal) | TC-266 SMT-01 | Yes |
| FR-355-AC-6 | Test | unit / example | TC-270 DP-01 | Only the empty-corpus half (FND-010) |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-290-AC-6 claims every row of the advertised-mode table. TC-271 covers rows 3, 4 and 2 (NG-05, NG-06, NG-07) only. Row 1 (`bounded` extent on a `bounded` advertisement) and row 5 (`bounded` extent on an `unbounded`-only advertisement, which settles at the arm) have no vector. An implementation that refuses row 5 as a mode mismatch passes TC-271. Fix: add NG-10 (bounded/bounded goes to the arm, never `requires-bound`) and NG-11 (bounded extent on an unbounded-only advertisement goes to the arm, never `unsupported` or `requires-bound`). | FR-290-AC-6; TC-271 | correct-requirement-no-evidence |
| FND-002 | medium | FR-290-AC-8 and TC-271 TA-01 assert "no other candidate runs" on tool absence. TA-01 builds no second registrant for the kind, so no fallback target exists and the assertion cannot fail. Mode downgrade is also untested: nothing checks that a backend advertising both modes does not retry under the other mode. Fix: TA-01 fixture has two registrants advertising the kind and the request names one; assert the other registrant's adapter is never invoked. Add TA-04, where the routed backend advertises (kind, `bounded`) and (kind, `unbounded`), and assert one run in the negotiated mode. | FR-290-AC-8; TC-271 TA-01 | correct-requirement-no-evidence |
| FND-003 | medium | FR-290-AC-5 says a `case` exhaustiveness obligation "requires none and requests no backend". CK-06 exercises only a non-exhaustive `case`, which refuses at admission, so it would request no backend in any implementation. The case that can fail is an exhaustive `case`, admitted, whose containing clause requests `value-validity` while the exhaustiveness obligation itself produces no candidate set. Fix: add CK-07. Admit an exhaustive `case` over a union. The clause's item requires exactly `value-validity`, and no request item or candidate set exists for the exhaustiveness obligation. | FR-290-AC-5; FR-146; TC-271 CK-06 | correct-requirement-no-evidence |
| FND-004 | medium | FR-290-AC-2 ("every pair of claim kinds") and FR-290-AC-3 ("a non-global claim") now range over ten kinds. Their only evidence is TC-060 and TC-061, whose fixtures and procedure are protocol-only. No vector pairs `value-validity`, `operation-contract`, `finite-replay` or `temporal-satisfaction` with each other or with a protocol kind. Fix: map AC-2/AC-3 to TC-271 and add a vector where two items of different families settle with independent outcomes and no premise transfer, and a `value-validity` result carrying the global-conformance premise set is refused as success. Alternatively, narrow the AC text to protocol-family members if that is the intent. | FR-290-AC-2; FR-290-AC-3; TC-060; TC-061 | correct-requirement-no-evidence |
| FND-005 | medium | FR-272-AC-9 requires "unsupported obligation form carries `unsupported_projection`/`unsupported-requested-capability`". TC-271 has no obligation-form vector. The only one, TC-266 SMT-01b, asserts no catalog code or cause and is not mapped to FR-272-AC-9 in either matrix. Fix: add the code/cause assertion (`unsupported_projection`/`unsupported-requested-capability`, retaining the obligation form) to SMT-01b and map FR-272-AC-9 to TC-266 as well, or add an NG vector to TC-271 with a fixture arm that refuses one obligation form. | FR-272-AC-9; TC-266 SMT-01b; spec/complete-v1/tests.md | correct-requirement-no-evidence |
| FND-006 | medium | FR-349-AC-2 says an obligation form *outside* quantification, induction, compositional discharge and mathematical integers settles `unsupported` at the arm. The FR-349 description and SMT-01b say an undischarged form, and SMT-01b uses induction, which is *inside* that set. The vector does not test the AC as written, and the AC does not state the behaviour the vector tests. Fix: change the AC to "an obligation form its negotiation arm does not discharge settles `unsupported` at that arm with a warning naming the form", matching the description and SMT-01b. | FR-349-AC-2; TC-266 SMT-01b | wrong-requirement |
| FND-007 | low | The FR-290 candidate-set rule makes the set empty when the request names a registered backend that lacks a required kind. The warning then names the kinds that backend leaves unadvertised, not the kinds across all registrants. No vector names a registered but incapable backend while another registrant is capable. Fix: add NG-12. Registrant A advertises `value-validity` and B does not; the request names B. The result is `unsupported` with a warning naming `value-validity` against B, and A never runs. | FR-331-AC-4; FR-290-AC-7; TC-271 | correct-requirement-no-evidence |
| FND-008 | low | FR-290-AC-1 refuses a carrier declaring an identity other than `quire.capability-kind/v1`. CK-03 mutates only the version (non-v1, absent). A carrier declaring a different identity name at `/v1` is untested. Fix: extend CK-03 with a foreign identity at v1 (for example `quire.claim-kind/v1`), refused as `invalid_capability`/`unsupported-version` with none of its labels read. | FR-290-AC-1; TC-271 CK-03 | correct-requirement-no-evidence |
| FND-009 | low | CK-05 samples the claim-form table. It skips operation precondition and invariant, the infinite-trace temporal profile, and five of the six protocol-family members. FR-290-AC-5 says "each claim form". Fix: extend CK-05 to every table row, including both temporal profiles and each protocol member mapping to itself. | FR-290-AC-5; TC-271 CK-05 | correct-requirement-no-evidence |
| FND-010 | medium | FR-355-AC-6 requires one corpus member per obligation form the FR-349 backend discharges. After this diff those forms are settled at the negotiation arm and are not advertised, so the set has no declared source for the check to compare against. DP-01 mutates only to an empty corpus, so a non-empty corpus missing one form still passes. Fix: declare where the discharged-form set is read from, such as a declared list in the SMT descriptor that the arm must agree with. Add DP-03: remove the only member of one discharged form and expect refusal naming that form. | FR-355-AC-6; FR-349; TC-270 DP-01 | correct-requirement-no-evidence |
| FND-011 | low | FR-061-AC-17 and FR-240 now restrict a protocol result to a protocol-family kind. TC-061 has no vector where a protocol result carrying `value-validity` (or another non-protocol member) is refused, so the narrowing cannot fail. Fix: add that mutation to TC-061's procedure and expected results. | FR-061-AC-17; FR-240; TC-061 | correct-requirement-no-evidence |
| FND-012 | low | Matrix consistency. In spec/complete-v1/tests.md, V1-BACK-017 (FR-290, Amend) still lists TC-060 and TC-231 only, while its TC catalog lists TC-271 as verifying FR-290. TC-271 is P1 in spec/protocol/tests.md and P0 in spec/complete-v1/tests.md. FR-272-AC-9 has no vector asserting that a producer claiming revision `1-draft.4` emits none of the new causes, or that the registration/request index is retained. Fix: add TC-271 to V1-BACK-017, use one priority for TC-271 in both catalogs, and add a retained-index and revision-gate assertion to CK-02/CK-04. | spec/complete-v1/tests.md; spec/protocol/tests.md; FR-272-AC-9 | correct-requirement-no-evidence |

## Deterministic advisor output

`quoin advise --json` at `5d3aa25`, filtered to the fifteen criteria in scope:
`mismatch: false` and `inconclusive: false` for every one. FR-331-AC-5
additionally matches `golden-approval-testing` on `serialization`, and NG-09's
byte comparison under both registration orders satisfies it. No finding above
comes from the advisor. All of them are reviewer judgement on whether a
vector can fail. The advisor does not assess that.

## Round 2 dispositions

Evaluated against the uncommitted working tree on 5d3aa25. Paths are relative to the repository root. TC-271 vectors are at `spec/test-cases/TC-271-capability-kind-candidate-negotiation.md:38-64`.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | NG-10 (TC-271:54) and NG-11 (TC-271:55) cover advertised-mode rows 1 and 2 |
| FND-002 | resolved | TA-01 (TC-271:60) has two registrants and asserts the other never runs; TA-04 (TC-271:63) covers mode downgrade |
| FND-003 | resolved | CK-07 (TC-271:44): an admitted exhaustive `case` requests no kind or backend |
| FND-004 | open | medium. FR-290-AC-2 and AC-3 now cite TC-271 (`spec/objects/protocol/FR-290-protocol-claim-kind.md:230-231`; `spec/protocol/tests.md:259` maps FR-290-AC-1..AC-8), but no TC-271 vector pairs two non-protocol or cross-family items for independent outcomes, and none refuses a `value-validity` result that carries the global-conformance premise set. The added trace cannot fail. Fix: add a TC-271 vector for both properties, or narrow AC-2 and AC-3 to protocol-family members and drop TC-271 from them. |
| FND-005 | resolved | NG-15 (TC-271:59) asserts `unsupported_projection`/`unsupported-requested-capability`; SMT-04 (TC-266:40) asserts the cause |
| FND-006 | resolved | FR-349-AC-2 (`spec/functional/ir-proof/FR-349-register-smt-backend.md:81`), the FR-349 body (FR-349:57-60) and SMT-04 all use the declared obligation-form set |
| FND-007 | resolved | NG-12 (TC-271:56) |
| FND-008 | resolved | CK-03 (TC-271:40) includes `quire.foreign-kind/v1` |
| FND-009 | open | low. CK-05 (TC-271:42) now covers every non-protocol row, the protocol refinement row and `global-conformance`. The FR-290 row "Other protocol analysis claim → its protocol-family member" (FR-290:102) is sampled only by `global-conformance`, and `monitorability`, `local-projection`, `realizability` and `composition` are not derived. Fix: extend CK-05 to derive each of the six protocol members. |
| FND-010 | resolved | FR-349 declares the obligation-form set (FR-349:57); FR-355-AC-6 refuses an uncovered form; DP-03 (`spec/test-cases/TC-270-smt-runtime-differential-parity.md:30`) |
| FND-011 | resolved | TC-061:67-70 substitutes each non-protocol member and expects a mismatch refusal |
| FND-012 | open | low. The matrix parts are fixed: V1-BACK-017 (`spec/complete-v1/tests.md:217`) and P0 in both catalogs. No vector yet asserts the retained request-item index or backend identity, or that a producer at revision `1-draft.4` emits none of the new causes. Fix: add both assertions to CK-02 and CK-04. |

Round 2 verdict: ACCEPT WITH FINDINGS
