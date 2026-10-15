---
id: SR-619
title: "Risk and complexity review of the AD-016 amendment (TK-10)"
type: SpecReview
analysis: risk-complexity
scope: "quire-specification task/ad016-amendment diff 55d2fcc..5bd933f; AD-016; FR-197; TC-272; spec/complete-v1/tests.md; against QSL ADR-013 and ADR-011 at 31b6054, IR 954c2f2, CG origin/main a4b2a73"
review_set: all
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/AD-016, type: references }
  - { target: ix://agent-ix/quire-specification/FR-197, type: references }
---

## Summary

The diff amends AD-016 for QSL ADR-013 QC-7, QC-13 to QC-17 and QC-20. It adds
the FR-197 Replay source rules (AC-6 to AC-10) and TC-272. Every TK-10 item is
present, and the text matches ADR-013 cell by cell.

The riskiest part is the structural rule for replay evidence (FR-197 AC-7 to
AC-9). It is a type-system guarantee, but three things are missing. No owner
is named for the verdict and result types. The rule stops at the process
boundary, while results also cross the `native-run-result/2` wire. And the FR's
own Replay envelope (unchanged context) still stores input values next to every
counterexample. The second risk is the obligation-identity change. The seed
vector keeps a `harnessSha256` that embeds the old identity digest, and the new
preimage wording can be read as three members only. Two smaller risks remain:
the pinned IR sha lets a `Witness` be deserialized without admission, and
minimized revisions have no rule for which arm they get.

## Verdict

**REVISE**: FND-001 to FND-007 (medium) must be fixed in this PR. FND-008 to
FND-010 (low) can be fixed here or recorded.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The seed vector marks `obligationIdentitySha256` `OPEN — decided in WP9` but keeps `harnessSha256` = `0d79…c538` as a fixed value. The CG harness embeds the identity digest as a comment line (`// Obligation identity sha256: {identity_sha256}`, CG `src/kani_obligations.rs:1393` at `a4b2a73`). So regenerating the identity also changes the harness bytes, and the pinned `harnessSha256` will fail once WP9 vendors the vector. Fix: mark `harnessSha256` `OPEN — decided in WP9` too, regenerated with the obligation identity. Keep `oracleDigest`, `concrete_vals` and the tool pins, which do not depend on the identity. | AD-016 seed counterexample vector; QC-14 |
| FND-002 | medium | Arrow 5 says obligation identity is "the `KaniObligationIdentity` digest over (clause node id, obligation kind, `arguments`)". This can be read as a preimage of those three members only. The CG identity has about twenty members (schema, adapter profile, bound-package, declaration and expression digests, oracles, symbols, tool pins, solver, IR revision). ADR-013 O-09 says "the full identity digest". If an implementer drops the other members, two runs of one clause under different solver or tool pins get the same identity. Fix: reword to "Obligation identity = the digest over every `KaniObligationIdentity` member except `source_span`. Clause node id, obligation kind and `arguments` are the members that tell apart the obligations of one clause, so the clause node id alone never identifies an obligation." | AD-016 arrow 5; ADR-013 O-09, QC-14; CG `src/kani_obligations.rs:363` |
| FND-003 | medium | FR-197 AC-7 and AC-8 hold only inside the process. A replay result also crosses the wire as `native-run-result/2` (FR-352, ADR-013 O-27), and a `Deserialize` impl is a public constructor. TC-272 has no vector that reads a record back. RS-06 calls constructors only with an `Input`-arm result or bare values, and RS-09 covers packets, not results. ADR-013 O-27 also says "one typed per-item result", while FR-197 requires two distinct types. The record must therefore carry the arm (the QC-7 carrier), and its reader must pick the type from that arm. Fix: in FR-197 Replay source, add "The replay-result reader SHALL decode a record of an `Input` replay only into the `Input`-arm result type." Add AC-11 for it. Add TC-272 RS-10: a record from an `Input` replay, and the same record with its arm member altered to `Witness`, each round-trips to its own arm type or refuses; neither yields a backend-evidence verdict. | FR-197 AC-7, AC-8; TC-272; ADR-013 O-27, QC-7; FR-352 |
| FND-004 | medium | Neither AD-016 nor FR-197 names an owner for the `Witness`-arm result type, the `Input`-arm result type or the backend-evidence verdict type. The Replay ownership table has rows for packet, witness, comparator and executor only. AC-7 to AC-9 are compile-fail checks across IR, CG and QSL (#231). Without one owning crate, "every position that accepts a backend-evidence verdict" is not a list anyone can enumerate, and RS-04 to RS-06 cannot be written. Fix: add a Replay ownership row: per-arm result types owned by QSL (#231, ADR-013 O-27); backend-evidence verdict owned by the CG parity comparator. State that the verdict is one type with private fields whose only public constructor takes the `Witness`-arm result. RS-04 to RS-06 then target that one constructor and the positions typed by it. | AD-016 Replay ownership; FR-197 AC-7 to AC-9; TC-272 RS-04 to RS-06 |
| FND-005 | medium | FR-197 Replay envelope (unchanged context) says the envelope holds "typed input values" and the "expected backend disposition" for every counterexample. The new Replay source section and AD-016 say a `Witness` packet stores no input beside the transcript; the input is `decode` of the transcript (ADR-013 O-25, QC-6). The two sections of FR-197 now disagree, and FR-197-AC-1/AC-4 (TC-219) will be built on the envelope text. Fix: in Replay envelope, replace "typed input values" with "the `ReplaySource`: for `Witness`, the admitted transcript, whose decode is the typed input and is not stored; for `Input`, the typed input values". | FR-197 Replay envelope, Replay source; AD-016 Replay source; ADR-013 O-25, QC-6 |
| FND-006 | medium | FR-197 minimization (unchanged, AC-5) makes linked revisions that must keep "both replay verdicts". The diff does not say which arm a minimized revision has. A shrunk candidate with no new backend run has no transcript of its own, so it is an `Input` packet and can never be backend evidence. The AC-5 wording "both replay verdicts" implies a backend verdict it cannot have. Fix: add to Replay source: "A minimized revision is a `Witness` packet only when the backend re-ran it and returned its own transcript. Otherwise it is an `Input` packet." Change AC-5 to require that re-run, and add a TC-219 vector for a minimized candidate with no re-run that settles `reproduced-without-witness`. | FR-197 Behavior, Replay envelope, AC-5; TC-219 |
| FND-007 | medium | The Witness shape row pins IR `954c2f2` and says a transcript that is not a single assertion block makes each accessor refuse. At that sha, `Witness` derives `Deserialize` with a `pub transcript` (IR `src/kani/witness.rs:100-107`), which skips `parse`. ADR-013 O-25 requires admission through `parse`, including on deserialization, and defines identity as lexical over the admitted transcript. The row leaves admission out. A deserialized untrimmed or two-block transcript is then a stored `Witness` that `parse` would never produce, and two witnesses of one run compare unequal. Fix: add to the Witness shape row: "A `Witness` is admitted only through `parse`, including on deserialization; `transcript` is private; the stored transcript is the selected, trimmed assertion block (ADR-013 O-25)." Name IR TK-04 as the change that brings `954c2f2` to this rule. | AD-016 Replay ownership (Witness shape); ADR-013 O-25; IR `954c2f2` |
| FND-008 | low | `Input(values)` has no named payload type. The AD does not say how values are keyed or how wide they are. Witness values are 8-byte `i64` widened to complete-V1 values. A corpus counterexample can hold values outside `i64`, and ADR-013 C-11 keys arguments by parameter `WireNodeId`. Fix: write `Input(assignments)`, the canonical assignments keyed by parameter `WireNodeId`, as complete-V1 values, validated against the declared domain before replay (ADR-013 C-11). | AD-016 Replay source, Replay ownership (Packet); ADR-013 C-11 |
| FND-009 | low | Arrow 7 "Identity retained" still reads "Obligation id, node id", while arrows 5 and 6 now say "Obligation identity". Fix: reword to "Obligation identity, node id". | AD-016 arrow 7 |
| FND-010 | low | The kernel row gives the five new digest newtypes (`UniverseId`, `ObjectId`, `UnitId`, `VariantId`, `MemberId`) one public constructor from a digest, but names no FR-201 digest domain and no minting check. The ADR-011 T-12 API-surface check covers only the `NodeKey` and `EffectiveId` constructors. So any crate can mint one of these from any 32 bytes. This is faithful to QC-15, so the fix is to record it as open, not to decide it: add `OPEN — decided in QSL #213 S-2`: the FR-201 digest domain of each of the five newtypes and whether T-12 limits their minting. | AD-016 kernel row; ADR-013 QC-15, T-6; ADR-011 T-12 |

## Risk register

Tech risk and volatility use the skill's definitions. The amended AD-016 cells
are scored as items too, because each one is a requirement that later work
builds against.

| Req | Tech Risk | Volatility | Drivers | Mitigation |
| --- | --- | --- | --- | --- |
| FR-197 AC-6 to AC-10 (Replay source) | High | Medium | Evidence integrity: a false backend-evidence verdict is an assurance failure. The guarantee is compile-time across three repositories (IR packet, QSL #231 results, CG comparator) and has to survive the FR-352 wire. The owning types do not exist yet (IR `954c2f2` still has `witness_backed: bool` and `input` beside `witness`). | FND-003, FND-004. Build TC-272 on the skeleton spine first (ADR-011 T-2), with one sealed verdict type; `trybuild` compile-fail cases in the owning crate. |
| FR-197 AC-1 to AC-5 (unchanged, now under ReplaySource) | Medium | Medium | Envelope and minimization text predates the two arms. | FND-005, FND-006. Amend in this PR so TC-219 is not built against the old envelope. |
| FR-197 Description (executor = QSL complete-V1, arrow 7) | Medium | Low | IR `replay_with_native_runtime` still calls `runtime::execute` at `954c2f2`; its retirement is Q209-5 / WP9. | No spec change. WP9 retires the IR path in the same change that lands the CG adapter. |
| AD-016 arrow 5 identity (QC-14) | Medium | Medium | Identity preimage change ripples into harness bytes, seed vector, FR-201 domain (QC-4, open) and ADR-011 E7 (pending note). | FND-001, FND-002. Regenerate identity and harness hash together; one CG conformance test (TK-05). |
| AD-016 arrow 6 WP9 category map (QC-16) | Low | Medium | Rows 3 to 6 (refusal, unsupported) wait on the QC-9 FR-331 result value. | Land the IR C-09 test for rows 1, 2 and 7 to 10 now and add rows 3 to 6 when QC-9 lands. No `_` arm in the meantime. |
| AD-016 arrow 7 parity carrier (QC-7) | Medium | Medium | The carrier is the arm in memory; its wire member in FR-352 / FR-331 is QC-6 and QC-8 work outside this PR. | FND-003. Keep the FR-352 member tied to QC-7 in the #231 round-trip tests. |
| AD-016 Witness shape (QC-13) | Medium | Low | Admission gap at the pinned sha. | FND-007. IR TK-04 adds `try_from` admission and tests for cover, untrimmed and two-block transcripts. |
| AD-016 kernel row (QC-15, QC-17) | Medium | Low | Six new public digest newtypes in a `no_std` kernel; domains not named. | FND-010. Settle domains in #213 S-2 before any wire carries them. |
| AD-016 Packet / Replay source (QC-20) | Medium | Low | `Input` payload type unnamed. | FND-008. |

## Top hazards

1. FR-197 AC-7 to AC-9: the evidence rule has no owner and stops at the wire
   (FND-003, FND-004). Review live before planning CG #50 and #231.
2. Obligation identity: seed `harnessSha256` is stale by construction, and the
   preimage wording can drop members (FND-001, FND-002).
3. FR-197 envelope and minimization still describe one stored input per
   counterexample (FND-005, FND-006).
4. Witness admission at the pinned IR sha (FND-007).

## Failure-domain gaps

The FND-003 wire bypass and the FND-006 minimization gap are also identity and
purity gaps. See the sibling failure-domain review in this review set
(`spec/reviews/ad016-amendment/`). Open gaps: FR-197.

## Evidence

- Diff read in full: `git -C /home/peter/dev/worktrees/qspec-ad016-amend diff 55d2fcc..HEAD` (HEAD `5bd933f`).
- ADR-013 at QSL `31b6054`: O-09, O-16, O-25, O-26, O-27, C-09, C-11, QC-6 to QC-9, QC-13 to QC-17, QC-20, OQ-3, OQ-5, TK-04, TK-10.
- ADR-011 at QSL `31b6054`: E8, E9, T-12, scenario 9.
- IR `954c2f2`: `src/kani/replay.rs` (packet with `input` and `witness: Option<Witness>`, `witness_backed: bool`, `replay_with_native_runtime` calling `runtime::execute`); `src/kani/witness.rs:100-107` (derived `Deserialize`, `pub transcript`).
- CG origin/main `a4b2a73`: `src/kani_obligations.rs:363` (`KaniObligationIdentity` members), `:1393` (identity digest embedded in harness text).

## Round 2 dispositions

Evaluated against the tree committed with these records (fixes in `201b4a4`, plus the TC-272 RS-13 compile-fail case). Paths are relative to `spec/`.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | AD-016 seed vector: `obligationIdentitySha256` and `harnessSha256` both `OPEN — decided in QSL ADR-013 TK-05`, after TK-07 (QC-14) |
| FND-002 | resolved | AD-016 arrow 5 Identity retained: the digest over every `KaniObligationIdentity` member except `source_span`; clause node id, kind and `arguments` tell apart the obligations of one clause |
| FND-003 | resolved | FR-197 Replay source reader bullet; FR-197-AC-18; TC-272 RS-16; AD-016 Replay source |
| FND-004 | resolved | AD-016 Replay ownership, Replay result and Backend-evidence verdict rows; FR-197 Replay source bullet 1 |
| FND-005 | resolved | FR-197 Replay envelope: a `Witness`-arm envelope's typed input values are the decode of its transcript and it stores no input beside it; backend run identity applies to the `Witness` arm |
| FND-006 | resolved | FR-197 Replay envelope: a minimized `Witness`-arm envelope is accepted only after native and backend replay preserve the failure, carries the `Witness` arm of that backend re-run and never inherits its parent's transcript; minimizing an `Input`-arm counterexample preserves the native verdict only and yields `Input`-arm revisions. FR-197-AC-5 (`Witness` arm) and FR-197-AC-19 (`Input` arm); TC-219 MR-01 (candidate with no backend re-run) and MR-02 |
| FND-007 | resolved | AD-016 Replay ownership, Witness shape row: admission only through `Witness::parse` including on deserialization, private `transcript`, selected trimmed block, refusal of cover playback and of zero or several assertion blocks (O-25); implementer QSL ADR-013 TK-04 |
| FND-008 | resolved | AD-016 Replay source `Input` row and FR-197 Replay source: canonical assignments keyed by parameter `WireNodeId` (QSL ADR-013 C-11) |
| FND-009 | resolved | AD-016 arrow 7 Identity retained: "Obligation identity, node id" |
| FND-010 | partially resolved | AD-016 kernel row: `EffectiveId` domain `quire.model.effective-declaration/v1` (QSL ADR-013 QC-2), constructor called only by QSL `model` (O-05; ADR-011 T-12). Deferred, owner QSL #213: the FR-201 digest domain of `UniverseId`, `ObjectId`, `UnitId`, `VariantId` and `MemberId` and whether T-12 limits their minting, marked `OPEN — decided in QSL #213` |

Rows for FND-010, FND-006 were updated after the PR #140 review at `9e52406`.
