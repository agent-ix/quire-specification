---
id: TC-986
title: "Planner settlement request"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-492
    type: verifies
---

# TC-986: Planner settlement request

## Description

Admit, refuse and settle FR-492 requests for TC-984 case 1's guarantee item.

Scope: FR-492-AC-1 to FR-492-AC-12.

## Test Procedure

Build the request for TC-984 case 1's item, each item-backed slot filled with
the complete FR-331 request item and a `proved` result entry of that item,
and the environment slot with its record. Run each case through the real
local Rust boundary:

1. The request as built.
2. Drop the frame slot; swap two slots; relabel the frame slot
   `transition-simulation`; set `package` to another package identity.
3. Replace `slots` with `true`; replace one slot's `result` with `true`.
4. Remove the initialization slot's `result`; remove the environment slot's
   `record`; then build the request for TC-984 case 9's declaration with no
   `progress` member.
5. Fill the transition-simulation slot with a result whose request item
   belongs to the frame slot, then with one compiled against a rebound
   relation; then with a result entry that lacks `certification`.
6. Seed, one per run, the frame slot's result `unsupported`, `incomplete`
   and `inconclusive`; then make both the initialization and frame results
   `inconclusive`.
7. Make the model-claim result `tested`.
8. Make one result a plugin's `trusted` proof; then make every result
   `certified`.
9. Set `max_work` to 2.
10. Read a provider result entry carrying `premise-unproved`, then one
    carrying `transfer-not-preserving`.
11. Replace the clause with `always (holds(c.value >= 1) implies eventually always[0,1] holds(c.value = 2))`, then with a clause using `previous`, every slot proved.
12. Make the model-claim result name an assumption no environment record
    supplies; then, for TC-985 case 1's composition, offer only an
    environment record for the rely slot through `discharges`.
13. Fill the model-claim slot with a result checked without
    `stutter_closure`, then under `fair strong each inc`; fill the
    transition-simulation slot with the frame item's result.

## Expected Results

1. `proved`, `decisive-witness`, every slot outcome and the conclusion
   record.
2. Each `invalid_runtime_input`/`invalid-value` at the member.
3. Each `invalid_runtime_input`/`wrong-value-kind` at the member.
4. `invalid_runtime_input`/`invalid-value` at the slot twice; then
   `inconclusive`, `premise-unproved` naming the no-divergence slot with
   `result` `absent`.
5. `invalid_runtime_input`/`invalid-value` at `/slots/<i>/result` twice;
   then FR-331's `missing-member` refusal at the slot.
6. `unsupported`, `incomplete` and `inconclusive`/`premise-unproved`, each
   naming the frame slot; then the initialization slot.
7. `inconclusive`, `premise-unproved` naming the model-claim slot; no
   `refuted`.
8. `proved` with `trusted`; then `proved` with `uncertified`.
9. `incomplete`, `limit-reached` naming `max_work`.
10. Each refused `unknown-cause`.
11. Each `inconclusive`, `transfer-not-preserving{reason: fragment}` naming
    the interval and the previous operator.
12. `inconclusive`, `premise-unproved` naming the model-claim slot; then
    `premise-unproved` naming the rely slot with `result` `assumption`.
13. Each `invalid_runtime_input`/`invalid-value` at `/slots/<i>/result`.
