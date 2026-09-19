---
id: SR-614
title: "Base review of the AD-016 amendment (TK-10)"
type: SpecReview
analysis: base
scope: "quire-specification diff 55d2fcc..5bd933f; AD-016; FR-197; TC-272; spec/complete-v1/tests.md; source of truth QSL ADR-013 at 31b6054 (QC-7, QC-13 to QC-17, QC-20, TK-10, OQ-3, O-16, O-25, O-27)"
review_set: all
evaluated_revision: "5bd933f"
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/AD-016, type: references }
  - { target: ix://agent-ix/quire-specification/FR-197, type: references }
---

## Summary

The diff amends AD-016 in place for the TK-10 set: QC-7, QC-13 to QC-17 and
QC-20. It also adds the coordinator's structural rule for `ReplaySource`:
FR-197 AC-6 to AC-10 and the new TC-272. Every QC in the set is carried, and
each one matches ADR-013. No compatibility, migration or rename wording
appears. Every new AC has a TC-272 row in the matrix.

Three defects block. The per-arm result types conflict with ADR-013 O-27's
"one typed per-item result" and need a stated relationship. The Witness shape
row misstates how IR `954c2f2` accessors behave. The rest of FR-197 still
assumes every counterexample came from a backend, which the new `Input` arm
contradicts. The low findings are test-input and wording gaps.

## Verdict

**REVISE**: FND-001, FND-002 and FND-003 (medium) must be fixed. The low
findings FND-004 to FND-010 can be fixed in this PR or recorded.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | ADR-013 O-27 says a replay has "one typed per-item result". The amendment says the `Witness` and `Input` arms yield "distinct result types" (AD-016 arrow 7 Accounting, Replay source; FR-197 statement 1, AC-7). The AD never says how the two arm types relate to O-27's one per-item result, so #231 cannot tell whether it builds one type or two. Fix: in AD-016 Replay source, state that the one per-item result (O-27) carries its settlement as a sum whose two arms are the distinct types, for example `ReplaySettlement { Witness(WitnessReplay), Input(InputReplay) }`. The backend-evidence verdict takes only `WitnessReplay`. Mirror one sentence in FR-197 Replay source. | AD-016 arrow 7; AD-016 Replay source; FR-197-AC-7; ADR-013 O-27 |
| FND-002 | medium | The Witness shape row says "a transcript that is not a single assertion block makes each of them refuse". At IR `954c2f2`, `Witness::derived()` re-runs `parse`'s selection. A stored multi-block transcript that holds one assertion block and some cover blocks re-selects that block and returns `Ok`. It does not refuse. It refuses only when there are zero or several assertion blocks (`src/kani/witness.rs`, `derive`). Fix: reword to "each re-selects the single assertion block from `transcript` and refuses when the transcript holds none or more than one". Carrying ADR-013 O-25's admission rule would give the stronger claim, but TK-10 does not include it. | AD-016 Replay ownership (Witness shape); QC-13; IR `954c2f2` `src/kani/witness.rs` |
| FND-003 | medium | FR-197's unchanged body still assumes a backend run for every counterexample. The Description trigger is "When a proof backend reports a counterexample". Inputs is "Backend counterexample". The Replay envelope holds "backend run identity" and "expected backend disposition". Minimization is "accepted only after native and backend replay preserve the failure". An `Input`-arm counterexample ran no backend, so under the new Replay source section these statements are either false or undefined. Fix: widen the Description and Inputs to "a counterexample (backend-reported or stored input)". Mark the backend run identity and the expected backend disposition as present only for the `Witness` arm. State that for an `Input` counterexample, minimization needs only native replay to preserve the failure. | FR-197 Description, Inputs, Replay envelope, Behavior; FR-197-AC-5 |
| FND-004 | low | TC-272 RS-09 (a packet that carries both a transcript and stored values is refused) has no AC. FR-197 states the one-arm rule but not the refusal, and it names no refusal cause. RS-09 expects "a typed cause" without naming it. Fix: add FR-197-AC-11, "Deserializing a packet that carries both arms, or neither, refuses with `<named cause>` before replay", map it to TC-272 in spec/complete-v1/tests.md, and name the cause in RS-09. Add a vector for a packet with neither arm. | FR-197 Replay source; TC-272 RS-09; spec/complete-v1/tests.md |
| FND-005 | low | TC-272 builds the `Witness` packet from "the AD-016 seed counterexample" transcript. The AD-016 seed vector has no transcript. It has only digests and `concrete_vals` (decoded 8, 992). Fix: name the transcript fixture, for example the IR PR #139 tc_042 transcript at `954c2f2`, or embed the assertion block in TC-272. | TC-272 Test Procedure; AD-016 seed counterexample vector |
| FND-006 | low | AC-7 ("every parameter, field and return position") and AC-8 ("every public constructor") quantify over sets that TC-272 never enumerates. A compile-fail suite covers only the positions its author lists. Fix: in TC-272, name the source of the set, for example every public item in the replay-result module that names the backend-evidence verdict type, listed in the test and checked against the crate's public API so a new position fails the test until a vector covers it. | FR-197-AC-7; FR-197-AC-8; TC-272 RS-04 to RS-06 |
| FND-007 | low | Arrow 7 "Identity retained" still reads "Obligation id, node id". Arrows 5 and 6 now say "Obligation identity" (QC-14), and QC-14 says the clause node id alone never identifies an obligation. Fix: change it to "Obligation identity, clause node id". | AD-016 arrow 7; QC-14 |
| FND-008 | low | Wording around QC-4 and QC-14. (a) Arrow 5 reads "The FR-201 digest domain of this identity is QSL ADR-013 QC-4", which names a change request as if it were the domain. (b) The seed `obligationIdentitySha256` is marked `OPEN — decided in WP9`, but regenerating a vector is not a decision. ADR-013 routes QC-14 to CG obligation conformance (TK-05), not WP9. Fix: (a) "Its FR-201 digest domain is added by QSL ADR-013 QC-4." (b) "Pending: regenerated by CG obligation conformance (ADR-013 TK-05) once the QC-4 domain exists." | AD-016 arrow 5; AD-016 seed counterexample vector; QC-4; QC-14 |
| FND-009 | low | Some new text states what things are not: AD-016 "The rule is structural, not documentary" and FR-197 "Type structure enforces this; a documentation comment or a Boolean field does not". Fix: restate positively, for example "Type structure enforces this rule (AC-7 to AC-9)." | AD-016 Replay source; FR-197 Replay source |
| FND-010 | low | Once this PR lands, ADR-013 O-27 ("The parity carrier field is `OPEN — decided in WP9` in AD-016 (QC-7)") is stale, because AD-016 settles the carrier as the `ReplaySource` arm with no field. IR `954c2f2` still has `ReplayAgreement.witness_backed` and `NativeReplayAgreement.witness_backed`, which FR-197-AC-9 forbids. Fix: outside this repo. Record a QSL follow-up to close the QC-7 cell in O-27, and confirm that IR TK-04 removes `witness_backed`. No change to this diff. | ADR-013 O-27; QC-7; FR-197-AC-9; IR `954c2f2` `src/kani/replay.rs` |

## TK-10 carry check

Each QC is compared with ADR-013 at `31b6054` §8.

| QC | ADR-013 text | Amendment | Result |
| --- | --- | --- | --- |
| QC-7 | WP9 parity carrier field (confirmed 2026-09-19 in TK-10) | Arrow 7 Accounting: the carrier is the packet's `ReplaySource` arm, with no witness-backed flag | Carried. It follows O-25's replay-source rule. O-27 wording goes stale (FND-010). |
| QC-13 | `Witness` stores `transcript` only; four facts derived | Witness shape row: `Witness { transcript: String }`, with `harness_symbol()`, `check()`, `check_text()` and `concrete_values()` derived; `decode` reads `transcript` | Carried. The refusal sentence is inaccurate (FND-002). |
| QC-14 | Identity = digest over (clause node id, obligation kind, `arguments`); `source_span` outside the preimage; seed vector regenerated | Arrow 5 and arrow 6 identity rows; seed digest marked pending | Carried. The seed is deferred until the QC-4 domain exists, which is sound. The wording and routing need fixes (FND-007, FND-008). |
| QC-15 | Kernel row gains exactly `EffectiveId`, `UniverseId`, `ObjectId`, `UnitId`, `VariantId`, `MemberId`: opaque 32-byte digest newtypes, one public constructor from a QSL-computed digest, no dependency on `check` or `model` | Kernel row lists exactly these six, with the same roles and constraints | Carried exactly. |
| QC-16 | WP9 category map of the ten `KaniOutcomeKind`s = O-16 proof column | Closed-set table gains Category and FR-331 terminal-record columns; prose covers `tested`, parity disagreement and `failed` | Carried exactly. All ten rows match O-16, including `Unavailable` → unsupported. The catalog-code part stays `OPEN` in WP9, as O-16 leaves it. |
| QC-17 | Node ids listed in the kernel row only; no rename | Kernel row: "this row is their only listing". The Semantic identities row is narrowed to `DeclarationKey`, and the `CheckedNodeId` round trip moves to the kernel row | Carried. No type is renamed. TC-195 (model normalization) stays with `DeclarationKey`, which is correct. |
| QC-20 | Arrow 6 output and Packet row: `source: ReplaySource { Witness(Witness), Input(values) }`; `Input` settles `reproduced-without-witness`, never backend evidence | Arrow 6 output, Packet row, new Replay source section | Carried exactly. |

Nothing outside the set is invented. Each addition beyond the QC list comes
from an authorized source:

- The arrow 7 layer-6 `replay` facade and the typed `QualifiedName` selection
  come from ADR-013's Context, O-26 and the OQ-5 ruling, and from the brief.
  Arrow 7 ownership is unchanged.
- The Replay source rationale, FR-197 AC-6 to AC-10 and TC-272 are the
  coordinator addition. It is ruled in, but it conflicts with O-27 (FND-001).
- The label "reproduced-with-evaluated-witness" was already in AD-016's
  Accounting row. It was not invented here.

## Checklist results

| Check | Result |
| --- | --- |
| ID formats | FR-197-AC-6 to AC-10 are sequential and not duplicated. TC-272 is the next free TC id. The RS-01 to RS-09 vector ids are sequential. |
| Links | FR-197 → `../../assurance/AD-016-...md` resolves. AD-016 → `../functional/ir-proof/FR-197-...md` resolves. The new frontmatter edges AD-016 → FR-197 and ADR-013, FR-197 → AD-016, and TC-272 → FR-197 (`verifies`) are well formed. |
| FR quality | The Replay source statements use EARS: three ubiquitous and one event-driven ("When a replay of the `Input` arm agrees"). The one-arm refusal has no AC (FND-004). The FR-197 inputs, outputs and envelope still assume a backend (FND-003). |
| Coverage rule | AC-6 to AC-10 each map to TC-272 in spec/complete-v1/tests.md. V1-BACK-006 lists TC-272. The TC matrix row is Property/P0. |
| Option permutation | Both `ReplaySource` arms are exercised (RS-01, RS-02, RS-08). |
| Constraint boundary | The one-arm rule is tested only for "both arms". "Neither arm" is missing (FND-004). |
| Error path | RS-09 has no AC and no named cause (FND-004). The TC-219 decode and parity-failure paths are unchanged. |
| State transition | Not applicable. |
| Edge case | The stub executor that always agrees (RS-03) shows that `Input` agreement cannot become evidence. The completeness of the compile-fail positions is not enumerated (FND-006). |
| TC fields | TC-272 has Description, Procedure, Vectors and Expected Results, like its siblings. Type and priority are in the matrix. The Witness fixture is undefined (FND-005). |
| Settled rules | The amendment is in place, states only the current design, has no compatibility or migration wording, renames nothing, uses EARS, and gives every new AC a TC row. The FR-197 Description edit aligns with arrow 7. |
| Terminology | "Obligation identity" vs "Obligation id" (FND-007). Everything else is consistent across AD-016, FR-197 and TC-272. |

## Evidence

- Diff read in full: `git -C /home/peter/dev/worktrees/qspec-ad016-amend diff 55d2fcc..HEAD` (HEAD `5bd933f`).
- ADR-013 read at `31b6054`: the §8 QC table, TK-10, OQ-3, O-16, O-25, O-26 and O-27.
- IR read at `954c2f2`: `src/kani/witness.rs` (`transcript` is the only field;
  `derived()` re-selects the assertion block) and `src/kani/replay.rs`
  (`CounterexamplePacket.witness: Option<Witness>`;
  `ReplayAgreement.witness_backed: bool`).
- Unchanged context cross-checked: AD-016 arrows 1 and 2 identity rows, the
  arrow 7 Identity retained row, the seed vector table, FR-197 Description,
  Inputs, Behavior and Replay envelope, TC-219 and TC-195.

## Round 2 dispositions

Evaluated against the tree committed with these records (fixes in `201b4a4`, plus the TC-272 RS-13 compile-fail case). Paths are relative to `spec/`.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | AD-016 Replay ownership, Replay result and Backend-evidence verdict rows; FR-197 Replay source bullet 1 |
| FND-002 | resolved | AD-016 Replay ownership, Witness shape row: admission only through `Witness::parse` including on deserialization, private `transcript`, selected trimmed block, refusal of cover playback and of zero or several assertion blocks (O-25); implementer QSL ADR-013 TK-04 |
| FND-003 | resolved | FR-197 Description, Inputs and Replay envelope cover both arms; `Input` defined as "did not come from a backend transcript (a corpus counterexample)" in AD-016 Replay source and FR-197 |
| FND-004 | resolved | FR-197-AC-14; TC-272 RS-09 (both arms), RS-10 (neither) |
| FND-005 | resolved | TC-272 Fixture: IR `tests/kani_replay.rs` tc_042 witness transcript at `954c2f2`, decoded values (8, 992) |
| FND-006 | resolved | TC-272 Test Procedure: `trybuild` cases with `.stderr` snapshots and compile-pass controls; closure taken from a public-API listing of the replay crates |
| FND-007 | resolved | AD-016 arrow 7 Identity retained: "Obligation identity, node id" |
| FND-008 | resolved | AD-016 arrow 5 Identity retained: `OPEN — decided in QSL ADR-013 TK-07` for the FR-201 digest domain (QC-4); AD-016 seed vector: `obligationIdentitySha256` and `harnessSha256` both `OPEN — decided in QSL ADR-013 TK-05`, after TK-07 (QC-14) |
| FND-009 | resolved | AD-016 Replay source: "Type structure carries the rule"; FR-197 Replay source closing paragraph stated positively |
| FND-010 | deferred | Deferred, owner QSL (ADR-013 maintainers): O-27's "parity carrier OPEN" cell and its "one typed per-item result" wording need a QSL follow-up; named in the PR body. IR `witness_backed` removal is QSL ADR-013 TK-04, recorded as OPEN in AD-016 arrow 7 Authority |
