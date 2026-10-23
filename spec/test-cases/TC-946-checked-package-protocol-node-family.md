---
id: TC-946
title: "Checked-package protocol node family: placement, keys, clause, membership, binders and causal vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-464
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-946: Checked-package protocol node family: placement, keys, clause, membership, binders and causal vectors

## Description

Schema and reader vectors for the `protocol` node family of
`quire.checked-semantic-graph/v2`: the shared `quire.op.protocol.control`
body, the operator catalog, node keys, declaration paths, the clause body,
membership and containment, binders and their scope, the derived causal
relation and progress, placement, profile fit, the reader order and the
activation relation. Scope: FR-464-AC-1 to FR-464-AC-13.

## Base package

Every vector edits this package. It is the choreography surface's order
protocol, trimmed to one of each construct that matters here.

Source: protocol `Example::OrderFlow` selecting the lock's `protocol_profile`
and `temporal_profile`; `over (view: M::OrderView)`, `activation on each
(opened: M::OrderStarted)`, capture `order` (`opened.order`); static roles
`Orders`, `Payments` and `Fulfillment`; relationship `ShipmentOwner =
M::OrderShipment`; channel `Charges` from `Orders` to `Payments` carrying
`M::ChargeRequest`, `fifo` by `message.order`, delivery `at-least-once`, with
queue capacity `literal 4`, overflow `reject`; `requires temporal
OrderClosureBounded`; `terminal any`; `scheduling adversarial`; and

```
run sequence Main {
  send Requested via Charges as (sent)        { sent.order = order };
  receive Received via Charges of Requested as (got) { got.order = order };
  attempt Charged by Payments on M::Payment::charge contracts [CanCharge,ChargePost] as (charge) { true };
  effect Applied of Charged as (paid) { true };
  parallel Shipments { branch east  attempt East  by Fulfillment on M::Shipment::dispatch contracts [] as (eastRes) { true };
                       branch west  attempt West  by Fulfillment on M::Shipment::dispatch contracts [] as (westRes) { true }; }
      join all [east, west];
  choice Outcome by Fulfillment visible (eastRes.ok) {
      case shipped when { eastRes.ok } check Done { true };
      case failed  when { not eastRes.ok } check Failed { true } }
  repeat Poll by Fulfillment visible (eastRes.ok) max 3 while { eastRes.ok }
      attempt Polled by Fulfillment on M::Shipment::poll contracts [] as (polled) { true }
      exhausted check Limit { true };
  commit Finalized by Orders as (done) { true };
}
finish Closed as (closed) { closed.order = order };
compensate Refund for Applied as (forward) by Payments on M::Payment::refund clock "refund-clock" { ...; commit Finalized; ... }
```

The encoded package holds one `protocol_clause` node (declaration
`Example::OrderFlow`), a `role` node for each role, one `channel` and one
`queue` node, a `terminal` and a `finish` `obligation` node, one `control`
node for every construct above (the `run` tree: `Main`, `Requested`,
`Received`, `Charged`, `Applied`, `Shipments`, `East`, `West`, `Outcome`,
`Done`, `Failed`, `Poll`, `Polled`, `Limit`, `Finalized`), one `compensation`
node and the `value`/`parameter` binder nodes. The binders have levels
`view` 0, `opened` 1, `order` 2; `sent` is `size(In(Requested))` = 3; and so on.

## Test Procedure

1. Encode the base package as a V2 package. Recompile it after reformatting
   the source, after adding a comment, and with a second `protocol
   Example::OrderFlow` reference in a user expression (K1).
2. Apply each vector of "Placement and operators", "Keys and declarations",
   "Clause body", "Membership", "Binders and scope", "Causal relation and
   progress" and "Activation relation" to the base package and run the full reader; recompute node and
   package identities after each edit unless the vector says the stale key is
   the point.
3. Feed a package with a state-step defect and a protocol defect, and one with
   a protocol placement defect and a member-node defect, and one with a
   protocol defect and an operation-step defect.

## Placement and operators

