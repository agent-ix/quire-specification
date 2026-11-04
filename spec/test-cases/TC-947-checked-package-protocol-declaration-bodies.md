---
id: TC-947
title: "Checked-package protocol role, channel, queue and obligation vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-465
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-464
    type: verifies
---

# TC-947: Checked-package protocol role, channel, queue and obligation vectors

## Description

Schema and reader vectors for the `protocol`/`role`, `channel`, `queue` and
`obligation` (`finish`, `terminal`) node bodies. Scope: FR-465-AC-1 to
FR-465-AC-12.

## Test Procedure

1. Encode TC-946's base package (protocol `Example::OrderFlow`). Its roles are
   `Orders`, `Payments` and `Fulfillment`; its channel `Charges` is `fifo` by
   `message.order` with delivery `at-least-once` and a `literal 4` queue with
   overflow `reject`; it has `finish Closed` and `terminal any`. Add a
   replicated role `w each M::Worker from workers max 2 lifetime
   until(w.done)` for the role vectors.
2. Apply each vector below to the base package and run the full reader.
3. Compile the base package with no workflow instance, delivery, clock
   observation or runtime binding supplied.

## Role vectors

| Vector | Edit | Expected |
| --- | --- | --- |
| RL1 | none: three static roles | admitted; `replication` is an empty Group |
| RL2 | a role on a `model`/`process`, `state_machine`, `systems_interface` and `persistence_interface` node, one each | admitted |
| RL3 | a role on a `model`/`record_value_type` node | `invalid_model_binding`/`malformed-declaration` at the target |
| RL4 | a role on a `model`/`event_type` node | `invalid_model_binding`/`malformed-declaration` at the target |
| RL5 | a role on a `model`/`variant_type` node | `invalid_model_binding`/`malformed-declaration` at the target |
| RL6 | `model` names a `value` node | `invalid_model_binding`/`malformed-declaration` at the target |
| RL7 | `model` names a node of the graph that is absent from `dependencies` | `invalid_semantic_graph` at `/semantic_graph/nodes/{i}/dependencies` of the role node (graph stage, FR-322-AC-96, before any body step) |
| RL7a | `model` names a local node id that is no node of the graph, listed in the node's `dependencies` (body target and dependency change together; the graph stage compares the list without resolving that target, FR-322-AC-96) | `missing_declaration`/`missing-name` at the target |
| RL8 | the replicated role with `lifetime` `workflow`, `scope` and `until` | admitted; `binder` is the `value`/`parameter` node named `w`, of type `Reference<M::Worker>` |
| RL9 | `until` present with `lifetime: workflow` | refuses `malformed_wire` |
| RL10 | `until` absent with `lifetime: until` | refuses `malformed_wire` |
| RL11 | `max` is `0` | `ill_typed`/`type-mismatch` at `max` |
| RL12 | `binder` has type `Reference<M::Other>` | `ill_typed`/`type-mismatch` at the role node, retaining expected and actual |
| RL13 | `binder` names a `role` node | `invalid_model_binding`/`malformed-declaration` at the target |
| RL14 | `until` reads the capture `order` | admitted (`C0` and the role's binder are readable) |
| RL15 | `until` reads `sent`, a binder of the `run` tree | `missing_declaration`/`missing-name` at the role node |
| RL16 | `until` is an integer term | `ill_typed`/`non-boolean-root` at the role node |
| RL17 | a role body with an `internality` binding | refuses `malformed_wire` |
| RL18 | `replication` holds only `population` | refuses `malformed_wire` |
| RL19 | `population` is an integer literal | refuses `malformed_wire` |
| RL20 | a static role whose `model` is the text `identity` of a `model_root` node of the package, with no `model` dependency | admitted as a role on a whole model; its node id differs from RL1's role on a model type of that selection (FR-465-AC-9) |
| RL21 | RL20's clause with a second role whose text is a second `model_root` node's `identity`; and RL20 in clauses whose `over` type is a scalar, a record and a `Set<M::Cell>`, one each | admitted; the two roles have different node ids; each `over` type is admitted |
| RL22 | RL20 with a text naming no `model_root` node | `missing_declaration`/`missing-selection` at the role node, retaining the text |
| RL23 | RL20 with a non-empty `replication` Group | `invalid_model_binding`/`malformed-declaration` at the role node |
| RL24 | `model` is an integer, a Boolean and a Group, one each | refuses `malformed_wire` |
| RL25 | RL22 with a non-empty `replication` Group: a text naming no `model_root` node and a replication | `missing_declaration`/`missing-selection` at the role node: the selection check precedes the replication check |
| RL26 | the architecture fixture's role `Outlet` on the Port `M::pump_out` and `Inlet` on `M::tank_in` | admitted; each carries `model_member` `{package, node}` of its own Port, the `model` reference targets `Sys`, and the two roles have different node ids (FR-465-AC-11) |
| RL27 | the role `Operator` on `M::Sys` and a role on the whole model `M` | admitted with no third argument; the bytes equal those of the same package without the member (FR-465-AC-11) |
| RL28 | RL26 with `package` naming another package; with `node` naming a component of `M`; with `node` naming no declaration | each `invalid_model_binding`/`malformed-declaration` at the role node, retaining `package` and `node` (FR-465-AC-12) |
| RL29 | RL26 with an empty `package` text; with an empty `node` text; with a third binding in the Group; with an empty Group | each `malformed_wire` (FR-465-AC-12) |
| RL30 | a `model_member` on a role whose `model` is a `model_root` text | `invalid_model_binding`/`malformed-declaration` at the role node (FR-465-AC-12) |

## Channel vectors

| Vector | Edit | Expected |
| --- | --- | --- |
| CH1 | none: `fifo` by `message.order`, `at-least-once` | admitted; five bindings `from`, `to`, `carries`, `ordering`, `delivery` in order |
| CH2 | `ordering` is `{kind: unordered}` | admitted |
| CH3 | `unordered` with a `binder` | refuses `malformed_wire` |
| CH4 | `fifo` with no `key` | refuses `malformed_wire` |
| CH5 | `fifo` `binder` has a type other than `carries` | `ill_typed`/`type-mismatch` at the channel node, retaining expected and actual |
| CH6 | `key` reads `sent` | `missing_declaration`/`missing-name` at the channel node, stage `channel-key` |
| CH7 | `key` reads the message binder's field only | admitted |
| CH7a | `key` reads the capture `order` | `missing_declaration`/`missing-name` at the channel node, stage `channel-key`, `read` `stage` |
| CH7e | `key` reads `view` (`over`) | `ill_typed`/`operator-ineligible` at the read, naming the key |
| CH7b | `key` holds `quire.op.model.deref` | `ill_typed`/`operator-ineligible` at the read, naming the key |
| CH7c | `key` has type `float64`; `Option<float64>`; a `model`/`event_type` | `ill_typed`/`operator-ineligible` at the channel node, retaining `=` and the type |
| CH7d | `key` has type `text`, `integer`, `uuid`, `enum`, `Reference<T>`, a record of integers | admitted: equality-stable (FR-322) |
| CH8 | `from` names the `Charges` queue node | `invalid_model_binding`/`malformed-declaration` at the target |
| CH9 | `from` names a role of another protocol | `invalid_model_binding`/`malformed-declaration` at the target |
| CH10 | `to` names a node of the graph that is absent from `dependencies` | `invalid_semantic_graph` at `/semantic_graph/nodes/{i}/dependencies` of the channel node (graph stage, FR-322-AC-96, before any body step) |
| CH10a | `to` names a local node id that is no node of the graph, listed in the node's `dependencies` (body target and dependency change together; the graph stage compares the list without resolving that target, FR-322-AC-96) | `missing_declaration`/`missing-name` at the target |
| CH11 | `carries` names a `model`/`record_value_type`, a `model`/`event_type`, a `model`/`variant_type`, a `composite_type` and a `scalar_type` node, one each | admitted |
| CH12 | `carries` names a `role` node | `invalid_model_binding`/`malformed-declaration` at the target |
| CH13 | `delivery` is each of `unknown`, `at-most-once`, `at-least-once` and `exactly-once-premise` | admitted: four distinct node ids |
| CH13a | `delivery` is each of `[0,3]`, `[0,0]`, `[2,2]` and `[0,18446744073709551615]` | admitted: four distinct channel bodies, each distinct from CH13's; the channel's node id, its declaration path, is unchanged |
| CH13b | `delivery` is `[0,3]`, then `[0,4]`; `delivery` is `[1,1]`, then `exactly-once-premise` | admitted; each pair gives two different channel bodies |
| CH14 | `delivery` is `exactly-once` | refuses `malformed_wire` |
| CH14a | `delivery` is each of `[4,3]`, `[0, 3]`, `[00,3]`, `[+0,3]`, `[-1,3]`, `[0.5,3]`, `[0,18446744073709551616]`, `[3]`, `[0,3` and `0,3` | refuses `malformed_wire` at the channel node pointer `/semantic_graph/nodes/{i}`: the V2 schema refuses a spelling that is not an interval shape (a space, a leading zero, a sign, a fraction, a missing bound, an unclosed or unopened bracket) at the channel node pointer `/semantic_graph/nodes/{i}`, and the reader's strict wire layout check refuses `lo > hi` and a numeral above 18446744073709551615 at the same pointer, because no JSON Schema keyword expresses either |
| CH15 | `delivery` removed | refuses `malformed_wire` |
| CH16 | `ordering` removed | refuses `malformed_wire` |
| CH17 | changing `delivery` from `at-least-once` to `unknown` | the channel's body, the clause's and the checked package change |

## Queue vectors

| Vector | Edit | Expected |
| --- | --- | --- |
| QU1 | none: `literal 4`, `reject` | admitted; occurrence role `generated`, no `declaration`, `dependencies` the channel only |
| QU2 | each of `reject`, `block`, `loss head`, `loss tail`, `loss new` with `literal` and with `symbolic(bound)` | admitted: ten distinct node ids (the selector enters the node id) |
| QU3 | `literal` capacity `0` | admitted |
| QU4 | `literal` capacity with a `name` binding | refuses `malformed_wire` |
| QU5 | `symbolic` capacity with a `value` binding | refuses `malformed_wire` |
| QU6 | `overflow` is `drop` | refuses `malformed_wire` |
| QU6a | `overflow` is `loss` with the `loss` member absent; with `middle` | refuses `malformed_wire` |
| QU6b | `overflow` is `reject` or `block` with a `loss` member | refuses `malformed_wire` |
| QU7 | `literal` capacity `-1` | `invalid_package`/`invalid-value` at the value, at strict wire validation |
| QU8 | `channel` names a `role` node | `invalid_model_binding`/`malformed-declaration` at the target |
| QU9 | `channel` names a node of the graph that is absent from `dependencies` | `invalid_semantic_graph` at `/semantic_graph/nodes/{i}/dependencies` of the queue node (graph stage, FR-322-AC-96, before any body step) |
| QU9a | `channel` names a local node id that is no node of the graph, listed in the node's `dependencies` (body target and dependency change together; the graph stage compares the list without resolving that target, FR-322-AC-96) | `missing_declaration`/`missing-name` at the target |
| QU11 | `at-least-once` delivery with overflow `loss` | admitted: overflow governs a full queue and delivery policy governs transport duplication and loss; an overflow loss is not a transport `lose` step |
| QU10 | the queue's occurrence has role `declaration` | refuses `malformed_wire` |

## Finish and terminal vectors

| Vector | Edit | Expected |
| --- | --- | --- |
| FN1 | none: `finish Closed as (closed) { closed.order = order }` | admitted; declaration `Example::OrderFlow`, `finish`, `Closed` |
| FN2 | the constraint is an integer term | `ill_typed`/`non-boolean-root` at the node |
| FN3 | the constraint reads `eastRes` | admitted |
| FN3b | the constraint reads `view` | admitted (`finish` may read `over`) |
| FN4 | the constraint reads `polled` | `missing_declaration`/`missing-name` at the node |
| FN5 | `binder` names an `expression` node | `invalid_model_binding`/`malformed-declaration` at the target |
| FN6 | the clause's `finish` names the `terminal` node | `invalid_model_binding`/`malformed-declaration` at the target |
| FN7 | a second `finish` node the clause does not name | `invalid_semantic_graph` at the node |
| TM1 | none: `terminal any` | admitted; `mode: any`, `condition` an empty optional; declaration `Example::OrderFlow`, `terminal` |
| TM2 | `terminal when { k.v = 3 }` | admitted; `condition` holds the Boolean term |
| TM3 | `mode: when` with no condition | refuses `malformed_wire` |
| TM4 | `mode: any` with a condition | refuses `malformed_wire` |
| TM5 | a second `terminal` node of the same clause | `ambiguous_declaration`/`ambiguous-name` naming both node ids |
| TM6 | the condition is an integer term | `ill_typed`/`non-boolean-root` at the node |
| TM7 | the condition reads `sent` | `missing_declaration`/`missing-name` at the node, stage `terminal` |
| TM10 | the condition reads the capture `order` of the `on each` base protocol | `missing_declaration`/`missing-name` at the node, stage `terminal` |
| TM13 | the same condition in a protocol activated `on origin` (no trigger, so no `opened` binder), whose capture `order` is initialized from a model constant instead of `opened.order` | admitted |
| TM12 | the condition reads the activation binder `opened` | `missing_declaration`/`missing-name` at the node, stage `terminal` |
| TM11 | the condition reads `view` (`over`, the model observation) | admitted |
| TM8 | the clause's `terminal` names the `finish` node | `invalid_model_binding`/`malformed-declaration` at the target |
| TM9 | `terminal` removed from the clause | admitted; the clause body changes (its terminal disposition), not its declared node id |

## Runtime-subject vectors

| Vector | Edit | Expected |
| --- | --- | --- |
| RT1 | compile the base with no workflow instance, delivery, clock observation or runtime binding | admitted; no node holds an instance, ordinal, delivery or observation |
| RT2 | a node body with an `instance` binding | refuses `malformed_wire` |
| RT3 | read the operators of the base package | the runtime subject kinds are `workflow-instance` (`protocol`), `role-instance` (`role`), `message` and `send` (`send`), `receive` and `delivery` (`receive`), `operation-attempt` (`attempt`), `business-effect` (`effect`), `commit` (`commit`), the compensation kinds (`compensate`) and the workflow-closure observation (`finish`) |
| RT4 | two sends of one protocol in two repeat iterations | one `send` node; the iteration ordinal is a runtime dimension, in no node |

## Expected Results

1. RL1, RL2, RL8, RL14, CH1, CH2, CH7, CH7d, CH11, CH13, CH13a, QU1 to QU3, QU11, FN1, FN3, FN3b, TM1, TM11,
   TM2, TM9, TM13, RT1 and RT4 are admitted with the stated keys (FR-465-AC-1 to
   FR-465-AC-8, FR-465-AC-10).
2. RL3 to RL7a, RL13, CH8 to CH10a, CH12, QU8, QU9a, FN5, FN6 and TM8 refuse
   `invalid_model_binding`/`malformed-declaration`, `missing_declaration`/
   `missing-name` at the target, or (RL7, CH10, QU9) `invalid_semantic_graph`
   at the node's `/dependencies` at the graph stage, as listed (FR-465-AC-1, FR-465-AC-3,
   FR-465-AC-5, FR-465-AC-6, FR-465-AC-7).
