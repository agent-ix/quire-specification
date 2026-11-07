---
id: TC-985
title: "Component composition declaration"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-491
    type: verifies
---

# TC-985: Component composition declaration

## Description

Check, emit and read back a `composition-decl` over TC-983's `Producer` and
`Consumer` packages, and show its slots, cycles and refusals.

Scope: FR-491-AC-1 to FR-491-AC-6.

## Test Procedure

The composing unit imports TC-983's checked packages as `P` and `Q`. Run each
case through the real local Rust boundary:

1. `compose Sys rule assume-guarantee { component Producer from P using M;
   component Consumer from Q using M; guarantee Producer::Sends;
   guarantee Consumer::Done; guarantee Producer::Counted;
   rely Consumer::Done on Producer::Counted;
   depends Consumer::Done on Producer::Sends;
   supplies fair weak each M::Producer::send; }`, where `Sends` is
   `always eventually holds(sent)`, `Done` is
   `always (holds(received) implies eventually holds(done))` and `Counted` is
   `always holds(sent_count >= received_count)`.
2. One `component`; `rule cooperative`; a `component Producer from Z using M`
   with no import `Z`; `guarantee Producer::Missing`; two components named
   `Producer`.
3. `rely Consumer::Done on Producer::Sends` (a liveness peer); `depends
   Consumer::Done on Producer::Counted` (a safety peer); a `rely` whose
   guarantee is not declared; `rely Producer::Sends on Producer::Counted`.
4. Add `depends Producer::Sends on Consumer::Done`; then select a profile
   that declares FR-176's lattice, initial element and convergence rule.
5. Case 1 with `rule interface`.
6. Emit case 1, read back and compare; swap two `relies` entries.

## Expected Results

1. The planner-local composition item and, in order, three component-guarantee
   slots, one rely slot, one liveness-dependency slot and one fairness slot
   per resolved constraint; the four lists separate.
2. `invalid_syntax` twice; `missing_declaration`/`missing-name` twice;
   `invalid_model_binding`/`conflicting-binding`.
3. Each `invalid_model_binding`/`malformed-declaration` naming the member.
4. The unit checks; the composition item refuses `invalid_model_binding`/
   `malformed-declaration` naming `Producer::Sends` and `Consumer::Done`, and
   the unit's other items are unchanged; with the profile, one fixed-point
   slot.
5. One FR-331 `composition` request item whose `premises` holds the slots;
   no planner-local item.
6. The same declaration, lists and slot identities, each slot's source span
   its member; `invalid_semantic_graph`.
