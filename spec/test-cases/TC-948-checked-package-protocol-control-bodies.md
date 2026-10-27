---
id: TC-948
title: "Checked-package protocol control and compensation vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-466
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-464
    type: verifies
---

# TC-948: Checked-package protocol control and compensation vectors

## Description

Schema and reader vectors for the thirteen `protocol`/`control` operators and
the `protocol`/`compensation` node: argument layouts, labels, joins,
progress, binder scope and the compensation checks. Scope: FR-466-AC-1 to
FR-466-AC-15.

## Test Procedure

1. Encode TC-946's base package. Its `run` tree is `Main` holding `Requested`,
   `Received`, `Charged`, `Applied`, `Shipments` (branches `east` holding
   `East` and `west` holding `West`, `join all [east, west]`), `Outcome`
   (cases `shipped` holding `Done` and `failed` holding `Failed`), `Poll`
   (`max 3`, body `Polled`, `exhausted` `Limit`) and `Finalized`; the clause
   names `finish Closed` and the template `Refund` (forward `Applied`, commit
   `Finalized`, `attempts 3 of M::RefundAttempt`).
2. Encode one admitted variant of each operator in the "Layouts" table.
3. Apply each vector of the other tables to the base package, recompute node
   and package identities after each edit, and run the full reader.

## Layouts

| Vector | Encoded | Expected |
| --- | --- | --- |
| L1 | the base package | admitted: every control has exactly its layout of FR-466 |
| L2 | an empty `sequence` | admitted; `children` is an empty Group |
| L3 | `parallel` with each policy: `all [east, west]`, `any [east]`, `quorum` with count 1 and 2 over `[east, west]`, `predicate` naming a `function` node over `[east, west]` | admitted |
| L4 | `parallel` with `memory` `sc`, `tso` and `ra`, and with none | admitted: four distinct node keys |
| L5 | `parallel` with `outstanding` `continue`, `cancel` and none | admitted: three distinct node keys |
| L6 | `attempt` with `ordering` each of `relaxed`, `acquire`, `release`, `acq_rel`, `seq_cst`, and none | admitted: six distinct node keys |
| L7 | `fence` with `ordering` each of `acquire`, `release`, `acq_rel`, `seq_cst` | admitted: four distinct node keys |
| L8 | `repeat` with `maximum` and `exhausted`; with `proof`; with both; with neither | admitted |
| L9 | `event` with and without a `compensation`; `commit`; `check` | admitted |
| L10 | `Requested`'s two top-level arguments swapped | refuses `malformed_wire` (binding names out of order) |
| L11 | a binding missing from `Charged`; a surplus binding on `Applied` | refuses `malformed_wire` |
| L12 | `fence` with `ordering: relaxed` | refuses `malformed_wire` |
| L13 | `attempt` `ordering: strong` | refuses `malformed_wire` |
| L14 | `parallel` `memory: pso` | refuses `malformed_wire` |
| L15 | `parallel` `outstanding: abort` | refuses `malformed_wire` |
| L16 | `parallel` `join.policy: majority` | refuses `malformed_wire` |
| L17 | a `join` with `policy: all` carrying a `count` | refuses `malformed_wire` |
| L18 | a `join` with `policy: quorum` and no `count`; with `policy: predicate` and no `predicate` | refuses `malformed_wire` |
| L19 | `Requested`'s member operator is `jump` | `invalid_package`/`operation-member-mismatch` |

## Sequence, choice and parallel

