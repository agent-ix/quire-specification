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

Admit a domain package whose type definitions name each business model meaning of FR-208, and mutate each node against that meaning's rules and against its use by other nodes.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-208 and FR-154. Execute each positive case and each explicit refusal mutation through the real local Rust boundary; compare the complete typed result and `normalize.record` accounting with the criterion.

Select `quire.model.complete/v1` at revision `1-draft.1` by its exact
DefinitionRefs in
[`complete-model-lock.json`](../../proposals/quire-v1/definitions/complete-model-lock.json).
Node names, declaration keys, the source `S1` and the admission of a
re-digested package follow TC-234. Every type definition and population is
declared by its own artifact, and its identity is `ix://test/orders/<artifact
id>`. `UUID`, `Int`, `String`, `Boolean` and `Timestamp` are native value
types and are not IR nodes.

Fixture **K2** (domain package `test/orders`, version `1`). Its `constructs`
table has these entries, all under module `test/business`, each kind named
differently from its meaning: `thing` with
`quire.meaning.model.object-type/v1`; `value` with
`quire.meaning.model.value-type/v1`; `money` with
`quire.meaning.model.record-value-type/v1`; `happened` with
`quire.meaning.model.event-type/v1`; `lifecycle` with
`quire.meaning.model.state-machine/v1`; `saga` with
`quire.meaning.model.process/v1`; `store` with
`quire.meaning.model.persistence-interface/v1`; `context` with
`quire.meaning.model.namespace/v1`; and `population` with
`quire.meaning.model.population/v1`. Nodes, with their declarations:

- `Order` (kind `thing`): identity field `order_id` typed `UUID`, field `total`
  typed `Money`. 3 declarations.
- `Instant` (kind `value`): bound as `Timestamp`. 1 declaration.
- `Money` (kind `money`): fields `amount_minor` typed `Int` and `currency`
  typed `String`, no identity field. 3 declarations.
- `OrderPlaced` (kind `happened`): fields `occurred_at` typed `Instant` and
  `order_id` typed `UUID`, no identity field, `occurrenceField` naming
  `OrderPlaced/occurred_at`. 3 declarations.
- `OrderLifecycle` (kind `lifecycle`): field `placed` typed `Boolean`;
  operations `place` and `cancel`, each with no parameters, no return, no
  `requires` or `ensures` and an empty frame; clause `NotYetPlaced`
  (`language: quire`, `not self.placed`); states `Draft`, `Placed`,
  `Cancelled`; transitions, in order, `[0]` `Draft → Placed` on `place`, guard
  `NotYetPlaced`, emits `[OrderPlaced]`; `[1]` `Draft → Cancelled` on
  `cancel`, emits `[]`; `[2]` `Placed → Cancelled` on `cancel`, emits `[]`.
  5 declarations.
- `Fulfilment` (kind `saga`): identity field `correlation_id` typed `UUID`;
  steps, in order, `[0]` `receive_order` (`event`, consumes `[OrderPlaced]`,
  emits `[]`), `[1]` `reserve_stock` (`command`, consumes `[]`, emits `[]`)
  and `[2]` `await_shipment` (`wait`, consumes `[]`, emits `[]`).
  2 declarations.
- `OrderStore` (kind `store`): operations `get` and `save`, each with no
  `requires` or `ensures` and an empty frame, no fields, `persists: [Order]`.
  3 declarations.
- `OrderManagement` (kind `context`): members `[Order, Money, OrderPlaced]`,
  vocabulary `[{term: Place, doc: "Commit a draft order"}]`. 1 declaration.

