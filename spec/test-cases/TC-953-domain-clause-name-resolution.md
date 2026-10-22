---
id: TC-953
title: "Domain clause name resolution"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-470
    type: verifies
---

# TC-953: Domain clause name resolution

## Description

Check the clauses a domain package carries, under two importing units with
different model aliases, and mutate the first segment of each qualified name,
the owner's namespace membership and the clause text's leading form.

## Test Procedure

Select `quire.model.complete/v1` through
[`complete-model-lock.json`](../../proposals/quire-v1/definitions/complete-model-lock.json).
Run every vector through the real local Rust check of domain clauses and
compare the complete typed result with the criterion.

Fixtures:

- **B1** (artifact bundle, package identity `test/orders`, version `1`):
  - the `domain` artifact with id `OrderManagement` and title `OrderContext`,
    members `Order` and `OrderStatus`;
  - the `enumeration` artifact with id `OrderStatus`, cases `Draft` and
    `Placed`;
  - the `aggregate_root` artifact with id `Order`, fields `status` typed
    `OrderStatus` (required, `1..1`) and `placed_at` typed `Timestamp`
    (optional, `0..1`), and operation `place()` with `Modifies: status`;
  - the `entity` artifact with id `Audit`, field `entries` typed `Integer`
    (required, `1..1`), listed by no namespace.
- **K1**: the domain package that quire-rs extraction and the FCD
  extraction-frontend lift produce from B1.
- **U1**: a unit with `model M = "test/orders" digest "<SHA-256 over K1's JCS bytes>";`.
- **U2**: U1 with the alias `Shop` in place of `M`.

Each vector adds the named clauses to B1, re-extracts, re-lifts and re-digests
K1, and updates both units' digests.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| R01 | `Order` invariant `DraftUnplaced`: `self.status = OrderManagement::OrderStatus::Draft implies not present(self.placed_at)`; `Order` invariant `StatusKnown`: `self.status = OrderManagement::OrderStatus::Draft or self.status = OrderManagement::OrderStatus::Placed`; checked under U1 | no refusal; `OrderManagement::OrderStatus::Draft` resolves to `(test/orders, ix://test/orders/variant/OrderStatus-Draft)` and `OrderManagement::OrderStatus::Placed` to `(test/orders, ix://test/orders/variant/OrderStatus-Placed)` (FR-470-AC-1) |
| R02 | `Order/place` given `pre` entry `self.status = OrderManagement::OrderStatus::Draft`; checked under U1 | no refusal; the name resolves to the R01 `Draft` key through `Order`'s self alias (FR-470-AC-2) |
| R03 | R01 and R02 checked under U1 and again under U2 | the resolved keys and the check results are byte-equal between U1 and U2 (FR-470-AC-3) |
| R04 | `Order` invariant `ByUnitAlias`: `self.status = M::OrderStatus::Draft`, checked under U1; then the same with `Shop::` in place of `M::`, checked under U2 | each run is exactly one `refused { code: missing_declaration, cause: missing-name }` at the first segment (`M`, then `Shop`), naming `ix://test/orders/Order/ByUnitAlias` and its `origin` (FR-470-AC-3) |
| R05 | `Order` invariant `ByTitle`: `self.status = OrderContext::OrderStatus::Draft` | exactly one `refused { code: missing_declaration, cause: missing-name }` at `OrderContext`, naming `ix://test/orders/Order/ByTitle` and its `origin` (FR-470-AC-4) |
| R06 | `Audit` invariant `NoAlias`: `OrderManagement::OrderStatus::Draft = OrderManagement::OrderStatus::Draft`; `Audit` invariant `NonNegative`: `self.entries >= 0` | `NoAlias` gives two `refused { code: missing_declaration, cause: missing-name }` at `OrderManagement`, in source order, naming `ix://test/orders/Audit/NoAlias`; `NonNegative` checks with no refusal (FR-470-AC-5) |
| R07 | B1 plus the `domain` artifact with id `Billing`, members `Invoice`, and the `entity` artifact with id `Invoice`, field `status` typed `OrderStatus`; `Invoice` invariant `OwnAlias`: `self.status = Billing::OrderStatus::Draft`; `Invoice` invariant `OtherAlias`: `self.status = OrderManagement::OrderStatus::Draft` | `OwnAlias` checks with no refusal and its name resolves to the R01 `Draft` key; `OtherAlias` is exactly one `refused { code: missing_declaration, cause: missing-name }` at `OrderManagement`, naming `ix://test/orders/Invoice/OtherAlias` (FR-470-AC-6) |
| R08 | `Order` invariant `Absent`: `self.status = OrderManagement::OrderStatus::Shipped`; then `Order` invariant `TwoAbsent`: `self.status = OrderManagement::OrderStatus::Shipped or self.status = OrderManagement::OrderStatus::Lost` | `Absent` is one `refused { code: missing_declaration, cause: missing-name }` naming absent node `ix://test/orders/variant/OrderStatus-Shipped`; `TwoAbsent` is two such refusals, `...OrderStatus-Shipped` then `...OrderStatus-Lost` (FR-470-AC-7) |
| R09 | Four `Order` invariants whose text begins, in turn, with `model M = "test/orders" digest "<K1 digest>";`, `import`, `profile` and the `quire` header, each followed by `self.status = OrderManagement::OrderStatus::Draft` | each is exactly one `refused { code: invalid_syntax, cause: unexpected-token }` at its first token, and no name of that clause is resolved (FR-470-AC-8) |

## Expected Results

Every name in a domain clause resolves through its owner's self alias to one
key of the clause's own domain package, identically under U1 and U2. Each
unit alias, namespace title, foreign namespace id, absent name and leading
header form refuses with the exact code, cause, segment, clause identity and
`origin` the vector states, and no refusal changes another clause's result.
