---
id: TC-982
title: "Implementation temporal guarantee through checked correspondence"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-488
    type: verifies
---

# TC-982: Implementation temporal guarantee through checked correspondence

## Description

Settle an implementation-guarantee item over the compare-and-set counter
implementation, and show which premise each defect defeats.

Scope: FR-488-AC-1 to FR-488-AC-12.

## Test Procedure

Premises: the abstract counter's TC-317 case 1 claim `always eventually
holds(c.value = 2)` proved over its stutter closure under `fair weak each
Spec::Counter::inc`; an FR-353 relation revision binding the counter model to
the Rust `Counter::inc_cas` and its atomic cell; the environment premise "the
scheduler is weakly fair to `inc_cas`". Run each case through the real local
Rust boundary and compare the complete typed result.

1. All premises proved and supplied, request the item as a liveness guarantee.
2. Mutate `inc_cas` so one step writes a value the relation maps to no model
   step and no stutter.
3. Prove only the per-step contracts of FR-350 for `inc_cas`, with no progress item.
4. Replace the retry loop so that a failed compare-and-set repeats without a
   decreasing measure; then bound the loop by a counted measure.
5. Keep the environment premise weak, as the environment supplies it, and
   request `F_P = fair strong each inc`, a scheduling premise stronger than
   the environment's.
6. Request the safety clause `always holds(c.value >= 0)`; then request case
   1's clause with the progress items omitted; then omit a required environment
   premise; then name an environment premise record that does not exist.
7. Remove the binding of the cell; then run with no Verus backend registered;
   then deny the work-unit bound of a premise.
8. Supply a finite 10000-step implementation trace over which the clause
   holds and request the item.
9. Rebind the model to a new relation revision; then change an environment premise.

## Expected Results

1. `proved`, `decisive-witness`, with the conclusion record of FR-488.
2. Transition premise refuted with a counterexample, item `inconclusive`/`premise-unproved` naming it.
3. `inconclusive`/`premise-unproved` naming the first progress item.
4. The unmeasured loop `inconclusive`/`premise-unproved` naming the progress item; the counted loop `proved`.
5. The fairness-correspondence item `inconclusive`/`transfer-not-preserving` naming the strong constraint; the guarantee item `inconclusive`/`premise-unproved` naming the fairness-correspondence item.
6. Safety `proved` without progress items; the liveness item `inconclusive`/`premise-unproved` naming the first progress item; the omitted environment premise `inconclusive`/`premise-unproved` naming it; the nonexistent record refused `invalid_runtime_input`/`invalid-value`.
7. Refused `missing_declaration`/`missing-name`; `unsupported` naming the capability; `incomplete` naming the limit; the three are distinct and sibling items settle independently.
8. Not `proved`; no item settles `refuted`.
9. Each invalidates the record and the item is settled again.
