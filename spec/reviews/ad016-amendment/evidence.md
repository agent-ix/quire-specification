---
id: SR-618
title: "Evidence review of the AD-016 amendment replay-source rule"
type: SpecReview
analysis: evidence
scope: "quire-specification diff 55d2fcc..5bd933f; FR-197-AC-6 to FR-197-AC-10; TC-272; AD-016 Replay source; spec/complete-v1/tests.md"
review_set: all
evaluated_revision: "5bd933f"
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/AD-016, type: references }
  - { target: ix://agent-ix/quire-specification/FR-197, type: references }
---

## Summary

This review asks one question: can TC-272 fail when FR-197-AC-6 to AC-10 are
violated? A stub that returns the expected answer must not pass.

`quoin advise` reports no method mismatch for FR-197-AC-6 to AC-10. All five
are authored `Test (TC-272)`. The advisor matched AC-6 to AC-9 on property
shape `universal` (property-based-testing) and AC-10 on `example`
(unit-testing, bdd-spec-by-example). No recommendation matched on an evidence
characteristic.

The method is right. The vectors are not strong enough. An adapter that never
calls the executor passes every runtime vector. Two paths mint a
backend-evidence verdict with no real backend or no real native run, and
TC-272 exercises neither: a hand-written transcript, and a caller-supplied
executor on the `Witness` arm. The compile-fail vectors can pass for the wrong
reason, and the sets they must cover ("every position", "every constructor",
"every replay-result type") are listed by hand with no closure check.

RS-07 does discriminate against the IR as merged at `954c2f2`.
`ReplayAgreement.witness_backed` and `NativeReplayAgreement.witness_backed`
(`src/kani/replay.rs:39`, `:51`) are stored flags, and the packet stores
`input` beside `witness`. A correct RS-07 fails on that code, as it should.

## Verdict

**REVISE**: FND-001 to FND-003 (high) and FND-004 to FND-006 (medium) must be
fixed. The low findings can be fixed in this PR or recorded.

## Method

1. `quoin advise --json` on the worktree, filtered to FR-197. This is the
   deterministic step.
2. For each AC, name a wrong implementation that a careless or adversarial
   author could write. Then check whether some TC-272 vector fails on it.
3. Read IR `954c2f2` `src/kani/witness.rs` and `src/kani/replay.rs` to find
   the constructors and executor seams that exist today.

Step 2 and step 3 are reviewer judgement, not catalog output.

## Stub matrix

| AC | Wrong implementation | Vector that should fail it | Fails today? |
| --- | --- | --- | --- |
| AC-6 | The adapter returns `reproduced-without-witness` for every `Input` packet and never calls the executor | none; RS-02 and RS-03 only use agreeing executors | No (FND-001) |
| AC-6 | The `Witness` arm returns a reproduced result without passing the decoded values to the executor | none; RS-01 does not look at the executor's arguments | No (FND-001) |
| AC-7 | The backend-evidence verdict type does not exist, or the test has a typo | RS-04 and RS-05 compile-fail | No, they pass vacuously (FND-004) |
| AC-7 | A new public fn that takes the verdict is added later | nothing lists the positions | No (FND-005) |
| AC-8 | The verdict derives `Deserialize`, has pub fields or `Default`, or implements `From<InputResult>` | RS-06 covers only calls to named constructors | No (FND-006) |
| AC-8 | A hand-written transcript of the `Input` values is wrapped as `Witness { transcript }` | none | No (FND-002) |
| AC-8 | The `Witness` arm is replayed with a stub executor that always agrees | none; RS-03 stubs only the `Input` arm | No (FND-003) |
| AC-9 | A stored flag is renamed, for example `is_backend: bool` or `source_kind`, and the destructuring pattern is updated to match | RS-07 has no list of allowed fields | No (FND-007) |
| AC-10 | The answer is right for the two fixture packets only | RS-08 checks two packets | Partly (FND-008) |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | TC-272 has no negative runtime vector. RS-01 to RS-03 all use an executor that agrees. So an adapter that ignores the executor and returns a fixed result per arm passes all three. AC-6 says an *agreeing* `Input` replay, but no vector shows that a disagreeing one settles anything else. RS-01 also does not show that the witness was evaluated. Fix: add RS-10: the `Input` packet with an executor that does not reproduce gives `inconclusive` with the typed parity cause (AD-016 arrow 7), not `reproduced-without-witness`. Add RS-11: the `Witness` packet with an executor that does not reproduce gives `inconclusive` and no backend-evidence verdict. In RS-01, use a recording executor and assert that it received `decode(transcript)` = (8, 992). | FR-197-AC-6; TC-272 RS-01 to RS-03 | correct-requirement-no-evidence |
| FND-002 | high | A `Witness` can be forged, so the type rule can be bypassed. In IR `954c2f2`, `Witness` has `pub transcript: String`, derives `Deserialize`, and has a public `Witness::parse`. Any caller can write a playback block that encodes the `Input` values (8, 992), wrap it as `ReplaySource::Witness`, and get a backend-evidence verdict that meets AC-8. No backend ran. FR-197 says "admitted transcript" but defines no admission. Fix: add an AC saying a `Witness` is constructible only by the backend-run admission path. That means a private field, no struct literal, no public `parse` outside admission, and `Deserialize` only through packet admission that binds the transcript to a recorded run (`harnessSha256`, obligation identity). Add TC-272 vectors: a compile-fail `Witness { transcript: .. }` literal, and a hand-written transcript of (8, 992) that is refused before any verdict. | FR-197 Replay source; FR-197-AC-8; AD-016 Replay source; TC-272 | missing-requirement |
| FND-003 | high | RS-03 puts the stub executor on the wrong arm. On the `Input` arm the stub does no harm, and from the adapter's side RS-03 is the same test as RS-02. The harmful case is the `Witness` arm. IR `954c2f2` `replay_counterexample(.., execute_native: impl FnOnce(&FiniteInput) -> KaniOutcome)` takes the executor from the caller. With a stub that always agrees, it mints backend evidence with no native run. AD-016 arrow 7 fixes the executor as `CheckedPackage::call` through the layer-6 `replay` facade, but no AC or vector enforces that. Fix: add an AC saying the public replay entry that yields a `Witness`-arm result takes no caller-supplied executor and always calls the QSL `replay` facade. Add a compile-fail vector that passes an executor closure to that entry. RS-01 and RS-02 should run the real facade on the seed package, not an "executor that reproduces". Keep executor injection behind a test-only seam that cannot produce a `Witness`-arm result. | FR-197-AC-8; AD-016 arrow 7; TC-272 RS-01 to RS-03, Test Procedure | missing-requirement |
| FND-004 | medium | The compile-fail vectors can pass for the wrong reason. A `compile_fail` doctest checks only that compilation fails, not the error text. A missing type, a typo or an unresolved import also fails to compile. "The compiler error names the rejected type" is also true of `cannot find type`. Fix: allow `trybuild` only, with committed `.stderr` files that show the expected type-mismatch or unsatisfied-bound error (E0308/E0277) between the `Input`-arm type and the verdict type. Pair each compile-fail case with a compile-pass case that puts the `Witness`-arm result in the same position. Remove the `compile_fail` doctest option. | FR-197-AC-7; FR-197-AC-8; TC-272 RS-04 to RS-06, Test Procedure | correct-requirement-no-evidence |
| FND-005 | medium | "Every parameter, field and return position", "every public constructor" and "every replay-result type" are sets that the vector author lists by hand. A position, constructor or result type added later has no vector, and TC-272 still passes. Generic positions are also missed, for example `fn f<T: Evidence>(t: T)` or `impl Into<Verdict>`. Fix: TC-272 names each set as a closed list. Add a closure vector that reads the crate's public API (rustdoc JSON or a `cargo public-api` snapshot). It fails when any public item mentions the verdict type, a replay-result type or `ReplaySource` and has no RS vector. | FR-197-AC-7; FR-197-AC-8; FR-197-AC-9; TC-272 | correct-requirement-no-evidence |
| FND-006 | medium | RS-06 covers calls to named constructors only. It leaves out the other ways to get a verdict value: a struct literal with pub fields, `Default`, `Deserialize`, and `From`/`TryFrom`/`Into` from an `Input`-arm result or from bare values. A verdict deserialized from JSON is backend evidence with no `Witness` behind it. Fix: in RS-06, add a compile-fail case for each of these. If the verdict must be serialized, state in FR-197 that deserializing a verdict re-admits its `Witness` (FND-002) and does not accept the verdict as stored. | FR-197-AC-8; TC-272 RS-06 | correct-requirement-no-evidence |
| FND-007 | medium | RS-07 does not say what counts as a "witness-backed field". The implementer writes the destructuring pattern. A stored flag under another name, such as `is_backend: bool`, `evidence: bool` or `source_kind: SourceKind`, passes once the pattern is updated. Fix: TC-272 lists the exact allowed field set for the packet and for each replay-result type. RS-07 asserts that the serialized key set equals that list and that the destructuring pattern names exactly that list. Any field whose value is a function of the arm fails the vector. | FR-197-AC-9; TC-272 RS-07 | correct-requirement-no-evidence |
| FND-008 | low | AC-10 says "every packet", but RS-08 checks two fixed packets. "Derived by a match" cannot be seen at runtime. It holds only because AC-9 bans a stored field. Fix: generate packets of both arms as a property (arbitrary `Input` values and admitted transcripts) and assert that the answer equals `matches!(source, ReplaySource::Witness(_))`. State that the "derived by a match" part rests on RS-07. | FR-197-AC-10; TC-272 RS-08 | correct-requirement-no-evidence |
| FND-009 | low | RS-09 traces to no AC. "Exactly one arm" is FR-197 prose with no SHALL. The "typed cause" is not named, so any error passes. With serde, an untagged enum, or an enum without `deny_unknown_fields`, can quietly pick one arm from a body that has both. Fix: add FR-197-AC-11: a packet with both arms or no arm is refused with a named cause before replay. Trace it to RS-09 and name the cause in the expected result. | FR-197 Replay source; TC-272 RS-09 | missing-requirement |
| FND-010 | low | The RS-01 fixture does not exist yet. The AD-016 seed vector holds `concrete_vals` but no transcript ("seed, not yet vendored"). Its `obligationIdentitySha256` is `OPEN — decided in WP9`. Fix: TC-272 names the WP9-vendored seed transcript as the RS-01 fixture, and states that RS-01 cannot run until it lands. Or TC-272 embeds a transcript of its own. | TC-272 RS-01; AD-016 seed counterexample vector | correct-requirement-no-evidence |
| FND-011 | low | Method, by judgement (the advisor did not recommend this). The advisor gives AC-7 and AC-8 property-based-testing from shape `universal`. By their wording, they are compile-time properties. The catalog method is `compile-time-check` (class Analysis, evidence kind Compile), and `trybuild` gives that evidence. `Test` is acceptable because `trybuild` runs under the test harness. Fix: author AC-7 and AC-8 as compile-time-check in their Verification cells. Or keep `Test` and state in TC-272 that RS-04 to RS-06 give Compile evidence. | FR-197-AC-7; FR-197-AC-8 | wrong-requirement |

## Round 2 dispositions

Evaluated against the tree committed with these records (fixes in `201b4a4`, plus the TC-272 RS-13 compile-fail case). Paths are relative to `spec/`.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | FR-197 Replay source disagreement bullet; FR-197-AC-13; TC-272 RS-11 (`Witness`), RS-12 (`Input`); AD-016 Replay source last paragraph; TC-272 RS-14 records the facade call with decoded arguments (8, 992) |
| FND-002 | partially resolved | FR-197 Replay source SHALL "A `Witness` SHALL be constructed only through `Witness::parse`, including on deserialization"; FR-197-AC-15; TC-272 RS-13 (refused transcripts and a compile-fail `Witness` literal); AD-016 Witness shape row (O-25, TK-04 (IR #144)). Deferred, owner WP9: binding a transcript to a recorded backend run, marked `OPEN — decided in WP9` in the AD-016 arrow 7 Refusal cell; listed in the PR body deferrals |
| FND-003 | resolved | FR-197 Replay source: the public `Witness` replay entry calls the QSL `replay` facade and takes no caller-supplied executor; FR-197-AC-16; TC-272 RS-14 |
| FND-004 | resolved | TC-272 Test Procedure: `trybuild` cases with `.stderr` snapshots and compile-pass controls; closure taken from a public-API listing of the replay crates |
| FND-005 | resolved | TC-272 Test Procedure: `trybuild` cases with `.stderr` snapshots and compile-pass controls; closure taken from a public-API listing of the replay crates |
| FND-006 | resolved | FR-197 Replay source: the backend-evidence verdict type has exactly one construction path, which takes an agreeing `Witness`-arm result; FR-197-AC-9, FR-197-AC-10 (no public field, `Deserialize`, `Default`, `From`, `TryFrom`); TC-272 RS-06; AD-016 Backend-evidence verdict row |
| FND-007 | resolved | FR-197 defines a witness-backed field; FR-197-AC-11; TC-272 RS-07 lists the exact allowed field set of the packet, both arm results and the per-item result |
| FND-008 | resolved | FR-197-AC-12 rests on FR-197-AC-11 (no stored field) and FR-197-AC-17 (no `ReplaySource` in results); TC-272 RS-07, RS-08, RS-15 |
| FND-009 | resolved | FR-197-AC-14; TC-272 RS-09 (both arms), RS-10 (neither) |
| FND-010 | resolved | TC-272 Fixture: IR `tests/kani_replay.rs` tc_042 witness transcript at `954c2f2`, decoded values (8, 992) |
| FND-011 | resolved | Kept `Test`; TC-272 Expected Results states RS-04 to RS-06 and RS-15 are Compile evidence run under the test harness |

Rows for FND-002, FND-007 were updated after the PR #140 review at `9e52406`.
