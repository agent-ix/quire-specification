---
id: SR-15800
title: "spec-review of the checked-package state-graph claim node FR-475 and TC-960 (quire-specification PR 370)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@4af567777; proposals/checked-package-v2/schema.json; spec/objects/interfaces/FR-475-checked-package-state-graph-claim-body.md; spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/spec.md; spec/test-cases/TC-960-checked-package-state-graph-claim-node.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-475
    type: references
  - target: ix://agent-ix/quire-specification/TC-960
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---

# Spec review of the state-graph claim node (quire-specification PR 370)

## Summary

Ticket: STD-267 (QSpec half; paired with quire-spec-language PR 1242,
SR-15810). One spec-review pass over `origin/main...4af5677` (main
5ee0071 is an ancestor). Integrity, EARS, object and failure-domain checks
are folded into this pass. Oracles read at the PR head: FR-322 (declaration,
owner, dependencies, derived-shape key stage, refusal order), FR-341, FR-370,
FR-390, FR-393, FR-402, FR-451, FR-464, the checked-package-v2 schema
(`OwnerPresenceRule`, `DeclarationTagRules`) and positive fixtures, TC-331,
TC-332, TC-333; QSL FR-093 (literal lowering) and FR-165.

Gates run: `quire validate --scope . 'spec/**/*.md'` (quire 0.36.2, engine
0.50.2) gives 86 failure lines, the baseline, with no finding on FR-475 or
TC-960. `cargo test --test checked_package_v2 --test backend_provider_v1`
passes 23 + 119. QSL conformance (`qsl-package --lib checked_v2`) is not
needed for this PR: QSL reads only `fixtures/positive-*.json` from
`QSPEC_DIR`, never `schema.json`, and the PR changes no fixture or vector,
so the run is identical to main's by construction.

Examined, clean:

- The `subject`-free body (lead default). A state-graph clause names no
  model in source; the subject is the request item's `temporal.subject`
  (FR-331), as for a temporal clause (FR-370 has no subject member). The
  identity binding the subject, the claim, its form and its instance matches
  FR-393 "Envelope and identity" and QSL FR-169. FR-402 carries `subjects`
  only because hyper quantifiers name aliases in source, so the two are
  consistent.
- `always possible` without `from` sharing a node id with `from (true)`
  follows FR-390 (`Q` is `true`) and QSL FR-165. The idea is sound; see
  FND-002 for the node form of `true`.
- Predicates as references: a nested application inside an aggregate is
  outside FR-322's body grammar, so a reference is required. The node is
  structural (aggregate body, no application) and is keyed by the
  derived-shape stage.
- Schema enum: `state_graph` is the only addition and only `schema.json`
  enumerates claim forms; no README, preimage schema or fixture needs it.
- Ids FR-475 and TC-960 are unique on main and across the 13 other open
  PRs.
- No status or changelog wording, no compat layer.