| Vector | Edit | Expected |
| --- | --- | --- |
| S1 | swap `Requested` and `Charged` in `Main`'s `children` | `Main`'s node key and the `package_id` change; the reader refuses `definition-cycle` for the `receive`/`send` edge (`Requested` and `Received`) |
| S2 | a child of `Main` that is a `role` node | `invalid_model_binding`/`malformed-declaration` at the target |
| S3 | a child of `Main` that is absent from `dependencies` | `missing_declaration`/`missing-name` at the target |
| CE1 | `Outcome` with one case | refuses `malformed_wire` (at least two cases) |
| CE2 | two cases labelled `shipped` | `ambiguous_declaration`/`ambiguous-name` at `Outcome`, naming the label |
| CE3 | the `shipped` guard dereferences a model object | `ill_typed`/`operator-ineligible` at the read |
| CE4 | the `shipped` guard is an integer term | `ill_typed`/`non-boolean-root` at `Outcome` |
| CE5 | `owner` names a channel | `invalid_model_binding`/`malformed-declaration` at the target |
| CE6 | `visible` empty, guards over constants and the capture `order` only | admitted |
| CE6b | `visible` empty, a guard reads `eastRes.ok` | `wrong_snapshot`/`wrong-observation`, `reason` `unlisted` |
| CE8 | `cases` listed `shipped`, `failed` (not in ascending label order) | `invalid_semantic_graph` |
| CE9 | `visible` entries out of digest order; a repeated entry | `invalid_semantic_graph` |
| CE10 | `contracts` out of digest order; `related` out of order; a repeated `related` entry | `invalid_semantic_graph` |
| CE11 | the same `eastRes.ok` node listed in `Outcome.visible` and `Poll.visible` | admitted (a shared expression node is not a duplicate declaration) |
| CE7 | the two guards overlap (`eastRes.ok` and `eastRes.ok or true`) | admitted: the reader does not re-prove exhaustiveness or exclusiveness |
| PA1 | `Shipments` with one branch | refuses `malformed_wire` (at least two branches) |
| PA2 | two branches labelled `east` | `ambiguous_declaration`/`ambiguous-name` at `Shipments`, naming the label |
| PA3 | `joined` is `[east, middle]` | `missing_declaration`/`missing-name` at `middle` |
| PA4 | `join all [east]` | `invalid_package`/`invalid-value` at `joined` |
| PA12 | `branches` listed `west`, `east` | `invalid_semantic_graph` (branches sorted by label) |
| PA13 | the same protocol declared with branches `west` then `east` in the source | one encoding, `east` then `west`: the same node keys as the base |
| PA5 | `joined` is `[west, east]` (branch order reversed) | `invalid_semantic_graph` |
| PA6 | `joined` is `[east, east, west]` | `invalid_semantic_graph` |
| PA7 | `quorum` with `count` 0, and with `count` 3 over two joined branches | `invalid_package`/`invalid-value` at `count` |
| PA8 | `any` with an empty `joined` | `invalid_package`/`invalid-value` at `joined` |
| PA9 | `predicate` naming a `role` node | `invalid_model_binding`/`malformed-declaration` at the target |
| PA10 | `join any [east]` and read `eastRes` after `Shipments` | `missing_declaration`/`missing-name` (no branch binder escapes a join that is not `all`) |
| PA11 | a branch body held by `east` also held by `west` | `invalid_semantic_graph` (held twice) |
| PA14 | `Shipments` joined `predicate` over `[east, west]`, its `function` node with two `boolean` parameters and a `boolean` result | admitted |
| PA15 | PA14 with a function of one parameter, then of three | `ill_typed`/`type-mismatch` at `predicate`, retaining expected and actual |
| PA16 | PA14 with the second parameter typed by an integer node | `ill_typed`/`type-mismatch` at `predicate` |
| PA17 | PA14 with an integer result type | `ill_typed`/`type-mismatch` at `predicate` |

## Repeat and await