K2 has 21 declarations. Each refusal below also names the refused node's
artifact and span, and no refused run exposes an effective view. "N charges"
means the run charges `normalize.record` exactly N times before it reports.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| M01 | Admit K2 under S1 | admitted after 21 charges; `Money` binds as a record value type; `OrderPlaced` as an event type whose occurrence field `OrderPlaced/occurred_at` resolves through `Instant` to `Timestamp`; `OrderLifecycle` as a state machine with states `{Draft, Placed, Cancelled}`, exactly transitions `[0]`–`[2]`, and `NotYetPlaced` bound as the guard of `[0]` and as no invariant; `Fulfilment` as a process with correlation key `[Fulfilment/correlation_id]` and steps `receive_order`, `reserve_stock`, `await_shipment` in that order; `OrderStore` as a persistence interface with operation signatures `get`, `save` and `persists: [Order]`; `OrderManagement` as exactly one original declaration with key `(test/orders, ix://test/orders/OrderManagement)`, members `[Order, Money, OrderPlaced]`, vocabulary `[Place]` and no effective type. No node binds a meaning from its kind name, and the result binds no current, initial or final state, no firing, no step execution order and no storage law |
| M02 | K2 with `Order` given field `quoted` typed `Money` and clause `QuotedIsTotal` (`self.quoted = self.total`), then evaluated on one `Order` object with `total` `{amount_minor: 100, currency: "EUR"}` and `quoted` `{amount_minor: 100, currency: "EUR"}`; then with `quoted` `{amount_minor: 100, currency: "USD"}` | admitted after 23 charges; the clause is `true` for the first object and `false` for the second, because record values are equal exactly when every field is equal (FR-208-AC-4) |
| M03 | K2 with `Money/currency` flagged as an identity field; then K2 with operation `Money/add`; then K2 with clause `Money/NonNegative` in `quire`; then K2 with clause `Money/NonNegative` in `ocl` | run 1, 21 charges: `refused { code: invalid_model_binding, cause: malformed-declaration }` naming `ix://test/orders/Money`, member path `TypeDefinition.identityFields`; run 2, 22 charges: `refused { code: unsupported_construct, cause: declaration-form }` naming `ix://test/orders/Money/add`, member path `TypeDefinition.operations[0]`; run 3, 22 charges: `unsupported_construct`/`declaration-form` naming `ix://test/orders/Money/NonNegative`, member path `TypeDefinition.clauses[0]`; run 4: admitted after 22 charges, with the `ocl` clause kept with its span and never evaluated (FR-208-AC-4) |
| M04 | K2 with `OrderPlaced/order_id` flagged as an identity field; then `OrderPlaced.occurrenceField` naming `OrderPlaced/order_id`; then naming `OrderPlaced/at`, which is no field; then K2 with operation `OrderPlaced/replay`; then K2 with `Instant` bound as `Int` | run 1, 21 charges: `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/OrderPlaced`, member path `TypeDefinition.identityFields`; run 2, 21 charges: `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/OrderPlaced`, member path `TypeDefinition.occurrenceField`; run 3, 21 charges: `refused { code: missing_declaration, cause: missing-name }` naming `ix://test/orders/OrderPlaced`, member path `TypeDefinition.occurrenceField`, absent node `ix://test/orders/OrderPlaced/at`; run 4, 22 charges: `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/OrderPlaced/replay`, member path `TypeDefinition.operations[0]`; run 5, 21 charges: `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/OrderPlaced`, member path `TypeDefinition.occurrenceField` (FR-208-AC-4) |
| M05 | K2 with `OrderLifecycle` transition `[0]` changed: `to` `Shipped`; `trigger` `ship`; `guard` `Unknown`; `emits` `[Order]`; `emits` `[OrderShipped]`, which names no node; and, separately, clause `NotYetPlaced` in `ocl` | 21 charges each. Runs 1–3: `missing_declaration`/`missing-name` naming `ix://test/orders/OrderLifecycle`, member paths `TypeDefinition.transitions[0].to`, `.trigger` and `.guard`, absent state `Shipped`, operation `ix://test/orders/OrderLifecycle/ship` and clause `ix://test/orders/OrderLifecycle/Unknown`, and run 3 then reports `unsupported_construct`/`declaration-form` naming `ix://test/orders/OrderLifecycle/NotYetPlaced` at `TypeDefinition.clauses[0]`, which no guard names; run 4: `invalid_model_binding`/`malformed-declaration`, member path `TypeDefinition.transitions[0].emits[0]`; run 5: `missing_declaration`/`missing-name`, member path `TypeDefinition.transitions[0].emits[0]`, absent node `ix://test/orders/OrderShipped`; run 6: `invalid_model_binding`/`malformed-declaration`, member path `TypeDefinition.transitions[0].guard` (FR-208-AC-5) |
| M06 | K2 with `OrderLifecycle.operations` empty; then `states` empty; then `transitions` empty; then an added `quire` clause `OrderLifecycle/Extra` that no guard names; then a fourth state `Draft`; then a fourth transition `Draft → Cancelled` on `cancel`; then operation `place` with `ensures` `self.placed` | run 1, 19 charges: `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.operations`, then `missing_declaration`/`missing-name` at `TypeDefinition.transitions[0].trigger`, `[1].trigger` and `[2].trigger`, all naming `ix://test/orders/OrderLifecycle`; run 2, 21 charges: `malformed-declaration` at `TypeDefinition.states`, then `missing-name` at `transitions[0].from`, `[0].to`, `[1].from`, `[1].to`, `[2].from`, `[2].to`; run 3, 21 charges: `malformed-declaration` naming `ix://test/orders/OrderLifecycle` at `TypeDefinition.transitions`, then `unsupported_construct`/`declaration-form` naming `ix://test/orders/OrderLifecycle/NotYetPlaced` at `TypeDefinition.clauses[0]`; run 4, 22 charges: `unsupported_construct`/`declaration-form` naming `ix://test/orders/OrderLifecycle/Extra` at `TypeDefinition.clauses[1]`; run 5, 21 charges: `refused { code: invalid_model_binding, cause: conflicting-binding }` at `TypeDefinition.states[3]`; run 6, 21 charges: `invalid_model_binding`/`conflicting-binding` at `TypeDefinition.transitions[3]`; run 7, 21 charges: `unsupported_construct`/`declaration-form` naming `ix://test/orders/OrderLifecycle/place` at `Operation.post` (FR-208-AC-5) |
| M07 | K2 with `Fulfilment/correlation_id` no longer an identity field; then `steps` empty; then a fourth step named `reserve_stock`; then step `[0]` consuming `[Money]`; then step `[1]` emitting `[Order]`; then step `[1]` emitting `[StockReserved]`, which names no node; then operation `Fulfilment/restart`; then `quire` clause `Fulfilment/Bounded` | runs 1–6, 21 charges each, naming `ix://test/orders/Fulfilment`: `invalid_model_binding`/`malformed-declaration` at `TypeDefinition.identityFields`; `malformed-declaration` at `TypeDefinition.steps`; `invalid_model_binding`/`conflicting-binding` at `TypeDefinition.steps[3]`; `malformed-declaration` at `TypeDefinition.steps[0].consumes[0]`; `malformed-declaration` at `TypeDefinition.steps[1].emits[0]`; `missing_declaration`/`missing-name` at `TypeDefinition.steps[1].emits[0]` with absent node `ix://test/orders/StockReserved`. Run 7, 22 charges: `unsupported_construct`/`declaration-form` naming `ix://test/orders/Fulfilment/restart` at `TypeDefinition.operations[0]`; run 8, 22 charges: `unsupported_construct`/`declaration-form` naming `ix://test/orders/Fulfilment/Bounded` at `TypeDefinition.clauses[0]` (FR-208-AC-6) |
| M08 | K2 with field `OrderStore/cache` typed `Order`; then `OrderStore.operations` empty; then `persists` empty; then `persists` `[Money]`; then `persists` `[Invoice]`, which names no node; then operation `save` with `requires` `true`; then `quire` clause `OrderStore/Healthy` | run 1, 22 charges: `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/OrderStore/cache` at `TypeDefinition.fields[0]`; run 2, 19 charges: `malformed-declaration` naming `ix://test/orders/OrderStore` at `TypeDefinition.operations`; run 3, 21 charges: `malformed-declaration` at `TypeDefinition.persists`; run 4, 21 charges: `malformed-declaration` at `TypeDefinition.persists[0]`; run 5, 21 charges: `missing_declaration`/`missing-name` at `TypeDefinition.persists[0]`, absent node `ix://test/orders/Invoice`; run 6, 21 charges: `unsupported_construct`/`declaration-form` naming `ix://test/orders/OrderStore/save` at `Operation.pre`; run 7, 22 charges: `unsupported_construct`/`declaration-form` naming `ix://test/orders/OrderStore/Healthy` at `TypeDefinition.clauses[0]` (FR-208-AC-7) |
| M09 | K2 with field `OrderManagement/name` typed `String`; then operation `OrderManagement/close`; then K2 plus namespace `Billing` (kind `context`) with members `[Order]` and one term; then K2 plus namespace `Outer` (kind `context`) with members `[OrderManagement]` and one term; then `OrderManagement.members` `[Order, Money, OrderPlaced, Ghost]`, where `Ghost` names no node; then `quire` clause `OrderManagement/Scoped` | run 1, 22 charges: `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/OrderManagement/name` at `TypeDefinition.fields[0]`; run 2, 22 charges: `malformed-declaration` naming `ix://test/orders/OrderManagement/close` at `TypeDefinition.operations[0]`; run 3, 22 charges: `malformed-declaration` naming `ix://test/orders/OrderManagement`, which follows `Billing` in node order, at `TypeDefinition.members[0]`; run 4, 22 charges: `malformed-declaration` naming `ix://test/orders/Outer` at `TypeDefinition.members[0]`; run 5, 21 charges: `missing_declaration`/`missing-name` naming `ix://test/orders/OrderManagement` at `TypeDefinition.members[3]`, absent node `ix://test/orders/Ghost`; run 6, 22 charges: `unsupported_construct`/`declaration-form` naming `ix://test/orders/OrderManagement/Scoped` at `TypeDefinition.clauses[0]` (FR-208-AC-8) |
| M10 | K2 with field `Order/ref` typed, in turn, `OrderManagement`, `OrderStore`, `OrderPlaced`, `OrderLifecycle` and `Fulfilment`; then `Order.supertypes` `[OrderManagement]`; then relationship `Order/scope` whose target end names `OrderManagement`; then relationship `OrderLifecycle/next` whose target end names `Order`; then K2 plus population `PA` (kind `population`, extent `closed`) with member types `[Fulfilment]`; then with `[OrderManagement]` | runs 1–5, 22 charges each: `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/Order/ref` at `Field.typeRef`; run 6, 21 charges: `malformed-declaration` naming `ix://test/orders/Order` at `TypeDefinition.supertypes[0]`; run 7, 22 charges: `malformed-declaration` naming `ix://test/orders/Order/scope` at `Relationship.target`; run 8, 22 charges: `malformed-declaration` naming `ix://test/orders/OrderLifecycle/next` at `TypeDefinition.relationships[0]`; runs 9–10, 22 charges each: `malformed-declaration` naming `ix://test/orders/PA` at `Population.members[0].type`. With field `Order/ref` typed `Money` the run is admitted after 22 charges (FR-208-AC-9) |
| M11 | K2 with `supertypes` `[Order]` on, in turn, `Money`, `OrderPlaced`, `OrderLifecycle`, `Fulfilment`, `OrderStore` and `OrderManagement` | 21 charges each: exactly one `unsupported_construct`/`declaration-form` naming that node at `TypeDefinition.supertypes`, and no refusal for the entry `Order` (FR-208-AC-9) |
| M12 | K2 with the `lifecycle` construct's `meaning` set to `quire.meaning.state-machine`; then the `saga` construct's `meaning` set to `quire.meaning.process` | 21 charges each: `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/OrderLifecycle`, then `ix://test/orders/Fulfilment`, each at `constructs[].meaning`; no run binds a state machine or process from the kind name or the construct shape (FR-208-AC-3) |

## Expected Results

Every positive result matches the FR-208 rules of its node's construct meaning; each refusal carries its code, cause, node, member path, artifact and span, reported in FR-154 order after exactly the stated charges; and no meaning or behavior is taken from a kind name or from a member the meaning leaves structural.