Merge order: #370 first. #375 (FR-476) and #377 (FR-477) both conflict
textually with #370: the FR-322 dependencies-list line and both spec.md
hunks, and for #377 also the `ClaimNode` enum line. After #370 lands, each
merges main and resolves as the union: enum `["verification_claim",
"analysis_claim", "hyperproperty", "state_graph", "step_relation",
"synthesis_request"]`, list `FR-451, FR-475, FR-476, FR-477 and FR-464 to
FR-466`, both spec.md rows.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | [major] The state-graph step's place in the reader order is not fixed. It runs "after the state step and before the operation step". FR-370's temporal step and FR-451's abstraction step have the same placement, FR-464's protocol step follows state, temporal and abstraction, and the open #375 (terminal, after temporal) and #377 (step relation, after abstraction) add more. A package with a temporal clause whose profile law names `quire.temporal.unknown/v1` and a `state_graph` node with `kind` `eventually-possible` has no defined first refusal. Fix: place it against every named step, for example after the protocol step and before the operation step, and reconcile with FR-476 and FR-477. | spec/objects/interfaces/FR-475-checked-package-state-graph-claim-body.md:88-91 | wrong-requirement |
| FND-002 | high | [major] `from` and `target` must reference an `expression` node, and reader step 4 refuses any other target `ill_typed`/`operator-ineligible`. The constant `true` is a `value`/`literal` node (FR-322 dependencies table; QSL FR-093 lowers a Boolean `Literal` to `value`/`literal`). So the from-less `always possible`, `from (true)` (SN-05, AC-2) and `possible Anything using v { true }` (SN-04) either have no conforming encoding or are refused by step 4. Fix: admit any node whose `semantic_type` is `scalar_type`/`boolean` (an `expression` or a Boolean `value`/`literal` node), and keep the parameter refusal of SN-11 by naming `value`/`parameter` as the refused form. | spec/objects/interfaces/FR-475-checked-package-state-graph-claim-body.md:45-56, :102-105; spec/test-cases/TC-960-checked-package-state-graph-claim-node.md:33-34 | wrong-requirement |
| FND-003 | high | [major] `declaration` `{qualified_name}` together with "every occurrence has role `claim`" contradicts FR-322: `declaration` "is carried exactly by a ... `claim` node ... that has a `declaration` source occurrence". With a `declaration` and an aggregate body, the schema's `OwnerPresenceRule` also requires a `SourceOwner` `owner`, which FR-475 does not list. And because `declaration` enters the node identity preimage, two declarations can never check to one node, so "one per declaration that checks to this node" cannot happen. Decide one of two forms: no `declaration` (content-addressed like a temporal clause, so equal claims share a node and differ by `claim` ordinal, which is what QSL FR-337's triple assumes), or a `declaration` occurrence plus `owner`, with one node per declaration. | spec/objects/interfaces/FR-475-checked-package-state-graph-claim-body.md:58-62 | wrong-requirement |
| FND-004 | high | [major] SN-11 (and the matching clause of AC-4) cannot produce its expected result. When SN-01's `target` is changed to reference `c`'s parameter node, the body's targets become `{c}` while `dependencies` stays `{c, expr}`. The procedure recomputes only the node key, so FR-322's graph-shape refusal (`invalid_semantic_graph` at `/dependencies`, which FR-475 says precedes this step, the SN-12 result) fires before `ill_typed`/`operator-ineligible`. Fix: SN-11 also sets `dependencies` to the body's targets. | spec/test-cases/TC-960-checked-package-state-graph-claim-node.md:22-24, :40; spec/objects/interfaces/FR-475-checked-package-state-graph-claim-body.md:118 | wrong-requirement |
| FND-005 | low | [minor] Term-shape gaps in reader steps 3 and 4. An `over` that is neither a `reference` nor the empty aggregate (a text literal, say) matches no step. For `always-possible` or `unique-path`, a `from` that is a non-empty non-reference term passes the form fit and has no target for step 4. No vector covers `unique-path` with an empty `from`. Fix: in step 3, require `over` to be a reference or the empty aggregate and `from` to be a reference unless `kind` is `possible`, and add one vector. | spec/objects/interfaces/FR-475-checked-package-state-graph-claim-body.md:98-105 | correct-requirement-no-evidence |
| FND-006 | low | [minor] An `over` that does not target a `value`/`parameter` node refuses `ill_typed`/`operator-ineligible` here, while FR-370 refuses the same `over` defect on a temporal clause `missing_declaration`/`missing-name` or `invalid_model_binding`/`malformed-declaration`. One member with one meaning should have one cause, unless the difference is intended and stated. | spec/objects/interfaces/FR-475-checked-package-state-graph-claim-body.md:101-105 | wrong-requirement |
| FND-007 | low | [minor] "A predicate is a state predicate: it reads no operation anchor, result or pre-state" has no reader refusal and no AC. A wire predicate node that references a `state`/`operation_anchor` node is admitted. Either name the refusal and add a vector, or state that this is a checker rule (FR-390 Admission) that the reader does not re-check. | spec/objects/interfaces/FR-475-checked-package-state-graph-claim-body.md:52-56 | correct-requirement-no-evidence |
| FND-008 | low | [minor] The headline lead default (the subject is a request input) has no AC or vector. Nothing checks that one claim node requested over two subjects with different initial states or proof bounds keeps one node id and gets two obligation identities. AC-3 varies only kind, instance key and spelling. | spec/objects/interfaces/FR-475-checked-package-state-graph-claim-body.md:69-82, :117 | correct-requirement-no-evidence |
| FND-009 | low | [minor] Smaller items. The Kind locus "at the `kind` binding" gives no pointer, while Shape does: use `/semantic_graph/nodes/{n}/body/members/0`. SN-02 spells `x.phase != Lost` and `x.phase = Won`, but TC-332 AP-02 is `G::Phase::Lost` and `G::Phase::Won`: cite AP-02. SN-06 should say its two `ReachesTwo` claims are separate units, or FR-322's `ambiguous-name` applies. In FR-322's list, FR-475 sits out of numeric order before FR-464. | spec/objects/interfaces/FR-475-checked-package-state-graph-claim-body.md:96-97; spec/test-cases/TC-960-checked-package-state-graph-claim-node.md:31, :35; spec/objects/interfaces/FR-322-checked-package-artifact.md:553-554 | correct-requirement-no-evidence |

