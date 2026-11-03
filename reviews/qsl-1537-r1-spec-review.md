---
id: SR-20261103145002
title: "QSL-1537 r1 spec review of PR #608 (FR-290 declaration-family applicability, invalid_capability/inapplicable-kind)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; PR #608 head c3904004 vs origin/main b1feb484; spec/objects/protocol/FR-290-protocol-claim-kind.md; spec/objects/foundation/FR-272-typed-diagnostic-cause.md; spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md; proposals/quire-v1/definitions/native-diagnostics.md; spec/test-cases/TC-271-capability-kind-candidate-negotiation.md; context: QSL FR-507, FR-057, ADR-012, ADR-020, FR-165, FR-173, FR-180 (quire-spec-language origin/main)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
  - target: ix://agent-ix/quire-specification/FR-271
    type: references
  - target: ix://agent-ix/quire-specification/TC-271
    type: references
---

## Summary

Ticket: QSL-1537. The PR adds the `invalid_capability` cause `inapplicable-kind`. FR-290 gets a new "Declaration-family applicability" section with one EARS statement and FR-290-AC-17. The negotiation-order paragraph and the item-level cause list now name the cause. FR-271, FR-272 (row and AC-27) and the native-diagnostics row add it, and TC-271 gains NG-21 to NG-24. This review covers the base review, the integrity sub-analysis and an EARS check of the one added SHALL statement.

Examined:
- FR-290 "Declaration-family applicability" section, lines 174-202 (examined)
- FR-290 negotiation-order paragraph, lines 372-378, and the item-level cause list, line 404 (examined)
- FR-290-AC-17 (examined)
- FR-272 `invalid_capability` row (line 100) and FR-272-AC-27 (examined)
- FR-271 `invalid_capability` row (examined)
- native-diagnostics `invalid_capability` row (examined)
- TC-271 NG-21 to NG-24 and the Expected Results edit (examined)
- FR-290 Families table and claim-form assignment table (context_only)
- FR-272 canonical cause payload contract (context_only)
- FR-331 item admission paragraph (line 521-529) and backend-provider schema `DiagnosticCause` with the FR-290 vectors (context_only)
- QSL FR-507, FR-057 kind applicability, ADR-012 §1, ADR-020 claim kinds (context_only)

## Verdict

**NOT MERGEABLE AS IS (one high, three medium, one low).** The cause is threaded consistently through FR-271, FR-272, native-diagnostics and FR-290's item-level cause list. The precedence the PR states is also consistent everywhere it appears: absent or unknown kind first, then `inapplicable-kind`, then `absent-extent`, then the candidate table. It agrees in the new Order bullet, the negotiation-order paragraph, FR-290-AC-17 and TC-271 NG-21 and NG-22. No other statement of the negotiation order exists: FR-331:524 lists wire-admitted causes, not an order. The added SHALL is a well-formed EARS unwanted-behaviour statement.

`quire validate` on the five touched files reports the same 14 findings on main and on the PR head. Only the line numbers move, by the two added frontmatter lines, so the PR adds no new finding.

