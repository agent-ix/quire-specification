---
id: FR-061
title: "Report orthogonal protocol and adequacy results"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-006
    type: implements
  - target: ix://agent-ix/quire-specification/FR-031
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-059
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-060
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-091
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-094
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-231
    type: references
  - target: ix://agent-ix/quire-specification/FR-240
    type: references
  - target: ix://agent-ix/quire-specification/FR-241
    type: references
  - target: ix://agent-ix/quire-specification/FR-242
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-244
    type: references
  - target: ix://agent-ix/quire-specification/FR-245
    type: references
  - target: ix://agent-ix/quire-specification/FR-246
    type: references
  - target: ix://agent-ix/quire-specification/FR-247
    type: references
  - target: ix://agent-ix/quire-specification/FR-248
    type: references
  - target: ix://agent-ix/quire-specification/FR-280
    type: references
  - target: ix://agent-ix/quire-specification/FR-281
    type: references
  - target: ix://agent-ix/engineering-assurance/FR-022
    type: references
  - target: ix://agent-ix/quire-specification/FR-282
    type: references
  - target: ix://agent-ix/quire-specification/FR-284
    type: references
  - target: ix://agent-ix/quire-specification/FR-285
    type: references
  - target: ix://agent-ix/quire-specification/FR-286
    type: references
  - target: ix://agent-ix/quire-specification/FR-295
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-291
    type: references
  - target: ix://agent-ix/quire-specification/FR-292
    type: references
---
## Description

When any protocol assessment is requested, the result producer SHALL retain
separate assessment-execution, decision-scope progress, decision-scope closure,
surrounding-execution progress, surrounding-execution closure,
complete-global-conformance closure, truth, settlement-basis, activation,
participation, assumptions, completeness, observation-adequacy,
protocol-adequacy, claim and provenance facts for the exact requested subject.

## Inputs

An exact assessment request plus conformance or analysis observations,
decision-scope and surrounding-execution progress/closure facts, the
complete-global-conformance closure record when global conformance is requested,
prerequisite outcomes, limitations and derived artifacts.

## Outputs

A portable per-request result consumed by the composed-language dispatcher,
temporal/observation integrations and Engineering Assurance, or a typed strict-
reader refusal if the result is inconsistent.

## Behavior

The result producer SHALL NOT attach a successful truth value to a refused,
unsupported, failed or resource-incomplete execution. The result producer SHALL
NOT classify an inactive obligation as satisfied or demonstrated. If required
participants, observations, population members, progress or adverse cases are
missing, then the result producer SHALL NOT change an unsuccessful or unknown
truth, adequacy or package status to successful. The result producer SHALL retain
each claim strength from the Engineering Assurance FR-022 vocabulary as a
distinct source-bound value. The result producer SHALL reuse Quoin evidence authority.
The result producer SHALL NOT create a local evidence store or verification-method
registry.
The result producer SHALL emit every canonical protocol result with
`contractVersion` equal to `quire.protocol.result/v1-draft.1`, kind
`ProtocolResult` and a `resultId` derived exactly under FR-286. The identity
preimage is the versioned wrapper around the complete closed result with only
`resultId` omitted. A strict reader SHALL validate closed shape, version and
kind before recomputing the identity, and SHALL refuse a missing, stale or
cross-domain identity. Two accepted results bearing one identity SHALL have
byte-identical RFC 8785 canonical result bytes. A later contradiction emits a
new identity and explicit lineage; it never reuses the old identity for changed
bytes.
Assessment execution states whether the requested evaluator completed or why it
did not. Decision-scope progress and closure and surrounding-execution progress
and closure are four independent facts with exact scope, authority and boundary
identities. Complete-global-conformance closure is a fifth independent fact for
a global-conformance claim: it carries one state from `closed`, `open`,
`incomplete` or `contradicted` plus the exact required execution, branch and
workflow closure-premise identities. For another claim it is explicitly
`not-required` and carries no global premise set. Completeness states which
facts required by the returned claim are available and identifies every missing
dependency. The result producer SHALL NOT use completeness, assessment
execution or any other progress/closure fact as a substitute for any closure
axis. A strict reader SHALL refuse an omitted required axis,
cross-scope substitution, global premise under `not-required`, or global claim
without its complete-global-conformance closure record.

