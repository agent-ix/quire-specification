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

## Dispositions

All findings are fixed in the commit that follows this review.

- FND-001: FR-290 "Declaration-family applicability" now owns a closed vocabulary of seven declaration families. It is separate from the kind Families table. The families are `value`, `state-model`, `temporal-trace`, `relation`, `protocol`, `probabilistic` and `real-time`. The claim-form assignment table gains a "Declaration family" column, and each family's applicable kinds are exactly the kinds its rows assign. So `relation` admits `temporal-satisfaction` and `operation-contract`, and one declaration may hold two kinds. A declaration that holds no claim, such as a union (`SumCase`) declaration or an FR-353 abstraction relation, takes no label. New FR-290-AC-18 checks that the two tables agree. Following FR-290's own row, `finite-replay` applies to `state-model` (replay against state semantics) and to `protocol`.
- FND-002: the payload keys are `request_index`, `capability` (the kind label) and `family` (the declaration-family label). FR-290, the FR-272 row, native-diagnostics and FR-272-AC-27 name them. `request_index` and `family` join FR-272's canonical payload-name list, and `capability` is defined there for `invalid_capability`. TC-271 NG-21 and NG-24 use the keys.
- FND-003: the check is the first item rule of `negotiate_*` after kind admission. It reads the declaration family from the checked-package node that the item's `claim` names, so negotiation still happens at one point. The negotiation-order paragraph says so. The inapplicable item stays in the FR-331 request at its position, and its terminal record is its `invalid-request` disposition (`settled_by` `disposition`, no result entry). It precedes `absent-evidence-kind`, `unknown-evidence-kind`, `not-statistical`, `absent-extent` and the candidate table. FR-331's before-consumption refusals (`check`/`requirements` misuse, malformed `extent`, missing `candidates`) refuse the whole request first and are unchanged. TC-271 NG-25 covers the evidence-kind and method precedence.
- FND-004: `invalid-state-graph-forms`, which retains the backend identity, is added to the FR-272 and native-diagnostics `invalid_capability` rows and to FR-272-AC-27.
- FND-005: the bound is now its own EARS statement: "`negotiate_*` SHALL NOT refuse `inapplicable-kind` an item whose kind is the kind the claim-form assignment table assigns to its claim's form." The refusal statement's actor is also `negotiate_*`.