| Vector | Edit | Expected |
| --- | --- | --- |
| RP1 | `Poll` with `maximum` 3 and `exhausted` | admitted |
| RP2 | `Poll` with `maximum` and no `exhausted` | refuses `malformed_wire` |
| RP3 | `Poll` with `exhausted` and no `maximum` | refuses `malformed_wire` |
| RP4 | `Poll` with `proof` of `invariant` only | refuses `malformed_wire` |
| RP5 | `Poll` with `maximum: 0` | admitted |
| RP6 | `Poll`'s guard is an integer term | `ill_typed`/`non-boolean-root` at `Poll` |
| RP7 | `Poll`'s guard reads model state | `ill_typed`/`operator-ineligible` at the read |
| RP8 | `Poll` body is a `check` | `unsupported_construct`/`declaration-form` at `Poll` |
| AW1 | `await ResponseWindow after Requested within [0,30] match Reply then ... timeout ...` with `Reply` a `receive` | admitted |
| AW2 | the awaited `event` is an `effect`; is an `event` control | admitted |
| AW3 | the awaited `event` is a `send` | `unsupported_construct`/`declaration-form` at the `await`, naming the form and the profile |
| AW4 | the awaited `event` is an `attempt` | `unsupported_construct`/`declaration-form` at the `await` |
| AW5 | `after` `kind: event` names a `check` | `invalid_model_binding`/`malformed-declaration` at the target |
| AW6 | `after` `kind: compensation` names `Refund` | admitted; the `await` reads `Refund`'s activation |
| AW7 | `after` `kind: compensation` names a control | `invalid_model_binding`/`malformed-declaration` at the target |
| AW8 | the anchor is a control that follows the `await` | `invalid_package`/`definition-cycle` naming the `await` and its anchor |
| AW9 | `within` is `{lower: 5, upper: 1}` | `ill_typed`/`type-mismatch` at the interval, member `within` |
| AW10 | `then` reads the matched event's binder | admitted |
| AW11 | `timeout` reads the matched event's binder | `missing_declaration`/`missing-name` at the `await` |
| AW12 | the `await` was written with no temporal profile in the lock | `missing_declaration`/`missing-selection` |

## Event controls

| Vector | Edit | Expected |
| --- | --- | --- |
| E1 | `Requested` and `Received` over `Charges`, binders typed `M::ChargeRequest` | admitted |
| E2 | `sent` has type `M::ChargeResponse` | `ill_typed`/`type-mismatch` at `Requested`, retaining expected and actual |
| E3 | `Received` receives `Requested` on another channel | `ill_typed`/`operator-ineligible` at `Received` |
| E4 | `Received.of` names `Charged` (an `attempt`) | `invalid_model_binding`/`malformed-declaration` at the target |
| E5 | `Received.channel` names a queue node | `invalid_model_binding`/`malformed-declaration` at the target |
| E6 | `Charged.contracts` lists `CanCharge` and `ChargePost` anchored at `charge` | admitted |
| E7 | `Charged.contracts` lists a clause anchored at `refund` | `invalid_model_binding`/`malformed-declaration` at the target |
| E8 | `Charged.contracts` lists an `invariant` clause | `invalid_model_binding`/`malformed-declaration` at the target |
| E9 | `Charged.contracts` lists a role node | `invalid_model_binding`/`malformed-declaration` at the target |
| E10 | `East.contracts` is `[]` | admitted |
| E11 | `Charged.operation` names a `state`/`frame` node | `invalid_model_binding`/`malformed-declaration` at the target |
| E12 | `Applied.of` names `Charged` | admitted |
| E13 | `Applied.of` names `Requested` | `invalid_model_binding`/`malformed-declaration` at the target |
| E14 | `related by ShipmentOwner(order, eastRes.shipment)` on `East`, with the relationship in the clause's list | admitted |
| E15 | `related` names a relationship the clause does not list | `missing_declaration`/`missing-name` at the target |
| E16 | `related` end term of a type other than the relationship's endpoint | `ill_typed`/`type-mismatch` at the end |
| E17 | `event Compensated by Payments for Refund` | admitted |
| E18 | the `event`'s `compensation` names a template of another protocol | `invalid_model_binding`/`malformed-declaration` at the target |
| E19 | `Finalized` is a `commit` by `Orders` | admitted |
| E20 | a constraint is an integer term | `ill_typed`/`non-boolean-root` at the slot |
| E21 | `Finalized`'s constraint reads `done` | admitted |
| E22 | `Requested`'s constraint reads `got` (a later control's binder) | `missing_declaration`/`missing-name` at `Requested` |

## Binder scope

