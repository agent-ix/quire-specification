---
id: SR-611
title: "Risk-complexity review of the FR-290 capability-vocabulary widening"
type: SpecReview
analysis: risk-complexity
scope: "quire-specification #134 diff 818f555..5d3aa25: FR-290, FR-331, interface_013, FR-270, FR-271, FR-272, native-diagnostics 1-draft.5, FR-349, FR-355, AD-016, FR-060, FR-061, FR-240, FR-291, FR-292, TC-266, TC-270, TC-271"
review_set: all
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-290, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
---

## Summary

Verdict: REVISE. The widening does what #134 asks. FR-290 stays the one
vocabulary authority. Each non-protocol family gets exactly one kind. Proof
techniques stay out of the vocabulary. Frames go to `operation-contract`, and
`case` exhaustiveness needs no kind. The four QSL capabilities map one to one:
`FamilyCheck` to `value-validity`, `StateOperation` to `operation-contract`,
`FiniteReplay` to `finite-replay` and `TemporalProjection` to
`temporal-satisfaction`. The catalog bump to `1-draft.5` has a low blast
radius.

One high finding blocks. FR-290-AC-1 refuses any carrier that does not declare
`quire.capability-kind/v1`. The two carriers this repository owns, FR-331 and
FR-240, declare no such member. Four medium findings concern ambiguity risk
from coarse kinds with kind-only matching, the Z3 tool-absence path, the
`refinement` gates of the Relation family, and artifacts left behind on tool
absence.

## Method