| Vector | Edit to the base | Expected |
| --- | --- | --- |
| V1 | none | admitted; every body is a `quire.op.protocol.control` application with one `protocol_profile` law equal to the lock's row, a `profile_operator` member, the Boolean `result_type` and `semantic_type` |
| V2 | `Requested`'s body applies `quire.op.state.clause` | `ill_typed`/`operator-ineligible` at the node (a state clause application at a protocol node's root) |
| V3 | the `Orders` role node's operator is `channel` | `ill_typed`/`operator-ineligible` at the node (operator of another form) |
| V4 | the `Orders` node's `semantic_form` is `control` with the `role` operator | `ill_typed`/`operator-ineligible` at the node |
| V5 | a `quire.op.protocol.control` application nested as an argument of `Main` | `malformed_wire` at the nested application (strict wire validation) |
| V6 | a `protocol_reference` carrier's body is a `quire.op.protocol.control` application | refuses `malformed_wire` (`malformed_wire`) |
| V7 | `Requested`'s member operator is `jump` | `invalid_package`/`operation-member-mismatch` at the application |
| V8 | `Requested`'s member is `null` | `invalid_package`/`operation-member-mismatch` |
| V9 | `Requested`'s `laws` is empty | `invalid_package`/`operation-law-missing` |
| V10 | `Requested`'s law names the package's `temporal_profile` definition | `invalid_package`/`operation-law-mismatch` |
| V11 | the lock has no `protocol_profile` row | `invalid_package`/`operation-law-unselected` |
| V12 | `Requested` has two laws | refuses `malformed_wire` (`malformed_wire`) |
| V13 | `Requested`'s `result_type` and `semantic_type` are the integer node | `ill_typed`/`operator-ineligible` at the operation step (a clause result is Boolean) |
| V14 | a state clause's condition references `Requested` | `ill_typed`/`operator-ineligible` at the state clause node |
| V15 | a diagnostic's `details` references a `role` node | `ill_typed`/`operator-ineligible` at `/diagnostics/entries/{e}/details/{d}` |
| V16 | the lock selects no `temporal_profile` (the base's `Refund` template reads a clock role) | `missing_declaration`/`missing-selection` at the `Refund` node |

## Keys and declarations

| Vector | Edit to the base | Expected |
| --- | --- | --- |
| K1 | reformat the source; add a comment; add a second source reference to the protocol; declare the roles in another source order | every node key and the `package_id` unchanged; the carrier gains one `expression` occurrence |
| K2 | rename role `Fulfillment` to `Shipping` | the role's node key, every node that references it and the `package_id` change |
| K3 | swap `Orders` and `Payments` in the clause's `roles` set | `invalid_semantic_graph` at the clause node (a set out of ascending digest order); no role node key changes |
| K13 | list `Outcome.cases` with `shipped` before `failed`'s sorted position; and, separately, swap two independent `children` of `Finalized`'s holder | the unsorted `cases` is `invalid_semantic_graph` at `Outcome`; swapping two `children` of a `sequence` changes the sequence's key and the `package_id` |
| K4 | change `max 3` to `max 4` | `Poll`'s node key, its ancestors' keys and the `package_id` change |
| K5 | retarget `Applied`'s `of` from `Charged` to `Requested` | `Applied`'s node key changes and the reader refuses `invalid_model_binding`/`malformed-declaration` at the target (not an `attempt`) |
| K6 | a role `Main`, a channel `Main`, a template `Main` and the root control `Main` | admitted: four distinct declarations (`role`, `channel`, `compensate`, `run` segments) |
| K7 | two roles named `Orders` | `ambiguous_declaration`/`ambiguous-name` naming both node keys |
| K8 | two controls on one path named `Done` | `ambiguous_declaration`/`ambiguous-name` naming both node keys |
| K9 | a node's retained key differs from the digest of its preimage | `invalid_package`/`stale-node-key` at the node, before any protocol check |
| K10 | a protocol node carries a `recursion_group` | refuses `malformed_wire` (`malformed_wire`) |
| K11 | a `queue` node carries a `declaration` | refuses `malformed_wire` (a `generated`-only node carries no declaration) |
| K12 | a `role` node has no `declaration` occurrence | refuses `malformed_wire` (`declaration` is present exactly with a `declaration` occurrence) |

## Clause body

| Vector | Edit to the base | Expected |
| --- | --- | --- |
| C1 | none | admitted: thirteen bindings `over`, `activation`, `captures`, `roles`, `relationships`, `channels`, `queues`, `requirements`, `compensations`, `terminal`, `scheduling`, `run`, `finish` in that order |
| C2 | `activation on origin` | `activation` is `{kind: origin}`; admitted |
| C3 | a protocol with no relationship, requirement, compensation, `terminal` or `scheduling` | empty lists and absent optionals; admitted |
| C4 | `scheduling` removed from the base | the clause node key and the `package_id` change |
| C5 | `roles` and `channels` swapped in the body | refuses `malformed_wire` (binding names out of order) |
| C6 | the `finish` binding missing | refuses `malformed_wire` |
| C7 | `over` is a `literal` | refuses `malformed_wire` |
| C8 | `activation.kind` is `always` | refuses `malformed_wire` |
| C9 | `activation` is `origin` and carries `trigger` | refuses `malformed_wire` |
| C10 | `requirements` names a `state`/`state_clause` node | `invalid_model_binding`/`malformed-declaration` at the target |
| C11 | `relationships` names a `protocol`/`role` node | `invalid_model_binding`/`wrong-export` at the target, retaining required kind `relationship`, the actual kind and both keys |
| C12 | `run` names a `role` node | `invalid_model_binding`/`malformed-declaration` at the target |
| C13 | `compensations` names a node that is not in the clause's `dependencies` | `missing_declaration`/`missing-name` at the target |

## Membership

| Vector | Edit to the base | Expected |
| --- | --- | --- |
| M1 | `Orders` listed in `roles` twice | `invalid_semantic_graph` at the clause node |
| M2 | an extra `role` node `Auditor` that no clause lists | `invalid_semantic_graph` at the node (unattached) |
| M3 | an extra `queue` for `Charges` | `invalid_semantic_graph` (a channel with two queues) |
| M4 | the `queue` removed | `invalid_semantic_graph` (a channel with no queue) |
| M5 | the `queue` names a channel of another clause | `invalid_semantic_graph` at the queue node |
| M6 | `Shipments`' `branches` names `Polled` as `east`'s body, while `Poll` also holds it | `invalid_semantic_graph` at `Polled` (held twice) |
| M7 | a control `Stray` that no control holds and `run` does not root | `invalid_semantic_graph` at `Stray` |
| M8 | `Done`'s declaration is `OrderFlow, run, Done` | `invalid_semantic_graph` at `Done` (declaration is not its holder's path followed by its name) |
| M9 | `Applied`'s declaration begins with another protocol's name | `invalid_semantic_graph` at `Applied` (declaration prefix) |
| M10 | a second `finish` node not named by any clause | `invalid_semantic_graph` at the node |

