---
id: TC-117
title: "Prove or refuse native-to-TL temporal correspondence"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-090
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-092
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-095
    type: verifies
---
## Description

Accepted FS04 portable differential and identity-mutation controls for the
native temporal bridge. Qcir #71 owns execution.

## Test Procedure

For event-position correspondence, exhaustively enumerate ASTs of depth at most
two from constants, propositions `{p,q}`, negation, conjunction and the four
future operators using every interval `0 <= a <= b <= 3`; enumerate trace
lengths zero through five, every total Boolean valuation, every valid anchor and
both closed and open-prefix lifecycle states. Record the population cardinality
and any infeasible resource stop. For fixed-sample correspondence, exercise the
same finite population under two declared rational periods and epochs. Compare
an independent native oracle with the exact selected TL evaluator profiles.
Include the one-position `always[0,1] p` closure discriminator. Request
timestamped finite-window, each past operator alone under an execution-origin and a history-cutoff boundary, and a past operator combined with a future operator. Independently
mutate every native/TL profile, source/span, model/predicate, clock, capture,
interval, closure/history and bridge identity. For each mapped result,
independently omit or change assessment execution, decision-scope progress and
closure, surrounding-execution progress and closure, settlement basis, one
decision-support identity, completeness and supersession. Offer numeric,
nullable, unavailable and pending proposition inputs.

## Expected Results

Every member of the declared finite population agrees and retains full
correspondence, or a resource stop is reported without an exhaustive claim.
Finite-window,
timestamped and mixed past-and-future mappings return typed unsupported without a
substitute formula; each past-only mapping with an execution-origin boundary maps
to `mltl.origin-complete-history/v1` and agrees with the native oracle, and with a
history-cutoff boundary is incomplete. Each identity mutation fails on its own axis, and only total
native Boolean values enter the TL proposition population. Equal TL truth or
formula bytes never supply a missing result axis; every valid mapping retains
the native/shared-result metadata outside the TL formula where necessary.
