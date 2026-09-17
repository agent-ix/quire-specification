---
id: TC-235
title: "Business construct meaning binding"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-208
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-154
    type: verifies
---

# TC-235: Business construct meaning binding

## Description

Admit a domain package whose type definitions name each business model meaning of FR-208, and mutate each node against that meaning's definition.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-208 and FR-154. Execute each positive case and each explicit refusal mutation through the real local Rust boundary; compare the complete typed result with the criterion.

Select `quire.model.complete/v1` at revision `1-draft.1` by its exact
DefinitionRefs in
[`complete-model-lock.json`](../../proposals/quire-v1/definitions/complete-model-lock.json).
Node names, declaration keys, the source `S1` and the admission of a
re-digested package follow TC-234. Every type definition is declared by its own
artifact, and its identity is `ix://test/orders/<artifact id>`.

Fixture **K2** (domain package `test/orders`, version `1`). Its `constructs`
table has these entries, all under module `test/business`, each kind named
differently from its meaning: `thing` with
`quire.meaning.model.object-type/v1`; `money` with
`quire.meaning.model.record-value-type/v1`; `happened` with
`quire.meaning.model.event-type/v1`; `lifecycle` with
`quire.meaning.model.state-machine/v1`; `saga` with
`quire.meaning.model.process/v1`; `store` with
`quire.meaning.model.persistence-interface/v1`; and `context` with
`quire.meaning.model.namespace/v1`. Nodes:

- `Order` (kind `thing`): identity field `order_id` typed `UUID`, field `total`
  typed `Money`.
- `Money` (kind `money`): fields `amount_minor` typed `Int` and `currency`
  typed `String`, no identity field.
- `OrderPlaced` (kind `happened`): fields `occurred_at` typed `Timestamp` and
  `order_id` typed `UUID`, no identity field, `occurrenceField` naming
  `OrderPlaced/occurred_at`.
- `OrderLifecycle` (kind `lifecycle`): field `placed` typed `Boolean`;
  operations `place` and `cancel`; clause `NotYetPlaced` (`not self.placed`);
  states `Draft`, `Placed`, `Cancelled`; transitions
  `Draft → Placed` on `place`, guard `NotYetPlaced`, emits `[OrderPlaced]`;
  `Draft → Cancelled` on `cancel`; `Placed → Cancelled` on `cancel`.
- `Fulfilment` (kind `saga`): identity field `correlation_id` typed `UUID`;
  steps `receive_order` (`event`, consumes `[OrderPlaced]`), `reserve_stock`
  (`command`) and `await_shipment` (`wait`), in that order.
- `OrderStore` (kind `store`): operations `get` and `save`, no fields,
  `persists: [Order]`.
- `OrderManagement` (kind `context`): members `[Order, Money, OrderPlaced]`,
  vocabulary `[{term: Place, doc: "Commit a draft order"}]`.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| M01 | Admit K2 under S1 | admitted; `Money` binds as a record value type, `OrderPlaced` as an event type with occurrence field `OrderPlaced/occurred_at`, `OrderLifecycle` as a state machine with states `{Draft, Placed, Cancelled}` and exactly the three transitions, `Fulfilment` as a process with correlation key `[Fulfilment/correlation_id]` and steps `receive_order`, `reserve_stock`, `await_shipment` in that order, `OrderStore` as a persistence interface persisting `[Order]`, and `OrderManagement` as a namespace; no node binds a meaning from its kind name; `OrderLifecycle` has no initial state, `Fulfilment` no execution order and `OrderStore` no storage law; `OrderManagement` declares no type, so no effective type has its identity |
| M02 | K2 with `Money/currency` flagged as an identity field; then K2 with `OrderPlaced/order_id` flagged as an identity field; then K2 with `OrderPlaced.occurrenceField` naming `OrderPlaced/order_id` | each run is exactly one `refused { code: invalid_model_binding, cause: malformed-declaration }`, naming `ix://test/orders/Money`, `ix://test/orders/OrderPlaced` and `ix://test/orders/OrderPlaced` respectively, with the artifact and its span; no run exposes an effective view (FR-208-AC-4) |
| M03 | K2 with the `Draft → Placed` transition's `to` set to `Shipped`; then its `trigger` set to `ship`; then its `guard` set to `Unknown`; then its `emits` set to `[Order]` | the first three runs each refuse `missing_declaration`/`missing-name` naming `ix://test/orders/OrderLifecycle`, the absent state `Shipped`, operation `ship` and clause `Unknown` respectively; the fourth refuses `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/OrderLifecycle`, because `Order` is not an event type; each names the artifact and span, and no run exposes an effective view (FR-208-AC-5) |
| M04 | K2 with `Fulfilment/correlation_id` no longer an identity field; then K2 with a second step named `reserve_stock`; then K2 with step `receive_order` consuming `[Money]`; then K2 with step `reserve_stock` emitting `[StockReserved]`, which names no node | the first three runs each refuse `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/Fulfilment`; the fourth refuses `missing_declaration`/`missing-name` naming `ix://test/orders/Fulfilment` and absent node `ix://test/orders/StockReserved`; each names the artifact and span, and no run exposes an effective view (FR-208-AC-6) |
| M05 | K2 with field `OrderStore/cache` typed `Order`; then K2 with `OrderStore.operations` empty; then K2 with `OrderStore.persists` set to `[Money]` | each run refuses `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/OrderStore`, the artifact and span; no run exposes an effective view (FR-208-AC-7) |
| M06 | K2 with field `OrderManagement/name` typed `String`; then K2 with operation `OrderManagement/close`; then K2 plus namespace `Billing` (kind `context`) with members `[Order]`; then K2 plus namespace `Outer` (kind `context`) with members `[OrderManagement]`; then K2 with field `Order/context` typed `OrderManagement` | the first two runs refuse `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/OrderManagement`; the third names `ix://test/orders/OrderManagement`, the later of the two namespaces sharing member `Order` in node order; the fourth names `ix://test/orders/Outer`; the fifth names `ix://test/orders/Order/context`; each names the artifact and span, and no run exposes an effective view (FR-208-AC-8) |
| M07 | K2 with the `lifecycle` construct's `meaning` set to `quire.meaning.state-machine`; then K2 with the `saga` construct's `meaning` set to `quire.meaning.process` | the first run refuses `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/OrderLifecycle`, the second naming `ix://test/orders/Fulfilment`, each with member path `constructs[].meaning`, the artifact and span; no run binds a state machine or process from the kind name or the construct shape (FR-208-AC-3) |

## Expected Results

Every positive result matches the FR-208 definition of its node's construct meaning, each refusal names its node, artifact and span as FR-154 requires, and no meaning is taken from a kind name.
