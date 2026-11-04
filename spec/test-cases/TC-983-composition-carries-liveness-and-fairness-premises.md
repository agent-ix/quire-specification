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

Scope: FR-489-AC-1 to FR-489-AC-11.

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
3. Supply the liveness dependency's guarantee over `Consumer` instead of
   `Producer`, then a weaker liveness claim, then a safety guarantee, each as
   a supplied guarantee that does not name a discharge record.
4. Make `Producer`'s guarantee depend on `Consumer`'s progress under a rule
   with no declared lattice; then under a profile declaring FR-176's monotone
   lattice, initial element and convergence rule.
5. Supply a premise discharged by an accepted plugin result carrying certification `trusted`; then a premise supplied only as an assumption with no plugin result; select an
   unsupported rule; fail a component check; deny a work-unit bound.
6. Supply a discharge record whose guarantee subject differs from its premise's
   subject; one whose identity names no settled result; one whose referenced
   result is `inconclusive`; add an unrelated supported claim to each request.

## Expected Results

1. `proved`, `decisive-witness`, with the premise discharge record.
2. The flag refused `invalid_runtime_input`/`invalid-value`; the four lists accepted.
3. Each `inconclusive`/`premise-unproved` naming the premise, premise undischarged.
4. Refused as FR-176-AC-3 refuses circular reasoning; with the lattice accepted and every iteration recorded.
5. `proved` with certification `trusted` citing the plugin result; `inconclusive`/`premise-unproved` naming the premise and the assumption; `unsupported` naming the capability; `inconclusive`/`premise-unproved`; `incomplete`; five distinct outcomes.
6. The first two refused `invalid_runtime_input`/`invalid-value` naming the member; the third leaves the premise undischarged (`inconclusive`/`premise-unproved`); the unrelated claim settles in each.
