---
id: SR-607
title: "Failure-domain review of the FR-290 capability-vocabulary widening"
type: SpecReview
analysis: failure-domain
scope: "quire-specification #134 diff 818f555..5d3aa25: FR-290, FR-331, interface_013, FR-271, FR-272, native-diagnostics 1-draft.5, FR-349, FR-355, FR-060, FR-061, FR-240, FR-291, FR-292, AD-016, TC-266, TC-270, TC-271"
review_set: all
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-290, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
---

## Summary

REVISE. The diff adds four non-protocol kinds to FR-290 and keeps FR-290 as
the one vocabulary authority. It adds (kind, mode) advertisements, the
candidate-set rule, the tool-absence result, the FR-331 `candidates` field and
the `invalid_capability` catalog code. The settlement paths it names are sound.
Their failure edges are not closed.

- FR-290's own vocabulary-identity rule refuses FR-331, because the diff gives
  FR-331 no member that declares the vocabulary identity (FND-001).
- The number of kinds an item requires is not fixed. The claim-form table does
  not partition claims, so one clause can match two kinds (FND-002).
- `negotiate_*` trusts a candidate set it cannot recompute, and FR-331's
  single-provider manifest cannot describe the backends that set names
  (FND-003, FND-004).
- Backend identity has no uniqueness key or ordering, and refusal granularity
  is undefined for requests and registrations (FND-005, FND-006).
- The tool probe has only two failure outcomes (FND-007).

## Review set

`all`. Failure-domain analysis covered every file in the diff. The checklist
has four parts:

- trust boundaries: the registry, the `negotiate_*` arms, the adapter probe;
- entity identity: kind, family, backend, descriptor, candidate set;
- purity: the registry's candidate computation;
- topological edges: empty, single, split and duplicate sets, and ordering.

This review does not reopen the settled rulings:

- FR-290 is widened in place and stays the one authority.
- Absence settles `unsupported` with a warning.
- `negotiate_*` is the one negotiation point.
- There is no path for the four-kind vocabulary.
- #222 owns the extent representation and the finite-bound predicate.

