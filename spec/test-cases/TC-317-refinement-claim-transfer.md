---
id: TC-317
title: "Claim transfer through a refinement"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-380
    type: verifies
---

# TC-317: Claim transfer through a refinement

## Description

Transfer stuttering-invariant claims through the compare-and-set refinement,
and settle each transfer that the fragment, the fairness implication, the
premise form, the premise results or the subject match does not allow.

## Test Procedure

Premise R is TC-312 case 1's proved refinement item. Abstract claim items
over the abstract counter subject:

1. `always eventually holds(c.value = 2)` proved over the abstract subject's
   stutter closure under `fair weak each Spec::Counter::inc`; request its
   transfer through R.
2. `always (holds(c.value = 1) implies eventually always[0,10] holds(c.value
   >= 1))`, proved on the abstract subject; request its transfer, then check
   it directly on the concrete subject.
3. `always (holds(c.value = 1) implies previous holds(c.value = 0))`, proved
   on the abstract subject; request its transfer, then check it directly on
   the concrete subject.
4. Case 1's clause under `F_P = fair weak each Spec::Counter::dec` for an
   added abstract operation `dec` not in `F_A`; then under
   `F_P = fair weak whole Spec::Counter::inc`.
5. `eventually holds(c.value = 2)` under empty `F_P`, proved over maximal
   abstract behaviours and refuted over the abstract stutter closure; request
   its transfer from the maximal-behaviour premise, then from a premise over
   a model where the clause holds over the stutter closure.
6. Case 1 with premise R replaced by TC-312 case 2's refuted refinement; then
   with an abstract premise that is `inconclusive`.
7. Case 1 with an abstract premise over the abstract counter with universe
   `{c, d}`.
8. A safety clause `always holds(c.value <= 2)` transferred through R, then
   used as the abstract premise of a transfer through a refinement of the
   concrete counter by a further model; then case 1's transferred liveness
   result used the same way, and a premise over the concrete counter's
   stutter closure used the same way.

## Expected Results

Case 1 settles `proved`, basis `decisive-witness`, with no exploration of
the concrete subject and both premise identities in its record
(FR-380-AC-1, FR-380-AC-8). Cases 2 and 3 settle `inconclusive`, cause
`transfer-not-preserving`, naming `always[0,10]` and `previous`; checked
directly, case 3's clause is refuted on the concrete subject at the first
`peek` after `commitA`, where `previous` reads `value = 1` (FR-380-AC-2). In
case 4 the `dec` constraint settles `transfer-not-preserving` naming it, and
the `whole` form transfers (FR-380-AC-3). Case 5 settles
`transfer-not-preserving` naming the missing stutter closure, and transfers
from the stutter-closure premise (FR-380-AC-4). Both runs of case 6 settle
`inconclusive`, cause `premise-unproved`, naming the premise; no case
settles `refuted` (FR-380-AC-5). Case 7 is refused
`invalid_runtime_input`/`invalid-value`, naming the universes (FR-380-AC-6).
In case 8 the transferred safety claim serves as a premise and its transfer
settles `proved`; the transferred liveness result does not serve, and the
stutter-closure premise does (FR-380-AC-7).
