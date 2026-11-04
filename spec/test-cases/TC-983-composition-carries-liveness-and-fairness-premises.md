---
id: TC-983
title: "Composition carries liveness and fairness premises"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-489
    type: verifies
---

# TC-983: Composition carries liveness and fairness premises

## Description

Compose two components and settle each premise, mismatch and cycle separately.

## Test Procedure

Components `Producer` and `Consumer` are checked packages. `Consumer`'s
liveness guarantee `always (holds(received) implies eventually holds(done))`
depends on `Producer`'s `always eventually holds(sent)` under
`fair weak each Producer::send`, and relies on the safety guarantee
`always holds(sent_count >= received_count)`. Run through the real local Rust
boundary:

1. Supply all premises, each discharged by a proved guarantee naming its own
   subject, under a supported composition rule.
2. Supply the liveness dependency as one success flag; then as the four lists.
3. Discharge the liveness dependency with a guarantee over `Consumer`, then
   with a weaker liveness claim, then with a safety guarantee.
4. Make `Producer`'s guarantee depend on `Consumer`'s progress; then declare a
   well-founded measure and discharge it.
5. Omit a premise; supply an assumption as trusted; select an unsupported
   rule; fail a component check; deny a work-unit bound.
6. Supply a premise identity naming no settled result; one whose subject
   differs; one whose referenced result is `inconclusive`; add an unrelated
   supported claim to each request.

## Expected Results

1. `proved`, `decisive-witness`, with the premise discharge record.
2. The flag refused `invalid_runtime_input`/`invalid-value`; the four lists accepted.
3. Each `inconclusive`/`premise-mismatch`, premise undischarged.
4. `inconclusive`/`circular-reasoning`; with the measure `proved`.
5. `premise-missing`; `trusted` naming the assumption; `unsupported` naming the capability; `premise-unproved`; `incomplete`; five distinct outcomes.
6. Refused naming the member; refused; premise undischarged; the unrelated claim settles in each.