Each of the four decision-scope and surrounding-execution facts carries exactly
one state from `open` or `closed`. A progress fact is `closed` when its retained
progress point covers every required source, subject and valuation through the
scope's required boundary, and `open` otherwise. A closure fact is `closed` when
the scope's exact closure authority has closed it, and `open` otherwise. The
result producer SHALL retain the four states independently; a state read on one
axis never populates another, and a `closed` progress fact never closes its own
scope's closure fact.

That two-member vocabulary is not the complete-global-conformance vocabulary.
`incomplete`, `contradicted` and `not-required` are members of the fifth axis
and are never carried on one of the four. FR-295 independently uses
`incomplete` and `contradicted` in the separate completeness vocabulary; equal
label spelling does not make those axes substitutable. A contradicted retained
premise is reported by supersession as well as by the current completeness
fact. `not-started`,
`advanced`, `stalled` and `unobserved` are members of no axis in this
specification. An absent axis compares only to absence, is never defaulted to
`open` or to `closed`, and remains the omitted required axis a strict reader
refuses.

Every assumption SHALL retain its exact assumption identity and one state from
`met`, `not-met`, `unknown` or `not-assessed`. Observation adequacy states
whether the exact observations required to interpret the requested result are
`adequate`, `inadequate`, `unknown` or `not-assessed`. Protocol adequacy states
whether the exact declared obligation and case set is `demonstrated`,
`not-demonstrated`, `inapplicable` or `unknown`. The result producer SHALL NOT
use either adequacy domain as evidence for the other.

Completeness SHALL carry exactly one FR-295 state together with the exact
required population, boundary, authority assertion and per-fact status set. The
producer SHALL recompute the state from that complete set and refuse an omitted
member, caller-authored aggregate state, or a progress/closure label supplied
as completeness.

Participation SHALL carry exactly one FR-285 `known` or `unknown` value object.
A known fact retains the complete required-role-instance set and its participating
subset. An unknown fact retains every known participant plus every missing
dependency and typed reason that prevents knowing the required set; it SHALL
NOT invent a complete set. Collections use the selected FR-212 role-instance
identity and canonical order. If that key or order is unavailable, construction
or reordering SHALL refuse rather than use display names. E02 participation
labels are derived projection states and SHALL NOT be accepted as source values.

Every result SHALL carry exactly one FR-290 protocol-family claim kind. The result producer SHALL
attach exactly one source-bound FR-281 claim strength, one exact FR-291
admitted fragment and one exact FR-292 backend selection to every advanced
result. The claim must
belong to both selected capability sets. A result producer SHALL NOT infer any
of these values from a display name, invoke an unreviewed fallback, or transfer
premises or success between claim kinds.

Every truth result SHALL carry exactly one settlement basis from
`closed-scope`, `decisive-witness`, `decisive-counterexample`, `unsettled` or
`unavailable`, plus the exact supporting fact and progress identities. A truth
may settle while the subject remains open only when the selected profile proves
that the retained witness or counterexample preserves that truth under every
admitted continuation. Missing facts outside that exact decision-support set
remain visible in completeness and SHALL NOT force the producer to delay or
falsify the settled truth. A missing fact inside the decision-support set SHALL
make that truth unavailable.

Every retained decision-support record SHALL identify its exact oracle and
observation plus one dependence relation from `independent`, `shared`, `derived`
or `unknown`. A derived relation SHALL identify its exact predecessor; a shared
relation SHALL identify its dependence group. A strict reader SHALL refuse a
missing, cyclic or self-referential predecessor and SHALL NOT infer independence
from distinct display names or implementations.