## Binders and scope

| Vector | Edit to the base | Expected |
| --- | --- | --- |
| B1 | none | `view` level 0, `opened` 1, `order` 2, `sent` 3, `got` 4, `charge` 5, `paid` 6 |
| B2 | `opened`'s `level` is 0 | `invalid_semantic_graph` at the binder (level is the entry count) |
| B3 | `got`'s `body` has a third binding | refuses `malformed_wire` (the FR-341 binder body is exactly `name`, `level`) |
| B4 | `got`'s occurrence has role `declaration` | refuses `malformed_wire` (binder occurrences are `expression`) |
| B5 | two binders named `order` readable at one place | `ambiguous_declaration`/`ambiguous-name` at the second binder's introducing node |
| B6 | the activation guard references capture `order` | `missing_declaration`/`missing-name` at the clause node, `stage` `protocol-activation`, `read` `stage` (the guard reads the activation binder only) |
| B20 | the activation guard references `view` (`over`) | `ill_typed`/`operator-ineligible` at the read, naming the guard (`over` is model state; the refusal precedes the stage table) |
| B21 | the `Poll` guard references `view` | `ill_typed`/`operator-ineligible` at the read, naming the guard |
| B22 | the `Poll` guard references the capture `order` | admitted (a guard may read a protocol capture without listing it) |
| B23 | a protocol capture initializer reads `view` and the activation binder | admitted |
| B24 | a `check` condition reads `view` | `ill_typed`/`operator-ineligible` at the read |
| B25 | an event constraint and a `related` end read `view` | admitted |
| B26 | the `Poll` guard is an integer term | `ill_typed`/`non-boolean-root` at the guard |
| B7 | the capture initializer references a later capture | `missing_declaration`/`missing-name` at the clause node, `read` `forward` |
| B8 | `Received`'s constraint reads `sent` | admitted (a successor reads its predecessor's binder) |
| B9 | `East`'s constraint reads `westRes` | `missing_declaration`/`missing-name` at `East` (sibling branches read nothing of each other) |
| B10 | `Done`'s `check` reads `eastRes` | admitted (`join all` exposes the branches' binders after `Shipments`) |
| B11 | `Limit`'s `check` reads `polled` | `missing_declaration`/`missing-name` at `Limit` (an iteration binder does not escape) |
| B12 | `Finalized`'s constraint reads `eastRes` | admitted (`Out(Outcome)` keeps the joined branches' binders) |
| B13 | `Finalized`'s constraint reads `polled` | `missing_declaration`/`missing-name` at `Finalized` |
| B14 | `Outcome`'s case `shipped` guard `{ eastRes.ok }` is replaced by `{ isTrue(M::Order(view.id).open) }` (a `quire.op.model.deref`) | `ill_typed`/`operator-ineligible` at the guard's read |
| B15 | the `Poll` guard holds `quire.op.state.pre` | `ill_typed`/`operator-ineligible` at the read |
| B16 | a `check` condition over a `quire.op.model.all_instances` | `ill_typed`/`operator-ineligible` at the read |
| B17 | `Requested`'s constraint (not binders-only) dereferences a model object | admitted |
| B18 | the `finish` constraint reads `eastRes` | admitted (`finish` reads `Out(root)`) |
| B19 | the `finish` constraint reads `polled` | `missing_declaration`/`missing-name` at the `finish` node |

