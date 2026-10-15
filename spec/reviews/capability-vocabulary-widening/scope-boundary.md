---
id: SR-612
title: "Scope-boundary review of the FR-290 capability vocabulary widening"
type: SpecReview
analysis: scope-boundary
scope: "quire-specification #134 diff 818f555..5d3aa25: FR-290, FR-331, interface_013, FR-270/271/272 and native-diagnostics 1-draft.5, FR-349, FR-355, FR-060/061/240/291/292, AD-016, TC-266, TC-270, TC-271, tests.md rows"
review_set: all
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-290, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
---

## Summary

The diff delivers all six #134 scope items. FR-290 is widened in place to ten
labels, with a descriptive family table and no second vocabulary. The frame
obligation and the sum/case claim have kinds. Backends advertise (kind, mode)
pairs. FR-331 carries per-item `candidates`. The Kani wording follows OBS-013.
Several candidates with no named backend settle `invalid-request`, with no
preference order. The narrowing of FR-060/061/240/291/292 to the
`protocol` family and the FR-349/FR-355 move from "capabilities" to
"obligation forms" are necessary follow-on edits. Without them FR-290 would not
remain the single authority.

Three boundary problems remain. First, the diff fixes the claim extent as the
binary `bounded`/`unbounded`, which prejudges QSL #222. Second, FR-290 states
how "the registry computes" candidate sets and when "the adapter probes" the
tool. These are QSL #185 registry mechanics and QSL/Codegen #86 probe
placement. Third, the diff decides the tool-absence FR-331 result and the
catalog codes, which ADR-012 §7.3, §7.4 and §13.3 Q1 assign to #229. #134's
scope does not list either item.

## Verdict

**ACCEPT WITH FINDINGS.** None of the findings reopens a settled ruling. FND-001 to
FND-003 are wording and allocation fixes inside this PR. FND-004 to FND-006
are low.

## System boundary