When a later observation, progress or closure record contradicts a premise in a
settled result, the result producer SHALL retain the earlier result bytes and
emit a typed superseding or invalidating result that identifies the earlier
result, the contradicted premise and the corrected input identity. The result
producer SHALL NOT rewrite, reseal or silently replace the earlier result. A
premise contradiction is not an admitted continuation under the earlier input
identity.

The strict reader SHALL enforce the following result-combination rules.

### Result combination rules

| Condition | Required relation |
| --- | --- |
| Assessment execution is refused, unsupported, resource-incomplete or failed | Truth and settlement basis are unavailable. Already-established activation, decision-scope progress/closure, surrounding-execution progress/closure, complete-global-conformance closure and provenance facts remain retained. |
| Truth is satisfied | Assessment execution is completed, activation is triggered and the settlement basis is `closed-scope` or `decisive-witness`. Every fact in the exact decision-support set is complete. An open decision scope is valid only with a decisive witness whose truth is preserved by every admitted continuation; the surrounding execution may remain open after either basis settles. |
| Truth is violated | Assessment execution is completed, activation is triggered and the settlement basis is `closed-scope` or `decisive-counterexample`. Every fact in the exact decision-support set is complete. An open decision scope is valid only with a decisive counterexample whose truth is preserved by every admitted continuation; the surrounding execution may remain open after either basis settles. |
| Truth is pending | Assessment execution is completed, activation is triggered, decision-scope progress is open and the settlement basis is `unsettled`. Required observations are complete through the retained progress point, but admitted continuations do not yet preserve one Boolean truth. |
| A required decision-support fact is missing | Truth and settlement basis are unavailable for that decision. A truth already settled from a disjoint exact support set remains settled; completeness retains the missing fact and prevents dependent adequacy or package success. |
| A retained decision-support or closure premise is later contradicted | The earlier result bytes remain unchanged. A new result identifies the earlier result and contradiction, makes dependent package success unavailable and recomputes truth only under the corrected exact input identity. |
| Activation is inactive | Assessment execution is completed, truth is unavailable and the trigger scope is complete under an exact closure authority. The surrounding subject execution may remain open. |
| Activation is activation-unknown | Truth is unavailable; completeness retains its independently derived FR-295 state and missing/contradicted dependencies, and an open trigger scope remains a separate progress/closure fact. |
| Protocol adequacy is demonstrated | Every assessment execution and required input for the declared case set is complete and the exact required case set is present. Subject closure is required only when the declared adequacy claim depends on it. An obligation-path demonstration additionally requires triggered activation. Observation adequacy remains independently adequate. |
| Global-conformance claim is complete success | Complete-global-conformance closure is `closed`, its exact required execution, branch and workflow closure-premise set is complete, and every selected required request satisfies its claim-specific success rule. A closed decision scope, settled truth, completed assessment or closed surrounding execution cannot replace a missing global premise. Unsupported, open, incomplete or contradicted requests and closure premises remain represented and prevent complete success. |
| Claim is not global conformance | Complete-global-conformance closure is `not-required` with no global premise set. A supplied global premise or another closure axis relabelled `not-required` is refused. |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-061-AC-1 | Healthy, violating, inactive, pending, incomplete, refused, unsupported, failed and settled-open-subject cases remain discriminable without diagnostic prose, including independent assumption, observation-adequacy and protocol-adequacy facts. | Test (TC-061) |
| FR-061-AC-2 | Removing a trigger, required role, provider observation, population member or negative case cannot improve truth, adequacy or package-level status. | Test (TC-061) |
| FR-061-AC-3 | Every result binds its exact result-contract version and identity plus source/profile/model/protocol/request/binding/input/decision-scope progress/closure/surrounding-execution progress/closure/complete-global-conformance closure/backend identities, settlement basis and decision-support identities. | Test (TC-061) |
| FR-061-AC-4 | A result with an impossible dimension combination, stale identity or conflicting claim strength is refused by a strict reader. | Test (TC-061) |
| FR-061-AC-5 | One unsupported request does not remove an independent supported result, while package complete success remains unavailable. | Test (TC-061) |
| FR-061-AC-6 | Every row of the normative result-combination table admits its valid boundary case and refuses every one-axis forbidden combination. | Test (TC-061) |
| FR-061-AC-7 | Every decision-support record retains its exact oracle/observation identities and dependence relation; missing, self-referential or cyclic derived lineage and invented independence are refused. | Test (TC-061) |
| FR-061-AC-8 | A decisive temporal witness or counterexample may settle truth while the subject remains open only with completed assessment execution, exact complete decision support and proof that every admitted continuation preserves the result; missing support, a non-decisive basis or false closure refuses that combination. | Test (TC-061) |
| FR-061-AC-9 | A late contradiction to decision support, progress or closure preserves the earlier result bytes and emits a source-bound superseding or invalidating result; it cannot restamp or silently replace the earlier result. | Test (TC-061) |
| FR-061-AC-10 | Decision-scope progress, decision-scope closure, surrounding-execution progress, surrounding-execution closure and complete-global-conformance closure remain independently readable and scope-bound; independently omitting or cross-wiring each required axis or each required global premise refuses the record or prevents global success without changing an independently settled per-obligation truth. | Test (TC-061) |
| FR-061-AC-11 | Each of decision-scope progress, decision-scope closure, surrounding-execution progress and surrounding-execution closure carries exactly `open` or `closed`; `incomplete`, `contradicted`, `not-required`, `not-started`, `advanced`, `stalled`, `unobserved` or any other label on one of the four is refused. | Test (TC-061) |
| FR-061-AC-12 | An omitted one of the four axes is refused rather than defaulted to `open` or to `closed`, and compares only to absence. | Test (TC-061) |
| FR-061-AC-13 | A `closed` progress fact leaves its own scope's closure fact independently readable and does not close it, and a `closed` closure fact on one scope does not close the other scope. | Test (TC-061) |
| FR-061-AC-14 | The independently recomputed FR-286 identity accepts object-member presentation permutations, changes for every semantic member mutation, and refuses missing identity or the same identity on different canonical result bytes. | Test (TC-061) |
| FR-061-AC-15 | Participation accepts exactly the FR-285 known/unknown shapes, retains exact role-instance and missing-dependency references, and refuses target labels, mixed variants, duplicate members or display-derived ordering. | Test (TC-061) |
| FR-061-AC-16 | Completeness accepts exactly the FR-295 state derived from the complete required population and remains independent from all five progress/closure facts and assessment execution. | Test (TC-061) |
| FR-061-AC-17 | Each result retains one exact FR-290 protocol-family claim kind; every advanced result retains one compatible FR-281 strength, FR-291 fragment and FR-292 backend, and unknown/absent/mismatched or display-derived selections refuse; a non-protocol FR-290 member in the claim refuses as mismatched. | Test (TC-061) |

## Dependencies

- [Shared capability dispatcher](../foundation/FR-031-report-requested-capabilities.md).
- [FR-059](FR-059-assess-finite-global-conformance.md).
- [FR-060](FR-060-separate-protocol-claims.md).
- [FR-284](../../objects/protocol/FR-284-scope-progress-and-closure-state.md)
  types the two-member state vocabulary the four axes carry.
- [FR-285](../../objects/protocol/FR-285-participation-fact.md) types the source
  participation fact.
- [FR-286](../../objects/protocol/FR-286-canonical-protocol-result-identity-preimage.md)
  defines the result identity preimage and validation order.
- [FR-295](../../objects/observation/FR-295-input-completeness-state.md)
  defines the independent input-completeness state and its population derivation.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md),
  [FR-291](../../objects/protocol/FR-291-admitted-analysis-fragment.md), and
  [FR-292](../../objects/protocol/FR-292-analysis-backend.md) define claim
  dispatch and retained analysis authority.
- [Protocol contract](../../../proposals/quire-v1/protocol-contract.md).
