---
id: SR-615
title: "Failure-domain review of the AD-016 amendment (TK-10)"
type: SpecReview
analysis: failure-domain
scope: "quire-specification task/ad016-amendment diff 55d2fcc..5bd933f; AD-016; FR-197; TC-272; spec/complete-v1/tests.md; checked against QSL ADR-013 at 31b6054 (O-16, O-25, O-26, O-27, C-11, QC-7, QC-13 to QC-17, QC-20) and IR src/kani/witness.rs and src/kani/replay.rs at 954c2f2"
review_set: all
evaluated_revision: "5bd933f"
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/AD-016, type: references }
  - { target: ix://agent-ix/quire-specification/FR-197, type: references }
---

## Summary

This review applies the failure-domain method to the diff: extension points
and trust boundaries, entity identity, evaluation purity and topological
robustness. The amendment covers every TK-10 item: QC-7, QC-13 to QC-17 and
QC-20. The new `ReplaySource` rule is sound for the happy path. It has gaps on
the failure paths.

The main gap is in the structural guarantee. FR-197-AC-8 and TC-272 check
only calls to public constructors. A backend-evidence verdict built by
`Deserialize` or by a struct literal with public fields gets past the rule,
and every AC still passes. IR has already had this exact bypass for
`Witness`: at `954c2f2`, `transcript` is public and the derived
`Deserialize` skips `parse`. Other gaps: a disagreeing `Witness` replay can
still build a backend-evidence verdict. An `Input` replay's disagreement and
its reference answer are not defined. `Input(values)` has no key and no
refusals. The `replay` facade's new selection and recompile steps have no
failure behavior. Minimized revisions have no rule for which arm they take.

## Verdict

**REVISE**. FND-001 (high) blocks. FND-002 to FND-009 (medium) also block and
should be fixed in this PR. FND-010 to FND-013 (low) can be fixed in this PR
or recorded.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Trust boundary: FR-197-AC-8 says "every public constructor of a backend-evidence verdict accepts only a `Witness`-arm result". TC-272 RS-06 tests this with compile-fail calls to constructors. A verdict type that derives `Deserialize`, or that has public fields, can still be built from bytes or from a struct literal with no `Witness`-arm result. All of AC-6 to AC-10 would still pass. This is the same hole IR PR #139 has for `Witness` (derived `Deserialize` bypasses `parse`, see `witness.rs` module doc at `954c2f2`). A persisted verdict read back from `native-run-result/2` is exactly this path. Fix: extend FR-197-AC-8 to "every way to obtain a backend-evidence verdict, including deserialization and struct literals, requires a `Witness`-arm result". Require the verdict's fields to be private. Require its `Deserialize`, if any, to re-establish the value from a `Witness`-arm result and to refuse otherwise. Add TC-272 vectors: RS-10 deserializes a backend-evidence verdict record with an `Input` source, or with no source, and expects a refusal; RS-11 is a compile-fail struct literal outside the owning module. | FR-197-AC-8; TC-272 RS-06; AD-016 Replay source |
| FND-002 | medium | Failure path: AD-016 arrow 7 says a backend/native disagreement is `inconclusive` with a typed cause. FR-197 lets a backend-evidence verdict be built from any `Witness`-arm result, including a disagreeing one, because it does not say "agreeing". So an `inconclusive` `Witness` replay can still become backend evidence. Fix: in FR-197 Replay source and AC-8, write "only from an agreeing `Witness`-arm result". Add TC-272 RS-12: a `Witness` packet replayed with an executor that does not reproduce the counterexample yields `inconclusive` with the parity cause, and no backend-evidence verdict exists. | FR-197 Replay source; FR-197-AC-8; AD-016 arrow 7; ADR-013 O-27 |
| FND-003 | medium | Undefined behavior: FR-197 and AD-016 define only what happens when an `Input` replay agrees. Two things are missing. First, the reference answer: AD-016 says an `Input` replay "reproduces an answer supplied with its input", but `Input(values)` holds only values, and no field names the expected verdict. Second, the outcome when the replay disagrees. Fix: state which packet member holds the expected verdict for an `Input` packet (the FR-197 envelope's expected disposition). Add an FR-197 SHALL and an AC: an `Input` replay that disagrees settles `inconclusive` with a typed cause in the `Input`-arm result type. Add a TC-272 vector for it. | FR-197 Replay source; AD-016 Replay source; TC-272 |
| FND-004 | medium | Entity identity: `Input(values)` gives no key for the values. ADR-013 C-11 says `Input` carries canonical assignments keyed by parameter `WireNodeId`, joined by node id, and that a mismatch refuses. AD-016 arrow 7 lists refusals for the `Witness` arm only (`parse`, `decode`). TC-272 builds the `Input` packet from positional values "(8, 992)". Fix: in AD-016 Replay source and FR-197, state that `Input` values are canonical assignments keyed by parameter node id and joined by node id, never by position. Add to the arrow-7 refusal cell: an `Input` value with no parameter, a parameter with no value, or a value outside the declared domain refuses with a typed cause before execution. Key the TC-272 `Input` packet by node id, and add one missing-binding vector. | AD-016 arrow 7; AD-016 Replay source; FR-197; TC-272; ADR-013 C-11 |
| FND-005 | medium | Extension point: the diff adds two steps to arrow 7. The `replay` facade selects by typed `QualifiedName`, and it recompiles the package from digest-addressed source. The arrow-7 refusal cell names no failure for either step: a `QualifiedName` that resolves to no function, a recompiled `package_id` that differs, a source digest that differs, or bytes missing from the provision. ADR-013 O-26 refuses each of these with a structured outcome and no partial substitute. Fix: add these four refusals to the AD-016 arrow-7 "Refusal / unsupported / incomplete / failure" cell. Each has category `refusal` with a typed cause, and each yields no verdict and no parity result. | AD-016 arrow 7; ADR-013 O-26 |
| FND-006 | medium | Trust boundary: the AD-016 Witness shape row points to IR at `954c2f2` and says the accessors refuse a transcript that is not a single assertion block. At that sha, `transcript` is `pub` and `Deserialize` is derived, so an unadmitted transcript reaches replay. The failure then shows up late, at an accessor call, with the placeholder provenance `"witness"`/`"transcript"`. ADR-013 O-25 requires admission through `parse`, including on deserialization, and a private `transcript`. O-25 also defines witness identity as lexical over the admitted transcript, and that identity is undefined for an unadmitted one. Fix: state the admission rule in the Witness shape row. A `Witness` exists only through `parse`, including on deserialization, and `transcript` is private. Refusals carry the packet's obligation identity. Name the IR ticket (TK-04) that brings the recorded sha into line. | AD-016 Replay ownership; ADR-013 O-25; IR `witness.rs` at `954c2f2` |
| FND-007 | medium | Identity confusion: the rule is "the arm is the fact, stored once". IR's result types today embed the whole packet (`ReplayAgreement.packet`, `NativeReplayAgreement.packet`), which includes the `ReplaySource`. If the per-arm result types keep embedding the packet, the arm is recorded twice: once by the result type and once by the embedded source. A `Witness`-arm result could then hold an `Input` packet. FR-197-AC-9 forbids only a witness-backed field. Fix: in FR-197, state that each arm's result type holds only that arm's payload (the `Witness`, or the keyed values) together with the shared packet members, and never a `ReplaySource`. Or add an AC that a `Witness`-arm result holding an `Input` source cannot be built. Add a TC-272 vector. | FR-197-AC-9; AD-016 Replay source; IR `replay.rs` at `954c2f2` |
| FND-008 | medium | Undefined behavior: existing FR-197-AC-5 says minimization creates linked revisions whose values change. The new rule says "one with a backend transcript is `Witness`, and one without is `Input`". Nothing says which arm a minimized revision takes. If the parent's transcript is copied to a child whose values differ, the `Witness` arm replays the parent's values, because its input is derived from the transcript. Fix: in FR-197 Replay source, state that a minimized revision is `Witness` only if the backend rerun on its values produced its own admitted transcript. Otherwise it is `Input`. A parent's transcript never moves to a child. Add a TC-219 or TC-272 vector. | FR-197-AC-5; FR-197 Replay source; AD-016 Replay source |
| FND-009 | medium | Trust boundary: "Backend evidence: Yes" depends only on the value being the `Witness` arm. Any string that passes `parse` can be admitted, and nothing ties the transcript to the obligation it claims to witness. Fix: in AD-016 arrow 7 and FR-197, require the harness symbol derived from the transcript to equal the harness that arrow 5 generated for the packet's obligation identity. Require the backend-evidence verdict to keep the packet's backend run identity (ADR-013 O-19 `backend` member). A mismatch refuses with a typed cause. Add a TC-272 vector with a well-formed transcript for another harness. | AD-016 arrow 7; AD-016 Replay source; FR-197; ADR-013 O-25 |
| FND-010 | low | TC-272 RS-09 refuses a packet that has both arms, but no FR-197 AC requires this, and no catalog cause is named. A packet with neither arm has no vector. Fix: add FR-197-AC-11: "A packet whose serialized source holds both arms or neither arm is refused at deserialization with a typed cause, before any replay runs." Map it to TC-272, and add a neither-arm vector. | TC-272 RS-09; FR-197; spec/complete-v1/tests.md |
| FND-011 | low | Identity wording: after QC-14, the arrow-6 "Identity retained" cell says "Obligation identity". Arrow 7 still says "Obligation id, node id". "Obligation id" is the clause-id reading that QC-14 removes. Fix: change arrow 7 to "Obligation identity (arrow 5 digest), node id". | AD-016 arrow 7; QC-14 |
| FND-012 | low | Vocabulary: the `Input`-arm settlement has a label, `reproduced-without-witness`. The `Witness`-arm settlement is plain prose, "reproduced-with-evaluated-witness", in AD-016. FR-197 gives it no name at all. Fix: give the `Witness`-arm settlement one label in code font, or name it by the FR-352 record value it produces. Use the same label in AD-016 and FR-197. | AD-016 Replay source; AD-016 arrow 7; FR-197 |
| FND-013 | low | Test fixture: TC-272 RS-01 uses "the AD-016 seed counterexample" as the `Witness` transcript. AD-016 records only `concrete_vals` for that seed, not the transcript. The seed is "not yet vendored", and its `obligationIdentitySha256` is now `OPEN — decided in WP9`. Fix: name the fixture source in TC-272 (the IR tc_042 transcript), or state that RS-01 runs on the WP9-vendored vector. | TC-272; AD-016 seed counterexample vector |

## Checklist results

| Check | Result |
| --- | --- |
| Extension points: failure behavior | The `Witness` path is strict: `parse` and `decode` refuse, and disagreement is `inconclusive`. Gaps remain: disagreement on the `Input` arm (FND-003), `Input` refusals (FND-004), failures in the `replay` facade's selection and recompile steps (FND-005), and when admission happens (FND-006). |
| Trust boundaries | The structural rule can be bypassed through deserialization or struct literals (FND-001). A disagreeing `Witness` result can still build a backend-evidence verdict (FND-002). Transcript provenance is not bound to the obligation (FND-009). |
| Entity identity | QC-14's obligation identity is explicit, and `source_span` is outside the preimage. The arrow-7 wording lags (FND-011). `Input` values have no key (FND-004). The arm can be recorded twice through an embedded packet (FND-007). A minimized revision's arm is undefined (FND-008). QC-15's six kernel newtypes each have one digest constructor, so their identity is explicit. QC-17 lists node ids in the kernel row only, so there is one listing. |
| Evaluation purity | The derived witness accessors and the witness-backed match are pure and deterministic. TC-272 RS-03, a stub executor, shows that executor behavior cannot upgrade an `Input` result. No gap. |
| Topological robustness | The diff adds no graph or tree algorithm. Minimization lineage (FR-197-AC-5) is unchanged context. Its only link to the new rule is FND-008. |
| Outcome map (QC-16) | All ten `KaniOutcomeKind`s map to one O-16 category and one FR-331 terminal record, matching the ADR-013 O-16 proof column. `tested` and `failed` are stated. No gap. |

## Evidence

- Diff read in full: `git -C /home/peter/dev/worktrees/qspec-ad016-amend diff 55d2fcc..HEAD` (HEAD `5bd933f`).
- QSL ADR-013 at `31b6054`: O-16 category table, O-25 (Admission, Carrier, Replay source), O-26 validation, O-27, C-11, QC-6 to QC-20, OQ-3, TK-10.
- IR at `954c2f2`: `src/kani/witness.rs` (`pub transcript`, derived `Deserialize`, placeholder `WITNESS_SOURCE_ID`), and `src/kani/replay.rs` (`CounterexamplePacket.input` plus `witness: Option<Witness>`, and `ReplayAgreement.packet` with `witness_backed: bool`). These show the current code shape that FND-001, FND-006 and FND-007 guard against.

## Round 2 dispositions

Evaluated against the tree committed with these records (fixes in `201b4a4`, plus the TC-272 RS-13 compile-fail case). Paths are relative to `spec/`.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | FR-197 Replay source: the backend-evidence verdict type has exactly one construction path, which takes an agreeing `Witness`-arm result; FR-197-AC-9, FR-197-AC-10 (no public field, `Deserialize`, `Default`, `From`, `TryFrom`); TC-272 RS-06; AD-016 Backend-evidence verdict row |
| FND-002 | resolved | FR-197 Replay source: the backend-evidence verdict type has exactly one construction path, which takes an agreeing `Witness`-arm result; FR-197-AC-9, FR-197-AC-10 (no public field, `Deserialize`, `Default`, `From`, `TryFrom`); TC-272 RS-06; AD-016 Backend-evidence verdict row; FR-197 Replay source disagreement bullet; FR-197-AC-13; TC-272 RS-11 (`Witness`), RS-12 (`Input`); AD-016 Replay source last paragraph |
| FND-003 | resolved | FR-197 Replay source: a replay agrees when the native verdict equals the envelope's expected disposition, mapped by the QSL #231 outcome-to-verdict map; Replay envelope: an `Input`-arm envelope's expected disposition is the one recorded with its counterexample; FR-197 Replay source disagreement bullet; FR-197-AC-13; TC-272 RS-11 (`Witness`), RS-12 (`Input`); AD-016 Replay source last paragraph |
| FND-004 | resolved | AD-016 Replay source `Input` row and FR-197 Replay source: canonical assignments keyed by parameter `WireNodeId` (QSL ADR-013 C-11) |
| FND-005 | resolved | AD-016 arrow 7 Refusal cell: the facade returns each O-26 refusal before execution, including unresolved `QualifiedName`, differing `package_id`, differing source digest, missing source bytes, `DependencyIdentityMismatch` (`stale_dependency`), unknown version, arity or type mismatch, a value outside the declared domain, a limit above the reader limit, an `Input` value with no parameter and a parameter with no value; FR-197-AC-20; TC-272 RS-17 |
| FND-006 | resolved | AD-016 Replay ownership, Witness shape row: admission only through `Witness::parse` including on deserialization, private `transcript`, selected trimmed block, refusal of cover playback and of zero or several assertion blocks (O-25); implementer QSL ADR-013 TK-04 |
| FND-007 | resolved | FR-197 Replay source: each arm's result type holds that arm's payload and no `ReplaySource`; FR-197-AC-17; TC-272 RS-15; AD-016 Replay source |
| FND-008 | resolved | FR-197 Replay envelope: a minimized `Witness`-arm envelope is accepted only after native and backend replay preserve the failure, carries the `Witness` arm of that backend re-run and never inherits its parent's transcript; minimizing an `Input`-arm counterexample preserves the native verdict only and yields `Input`-arm revisions. FR-197-AC-5 (`Witness` arm) and FR-197-AC-19 (`Input` arm); TC-219 MR-01 (candidate with no backend re-run) and MR-02 |
| FND-009 | deferred | Deferred, owner WP9: AD-016 arrow 7 Refusal cell `OPEN — decided in WP9` records the check that the transcript's harness symbol equals the arrow-5 harness of the packet's obligation identity. Binding the verdict to a backend run identity is carried by the `Witness`-arm envelope's backend run identity (FR-197 Replay envelope) |
| FND-010 | resolved | FR-197-AC-14; TC-272 RS-09 (both arms), RS-10 (neither) |
| FND-011 | resolved | AD-016 arrow 7 Identity retained: "Obligation identity, node id" |
| FND-012 | resolved | Both labels code-fonted; AD-016 Replay source states they are parity settlements under category success (O-16), not new categories |
| FND-013 | resolved | TC-272 Fixture: IR `tests/kani_replay.rs` tc_042 witness transcript at `954c2f2`, decoded values (8, 992) |

Rows for FND-005, FND-008, FND-012 were updated after the PR #140 review at `9e52406`.