3. RL9, RL10, RL17 to RL19, CH3, CH4, CH14, CH14a, CH15, CH16, QU4 to QU6b, QU10, TM3, TM4
   and RT2 refuse `malformed_wire` (FR-465-AC-2, FR-465-AC-4,
   FR-465-AC-5, FR-465-AC-7, FR-465-AC-8, FR-465-AC-10).
4. QU7 refuses `invalid_package`/`invalid-value`; RL11, RL12, RL15, RL16, CH5, CH6, CH7a to CH7c, CH7e, FN2, FN4, TM6, TM7, TM10, TM12 and FN7 refuse as
   listed (FR-465-AC-2, FR-465-AC-3, FR-465-AC-6, FR-465-AC-7).
5. CH13b and CH17 change the bodies stated and no declared node id; TM5 refuses `ambiguous-name` (FR-465-AC-4,
   FR-465-AC-7, FR-465-AC-10).
6. RT3 holds (FR-465-AC-8).
7. RL20 and RL21 are admitted with distinct keys, each distinct from RL1's;
   RL22 and RL25 refuse `missing_declaration`/`missing-selection`, RL23
   `invalid_model_binding`/`malformed-declaration` at the role node, and RL24
   `malformed_wire` (FR-465-AC-9).
Assertions compare typed fields, causes and pointers, never message text.
