---
id: FR-492
title: "Planner settlement request, premise results and planner result"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-488
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-489
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-490
    type: references
  - target: ix://agent-ix/quire-specification/FR-491
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-354
    type: references
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
---

# FR-492: Planner settlement request, premise results and planner result

## Description

When the verification planner settles a planner-local implementation-guarantee
item (FR-488, FR-490) or assume-guarantee composition item (FR-489, FR-491),
it SHALL read one `quire.planner-settlement/v1` request that carries every
premise slot the item's declaration derives and the complete premise result
behind each filled slot, SHALL refuse a malformed, forged or mismatched
request before settlement, and SHALL write one planner result whose cause is
drawn from FR-331's closed inconclusive causes.

## Inputs

- The checked package that carries the declaration.
- A `quire.planner-settlement/v1` request (below).

## Outputs

A planner result (below), or an `invalid_runtime_input` refusal naming the
first defect's member by its RFC 6901 pointer.

## Behavior

### Request

```json
{
  "format": "quire.planner-settlement/v1",
  "package": "<checked package identity>",
  "item": {"role": "guarantee", "node": "<NodeId>"},
  "slots": [ {"role": "model-claim", "identity": {}, "result": {}} ],
  "guarantees": [ {} ],
  "discharges": [ {"slot": 3, "guarantee": 0} ],
  "limits": {"max_work": 100000}
}
```

- `package` — the identity of the checked package that carries the
  declaration; `item` — `role` `guarantee` (an FR-490 node) or
  `composition` (an FR-491 node with rule `assume-guarantee`), and the
  declaration node's id.
- `slots` — exactly the declaration's derived premise slots, in derived
  order (FR-490 "Derived items", FR-491 "Derived item and slots"). Each is
  `{role, identity, result?, record?}`: `role` the slot's closed role;
  `identity` the slot key the declaration derives; `result` a
  premise result, present exactly on a filled item-backed slot; `record` an
  environment record, present exactly on a filled `environment` slot. A slot
  the declaration derives with no item (an operation with no `progress`
  member, a required scheduling premise no `environment` member lists)
  carries neither, and is **unfilled**.
- A **slot key** is, for a slot backed by a request item, that derived
  item's FR-331 request identity without its limits and wall-clock members:
  its FR-201 obligation identity together with its subject, claim node,
  FR-290 kind, `item_kind`, fairness set, `stutter_closure`,
  `correspondence`, `premises` and extent classification. It is wider than
  the FR-201 obligation identity on purpose: two derived items that share an
  anchor node (the four `operation-contract` roles of one guarantee) or that
  differ only in fairness or `stutter_closure` have different slot keys. For
  any other slot it is its role and the members that produced it.
- A **premise result** is `{request, result}`: `request` the complete FR-331
  request item the result answers, with the request's `dependencies`, and
  `result` the complete FR-331 result entry, with its `counterexamples`
  entry when it is `refuted`. The planner reads both with FR-331's strict
  readers and reads the premise's subject, claim, fairness, `stutter_closure`,
  kind, extent, value, basis and certification from them, never from a
  summary or a reference; a result that only names another result is not a
  premise result. A premise result read from a request carries the
  certification FR-331 gives its entry, except that a `certified` entry is
  read as `uncertified`: the planner checks no certificate, so it never
  accepts `certified` on the request's word. A plugin's entry is `trusted`
  (FR-331-AC-23). A planner that runs in the same process as the producing
  core may pass the core's own records with their certification instead of
  bytes; such a record never comes from a request.
- An **environment record** is `{name, supplier, supplies}`, spelled as the
  declaration's environment entry (FR-490 "Checked-package node"). It is an
  assumption: it carries no certification and is never `trusted`.
- `guarantees` (composition only) — the pool of supplied component guarantee
  results, each a premise result; `discharges` (composition only, optional)
  — explicit pairs `{slot, guarantee}` naming a slot index and a
  `guarantees` index claimed to discharge it, or `{slot, environment}`
  naming a slot index and an `environment` slot index whose record is
  offered as its only discharger (a premise supplied only as an
  assumption).
- `limits` — `max_work`, the planner's work units (one per slot read, per
  pool entry compared and per fixed-point iteration checked), FR-461
  setting `planner.max_work`, defaulting to the planner's published
  default.

These two wire formats have no published JSON schema; this requirement's
prose is their authority.

The request identity covers every member, so a changed relation revision
(package identity), implementation source, premise result or environment
record is a different request, and a planner result is reused only for an
equal request (FR-488-AC-12).

### Admission

The planner SHALL refuse the request `invalid_runtime_input` with the FR-272
cause given, at the first defect in this order:

1. Not JSON, or not an object: `malformed-json`; a missing, unknown or
   duplicate member: `missing-member`, `unknown-member`, `duplicate-member`;
   a member of the wrong JSON kind, including a slot list, slot, result or
   pool carried as a Boolean success flag: `wrong-value-kind`.
2. `package` not the checked package's identity, `item.node` not a
   declaration node of `item.role`'s form, or a composition `item` whose
   rule is not `assume-guarantee`: `invalid-value` at the member.
3. `slots` not exactly the derived slots (count, order, `role` and
   `identity`): `invalid-value` at the first differing slot. A forged or
   reordered slot is this defect.
4. A filled item-backed slot with no `result`, an `environment` slot whose
   declaration member is listed but which has no `record`, or an unfilled
   slot carrying either: `invalid-value` at the slot (a premise identity
   that names no settled result, FR-488-AC-7, FR-488-AC-9).
5. A `result` that FR-331's request or result reader refuses: that refusal,
   at the slot. A `result` whose request item's slot key is not the slot's
   `identity`: `invalid-value` at `/slots/<i>/result`. The key binds the
   subject's package and so the relation revision, the fairness set,
   `stutter_closure` and the `correspondence` role, so a premise over
   another subject or relation revision, a model claim checked without its
   stutter closure or under another fairness set, and a frame result in a
   transition-simulation slot are each this defect.
6. A `record` that differs from its declaration member: `invalid-value` at
   the record.
7. A `discharges` entry whose slot is not a `rely` or
   `liveness-dependency` slot, whose `guarantee` names no pool entry or a
   pool entry whose request subject differs from the slot's premise
   subject, or whose `environment` names no filled `environment` slot:
   `invalid-value` at the entry (FR-489-AC-10).

A refusal settles no slot and writes no planner result.

### Slot outcome

Before any slot, a guarantee item whose clause `P` lies outside FR-380's
stuttering-invariant fragment (an interval operator nested under an
unbounded one, a previous or strong previous operator) settles
`inconclusive`, cause `transfer-not-preserving{reason: fragment,
operator}`, naming the first such operator in clause order: an allowed
stutter changes its truth. Otherwise each slot settles one outcome, in slot
order:

| Slot | Outcome |
| --- | --- |
| filled item-backed slot | its premise result's FR-331 value, certification and extent |
| `model-claim`, when its result entry names an assumption it was proved under that no filled `environment` record supplies | `inconclusive`, so the item settles `premise-unproved` naming the model-claim slot (FR-488 "Environment premises") |
| `fairness-correspondence` (FR-488) | settled by the planner: `proved`, `decisive-witness`, certification `certified`, when FR-488 "Fairness correspondence" holds; otherwise `inconclusive`, cause `transfer-not-preserving{reason: fairness, constraint}`, naming the constraint no supplied scheduling premise implies |
| `environment`, filled | `supplied`, listed in the result as an assumption |
| unfilled | `absent` |
| composition `rely` or `liveness-dependency` whose only discharger is a `{slot, environment}` entry | `assumption`: not discharged (FR-489-AC-11) |
| composition `rely`, `liveness-dependency`, `fairness` and `fixed-point` otherwise | settled by FR-489 "Discharge" and "Cycles" from the pool and `discharges` |

### Planner result

```json
{
  "format": "quire.planner-result/v1",
  "item": {"role": "guarantee", "node": "<NodeId>"},
  "value": "inconclusive",
  "basis": "unsettled",
  "cause": {"type": "premise-unproved", "premise": 3, "result": "absent"},
  "slots": [ {"slot": 0, "value": "proved", "certification": "certified", "extent": "unbounded"} ]
}
```

- `value` and `basis` — the FR-360 label and FR-243 basis of the item:
  `proved`/`decisive-witness`, `inconclusive`/`unsettled`,
  `unsupported`/`unavailable` or `incomplete`/`unavailable`; never
  `refuted`.
- `cause` — on `inconclusive`, `premise-unproved{premise, result}`, where
  `premise` is the slot index and `result` its outcome (an FR-331 result
  value, `absent` for an unfilled slot, or `assumption` for a premise
  supplied only as an assumption), or the fragment cause above; on
  `unsupported`, the deciding slot's unsupported cause; on `incomplete`, the
  deciding slot's incomplete cause, or `limit-reached` with `limit`
  `max_work`, its `value` and `setting` `planner.max_work` when the
  planner's own budget stops it.
- `certification` — on `proved` only: `trusted` when any deciding premise
  is `trusted`, else `uncertified` when any is, else `certified` (FR-354).