The defect is the meaning of "declaration family". The PR defines it as a label from FR-290's Families table, but that table groups capability kinds, not declarations. A declaration that holds claims of two kinds, or a QSL `Relation` declaration, has no label the payload can carry (FND-001). The payload keys are also unnamed (FND-002). Where the producer-side settlement meets FR-331 is unstated (FND-003). The PR edits the `invalid_capability` row, which has been missing `invalid-state-graph-forms` since before this PR (FND-004).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new section defines a claim's declaration family as "the family of that declaration, written as one label of the Families table above". FR-290's Families table partitions the twelve kinds ("A family is a descriptive grouping of members"); it is not a declaration taxonomy. One declaration can hold claims of two kinds that the claim-form table assigns. A `relation` clause's model claim is `temporal-satisfaction` and its code claim is `operation-contract`. A model refinement's whole claim is `temporal-satisfaction` and its per-step obligations are `operation-contract`. No single Families label is "the family" of such a declaration. QSL's `Relation` declarations (ADR-012 §1, as ADR-020 §1 amends it) and its `SumCase` declarations have no FR-290 label at all, so the `inapplicable-kind` payload, "the claim's declaration family … as its exact FR-290 label", cannot be written for them. Two producers would pick different labels, and FR-272-AC-2 equality would then depend on that choice. Fix: define the declaration-family label set the payload carries, either a closed set owned by FR-290 that includes the relation/refinement declaration, or the producer's own family label with its spelling rule. Stop deriving it from the kind-grouping table. | spec/objects/protocol/FR-290-protocol-claim-kind.md:69-80, :176-186; spec/objects/foundation/FR-272-typed-diagnostic-cause.md:100 |
| FND-002 | medium | The `inapplicable-kind` payload names its three values (request index, kind, declaration family) but no payload keys. FR-272's canonical payload names reserve `kind` for "a limit's kind as one exact ASCII string", and the list has no name for a family. Implementers will choose different keys, for example `kind`/`capability` and `family`/`declaration_family`. Payload-keyed equality (FR-272-AC-2, TC-271 NG-24) and the backend-provider `DiagnosticCause.detail` object, which is "keyed by FR-272 payload names" and whose FR-290 vectors use `request_index` and `backend`, then disagree across producers. Fix: name the keys in FR-290, the FR-272 row and native-diagnostics, for example `request_index`, `capability` (already canonical) and a new canonical `family`. Add the new name to the canonical list and name the keys in FR-272-AC-27. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:100, :209-213; proposals/backend-provider-v1/schema.json:202-211 |
| FND-003 | medium | It is unclear where the refusal is settled and how it reaches FR-331. FR-290:372 still says "Capability negotiation happens at one point, `quire-contract-codegen`'s `negotiate_*`". The new section has "the producer" settle an item `invalid-request` before `negotiate_*` reads it, and the declaration family is not an FR-331 item member, so `negotiate_*` cannot run the check itself. Nothing says whether the inapplicable item is still written into the FR-331 request, and so into the request identity and backend input. Nothing says whether its terminal record is an FR-331 `dispositions` entry, which the schema describes as "one negotiation disposition per item". The precedence over the other item-level refusals is also unstated: `absent-evidence-kind`, `unknown-evidence-kind`, `not-statistical`, and the refuse-before-consumption rules for `check` and `requirements` on the wrong kind (FR-331:519-520). The Order bullet covers only extent, candidates and the named backend. Fix: amend the "one point" sentence. State whether the item is carried in the request and how its disposition is recorded. Extend the Order bullet to the evidence-kind, `not-statistical` and member-misuse refusals. | spec/objects/protocol/FR-290-protocol-claim-kind.md:174-202, :372-378; spec/objects/interfaces/FR-331-backend-provider-envelope.md:519-529 |
| FND-004 | medium | Confirmed gap, from before this PR, in a row this PR edits. The FR-272 `invalid_capability` row and the native-diagnostics row list 16 causes but not `invalid-state-graph-forms`. FR-290:404-409, FR-290-AC-16, FR-331-AC-42, the manifest schema and TC-968 SGF-05/06 all use that cause. FR-272-AC-1 requires the row to carry the family's exact closed set, so a reader built from FR-272 refuses a cause FR-290 requires. Fix in this PR: add `invalid-state-graph-forms`, which retains the backend identity, to both rows. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:100; proposals/quire-v1/definitions/native-diagnostics.md:95; spec/objects/protocol/FR-290-protocol-claim-kind.md:404-409 |
| FND-005 | low | EARS: the bound that the PR relies on to constrain every producer's table is the "Assigned kinds apply" bullet. It says such an item "applies, and is never refused". That is normative, but it has no SHALL and no actor, unlike the section's one EARS statement. Fix: state it as a SHALL on the producer, for example "A producer's applicability SHALL admit, for each claim form, the kind the claim-form assignment table assigns to it". | spec/objects/protocol/FR-290-protocol-claim-kind.md:193-195 |

## New findings (disposition pass 1)