| Vector | Edit | Expected |
| --- | --- | --- |
| BS1 | `Received` reads `sent` | admitted |
| BS2 | `East` reads `westRes` | `missing_declaration`/`missing-name` at `East` |
| BS3 | `Done` reads `eastRes` and `westRes` | admitted |
| BS4 | `Done` reads the binder of a step control added to case `failed` | `missing_declaration`/`missing-name` at `Done` |
| BS5 | `Finalized` reads `eastRes` | admitted |
| BS6 | `Finalized` reads `polled` | `missing_declaration`/`missing-name` at `Finalized` |
| BS7 | `Limit` reads `polled` | `missing_declaration`/`missing-name` at `Limit` |
| BS8 | `Polled` reads `charge` | admitted |
| BS9 | levels: `sent` is `size(In(Requested))` and `got` is `size(In(Received))`, one greater | admitted; a `got` of the same level as `sent` is `invalid_semantic_graph` |
| BS10 | a `repeat` and a `choice` guard read the capture `order` | admitted (binders-only: binders, captures and constants) |
| BS11 | a `repeat` and a `choice` guard read `view` | `ill_typed`/`operator-ineligible` at the read (`over` is model state) |

## Decision facts

| Vector | Edit | Expected |
| --- | --- | --- |
| DF1 | `Outcome` owned by `Fulfillment` deciding on the Boolean field `eastRes.ok`, listed individually in `visible`, with guards `eastRes.ok` and `not eastRes.ok` | admitted (`eastRes` is an `attempt` `by Fulfillment`) |
| DF2 | `Outcome` owned by `Payments` over the same `visible` | `wrong_snapshot`/`wrong-observation`, `reason` `other-role` |
| DF3 | a `choice` owned by `Payments` deciding on the Boolean field of `Received`'s `got` | admitted (`Payments` is `Charges`' `to` role) |
| DF4 | the same, owned by `Orders` | `wrong_snapshot`/`wrong-observation`, `reason` `other-role` |
| DF5 | a guard reads a field of `sent` (a `send` record) | `wrong_snapshot`/`wrong-observation`, `reason` `ineligible-kind` |
| DF6 | a guard reads a field of `paid` (an `effect` record); of the commit binder | each `wrong_snapshot`/`wrong-observation`, `reason` `ineligible-kind` |
| DF6b | a `choice` guard, a `choice` `visible` entry, a `repeat` guard and a `repeat` `visible` entry each read a field of the finish binder (not readable in a `run` slot) | each `missing_declaration`/`missing-name` at the read, `read` `stage`, before the decision-fact check; never `wrong_snapshot` (FR-466-AC-16) |
| DF7 | a guard reads `view.ready` (the protocol input) | `ill_typed`/`operator-ineligible` at the read, naming the guard (before the decision-fact table) |
| DF7b | a guard reads a field of the protocol activation binder `opened` | `wrong_snapshot`/`wrong-observation`, `reason` `ineligible-kind` |
| DF7d | a `choice` guard, a `choice` `visible` entry, a `repeat` guard and a `repeat` `visible` entry each read `view.ready` | each `ill_typed`/`operator-ineligible` at the read, naming the slot; never `wrong_snapshot` (FR-466-AC-16) |
| DF7e | one guard reads both `view.ready` and a field of the finish binder; one guard reads both a field of the finish binder and a field of `sent` | the first `ill_typed`/`operator-ineligible` at the `view` read; the second `missing_declaration`/`missing-name` at the finish read (FR-466-AC-16) |
| DF7f | a guard reads a field of a binder a compensation template declares | `missing_declaration`/`missing-name`, `read` `foreign` (FR-466-AC-16) |
| DF7g | a `choice` guard, a `choice` `visible` entry, a `repeat` guard and a `repeat` `visible` entry each read the protocol activation binder `opened` bare (no field projection); the same slots each read the `receive` binder `got` bare | each `ill_typed`/`operator-ineligible` at the read, naming the slot; never `wrong_snapshot`, and no `wrong_snapshot` detail without a field (FR-466-AC-14, FR-466-AC-16) |
| DF7c | a guard and a `visible` entry read the capture `order` | admitted without listing |
| DF8 | a guard reads `eastRes.ok` that no `visible` entry lists individually | `wrong_snapshot`/`wrong-observation`, `reason` `unlisted` |
| DF9 | `visible` lists the composite `eastRes.ok and westRes.ok` and a guard reads `eastRes.ok` | `wrong_snapshot`/`wrong-observation`, `reason` `unlisted` |
| DF10 | a `visible` entry reads `sent.ok` | `wrong_snapshot`/`wrong-observation` at the entry, `reason` `ineligible-kind` |
| DF11 | a guard reads the binder of a `repeat` body from the repeat's own guard | `missing_declaration`/`missing-name` (out-of-scope read, before the decision-fact check) |
| DF12 | a `repeat` owned by `Fulfillment` whose `visible` lists `eastRes.ok` and whose guard reads it | admitted |
| DF13 | an `event` binder `by Payments`, read by a choice owned by `Payments`, and by one owned by `Orders` | admitted; `wrong_snapshot`/`wrong-observation`, `reason` `other-role` |

