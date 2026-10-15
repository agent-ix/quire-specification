---
id: SR-616
title: "Integrity review of the AD-016 amendment"
type: SpecReview
analysis: integrity
scope: "quire-specification task/ad016-amendment diff 55d2fcc..5bd933f; AD-016; FR-197; TC-272; spec/complete-v1/tests.md; unchanged context IT-075, AD-010, FR-349, TC-219; source of truth QSL ADR-013 at 31b6054 (QC-7, QC-13 to QC-17, QC-20, O-16, O-25, O-27) and IR 954c2f2"
review_set: all
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/AD-016, type: references }
  - { target: ix://agent-ix/quire-specification/FR-197, type: references }
---

## Summary

This review checks the AD-016 amendment for completeness, consistency and
testability. The amendment covers every item TK-10 names: QC-7, QC-13 to
QC-17 and QC-20. It also adds the coordinator's structural rule as FR-197
AC-6 to AC-10 and TC-272, with matrix rows. The WP9 category table matches the
ADR-013 O-16 proof column row by row. The kernel row lists exactly the six
QC-15 types.

Six medium findings block. The executor change in FR-197 now conflicts with
IT-075, AD-010 and FR-349. FR-197's trigger and inputs still assume a backend,
so the new `Input` arm has no trigger. The `Witness` admission rule from O-25 is
missing, and the cited IR sha does not implement it. Arrow 7 still says
"Obligation id", which arrow 5 redefined. The backend-evidence verdict and the
two result types have no owner. AC-7 to AC-9 have open-ended oracles.

## Verdict

**REVISE**: FND-001 to FND-006 (medium) must be fixed. The low findings can be
fixed in this PR or recorded.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-197's Description now routes every proof-backend counterexample through the QSL complete-V1 executor. Unchanged context still names native `runtime::execute`. IT-075 verifies FR-197 and its Objective says "native `runtime::execute`". AD-010 ("replay through `runtime::execute`") and FR-349 (SMT counterexamples "replay through `runtime::execute` per AD-010") say the same for all backends. AD-016 arrow 7 says `runtime::execute` is not a replay target. Fix: amend the IT-075 Objective, the AD-010 counterexample paragraph and the FR-349 sentence to name the QSL complete-V1 executor reached through the layer-6 `replay` facade (AD-016 arrow 7). If SMT replay has a different executor, FR-197 must name it. | FR-197; IT-075; AD-010; FR-349; AD-016 arrow 7 |
| FND-002 | medium | FR-197's trigger and body assume a backend. The trigger is "When a proof backend reports a counterexample". Inputs say "Backend counterexample". The envelope holds "backend run identity" and "expected backend disposition". AC-5 keeps candidates that preserve "both replay verdicts". An `Input`-arm counterexample ran no backend, so the new Replay source rules have no trigger, and the envelope members and minimization are undefined for that arm. Fix: add an EARS clause for replay of an `Input`-arm counterexample. Change Inputs to "counterexample (backend transcript or stored input)". State which envelope members an `Input` packet omits. State that minimization of an `Input` packet preserves the native verdict only, or that it is out of scope. | FR-197 Description, Inputs, Replay envelope, AC-5 |
| FND-003 | medium | The Witness shape row records `Witness { transcript: String }` at IR `954c2f2`. It leaves out the ADR-013 O-25 admission rule: a `Witness` is admitted only through `parse`, including on deserialization, and `transcript` is private. At `954c2f2` (and at IR `origin/main` `65aa282`), `transcript` is `pub` and `Deserialize` is derived, which bypasses `parse`. ADR-013 makes admission through `parse` a precondition for recording the sha and for #231. The row's "a transcript that is not a single assertion block makes each of them refuse" describes the unadmitted state. Fix: add the admission rule to the Witness shape row as the contract. Cite `954c2f2` only for the transcript-only store, and name admission as open IR work (TK-04). | AD-016 Replay ownership, Witness shape; ADR-013 O-25; IR `src/kani/witness.rs:100-107` |
| FND-004 | medium | Arrow 7 "Identity retained" still reads "Obligation id, node id". Arrow 5 now says the obligation identity is the `KaniObligationIdentity` digest and "the clause node id alone never identifies an obligation". "Obligation id" can be read as either the digest or the clause node id. Fix: change the arrow 7 cell to "Obligation identity (arrow 5) and clause node id". | AD-016 arrows 5 and 7; QC-14 |
| FND-005 | medium | FR-197 AC-7 and AC-8 and TC-272 test a "backend-evidence verdict" type, a `Witness`-arm result type and an `Input`-arm result type. No artifact names these types or their owners. The AD-016 Replay ownership table has no row for them. ADR-013 O-27 splits result production (QSL executor, #231) from parity comparison (CG), so the owner is not implied. TC-272 cannot say which crate's API it compiles against. Fix: add Replay ownership rows naming each type and its owner, for example the two result types in QSL #231 and the backend-evidence verdict in the CG replay adapter. Reference them from FR-197. | AD-016 Replay ownership; FR-197 AC-7, AC-8; TC-272; ADR-013 O-27 |
| FND-006 | medium | AC-7, AC-8 and AC-9 have open-ended oracles. "Every parameter, field and return position" and "every public constructor" have no bounded set. "Witness-backed field" is undefined. The packet's own `source: ReplaySource` field decides witness-backing, so a literal reading of AC-9 flags it. Fix: bound the sets to the public API of the owning crates (FND-005), listed in TC-272 or enumerated from rustdoc JSON. Define a witness-backed field as any stored field other than `source` whose value is determined by the `ReplaySource` arm. Or state AC-9 as an exact field-set assertion against a declared list. | FR-197 AC-7, AC-8, AC-9; TC-272 RS-04 to RS-07 |
| FND-007 | low | ADR-013 O-27 says "One typed per-item result". FR-197 requires distinct result types for the two arms. The two agree only if the per-item result is a sum over the two arm types. Fix: state that shape in AD-016 Replay source, and record a QSL follow-up to align the O-27 wording. | FR-197 Replay source; AD-016 Replay source; ADR-013 O-27 |
| FND-008 | low | `reproduced-without-witness` is backticked as a label. Reproduced-with-evaluated-witness is plain prose. The amendment does not say what kind of value either one is. O-16 fixes eight categories and says no family adds one, yet ADR-013 calls `reproduced-without-witness` a "WP9 category". Fix: backtick both. State that both are parity settlements carried by the replay result under O-16 category success, with the serialized spelling from QC-8 (FR-323). | AD-016 arrow 7, Replay source; FR-197 AC-6; ADR-013 O-16, O-25 |
| FND-009 | low | TC-272 RS-09 (a serialized packet carrying both a transcript and stored values refuses) backs no acceptance criterion. Its expected "typed cause" has no name. Fix: add an FR-197 criterion for the both-arms refusal with a named cause, and trace it to TC-272, or drop RS-09. | TC-272 RS-09; FR-197; AD-016 Packet row |
| FND-010 | low | TC-272 builds the `Witness` packet from "the AD-016 seed counterexample" transcript. The AD-016 seed vector records `concrete_vals`, not a transcript, and is marked not yet vendored. The test input cannot be reproduced from the spec. Fix: name the vendored transcript source, for example the IR tc_042 fixture path, or the WP9 vendored vector. | TC-272 Test Procedure; AD-016 seed vector |
| FND-011 | low | FR-197 names the actor "the verification boundary" in its Description and "the replay boundary" in Replay source. Fix: use one name throughout. | FR-197 |
| FND-012 | low | Arrow 5 says "The FR-201 digest domain of this identity is QSL ADR-013 QC-4". QC-4 is a pending FR-201 change (TK-07), and the seed row says the domain does not exist yet. Fix: phrase arrow 5 the same way as the seed row: `OPEN — decided in TK-07` (QC-4). | AD-016 arrow 5, seed vector; ADR-013 QC-4 |

## Completeness and traceability

| Item | Where in the diff | Status |
| --- | --- | --- |
| QC-7 WP9 parity carrier | Arrow 7 Accounting: the `ReplaySource` arm | Covered |
| QC-13 transcript-only `Witness` | Witness shape row, invariant sentence | Covered; admission gap FND-003 |
| QC-14 obligation identity | Arrow 5 Identity retained; seed row regenerated as OPEN | Covered; residual FND-004, FND-012 |
| QC-15 six `Value` component types | Kernel row | Covered, exactly six, no others |
| QC-16 WP9 category map | Arrow 6 closed-set table | Covered; matches O-16 proof column on all ten rows, `tested`, parity disagreement and `failed` |
| QC-17 node ids in kernel row only | Kernel row; `DeclarationKey` row split | Covered |
| QC-20 `ReplaySource` | Arrow 6 output, arrow 7, Packet row, Replay source | Covered |
| Coordinator rule: rationale | AD-016 Replay source | Covered |
| Coordinator rule: AC and TC | FR-197 AC-6 to AC-10; TC-272; tests.md | Covered; owner and oracle gaps FND-005, FND-006 |

Trace chain: US-011 → FR-197 (implements) → AC-6 to AC-10 → TC-272 (verifies
FR-197) → `spec/complete-v1/tests.md` criterion rows, TC row and V1-BACK-006.
Every new criterion has a TC row. No new StR, US or NFR is needed.

## Consistency and atomicity

- The FR-197 SHALL statements are ubiquitous EARS, each with one obligation,
  except the fourth, which pairs "derive by match" with "store no field". AC-9
  and AC-10 test the two halves separately, so no finding is raised.
- The arrow 7 executor text (layer-6 `replay` facade, typed `QualifiedName`)
  matches ADR-013 O-26 and ADR-011's layer-6 facade.
- No "was X", compatibility, migration or fallback wording was found in the diff.
  No rename was found.

## Failure domain check

- Identity keys: arrow 5 separates obligation identity from clause node id.
  Arrow 7 does not yet (FND-004).
- Extension failures: the `Input` arm is a new variant with no trigger in
  FR-197 (FND-002).
- Evaluation purity: the transcript is the single stored fact. Admission through
  `parse` is not yet enforced (FND-003).

## Round 2 dispositions

Evaluated against the tree committed with these records (fixes in `201b4a4`, plus the TC-272 RS-13 compile-fail case). Paths are relative to `spec/`.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | AD-010 Summary, FR-349 Behavior and IT-075 Objective name the QSL complete-V1 executor (FR-197, AD-016 arrow 7); FR-355 and TC-270 keep SMT differential parity, which is a different comparison |
| FND-002 | resolved | FR-197 Description, Inputs and Replay envelope cover both arms; `Input` defined as "did not come from a backend transcript (a corpus counterexample)" in AD-016 Replay source and FR-197 |
| FND-003 | resolved | AD-016 Replay ownership, Witness shape row: admission only through `Witness::parse` including on deserialization, private `transcript`, selected trimmed block, refusal of cover playback and of zero or several assertion blocks (O-25); implementer QSL ADR-013 TK-04 |
| FND-004 | resolved | AD-016 arrow 7 Identity retained: "Obligation identity, node id" |
| FND-005 | resolved | AD-016 Replay ownership, Replay result and Backend-evidence verdict rows; FR-197 Replay source bullet 1 |
| FND-006 | resolved | TC-272 Test Procedure: `trybuild` cases with `.stderr` snapshots and compile-pass controls; closure taken from a public-API listing of the replay crates |
| FND-007 | partially resolved | AD-016 Replay result row: one per-item result whose member `arm` is the sum of the two arm types. Deferred, owner QSL #244: the ADR-013 O-27, QC-7 and QC-14 wording follow-up, named in the PR body. IR `witness_backed` removal is IR TK-04 (IR #144); AD-016 arrow 7 Authority states only that TK-04 (IR #144) implements this contract |
| FND-008 | resolved | Both labels code-fonted; AD-016 Replay source states they are parity settlements under category success (O-16), not new categories |
| FND-009 | resolved | FR-197-AC-14; TC-272 RS-09 (both arms), RS-10 (neither) |
| FND-010 | resolved | TC-272 Fixture: IR `tests/kani_replay.rs` tc_042 witness transcript at `954c2f2`, decoded values (8, 992) |
| FND-011 | resolved | FR-197 uses "the verification boundary" for serialize and replay and defines "the replay boundary" as its CG adapter plus QSL facade part |
| FND-012 | resolved | AD-016 arrow 5 Identity retained: `OPEN — decided in QSL ADR-013 TK-07` for the FR-201 digest domain (QC-4); AD-016 seed vector: `obligationIdentitySha256` and `harnessSha256` both `OPEN — decided in QSL ADR-013 TK-05`, after TK-07 (QC-14) |

Rows for FND-007, FND-008 were updated after the PR #140 review at `9e52406`.