The design references are QSL ADR-012 (QSL PR #234) §1.1, §7.2–§7.4 and §13.3.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-290 refuses any carrier that declares no `quire.capability-kind/v1` identity, and none of its labels is read. It leaves the declaring member to each format's owner. QSpec owns FR-331, which carries kinds in `manifest` and `request`. The diff adds no vocabulary-identity member to FR-331, so every FR-331 manifest and request would be refused as `unsupported-version`. The protocol result carriers FR-061 and FR-240 carry one kind each, and they have the same gap. Fix: add a `capability_vocabulary` property (exactly `quire.capability-kind/v1`) to FR-331 `manifest` and `request`, and include it in request identity. State which member of FR-061 and FR-240 declares the identity, or name the containing contract version that implies it. Add a TC-271 vector for an FR-331 request with this member absent. | FR-290 Vocabulary authority; FR-290-AC-1; FR-331 Properties; FR-061; FR-240; TC-271 CK-03 |
| FND-002 | medium | The number of kinds an item requires is not fixed. FR-290's Description says "exactly one member". FR-331 (`required FR-290 kinds`) and the candidate-set rule (`every kind the item requires`) are plural, and the mode table compares against "that kind". The claim-form table is not a partition: an operation postcondition that contains a function application or a `case` matches both `value-validity` and `operation-contract`. A temporal requirement over function applications has the same problem. With more than one kind, split coverage (backend X advertises A, backend Y advertises B) gives an empty candidate set. The empty-set warning then names "every required kind that … every registrant leaves unadvertised", which is no kind, so the warning is empty. Fix: say whether an item requires exactly one kind. If it does, give the claim-form table a precedence rule (for example, the innermost enclosing claim form wins: operation clause over value clause, temporal requirement over value clause) and make the FR-331 wording singular. If a set is allowed, define mode comparison per kind and make the empty-set warning name every required kind that has no single registrant advertising the whole set. Add a TC-271 vector for split coverage. | FR-290 Description, Claim-form assignment, Candidate set and negotiation; FR-331 `request`; TC-271 CK-05, NG-01 |
| FND-003 | medium | `negotiate_*` settles over a candidate set that it "never computes or reorders" (interface_013). No rule says what happens when that set does not match the manifest and request. Cases: a candidate that is not registered; a candidate that does not advertise a required kind; a named backend X with `candidates` = {Y}; a non-empty set on an item whose named backend is unregistered. The trust boundary has no strict or resilient policy, so a stale or forged set routes an item to a backend that cannot discharge it. Fix: state that `negotiate_*` checks each candidate against the manifest and the item's required kinds and named backend, without recomputing the set, and that a mismatch settles `invalid-request` under a new `invalid_capability` cause (for example `inconsistent-candidates`) that retains the item index and the offending identity. Add a TC-271 vector. | interface_013 invariants; FR-331 `candidates`; FR-331-AC-4; FR-272 `invalid_capability` |
| FND-004 | medium | Provider, backend and registry identity are mixed together. The FR-331 `manifest` holds one provider identity and its advertised pairs and pinned tool. A request item's `candidates` names many registered backends, and the other backends' descriptors are not in the envelope, so a single-provider envelope cannot describe or check a multi-backend candidate set. Tool identity also has two sources: the request's `tool lock` and the descriptor's pinned tool identity, which the probe compares against. The diff does not say which one the probe uses or what happens when they differ. Fix: say whether FR-331 is the registry-wide negotiation envelope (a manifest per registered backend, keyed by backend identity) or a per-provider envelope (and then where the other candidates' descriptors come from). Name one tool identity for the probe, and refuse a request whose tool lock disagrees with the routed descriptor's pin. | FR-331 `manifest`, `request`, `candidates`, `results`; FR-290 Tool absence; interface_013 |
| FND-005 | medium | Backend identity has no uniqueness key or order. The diff does not say whether descriptors are keyed by identity alone or by identity plus manifest digest or version, as ADR-012 O-19 has it. It does not say how a duplicate registration is refused, or what "ascending identity order" compares (bytes, code points, or a tuple). Registering the same identity twice could give a two-member candidate set with one name, and so an `ambiguous-backend` naming the same backend twice. Fix: declare the backend-identity key and its total order (for example, bytewise over the canonical UTF-8 identity string). Add an `invalid_capability` cause `duplicate-backend` that refuses a second registration of an existing key. Add TC-271 vectors for duplicate registration and for ordering identities that differ only in case or length. | FR-290 Candidate set and negotiation; FR-331 `candidates`, FR-331-AC-5; FR-272 `invalid_capability` |
| FND-006 | medium | The granularity of each refusal is undefined. `absent-kind`, `unknown-kind`, `unknown-mode`, `unsupported-version`, `unknown-backend` and `ambiguous-backend` share one code. Two of them are `invalid-request` dispositions for one item, and FR-331 refuses the whole request "before consumption" on a missing or out-of-domain property. The diff does not say whether a bad kind in one item refuses the whole request (so no item gets a disposition, against interface_013's "every requested item has exactly one disposition") or settles only that item. A registration refusal drops one backend and keeps the registry. Items that backend would have served then settle `unsupported` with a warning that does not mention the refused registration. The refusal also keeps the "registration index", which changes with registration order, although TC-271 says order changes nothing. Fix: give a scope for each cause: whole request, one item, or one registration. Key registration refusals by backend identity, not index. Define the report order when several registrations fail (ascending identity). Make the empty-set warning name the refused registrations that advertised the missing kind. | FR-290 Advertised mode, Identity and comparison; FR-331 Identity and validation; interface_013 invariants; native-diagnostics `invalid_capability`; TC-271 Expected Results |
| FND-007 | medium | The tool probe has only two failure outcomes: missing, or present with a mismatched identity. Other probe results are not assigned: a probe that errors (permission denied, a version query that fails to parse) or hangs has no outcome and no time limit. A tool that passes the probe but is gone or changed at run start has no outcome either. Without these rules, an implementation can hold the item, panic, or report it as `failed`. The solver-absence ruling says absence is never a hold. Fix: give the probe a closed set of outcomes: match, `tool-unavailable` (missing, mismatched, unreadable identity, or probe error), and probe timeout under a stated limit, mapped to `tool-unavailable`. Say that a tool change after the probe records `failed` with a typed cause. Add TC-271 vectors for a probe error and a probe timeout. | FR-290 Tool absence; FR-331 `results`; FR-272 `unsupported_projection`/`tool-unavailable`; TC-271 TA-01..TA-03 |
| FND-008 | low | FR-290 fixes the request extent as `bounded` or `unbounded` and adds a mode table keyed on those two values. ADR-012 §1.1 lists finite, bounded and unbounded domains, and the settled ruling gives the extent representation to QSL #222. As written, a finite (enumerable) extent has no spelling and no row, and an extent value outside the two, or a missing extent, has no named refusal cause. Fix: state that the item's extent is #222's representation and that FR-290 reads only its bounded/unbounded classification of it. Route an absent or unclassifiable extent to one named refusal cause, for example `invalid_capability`/`unknown-extent`. | FR-290 Advertised mode; FR-331 `request`; QSL #222; ADR-012 §1.1 |
| FND-009 | low | FR-291, FR-292, FR-061 and FR-240 now allow only protocol-family members. The diff does not say how a carrier refuses a valid FR-290 kind from another family (for example `value-validity` in an FR-292 claim-capability set). It is not `unknown-kind`, because the label is a member, and no `invalid_capability` cause covers it. Fix: add a cause (for example `family-mismatch`, keeping the label and the carrier's permitted family), or state that FR-061-AC-17's "mismatched" refusal covers it and name its code. Add a TC-060 or TC-061 vector. | FR-291 claim kinds; FR-292 claim capabilities; FR-061-AC-17; FR-240 claim |
| FND-010 | low | The catalog meaning of `unsupported_projection` does not match its causes. FR-271's new meaning covers "no registered backend supports", unbounded extent and tool absence. It drops the case where the one candidate's negotiation arm does not discharge an obligation form (FR-349 SMT-01b), which FR-272 and native-diagnostics still send to `unsupported-requested-capability`. FR-271 tags only tool absence as revision `1-draft.5`, but `unbounded-extent` is also new in `1-draft.5`. Fix: add "an obligation form the selected candidate's negotiation arm does not discharge" to the FR-271 meaning, and put the `1-draft.5` tag on both new causes. | FR-271 `unsupported_projection`; FR-272; native-diagnostics 1-draft.5; FR-349-AC-2; TC-266 SMT-01b |

## Checklist coverage

| Checklist item | Subject in this diff | Outcome |
| --- | --- | --- |
| Extension points | Backend registration, `negotiate_*` arms, adapter probe | Policy defined for absence and ambiguity. Candidate-set trust, refusal scope and probe errors are open (FND-003, FND-006, FND-007). |
| Entity identity | Kind label, family, backend identity, descriptor, tool pin | Kind identity and serialization are defined. Backend key and order, the provider/backend split and the tool-pin source are open (FND-004, FND-005). The family name `finite-replay` equals the member label `finite-replay`. This is harmless because families never appear on the wire. |
| Evaluation purity | Registry candidate computation | Registry output is order-free and does not probe tools (ADR-012 §6). The diff states order-independence and no ambient state. No gap. |
| Topological robustness | Candidate sets and required-kind sets | Empty, one and many candidates are covered. Split coverage, duplicate registration and inconsistent sets are open (FND-002, FND-003, FND-005). |

## Round 2 dispositions

Evaluated against the uncommitted working tree on 5d3aa25. Paths are relative to the repository root.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | `spec/objects/protocol/FR-290-protocol-claim-kind.md:65-77`; `spec/objects/interfaces/FR-331-backend-provider-envelope.md:26,40,52`; FR-240:142-143; TC-271 CK-03 (TC-271:40) |
| FND-002 | resolved | FR-290:86-88: each item has exactly one kind, and nested expressions add no kind. FR-331:28 is singular. |
| FND-003 | resolved | `inconsistent-candidates` row, FR-290:175; native-diagnostics:97; TC-271 NG-13 (TC-271:57) |
| FND-004 | resolved | Registry-wide manifest with one descriptor per backend, FR-331:27. Tool lock equals the pin and is refused otherwise, FR-331:28,40 and FR-290:221-222. TA-05 (TC-271:64) |
| FND-005 | resolved | Unique identity, (identity, digest) candidate and bytewise order, FR-290:147-153; `duplicate-backend` cause; CK-04 (TC-271:41). No vector orders identities that differ only in case or length; NG-02 and NG-09 exercise the order. |
| FND-006 | resolved | Each cause has a scope, and registration causes are keyed by backend identity, FR-290:188-193; native-diagnostics:97 |
| FND-007 | resolved (post-round-2, SR-606) | low. Probe error and probe timeout map to `tool-unavailable`, FR-290:213-217, TA-03 (TC-271:62). No outcome is assigned to a tool that passes the probe and then changes or disappears before the run. Fix: one sentence in FR-290 Tool absence stating that this case records FR-331 `failed` with a typed cause. |
| FND-008 | resolved | #222 owns the representation and classification, and `absent-extent` covers a missing classification, FR-290:119-128; NG-14 (TC-271:58) |
| FND-009 | resolved | FR-061-AC-17 (FR-061:237); TC-060:21; TC-061:67-70 |
| FND-010 | resolved | FR-271:62 |
| FND-011 (new) | resolved (post-round-2, SR-606) | medium. Item-level settlement rules overlap, and no precedence is given. FR-290:127-128 unconditionally settles an item with no extent classification `invalid-request`/`absent-extent`. FR-290:170-178 settles every item "exactly as" the candidate table does. An item with no extent classification and an empty, inconsistent or multi-member candidate set therefore matches two rules with different dispositions (for example `invalid-request` versus warned `unsupported`). Fix: state one evaluation order in FR-290 (for example: unknown-backend, inconsistent-candidates, empty, ambiguous, absent-extent at the single-candidate arm, then the mode table), with the first applicable rule settling the item. |
| FND-012 (new) | resolved (post-round-2, SR-606) | low. FR-290:191-193 reports registration refusals "in candidate order of backend identity". A refused registration is never a candidate, and `duplicate-backend` shares its identity with the accepted registration. Fix: "ordered bytewise by backend identity, then by manifest digest". |

Round 2 verdict: ACCEPT WITH FINDINGS