## Compensation vectors

| Vector | Edit | Expected |
| --- | --- | --- |
| CP1 | the base `Refund`: `commit Finalized`, `attempts 3 of M::RefundAttempt` | admitted; thirteen bindings in order |
| CP2 | `Refund` with `commit never` | admitted; `commit` is an empty optional; the node key differs from CP1's |
| CP3 | `forward` names `Charged` (an `attempt`) | `invalid_model_binding`/`malformed-declaration` at the target |
| CP4 | `forward_binder` has a type other than `Applied`'s own binder | `ill_typed`/`type-mismatch` at the template, retaining expected and actual |
| CP5 | `role` names a role of another protocol | `invalid_model_binding`/`malformed-declaration` at the target |
| CP6 | `operation` names a `state`/`state_clause` node | `invalid_model_binding`/`malformed-declaration` at the target |
| CP7 | `record_type` names a `scalar_type` node | `invalid_model_binding`/`malformed-declaration` at the target |
| CP8 | `commit` names `Charged` | `invalid_model_binding`/`malformed-declaration` at the target |
| CP9 | `Applied` is moved after `Outcome` and `commit` names an extra `commit` control placed before `Applied` in `Main` (on every path to it) | `unsupported_construct`/`declaration-form` at `commit`, retaining the selected profile and reason `commit-precedes-effect` |
| CP10 | `Applied` and a `commit` control are placed in different branches of a new `parallel`, and `commit` names that control | admitted (no edge orders it against `Applied`) |
| CP9b | `Applied` after a `parallel` with `join all`; `commit` names a `commit` control in one of its branches | `unsupported_construct`/`declaration-form` at `commit`, profile and reason `commit-precedes-effect` (`taken` of a `join all` is the union over its branches) |
| CP9c | as CP9b with `join any [east]` | admitted (`taken` is the intersection over the branches) |
| CP9d | `Applied` after a `choice` whose two cases hold separate `commit` controls `C1` and `C2`; `commit` names `C1` | admitted (`taken` is `{C1}` intersected with `{C2}`, empty) |
| CP9f | a `commit` control that is a sequence child before that `choice`, `Applied` after it | `unsupported_construct`/`declaration-form` at `commit`, reason `commit-precedes-effect` |
| CP9e | `Applied` and the `commit` in sibling branches of one `parallel` | admitted (unordered) |
| CP10b | `commit` names a `commit` control in one case of a `choice` that precedes `Applied` | admitted (it does not dominate the effect) |
| CP11 | `attempts.maximum` is `0` | `ill_typed`/`type-mismatch` at `maximum`, member `attempts`, expected a count of at least one, actual `0` |
| CP12 | `attempts.maximum` is `1` | admitted |
| CP13 | `within` is `{lower: 31, upper: 30}` | `ill_typed`/`type-mismatch` at the interval, member `within`, expected a closed interval with `lower <= upper` |
| CP14 | `earlier` has type `M::RefundAttempt`, `later` has type `M::Other` | `ill_typed`/`type-mismatch` at the template, retaining expected and actual |
| CP15 | `retry.relation` is an integer term | `ill_typed`/`non-boolean-root` at the slot |
| CP16 | the trigger guard reads `forward` | `missing_declaration`/`missing-name` at the template, stage `trigger-guard`, `read` `stage` |
| CP17 | the trigger guard reads `order` (a clause capture) and the trigger binder | admitted |
| CP18 | the activation capture reads the trigger and a registration capture | admitted |
| CP19 | an activation capture reads a later activation capture | `missing_declaration`/`missing-name` at the template |
| CP20 | a registration capture reads the trigger binder | `missing_declaration`/`missing-name` at the template, stage `registration-capture` |
| CP21 | the registration capture reads `forward` | admitted |
| CP22 | the retry relation reads `earlier`, `later` and a capture | admitted |
| CP23 | the retry relation reads `sent` | `missing_declaration`/`missing-name` at the template |
| CP24 | the recovery predicate reads its binder and a capture | admitted |
| CP25 | the recovery predicate reads `earlier` | `missing_declaration`/`missing-name` at the template |
| CP26 | two templates named `Refund` | `ambiguous_declaration`/`ambiguous-name` naming both node keys |
| CP27 | a template that no clause lists | `invalid_semantic_graph` at the template |
| CP28 | the lock selects no `temporal_profile` | `missing_declaration`/`missing-selection` at the template |
| CP29 | the trigger guard reads `view` (`over`) | `ill_typed`/`operator-ineligible` at the read, naming the guard |
| CP30 | the trigger guard reads the protocol activation binder `opened` | `missing_declaration`/`missing-name`, stage `trigger-guard`, `read` `stage` |
| CP31 | an activation capture reads `view` | admitted (an activation capture may read `over`) |
| CP32 | the retry relation reads `view` | `ill_typed`/`operator-ineligible` at the read, naming the retry relation |
| CP33 | the recovery predicate reads `view` | `ill_typed`/`operator-ineligible` at the read, naming the recovery predicate |
| CP34 | a registration capture reads `view`, the activation binder `opened` and a protocol capture | admitted |
| CP35 | the trigger guard holds a `quire.op.model.deref` | `ill_typed`/`operator-ineligible` at the read, naming the guard |
| CP36 | a registration capture reads a capture of another template `Other` | `missing_declaration`/`missing-name`, `read` `foreign` |
| CP37 | a registration capture reads itself | `missing_declaration`/`missing-name`, `read` `forward` |