Re-reviewed at head c362fbae. Main is now 4bfdfc2f. On the six touched files and the two committed review files, `quire validate` reports the same 15 findings on main and on the head, apart from line numbers, so the head adds no new finding.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | `finite-replay` now applies to `state-model` because of FR-290's "replay … against state semantics" half. No claim form, checked-package node or declaration in QSpec or QSL requests `finite-replay` against state semantics. Only FR-290 and TC-271 use the label, and QSL FR-057:206 says only a choreography is replayed. The applicability check works at family granularity, so this half's only effect is that a `finite-replay` item on any operation precondition, postcondition, invariant or frame claim passes the check and goes on to negotiation. That is why NG-21 had to switch to `global-conformance`, and why QSL FR-507-AC-2 and FR-057-AC-11 now contradict QSpec. Fix: narrow FR-290's replay row (and the `finite-replay` value description) to protocol semantics, and make `state-model` admit `operation-contract` only. | spec/objects/protocol/FR-290-protocol-claim-kind.md:58, :132, :186 |
| FND-007 | medium | "`negotiate_*` reads the claim's declaration family from the node … that the item's `claim` names" gives no mapping from a node to a family. The V2 graph tells claim forms apart by (tag, `semantic_form`), for example `state`/`state_clause`, `temporal`/`temporal_clause`, `claim`/`state_graph`, `claim`/`hyperproperty`, `claim`/`step_relation`, `claim`/`probabilistic_claim`, `claim`/`schedulability`, `relation`/`refinement` and the protocol nodes. Neither FR-290 nor FR-322 tables those pairs against the "Declaration family" column. A scalar-operation claim names an `expression` node, and that node's family depends on the enclosing function body, not on the node. An item whose `claim` names a node that is the claim of no row has no family and no stated outcome. The family label `relation` also collides with the V2 tag `relation`, which covers relationships and populations, while relation clauses are `claim`/`step_relation`. Fix: add a (tag, `semantic_form`) → family table, with the context rule for scalar-operation `expression` occurrences, and say what a claim node of no row settles. | spec/objects/protocol/FR-290-protocol-claim-kind.md:193-205; spec/objects/interfaces/FR-322-checked-package-artifact.md:183-188 |
| FND-008 | medium | The `item_kind` rule is incomplete. FR-331 and FR-368 give derived items `deadlock-freedom` (FR-366 for model subjects, FR-429 for protocol subjects), `time-lock-freedom` and `race-freedom` (FR-438, protocol subject), plus `refinement`. FR-290 maps only `time-lock-freedom` (to `temporal-trace`) and the FR-377 whole refinement (to `relation`). `deadlock-freedom` and `race-freedom` have no claim-form row and no family. Their anchor over a protocol subject is the protocol clause node, which reads as `protocol`, a family that does not admit `temporal-satisfaction`. Implementers could therefore refuse a derived item that FR-368 requires. Fix: add rows for `deadlock-freedom` and `race-freedom` (`temporal-satisfaction`, `temporal-trace`) and an explicit `item_kind` → family table. | spec/objects/protocol/FR-290-protocol-claim-kind.md:145, :187, :202-205; spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md:65-113 |
| FND-009 | low | The row "Claim transferred through a model refinement (FR-380)" now names `relation`, and TC-271 NG-23 requests it as an item. FR-380:51-56 says a transfer item is "a planner-local item … not a request item", so it never reaches `negotiate_*`, and the NG-23 vector cannot be built as a request. Fix: give the row family "none (planner-local, FR-380)" and drop it from NG-23, or state that FR-290's family applies only to request items. | spec/objects/protocol/FR-290-protocol-claim-kind.md:141, :188; spec/functional/analysis/FR-380-transfer-claims-through-refinement.md:51-56 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Closed seven-label declaration-family vocabulary, separate from the kind Families table; a "Declaration family" column on every claim-form row; `relation` admits both kinds; a declaration that holds no claim takes no label; FR-290-AC-18. Residuals are FND-006 to FND-008. |
| FND-002 | fixed | Keys `request_index`, `capability` and `family` are named in FR-290:209-211, the FR-272 row, native-diagnostics and AC-27; `request_index` and `family` join the canonical list, which also defines `capability` for `invalid_capability`. |
| FND-003 | fixed | The check is now `negotiate_*`'s first item rule after kind admission (FR-290:410-416). The Record bullet keeps the item in the request with an `invalid-request` disposition and `settled_by` `disposition`. The Order bullet covers the evidence-kind, `not-statistical`, `absent-extent` and before-consumption refusals. |
| FND-004 | fixed | `invalid-state-graph-forms`, which retains the backend identity, is in both rows and in AC-27. |
| FND-005 | fixed | FR-290:214-215 "`negotiate_*` SHALL NOT refuse `inapplicable-kind` an item whose kind is the kind the claim-form assignment table assigns to its claim's form." |