## Causal relation and progress

| Vector | Edit to the base | Expected |
| --- | --- | --- |
| R1 | none | acyclic: `message` edge `Requested`.exit to `Received`.enter, `effect` edge `Charged` to `Applied`, `join` edges from `East` and `West` to `Shipments` |
| R2 | `Received` is sequenced before `Requested` (children swapped) while `Received` still receives `Requested` | `invalid_package`/`definition-cycle` naming `Requested` and `Received` |
| R3 | `Requested` on branch `east` and `Received` on branch `west` of one `parallel` | admitted |
| R4 | `Applied` sequenced before `Charged` | `invalid_package`/`definition-cycle` naming `Charged` and `Applied` |
| R5 | an `await` anchored at a control that follows the `await` | `invalid_package`/`definition-cycle` naming the `await` and its anchor |
| R6 | `Poll`'s back edge | not a cycle: admitted |
| R7 | `Poll`'s body is only a `check` | `unsupported_construct`/`declaration-form` at `Poll`, naming it and the profile |
| R8 | `Poll`'s body is a `choice` with an `attempt` in one case and an empty `sequence` in the other | `unsupported_construct`/`declaration-form` at `Poll` |
| R9 | `Poll`'s body is a `choice` with an `attempt` in one case and a nested `repeat` around an `attempt` in the other | `unsupported_construct`/`declaration-form` at the outer `repeat` |
| R10 | `Poll`'s body is one `await` with a `timeout` control, or one `parallel` with a `join` | admitted |
| R11 | `Poll` with a `maximum` only, a `proof` only, both and neither | admitted |
| R12 | `Poll` with `maximum: 0` | admitted |