- `slots` — every slot's outcome, with its certification and extent.
- `conclusion` (a proved guarantee item) — FR-488's conclusion record: the
  relation (`package`, `relation` node id) and the implementation sources it
  binds (every `RustPath` of the relation and the declaration), `clause`
  and `F_P`, every slot's outcome, the environment records used as
  assumptions, the extent (`unbounded`, or each bounded premise's bound) and
  each premise's certification.
- `discharge` (a composition item) — FR-489's premise discharge record.

**Decision order.** The item's outcome is decided by the first slot, in
slot order, whose outcome is not `proved` or `supplied`: an `unsupported` outcome settles the item
`unsupported` with its cause; an `incomplete` outcome settles it
`incomplete` with its cause; any other outcome (`refuted`, `inconclusive`,
`tested`, `measured`, `declined`, `failed`, `absent` or `assumption`) settles it
`inconclusive`, `premise-unproved` naming that slot. Only when every needed
slot is `proved` or `supplied` is the item `proved`. A `tested` premise, a
finite trace, is never `proved` (FR-488-AC-11).

`premise-unproved` and `transfer-not-preserving` here are the FR-331
closed causes, written only by the planner: a provider's result entry never
carries them (FR-331 "Result entries").

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-492-AC-1 | A request whose slots equal the derived slots and each filled with a matching proved premise result settles the item `proved`, `decisive-witness`, with every slot outcome, and for a guarantee item the conclusion record. | Test (TC-986) |
| FR-492-AC-2 | A request with a slot removed, reordered or relabelled, and one whose `package` differs from the checked package, each refuse `invalid_runtime_input`/`invalid-value` at the member. | Test (TC-986) |
| FR-492-AC-3 | A slot list or slot result carried as a Boolean success flag refuses `invalid_runtime_input`/`wrong-value-kind` at the member. | Test (TC-986) |
| FR-492-AC-4 | A filled item-backed slot with no result, and a listed environment slot with no record, refuse `invalid_runtime_input`/`invalid-value` at the slot; an unfilled slot settles `absent` and the item `inconclusive`, `premise-unproved` naming it with `result` `absent`. | Test (TC-986) |
| FR-492-AC-5 | A premise result whose request item's slot key differs from the slot's (one over another relation revision, a model claim without `stutter_closure` or under another fairness set, and a frame result in a transition-simulation slot) refuses `invalid_runtime_input`/`invalid-value` at `/slots/<i>/result`; a result entry FR-331's reader refuses keeps that refusal at the slot. | Test (TC-986) |
| FR-492-AC-6 | With slot results `unsupported`, `incomplete` and `inconclusive` seeded one per run, the item settles `unsupported`, `incomplete` and `inconclusive`/`premise-unproved` respectively, each naming its slot; with two non-proved slots the earlier slot decides. | Test (TC-986) |
| FR-492-AC-7 | A slot result `tested` settles the item `inconclusive`, `premise-unproved` naming the slot; no planner result is `refuted`. | Test (TC-986) |
| FR-492-AC-8 | A proved item whose premises include one `trusted` plugin result is `proved` with certification `trusted`; with only `certified` entries read from the request it is `uncertified`, never `certified`. | Test (TC-986) |
| FR-492-AC-11 | A guarantee item whose clause nests an interval operator under an unbounded one, and one whose clause uses previous, each settle `inconclusive`, `transfer-not-preserving{reason: fragment}` naming the operator, with every slot proved. | Test (TC-986) |
| FR-492-AC-12 | A model-claim result naming an assumption no environment record supplies settles the item `inconclusive`, `premise-unproved` naming the model-claim slot; a `rely` slot whose only discharger is an environment record settles `premise-unproved` with `result` `assumption`. | Test (TC-986) |
| FR-492-AC-9 | A `max_work` budget smaller than the slot count settles the item `incomplete`, `limit-reached` naming `max_work`. | Test (TC-986) |
| FR-492-AC-10 | A provider result entry carrying `premise-unproved` or `transfer-not-preserving` is refused by FR-331's reader (`unknown-cause`), so no premise result carries a planner-only cause from a provider. | Test (TC-986) |

## Dependencies

- [FR-488](../../functional/ir-proof/FR-488-attach-a-temporal-guarantee-to-implementation-rust.md),
  [FR-489](../../functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md),
  [FR-490](../../functional/ir-proof/FR-490-spell-and-check-an-implementation-guarantee-declaration.md)
  and [FR-491](../../functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md):
  the items, slots and their settlement.
- [FR-331](FR-331-backend-provider-envelope.md): the request item, the
  result entry, their readers and the closed causes.
- [FR-354](../../functional/ir-proof/FR-354-record-proof-trust-basis.md),
  [FR-360](../temporal/FR-360-infinite-trace-result-disposition.md),
  [FR-243](../protocol/FR-243-settlement-basis.md),
  [FR-201](../foundation/FR-201-canonical-identity-domain.md) and
  [FR-272](../foundation/FR-272-typed-diagnostic-cause.md).

## References

- Tickets: Linear QSL-1541, QSL-1542.