Each changed requirement was scored on technical risk and volatility, per
`spec-risk-complexity-analysis`. The diff was read against QSL ADR-012 draft
(QSL PR #234, §1.1, §7, §12, §13.3) and issue #134. The following rulings are
settled and were not reopened:

- FR-290 is the single vocabulary, widened in place.
- Absence settles `unsupported`, warned.
- CG `negotiate_*` is the single negotiation point.
- The old four-kind vocabulary gets no compatibility path.
- More than one candidate with none named settles `invalid-request`.
- Extent representation belongs to QSL #222.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-290 says every carrier of capability kinds declares the vocabulary identity `quire.capability-kind/v1`, and FR-290-AC-1 refuses a carrier that declares none, with none of its labels read. It also gives each format's owner the member that carries the identity. This repository owns FR-331 (manifest, request) and FR-240 (result `claim`), and the diff adds no such member to either. Read literally, every FR-331 request and every FR-240 result is refused. TC-271 CK-03 has no concrete carrier to mutate. Fix: add a vocabulary-identity member to the FR-331 manifest and request, and to the FR-240 claim and FR-286 preimage. Or state in FR-331 and FR-240 that their contract version (`quire.backend-provider/v1`, the FR-240 result version) fixes the vocabulary identity. Then name the carrier in CK-03. | FR-290-AC-1; FR-331; FR-240; FR-286; TC-271 CK-03 |
| FND-002 | medium | There is one coarse kind per family, candidates match on kind alone, and there is no preference order. Once Kani and the FR-349 SMT backend both advertise `value-validity` or `operation-contract`, every item that names no backend settles `invalid-request`. The registry cannot tell a caller which backend discharges induction, frames or quantification; only the arm knows. So naming a backend becomes the normal path, not the exception. Matching on kind alone also departs from ADR-012 §1.1 and §7.2, which match on (capability, mode) and say a bounded-only advertiser "is not a candidate" for an unbounded requirement. Fix: keep the ruling, but state the consequence in FR-290 (a caller names a backend whenever two registrants share a kind). Add a TC-271 vector with Kani (`value-validity`, `bounded`) and SMT (`value-validity`, `unbounded`) and no named backend, settling `invalid-request`. Record on QSL #229 / PR #234 that ADR-012 §1.1 and §7.2 must adopt kind-only matching with the mode compared at `negotiate_*`. | FR-290 Advertised mode, Candidate set; FR-349; ADR-012 §1.1, §7.2 |
| FND-003 | medium | Tool absence is ambiguous for the FR-349 cross-check. FR-349 makes an unregistered Z3 a warned absent cross-check, where cvc5 settles alone. FR-290 "Tool absence" makes a missing or mismatched pinned tool on the routed backend the FR-331 result `unsupported`. When Z3 is registered but its tool is missing at probe, neither text says which rule applies: a warned cross-check absence with cvc5's verdict kept, or `unsupported` for the whole item. Fix: in FR-349, state that a registered Z3 whose tool is missing or mismatched at probe is the warned absent cross-check, and that it never turns cvc5's settlement into `unsupported`. Add an SMT-03b vector to TC-266. | FR-290 Tool absence; FR-349-AC-6; TC-266 SMT-03 |
| FND-004 | medium | The claim-form assignment table has no row for the ADR-012 `Relation` family gates: QSL #191 (spec-versioning refinement) and #192 (profile-layering refinement). The only fitting label is `refinement`, which is in the `protocol` family. Under "a protocol claim is a claim whose kind is in the `protocol` family", these gates would become protocol claims that need an FR-291 fragment and an FR-292 backend. That breaks the #134 acceptance "every capability QSL #185 must route has exactly one kind" if the gates route to a backend. Fix: add a claim-form row for the #191 and #192 refinement gates. Either give them "none; discharged during language admission", like `case` exhaustiveness, or assign `refinement` and move it out of the protocol-only family, with FR-291 and FR-292 still limited to protocol claims. Note in the row that FR-353's abstraction relation adds no kind: it is carried into the `operation-contract` claim it binds. | FR-290 Claim-form assignment, Families; FR-291; FR-292; QSL #191, #192, #198 |
| FND-005 | medium | FR-331-AC-2 and TC-271's expected results say no `unsupported` item emits an artifact. The tool probe runs after routing and "before the run". But FR-331-AC-1 puts artifact emission after negotiation, so a `supported` item may already have generated artifacts when the probe finds the tool missing. A tool-absence item (`supported` disposition, `unsupported` result) therefore may or may not have artifacts, and "unsupported" in AC-2 could mean the disposition or the result. Fix: state in FR-290 Tool absence and FR-331-AC-6 whether artifacts emitted before the probe are kept in `artifacts` for a tool-absence result, or are not emitted. Then add that outcome to TC-271 TA-01. | FR-331-AC-1, AC-2, AC-6; FR-290 Tool absence; TC-271 TA-01, TA-03 |
| FND-006 | low | There are two tool identities with no stated authority between them. FR-331 carries both a manifest "pinned tool identity" and a request "tool lock". FR-290 compares the probed tool with "the descriptor's pinned tool identity". Fix: state in FR-331 that the request tool lock must equal the manifest's pinned tool identity, and refuse the request before consumption if it does not. Or name the one of the two that the probe compares against. | FR-331 Properties; FR-290 Tool absence; TC-271 TA-02 |
| FND-007 | low | Two gaps in the catalog wording. The FR-271 `unsupported_projection` row now reads "no registered backend supports", which drops the arm-level case: the obligation form that the one candidate's arm does not discharge, which native-diagnostics and FR-272 both keep. It also marks only `tool-unavailable` as revision `1-draft.5`, although `unbounded-extent` is new in `1-draft.5` too. Fix: add the arm-level obligation-form case to the FR-271 row, and mark both `unbounded-extent` and `tool-unavailable` as revision `1-draft.5`. | FR-271; FR-272; native-diagnostics `1-draft.5` |
| FND-008 | low | The catalog revision bump is low risk. No open QSpec PR claims `1-draft.5`. Open PR #76 edits the same catalog, FR-270, FR-271, FR-272 and `tests/checked_package_v2.rs`, but from `1-draft.3`, so it must rebase in any case. QSL's composed definition source still embeds `quire.native.diagnostics/v1` at `1-draft.1`, so no downstream digest pin breaks. Renaming the header test to `..._pins_current_revision` keeps it tracing FR-272-AC-7. Fix: none required. Land #134 before rebasing #76, and have #76 bump to `1-draft.6` if it adds causes. | FR-270; native-diagnostics; tests/checked_package_v2.rs; PR #76 |

## Risk register

| Req | Tech Risk | Volatility | Drivers | Mitigation |
| --- | --- | --- | --- | --- |
| FR-290 | Medium | Medium | Ten-member closed vocabulary with a new identity. Kind-only candidate matching departs from the draft ADR-012 (FND-002). Relation-family gap (FND-004). | TC-271 CK and NG vectors, run under both registration orders. Hand FND-002 to QSL #229. Add the FND-004 row. |
| FR-331 | Medium | Medium | New required `candidates` member changes request identity. New result `unsupported`, recorded after a `supported` disposition. Carrier identity missing (FND-001). | TC-271 NG-08, NG-09 and TA vectors. FND-001 fix before landing. |
| interface_013 | Low | Low | `negotiate` takes candidate sets and never computes them. | Covered by TC-271 NG vectors. |
| FR-270 / FR-271 / FR-272 | Low | Low | Catalog `1-draft.5` adds one code and two causes. | FR-272-AC-9 via TC-271. FND-007 text fix. |
| FR-349 / FR-355 | Medium | Medium | Techniques moved from kinds to arm-settled obligation forms. Z3 tool-absence ambiguity (FND-003). | TC-266 SMT-01b. Add SMT-03b. |
| AD-016 | Low | Low | Frame row now names `operation-contract`; the arm still settles `unsupported` until IR #109. | None beyond TC-271 CK-05. |
| FR-060 / FR-061 / FR-240 / FR-291 / FR-292 | Low | Low | Scope narrowed to protocol-family members. | FND-001 carrier fix for FR-240. |

## Assessment of the targeted questions

- **Kind granularity.** One kind per non-protocol family, with techniques kept
  out, is the right call for vocabulary stability. The FR-349 techniques, which
  depend on the backend, do not make the closed vocabulary change. The cost is
  more ambiguity at selection time (FND-002). The ruling accepts that cost, and
  the spec should state it.
- **Frame obligation to `operation-contract`.** This is low risk. It agrees
  with AD-016's frame row: Kani is a candidate by kind, and its arm settles
  `unsupported`, warned, until IR #109. The warning names the obligation form.
- **`case` exhaustiveness needs no kind.** This is low risk. It agrees with
  ADR-012 §12.1 ("Requirements: none beyond the `Value` family's"). TC-271
  CK-06 proves that no candidate set is computed.
- **Tool absence as FR-331 result `unsupported`.** The disposition and the
  result stay orthogonal, and there is one terminal record. This matches
  ADR-012 §7.4 (probe at execute, no fallback, no downgrade). Open edges:
  FND-003, FND-005 and FND-006.
- **Catalog revision bump.** See FND-008. It is safe.

## Top hazards

1. FND-001: the carrier-identity rule refuses this repository's own carriers.
   Fix it before landing.
2. FND-002: coarse kinds with kind-only matching make `invalid-request` the
   default once a second backend registers. ADR-012 must follow.
3. FND-004: the Relation-family refinement gates have no kind assignment.
4. FND-003: the Z3 cross-check tool-absence path is ambiguous.

## Failure-domain gaps

No `failure-domain` review for this change set exists in
`spec/reviews/capability-vocabulary-widening/` at this revision. FND-003,
FND-005 and FND-006 are failure-domain edges, and they are recorded here until
that review covers them.

## Round 2 dispositions

Evaluated against the uncommitted working tree on 5d3aa25. Paths are relative to the repository root.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | `spec/objects/interfaces/FR-331-backend-provider-envelope.md:26,40,52`; `spec/objects/protocol/FR-240-canonical-protocol-result.md:142-143`; FR-290:69-77; CK-03 names the FR-331 carrier (TC-271:40). The FR-286 preimage is covered because FR-240's contract version selects the vocabulary. |
| FND-002 | resolved | The consequence is stated at FR-290:183-186; NG-02 uses a bounded-only and an unbounded-only registrant (TC-271:46). The ADR-012 alignment belongs to QSL #229 and PR #234, outside this repository. |
| FND-003 | resolved | FR-349:66-69 and AC-6 (FR-349:85); SMT-03b (`spec/test-cases/TC-266-smt-backend-registration.md:39`) |
| FND-004 | resolved | Relation-family refinement gate gives `operation-contract`; FR-353 abstraction relation gives no kind, FR-290:99-101; CK-05 and CK-07 |
| FND-005 | resolved | Artifacts emitted before the probe are retained with no verdict, FR-290:218-220; FR-331-AC-2 (FR-331:47); TA-01 |
| FND-006 | resolved | The tool lock equals the routed pin and is refused otherwise, FR-331:28,40 and AC-7; FR-290:221-222; TA-05 |
| FND-007 | resolved | `spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:62` |
| FND-008 | resolved | No fix required. native-diagnostics:144 states nine causes and matches FR-272. |
| FND-009 (new) | resolved (post-round-2, SR-606) | medium. FR-349:63-69 and AC-6 speak of Z3 being "registered" or absent from registration, and TC-266 SMT-03b says "Register Z3". Under FR-290:163-164,183-186 every registered backend that advertises a kind is a candidate, matched on kind alone. A Z3 registered with the same (kind, mode) pairs as cvc5 would make every unnamed SMT item settle `invalid-request`/`ambiguous-backend`, and naming cvc5 alone would leave Z3 no route for the cross-check. Fix: state in FR-349 that Z3 is the cross-check component of the one SMT backend descriptor, advertises no FR-290 pairs of its own and is never a candidate, and reword "registered"/"registration" for Z3 accordingly (FR-349:63-69,85; TC-266:38-39). |

Round 2 verdict: ACCEPT WITH FINDINGS