## Expected Results

1. L1 to L9 are admitted with the keys stated; L10 to L18 refuse `malformed_wire`; L19 refuses `operation-member-mismatch` (FR-466-AC-1).
2. S1 changes the keys stated and refuses `definition-cycle`; S2 and S3
   refuse as listed (FR-466-AC-2).
3. CE1 to CE11 behave as listed; CE6, CE7 and CE11 are admitted
   (FR-466-AC-3).
4. PA1 to PA13 behave as listed (FR-466-AC-4).
5. RP1 to RP8 behave as listed (FR-466-AC-5, FR-464-AC-10).
6. AW1 to AW12 behave as listed (FR-466-AC-6).
7. E1 to E22 behave as listed (FR-466-AC-7, FR-466-AC-8, FR-466-AC-9).
8. BS1 to BS11 behave as listed (FR-466-AC-10).
9. CP1 to CP37 (with CP9b to CP9f and CP10b) behave as listed (FR-466-AC-11, FR-466-AC-12).
10. DF1 to DF13 (with DF6b and DF7b to DF7g) behave as listed (FR-466-AC-14, FR-466-AC-16).
11. Every admitted vector's node keys are unchanged by reformatting the source
    and by a second reference to the protocol, and change with any retargeted
    reference, renamed control, reordered child or changed literal
    (FR-466-AC-13).
12. PA14 to PA17 behave as listed (FR-466-AC-15).
Assertions compare typed fields, causes and pointers, never message text.
