---
id: SR-202611141503001
title: "Base review of the QSL-1898 protocol-analysis result form"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR #655 at e8c00f9f, diff origin/main...HEAD: proposals/backend-provider-v1/schema.json (Item.assessed_root, InconclusiveCause protocol branch), spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md (Protocol-analysis result, FR-352-AC-14, FR-352-AC-15), spec/functional/protocol/FR-174-project-global-protocols.md, spec/functional/protocol/FR-175-analyze-protocol-realizability.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md (assessed_root item member, six protocol-analysis causes, FR-331-AC-51), spec/objects/protocol/FR-351-separating-witness-record.md (Assessed subject's root, FR-351-AC-8), spec/objects/temporal/FR-360-infinite-trace-result-disposition.md (Scope), spec/test-cases/TC-254-separating-witness-record.md, spec/test-cases/TC-255-native-run-result-v2-wire.md (NR-15, NR-16)"
review_set: base
relationships:
  - { target: ix://agent-ix/quire-specification/FR-352, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
  - { target: ix://agent-ix/quire-specification/FR-351, type: references }
  - { target: ix://agent-ix/quire-specification/FR-360, type: references }
  - { target: ix://agent-ix/quire-specification/FR-174, type: references }
  - { target: ix://agent-ix/quire-specification/FR-175, type: references }
  - { target: ix://agent-ix/quire-specification/TC-254, type: references }
  - { target: ix://agent-ix/quire-specification/TC-255, type: references }
---

## Summary

Ticket: QSL-1898. Base checklist over the QSpec amendment that gives `native-run-result/2` a
protocol-analysis record, adds FR-174/FR-175 to FR-360 Scope, adds six protocol-analysis
inconclusive causes to FR-331 and the schema, and carries the assessed-subject root as
`assessed_root` on `local-projection` and `realizability` request items. Checked against QSL
ADR-036 PA-24, QSL FR-572/573/574 and the cause usage in `qsl-analyze/src/protocol_analysis`
(project.rs, realize.rs) and `qsl-replay/src/protocol_analysis` (protocol_analysis.rs, settle.rs)
and `qsl-replay/src/result/wire.rs` at quire-spec-language f32450ae9. The sub-analyses were not
run as separate lenses; cross-reference and schema/prose consistency checks are folded in here.
`quire validate` on the changed files reports only the pre-existing `required 'id'` errors on
FR-331, FR-351 and FR-360; schema.json parses.

## Verdict

FAIL: FND-001 to FND-004 and FND-007 are medium; the rest are low.

Examined and clean:
- FR-360 Scope wording equals QSL FR-573 "FR-360 Scope" verbatim; the FR-177 link fix is correct.
- The six cause spellings in FR-331, FR-352 and the schema enum equal `InconclusiveCause::as_str`
  (qsl-replay/src/protocol_analysis.rs:792-798), and bare-`type` matches the settle map, which
  drops the `PremiseNotStated.member` field (settle.rs:57-60). `replay-disagreed` (both kinds,
  project.rs:225, realize.rs:908), `strategy-replay-failed` (realize.rs:922-926),
  `premise-not-stated` (`Some(None)` only, declared-empty decides, realize.rs:538-548) and
  `undecided-evaluation` (Open::Undecided, realize.rs:150-163) are defined as the code uses them.
- The FR-360 to FR-331 mapping in FR-352 matches settle.rs (proved/exhaustive/uncertified,
  refuted, inconclusive with bound-reached{max_depth} or a protocol cause, unsupported,
  failed/resource-incomplete to incomplete limit-reached).
- Writing a record for a limit-stopped run and none for a declined item or engine failure matches
  the ADR-036 handoff note.
- The protocol witness shape (no quantifier, no index, JSON-string deciding_element, string
  trace_position, empty steps) matches `RecordWire::of` (wire.rs:801-835) and FR-574.
- Schema: `assessed_root` gated to the two kinds by the `allOf` `if/then`; the new
  `InconclusiveCause` branch is disjoint from the other `oneOf` branches; closed list.
- FR-351-AC-8 traces to TC-254; FR-352-AC-14/15 trace to TC-255 NR-15/NR-16 and the NR rows match
  the ACs member for member.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-331 gives `bound-reached{depth}` only to a search that reached `max_depth` and says every other inconclusive cause is one of the six; QSL emits `bound-reached{depth: max_depth}` also for a `max_history` cut and a qsl-eval method bound (realize.rs Open::Horizon, settle.rs:57), and QSL FR-573 lists `max_history` in the horizon row. A `max_history` horizon has no cause under the PR text | spec/objects/interfaces/FR-331-backend-provider-envelope.md:725-729 |
| FND-002 | medium | FR-352's base witness table still says `deciding_element` is the canonical typed value encoding and `trace_position` is the unsigned trace position; the new section makes them a JSON string of node-id text and a canonical decimal string for the protocol family. The closed table is not amended, so the two sections contradict | spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:157; spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:160; spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:266-276 |
| FND-003 | medium | `settlement` omits `request_index`, `backend`, `kind` and `counterexample`, but the reader refuses a settlement "that FR-331 Result entries refuses"; FR-331 and its schema require those members (and `counterexample` on `refuted`), so read literally every settlement is refused. Say it is read with those four members disregarded | spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:251; spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:289-290 |
| FND-004 | medium | Kind scope of the causes is inconsistent: four causes are defined "Realizability:" and `premise-not-stated` as a realizability claim, yet FR-331-AC-51 admits all six on either kind, the schema admits them on any item kind, and FR-352-AC-14 / NR-15 ask for each of the six on both a local-projection and a realizability record, which cannot arise for projection. The prose restriction of protocol items to these seven causes is not enforced either | spec/objects/interfaces/FR-331-backend-provider-envelope.md:732-737; spec/objects/interfaces/FR-331-backend-provider-envelope.md:976; proposals/backend-provider-v1/schema.json:1094; spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:312; spec/test-cases/TC-255-native-run-result-v2-wire.md:42 |
| FND-005 | low | `unforced-refutation` is defined by a class with no common winning action, but the code also emits it when the game value is a loss and no forced defeating play exists (realize.rs:666-669), which need not involve such a class | spec/objects/interfaces/FR-331-backend-provider-envelope.md:734 |
| FND-006 | low | `cut-count` says the result depended on a count beyond the cut; the code checks no dependence: it replaces every result that passed the strategy gate whenever any channel has an open-range policy (realize.rs:493-498, 929-930), and a refutation under an open range stays `refuted` | spec/objects/interfaces/FR-331-backend-provider-envelope.md:736 |
| FND-007 | medium | An `assessed_root` missing `observation` or `subject` settles `declined` `invalid-input` per the prose and FR-331-AC-51, but the schema makes both required inside `assessed_root`, so a schema-strict reader refuses the envelope before consumption instead. The schema `subject` is also open (no FR-204 identity required, no additionalProperties false), so `{"tag":"object"}` passes | spec/objects/interfaces/FR-331-backend-provider-envelope.md:571-575; proposals/backend-provider-v1/schema.json:313 |
| FND-008 | low | `premise-not-stated` cites FR-175-AC-3, which speaks of missing environment/channel premises, while the cause (and the code) covers role-knowledge and environment premises | spec/objects/interfaces/FR-331-backend-provider-envelope.md:732; spec/functional/protocol/FR-175-analyze-protocol-realizability.md:86 |
| FND-009 | low | FR-331-AC-51 bundles three independent behaviors (cause admission, assessed_root kind gating, absent-root declination) and is verified by Inspection with no TC, though each is a testable admit/refuse behavior | spec/objects/interfaces/FR-331-backend-provider-envelope.md:976 |
| FND-010 | low | The correspondence and refusal lists give no rule for a protocol-analysis record with FR-360 `failed` and execution `failed` (an engine failure, which the prose says gets no record), and do not check that `bound-reached.depth` equals the key's `max_depth` | spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:253-259; spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:285-293 |
| FND-011 | low | One record carries two `basis` members with different vocabularies: top-level FR-243 `basis` and `analysis.settlement.basis` (FR-331 proof basis, `exhaustive` on proved); nothing tells a reader they differ | spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:251 |
| FND-012 | low | `key` "holds each key member as a typed value" can be read as the FR-331 typed-value encoding; QSL's `ProtocolAnalysisKey::to_json` writes plain serde JSON with a `kind` of `projection`/`realizability` | spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:247 |