## Reader order

| Vector | Edit to the base | Expected |
| --- | --- | --- |
| O1 | a `state`/`state_clause` defect and `V7` | the state-step defect is reported |
| O2 | `V2` (placement) and `K7` (duplicate declaration) | the placement defect is reported |
| O3 | `K7` and `M2` | the duplicate declaration is reported |
| O4 | `M2` and `B6` | the membership defect `M2` is reported |
| O5 | `R2` and `B9` | the cycle `R2` is reported (the causal relation precedes binder scope) |
| O6 | `V7` and an operation-step defect on another node | `invalid_package`/`operation-member-mismatch` (protocol step) is reported before the operation-step defect |
| O7 | `R2` (cycle) and `B14` (a model-state read in a guard) | the cycle `R2` is reported (cycle and progress precede the slot checks) |
| O8 | one guard that reads `view` and an unlisted fact `eastRes.ok` | `ill_typed`/`operator-ineligible` (the `over` refusal precedes the decision-fact check) |
| O9 | one guard that reads an out-of-scope binder and an unlisted fact | `missing_declaration`/`missing-name` (the scope refusal precedes the decision-fact check) |

## Expected Results

1. V1 is admitted, as are K1, K6, B1, B22, B23, B25, B8, B10, B12, B17, B18, R1, R3, R6 and
   R10 to R12; their keys and identities are as stated (FR-464-AC-1,
   FR-464-AC-3, FR-464-AC-5, FR-464-AC-9, FR-464-AC-10).
2. V2 to V6, V15 and the placement vectors refuse as listed; V7 to V13 refuse
   with the operation causes FR-322-AC-22 states (FR-464-AC-1, FR-464-AC-2,
   FR-464-AC-11).
3. K2 to K4 change exactly the keys stated; K7 to K12 refuse as listed
   (FR-464-AC-3, FR-464-AC-4).
4. C1 to C13 hold (FR-464-AC-5, FR-464-AC-6).
5. M1 to M10 refuse as listed (FR-464-AC-4, FR-464-AC-6).
6. B2 to B26 behave as listed (FR-464-AC-7, FR-464-AC-8).
7. R2, R4, R5 refuse `definition-cycle`; R7 to R9 refuse `declaration-form`;
   R1, R3, R6 and R10 to R12 are admitted (FR-464-AC-9, FR-464-AC-10).
8. O1 to O9 report the first defect in the reader order of FR-464
   (FR-464-AC-12).
Assertions compare typed fields, causes and pointers, never message text.

## Activation relation

Each vector derives the activation relation from the admitted package
(FR-464 "Activation relation") after the edit.

| Vector | Edit to the base | Expected |
| --- | --- | --- |
| A1 | none | one row: the `Example::OrderFlow` clause key, `each` with `opened`'s node key, and `OrderClosureBounded`'s node key |
| A2 | `activation on origin` (C2) | one row: the clause key, `origin` with no trigger, and `OrderClosureBounded`'s node key |
| A3 | the protocol requires no temporal clause (C3) | no row |
| A4 | a second `requires temporal OrderClosureBounded` | one `requirements` entry and one row, equal to A1's |
| A5 | a second temporal clause `OrderPaidBounded` and `requires temporal OrderPaidBounded` | two rows, in ascending node-id digest order of the two temporal clause nodes |
| A6 | a temporal clause `RefundBounded` on the clock `refund-clock`, which `Refund` reads, not required by the protocol | the relation equals A1's: no row for `RefundBounded` |
| A7 | K1's reformatting, comment and second protocol reference | the relation equals A1's |
| A8 | the `requirements` entry names a `temporal_clause` node absent from the clause's `dependencies` | `missing_declaration`/`missing-name` at the target; no relation |
| A9 | the `each` `trigger` names a `value`/`parameter` node absent from the clause's `dependencies` | `missing_declaration`/`missing-name` at the target; no relation |