| Responsibility | Owner | This diff |
| --- | --- | --- |
| Capability-kind vocabulary, its identity `quire.capability-kind/v1`, claim-form → kind assignment | QSpec FR-290 (#134 items 1, 2) | Owned and delivered |
| (kind, mode) advertisement shape; `requires-bound` as extent-vs-mode comparison | QSpec FR-290 (#134 item 3) | Owned; extent labels overreach (FND-001) |
| Extent representation, "finite bound available" predicate | QSL #222 | Cited as external; the extent labels themselves are fixed here (FND-001) |
| `candidates` wire field of the negotiation request | QSpec FR-331 (#134 item 4; ADR-012 §13.4 Q2) | Owned and delivered; member shape (FND-004) |
| Candidate-set computation, registry value, routing | QSL #185 (ADR-012 §7.1, §7.2) | Restated normatively in FR-290 (FND-002) |
| Single negotiation point and dispositions | CG `negotiate_*` (AD-016) | Referenced; interface_013 invariant "never computes or reorders" keeps the boundary |
| Multi-candidate rule | QSpec FR-290 (#134 item 6) | Owned and delivered |
| Kani descriptor ownership wording | QSpec FR-290 (#134 item 5) | Delivered |
| Solver/tool absence outcome, catalog code | #229 decides (ADR-012 §7.3, §7.4, §13.3 Q1); catalog file is QSpec's | Decided here, outside #134 scope (FND-003) |
| Probe placement and fault-injection evidence | Codegen #86 (ADR-012 §7.4, §14.1) | Specified and tested in QSpec (FND-002) |

## Scope-item coverage

| #134 item | Delivered by | Status |
| --- | --- | --- |
| 1. Non-protocol kinds, same identity block, record the authority choice | FR-290 table, Families, Vocabulary authority | Met; the authority choice (widen in place, families descriptive) is recorded |
| 2. Frame obligation and sum/case kinds | FR-290 claim-form table; AD-016 frame row | Met; Relation-family claims have no row (FND-005) |
| 3. Advertised mode beside each kind | FR-290 Advertised mode | Met; extent labels overreach (FND-001) |
| 4. FR-331 candidate set | FR-331 `candidates`, AC-4, AC-5; interface_013 | Met; member shape (FND-004) |
| 5. Kani wording (OBS-013) | FR-290 Backend absence bullet | Met |
| 6. Several candidates, none named → `invalid-request`, no preference order | FR-290 candidate table, AC-7 | Met |
| Acceptance: every #185-routed capability has one kind | QSL `FamilyCheck`/`StateOperation`/`FiniteReplay`/`TemporalProjection` map to one kind each by claim form; no label is mapped | Met |
| Non-goal: no four-kind compatibility | FR-290-AC-1, TC-271 CK-02, `invalid_capability` "no cause maps a refused label" | Met |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-290 "Advertised mode" says a requested item "records its claim's extent as `bounded` or `unbounded`". FR-331 carries that value, and TC-271 CK-04 refuses the mode `finite` as `unknown-mode`. This fixes a two-valued extent vocabulary. ADR-012 §1.1 lists finite, bounded and unbounded extents and leaves the extent vocabulary to QSL #222; ADR-013 Q210-2 says the same. FR-290's own sentence gives the extent representation to #222, so the text contradicts itself. Fix: keep the two advertised modes, which are #134 item 3. Word the request side as "the claim's extent in the #222 representation, classified as within or outside each advertised mode". Word the table rows as that classification rather than as extent labels. Drop the `finite` mutation from CK-04, or reword it as a mode-label refusal that does not depend on the extent vocabulary. | FR-290; FR-331; TC-271 CK-04; ADR-012 §1.1; QSL #222 |
| FND-002 | medium | FR-290 "Candidate set and negotiation" opens "The registry computes, for each requested item, the candidate set". The "Tool absence" bullet fixes when "the adapter probes it before the run", and TC-271 TA-03 tests "the probe runs after routing and before the run, never before routing". The computation and routing belong to QSL #185, and the probe placement and its fault-injection evidence belong to Codegen #86 (ADR-012 §7.1, §7.2, §7.4, §14.1). QSpec becomes a second normative source that can drift from them. Fix: state the candidate rule as the validity condition of an FR-331 `candidates` value: each member advertises every required kind; a named backend gives that backend or empty; an unregistered name gives the unknown-backend mark. Cite QSL #185 / ADR-012 §7.2 as the computation owner. Remove the probe-timing clause from FR-290, keeping only the observable result. Delete TA-03, or move it to a Codegen #86 note. | FR-290; TC-271 TA-03; interface_013; ADR-012 §7.2, §7.4 |
| FND-003 | medium | Tool absence is outside #134's six scope items. It includes the FR-290 "Tool absence" bullet, the FR-331 `unsupported` result, `pinned tool identity` in the manifest, FR-290-AC-8, FR-331-AC-6, `unsupported_projection`/`tool-unavailable` and TC-271 TA-01/02. The catalog codes (`invalid_capability`, `unbounded-extent`) are also decided here, and ADR-012 §7.3, §7.4 and §13.3 Q1 give that decision to #229. The content matches the settled ruling (warned `unsupported`, never a hold). The catalog and FR-331 are QSpec files, so QSpec is the right place to write it. Allocation is the gap: #229 or #134 must record that QSpec made this decision. Fix: amend #134's scope with the owner, adding a 7th item: "tool-absence FR-331 result and the `invalid_capability`/`unbounded-extent`/`tool-unavailable` catalog causes, answering ADR-012 §13.3 Q1 for #229". Comment on #229 that the answer lands in QSpec catalog `1-draft.5`, so that #229 does not mint a second code. The #222 confirmation of the boundedness reading that §7.4 asks for is still owed. | FR-290-AC-8; FR-331-AC-6; FR-272-AC-9; native-diagnostics `1-draft.5`; ADR-012 §7.3, §7.4, §13.3 |
| FND-004 | low | FR-331 `candidates` is "the set of registered backend identities … in ascending identity order". ADR-013 O-19, as adopted in ADR-012 §7.1, puts a `BackendId` on the wire as `backend{identity, manifest_digest}`, sorted by (identity, manifest digest). Under the FR-331 text, two registrants with one identity and different manifests have no defined order and cannot be told apart. Fix: define each `candidates` member as the FR-331 manifest's `backend{identity, manifest_digest}` and the order as (identity, manifest_digest). Apply the same wording to `ambiguous-backend`'s "every candidate identity in ascending order" in FR-272 and native-diagnostics. | FR-331; FR-272; native-diagnostics; ADR-012 §7.1 |
| FND-005 | low | The claim-form assignment table has no row for the ADR-012 `Relation` family, which covers refinement between models and model-to-implementation relations. #134's acceptance requires one kind for every capability #185 must route. Protocol refinement is covered by `refinement`, and FR-353 bindings feed the `operation-contract` path. Neither is stated, so #223/#191 must infer it. Fix: add one row that names the kind for a model refinement claim and states that an FR-353 abstraction relation is consumed within `operation-contract`, not requested as a claim. | FR-290 claim-form table; FR-353; FR-177; ADR-012 §1 |
| FND-006 | low | FR-290 is now the ecosystem-wide vocabulary, but it still lives in `spec/objects/protocol/FR-290-protocol-claim-kind.md` and the protocol module's inventory. The ownership allocation reads as protocol-owned, while the families it now covers belong to value, state and temporal subsystems. The fix is not required in this PR, because moving the file is a path churn that needs owner approval. Fix: ask the owner whether to relocate the file (for example to `objects/foundation/` or `objects/interfaces/`). Until then, add a one-line note in spec.md's FR-290 row that it is the cross-family authority. | FR-290; spec/spec.md |

## Responsibility allocation

| Requirement | Owning component | Class |
| --- | --- | --- |
| FR-290 (capability-kind vocabulary, claim-form assignment, mode comparison, multi-candidate rule) | QSpec object layer, consumed by QSL #185/#229, CG `negotiate_*`, quire-protocol | cross-cutting |
| FR-331 (backend provider envelope with `candidates`) | QSpec interfaces, produced by QSL `route`, consumed by CG | infrastructure |
| interface_013 (BackendProvider negotiate/execute) | CG `negotiate_*` | infrastructure |
| FR-270/271/272, native-diagnostics `1-draft.5` | QSpec foundation catalog, re-vendored by QSL | cross-cutting |
| FR-349, FR-355 (SMT registration, differential parity) | SMT backend negotiation arm (CG) and runner | core |
| FR-060, FR-061, FR-240, FR-291, FR-292 (protocol-family results) | quire-protocol analysis | core |
| AD-016 frame row | QSpec assurance | cross-cutting |

## Round 2 dispositions

Evaluated against the uncommitted working tree on 5d3aa25. Paths are relative to the repository root.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | The request carries the #222 extent and #222's classification; #222 owns representation, classification and the finite-bound predicate, `spec/objects/protocol/FR-290-protocol-claim-kind.md:119-125`. CK-04's `finite` is now only a refused mode label (TC-271:41). |
| FND-002 | resolved | The candidate rule is now the validity condition of an FR-331 `candidates` value, and QSL #185 is cited as the computation owner (FR-290:155-167). The probe's placement and fault-injection evidence are assigned to `quire-contract-codegen` (FR-290:210-212). The old TA-03 probe-timing vector is replaced by probe error and limit (TC-271:62). |
| FND-003 | accepted-as-ruled | Tool absence and the catalog codes are kept in these specs at the coordinator's direction. |
| FND-004 | resolved | Candidate = (identity, manifest digest), ordered by identity then digest, FR-290:147-150 and FR-331:29; `ambiguous-backend` retains candidates "in candidate order", native-diagnostics:97 and the FR-272 row |
| FND-005 | resolved | FR-290:99-101 |
| FND-006 | accepted-as-ruled | Relocation is an owner question. The cross-family note is added at `spec/spec.md:1076`. |

Round 2 verdict: ACCEPT