## Verdict

CHANGES REQUESTED. Four majors block merge. FND-002, FND-003 and FND-004
make AC-2 and AC-4 vectors unproducible or contradict FR-322. FND-001 leaves
the first-refusal order undefined across sibling steps. FND-003 needs a
planner decision (declared versus content-addressed claim node) that the
QSL half must follow. The subject-free body, the `from (true)` identity
rule, predicates as references, the schema enum change, id uniqueness and
both gates are clean. The QSL conformance run is not required for this
PR.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-010 | medium | [major] The fix for FND-002 swaps the allow-list for a deny-list. A predicate target may now be any node whose `semantic_type` is `scalar_type`/`boolean` other than `value`/`parameter`. Every `temporal`/`temporal_clause`, `state`/`state_clause` and `claim` node (this node form included) has that type, and so does a `function` node with a Boolean result, because a function's `semantic_type` is its result type (FR-322). SN-01 with `target` pointed at a temporal clause node of the same package, `dependencies` rewritten, passes step 4. No later step refuses it: the temporal step's placement check covers only `temporal`/`formula` and `temporal`/`fairness` references. That admits a temporal clause as a state predicate, which FR-390 forbids. Fix: allow exactly an `expression` node, or a `value`/`literal` node, of `semantic_type` `scalar_type`/`boolean`, and add a vector with a `temporal_clause` target. | spec/objects/interfaces/FR-475-checked-package-state-graph-claim-body.md:54-60, :116-122 | wrong-requirement |
| FND-011 | low | [minor] SN-16 names "§7.1's subject", but no §7.1 exists in QSpec; the section is QSL ADR-022 §7.1. Name the source, or name it through TC-331's fixture. | spec/test-cases/TC-960-checked-package-state-graph-claim-node.md:47 | correct-requirement-no-evidence |

## Dispositions

Round 1, reviewed at agent-ix/quire-specification@afe6d822 (fix commit 12b011fc plus the merge of main c1033008). Gates at the head: `quire validate` 86 failure lines (baseline); `cargo test --test checked_package_v2 --test backend_provider_v1` 23 + 119 passed. origin/main is an ancestor.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 12b011fc |
| FND-002 | fixed | 12b011fc (the replacement rule is too wide; see FND-010) |
| FND-003 | fixed | 12b011fc |
| FND-004 | fixed | 12b011fc |
| FND-005 | fixed | 12b011fc |
| FND-006 | fixed | 12b011fc |
| FND-007 | fixed | 12b011fc |
| FND-008 | fixed | 12b011fc |
| FND-009 | fixed | 12b011fc |
